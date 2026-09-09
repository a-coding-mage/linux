/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Cadence Design Systems Inc.
 */

use core::ffi::{c_int, c_ulong, c_ulonglong, c_uint, c_uchar};

/**
 * struct phy_configure_opts_mipi_dphy - MIPI D-PHY configuration set
 *
 * This structure is used to represent the configuration state of a
 * MIPI D-PHY phy.
 */
#[repr(C)]
pub struct phy_configure_opts_mipi_dphy {
	/**
	 * @clk_miss:
	 *
	 * Timeout, in picoseconds, for receiver to detect absence of
	 * Clock transitions and disable the Clock Lane HS-RX.
	 *
	 * Maximum value: 60000 ps
	 */
	pub clk_miss: c_uint,

	/**
	 * @clk_post:
	 *
	 * Time, in picoseconds, that the transmitter continues to
	 * send HS clock after the last associated Data Lane has
	 * transitioned to LP Mode. Interval is defined as the period
	 * from the end of @hs_trail to the beginning of @clk_trail.
	 *
	 * Minimum value: 60000 ps + 52 * @hs_clk_rate period in ps
	 */
	pub clk_post: c_uint,

	/**
	 * @clk_pre:
	 *
	 * Time, in UI, that the HS clock shall be driven by
	 * the transmitter prior to any associated Data Lane beginning
	 * the transition from LP to HS mode.
	 *
	 * Minimum value: 8 UI
	 */
	pub clk_pre: c_uint,

	/** @clk_prepare: Time, in picoseconds, that the transmitter drives the Clock Lane LP-00 Line state immediately before the HS-0 Line state starting the HS transmission. Minimum value: 38000 ps; Maximum value: 95000 ps */
	pub clk_prepare: c_uint,
	/** @clk_settle: Time interval, in picoseconds, during which the HS receiver should ignore any Clock Lane HS transitions, starting from the beginning of @clk_prepare. Minimum value: 95000 ps; Maximum value: 300000 ps */
	pub clk_settle: c_uint,
	/** @clk_term_en: Time, in picoseconds, for the Clock Lane receiver to enable the HS line termination. Maximum value: 38000 ps */
	pub clk_term_en: c_uint,
	/** @clk_trail: Time, in picoseconds, that the transmitter drives the HS-0 state after the last payload clock bit of a HS transmission burst. Minimum value: 60000 ps */
	pub clk_trail: c_uint,
	/** @clk_zero: Time, in picoseconds, that the transmitter drives the HS-0 state prior to starting the Clock. */
	pub clk_zero: c_uint,
	/** @d_term_en: Time, in picoseconds, for the Data Lane receiver to enable the HS line termination. Maximum value: 35000 ps + 4 * @hs_clk_rate period in ps */
	pub d_term_en: c_uint,
	/** @eot: Transmitted time interval, in picoseconds, from the start of @hs_trail or @clk_trail, to the start of the LP- 11 state following a HS burst. Maximum value: 105000 ps + 12 * @hs_clk_rate period in ps */
	pub eot: c_uint,
	/** @hs_exit: Time, in picoseconds, that the transmitter drives LP-11 following a HS burst. Minimum value: 100000 ps */
	pub hs_exit: c_uint,
	/** @hs_prepare: Time, in picoseconds, that the transmitter drives the Data Lane LP-00 Line state immediately before the HS-0 Line state starting the HS transmission. Minimum value: 40000 ps + 4 * @hs_clk_rate period in ps; Maximum value: 85000 ps + 6 * @hs_clk_rate period in ps */
	pub hs_prepare: c_uint,
	/** @hs_settle: Time interval, in picoseconds, during which the HS receiver shall ignore any Data Lane HS transitions, starting from the beginning of @hs_prepare. Minimum value: 85000 ps + 6 * @hs_clk_rate period in ps; Maximum value: 145000 ps + 10 * @hs_clk_rate period in ps */
	pub hs_settle: c_uint,
	/** @hs_skip: Time interval, in picoseconds, during which the HS-RX should ignore any transitions on the Data Lane, following a HS burst. The end point of the interval is defined as the beginning of the LP-11 state following the HS burst. Minimum value: 40000 ps; Maximum value: 55000 ps + 4 * @hs_clk_rate period in ps */
	pub hs_skip: c_uint,
	/** @hs_trail: Time, in picoseconds, that the transmitter drives the flipped differential state after last payload data bit of a HS transmission burst. Minimum value: max(8 * @hs_clk_rate period in ps, 60000 ps + 4 * @hs_clk_rate period in ps) */
	pub hs_trail: c_uint,
	/** @hs_zero: Time, in picoseconds, that the transmitter drives the HS-0 state prior to transmitting the Sync sequence. */
	pub hs_zero: c_uint,
	/** @init: Time, in microseconds for the initialization period to complete. Minimum value: 100 us */
	pub init: c_uint,
	/** @lpx: Transmitted length, in picoseconds, of any Low-Power state period. Minimum value: 50000 ps */
	pub lpx: c_uint,
	/** @ta_get: Time, in picoseconds, that the new transmitter drives the Bridge state (LP-00) after accepting control during a Link Turnaround. Value: 5 * @lpx */
	pub ta_get: c_uint,
	/** @ta_go: Time, in picoseconds, that the transmitter drives the Bridge state (LP-00) before releasing control during a Link Turnaround. Value: 4 * @lpx */
	pub ta_go: c_uint,
	/** @ta_sure: Time, in picoseconds, that the new transmitter waits after the LP-10 state before transmitting the Bridge state (LP-00) during a Link Turnaround. Minimum value: @lpx; Maximum value: 2 * @lpx */
	pub ta_sure: c_uint,
	/** @wakeup: Time, in microseconds, that a transmitter drives a Mark-1 state prior to a Stop state in order to initiate an exit from ULPS. Minimum value: 1000 us */
	pub wakeup: c_uint,
	/** @hs_clk_rate: Clock rate, in Hertz, of the high-speed clock. */
	pub hs_clk_rate: c_ulong,
	/** @lp_clk_rate: Clock rate, in Hertz, of the low-power clock. */
	pub lp_clk_rate: c_ulong,
	/** @lanes: Number of active, consecutive, data lanes, starting from lane 0, used for the transmissions. */
	pub lanes: c_uchar,
}

extern "C" {
	pub fn phy_mipi_dphy_get_default_config(
		pixel_clock: c_ulong,
		bpp: c_uint,
		lanes: c_uint,
		cfg: *mut phy_configure_opts_mipi_dphy,
	) -> c_int;

	pub fn phy_mipi_dphy_get_default_config_for_hsclk(
		hs_clk_rate: c_ulonglong,
		lanes: c_uint,
		cfg: *mut phy_configure_opts_mipi_dphy,
	) -> c_int;

	pub fn phy_mipi_dphy_config_validate(
		cfg: *mut phy_configure_opts_mipi_dphy,
	) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
