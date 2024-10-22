// #[inline(always)]
// const fn sqrt(n: f32) -> f32 {
//     // https://bits.stephan-brumme.com/squareRoot.html / no negative check because whatever
//     f32::from_bits((n.to_bits() + 0x3f80_0000) >> 1)
// }

#[derive(Debug, Clone, Copy)]
pub struct ClockDividers {
    pub fin_div: u16,
    pub fref_div: u16,
}

impl ClockDividers {
    pub const fn with_fref_div(self, div: u8) -> Self {
        Self { fin_div: self.fin_div, fref_div: div as u16 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fsensor(pub f32);

impl Fsensor {
    /// Calculate sensor frequency based on inductance in μH and capacitance in pF.
    pub const fn from_inductance_capacitance(inductance: f32, capacitance: f32) -> Self {
        Self(
            1.0 / (2.0
                * 3.14
                * (inductance * 1e-6_f32 * capacitance * 1e-12_f32)
                * (inductance * 1e-6_f32 * capacitance * 1e-12_f32))
                * 1e-6_f32,
        )
    }

    /// Calculate minimum clock dividers based on Fsensor and Fref (oscillator frequency in MHz).
    ///
    /// If using the internal oscillator, you can pass None, it will default to 43 MHz.
    pub const fn to_clock_dividers(&self, ext_clk_freq: Option<f32>) -> ClockDividers {
        // unwrap_or is not const fn?!
        let fref = match ext_clk_freq {
            None => 43.0, // internal oscillator
            Some(x) => x,
        };
        ClockDividers {
            fin_div: (self.0 / 8.75 + 1.0) as u16,
            fref_div: if self.0 * 4.0 < fref {
                1
            } else if self.0 / 2.0 * 4.0 < fref {
                2
            } else {
                4
            },
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    Zero = 0,
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy)]
pub struct Status(pub u16);

impl Status {
    #[inline(always)]
    pub const fn error_channel(&self) -> Channel {
        match self.0 >> 14 {
            0 => Channel::Zero,
            1 => Channel::One,
            2 => Channel::Two,
            3 => Channel::Three,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn conversion_under_range_error(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    #[inline(always)]
    pub const fn conversion_over_range_error(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    #[inline(always)]
    pub const fn watchdog_timeout_error(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    #[inline(always)]
    pub const fn sensor_amplitude_high_error(&self) -> bool {
        self.0 & (1 << 10) != 0
    }

    #[inline(always)]
    pub const fn sensor_amplitude_low_error(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    #[inline(always)]
    pub const fn zero_count_error(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    #[inline(always)]
    pub const fn data_ready(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    #[inline(always)]
    pub const fn channel_0_unread(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    #[inline(always)]
    pub const fn channel_1_unread(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    #[inline(always)]
    pub const fn channel_2_unread(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    #[inline(always)]
    pub const fn channel_3_unread(&self) -> bool {
        self.0 & 1 != 0
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ErrorConfig(pub u16);

impl ErrorConfig {
    #[inline(always)]
    pub const fn under_range_error_to_data_register(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    #[inline(always)]
    pub const fn with_under_range_error_to_data_register(self, val: bool) -> Self {
        Self(self.0 & !(1 << 15) | (val as u16) << 15)
    }

    #[inline(always)]
    pub const fn over_range_error_to_data_register(&self) -> bool {
        self.0 & (1 << 14) != 0
    }

    #[inline(always)]
    pub const fn with_over_range_error_to_data_register(self, val: bool) -> Self {
        Self(self.0 & !(1 << 14) | (val as u16) << 14)
    }

    #[inline(always)]
    pub const fn watchdog_timeout_error_to_data_register(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    #[inline(always)]
    pub const fn with_watchdog_timeout_error_to_data_register(self, val: bool) -> Self {
        Self(self.0 & !(1 << 13) | (val as u16) << 13)
    }

    #[inline(always)]
    pub const fn amplitude_high_error_to_data_register(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    #[inline(always)]
    pub const fn with_amplitude_high_error_to_data_register(self, val: bool) -> Self {
        Self(self.0 & !(1 << 12) | (val as u16) << 12)
    }

    #[inline(always)]
    pub const fn amplitude_low_error_to_data_register(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    #[inline(always)]
    pub const fn with_amplitude_low_error_to_data_register(self, val: bool) -> Self {
        Self(self.0 & !(1 << 11) | (val as u16) << 11)
    }

    #[inline(always)]
    pub const fn under_range_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    #[inline(always)]
    pub const fn with_under_range_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 7) | (val as u16) << 7)
    }

    #[inline(always)]
    pub const fn over_range_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    #[inline(always)]
    pub const fn with_over_range_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 6) | (val as u16) << 6)
    }

    #[inline(always)]
    pub const fn watchdog_timeout_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    #[inline(always)]
    pub const fn with_watchdog_timeout_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 5) | (val as u16) << 5)
    }

    #[inline(always)]
    pub const fn amplitude_high_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    #[inline(always)]
    pub const fn with_amplitude_high_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 4) | (val as u16) << 4)
    }

    #[inline(always)]
    pub const fn amplitude_low_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    #[inline(always)]
    pub const fn with_amplitude_low_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 3) | (val as u16) << 3)
    }

    #[inline(always)]
    pub const fn zero_count_error_to_interrupt(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    #[inline(always)]
    pub const fn with_zero_count_error_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !(1 << 2) | (val as u16) << 2)
    }

    #[inline(always)]
    pub const fn data_ready_to_interrupt(&self) -> bool {
        self.0 & 1 != 0
    }

    #[inline(always)]
    pub const fn with_data_ready_to_interrupt(self, val: bool) -> Self {
        Self(self.0 & !1 | val as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config(pub u16);

impl Default for Config {
    fn default() -> Self {
        Self(0x1001 /* ch0, only Rp override, reserved first bit 1 */)
    }
}

impl Config {
    #[inline(always)]
    pub const fn active_chan(&self) -> Channel {
        match self.0 >> 14 {
            0 => Channel::Zero,
            1 => Channel::One,
            2 => Channel::Two,
            3 => Channel::Three,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn with_active_chan(self, ch: Channel) -> Self {
        Self(self.0 & !(0b11 << 14) | (ch as u16) << 14)
    }

    #[inline(always)]
    pub const fn sleep_mode(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    #[inline(always)]
    pub const fn with_sleep_mode(self, val: bool) -> Self {
        Self(self.0 & !(1 << 13) | (val as u16) << 13)
    }

    #[inline(always)]
    pub const fn sensor_rp_override(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    #[inline(always)]
    pub const fn with_sensor_rp_override(self, val: bool) -> Self {
        Self(self.0 & !(1 << 12) | (val as u16) << 12)
    }

    #[inline(always)]
    pub const fn sensor_activation_low_power(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    #[inline(always)]
    pub const fn with_sensor_activation_low_power(self, val: bool) -> Self {
        Self(self.0 & !(1 << 11) | (val as u16) << 11)
    }

    #[inline(always)]
    pub const fn automatic_sensor_amplitude_correction(&self) -> bool {
        // NOTE: field is inverted ("disable")
        self.0 & (1 << 10) == 0
    }

    #[inline(always)]
    pub const fn with_automatic_sensor_amplitude_correction(self, val: bool) -> Self {
        // NOTE: field is inverted ("disable")
        Self(self.0 & !(1 << 10) | (!val as u16) << 10)
    }

    #[inline(always)]
    pub const fn reference_clock_external(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    #[inline(always)]
    pub const fn with_reference_clock_external(self, val: bool) -> Self {
        Self(self.0 & !(1 << 9) | (val as u16) << 9)
    }

    #[inline(always)]
    pub const fn interrupt_on_status_update(&self) -> bool {
        // NOTE: field is inverted ("disable")
        self.0 & (1 << 7) == 0
    }

    #[inline(always)]
    pub const fn with_interrupt_on_status_update(self, val: bool) -> Self {
        // NOTE: field is inverted ("disable")
        Self(self.0 & !(1 << 7) | (!val as u16) << 7)
    }

    #[inline(always)]
    pub const fn high_current_sensor_drive(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    #[inline(always)]
    pub const fn with_high_current_sensor_drive(self, val: bool) -> Self {
        Self(self.0 & !(1 << 6) | (val as u16) << 6)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AutoScanSequence {
    ZeroOne = 0,
    ZeroOneTwo,
    ZeroOneTwoThree,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Deglitch {
    OneMHz = 0b001,
    ThreePointThreeMHz = 0b100,
    TenMHz = 0b101,
    ThirtyThreeMHz = 0b111,
}

#[derive(Debug, Clone, Copy)]
pub struct MuxConfig(pub u16);

impl Default for MuxConfig {
    fn default() -> Self {
        Self(0b0100_0001_111 /* reserved + default 33 MHz deglitch */)
    }
}

impl MuxConfig {
    #[inline(always)]
    pub const fn auto_scan(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    #[inline(always)]
    pub const fn with_auto_scan(self, val: bool) -> Self {
        Self(self.0 & !(1 << 15) | (val as u16) << 15)
    }

    #[inline(always)]
    pub const fn auto_scan_sequence(&self) -> AutoScanSequence {
        match (self.0 >> 13) & 0b11 {
            0 | 3 => AutoScanSequence::ZeroOne,
            1 => AutoScanSequence::ZeroOneTwo,
            2 => AutoScanSequence::ZeroOneTwoThree,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn with_auto_scan_sequence(self, ch: AutoScanSequence) -> Self {
        Self(self.0 & !(0b11 << 13) | (ch as u16) << 13)
    }

    #[inline(always)]
    pub const fn deglitch_filter_bandwidth(&self) -> Result<Deglitch, u8> {
        match self.0 & 0b111 {
            0b001 => Ok(Deglitch::OneMHz),
            0b100 => Ok(Deglitch::ThreePointThreeMHz),
            0b101 => Ok(Deglitch::TenMHz),
            0b111 => Ok(Deglitch::ThirtyThreeMHz),
            x => Err(x as u8),
        }
    }

    #[inline(always)]
    pub const fn with_deglitch_filter_bandwidth(self, bw: Deglitch) -> Self {
        Self(self.0 & !0b111 | bw as u16)
    }
}
