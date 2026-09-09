/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions.
 */

// Translated from dce_clock_source.c.  Types, register helpers, and services
// supplied by the surrounding display-core implementation remain external.

const FRACT_FB_DIVIDER_DEC_POINTS_MAX_NUM: u32 = 6;
const CALC_PLL_CLK_SRC_ERR_TOLERANCE: u32 = 1;
const MAX_PLL_CALC_ERROR: u32 = 0xffff_ffff;

unsafe fn get_ss_data_entry(
    clk_src: *mut dce110_clk_src, signal: signal_type, pix_clk_khz: u32,
) -> *mut spread_spectrum_data {
    let (mut p, n) = match signal {
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK =>
            ((*clk_src).dvi_ss_params, (*clk_src).dvi_ss_params_cnt),
        SIGNAL_TYPE_HDMI_TYPE_A => ((*clk_src).hdmi_ss_params, (*clk_src).hdmi_ss_params_cnt),
        SIGNAL_TYPE_LVDS => ((*clk_src).lvds_ss_params, (*clk_src).lvds_ss_params_cnt),
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST |
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_VIRTUAL =>
            ((*clk_src).dp_ss_params, (*clk_src).dp_ss_params_cnt),
        _ => (core::ptr::null_mut(), 0),
    };
    if p.is_null() { return core::ptr::null_mut(); }
    for _ in 0..n {
        if (*p).freq_range_khz >= pix_clk_khz { return p; }
        p = p.add(1);
    }
    core::ptr::null_mut()
}

unsafe fn calculate_fb_and_fractional_fb_divider(
    c: *mut calc_pll_clock_source, target: u32, reference: u32, post: u32,
    fb: *mut u32, fract: *mut u32,
) -> bool {
    let mut v = target as u64 * reference as u64 * post as u64;
    v *= 10;
    v *= (*c).fract_fb_divider_factor as u64;
    v = div_u64(v, (*c).ref_freq_khz as u64 * 10);
    v += 5 * (*c).fract_fb_divider_precision_factor as u64;
    v = div_u64(v, (*c).fract_fb_divider_precision_factor as u64 * 10);
    v *= (*c).fract_fb_divider_precision_factor as u64;
    *fb = div_u64_rem(v, (*c).fract_fb_divider_factor as u64, fract) as u32;
    *fb != 0
}

unsafe fn calc_fb_divider_checking_tolerance(c: *mut calc_pll_clock_source,
    s: *mut pll_settings, reference: u32, post: u32, tolerance: u32) -> bool {
    let (mut fb, mut fract) = (0, 0);
    calculate_fb_and_fractional_fb_divider(c, (*s).adjusted_pix_clk_100hz,
        reference, post, &mut fb, &mut fract);
    let mut actual = (fb as u64 * (*c).fract_fb_divider_factor as u64 + fract as u64)
        * (*c).ref_freq_khz as u64 * 10;
    actual = div_u64(actual, reference as u64 * post as u64 * (*c).fract_fb_divider_factor as u64);
    let a = actual as u32;
    let err = if a > (*s).adjusted_pix_clk_100hz { a - (*s).adjusted_pix_clk_100hz }
              else { (*s).adjusted_pix_clk_100hz - a };
    if err <= tolerance {
        (*s).reference_freq = (*c).ref_freq_khz;
        (*s).reference_divider = reference; (*s).feedback_divider = fb;
        (*s).fract_feedback_divider = fract; (*s).pix_clk_post_divider = post;
        (*s).calculated_pix_clk_100hz = actual as u32;
        (*s).vco_freq = div_u64(actual * post as u64, 10) as u32;
        true
    } else { false }
}

unsafe fn calc_pll_dividers_in_range(c: *mut calc_pll_clock_source, s: *mut pll_settings,
    min_ref: u32, max_ref: u32, min_post: u32, max_post: u32, error: u32) -> bool {
    let mut tolerance = (*s).adjusted_pix_clk_100hz * error / 100000;
    if tolerance < CALC_PLL_CLK_SRC_ERR_TOLERANCE { tolerance = CALC_PLL_CLK_SRC_ERR_TOLERANCE; }
    let mut post = max_post;
    while post >= min_post {
        let mut reference = min_ref;
        while reference <= max_ref {
            if calc_fb_divider_checking_tolerance(c, s, reference, post, tolerance) { return true; }
            reference += 1;
        }
        if post == 0 { break; } post -= 1;
    }
    false
}

// The following interfaces retain the C implementation's ABI and ownership
// conventions; their definitions use the external display-core structures.
unsafe extern "C" {
    fn dce110_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dce112_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn20_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn3_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn31_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn401_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn50_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
    fn dcn301_clk_src_construct(clk_src: *mut dce110_clk_src, ctx: *mut dc_context,
        bios: *mut dc_bios, id: clock_source_id, regs: *const dce110_clk_src_regs,
        shift: *const dce110_clk_src_shift, mask: *const dce110_clk_src_mask) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
