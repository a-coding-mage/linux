/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ADF4350/ADF4351 SPI PLL driver
 *
 * Copyright 2012-2013 Analog Devices Inc.
 */

/* Registers */
pub const ADF4350_REG0: u32 = 0;
pub const ADF4350_REG1: u32 = 1;
pub const ADF4350_REG2: u32 = 2;
pub const ADF4350_REG3: u32 = 3;
pub const ADF4350_REG4: u32 = 4;
pub const ADF4350_REG5: u32 = 5;

/* REG0 Bit Definitions */
macro_rules! ADF4350_REG0_FRACT { ($x:expr) => { (($x & 0xFFF) << 3) }; }
macro_rules! ADF4350_REG0_INT { ($x:expr) => { (($x & 0xFFFF) << 15) }; }

/* REG1 Bit Definitions */
macro_rules! ADF4350_REG1_MOD { ($x:expr) => { (($x & 0xFFF) << 3) }; }
macro_rules! ADF4350_REG1_PHASE { ($x:expr) => { (($x & 0xFFF) << 15) }; }
pub const ADF4350_REG1_PRESCALER: u32 = 1 << 27;

/* REG2 Bit Definitions */
pub const ADF4350_REG2_COUNTER_RESET_EN: u32 = 1 << 3;
pub const ADF4350_REG2_CP_THREESTATE_EN: u32 = 1 << 4;
pub const ADF4350_REG2_POWER_DOWN_EN: u32 = 1 << 5;
pub const ADF4350_REG2_PD_POLARITY_POS: u32 = 1 << 6;
pub const ADF4350_REG2_LDP_6ns: u32 = 1 << 7;
pub const ADF4350_REG2_LDP_10ns: u32 = 0 << 7;
pub const ADF4350_REG2_LDF_FRACT_N: u32 = 0 << 8;
pub const ADF4350_REG2_LDF_INT_N: u32 = 1 << 8;
macro_rules! ADF4350_REG2_CHARGE_PUMP_CURR_uA { ($x:expr) => { (((($x - 312) / 312) & 0xF) << 9) }; }
pub const ADF4350_REG2_DOUBLE_BUFF_EN: u32 = 1 << 13;
macro_rules! ADF4350_REG2_10BIT_R_CNT { ($x:expr) => { ($x << 14) }; }
pub const ADF4350_REG2_RDIV2_EN: u32 = 1 << 24;
pub const ADF4350_REG2_RMULT2_EN: u32 = 1 << 25;
macro_rules! ADF4350_REG2_MUXOUT { ($x:expr) => { ($x << 26) }; }
macro_rules! ADF4350_REG2_NOISE_MODE { ($x:expr) => { (($x as u32) << 29) }; }
pub const ADF4350_MUXOUT_THREESTATE: u32 = 0;
pub const ADF4350_MUXOUT_DVDD: u32 = 1;
pub const ADF4350_MUXOUT_GND: u32 = 2;
pub const ADF4350_MUXOUT_R_DIV_OUT: u32 = 3;
pub const ADF4350_MUXOUT_N_DIV_OUT: u32 = 4;
pub const ADF4350_MUXOUT_ANALOG_LOCK_DETECT: u32 = 5;
pub const ADF4350_MUXOUT_DIGITAL_LOCK_DETECT: u32 = 6;

/* REG3 Bit Definitions */
macro_rules! ADF4350_REG3_12BIT_CLKDIV { ($x:expr) => { ($x << 3) }; }
macro_rules! ADF4350_REG3_12BIT_CLKDIV_MODE { ($x:expr) => { ($x << 15) }; }
pub const ADF4350_REG3_12BIT_CSR_EN: u32 = 1 << 18;
pub const ADF4351_REG3_CHARGE_CANCELLATION_EN: u32 = 1 << 21;
pub const ADF4351_REG3_ANTI_BACKLASH_3ns_EN: u32 = 1 << 22;
pub const ADF4351_REG3_BAND_SEL_CLOCK_MODE_HIGH: u32 = 1 << 23;

/* REG4 Bit Definitions */
macro_rules! ADF4350_REG4_OUTPUT_PWR { ($x:expr) => { ($x << 3) }; }
pub const ADF4350_REG4_RF_OUT_EN: u32 = 1 << 5;
macro_rules! ADF4350_REG4_AUX_OUTPUT_PWR { ($x:expr) => { ($x << 6) }; }
pub const ADF4350_REG4_AUX_OUTPUT_EN: u32 = 1 << 8;
pub const ADF4350_REG4_AUX_OUTPUT_FUND: u32 = 1 << 9;
pub const ADF4350_REG4_AUX_OUTPUT_DIV: u32 = 0 << 9;
pub const ADF4350_REG4_MUTE_TILL_LOCK_EN: u32 = 1 << 10;
pub const ADF4350_REG4_VCO_PWRDOWN_EN: u32 = 1 << 11;
macro_rules! ADF4350_REG4_8BIT_BAND_SEL_CLKDIV { ($x:expr) => { ($x << 12) }; }
macro_rules! ADF4350_REG4_RF_DIV_SEL { ($x:expr) => { ($x << 20) }; }
pub const ADF4350_REG4_FEEDBACK_DIVIDED: u32 = 0 << 23;
pub const ADF4350_REG4_FEEDBACK_FUND: u32 = 1 << 23;

/* REG5 Bit Definitions */
pub const ADF4350_REG5_LD_PIN_MODE_LOW: u32 = 0 << 22;
pub const ADF4350_REG5_LD_PIN_MODE_DIGITAL: u32 = 1 << 22;
pub const ADF4350_REG5_LD_PIN_MODE_HIGH: u32 = 3 << 22;

/* Specifications */
pub const ADF4350_MAX_OUT_FREQ: u64 = 4400000000; /* Hz */
pub const ADF4350_MIN_OUT_FREQ: u32 = 137500000; /* Hz */
pub const ADF4351_MIN_OUT_FREQ: u32 = 34375000; /* Hz */
pub const ADF4350_MIN_VCO_FREQ: u64 = 2200000000; /* Hz */
pub const ADF4350_MAX_FREQ_45_PRESC: u64 = 3000000000; /* Hz */
pub const ADF4350_MAX_FREQ_PFD: u32 = 32000000; /* Hz */
pub const ADF4350_MAX_BANDSEL_CLK: u32 = 125000; /* Hz */
pub const ADF4350_MAX_FREQ_REFIN: u32 = 250000000; /* Hz */
pub const ADF4350_MAX_MODULUS: u32 = 4095;
pub const ADF4350_MAX_R_CNT: u32 = 1023;

/// struct adf4350_platform_data - platform specific information
/// @name: Optional device name.
/// @clkin: REFin frequency in Hz.
/// @channel_spacing: Channel spacing in Hz (influences MODULUS).
/// @power_up_frequency: Optional, If set in Hz the PLL tunes to the desired frequency on probe.
/// @ref_div_factor: Optional, if set the driver skips dynamic calculation and uses this default value instead.
/// @ref_doubler_en: Enables reference doubler.
/// @ref_div2_en: Enables reference divider.
/// @r2_user_settings: User defined settings for ADF4350/1 REGISTER_2.
/// @r3_user_settings: User defined settings for ADF4350/1 REGISTER_3.
/// @r4_user_settings: User defined settings for ADF4350/1 REGISTER_4.
#[repr(C)]
pub struct adf4350_platform_data {
    pub name: [i8; 32],
    pub clkin: u64,
    pub channel_spacing: u64,
    pub power_up_frequency: u64,
    pub ref_div_factor: u16, /* 10-bit R counter */
    pub ref_doubler_en: bool,
    pub ref_div2_en: bool,
    pub r2_user_settings: u32,
    pub r3_user_settings: u32,
    pub r4_user_settings: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
