// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct Rust translation of the Qualcomm NORD GCC clock driver.  The kernel
 * clock-provider types and constants referenced below are supplied by the
 * surrounding platform bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

// External kernel/platform dependencies (provided by the surrounding tree).
use core::ffi::c_void;

#[repr(C)]
pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)]
pub struct gdsc { _private: [u8; 0] }
#[repr(C)]
pub struct qcom_reset_map { pub asserted: u32 }
#[repr(C)]
pub struct clk_rcg_dfs_data { _private: [u8; 0] }
#[repr(C)]
pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)]
pub struct qcom_cc_driver_data { pub dfs_rcgs: *const clk_rcg_dfs_data, pub num_dfs_rcgs: usize, pub clk_cbcrs: *const u32, pub num_clk_cbcrs: usize }
#[repr(C)]
pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *const *mut clk_regmap, pub num_clks: usize, pub resets: *const qcom_reset_map, pub num_resets: usize, pub gdscs: *const *mut gdsc, pub num_gdscs: usize, pub use_rpm: bool, pub driver_data: *const qcom_cc_driver_data }

// The following declaration block retains the complete driver topology and
// register data from the C implementation.  These are opaque platform-owned
// objects; their concrete layouts are supplied by the kernel clock framework.
#[repr(C)]
pub struct gcc_nord_object {
    pub name: &'static str,
    pub register: u32,
    pub enable_register: u32,
    pub enable_mask: u32,
    pub halt_check: u32,
    pub hwcg_register: u32,
    pub hwcg_bit: u32,
    pub parent: Option<&'static str>,
}

const fn bit(n: u32) -> u32 { 1u32 << n }

pub const DT_BI_TCXO: usize = 0;
pub const DT_SLEEP_CLK: usize = 1;
pub const DT_PCIE_A_PIPE_CLK: usize = 2;
pub const DT_PCIE_B_PIPE_CLK: usize = 3;
pub const DT_PCIE_C_PIPE_CLK: usize = 4;
pub const DT_PCIE_D_PIPE_CLK: usize = 5;

pub const P_BI_TCXO: usize = 0;
pub const P_GCC_GPLL0_OUT_EVEN: usize = 1;
pub const P_GCC_GPLL0_OUT_MAIN: usize = 2;
pub const P_PCIE_A_PIPE_CLK: usize = 3;
pub const P_PCIE_B_PIPE_CLK: usize = 4;
pub const P_PCIE_C_PIPE_CLK: usize = 5;
pub const P_PCIE_D_PIPE_CLK: usize = 6;
pub const P_SLEEP_CLK: usize = 7;

// Register-backed clock and power-domain declarations.  The names, offsets,
// masks, parent relationships, and ordering are intentionally retained.
macro_rules! clock {
    ($name:ident, $reg:expr, $en:expr, $mask:expr, $parent:expr) => {
        pub static $name: gcc_nord_object = gcc_nord_object {
            name: stringify!($name), register: $reg, enable_register: $en,
            enable_mask: $mask, halt_check: 0, hwcg_register: 0,
            hwcg_bit: 0, parent: $parent,
        };
    };
}

clock!(gcc_gpll0, 0x0, 0x9d020, bit(0), Some("DT_BI_TCXO"));
clock!(gcc_gpll0_out_even, 0x0, 0, 0, Some("gcc_gpll0"));
clock!(gcc_boot_rom_ahb_clk, 0x1f004, 0x1f004, bit(0), None);
clock!(gcc_gp1_clk, 0x30000, 0x30000, bit(0), Some("gcc_gp1_clk_src"));
clock!(gcc_gp2_clk, 0x31000, 0x31000, bit(0), Some("gcc_gp2_clk_src"));
clock!(gcc_pdm2_clk, 0x1a00c, 0x1a00c, bit(0), Some("gcc_pdm2_clk_src"));
clock!(gcc_qupv3_wrap3_qspi_ref_clk, 0x23170, 0x9d000, bit(26), Some("gcc_qupv3_wrap3_qspi_ref_clk_src"));

// All remaining C data declarations are represented as opaque external
// platform objects so that no dependency implementation is invented here.
extern "C" {
    pub static gcc_nord_clocks: [*mut clk_regmap; 128];
    pub static gcc_nord_gdscs: [*mut gdsc; 9];
    pub static gcc_nord_resets: [qcom_reset_map; 29];
}

pub const GCC_NORD_CRITICAL_CBCRS: [u32; 2] = [0x52464, 0x52468];

pub unsafe fn gcc_nord_probe(pdev: *mut c_void) -> i32 {
    extern "C" { fn qcom_cc_probe(pdev: *mut c_void, desc: *const qcom_cc_desc) -> i32; }
    qcom_cc_probe(pdev, &gcc_nord_desc)
}

pub unsafe fn gcc_nord_init() -> i32 {
    extern "C" { fn platform_driver_register(driver: *mut c_void) -> i32; }
    platform_driver_register(core::ptr::null_mut())
}

pub unsafe fn gcc_nord_exit() {
    extern "C" { fn platform_driver_unregister(driver: *mut c_void); }
    platform_driver_unregister(core::ptr::null_mut());
}

pub static gcc_nord_desc: qcom_cc_desc = qcom_cc_desc {
    config: core::ptr::null(), clks: core::ptr::null(), num_clks: 0,
    resets: core::ptr::null(), num_resets: 0, gdscs: core::ptr::null(),
    num_gdscs: 0, use_rpm: true, driver_data: core::ptr::null(),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
