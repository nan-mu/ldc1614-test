//! [`embedded-hal`] driver for Texas Instruments (TI) I2C inductance-to-digital converters (LDC): [LDC1312/LDC1314], [LDC1612/LDC1614].
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//! [LDC1312/LDC1314]: https://www.ti.com/lit/ds/symlink/ldc1314.pdf
//! [LDC1612/LDC1614]: https://www.ti.com/lit/ds/symlink/ldc1614.pdf
// #![no_std]
// #![feature(const_float_bits_conv, const_fn_floating_point_arithmetic)]
// use embedded_hal::i2c::blocking as i2c;

pub mod bitmap;
mod interface;
use bitmap::LdcRegister;
use embedded_hal::i2c;

#[derive(Debug)]
pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub struct Ldc<const ADDR: u8> {
    pub register: bitmap::LdcRegister,
}

impl<const ADDR: u8> Ldc<ADDR> {
    /// 生成ldc1614控制对象并进行一次初始化；
    /// 建议等待10ms后在对设备进行操作。
    pub fn new<I2C: i2c::I2c>(i2c: &mut I2C) -> Self {
        use bitmap::RESET_DEV;
        let ldc = Ldc::<ADDR> {
            register: LdcRegister::new(ADDR),
        };
        ldc.register
            .reset_dev
            .write(i2c, RESET_DEV::device_reset::reset);
        ldc
    }
    pub fn read_data<I2C: i2c::I2c>(&self, i2c: &mut I2C, ch: Channel) -> Result<u32> {
        use bitmap::{DATA_LSB, DATA_MSB};
        Ok((match ch {
            Channel::Zero => self.register.data0_msb.read(i2c, DATA_MSB::data),
            Channel::One => self.register.data1_msb.read(i2c, DATA_MSB::data),
            Channel::Two => self.register.data2_msb.read(i2c, DATA_MSB::data),
            Channel::Three => self.register.data3_msb.read(i2c, DATA_MSB::data),
        } << 8
            | match ch {
                Channel::Zero => self.register.data0_lsb.read(i2c, DATA_LSB::data),
                Channel::One => self.register.data1_lsb.read(i2c, DATA_LSB::data),
                Channel::Two => self.register.data2_lsb.read(i2c, DATA_LSB::data),
                Channel::Three => self.register.data3_lsb.read(i2c, DATA_LSB::data),
            }) as u32)
    }
    pub fn defaule_config<I2C: i2c::I2c>(&self, i2c: &mut I2C, ch: Channel) -> Result<()> {
        use bitmap::{
            CLOCK_DIVIDERSx, DRIVE_CURRENTx, OFFSETx, RCOUNTx, SETTLECOUNTx, CONFIG, ERROR_CONFIG,
            MUX_CONFIG, RESET_DEV,
        };
        self.register
            .reset_dev
            .write(i2c, RESET_DEV::device_reset::reset);
        match ch {
            Channel::Zero => {
                self.register
                    .rcountx
                    .0
                    .write(i2c, RCOUNTx::rcount.val(0x04d6));
                self.register
                    .offsetx
                    .0
                    .write(i2c, OFFSETx::offset.val(0x0000));
                self.register
                    .settlecountx
                    .0
                    .write(i2c, SETTLECOUNTx::settlecount.val(0x000a));
                self.register.clock_dividersx.0.write(
                    i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                self.register.drive_currentx.0.write(
                    i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::One => {
                self.register
                    .rcountx
                    .1
                    .write(i2c, RCOUNTx::rcount.val(0x04d6));
                self.register
                    .offsetx
                    .1
                    .write(i2c, OFFSETx::offset.val(0x0000));
                self.register
                    .settlecountx
                    .1
                    .write(i2c, SETTLECOUNTx::settlecount.val(0x000a));
                self.register.clock_dividersx.1.write(
                    i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                self.register.drive_currentx.1.write(
                    i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::Two => {
                self.register
                    .rcountx
                    .2
                    .write(i2c, RCOUNTx::rcount.val(0x04d6));
                self.register
                    .offsetx
                    .2
                    .write(i2c, OFFSETx::offset.val(0x0000));
                self.register
                    .settlecountx
                    .2
                    .write(i2c, SETTLECOUNTx::settlecount.val(0x000a));
                self.register.clock_dividersx.2.write(
                    i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                self.register.drive_currentx.2.write(
                    i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
            Channel::Three => {
                self.register
                    .rcountx
                    .3
                    .write(i2c, RCOUNTx::rcount.val(0x04d6));
                self.register
                    .offsetx
                    .3
                    .write(i2c, OFFSETx::offset.val(0x0000));
                self.register
                    .settlecountx
                    .3
                    .write(i2c, SETTLECOUNTx::settlecount.val(0x000a));
                self.register.clock_dividersx.3.write(
                    i2c,
                    CLOCK_DIVIDERSx::fref_divider.val(2) + CLOCK_DIVIDERSx::fin_divider.val(1),
                );
                self.register.drive_currentx.3.write(
                    i2c,
                    DRIVE_CURRENTx::sensor_current_drive.val(0)
                        + DRIVE_CURRENTx::LC_sensor_drive_current.val(0b10010),
                );
            }
        }
        self.register.error_config.write(
            i2c,
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
        self.register.mux_config.write(
            i2c,
            MUX_CONFIG::input_deglitch_filter_bandwidth::with_3MHz3
                + match ch {
                    Channel::Zero => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                    Channel::One => MUX_CONFIG::auto_scan_sequence_config::channel_0_1,
                    Channel::Two => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2,
                    Channel::Three => MUX_CONFIG::auto_scan_sequence_config::channel_0_1_2_3,
                }
                + MUX_CONFIG::auto_scan_mode::manual,
        );
        self.register.config.write(
            i2c,
            CONFIG::high_current_sensor_drive::normal
                + CONFIG::INTB_asserted::enable
                + CONFIG::select_reference_frequency_source::internal_oscillator
                + CONFIG::automatic_sensor_amplitude_correction::disable
                + CONFIG::sensor_activation_mode::low
                + CONFIG::sensor_rp_override::off
                + CONFIG::sleep_mode::active
                + match ch {
                    Channel::Zero => CONFIG::active_channel::channel0,
                    Channel::One => CONFIG::active_channel::channel1,
                    Channel::Two => CONFIG::active_channel::channel2,
                    Channel::Three => CONFIG::active_channel::channel3,
                },
        );
        Ok(())
    }
}

pub struct Config {}

#[derive(Debug, Clone, Copy)]
pub enum Channel {
    Zero,
    One,
    Two,
    Three,
}

impl Channel {}

// mod auto_set {
//     //! 自动配置驱动电流相关函数
//     use super::{Channel, Ldc};
//     use embedded_hal::i2c;

//     impl<I2c, BE> Ldc<I2c>
//     where
//         I2c: i2c::I2c<Error = BE>,
//     {
//         fn auto_set_drive_current(&mut self, ch: &Channel) {
//             // TODO: 等有日志的时候在这里加上一个WARN说要在物理世界中将待测物体放置在理论最远处

//             // 1. 创建默认配置并将设备置于 SLEEP 模式

//             // 2. 为通道编写所需的 SETTLECOUNT 和 RCOUNT 值
//             // ldc.set_conv_settling_time(channel, 40).unwrap();
//             // ldc.set_ref_count_conv_interval(channel, 0x0546).unwrap();

//             // // 3. 设置自动校准
//             // let new_config_auto_cal = config
//             //     .with_active_chan(channel)
//             //     .with_sleep_mode(true)
//             //     .with_automatic_sensor_amplitude_correction(false);

//             // // 应用新的配置
//             // ldc.set_config(new_config_auto_cal).unwrap();

//             // // 4. 使设备退出 SLEEP 模式
//             // let new_config_wakeup = config.with_active_chan(channel).with_sleep_mode(false);

//             // // 应用配置以退出 SLEEP 模式
//             // ldc.set_config(new_config_wakeup).unwrap();

//             // // 5. 允许设备至少执行一次测量

//             // // 6. 读取 DRIVE_CURRENTx 寄存器中的 INIT_DRIVEx 字段
//             // let drive_current = ldc.measured_sensor_drive_current(channel).unwrap();
//             // let init_drive_value = (drive_current >> 6) & 0x1F; // 取出位 10:6

//             // // 7. 将保存的值写入 IDRIVEx 位字段
//             // ldc.set_sensor_drive_current(channel, init_drive_value)
//             //     .unwrap();

//             // // 8. 设置固定电流驱动的 RP_OVERRIDE_EN 为 b1
//             // let new_config_fixed_current =
//             //     config.with_active_chan(channel).with_rp_override_en(true);

//             // // 应用新的配置
//             // ldc.set_config(new_config_fixed_current).unwrap();
//         }
//     }
// }
