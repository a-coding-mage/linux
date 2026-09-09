/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const HAVE_RST_BAR: u32 = 1 << 0;
pub const PLL_AO: u32 = 1 << 1;
pub const PLL_PARENT_EN: u32 = 1 << 2;
pub const POSTDIV_MASK: u32 = (1 << 3) - 1;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct clk_ops;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw_onecell_data;

#[repr(C)]
pub struct clk_rate_request;

#[repr(C)]
pub struct mtk_pll_div_table {
    pub div: u32,
    pub freq: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct mtk_pll_data {
    pub id: i32,
    pub name: *const ::core::ffi::c_char,
    pub reg: u32,
    pub pwr_reg: u32,
    pub en_mask: u32,
    pub fenc_sta_ofs: u32,
    pub pd_reg: u32,
    pub tuner_reg: u32,
    pub tuner_en_reg: u32,
    pub tuner_en_bit: u8,
    pub pd_shift: i32,
    pub flags: u32,
    pub ops: *const clk_ops,
    pub rst_bar_mask: u32,
    pub fmin: ::core::ffi::c_ulong,
    pub fmax: ::core::ffi::c_ulong,
    pub pcwbits: i32,
    pub pcwibits: i32,
    pub pcw_reg: u32,
    pub pcw_shift: i32,
    pub pcw_chg_reg: u32,
    pub div_table: *const mtk_pll_div_table,
    pub parent_name: *const ::core::ffi::c_char,
    pub en_reg: u32,
    pub en_set_reg: u32,
    pub en_clr_reg: u32,
    pub pll_en_bit: u8, /* Assume 0, indicates BIT(0) by default */
    pub pcw_chg_bit: u8,
    pub fenc_sta_bit: u8,
}

/*
 * MediaTek PLLs are configured through their pcw value. The pcw value describes
 * a divider in the PLL feedback loop which consists of 7 bits for the integer
 * part and the remaining bits (if present) for the fractional part. Also they
 * have a 3 bit power-of-two post divider.
 */
#[repr(C)]
pub struct mtk_clk_pll {
    pub dev: *mut device,
    pub hw: clk_hw,
    pub base_addr: *mut ::core::ffi::c_void,
    pub pd_addr: *mut ::core::ffi::c_void,
    pub pwr_addr: *mut ::core::ffi::c_void,
    pub tuner_addr: *mut ::core::ffi::c_void,
    pub tuner_en_addr: *mut ::core::ffi::c_void,
    pub pcw_addr: *mut ::core::ffi::c_void,
    pub pcw_chg_addr: *mut ::core::ffi::c_void,
    pub en_addr: *mut ::core::ffi::c_void,
    pub en_set_addr: *mut ::core::ffi::c_void,
    pub en_clr_addr: *mut ::core::ffi::c_void,
    pub fenc_addr: *mut ::core::ffi::c_void,
    pub data: *const mtk_pll_data,
}

extern "C" {
    pub fn mtk_clk_register_plls(dev: *mut device, plls: *const mtk_pll_data,
                                 num_plls: i32, clk_data: *mut clk_hw_onecell_data) -> i32;
    pub fn mtk_clk_unregister_plls(plls: *const mtk_pll_data, num_plls: i32,
                                   clk_data: *mut clk_hw_onecell_data);
    pub static mtk_pll_ops: clk_ops;
    pub static mtk_pll_fenc_clr_set_ops: clk_ops;
    pub fn mtk_pll_is_prepared(hw: *mut clk_hw) -> i32;
    pub fn mtk_pll_prepare(hw: *mut clk_hw) -> i32;
    pub fn mtk_pll_unprepare(hw: *mut clk_hw);
    pub fn mtk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn mtk_pll_calc_values(pll: *mut mtk_clk_pll, pcw: *mut u32, postdiv: *mut u32,
                               freq: u32, fin: u32);
    pub fn mtk_pll_set_rate(hw: *mut clk_hw, rate: ::core::ffi::c_ulong,
                            parent_rate: ::core::ffi::c_ulong) -> i32;
    pub fn mtk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32;
    pub fn mtk_clk_register_pll_ops(pll: *mut mtk_clk_pll, data: *const mtk_pll_data,
                                    base: *mut ::core::ffi::c_void, pll_ops: *const clk_ops) -> *mut clk_hw;
    pub fn mtk_clk_register_pll(dev: *mut device, data: *const mtk_pll_data,
                                base: *mut ::core::ffi::c_void) -> *mut clk_hw;
    pub fn mtk_clk_unregister_pll(hw: *mut clk_hw);
    pub fn mtk_clk_pll_get_base(hw: *mut clk_hw, data: *const mtk_pll_data) -> *mut ::core::ffi::c_void;
}

#[inline]
pub unsafe fn to_mtk_clk_pll(hw: *mut clk_hw) -> *mut mtk_clk_pll {
    (hw as *mut u8).sub(::core::mem::offset_of!(mtk_clk_pll, hw)) as *mut mtk_clk_pll
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
