/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AD9523 SPI Low Jitter Clock Generator
 *
 * Copyright 2012 Analog Devices Inc.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum outp_drv_mode {
    TRISTATE,
    LVPECL_8mA,
    LVDS_4mA,
    LVDS_7mA,
    HSTL0_16mA,
    HSTL1_8mA,
    CMOS_CONF1,
    CMOS_CONF2,
    CMOS_CONF3,
    CMOS_CONF4,
    CMOS_CONF5,
    CMOS_CONF6,
    CMOS_CONF7,
    CMOS_CONF8,
    CMOS_CONF9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ref_sel_mode {
    NONEREVERTIVE_STAY_ON_REFB,
    REVERT_TO_REFA,
    SELECT_REFA,
    SELECT_REFB,
    EXT_REF_SEL,
}

/**
 * struct ad9523_channel_spec - Output channel configuration
 *
 * @channel_num: Output channel number.
 * @divider_output_invert_en: Invert the polarity of the output clock.
 * @sync_ignore_en: Ignore chip-level SYNC signal.
 * @low_power_mode_en: Reduce power used in the differential output modes.
 * @use_alt_clock_src: Channel divider uses alternative clk source.
 * @output_dis: Disables, powers down the entire channel.
 * @driver_mode: Output driver mode (logic level family).
 * @divider_phase: Divider initial phase after a SYNC. Range 0..63
 *                 LSB = 1/2 of a period of the divider input clock.
 * @channel_divider: 10-bit channel divider.
 * @extended_name: Optional descriptive channel name.
 */
#[repr(C)]
pub struct ad9523_channel_spec {
    pub channel_num: ::core::ffi::c_uint,
    pub divider_output_invert_en: bool,
    pub sync_ignore_en: bool,
    pub low_power_mode_en: bool,
    /* CH0..CH3 VCXO, CH4..CH9 VCO2 */
    pub use_alt_clock_src: bool,
    pub output_dis: bool,
    pub driver_mode: outp_drv_mode,
    pub divider_phase: ::core::ffi::c_uchar,
    pub channel_divider: ::core::ffi::c_ushort,
    pub extended_name: [::core::ffi::c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pll1_rzero_resistor {
    RZERO_883_OHM,
    RZERO_677_OHM,
    RZERO_341_OHM,
    RZERO_135_OHM,
    RZERO_10_OHM,
    RZERO_USE_EXT_RES = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rpole2_resistor {
    RPOLE2_900_OHM,
    RPOLE2_450_OHM,
    RPOLE2_300_OHM,
    RPOLE2_225_OHM,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rzero_resistor {
    RZERO_3250_OHM,
    RZERO_2750_OHM,
    RZERO_2250_OHM,
    RZERO_2100_OHM,
    RZERO_3000_OHM,
    RZERO_2500_OHM,
    RZERO_2000_OHM,
    RZERO_1850_OHM,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cpole1_capacitor {
    CPOLE1_0_PF,
    CPOLE1_8_PF,
    CPOLE1_16_PF,
    CPOLE1_24_PF,
    _CPOLE1_24_PF,
    CPOLE1_32_PF,
    CPOLE1_40_PF,
    CPOLE1_48_PF,
}

/**
 * struct ad9523_platform_data - platform specific information
 *
 * @vcxo_freq: External VCXO frequency in Hz
 * @refa_diff_rcv_en: REFA differential/single-ended input selection.
 * @refb_diff_rcv_en: REFB differential/single-ended input selection.
 * @zd_in_diff_en: Zero Delay differential/single-ended input selection.
 * @osc_in_diff_en: OSC differential/ single-ended input selection.
 * @refa_cmos_neg_inp_en: REFA single-ended neg./pos. input enable.
 * @refb_cmos_neg_inp_en: REFB single-ended neg./pos. input enable.
 * @zd_in_cmos_neg_inp_en: Zero Delay single-ended neg./pos. input enable.
 * @osc_in_cmos_neg_inp_en: OSC single-ended neg./pos. input enable.
 * @refa_r_div: PLL1 10-bit REFA R divider.
 * @refb_r_div: PLL1 10-bit REFB R divider.
 * @pll1_feedback_div: PLL1 10-bit Feedback N divider.
 * @pll1_charge_pump_current_nA: Magnitude of PLL1 charge pump current (nA).
 * @zero_delay_mode_internal_en: Internal, external Zero Delay mode selection.
 * @osc_in_feedback_en: PLL1 feedback path, local feedback from
 *                       the OSC_IN receiver or zero delay mode
 * @pll1_loop_filter_rzero: PLL1 Loop Filter Zero Resistor selection.
 * @ref_mode: Reference selection mode.
 * @pll2_charge_pump_current_nA: Magnitude of PLL2 charge pump current (nA).
 * @pll2_ndiv_a_cnt: PLL2 Feedback N-divider, A Counter, range 0..4.
 * @pll2_ndiv_b_cnt: PLL2 Feedback N-divider, B Counter, range 0..63.
 * @pll2_freq_doubler_en: PLL2 frequency doubler enable.
 * @pll2_r2_div: PLL2 R2 divider, range 0..31.
 * @pll2_vco_div_m1: VCO1 divider, range 3..5.
 * @pll2_vco_div_m2: VCO2 divider, range 3..5.
 * @rpole2: PLL2 loop filter Rpole resistor value.
 * @rzero: PLL2 loop filter Rzero resistor value.
 * @cpole1: PLL2 loop filter Cpole capacitor value.
 * @rzero_bypass_en: PLL2 loop filter Rzero bypass enable.
 * @num_channels: Array size of struct ad9523_channel_spec.
 * @channels: Pointer to channel array.
 * @name: Optional alternative iio device name.
 */
#[repr(C)]
pub struct ad9523_platform_data {
    pub vcxo_freq: ::core::ffi::c_ulong,

    /* Differential/ Single-Ended Input Configuration */
    pub refa_diff_rcv_en: bool,
    pub refb_diff_rcv_en: bool,
    pub zd_in_diff_en: bool,
    pub osc_in_diff_en: bool,

    /* Valid if differential input disabled
     * if false defaults to pos input */
    pub refa_cmos_neg_inp_en: bool,
    pub refb_cmos_neg_inp_en: bool,
    pub zd_in_cmos_neg_inp_en: bool,
    pub osc_in_cmos_neg_inp_en: bool,

    /* PLL1 Setting */
    pub refa_r_div: ::core::ffi::c_ushort,
    pub refb_r_div: ::core::ffi::c_ushort,
    pub pll1_feedback_div: ::core::ffi::c_ushort,
    pub pll1_charge_pump_current_nA: ::core::ffi::c_ushort,
    pub zero_delay_mode_internal_en: bool,
    pub osc_in_feedback_en: bool,
    pub pll1_loop_filter_rzero: pll1_rzero_resistor,

    /* Reference */
    pub ref_mode: ref_sel_mode,

    /* PLL2 Setting */
    pub pll2_charge_pump_current_nA: ::core::ffi::c_uint,
    pub pll2_ndiv_a_cnt: ::core::ffi::c_uchar,
    pub pll2_ndiv_b_cnt: ::core::ffi::c_uchar,
    pub pll2_freq_doubler_en: bool,
    pub pll2_r2_div: ::core::ffi::c_uchar,
    pub pll2_vco_div_m1: ::core::ffi::c_uchar,
    pub pll2_vco_div_m2: ::core::ffi::c_uchar,

    /* Loop Filter PLL2 */
    pub rpole2: rpole2_resistor,
    pub rzero: rzero_resistor,
    pub cpole1: cpole1_capacitor,
    pub rzero_bypass_en: bool,

    /* Output Channel Configuration */
    pub num_channels: ::core::ffi::c_int,
    pub channels: *mut ad9523_channel_spec,

    pub name: [::core::ffi::c_char; SPI_NAME_SIZE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
