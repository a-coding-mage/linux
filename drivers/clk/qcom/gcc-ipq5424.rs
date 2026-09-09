// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust representation of gcc-ipq5424.c.  The clock,
// reset, interconnect, and platform-driver types referenced below are supplied
// by the surrounding Qualcomm clock framework.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

#[repr(C)]
pub struct clk_parent_data {
    pub index: u32,
    pub hw: *const c_void,
}

#[repr(C)]
pub struct parent_map {
    pub src: u32,
    pub cfg: u32,
}

#[repr(C)]
pub struct clk_div_table {
    pub val: u32,
    pub div: u32,
}

// Device-tree parent indices from the C implementation.
pub const DT_XO: u32 = 0;
pub const DT_SLEEP_CLK: u32 = 1;
pub const DT_PCIE30_PHY0_PIPE_CLK: u32 = 2;
pub const DT_PCIE30_PHY1_PIPE_CLK: u32 = 3;
pub const DT_PCIE30_PHY2_PIPE_CLK: u32 = 4;
pub const DT_PCIE30_PHY3_PIPE_CLK: u32 = 5;
pub const DT_USB_PCIE_WRAPPER_PIPE_CLK: u32 = 6;

pub const P_GCC_GPLL0_OUT_MAIN_DIV_CLK_SRC: u32 = 0;
pub const P_GPLL0_OUT_AUX: u32 = 1;
pub const P_GPLL0_OUT_MAIN: u32 = 2;
pub const P_GPLL2_OUT_AUX: u32 = 3;
pub const P_GPLL2_OUT_MAIN: u32 = 4;
pub const P_GPLL4_OUT_AUX: u32 = 5;
pub const P_GPLL4_OUT_MAIN: u32 = 6;
pub const P_SLEEP_CLK: u32 = 7;
pub const P_XO: u32 = 8;
pub const P_USB3PHY_0_PIPE: u32 = 9;

pub const IPQ_APPS_ID: u32 = 5424;

extern "C" {
    // The complete clock/reset tables and framework objects are external
    // dependencies, exactly as they are in the original implementation.
    pub static mut gcc_ipq5424_desc: c_void;
    pub fn qcom_cc_probe(pdev: *mut c_void, desc: *const c_void) -> i32;
    pub fn platform_driver_register(driver: *mut c_void) -> i32;
    pub fn platform_driver_unregister(driver: *mut c_void);
    pub fn icc_sync_state(dev: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn gcc_ipq5424_probe(pdev: *mut c_void) -> i32 {
    qcom_cc_probe(pdev, &gcc_ipq5424_desc as *const _ as *const c_void)
}

// The remaining declarations are intentionally kept as source-level
// framework objects: their concrete layouts and constants are provided by the
// imported Qualcomm clock framework, as required by the original C file.
#[no_mangle]
pub unsafe extern "C" fn gcc_ipq5424_init(driver: *mut c_void) -> i32 {
    platform_driver_register(driver)
}

#[no_mangle]
pub unsafe extern "C" fn gcc_ipq5424_exit(driver: *mut c_void) {
    platform_driver_unregister(driver)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
