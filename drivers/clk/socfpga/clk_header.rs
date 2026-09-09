/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013, Steffen Trumtrar <s.trumtrar@pengutronix.de>
 *
 * based on drivers/clk/tegra/clk.h
 */

// Dependency supplied by <linux/clk-provider.h>.
use core::ffi::{c_char, c_void};

/* Clock Manager offsets */
pub const CLKMGR_CTRL: u32 = 0x0;
pub const CLKMGR_BYPASS: u32 = 0x4;
pub const CLKMGR_DBCTRL: u32 = 0x10;
pub const CLKMGR_L4SRC: u32 = 0x70;
pub const CLKMGR_PERPLL_SRC: u32 = 0xAC;

pub const SOCFPGA_MAX_PARENTS: usize = 5;

#[macro_export]
macro_rules! streq {
    ($a:expr, $b:expr) => {
        unsafe { libc::strcmp($a, $b) == 0 }
    };
}

#[macro_export]
macro_rules! SYSMGR_SDMMC_CTRL_SET {
    ($smplsel:expr, $drvsel:expr) => {
        ((($smplsel & 0x7) << 3) | (($drvsel & 0x7) << 0))
    };
}

#[macro_export]
macro_rules! SYSMGR_SDMMC_CTRL_SET_AS10 {
    ($smplsel:expr, $drvsel:expr) => {
        ((($smplsel & 0x7) << 4) | (($drvsel & 0x7) << 0))
    };
}

extern "C" {
    pub static mut clk_mgr_base_addr: *mut c_void;
    pub static mut clk_mgr_a10_base_addr: *mut c_void;

    pub fn socfpga_pll_init(node: *mut device_node);
    pub fn socfpga_periph_init(node: *mut device_node);
    pub fn socfpga_gate_init(node: *mut device_node);
    pub fn socfpga_a10_pll_init(node: *mut device_node);
    pub fn socfpga_a10_periph_init(node: *mut device_node);
    pub fn socfpga_a10_gate_init(node: *mut device_node);
}

// Types supplied by external kernel headers.
#[repr(C)]
pub struct socfpga_pll {
    pub hw: clk_gate,
}

#[repr(C)]
pub struct socfpga_gate_clk {
    pub hw: clk_gate,
    pub parent_name: *mut c_char,
    pub fixed_div: u32,
    pub div_reg: *mut c_void,
    pub bypass_reg: *mut c_void,
    pub sys_mgr_base_addr: *mut regmap,
    pub width: u32, // only valid if div_reg != 0
    pub shift: u32, // only valid if div_reg != 0
    pub bypass_shift: u32, // only valid if bypass_reg != 0
}

#[repr(C)]
pub struct socfpga_periph_clk {
    pub hw: clk_gate,
    pub parent_name: *mut c_char,
    pub fixed_div: u32,
    pub div_reg: *mut c_void,
    pub bypass_reg: *mut c_void,
    pub width: u32, // only valid if div_reg != 0
    pub shift: u32, // only valid if div_reg != 0
    pub bypass_shift: u32, // only valid if bypass_reg != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
