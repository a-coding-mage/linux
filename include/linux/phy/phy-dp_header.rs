/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 Cadence Design Systems Inc.
 */

// Dependency intent: u8 corresponds to the Linux kernel's u8 type.

pub const PHY_SUBMODE_DP: u32 = 0;
pub const PHY_SUBMODE_EDP: u32 = 1;

/**
 * struct phy_configure_opts_dp - DisplayPort PHY configuration set
 *
 * This structure is used to represent the configuration state of a
 * DisplayPort phy.
 */
#[repr(C)]
pub struct phy_configure_opts_dp {
    /**
     * @link_rate:
     *
     * Link Rate, in Mb/s, of the main link.
     *
     * Allowed values: 1620, 2160, 2430, 2700, 3240, 4320, 5400, 8100 Mb/s
     */
    pub link_rate: u32,

    /**
     * @lanes:
     *
     * Number of active, consecutive, data lanes, starting from
     * lane 0, used for the transmissions on main link.
     *
     * Allowed values: 1, 2, 4
     */
    pub lanes: u32,

    /**
     * @voltage:
     *
     * Voltage swing levels, as specified by DisplayPort specification,
     * to be used by particular lanes. One value per lane.
     * voltage[0] is for lane 0, voltage[1] is for lane 1, etc.
     *
     * Maximum value: 3
     */
    pub voltage: [u32; 4],

    /**
     * @pre:
     *
     * Pre-emphasis levels, as specified by DisplayPort specification, to be
     * used by particular lanes. One value per lane.
     *
     * Maximum value: 3
     */
    pub pre: [u32; 4],

    /**
     * The following fields are C bit-fields of type u8. Rust has no native
     * bit-field syntax; each field retains the source name and one-bit value
     * intent. Values are expected to be 0 or 1.
     */
    /// Flag indicating whether or not to enable spread-spectrum clocking.
    pub ssc: u8,

    /// Flag indicating whether or not to reconfigure link rate and SSC.
    pub set_rate: u8,

    /// Flag indicating whether or not to reconfigure lane count.
    pub set_lanes: u8,

    /// Flag indicating whether or not to reconfigure voltage swing and pre-emphasis.
    pub set_voltages: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
