// Mycelium plant + sensor simulation for Wokwi.
// Emulates BH1730FVC (0x29), SHTC3 (0x70), and custom soil sensor (0x55).

#include "wokwi-api.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ADDR_BH1730 0x29
#define ADDR_SHTC3 0x70
#define ADDR_SOIL 0x55

#define BH_REG_CONTROL 0x00
#define BH_REG_TIMING 0x01
#define BH_REG_GAIN 0x07
#define BH_REG_DATA0_LOW 0x14

typedef struct {
  float soil_pf;
  float ambient_temp;
  float ambient_humidity;
  float ambient_lux;

  uint32_t attr_soil_pf_dry;
  uint32_t attr_soil_pf_wet;
  uint32_t attr_evap_rate;
  uint32_t attr_water_amount;
  uint32_t attr_ambient_temp;
  uint32_t attr_ambient_humidity;
  uint32_t attr_ambient_lux;

  pin_t pin_water;
  bool water_pressed;

  uint8_t bh_regs[32];
  uint8_t bh_reg_addr;
  bool bh_cmd_byte;
  uint8_t bh_read_reg;
  uint8_t bh_read_idx;

  uint8_t sht_cmd[2];
  uint8_t sht_cmd_len;
  uint8_t sht_read_idx;
  uint8_t sht_buf[6];

  uint8_t soil_cmd[2];
  uint8_t soil_cmd_len;
  uint8_t soil_read_idx;
  uint8_t soil_read_buf[3];

  timer_t physics_timer;
  double last_physics_ns;
} plant_state_t;

static uint8_t sensirion_crc8(const uint8_t *data, int len) {
  uint8_t crc = 0xff;
  for (int i = 0; i < len; i++) {
    crc ^= data[i];
    for (int bit = 0; bit < 8; bit++) {
      if (crc & 0x80) {
        crc = (uint8_t)((crc << 1) ^ 0x31);
      } else {
        crc <<= 1;
      }
    }
  }
  return crc;
}

static void soil_pf_to_bytes(float pf, uint8_t out[3]) {
  if (pf < 0.0f) {
    pf = 0.0f;
  }
  uint32_t whole = (uint32_t)pf;
  float frac = pf - (float)whole;
  uint32_t d1 = (uint32_t)(frac * 256.0f);
  float frac2 = frac * 256.0f - (float)d1;
  uint32_t d2 = (uint32_t)(frac2 * 256.0f);
  out[0] = (uint8_t)(whole & 0xff);
  out[1] = (uint8_t)(d1 & 0xff);
  out[2] = (uint8_t)(d2 & 0xff);
}

static uint16_t encode_sht_temp(float temp_c) {
  float millideg = (temp_c + 45.0f) * 1000.0f;
  uint32_t raw = (uint32_t)(((millideg + 45000.0f) * 8192.0f) / 21875.0f);
  if (raw > 0xffff) {
    raw = 0xffff;
  }
  return (uint16_t)raw;
}

static uint16_t encode_sht_humidity(float humidity_pct) {
  float millipct = humidity_pct * 1000.0f;
  uint32_t raw = (uint32_t)((millipct * 8192.0f) / 12500.0f);
  if (raw > 0xffff) {
    raw = 0xffff;
  }
  return (uint16_t)raw;
}

static void prepare_sht_buffer(plant_state_t *chip) {
  uint16_t raw_t = encode_sht_temp(chip->ambient_temp);
  uint16_t raw_h = encode_sht_humidity(chip->ambient_humidity);
  chip->sht_buf[0] = (uint8_t)(raw_t >> 8);
  chip->sht_buf[1] = (uint8_t)(raw_t & 0xff);
  chip->sht_buf[2] = sensirion_crc8(chip->sht_buf, 2);
  chip->sht_buf[3] = (uint8_t)(raw_h >> 8);
  chip->sht_buf[4] = (uint8_t)(raw_h & 0xff);
  chip->sht_buf[5] = sensirion_crc8(&chip->sht_buf[3], 2);
}

static void bh_update_light_data(plant_state_t *chip) {
  float lux = chip->ambient_lux;
  if (lux < 0.0f) {
    lux = 0.0f;
  }
  uint16_t data0 = (uint16_t)(lux / 1.29f + 2.0f);
  if (data0 < 4) {
    data0 = 4;
  }
  uint16_t data1 = 1;
  chip->bh_regs[BH_REG_DATA0_LOW] = (uint8_t)(data0 & 0xff);
  chip->bh_regs[BH_REG_DATA0_LOW + 1] = (uint8_t)(data0 >> 8);
  chip->bh_regs[BH_REG_DATA0_LOW + 2] = (uint8_t)(data1 & 0xff);
  chip->bh_regs[BH_REG_DATA0_LOW + 3] = (uint8_t)(data1 >> 8);
  chip->bh_regs[BH_REG_CONTROL] |= 0x10;
}

static void refresh_attrs(plant_state_t *chip) {
  chip->ambient_temp = (float)attr_read(chip->attr_ambient_temp);
  chip->ambient_humidity = (float)attr_read(chip->attr_ambient_humidity);
  chip->ambient_lux = (float)attr_read(chip->attr_ambient_lux);
}

static void apply_water(plant_state_t *chip) {
  float wet = (float)attr_read(chip->attr_soil_pf_wet);
  float amount = (float)attr_read(chip->attr_water_amount);
  chip->soil_pf += amount;
  if (chip->soil_pf > wet) {
    chip->soil_pf = wet;
  }
  printf("Plant sim: watered, soil_pf=%.2f pF\n", chip->soil_pf);
}

static void on_water_change(void *user_data, pin_t pin, uint32_t value) {
  plant_state_t *chip = (plant_state_t *)user_data;
  (void)pin;
  if (value == LOW && !chip->water_pressed) {
    chip->water_pressed = true;
    apply_water(chip);
  } else if (value == HIGH) {
    chip->water_pressed = false;
  }
}

static void on_physics_tick(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  double now = get_sim_nanos_d();
  double dt_s = (now - chip->last_physics_ns) / 1e9;
  if (dt_s <= 0.0) {
    return;
  }
  chip->last_physics_ns = now;

  refresh_attrs(chip);

  float dry = (float)attr_read(chip->attr_soil_pf_dry);
  float base_evap = (float)attr_read(chip->attr_evap_rate);
  float lux_factor = 1.0f + (chip->ambient_lux / 1000.0f) * 0.2f;
  float temp_factor = 1.0f + ((chip->ambient_temp - 20.0f) / 10.0f) * 0.1f;
  float hum_factor = 1.0f - (chip->ambient_humidity / 100.0f) * 0.3f;
  if (hum_factor < 0.2f) {
    hum_factor = 0.2f;
  }

  float evap = base_evap * lux_factor * temp_factor * hum_factor * (float)dt_s;
  chip->soil_pf -= evap;
  if (chip->soil_pf < dry) {
    chip->soil_pf = dry;
  }

  bh_update_light_data(chip);
  prepare_sht_buffer(chip);
}

static bool bh_connect(void *user_data, uint32_t address, bool read) {
  plant_state_t *chip = (plant_state_t *)user_data;
  (void)address;
  (void)read;
  chip->bh_cmd_byte = false;
  chip->bh_read_idx = 0;
  return true;
}

static bool bh_write(void *user_data, uint8_t data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  if (data == 0xe4) {
    memset(chip->bh_regs, 0, sizeof(chip->bh_regs));
    chip->bh_regs[BH_REG_TIMING] = 0xda;
    chip->bh_regs[BH_REG_GAIN] = 0x00;
    chip->bh_cmd_byte = false;
    return true;
  }

  if (!chip->bh_cmd_byte) {
    if ((data & 0x80) == 0) {
      return false;
    }
    chip->bh_reg_addr = data & 0x7f;
    chip->bh_cmd_byte = true;
    return true;
  }

  chip->bh_regs[chip->bh_reg_addr] = data;
  chip->bh_cmd_byte = false;
  if (chip->bh_reg_addr == BH_REG_CONTROL) {
    chip->bh_regs[BH_REG_CONTROL] |= 0x10;
    bh_update_light_data(chip);
  }
  return true;
}

static uint8_t bh_read(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  if (chip->bh_read_reg == BH_REG_DATA0_LOW) {
    uint8_t value = chip->bh_regs[BH_REG_DATA0_LOW + chip->bh_read_idx];
    chip->bh_read_idx++;
    if (chip->bh_read_idx >= 4) {
      chip->bh_read_idx = 0;
    }
    return value;
  }

  uint8_t value = chip->bh_regs[chip->bh_read_reg];
  return value;
}

static void bh_disconnect(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  if (chip->bh_cmd_byte) {
    chip->bh_read_reg = chip->bh_reg_addr;
    chip->bh_read_idx = 0;
    chip->bh_cmd_byte = false;
  }
}

static bool sht_connect(void *user_data, uint32_t address, bool read) {
  plant_state_t *chip = (plant_state_t *)user_data;
  (void)address;
  if (read) {
    chip->sht_read_idx = 0;
    prepare_sht_buffer(chip);
  } else {
    chip->sht_cmd_len = 0;
  }
  return true;
}

static bool sht_write(void *user_data, uint8_t data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  if (chip->sht_cmd_len < 2) {
    chip->sht_cmd[chip->sht_cmd_len++] = data;
  }
  return true;
}

static uint8_t sht_read(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  uint8_t value = chip->sht_buf[chip->sht_read_idx++];
  if (chip->sht_read_idx >= 6) {
    chip->sht_read_idx = 0;
  }
  return value;
}

static void sht_disconnect(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  chip->sht_cmd_len = 0;
  chip->sht_read_idx = 0;
}

static bool soil_connect(void *user_data, uint32_t address, bool read) {
  plant_state_t *chip = (plant_state_t *)user_data;
  (void)address;
  if (read) {
    chip->soil_read_idx = 0;
    soil_pf_to_bytes(chip->soil_pf, chip->soil_read_buf);
  } else {
    chip->soil_cmd_len = 0;
  }
  return true;
}

static bool soil_write(void *user_data, uint8_t data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  if (chip->soil_cmd_len < 2) {
    chip->soil_cmd[chip->soil_cmd_len++] = data;
  }
  return true;
}

static uint8_t soil_read(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  uint8_t value = chip->soil_read_buf[chip->soil_read_idx++];
  if (chip->soil_read_idx >= 3) {
    chip->soil_read_idx = 0;
  }
  return value;
}

static void soil_disconnect(void *user_data) {
  plant_state_t *chip = (plant_state_t *)user_data;
  chip->soil_cmd_len = 0;
  chip->soil_read_idx = 0;
}

void chip_init(void) {
  plant_state_t *chip = (plant_state_t *)malloc(sizeof(plant_state_t));
  memset(chip, 0, sizeof(plant_state_t));

  chip->attr_soil_pf_dry = attr_init("soil_pf_dry", 45);
  chip->attr_soil_pf_wet = attr_init("soil_pf_wet", 120);
  chip->attr_evap_rate = attr_init("evap_rate", 2);
  chip->attr_water_amount = attr_init("water_amount", 25);
  chip->attr_ambient_temp = attr_init("ambient_temp", 22);
  chip->attr_ambient_humidity = attr_init("ambient_humidity", 55);
  chip->attr_ambient_lux = attr_init("ambient_lux", 800);

  chip->soil_pf = (float)attr_read(chip->attr_soil_pf_dry);
  refresh_attrs(chip);
  bh_update_light_data(chip);
  prepare_sht_buffer(chip);

  chip->pin_water = pin_init("WATER", INPUT_PULLUP);
  pin_watch(chip->pin_water,
            &(pin_watch_config_t){
                .user_data = chip,
                .edge = FALLING,
                .pin_change = on_water_change,
            });

  chip->physics_timer = timer_init(&(timer_config_t){
      .user_data = chip,
      .callback = on_physics_tick,
  });
  chip->last_physics_ns = get_sim_nanos_d();
  timer_start(chip->physics_timer, 1000000, true);

  i2c_init(&(i2c_config_t){
      .user_data = chip,
      .address = ADDR_BH1730,
      .scl = pin_init("SCL0", INPUT_PULLUP),
      .sda = pin_init("SDA0", INPUT_PULLUP),
      .connect = bh_connect,
      .read = bh_read,
      .write = bh_write,
      .disconnect = bh_disconnect,
  });

  i2c_init(&(i2c_config_t){
      .user_data = chip,
      .address = ADDR_SHTC3,
      .scl = pin_init("SCL0", INPUT_PULLUP),
      .sda = pin_init("SDA0", INPUT_PULLUP),
      .connect = sht_connect,
      .read = sht_read,
      .write = sht_write,
      .disconnect = sht_disconnect,
  });

  i2c_init(&(i2c_config_t){
      .user_data = chip,
      .address = ADDR_SOIL,
      .scl = pin_init("SCL1", INPUT_PULLUP),
      .sda = pin_init("SDA1", INPUT_PULLUP),
      .connect = soil_connect,
      .read = soil_read,
      .write = soil_write,
      .disconnect = soil_disconnect,
  });

  pin_init("PWR", INPUT);
  printf("Mycelium plant sim ready (BH1730=0x29, SHTC3=0x70, soil=0x55)\n");
}
