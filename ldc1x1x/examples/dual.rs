use ldc1x1x as ldc;
use std::str::FromStr;

fn main() {
    #[cfg(target_os = "freebsd")]
    let iic = freebsd_embedded_hal::I2cBus::from_unit(
        u32::from_str(&std::env::var("I2C_UNIT").unwrap_or_else(|_| "1".to_string())).unwrap(),
    )
    .unwrap();
    // #[cfg(target_os = "linux")] port me

    let adr =
        u8::from_str_radix(&std::env::var("I2C_ADDR").unwrap_or_else(|_| "2a".to_string()), 16)
            .unwrap();

    let mut ldc = ldc::Ldc::new(iic, adr);
    ldc.reset().unwrap();

    let div = ldc::Fsensor::from_inductance_capacitance(12.583, 100.0).to_clock_dividers(None);
    for ch in [ldc::Channel::Zero, ldc::Channel::One] {
        ldc.set_clock_dividers(ch, div).unwrap();
        ldc.set_conv_settling_time(ch, 40).unwrap();
        ldc.set_ref_count_conv_interval(ch, 0x0546).unwrap();
        ldc.set_sensor_drive_current(ch, 0b01110).unwrap();
    }

    ldc.set_mux_config(
        ldc::MuxConfig::default()
            .with_auto_scan(true)
            .with_deglitch_filter_bandwidth(ldc::Deglitch::ThreePointThreeMHz),
    )
    .unwrap();
    ldc.set_config(ldc::Config::default()).unwrap();
    ldc.set_error_config(
        ldc::ErrorConfig::default().with_amplitude_high_error_to_data_register(true),
    )
    .unwrap();

    // timing ignored because polling with a cp2112 with no delays is slow enough already
    // outputting just newline separated numbers so you can feed it into https://github.com/mogenson/ploot
    loop {
        println!(
            "{} {}",
            ldc.read_data_24bit(ldc::Channel::Zero).unwrap(),
            ldc.read_data_24bit(ldc::Channel::One).unwrap(),
        );
    }
}
