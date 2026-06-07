edge-central
---

### Builing Linux AARCH64

Since we use an Orange Pi Zero 2W we need to build for an AARCH64 processor. Also we installed DietPi on it, so to speed up the dev cycle have the following installed

```
brew tap messense/macos-cross-toolchains
brew reinstall aarch64-unknown-linux-musl
```

`bluer` and `btleplug` depend on D-Bus. For `aarch64-unknown-linux-musl` cross-builds, `edge-central` enables the `dbus` crate's `vendored` feature so libdbus is compiled and linked statically — you do not need Homebrew `dbus` or `PKG_CONFIG_*` overrides for cross-compilation.

To run an the build:

```
cargo build --target aarch64-unknown-linux-musl
```

or

```
just central-build-dietpi
```

or

```
just central-run-local
```

and to roll-out on the diet / orange pi directly

```
just central-build-roll
```


### Orange Pi Zero 2W
To run edge-central on a OrangePi you can use the `DietPi_OrangePiZero2W-ARMv8-Trixie` distribution

To connect an 0.91 inch OLED Display 128*32 pixels White - I2C with SSD1306 you need to connect it to the GPIO headers of the Orange Pi

![wiring](https://lh6.googleusercontent.com/proxy/dap_AdqlDHhF1VcgtIjEuCAcxUk5Qa8dccYOqOGKaTpTVcN1lZ0mcJ4iw7tUG7I4vmrRoI-UjGsRJijyxoyOND90SnrnWNAKiGQFDnWNwdcT_sORN8eRLk-hUbxc0GqwiVD97HznX0Ia96T7ANes)

As you can see you different options, but I went for the SDA (PI10) and SCL (PI9) setup. 

To figure out how this is wired into the kernel you need the dtc (device tree compiler)

```
apt-get install device-tree-compiler
dtc --sort -I fs -O dts  /sys/firmware/devicetree/base > dts-spi-out.txt
```

In this text file you see how i2c and the respective busses are wired.

In the end ChatGPT stumbled upon this fragment suggested that this fragment should match the PI10 and PI9 setup

```
i2c3-pg-pins {
    function = "i2c3";
    phandle = <0x61>;
    pins = "PG17", "PG18";
};
```

These correspond with physical wire setup, so it's connected to `i2c-3` which is located at `/dev/i2c-3`

### BLE onboarding WiFi

By default (`APP.WIFI_PROVISIONER=noop`) the hub accepts WiFi credentials over BLE but does not change system networking — configure WiFi on the Orange Pi separately (e.g. DietPi). Set `APP.WIFI_PROVISIONER=system` to join via `wpa_cli` or `nmcli` on the hub.
