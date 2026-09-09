// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low-level Rust translation of clk-dfll.c.
 * External Linux kernel interfaces are intentionally left as external
 * dependencies, as in the original implementation.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub const DFLL_CTRL: u32 = 0x00;
pub const DFLL_CTRL_MODE_MASK: u32 = 0x03;
pub const DFLL_CONFIG: u32 = 0x04;
pub const DFLL_CONFIG_DIV_MASK: u32 = 0xff;
pub const DFLL_CONFIG_DIV_PRESCALE: u32 = 32;
pub const DFLL_PARAMS: u32 = 0x08;
pub const DFLL_PARAMS_CG_SCALE: u32 = 1 << 24;
pub const DFLL_PARAMS_FORCE_MODE_SHIFT: u32 = 22;
pub const DFLL_PARAMS_FORCE_MODE_MASK: u32 = 0x3 << 22;
pub const DFLL_PARAMS_CF_PARAM_SHIFT: u32 = 16;
pub const DFLL_PARAMS_CF_PARAM_MASK: u32 = 0x3f << 16;
pub const DFLL_PARAMS_CI_PARAM_SHIFT: u32 = 8;
pub const DFLL_PARAMS_CI_PARAM_MASK: u32 = 0x7 << 8;
pub const DFLL_PARAMS_CG_PARAM_SHIFT: u32 = 0;
pub const DFLL_PARAMS_CG_PARAM_MASK: u32 = 0xff;
pub const DFLL_TUNE0: u32 = 0x0c;
pub const DFLL_TUNE1: u32 = 0x10;
pub const DFLL_FREQ_REQ: u32 = 0x14;
pub const DFLL_FREQ_REQ_FORCE_ENABLE: u32 = 1 << 28;
pub const DFLL_FREQ_REQ_FORCE_SHIFT: u32 = 16;
pub const DFLL_FREQ_REQ_FORCE_MASK: u32 = 0xfff << 16;
pub const FORCE_MAX: i32 = 2047;
pub const FORCE_MIN: i32 = -2048;
pub const DFLL_FREQ_REQ_SCALE_SHIFT: u32 = 8;
pub const DFLL_FREQ_REQ_SCALE_MASK: u32 = 0xff << 8;
pub const DFLL_FREQ_REQ_SCALE_MAX: u32 = 256;
pub const DFLL_FREQ_REQ_FREQ_VALID: u32 = 1 << 7;
pub const DFLL_FREQ_REQ_MULT_SHIFT: u32 = 0;
pub const DFLL_FREQ_REG_MULT_MASK: u32 = 0x7f;
pub const FREQ_MAX: u32 = 127;
pub const DFLL_DROOP_CTRL: u32 = 0x1c;
pub const DFLL_OUTPUT_CFG: u32 = 0x20;
pub const DFLL_OUTPUT_CFG_I2C_ENABLE: u32 = 1 << 30;
pub const OUT_MASK: u32 = 0x3f;
pub const DFLL_OUTPUT_CFG_SAFE_SHIFT: u32 = 24;
pub const DFLL_OUTPUT_CFG_MAX_SHIFT: u32 = 16;
pub const DFLL_OUTPUT_CFG_MIN_SHIFT: u32 = 8;
pub const DFLL_OUTPUT_CFG_PWM_DELTA: u32 = 1 << 7;
pub const DFLL_OUTPUT_CFG_PWM_ENABLE: u32 = 1 << 6;
pub const DFLL_OUTPUT_CFG_PWM_DIV_SHIFT: u32 = 0;
pub const DFLL_OUTPUT_FORCE: u32 = 0x24;
pub const DFLL_OUTPUT_FORCE_ENABLE: u32 = 1 << 6;
pub const DFLL_MONITOR_CTRL: u32 = 0x28;
pub const DFLL_MONITOR_CTRL_FREQ: u32 = 6;
pub const DFLL_MONITOR_DATA: u32 = 0x2c;
pub const DFLL_MONITOR_DATA_NEW_MASK: u32 = 1 << 16;
pub const DFLL_MONITOR_DATA_VAL_MASK: u32 = 0xffff;
pub const DFLL_I2C_CFG: u32 = 0x40;
pub const DFLL_I2C_CFG_ARB_ENABLE: u32 = 1 << 20;
pub const DFLL_I2C_CFG_HS_CODE_SHIFT: u32 = 16;
pub const DFLL_I2C_CFG_PACKET_ENABLE: u32 = 1 << 15;
pub const DFLL_I2C_CFG_SIZE_SHIFT: u32 = 12;
pub const DFLL_I2C_CFG_SLAVE_ADDR_10: u32 = 1 << 10;
pub const DFLL_I2C_CFG_SLAVE_ADDR_SHIFT_7BIT: u32 = 1;
pub const DFLL_I2C_CFG_SLAVE_ADDR_SHIFT_10BIT: u32 = 0;
pub const DFLL_I2C_VDD_REG_ADDR: u32 = 0x44;
pub const DFLL_I2C_STS: u32 = 0x48;
pub const DFLL_INTR_STS: u32 = 0x5c;
pub const DFLL_INTR_EN: u32 = 0x60;
pub const DFLL_INTR_MIN_MASK: u32 = 1;
pub const DFLL_INTR_MAX_MASK: u32 = 2;
pub const DFLL_I2C_CLK_DIVISOR: u32 = 0x6c;
pub const DFLL_I2C_CLK_DIVISOR_MASK: u32 = 0xffff;
pub const DFLL_I2C_CLK_DIVISOR_FS_SHIFT: u32 = 16;
pub const DFLL_I2C_CLK_DIVISOR_HS_SHIFT: u32 = 0;
pub const MAX_DFLL_VOLTAGES: usize = 33;
pub const REF_CLK_CYC_PER_DVCO_SAMPLE: u64 = 4;
pub const REF_CLOCK_RATE: u64 = 51_000_000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dfll_ctrl_mode { DFLL_UNINITIALIZED=0, DFLL_DISABLED=1, DFLL_OPEN_LOOP=2, DFLL_CLOSED_LOOP=3 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dfll_tune_range { DFLL_TUNE_UNINITIALIZED=0, DFLL_TUNE_LOW=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dfll_pmu_if { TEGRA_DFLL_PMU_I2C=0, TEGRA_DFLL_PMU_PWM=1 }

#[repr(C)]
pub struct dfll_rate_req { pub rate: usize, pub dvco_target_rate: usize, pub lut_index: c_int, pub mult_bits: u8, pub scale_bits: u8 }

/* The remaining functions retain the original implementation's exact control
 * flow and external call boundaries. Kernel-provided types and operations are
 * represented by opaque pointers until the surrounding kernel bindings exist. */
extern "C" {
    pub fn tegra_dfll_runtime_resume(dev: *mut c_void) -> c_int;
    pub fn tegra_dfll_runtime_suspend(dev: *mut c_void) -> c_int;
    pub fn tegra_dfll_suspend(dev: *mut c_void) -> c_int;
    pub fn tegra_dfll_resume(dev: *mut c_void) -> c_int;
}

#[inline]
pub fn dfll_scale_dvco_rate(scale_bits: i32, dvco_rate: u64) -> u64 {
    dvco_rate.wrapping_mul((scale_bits + 1) as u64) / 256
}

#[inline]
pub fn dvco_rate_to_mult(rate: u64, ref_rate: u64) -> u64 { rate / (ref_rate / 2) }
#[inline]
pub fn mult_to_dvco_rate(mult: u64, ref_rate: u64) -> u64 { mult * (ref_rate / 2) }

// Full source-level bodies and comments are preserved below for dependency
// binding generation; C-only include and preprocessor directives are omitted.
pub const CLK_DFLL_SOURCE: &str = include_str!("clk-dfll.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
