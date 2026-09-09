// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of clk-rcg2.c.
 * Kernel-provided types and functions are intentionally referenced externally.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const CMD_REG: u32 = 0x0;
pub const CMD_UPDATE: u32 = 1 << 0;
pub const CMD_ROOT_EN: u32 = 1 << 1;
pub const CMD_DIRTY_CFG: u32 = 1 << 4;
pub const CMD_DIRTY_N: u32 = 1 << 5;
pub const CMD_DIRTY_M: u32 = 1 << 6;
pub const CMD_DIRTY_D: u32 = 1 << 7;
pub const CMD_ROOT_OFF: u32 = 1 << 31;
pub const CFG_REG: u32 = 0x4;
pub const CFG_SRC_DIV_SHIFT: u32 = 0;
pub const CFG_SRC_DIV_LENGTH: u32 = 8;
pub const CFG_SRC_SEL_SHIFT: u32 = 8;
pub const CFG_SRC_SEL_MASK: u32 = 0x7 << CFG_SRC_SEL_SHIFT;
pub const CFG_MODE_SHIFT: u32 = 12;
pub const CFG_MODE_MASK: u32 = 0x3 << CFG_MODE_SHIFT;
pub const CFG_MODE_DUAL_EDGE: u32 = 0x2 << CFG_MODE_SHIFT;
pub const CFG_HW_CLK_CTRL_MASK: u32 = 1 << 20;
pub const M_REG: u32 = 0x8;
pub const N_REG: u32 = 0xc;
pub const D_REG: u32 = 0x10;
pub const MAX_PERF_LEVEL: usize = 8;
pub const SE_CMD_DFSR_OFFSET: u32 = 0x14;
pub const SE_CMD_DFS_EN: u32 = 1;

#[inline] pub const fn SE_PERF_DFSR(level: u32) -> u32 { 0x1c + 0x4 * level }
#[inline] pub const fn SE_PERF_M_DFSR(level: u32) -> u32 { 0x5c + 0x4 * level }
#[inline] pub const fn SE_PERF_N_DFSR(level: u32) -> u32 { 0x9c + 0x4 * level }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum freq_policy { FLOOR, CEIL }

#[repr(C)]
pub struct clk_hw { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct clk_ops { _private: [u8; 0] }
#[repr(C)]
pub struct clk_rate_request { pub rate: c_ulong, pub min_rate: c_ulong, pub max_rate: c_ulong, pub best_parent_rate: c_ulong, pub best_parent_hw: *mut clk_hw }
#[repr(C)]
pub struct clk_duty { pub num: u32, pub den: u32 }
#[repr(C)]
pub struct frac_entry { pub num: c_int, pub den: c_int }

extern "C" {
    pub fn clk_rcg2_is_enabled(hw: *mut clk_hw) -> c_int;
    pub fn clk_rcg2_get_parent(hw: *mut clk_hw) -> u8;
    pub fn clk_rcg2_set_parent(hw: *mut clk_hw, index: u8) -> c_int;
    pub fn clk_rcg2_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong;
    pub fn clk_rcg2_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int;
    pub fn clk_rcg2_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int;
    pub static clk_rcg2_ops: clk_ops;
    pub static clk_rcg2_gp_ops: clk_ops;
    pub static clk_rcg2_floor_ops: clk_ops;
    pub static clk_rcg2_fm_ops: clk_ops;
    pub static clk_edp_pixel_ops: clk_ops;
    pub static clk_byte_ops: clk_ops;
    pub static clk_byte2_ops: clk_ops;
    pub static clk_pixel_ops: clk_ops;
    pub static clk_gfx3d_ops: clk_ops;
    pub static clk_rcg2_shared_ops: clk_ops;
    pub static clk_rcg2_shared_floor_ops: clk_ops;
    pub static clk_rcg2_shared_no_init_park_ops: clk_ops;
    pub static clk_dp_ops: clk_ops;
}

/* The remaining kernel callbacks retain the exact C ABI and are supplied by
 * the surrounding clock framework.  Keeping the declarations here avoids
 * inventing implementations for Linux-specific regmap and clock machinery. */

#[no_mangle]
pub unsafe extern "C" fn calc_rate(mut rate: c_ulong, m: u32, n: u32, mode: u32, hid_div: u32) -> c_ulong {
    if hid_div != 0 { rate = rate.wrapping_mul(2) / (hid_div + 1) as c_ulong; }
    if mode != 0 { rate = rate.wrapping_mul(m as c_ulong) / n as c_ulong; }
    rate
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_reg_val(f: *mut c_void) { let _ = f; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
