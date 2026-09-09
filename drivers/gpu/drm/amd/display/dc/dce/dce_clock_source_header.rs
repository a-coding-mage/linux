/* Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency: ../inc/clock_source.h

macro_rules! TO_DCE110_CLK_SRC { ($clk_src:expr) => { container_of!($clk_src, dce110_clk_src, base) }; }
macro_rules! CS_COMMON_REG_LIST_DCE_100_110 { ($id:expr) => { SRI!(RESYNC_CNTL, PIXCLK, $id), SRI!(PLL_CNTL, BPHYC_PLL, $id) }; }
macro_rules! CS_COMMON_REG_LIST_DCE_80 { ($id:expr) => { SRI!(RESYNC_CNTL, PIXCLK, $id), SRI!(PLL_CNTL, DCCG_PLL, $id) }; }
macro_rules! CS_COMMON_REG_LIST_DCE_112 { ($id:expr) => { SRI!(PIXCLK_RESYNC_CNTL, PHYPLL, $id) }; }
macro_rules! CS_SF { ($reg:ident, $field:ident, $post_fix:ident) => { .$field = concat_idents!($reg, __, $field, $post_fix) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCE_COMMON_BASE { ($m:expr) => { CS_SF!(PLL_CNTL, PLL_REF_DIV_SRC, $m), CS_SF!(PIXCLK1_RESYNC_CNTL, DCCG_DEEP_COLOR_CNTL1, $m), CS_SF!(PLL_POST_DIV, PLL_POST_DIV_PIXCLK, $m), CS_SF!(PLL_REF_DIV, PLL_REF_DIV, $m) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCE_112 { ($m:expr) => { CS_SF!(PHYPLLA_PIXCLK_RESYNC_CNTL, PHYPLLA_DCCG_DEEP_COLOR_CNTL, $m), CS_SF!(PHYPLLA_PIXCLK_RESYNC_CNTL, PHYPLLA_PIXCLK_DOUBLE_RATE_ENABLE, $m) }; }

// Register-list macros are retained as source-level declarations; SRI/SRII are supplied by dependencies.
macro_rules! CS_COMMON_REG_LIST_DCN2_0 { ($index:expr, $pllid:expr) => { SRI!(PIXCLK_RESYNC_CNTL, PHYPLL, $pllid), $(SRII!(PHASE, DP_DTO, $index)),* }; }
macro_rules! CS_COMMON_REG_LIST_DCN201 { ($index:expr, $pllid:expr) => { SRI!(PIXCLK_RESYNC_CNTL, PHYPLL, $pllid), SRII!(PHASE, DP_DTO, 0), SRII!(PHASE, DP_DTO, 1), SRII!(MODULO, DP_DTO, 0), SRII!(MODULO, DP_DTO, 1), SRII!(PIXEL_RATE_CNTL, OTG, 0), SRII!(PIXEL_RATE_CNTL, OTG, 1) }; }
macro_rules! CS_COMMON_REG_LIST_DCN2_1 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN201!($index, $pllid) }; }
macro_rules! CS_COMMON_REG_LIST_DCN3_0 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN2_1!($index, $pllid) }; }
macro_rules! CS_COMMON_REG_LIST_DCN3_01 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN3_0!($index, $pllid) }; }
macro_rules! CS_COMMON_REG_LIST_DCN3_02 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN3_0!($index, $pllid) }; }
macro_rules! CS_COMMON_REG_LIST_DCN3_03 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN201!($index, $pllid) }; }
macro_rules! CS_COMMON_REG_LIST_DCN1_0 { ($index:expr, $pllid:expr) => { CS_COMMON_REG_LIST_DCN2_1!($index, $pllid) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCN2_0 { ($m:expr) => { CS_SF!(DP_DTO0_PHASE, DP_DTO0_PHASE, $m), CS_SF!(DP_DTO0_MODULO, DP_DTO0_MODULO, $m), CS_SF!(PHYPLLA_PIXCLK_RESYNC_CNTL, PHYPLLA_DCCG_DEEP_COLOR_CNTL, $m), CS_SF!(OTG0_PIXEL_RATE_CNTL, DP_DTO0_ENABLE, $m) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCN3_1_4 { ($m:expr) => { CS_COMMON_MASK_SH_LIST_DCN2_0!($m), CS_SF!(OTG0_PIXEL_RATE_CNTL, PIPE0_DTO_SRC_SEL, $m) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCN3_2 { ($m:expr) => { CS_COMMON_MASK_SH_LIST_DCN2_0!($m), CS_SF!(OTG0_PIXEL_RATE_CNTL, PIPE0_DTO_SRC_SEL, $m) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCN4_0_1 { ($m:expr) => { CS_COMMON_MASK_SH_LIST_DCN3_2!($m), CS_SF!(OTG_PIXEL_RATE_DIV, OTG0_TMDS_PIXEL_RATE_DIV, $m), CS_SF!(OTG_PIXEL_RATE_DIV, DPDTO0_INT, $m), CS_SF!(OTG_PIXEL_RATE_DIV, OTG1_TMDS_PIXEL_RATE_DIV, $m), CS_SF!(OTG_PIXEL_RATE_DIV, DPDTO1_INT, $m), CS_SF!(OTG_PIXEL_RATE_DIV, OTG2_TMDS_PIXEL_RATE_DIV, $m), CS_SF!(OTG_PIXEL_RATE_DIV, DPDTO2_INT, $m), CS_SF!(OTG_PIXEL_RATE_DIV, OTG3_TMDS_PIXEL_RATE_DIV, $m), CS_SF!(OTG_PIXEL_RATE_DIV, DPDTO3_INT, $m) }; }
macro_rules! CS_COMMON_MASK_SH_LIST_DCN1_0 { ($m:expr) => { CS_COMMON_MASK_SH_LIST_DCN2_0!($m) }; }

#[repr(C)] pub struct dce110_clk_src_shift { pub PLL_REF_DIV_SRC: u8, pub DCCG_DEEP_COLOR_CNTL1: u8, pub PHYPLLA_DCCG_DEEP_COLOR_CNTL: u8, pub PHYPLLA_PIXCLK_DOUBLE_RATE_ENABLE: u8, pub PLL_POST_DIV_PIXCLK: u8, pub PLL_REF_DIV: u8, pub DP_DTO0_PHASE: u8, pub DP_DTO0_MODULO: u8, pub DP_DTO0_ENABLE: u8, pub PIPE0_DTO_SRC_SEL: u8, pub DPDTO0_INT: u8, pub DPDTO1_INT: u8, pub DPDTO2_INT: u8, pub DPDTO3_INT: u8, pub OTG0_TMDS_PIXEL_RATE_DIV: u8, pub OTG1_TMDS_PIXEL_RATE_DIV: u8, pub OTG2_TMDS_PIXEL_RATE_DIV: u8, pub OTG3_TMDS_PIXEL_RATE_DIV: u8 }
#[repr(C)] pub struct dce110_clk_src_mask { pub PLL_REF_DIV_SRC: u32, pub DCCG_DEEP_COLOR_CNTL1: u32, pub PHYPLLA_DCCG_DEEP_COLOR_CNTL: u32, pub PHYPLLA_PIXCLK_DOUBLE_RATE_ENABLE: u32, pub PLL_POST_DIV_PIXCLK: u32, pub PLL_REF_DIV: u32, pub DP_DTO0_PHASE: u32, pub DP_DTO0_MODULO: u32, pub DP_DTO0_ENABLE: u32, pub PIPE0_DTO_SRC_SEL: u32, pub DPDTO0_INT: u32, pub DPDTO1_INT: u32, pub DPDTO2_INT: u32, pub DPDTO3_INT: u32, pub OTG0_TMDS_PIXEL_RATE_DIV: u32, pub OTG1_TMDS_PIXEL_RATE_DIV: u32, pub OTG2_TMDS_PIXEL_RATE_DIV: u32, pub OTG3_TMDS_PIXEL_RATE_DIV: u32 }
#[repr(C)] pub struct dce110_clk_src_regs { pub RESYNC_CNTL: u32, pub PIXCLK_RESYNC_CNTL: u32, pub PLL_CNTL: u32, pub OTG_PIXEL_RATE_DIV: u32, pub PHASE: [u32; MAX_PIPES], pub MODULO: [u32; MAX_PIPES], pub PIXEL_RATE_CNTL: [u32; MAX_PIPES] }

#[repr(C)] pub struct dce110_clk_src { pub base: clock_source, pub regs: *const dce110_clk_src_regs, pub cs_mask: *const dce110_clk_src_mask, pub cs_shift: *const dce110_clk_src_shift, pub bios: *mut dc_bios, pub dp_ss_params: *mut spread_spectrum_data, pub dp_ss_params_cnt: u32, pub hdmi_ss_params: *mut spread_spectrum_data, pub hdmi_ss_params_cnt: u32, pub dvi_ss_params: *mut spread_spectrum_data, pub dvi_ss_params_cnt: u32, pub lvds_ss_params: *mut spread_spectrum_data, pub lvds_ss_params_cnt: u32, pub ext_clk_khz: u32, pub ref_freq_khz: u32, pub calc_pll: calc_pll_clock_source, pub calc_pll_hdmi: calc_pll_clock_source }

extern "C" {
    pub fn dce110_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dce112_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn20_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn3_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn301_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn31_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn401_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub fn dcn50_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context, bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs, cs_shift: *const dce110_clk_src_shift, cs_mask: *const dce110_clk_src_mask) -> bool;
    pub static pixel_rate_range_table_entry: [pixel_rate_range_table_entry; 0];
    pub fn look_up_in_video_optimized_rate_tlb(pixel_rate_khz: c_uint) -> *const pixel_rate_range_table_entry;
}

#[repr(C)] pub struct pixel_rate_range_table_entry { pub range_min_khz: c_uint, pub range_max_khz: c_uint, pub target_pixel_rate_khz: c_uint, pub mult_factor: c_ushort, pub div_factor: c_ushort }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
