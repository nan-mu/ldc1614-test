use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    DataRegisters {
        (0x00 => data0_msb: ReadOnly<u8,DATA_MSB::Register>),
        (0x01 => data0_lsb: ReadOnly<u8,DATA_LSB::Register>),
        (0x02 => data1_msb: ReadOnly<u8,DATA_MSB::Register>),
        (0x03 => data1_lsb: ReadOnly<u8,DATA_LSB::Register>),
        (0x04 => data2_msb: ReadOnly<u8,DATA_MSB::Register>),
        (0x05 => data2_lsb: ReadOnly<u8,DATA_LSB::Register>),
        (0x06 => data3_msb: ReadOnly<u8,DATA_MSB::Register>),
        (0x07 => data3_lsb: ReadOnly<u8,DATA_LSB::Register>),
        (0x08 => @END),
    }
}
register_structs! {
    ChannelRegisters {
        (0x00 => _reserved),
        (0x08 => rcountx: [ReadWrite<u8,RCOUNTx::Register>;4]),
        (0x0c => offsetx: [ReadWrite<u8,OFFSETx::Register>;4]),
        (0x10 => settlecountx: [ReadWrite<u8,SETTLECOUNTx::Register>;4]),
        (0x14 => clock_dividersx: [ReadWrite<u8,CLOCK_DIVIDERSx::Register>;4]),
        (0x18 => @END),
    }
}
register_structs! {
    ConfigRegisters {
        (0x00 => _reserved),
        (0x18 => status: ReadOnly<u8,STATUS::Register>),
        (0x19 => error_config: ReadWrite<u8,ERROR_CONFIG::Register>),
        (0x1a => config: ReadWrite<u8,CONFIG::Register>),
        (0x1b => mux_config: ReadWrite<u8,MUX_CONFIG::Register>),
        (0x1c => reset_dev: ReadWrite<u8,RESET_DEV::Register>),
        (0x1d => @END),
    }
}
register_structs! {
    DriveCurrentRegisters {
        (0x00 => _reserved),
        (0x1e => drive_currentx: [ReadWrite<u8,DRIVE_CURRENTx::Register>;4]),
        (0x22 => @END),
    }
}

// register_structs! {
//  InfoRegisters {// 这两个应该没用，不写了
//      (0x7E => manufacturer_id: ReadOnly<u16,MANUFACTURER_ID::Register>),
//      (0x7F => device_id: ReadOnly<u16,DEVICE_ID::Register>),
//  }
// }

register_bitfields![
    u16,
    DATA_MSB [
        data OFFSET(0) NUMBITS(12),
        err_ae OFFSET(0) NUMBITS(1),
        err_wd OFFSET(0) NUMBITS(1),
        err_or OFFSET(0) NUMBITS(1),
        err_ur OFFSET(0) NUMBITS(1),
    ],
    DATA_LSB [
        data OFFSET(0) NUMBITS(16),
    ],
    RCOUNTx [
        rcount OFFSET(0) NUMBITS(16),
    ],
    OFFSETx [
        offset OFFSET(0) NUMBITS(16),
    ],
    SETTLECOUNTx [
        settlecount OFFSET(0) NUMBITS(16),
    ],
    CLOCK_DIVIDERSx [
        fref_divider OFFSET(0) NUMBITS(10),
        reserved OFFSET(0) NUMBITS(2),
        fin_divider OFFSET(0) NUMBITS(4),
    ],
    STATUS [
        unread_conv0 OFFSET(0) NUMBITS(1) [
            unread_conversion = 1,
            fine = 0
        ],
        unread_conv1 OFFSET(0) NUMBITS(1) [
            unread_conversion = 1,
            fine = 0
        ],
        unread_conv2 OFFSET(0) NUMBITS(1) [
            unread_conversion = 1,
            fine = 0
        ],
        unread_conv3 OFFSET(0) NUMBITS(1) [
            unread_conversion = 1,
            fine = 0
        ],
        data_ready OFFSET(2) NUMBITS(1) [
            no_ready = 0,
            ready = 1
        ],
        zero_count_error OFFSET(1) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        sensor_activation_low_error OFFSET(0) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        sensor_amplitude_high_error OFFSET(0) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        watchdog_timeout_error OFFSET(0) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        conversion_over_range_error OFFSET(0) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        conversion_under_range_error OFFSET(0) NUMBITS(1) [
            no_error = 0,
            error = 1
        ],
        error_channel OFFSET(0) NUMBITS(2) [
            channel0 = 0,
            channel1 = 1,
            channel2 = 2,
            channel3 = 3,
        ],
    ],
    ERROR_CONFIG [
        data_ready_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        zero_count_error_to_INTB OFFSET(1) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        amplitude_low_error_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        amplitude_high_error_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        watchdog_timeout_error_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        over_range_error_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        under_range_error_to_INTB OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        amplitude_low_error_to_output_register OFFSET(2) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        amplitude_high_error_to_output_register OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        watchdog_timeout_error_to_output_register OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        over_range_error_to_output_register OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
        under_range_error_to_output_register OFFSET(0) NUMBITS(1) [
            no_report = 0,
            report = 1
        ],
    ],
    CONFIG [
        high_current_sensor_drive OFFSET(6) NUMBITS(1) [
            normal = 0,
            high = 1,
        ],
        INTB_asserted OFFSET(0) NUMBITS(1) [
            enable = 0,
            disable = 1
        ],
        select_reference_frequency_source OFFSET(1) NUMBITS(1) [
            internal_oscillator = 0,
            from_CLKIN = 1
        ],
        automatic_sensor_amplitude_correction OFFSET(0) NUMBITS(1) [
            enable = 0,
            disable = 1
        ],
        sensor_activation_mode OFFSET(0) NUMBITS(1) [
            full = 0,
            low = 1
        ],
        sensor_rp_override OFFSET(0) NUMBITS(1) [
            off = 0,
            on = 1
        ],
        sleep_mode OFFSET(0) NUMBITS(1) [
            active = 0,
            sleep = 1
        ],
        active_channel OFFSET(0) NUMBITS(2) [
            channel0 = 0,
            channel1 = 1,
            channel2 = 2,
            channel3 = 3
        ]
    ],
    MUX_CONFIG [
        input_deglitch_filter_bandwidth OFFSET(0) NUMBITS(3) [
            with_1MHz = 0b001,
            with_3MHz3 = 0b100,
            with_10MHz = 0b101,
            with_33MHz = 0b111,
        ],
        auto_scan_sequence_config OFFSET(10) NUMBITS(2) [
            channel_0_1 = 0b00,
            channel_0_1_2 = 0b01,
            channel_0_1_2_3 = 0b10,
            // channel_0_1 = 0b11 这个抽象规格书，神tm相同配置映射两个值，不管了！
        ],
        auto_scan_mode OFFSET(0) NUMBITS(1) [
            manual = 0,
            auto = 1,
        ],
    ],
    RESET_DEV [
        device_reset OFFSET(15) NUMBITS(1) [
            reset = 1
        ]
    ],
    DRIVE_CURRENTx [
        sensor_current_drive OFFSET(6) NUMBITS(5) [],
        LC_sensor_drive_current OFFSET(0) NUMBITS(5) [],
    ],
];

use core::marker;
use embedded_hal::i2c;
use std::sync;
use tock_registers::interfaces::{Readable, Writeable};

struct ReadOnlyI2c<
    I2C,
    SlaveAddr,
    const LEN: usize,
    const REG_ADDR: u8,
    R: tock_registers::RegisterLongName = (),
> where
    I2C: i2c::I2c<SlaveAddr>,
    SlaveAddr: i2c::AddressMode + Copy,
{
    inner: sync::RwLock<I2C>,
    slave_address: SlaveAddr,
    register_address: u8,
    associated_register: marker::PhantomData<R>,
}

impl<I2C, SlaveAddr, const LEN: usize, const REG_ADDR: u8, R> Readable
    for ReadOnlyI2c<I2C, SlaveAddr, LEN, REG_ADDR, R>
where
    I2C: i2c::I2c<SlaveAddr>,
    R: tock_registers::RegisterLongName,
    SlaveAddr: i2c::AddressMode + Copy,
{
    type T = usize;
    type R = R;
    fn get(&self) -> Self::T {
        // let reg_address = self._register_long_name.
        let mut result: [u8; LEN] = [0; LEN];
        self.inner
            .write()
            .unwrap()
            .write_read(self.slave_address, &[self.register_address], &mut result)
            .unwrap();
        assert!(LEN == 1 || LEN == 2 || LEN == 4 || LEN == 8, "LEN must be 1, 2, 4, or 8");
        match LEN {
            1 => result[0] as usize,
            2 => (result[0] as usize) << 8 | result[1] as usize,
            4 => {
                (result[0] as usize) << 24
                    | (result[1] as usize) << 16
                    | (result[2] as usize) << 8
                    | result[3] as usize
            },
            8 => {
                (result[0] as usize) << 56
                    | (result[1] as usize) << 48
                    | (result[2] as usize) << 40
                    | (result[3] as usize) << 32
                    | (result[4] as usize) << 24
                    | (result[5] as usize) << 16
                    | (result[6] as usize) << 8
                    | result[7] as usize
            },
            _ => unreachable!(),
        }
    }
}
