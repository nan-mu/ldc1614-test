use crate::interface::{self, ReadOnlyI2cRegister, ReadWriteI2cRegister};
use embedded_hal::i2c;

type ReadOnlyLdc<Register, const ADDR: u16> =
    interface::ReadOnlyI2cRegister<i2c::SevenBitAddress, Register, ADDR, 2>;
type ReadWriteLdc<Register, const ADDR: u16> =
    interface::ReadWriteI2cRegister<i2c::SevenBitAddress, Register, ADDR, 2>;

pub struct LdcRegister {
    pub data0_msb: ReadOnlyLdc<DATA_MSB::Register, 0x00>,
    pub data0_lsb: ReadOnlyLdc<DATA_LSB::Register, 0x01>,
    pub data1_msb: ReadOnlyLdc<DATA_MSB::Register, 0x02>,
    pub data1_lsb: ReadOnlyLdc<DATA_LSB::Register, 0x03>,
    pub data2_msb: ReadOnlyLdc<DATA_MSB::Register, 0x04>,
    pub data2_lsb: ReadOnlyLdc<DATA_LSB::Register, 0x05>,
    pub data3_msb: ReadOnlyLdc<DATA_MSB::Register, 0x06>,
    pub data3_lsb: ReadOnlyLdc<DATA_LSB::Register, 0x07>,
    pub rcountx: (
        ReadWriteLdc<RCOUNTx::Register, 0x08>,
        ReadWriteLdc<RCOUNTx::Register, 0x09>,
        ReadWriteLdc<RCOUNTx::Register, 0x0a>,
        ReadWriteLdc<RCOUNTx::Register, 0x0b>,
    ),
    pub offsetx: (
        ReadWriteLdc<OFFSETx::Register, 0x0c>,
        ReadWriteLdc<OFFSETx::Register, 0x0d>,
        ReadWriteLdc<OFFSETx::Register, 0x0e>,
        ReadWriteLdc<OFFSETx::Register, 0x0f>,
    ),
    pub settlecountx: (
        ReadWriteLdc<SETTLECOUNTx::Register, 0x10>,
        ReadWriteLdc<SETTLECOUNTx::Register, 0x11>,
        ReadWriteLdc<SETTLECOUNTx::Register, 0x12>,
        ReadWriteLdc<SETTLECOUNTx::Register, 0x13>,
    ),
    pub clock_dividersx: (
        ReadWriteLdc<CLOCK_DIVIDERSx::Register, 0x14>,
        ReadWriteLdc<CLOCK_DIVIDERSx::Register, 0x15>,
        ReadWriteLdc<CLOCK_DIVIDERSx::Register, 0x16>,
        ReadWriteLdc<CLOCK_DIVIDERSx::Register, 0x17>,
    ),
    pub status: ReadOnlyLdc<STATUS::Register, 0x18>,
    pub error_config: ReadWriteLdc<ERROR_CONFIG::Register, 0x19>,
    pub config: ReadWriteLdc<CONFIG::Register, 0x1a>,
    pub mux_config: ReadWriteLdc<MUX_CONFIG::Register, 0x1b>,
    pub reset_dev: ReadWriteLdc<RESET_DEV::Register, 0x1c>,
    pub drive_currentx: (
        ReadWriteLdc<DRIVE_CURRENTx::Register, 0x1e>,
        ReadWriteLdc<DRIVE_CURRENTx::Register, 0x1e>,
        ReadWriteLdc<DRIVE_CURRENTx::Register, 0x1e>,
        ReadWriteLdc<DRIVE_CURRENTx::Register, 0x1e>,
    ),
    pub manufcturer_id: ReadOnlyLdc<MANUFCTURER_ID::Register, 0x7e>,
}

impl LdcRegister {
    pub(crate) fn new(addr: u8) -> Self {
        LdcRegister {
            data0_msb: ReadOnlyI2cRegister::new(addr),
            data0_lsb: ReadOnlyI2cRegister::new(addr),
            data1_msb: ReadOnlyI2cRegister::new(addr),
            data1_lsb: ReadOnlyI2cRegister::new(addr),
            data2_msb: ReadOnlyI2cRegister::new(addr),
            data2_lsb: ReadOnlyI2cRegister::new(addr),
            data3_msb: ReadOnlyI2cRegister::new(addr),
            data3_lsb: ReadOnlyI2cRegister::new(addr),
            rcountx: (
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
            ),
            offsetx: (
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
            ),
            settlecountx: (
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
            ),
            clock_dividersx: (
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
            ),
            status: ReadOnlyI2cRegister::new(addr),
            error_config: ReadWriteI2cRegister::new(addr),
            config: ReadWriteI2cRegister::new(addr),
            mux_config: ReadWriteI2cRegister::new(addr),
            reset_dev: ReadWriteI2cRegister::new(addr),
            drive_currentx: (
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
                ReadWriteI2cRegister::new(addr),
            ),
            manufcturer_id: ReadOnlyI2cRegister::new(addr),
        }
    }
}

tock_registers::register_bitfields![
    usize,
    pub DATA_MSB [
        data OFFSET(0) NUMBITS(12),
        err_ae OFFSET(0) NUMBITS(1),
        err_wd OFFSET(0) NUMBITS(1),
        err_or OFFSET(0) NUMBITS(1),
        err_ur OFFSET(0) NUMBITS(1),
    ],
    pub DATA_LSB [
        data OFFSET(0) NUMBITS(16),
    ],
    pub RCOUNTx [
        rcount OFFSET(0) NUMBITS(16),
    ],
    pub OFFSETx [
        offset OFFSET(0) NUMBITS(16),
    ],
    pub SETTLECOUNTx [
        settlecount OFFSET(0) NUMBITS(16),
    ],
    pub CLOCK_DIVIDERSx [
        fref_divider OFFSET(0) NUMBITS(10),
        reserved OFFSET(0) NUMBITS(2),
        fin_divider OFFSET(0) NUMBITS(4),
    ],
    pub STATUS [
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
    pub ERROR_CONFIG [
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
    pub CONFIG [
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
    pub MUX_CONFIG [
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
    pub RESET_DEV [
        device_reset OFFSET(15) NUMBITS(1) [
            reset = 1
        ]
    ],
    pub DRIVE_CURRENTx [
        sensor_current_drive OFFSET(6) NUMBITS(5) [],
        LC_sensor_drive_current OFFSET(0) NUMBITS(5) [],
    ],
    pub MANUFCTURER_ID [
        manufcturer_id OFFSET(0) NUMBITS(16) [],
    ],
];

// register_structs! {
//     DataRegisters {
//         (0x00 => data0_msb: ReadOnly<u8,DATA_MSB::Register>),
//         (0x01 => data0_lsb: ReadOnly<u8,DATA_LSB::Register>),
//         (0x02 => data1_msb: ReadOnly<u8,DATA_MSB::Register>),
//         (0x03 => data1_lsb: ReadOnly<u8,DATA_LSB::Register>),
//         (0x04 => data2_msb: ReadOnly<u8,DATA_MSB::Register>),
//         (0x05 => data2_lsb: ReadOnly<u8,DATA_LSB::Register>),
//         (0x06 => data3_msb: ReadOnly<u8,DATA_MSB::Register>),
//         (0x07 => data3_lsb: ReadOnly<u8,DATA_LSB::Register>),
//         (0x08 => @END),
//     }
// }
// register_structs! {
//     ChannelRegisters {
//         (0x00 => _reserved),
//         (0x08 => rcountx: [ReadWrite<u8,RCOUNTx::Register>;4]),
//         (0x0c => offsetx: [ReadWrite<u8,OFFSETx::Register>;4]),
//         (0x10 => settlecountx: [ReadWrite<u8,SETTLECOUNTx::Register>;4]),
//         (0x14 => clock_dividersx: [ReadWrite<u8,CLOCK_DIVIDERSx::Register>;4]),
//         (0x18 => @END),
//     }
// }
// register_structs! {
//     ConfigRegisters {
//         (0x00 => _reserved),
//         (0x18 => status: ReadOnly<u8,STATUS::Register>),
//         (0x19 => error_config: ReadWrite<u8,ERROR_CONFIG::Register>),
//         (0x1a => config: ReadWrite<u8,CONFIG::Register>),
//         (0x1b => mux_config: ReadWrite<u8,MUX_CONFIG::Register>),
//         (0x1c => reset_dev: ReadWrite<u8,RESET_DEV::Register>),
//         (0x1d => @END),
//     }
// }
// register_structs! {
//     DriveCurrentRegisters {
//         (0x00 => _reserved),
//         (0x1e => drive_currentx: [ReadWrite<u8,DRIVE_CURRENTx::Register>;4]),
//         (0x22 => @END),
//     }
// }
