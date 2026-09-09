/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Ingenic SoC CGU driver
 *
 * Copyright (c) 2013-2015 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Linux dependencies supplied by other translated modules.

#[repr(C)]
pub struct ingenic_cgu_pll_info {
    pub reg: core::ffi::c_uint,
    pub rate_multiplier: core::ffi::c_uint,
    pub od_encoding: *const i8,
    pub m_shift: u8,
    pub m_bits: u8,
    pub m_offset: u8,
    pub n_shift: u8,
    pub n_bits: u8,
    pub n_offset: u8,
    pub od_shift: u8,
    pub od_bits: u8,
    pub od_max: u8,
    pub bypass_reg: core::ffi::c_uint,
    pub bypass_bit: i8,
    pub enable_bit: i8,
    pub stable_bit: i8,
    pub calc_m_n_od: Option<unsafe extern "C" fn(
        pll_info: *const ingenic_cgu_pll_info,
        rate: core::ffi::c_ulong,
        parent_rate: core::ffi::c_ulong,
        m: *mut core::ffi::c_uint,
        n: *mut core::ffi::c_uint,
        od: *mut core::ffi::c_uint,
    )>,
    pub set_rate_hook: Option<unsafe extern "C" fn(
        pll_info: *const ingenic_cgu_pll_info,
        rate: core::ffi::c_ulong,
        parent_rate: core::ffi::c_ulong,
    )>,
}

#[repr(C)]
pub struct ingenic_cgu_mux_info {
    pub reg: core::ffi::c_uint,
    pub shift: u8,
    pub bits: u8,
}

#[repr(C)]
pub struct ingenic_cgu_div_info {
    pub reg: core::ffi::c_uint,
    pub shift: u8,
    pub div: u8,
    pub bits: u8,
    pub ce_bit: i8,
    pub busy_bit: i8,
    pub stop_bit: i8,
    pub bypass_mask: u8,
    pub div_table: *const u8,
}

#[repr(C)]
pub struct ingenic_cgu_fixdiv_info {
    pub div: core::ffi::c_uint,
}

#[repr(C)]
pub struct ingenic_cgu_gate_info {
    pub reg: core::ffi::c_uint,
    pub bit: u8,
    pub clear_to_gate: bool,
    pub delay_us: u16,
}

#[repr(C)]
pub struct ingenic_cgu_custom_info {
    pub clk_ops: *const clk_ops,
}

pub const CGU_CLK_NONE: core::ffi::c_uint = 0;
pub const CGU_CLK_EXT: core::ffi::c_uint = 1 << 0;
pub const CGU_CLK_PLL: core::ffi::c_uint = 1 << 1;
pub const CGU_CLK_GATE: core::ffi::c_uint = 1 << 2;
pub const CGU_CLK_MUX: core::ffi::c_uint = 1 << 3;
pub const CGU_CLK_MUX_GLITCHFREE: core::ffi::c_uint = 1 << 4;
pub const CGU_CLK_DIV: core::ffi::c_uint = 1 << 5;
pub const CGU_CLK_FIXDIV: core::ffi::c_uint = 1 << 6;
pub const CGU_CLK_CUSTOM: core::ffi::c_uint = 1 << 7;

#[repr(C)]
pub struct ingenic_cgu_clk_info_fixed {
    pub gate: ingenic_cgu_gate_info,
    pub mux: ingenic_cgu_mux_info,
    pub div: ingenic_cgu_div_info,
    pub fixdiv: ingenic_cgu_fixdiv_info,
}

#[repr(C)]
pub union ingenic_cgu_clk_info_union {
    pub pll: ingenic_cgu_pll_info,
    pub fixed: ingenic_cgu_clk_info_fixed,
    pub custom: ingenic_cgu_custom_info,
}

#[repr(C)]
pub struct ingenic_cgu_clk_info {
    pub name: *const core::ffi::c_char,
    pub r#type: core::ffi::c_uint,
    pub flags: core::ffi::c_ulong,
    pub parents: [core::ffi::c_int; 4],
    pub data: ingenic_cgu_clk_info_union,
}

#[repr(C)]
pub struct ingenic_cgu {
    pub np: *mut device_node,
    pub base: *mut core::ffi::c_void,
    pub clock_info: *const ingenic_cgu_clk_info,
    pub clocks: clk_onecell_data,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct ingenic_clk {
    pub hw: clk_hw,
    pub cgu: *mut ingenic_cgu,
    pub idx: core::ffi::c_uint,
}

// Equivalent of container_of(_hw, struct ingenic_clk, hw).
#[macro_export]
macro_rules! to_ingenic_clk {
    ($hw:expr) => {{
        ($hw as *mut u8).wrapping_sub(core::mem::offset_of!(ingenic_clk, hw)) as *mut ingenic_clk
    }};
}

extern "C" {
    pub fn ingenic_cgu_new(
        clock_info: *const ingenic_cgu_clk_info,
        num_clocks: core::ffi::c_uint,
        np: *mut device_node,
    ) -> *mut ingenic_cgu;

    pub fn ingenic_cgu_register_clocks(cgu: *mut ingenic_cgu) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
