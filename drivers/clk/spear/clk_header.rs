/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Clock framework definitions for SPEAr platform
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// Dependencies supplied by the Linux clock, spinlock, and type headers.
use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

/* Auxiliary Synth clk */
/* Default masks */
pub const AUX_EQ_SEL_SHIFT: u32 = 30;
pub const AUX_EQ_SEL_MASK: u32 = 1;
pub const AUX_EQ1_SEL: u32 = 0;
pub const AUX_EQ2_SEL: u32 = 1;
pub const AUX_XSCALE_SHIFT: u32 = 16;
pub const AUX_XSCALE_MASK: u32 = 0xFFF;
pub const AUX_YSCALE_SHIFT: u32 = 0;
pub const AUX_YSCALE_MASK: u32 = 0xFFF;
pub const AUX_SYNT_ENB: u32 = 31;

#[repr(C)]
pub struct aux_clk_masks {
    pub eq_sel_mask: u32,
    pub eq_sel_shift: u32,
    pub eq1_mask: u32,
    pub eq2_mask: u32,
    pub xscale_sel_mask: u32,
    pub xscale_sel_shift: u32,
    pub yscale_sel_mask: u32,
    pub yscale_sel_shift: u32,
    pub enable_bit: u32,
}

#[repr(C)]
pub struct aux_rate_tbl {
    pub xscale: u16,
    pub yscale: u16,
    pub eq: u8,
}

#[repr(C)]
pub struct clk_aux {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub masks: *const aux_clk_masks,
    pub rtbl: *mut aux_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

/* Fractional Synth clk */
#[repr(C)]
pub struct frac_rate_tbl {
    pub div: u32,
}

#[repr(C)]
pub struct clk_frac {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub rtbl: *mut frac_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

/* GPT clk */
#[repr(C)]
pub struct gpt_rate_tbl {
    pub mscale: u16,
    pub nscale: u16,
}

#[repr(C)]
pub struct clk_gpt {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub rtbl: *mut gpt_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

/* VCO-PLL clk */
#[repr(C)]
pub struct pll_rate_tbl {
    pub mode: u8,
    pub m: u16,
    pub n: u8,
    pub p: u8,
}

#[repr(C)]
pub struct clk_vco {
    pub hw: clk_hw,
    pub mode_reg: *mut c_void,
    pub cfg_reg: *mut c_void,
    pub rtbl: *mut pll_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub vco: *mut clk_vco,
    pub parent: [*const c_char; 1],
    pub lock: *mut spinlock_t,
}

pub type clk_calc_rate = Option<unsafe extern "C" fn(
    hw: *mut clk_hw,
    prate: c_ulong,
    index: c_int,
) -> c_ulong>;

/* clk register routines */
extern "C" {
    pub fn clk_register_aux(
        aux_name: *const c_char,
        gate_name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        masks: *const aux_clk_masks,
        rtbl: *mut aux_rate_tbl,
        rtbl_cnt: u8,
        lock: *mut spinlock_t,
        gate_clk: *mut *mut clk,
    ) -> *mut clk;

    pub fn clk_register_frac(
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        rtbl: *mut frac_rate_tbl,
        rtbl_cnt: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;

    pub fn clk_register_gpt(
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        rtbl: *mut gpt_rate_tbl,
        rtbl_cnt: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;

    pub fn clk_register_vco_pll(
        vco_name: *const c_char,
        pll_name: *const c_char,
        vco_gate_name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        mode_reg: *mut c_void,
        cfg_reg: *mut c_void,
        rtbl: *mut pll_rate_tbl,
        rtbl_cnt: u8,
        lock: *mut spinlock_t,
        pll_clk: *mut *mut clk,
        vco_gate_clk: *mut *mut clk,
    ) -> *mut clk;

    pub fn clk_round_rate_index(
        hw: *mut clk_hw,
        drate: c_ulong,
        parent_rate: c_ulong,
        calc_rate: clk_calc_rate,
        rtbl_cnt: u8,
        index: *mut c_int,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
