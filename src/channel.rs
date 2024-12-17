//! 包含从ldc1614::Ldc得到对应通道代码的结构。最后的形式是从一个tokio的广播结构发送数据

use super::{Error, Result};
use std::collections::HashMap;
use tokio::sync::{self, broadcast};

pub struct Channel<const ADDR: u8> {
    ldc: std::sync::Arc<sync::Mutex<ldc1614::Ldc<ADDR>>>,
    i2c: std::sync::Arc<sync::Mutex<rppal::i2c::I2c>>,
    channel: ldc1614::Channel,
    rx: broadcast::Sender<super::Record>,
}

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

    pub async fn submit(
        &self,
        mark: Option<std::sync::Arc<str>>,
        record_type: crate::RecordType,
        settlecount: Option<u16>,
        rcount: Option<u16>,
        
    ) -> Result<usize, crate::Error> {
        use chrono::Local;
        let (ldc, mut i2c) = tokio::join!(self.ldc.lock(), self.i2c.lock());
        let data = ldc.read_data(&mut *i2c, self.channel).map_err(|e| crate::Error::Ldc1614(e))?;
    
        // 根据传入的 record_type 创建对应的 RecordA 或 RecordB
        let record = match record_type {
            crate::RecordType::A => crate::Record::new_a(data, self.channel, mark),
            crate::RecordType::B => crate::Record::new_b(
                data,
                self.channel,
                mark,
                settlecount.unwrap_or(0), // 提供默认值
                rcount.unwrap_or(0),      // 提供默认值
            ),
        };
    
        // 发送消息
        Ok(self.rx.send(record)?)
    }

    pub async fn apply_reg_config(&mut self, register: &HashMap<String, u16>) -> Result<()> {
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

        ldc.register
            .config
            .write(&mut *i2c, CONFIG::sleep_mode::sleep);

        match self.channel {
            Channel::Zero => {
                ldc.register.rcountx.0.write(
                    &mut *i2c,
                    RCOUNTx::rcount.val(match register.get("RCOUNT") {
                        Some(&value) => value,
                        None => 0x04d6,
                    }),
                );
                ldc.register.offsetx.0.write(
                    &mut *i2c,
                    OFFSETx::offset.val(match register.get("OFFSET") {
                        Some(&value) => value,
                        None => 0x0000,
                    }),
                );
                ldc.register.settlecountx.0.write(
                    &mut *i2c,
                    SETTLECOUNTx::settlecount.val(match register.get("SETTLECOUNT") {
                        Some(&value) => value,
                        None => 0x000a,
                    }),
                );

                ldc.register.clock_dividersx.0.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(match register.get("FREF_DIVIDERS") {
                        Some(&value) => value,
                        None => 2,
                    }) + CLOCK_DIVIDERSx::fin_divider.val(match register.get("FIN_DIVIDERS") {
                        Some(&value) => value,
                        None => 1,
                    }),
                );
                //0 0b10010
                ldc.register.drive_currentx.0.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::LC_sensor_drive_current.val(
                        match register.get("LC_SENSOR_DRIVE_CURRENT") {
                            Some(&value) => value,
                            None => 0b10010,
                        },
                    ),
                );
            }
            Channel::One => {
                ldc.register.rcountx.1.write(
                    &mut *i2c,
                    RCOUNTx::rcount.val(match register.get("RCOUNT") {
                        Some(&value) => value,
                        None => 0x04d6,
                    }),
                );
                ldc.register.offsetx.1.write(
                    &mut *i2c,
                    OFFSETx::offset.val(match register.get("OFFSET") {
                        Some(&value) => value,
                        None => 0x0000,
                    }),
                );
                ldc.register.settlecountx.1.write(
                    &mut *i2c,
                    SETTLECOUNTx::settlecount.val(match register.get("SETTLECOUNT") {
                        Some(&value) => value,
                        None => 0x000a,
                    }),
                );

                ldc.register.clock_dividersx.1.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(match register.get("FREF_DIVIDERS") {
                        Some(&value) => value,
                        None => 2,
                    }) + CLOCK_DIVIDERSx::fin_divider.val(match register.get("FIN_DIVIDERS") {
                        Some(&value) => value,
                        None => 1,
                    }),
                );
                //0 0b10010
                ldc.register.drive_currentx.1.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::LC_sensor_drive_current.val(
                        match register.get("LC_SENSOR_DRIVE_CURRENT") {
                            Some(&value) => value,
                            None => 0b10010,
                        },
                    ),
                );
            }
            Channel::Two => {
                ldc.register.rcountx.2.write(
                    &mut *i2c,
                    RCOUNTx::rcount.val(match register.get("RCOUNT") {
                        Some(&value) => value,
                        None => 0x04d6,
                    }),
                );
                ldc.register.offsetx.2.write(
                    &mut *i2c,
                    OFFSETx::offset.val(match register.get("OFFSET") {
                        Some(&value) => value,
                        None => 0x0000,
                    }),
                );
                ldc.register.settlecountx.2.write(
                    &mut *i2c,
                    SETTLECOUNTx::settlecount.val(match register.get("SETTLECOUNT") {
                        Some(&value) => value,
                        None => 0x000a,
                    }),
                );

                ldc.register.clock_dividersx.2.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(match register.get("FREF_DIVIDERS") {
                        Some(&value) => value,
                        None => 2,
                    }) + CLOCK_DIVIDERSx::fin_divider.val(match register.get("FIN_DIVIDERS") {
                        Some(&value) => value,
                        None => 1,
                    }),
                );
                //0 0b10010
                ldc.register.drive_currentx.2.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::LC_sensor_drive_current.val(
                        match register.get("LC_sensor_drive_current") {
                            Some(&value) => value,
                            None => 0b10010,
                        },
                    ),
                );
            }
            Channel::Three => {
                ldc.register.rcountx.3.write(
                    &mut *i2c,
                    RCOUNTx::rcount.val(match register.get("RCOUNT") {
                        Some(&value) => value,
                        None => 0x04d6,
                    }),
                );
                ldc.register.offsetx.3.write(
                    &mut *i2c,
                    OFFSETx::offset.val(match register.get("OFFSET") {
                        Some(&value) => value,
                        None => 0x0000,
                    }),
                );
                ldc.register.settlecountx.3.write(
                    &mut *i2c,
                    SETTLECOUNTx::settlecount.val(match register.get("SETTLECOUNT") {
                        Some(&value) => value,
                        None => 0x000a,
                    }),
                );

                ldc.register.clock_dividersx.3.write(
                    &mut *i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(match register.get("FREF_DIVIDERS") {
                        Some(&value) => value,
                        None => 2,
                    }) + CLOCK_DIVIDERSx::fin_divider.val(match register.get("FIN_DIVIDERS") {
                        Some(&value) => value,
                        None => 1,
                    }),
                );
                //0 0b10010
                ldc.register.drive_currentx.3.write(
                    &mut *i2c,
                    DRIVE_CURRENTx::LC_sensor_drive_current.val(
                        match register.get("LC_SENSOR_DRIVE_CURRENT") {
                            Some(&value) => value,
                            None => 0b10010,
                        },
                    ),
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
            MUX_CONFIG::input_deglitch_filter_bandwidth.val(match register.get("DEGLITCH") {
                Some(&value) => value,
                None => 0b001,
            }) + match self.channel {
                Channel::Zero => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                Channel::One => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                Channel::Two => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2,
                Channel::Three => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2_3,
            } + MUX_CONFIG::auto_scan_mode::manual,
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
        Ok(())
    }
}
