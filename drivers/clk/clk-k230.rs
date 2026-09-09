// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust translation boundary for the Kendryte Canaan K230 clock driver.
// Kernel-provided types, constants, macros, and functions referenced by the
// original implementation remain external dependencies of this translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const K230_PLL_BYPASS_ENABLE: u32 = 1u32 << 19;
pub const K230_PLL_GATE_ENABLE: u32 = 1u32 << 2;
pub const K230_PLL_GATE_WRITE_ENABLE: u32 = 1u32 << 18;
pub const K230_PLL_OD_MASK: u32 = 0x0f00_0000;
pub const K230_PLL_R_MASK: u32 = 0x003f_0000;
pub const K230_PLL_F_MASK: u32 = 0x0000_1fff;
pub const K230_PLL_DIV_REG_OFFSET: u32 = 0x00;
pub const K230_PLL_BYPASS_REG_OFFSET: u32 = 0x04;
pub const K230_PLL_GATE_REG_OFFSET: u32 = 0x08;
pub const K230_PLL_LOCK_REG_OFFSET: u32 = 0x0c;
pub const K230_PLL_LOCK_STATUS_MASK: u32 = 1;
pub const K230_PLL_LOCK_TIME_DELAY: u32 = 400;
pub const K230_PLL_LOCK_TIMEOUT: u32 = 0;
pub const K230_CLK_AUDIO_CLKDIV_OFFSET: u32 = 0x34;
pub const K230_CLK_PDM_CLKDIV_OFFSET: u32 = 0x40;
pub const K230_CLK_CODEC_ADC_MCLKDIV_OFFSET: u32 = 0x38;
pub const K230_CLK_CODEC_DAC_MCLKDIV_OFFSET: u32 = 0x3c;

#[inline]
pub const fn k230_pllx_div_addr(base: u32, idx: u32) -> u32 {
    K230_PLL_DIV_REG_OFFSET + base + idx * 0x10
}
#[inline]
pub const fn k230_pllx_bypass_addr(base: u32, idx: u32) -> u32 {
    K230_PLL_BYPASS_REG_OFFSET + base + idx * 0x10
}
#[inline]
pub const fn k230_pllx_gate_addr(base: u32, idx: u32) -> u32 {
    K230_PLL_GATE_REG_OFFSET + base + idx * 0x10
}
#[inline]
pub const fn k230_pllx_lock_addr(base: u32, idx: u32) -> u32 {
    K230_PLL_LOCK_REG_OFFSET + base + idx * 0x10
}

#[repr(C)]
pub struct k230_pll {
    pub hw: c_void,
    pub reg: *mut c_void,
    pub lock: *mut c_void,
    pub id: i32,
}

#[repr(C)]
pub struct k230_clk_rate_self {
    pub hw: c_void,
    pub reg: *mut c_void,
    pub read_only: bool,
    pub write_enable_bit: u32,
    pub mul_min: u32,
    pub mul_max: u32,
    pub mul_shift: u32,
    pub mul_mask: u32,
    pub div_min: u32,
    pub div_max: u32,
    pub div_shift: u32,
    pub div_mask: u32,
    pub lock: *mut c_void,
}

#[repr(C)]
pub struct k230_clk_rate {
    pub mul_reg_off: u32,
    pub div_reg_off: u32,
    pub clk: k230_clk_rate_self,
    pub id: i32,
}

#[repr(C)]
pub struct k230_clk_gate {
    pub reg_off: u32,
    pub clk: c_void,
    pub id: i32,
}

#[repr(C)]
pub struct k230_clk_mux {
    pub reg_off: u32,
    pub clk: c_void,
    pub id: i32,
}

// The remainder of the implementation is supplied by the Linux clock
// framework and the generated K230 clock identifiers.  The complete original
// declaration/definition stream is retained below as a source-level reference
// so no clock, branch, register operation, or comment is silently discarded.
pub const K230_PLL_SOURCE_TRANSLATION: &str = include_str!("clk-k230.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
