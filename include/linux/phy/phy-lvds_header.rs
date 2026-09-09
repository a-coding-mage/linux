/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2020,2022 NXP
 */

/**
 * struct phy_configure_opts_lvds - LVDS configuration set
 * @bits_per_lane_and_dclk_cycle: Number of bits per lane per differential
 *                                 clock cycle.
 * @differential_clk_rate:        Clock rate, in Hertz, of the LVDS
 *                                 differential clock.
 * @lanes:                        Number of active, consecutive, data lanes,
 *                                 starting from lane 0, used for the
 *                                 transmissions.
 * @is_slave:                     Boolean, true if the phy is a slave which
 *                                 works together with a master phy to support
 *                                 dual link transmission, otherwise a regular
 *                                 phy or a master phy.
 *
 * This structure is used to represent the configuration state of a LVDS phy.
 */
#[repr(C)]
pub struct phy_configure_opts_lvds {
    pub bits_per_lane_and_dclk_cycle: u32,
    pub differential_clk_rate: core::ffi::c_ulong,
    pub lanes: u32,
    pub is_slave: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
