//! 用于测试ldc161x板子能否正常工作

use std::{thread::sleep, time::Duration};

fn main() {
    // 初始化日志
    use env_logger::Builder;
    use log::debug;
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    use ldc1614::{
        bitmap::{
            CLOCK_DIVIDERSx, DRIVE_CURRENTx, OFFSETx, RCOUNTx, SETTLECOUNTx, CONFIG, ERROR_CONFIG,
            MUX_CONFIG, RESET_DEV,
        },
        Ldc,
    };
    use rppal::i2c::I2c;
    debug!("初始化i2c设备");
    let mut i2c = I2c::new().unwrap();
    let ldc = Ldc::<0x2b>::new(&mut i2c);
    ldc.register
        .reset_dev
        .write(&mut i2c, RESET_DEV::device_reset::reset);
    ldc.register
        .rcountx
        .0
        .write(&mut i2c, RCOUNTx::rcount.val(0x04d6));
    ldc.register
        .offsetx
        .0
        .write(&mut i2c, OFFSETx::offset.val(0x0000));
    ldc.register
        .settlecountx
        .0
        .write(&mut i2c, SETTLECOUNTx::settlecount.val(0x000a));
    ldc.register.clock_dividersx.0.write(
        &mut i2c,
        CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
    );
    ldc.register.error_config.write(
        &mut i2c,
        ERROR_CONFIG::data_ready_to_INTB::no_report
            + ERROR_CONFIG::zero_count_error_to_INTB::no_report
            + ERROR_CONFIG::amplitude_low_error_to_INTB::no_report
            + ERROR_CONFIG::amplitude_high_error_to_INTB::no_report
            + ERROR_CONFIG::watchdog_timeout_error_to_INTB::no_report
            + ERROR_CONFIG::over_range_error_to_INTB::no_report
            + ERROR_CONFIG::under_range_error_to_INTB::no_report
            + ERROR_CONFIG::amplitude_low_error_to_output_register::no_report
            + ERROR_CONFIG::amplitude_high_error_to_output_register::no_report
            + ERROR_CONFIG::watchdog_timeout_error_to_output_register::no_report
            + ERROR_CONFIG::over_range_error_to_output_register::no_report
            + ERROR_CONFIG::under_range_error_to_output_register::no_report,
    );
    ldc.register.config.write(
        &mut i2c,
        CONFIG::high_current_sensor_drive::normal
            + CONFIG::INTB_asserted::enable
            + CONFIG::select_reference_frequency_source::internal_oscillator
            + CONFIG::automatic_sensor_amplitude_correction::disable
            + CONFIG::sensor_activation_mode::low
            + CONFIG::sensor_rp_override::off
            + CONFIG::sleep_mode::sleep
            + CONFIG::active_channel::channel0,
    );
    ldc.register.mux_config.write(
        &mut i2c,
        MUX_CONFIG::input_deglitch_filter_bandwidth::with_3MHz3
            + MUX_CONFIG::auto_scan_sequence_config::channel_0_1
            + MUX_CONFIG::auto_scan_mode::manual,
    );
    ldc.register.drive_currentx.0.write(
        &mut i2c,
        DRIVE_CURRENTx::sensor_current_drive.val(0)
            + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
    );
    ldc.register.config.write(
        &mut i2c,
        CONFIG::high_current_sensor_drive::normal
            + CONFIG::INTB_asserted::enable
            + CONFIG::select_reference_frequency_source::internal_oscillator
            + CONFIG::automatic_sensor_amplitude_correction::disable
            + CONFIG::sensor_activation_mode::low
            + CONFIG::sensor_rp_override::off
            + CONFIG::sleep_mode::active
            + CONFIG::active_channel::channel0,
    );

    loop {
        sleep(Duration::from_secs(1));
        println!(
            "{}",
            ldc.read_data(&mut i2c, ldc1614::Channel::Zero).unwrap()
        )
    }
}
