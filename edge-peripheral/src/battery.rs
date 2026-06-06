use esp_hal::{analog::adc::{Adc, AdcChannel, AdcPin}, peripherals::ADC1, Blocking};

pub struct BatteryMeasurement<'a, PIN> where PIN : AdcChannel {
    adc: Adc<'a, ADC1<'a>, Blocking>,
    adc_pin: AdcPin<PIN, ADC1<'a>>
}

impl <'a, PIN : AdcChannel> BatteryMeasurement<'a, PIN> {
    pub fn new(adc: Adc<'a, ADC1<'a>, Blocking>, pin: AdcPin<PIN, ADC1<'a>>) -> Self {
        BatteryMeasurement { adc: adc, adc_pin: pin }
    }

    pub fn sample(&mut self) -> u8 {
        let reading = nb::block!(self.adc.read_oneshot(&mut self.adc_pin)).expect("Unable to read from ADC");

        if reading < ADC_LUT[0] {
            return 0;
        }

        for n in 0..ADC_LUT.len() - 1 {
            let lut_n = ADC_LUT[n];
            let lut_n_1 = ADC_LUT[n + 1];

            if reading >= lut_n && reading < lut_n_1 {
                let v = 1.7 + n as f32 * 0.1
                    + (reading - lut_n) as f32 * 0.1 / (lut_n_1 - lut_n) as f32;

                let p = (v / 2200_f32) * 100_f32;

                return p as u8;
            }
        }

        return 100;
    }
    
}

const ADC_LUT: &[u16; 24] = &[
    825,      // 1.7V (brownout below this)
    887,      // 1.8V 
    946,      // 1.9V
    1011,     // 2.0V
    1076,     // 2.1V
    1136,     // 2.2V
    1200,     // 2.3V
    1260,     // 2.4V
    1318,     // 2.5V
    1377,     // 2.6V
    1437,     // 2.7V
    1495,     // 2.8V
    1557,     // 2.9V
    1616,     // 3.0V
    1680,     // 3.1V
    1743,     // 3.2V
    1800,     // 3.3V
    1857,     // 3.4V
    1919,     // 3.5V
    1977,     // 3.6V
    2034,     // 3.7V
    2086,     // 3.8V
    2145,     // 3.9V
    2200,     // 4.0V
];
