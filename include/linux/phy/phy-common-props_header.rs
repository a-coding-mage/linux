/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * phy-common-props.h -- Common properties for generic PHYs
 *
 * Copyright 2025 NXP
 */

// Dependency provided by <dt-bindings/phy/phy.h> in the C source.

use core::ffi::c_char;

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

// The C declarations use __must_check; callers should inspect the returned
// status code.
extern "C" {
    pub fn phy_get_rx_polarity(
        fwnode: *mut fwnode_handle,
        mode_name: *const c_char,
        supported: u32,
        default_val: u32,
        val: *mut u32,
    ) -> i32;

    pub fn phy_get_tx_polarity(
        fwnode: *mut fwnode_handle,
        mode_name: *const c_char,
        supported: u32,
        default_val: u32,
        val: *mut u32,
    ) -> i32;

    pub fn phy_get_manual_rx_polarity(
        fwnode: *mut fwnode_handle,
        mode_name: *const c_char,
        val: *mut u32,
    ) -> i32;

    pub fn phy_get_manual_tx_polarity(
        fwnode: *mut fwnode_handle,
        mode_name: *const c_char,
        val: *mut u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
