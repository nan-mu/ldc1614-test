//! 包含从ldc1614::Ldc得到对应通道代码的结构。最后的形式是从一个tokio的广播结构发送数据

use super::{Error, Result};

use chrono::Local;
use tokio::sync::{self, broadcast};

pub struct Channel<const ADDR: u8> {
    ldc: std::sync::Arc<sync::Mutex<ldc1614::Ldc<ADDR>>>,
    i2c: std::sync::Arc<sync::Mutex<rppal::i2c::I2c>>,
    channel: ldc1614::Channel,
    rx: broadcast::Sender<super::Record>,
}

use crate::config;

impl<const ADDR: u8> Channel<ADDR> {
    pub fn from(
        channel: ldc1614::Channel,
        rx: broadcast::Sender<super::Record>,
        ldc: std::sync::Arc<sync::Mutex<ldc1614::Ldc<ADDR>>>,
        i2c: std::sync::Arc<sync::Mutex<rppal::i2c::I2c>>,
    ) -> Self {
        Self {
            ldc,
            i2c,
            channel,
            rx,
        }
    }
}

impl<const ADDR: u8> Channel<ADDR> {
    pub async fn submit(&self, mark: Option<std::sync::Arc<str>>) -> Result<usize> {
        let (ldc, mut i2c) = tokio::join!(self.ldc.lock(), self.i2c.lock());
        Ok(self.rx.send(crate::Record {
            timestamp: Local::now(),
            data: ldc
                .read_data(&mut *i2c, self.channel)
                .map_err(|e| Error::Ldc1614(e))?,
            channel: self.channel,
            mark,
        })?)
    }
    pub async fn apply_reg_config(&mut self, register: Vec<config::Register>) -> Result<()> {
        use ldc1614::{
            bitmap::{
                CLOCK_DIVIDERSx, DRIVE_CURRENTx, OFFSETx, RCOUNTx, SETTLECOUNTx, CONFIG,
                ERROR_CONFIG, MUX_CONFIG, RESET_DEV,
            },
            Channel,
        };

        let (ldc, mut i2c) = tokio::join!(self.ldc.lock(), self.i2c.lock());
        ldc.register
            .reset_dev
            .write(&mut *i2c, RESET_DEV::device_reset::reset);
        match self.channel {
            Channel::Zero => {
                ldc.register
                    .rcountx
                    .0
                    .write(&mut *i2c, RCOUNTx::rcount.val(0x04d6));
                ldc.register
                    .offsetx
                    .0
                    .write(&mut *i2c, OFFSETx::offset.val(0x0000));
                ldc.register
                    .settlecountx
                    .0
                    .write(&mut *i2c, SETTLECOUNTx::settlecount.val(0x000a));
                ldc.register.clock_dividersx.0.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                ldc.register.drive_currentx.0.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::One => {
                ldc.register
                    .rcountx
                    .1
                    .write(&mut *i2c, RCOUNTx::rcount.val(0x04d6));
                ldc.register
                    .offsetx
                    .1
                    .write(&mut *i2c, OFFSETx::offset.val(0x0000));
                ldc.register
                    .settlecountx
                    .1
                    .write(&mut *i2c, SETTLECOUNTx::settlecount.val(0x000a));
                ldc.register.clock_dividersx.1.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                ldc.register.drive_currentx.1.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::Two => {
                ldc.register
                    .rcountx
                    .2
                    .write(&mut *i2c, RCOUNTx::rcount.val(0x04d6));
                ldc.register
                    .offsetx
                    .2
                    .write(&mut *i2c, OFFSETx::offset.val(0x0000));
                ldc.register
                    .settlecountx
                    .2
                    .write(&mut *i2c, SETTLECOUNTx::settlecount.val(0x000a));
                ldc.register.clock_dividersx.2.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                ldc.register.drive_currentx.2.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::Three => {
                ldc.register
                    .rcountx
                    .3
                    .write(&mut *i2c, RCOUNTx::rcount.val(0x04d6));
                ldc.register
                    .offsetx
                    .3
                    .write(&mut *i2c, OFFSETx::offset.val(0x0000));
                ldc.register
                    .settlecountx
                    .3
                    .write(&mut *i2c, SETTLECOUNTx::settlecount.val(0x000a));
                ldc.register.clock_dividersx.3.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                ldc.register.drive_currentx.3.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
        }
        ldc.register.error_config.write(
            &mut *i2c,
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
        ldc.register.mux_config.write(
            &mut *i2c,
            MUX_CONFIG::input_deglitch_filter_bandwidth::with_3MHz3
                + match self.channel {
                    Channel::Zero => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                    Channel::One => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                    Channel::Two => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2,
                    Channel::Three => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2_3,
                }
                + MUX_CONFIG::auto_scan_mode::manual,
        );
        ldc.register.config.write(
            &mut *i2c,
            CONFIG::high_current_sensor_drive::normal
                + CONFIG::INTB_asserted::enable
                + CONFIG::select_reference_frequency_source::internal_oscillator
                + CONFIG::automatic_sensor_amplitude_correction::disable
                + CONFIG::sensor_activation_mode::low
                + CONFIG::sensor_rp_override::off
                + CONFIG::sleep_mode::active
                + match self.channel {
                    Channel::Zero => CONFIG::active_channel::channel0,
                    Channel::One => CONFIG::active_channel::channel1,
                    Channel::Two => CONFIG::active_channel::channel2,
                    Channel::Three => CONFIG::active_channel::channel3,
                },
        );
        unimplemented!()
    }
}
