/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Marvell EBU SoC common clock handling
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 *
 */

// Dependency declarations corresponding to <linux/kernel.h> and related
// kernel-provided types.

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub static mut ctrl_gating_lock: spinlock_t;
}

#[repr(C)]
pub struct coreclk_ratio {
    pub id: i32,
    pub name: *const ::std::os::raw::c_char,
}

#[repr(C)]
pub struct coreclk_soc_desc {
    pub get_tclk_freq:
        Option<unsafe extern "C" fn(sar: *mut ::std::ffi::c_void) -> u32>,
    pub get_cpu_freq:
        Option<unsafe extern "C" fn(sar: *mut ::std::ffi::c_void) -> u32>,
    pub get_clk_ratio: Option<unsafe extern "C" fn(
        sar: *mut ::std::ffi::c_void,
        id: i32,
        mult: *mut i32,
        div: *mut i32,
    )>,
    pub get_refclk_freq:
        Option<unsafe extern "C" fn(sar: *mut ::std::ffi::c_void) -> u32>,
    pub is_sscg_enabled:
        Option<unsafe extern "C" fn(sar: *mut ::std::ffi::c_void) -> bool>,
    pub fix_sscg_deviation: Option<unsafe extern "C" fn(system_clk: u32) -> u32>,
    pub ratios: *const coreclk_ratio,
    pub num_ratios: i32,
}

#[repr(C)]
pub struct clk_gating_soc_desc {
    pub name: *const ::std::os::raw::c_char,
    pub parent: *const ::std::os::raw::c_char,
    pub bit_idx: i32,
    pub flags: usize,
}

extern "C" {
    pub fn mvebu_coreclk_setup(
        np: *mut device_node,
        desc: *const coreclk_soc_desc,
    );

    pub fn mvebu_clk_gating_setup(
        np: *mut device_node,
        desc: *const clk_gating_soc_desc,
    );

/*
 * This function is shared among the Kirkwood, Armada 370, Armada XP
 * and Armada 375 SoC
 */
    pub fn kirkwood_fix_sscg_deviation(system_clk: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
