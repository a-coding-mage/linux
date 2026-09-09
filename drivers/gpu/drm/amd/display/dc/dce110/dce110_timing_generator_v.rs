/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 */

// Dependencies supplied by the surrounding DCE11 implementation.

static unsafe fn dce110_timing_generator_v_enable_crtc(tg: *mut timing_generator) -> bool {
    let mut value: u32 = 0;
    set_reg_field_value(value, 0, CRTCV_MASTER_UPDATE_MODE, MASTER_UPDATE_MODE);
    dm_write_reg((*(*tg).ctx), mmCRTCV_MASTER_UPDATE_MODE, value);
    value = 0;
    dm_write_reg((*(*tg).ctx), mmCRTCV_MASTER_UPDATE_MODE, value);
    value = 0;
    set_reg_field_value(value, 1, CRTCV_MASTER_EN, CRTC_MASTER_EN);
    dm_write_reg((*(*tg).ctx), mmCRTCV_MASTER_EN, value);
    true
}

static unsafe fn dce110_timing_generator_v_disable_crtc(tg: *mut timing_generator) -> bool {
    let mut value = dm_read_reg((*(*tg).ctx), mmCRTCV_CONTROL);
    set_reg_field_value(value, 0, CRTCV_CONTROL, CRTC_DISABLE_POINT_CNTL);
    set_reg_field_value(value, 0, CRTCV_CONTROL, CRTC_MASTER_EN);
    dm_write_reg((*(*tg).ctx), mmCRTCV_CONTROL, value);
    // TODO: call this when adding stereo support: tg->funcs->disable_stereo(tg);
    true
}

static unsafe fn dce110_timing_generator_v_blank_crtc(tg: *mut timing_generator) {
    let addr = mmCRTCV_BLANK_CONTROL;
    let mut value = dm_read_reg((*(*tg).ctx), addr);
    set_reg_field_value(value, 1, CRTCV_BLANK_CONTROL, CRTC_BLANK_DATA_EN);
    set_reg_field_value(value, 0, CRTCV_BLANK_CONTROL, CRTC_BLANK_DE_MODE);
    dm_write_reg((*(*tg).ctx), addr, value);
}

static unsafe fn dce110_timing_generator_v_unblank_crtc(tg: *mut timing_generator) {
    let addr = mmCRTCV_BLANK_CONTROL;
    let mut value = dm_read_reg((*(*tg).ctx), addr);
    set_reg_field_value(value, 0, CRTCV_BLANK_CONTROL, CRTC_BLANK_DATA_EN);
    set_reg_field_value(value, 0, CRTCV_BLANK_CONTROL, CRTC_BLANK_DE_MODE);
    dm_write_reg((*(*tg).ctx), addr, value);
}

static unsafe fn dce110_timing_generator_v_is_in_vertical_blank(tg: *mut timing_generator) -> bool {
    let value = dm_read_reg((*(*tg).ctx), mmCRTCV_STATUS);
    get_reg_field_value(value, CRTCV_STATUS, CRTC_V_BLANK) == 1
}

static unsafe fn dce110_timing_generator_v_is_counter_moving(tg: *mut timing_generator) -> bool {
    let value = dm_read_reg((*(*tg).ctx), mmCRTCV_STATUS_POSITION);
    let h1 = get_reg_field_value(value, CRTCV_STATUS_POSITION, CRTC_HORZ_COUNT);
    let v1 = get_reg_field_value(value, CRTCV_STATUS_POSITION, CRTC_VERT_COUNT);
    let value = dm_read_reg((*(*tg).ctx), mmCRTCV_STATUS_POSITION);
    let h2 = get_reg_field_value(value, CRTCV_STATUS_POSITION, CRTC_HORZ_COUNT);
    let v2 = get_reg_field_value(value, CRTCV_STATUS_POSITION, CRTC_VERT_COUNT);
    !(h1 == h2 && v1 == v2)
}

static unsafe fn dce110_timing_generator_v_wait_for_vblank(tg: *mut timing_generator) {
    while dce110_timing_generator_v_is_in_vertical_blank(tg) {
        if !dce110_timing_generator_v_is_counter_moving(tg) { break; }
    }
    while !dce110_timing_generator_v_is_in_vertical_blank(tg) {
        if !dce110_timing_generator_v_is_counter_moving(tg) { break; }
    }
}

static unsafe fn dce110_timing_generator_v_wait_for_vactive(tg: *mut timing_generator) {
    while dce110_timing_generator_v_is_in_vertical_blank(tg) {
        if !dce110_timing_generator_v_is_counter_moving(tg) { break; }
    }
}

static unsafe fn dce110_timing_generator_v_wait_for_state(tg: *mut timing_generator, state: crtc_state) {
    match state {
        CRTC_STATE_VBLANK => dce110_timing_generator_v_wait_for_vblank(tg),
        CRTC_STATE_VACTIVE => dce110_timing_generator_v_wait_for_vactive(tg),
        _ => (),
    }
}

static unsafe fn dce110_timing_generator_v_program_blanking(tg: *mut timing_generator, timing: *const dc_crtc_timing) {
    let vsync_offset = (*timing).v_border_bottom + (*timing).v_front_porch;
    let v_sync_start = (*timing).v_addressable + vsync_offset;
    let hsync_offset = (*timing).h_border_right + (*timing).h_front_porch;
    let h_sync_start = (*timing).h_addressable + hsync_offset;
    let ctx = (*tg).ctx;
    let mut value: u32;
    let mut addr: u32;
    let mut tmp: u32;
    addr = mmCRTCV_H_TOTAL; value = dm_read_reg(ctx, addr);
    set_reg_field_value(value, (*timing).h_total - 1, CRTCV_H_TOTAL, CRTC_H_TOTAL); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_V_TOTAL; value = dm_read_reg(ctx, addr);
    set_reg_field_value(value, (*timing).v_total - 1, CRTCV_V_TOTAL, CRTC_V_TOTAL); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_H_BLANK_START_END; value = dm_read_reg(ctx, addr);
    tmp = (*timing).h_total - (h_sync_start + (*timing).h_border_left);
    set_reg_field_value(value, tmp, CRTCV_H_BLANK_START_END, CRTC_H_BLANK_END);
    tmp = tmp + (*timing).h_addressable + (*timing).h_border_left + (*timing).h_border_right;
    set_reg_field_value(value, tmp, CRTCV_H_BLANK_START_END, CRTC_H_BLANK_START); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_V_BLANK_START_END; value = dm_read_reg(ctx, addr);
    tmp = (*timing).v_total - (v_sync_start + (*timing).v_border_top);
    set_reg_field_value(value, tmp, CRTCV_V_BLANK_START_END, CRTC_V_BLANK_END);
    tmp = tmp + (*timing).v_addressable + (*timing).v_border_top + (*timing).v_border_bottom;
    set_reg_field_value(value, tmp, CRTCV_V_BLANK_START_END, CRTC_V_BLANK_START); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_H_SYNC_A; value = 0;
    set_reg_field_value(value, (*timing).h_sync_width, CRTCV_H_SYNC_A, CRTC_H_SYNC_A_END); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_H_SYNC_A_CNTL; value = dm_read_reg(ctx, addr);
    set_reg_field_value(value, if (*timing).flags.HSYNC_POSITIVE_POLARITY { 0 } else { 1 }, CRTCV_H_SYNC_A_CNTL, CRTC_H_SYNC_A_POL); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_V_SYNC_A; value = 0;
    set_reg_field_value(value, (*timing).v_sync_width, CRTCV_V_SYNC_A, CRTC_V_SYNC_A_END); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_V_SYNC_A_CNTL; value = dm_read_reg(ctx, addr);
    set_reg_field_value(value, if (*timing).flags.VSYNC_POSITIVE_POLARITY { 0 } else { 1 }, CRTCV_V_SYNC_A_CNTL, CRTC_V_SYNC_A_POL); dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_INTERLACE_CONTROL; value = dm_read_reg(ctx, addr);
    set_reg_field_value(value, (*timing).flags.INTERLACE, CRTCV_INTERLACE_CONTROL, CRTC_INTERLACE_ENABLE); dm_write_reg(ctx, addr, value);
}

static unsafe fn dce110_timing_generator_v_enable_advanced_request(tg: *mut timing_generator, enable: bool, timing: *const dc_crtc_timing) {
    let addr = mmCRTCV_START_LINE_CONTROL;
    let mut value = dm_read_reg((*(*tg).ctx), addr);
    let position = if enable { if (*timing).v_sync_width + (*timing).v_front_porch <= 3 { 3 } else { 4 } } else { 2 };
    let legacy = if enable { 0 } else { 1 };
    set_reg_field_value(value, position, CRTCV_START_LINE_CONTROL, CRTC_ADVANCED_START_LINE_POSITION);
    set_reg_field_value(value, legacy, CRTCV_START_LINE_CONTROL, CRTC_LEGACY_REQUESTOR_EN);
    dm_write_reg((*(*tg).ctx), addr, value);
}

static unsafe fn dce110_timing_generator_v_set_blank(tg: *mut timing_generator, enable_blanking: bool) {
    if enable_blanking { dce110_timing_generator_v_blank_crtc(tg); } else { dce110_timing_generator_v_unblank_crtc(tg); }
}

static unsafe fn dce110_timing_generator_v_program_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing, vready_offset: i32, vstartup_start: i32, vupdate_offset: i32, vupdate_width: i32, pstate_keepout: i32, signal: signal_type, use_vbios: bool) {
    let _ = (vready_offset, vstartup_start, vupdate_offset, vupdate_width, pstate_keepout, signal);
    if use_vbios { dce110_timing_generator_program_timing_generator(tg, timing); } else { dce110_timing_generator_v_program_blanking(tg, timing); }
}

static unsafe fn dce110_timing_generator_v_program_blank_color(tg: *mut timing_generator, black_color: *const tg_color) {
    let addr = mmCRTCV_BLACK_COLOR; let mut value = dm_read_reg((*(*tg).ctx), addr);
    set_reg_field_value(value, (*black_color).color_b_cb, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_B_CB);
    set_reg_field_value(value, (*black_color).color_g_y, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_G_Y);
    set_reg_field_value(value, (*black_color).color_r_cr, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_R_CR);
    dm_write_reg((*(*tg).ctx), addr, value);
}

static unsafe fn dce110_timing_generator_v_set_overscan_color_black(tg: *mut timing_generator, color: *const tg_color) {
    let ctx = (*tg).ctx; let mut value = 0; let mut addr: u32;
    set_reg_field_value(value, (*color).color_b_cb, CRTC_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_BLUE);
    set_reg_field_value(value, (*color).color_r_cr, CRTC_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_RED);
    set_reg_field_value(value, (*color).color_g_y, CRTC_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_GREEN);
    addr = mmCRTCV_OVERSCAN_COLOR; dm_write_reg(ctx, addr, value); addr = mmCRTCV_BLACK_COLOR; dm_write_reg(ctx, addr, value);
    addr = mmCRTCV_BLANK_DATA_COLOR; dm_write_reg(ctx, addr, value);
    // TODO: program EXT registers once the LB DATA format is known.
}

static unsafe fn dce110_tg_v_program_blank_color(tg: *mut timing_generator, black_color: *const tg_color) {
    let addr = mmCRTCV_BLACK_COLOR; let mut value = dm_read_reg((*(*tg).ctx), addr);
    set_reg_field_value(value, (*black_color).color_b_cb, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_B_CB);
    set_reg_field_value(value, (*black_color).color_g_y, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_G_Y);
    set_reg_field_value(value, (*black_color).color_r_cr, CRTCV_BLACK_COLOR, CRTC_BLACK_COLOR_R_CR);
    dm_write_reg((*(*tg).ctx), addr, value); dm_write_reg((*(*tg).ctx), mmCRTCV_BLANK_DATA_COLOR, value);
}

static unsafe fn dce110_timing_generator_v_set_overscan_color(tg: *mut timing_generator, overscan_color: *const tg_color) {
    let ctx = (*tg).ctx; let mut value = 0;
    set_reg_field_value(value, (*overscan_color).color_b_cb, CRTCV_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_BLUE);
    set_reg_field_value(value, (*overscan_color).color_g_y, CRTCV_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_GREEN);
    set_reg_field_value(value, (*overscan_color).color_r_cr, CRTCV_OVERSCAN_COLOR, CRTC_OVERSCAN_COLOR_RED);
    dm_write_reg(ctx, mmCRTCV_OVERSCAN_COLOR, value);
}

static unsafe fn dce110_timing_generator_v_set_colors(tg: *mut timing_generator, blank_color: *const tg_color, overscan_color: *const tg_color) {
    if !blank_color.is_null() { dce110_tg_v_program_blank_color(tg, blank_color); }
    if !overscan_color.is_null() { dce110_timing_generator_v_set_overscan_color(tg, overscan_color); }
}

static unsafe fn dce110_timing_generator_v_set_early_control(tg: *mut timing_generator, early_cntl: u32) {
    let address = mmCRTC_CONTROL; let mut regval = dm_read_reg((*(*tg).ctx), address);
    set_reg_field_value(regval, early_cntl, CRTCV_CONTROL, CRTC_HBLANK_EARLY_CONTROL); dm_write_reg((*(*tg).ctx), address, regval);
}

static unsafe fn dce110_timing_generator_v_get_vblank_counter(tg: *mut timing_generator) -> u32 {
    let value = dm_read_reg((*(*tg).ctx), mmCRTCV_STATUS_FRAME_COUNT);
    get_reg_field_value(value, CRTCV_STATUS_FRAME_COUNT, CRTC_FRAME_COUNT)
}

static unsafe fn dce110_timing_generator_v_did_triggered_reset_occur(tg: *mut timing_generator) -> bool {
    let _ = tg; DC_LOG_ERROR!("Timing Sync not supported on underlay pipe\n"); false
}
static unsafe fn dce110_timing_generator_v_setup_global_swap_lock(tg: *mut timing_generator, gsl_params: *const dcp_gsl_params) { let _ = (tg, gsl_params); DC_LOG_ERROR!("Timing Sync not supported on underlay pipe\n"); }
static unsafe fn dce110_timing_generator_v_enable_reset_trigger(tg: *mut timing_generator, source_tg_inst: i32) { let _ = (tg, source_tg_inst); DC_LOG_ERROR!("Timing Sync not supported on underlay pipe\n"); }
static unsafe fn dce110_timing_generator_v_disable_reset_trigger(tg: *mut timing_generator) { let _ = tg; DC_LOG_ERROR!("Timing Sync not supported on underlay pipe\n"); }
static unsafe fn dce110_timing_generator_v_tear_down_global_swap_lock(tg: *mut timing_generator) { let _ = tg; DC_LOG_ERROR!("Timing Sync not supported on underlay pipe\n"); }
static unsafe fn dce110_timing_generator_v_disable_vga(tg: *mut timing_generator) { let _ = tg; }

static dce110_tg_v_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: dce110_tg_validate_timing,
    program_timing: dce110_timing_generator_v_program_timing,
    enable_crtc: dce110_timing_generator_v_enable_crtc,
    disable_crtc: dce110_timing_generator_v_disable_crtc,
    is_counter_moving: dce110_timing_generator_v_is_counter_moving,
    get_position: None,
    get_frame_count: dce110_timing_generator_v_get_vblank_counter,
    set_early_control: dce110_timing_generator_v_set_early_control,
    wait_for_state: dce110_timing_generator_v_wait_for_state,
    set_blank: dce110_timing_generator_v_set_blank,
    set_colors: dce110_timing_generator_v_set_colors,
    set_overscan_blank_color: dce110_timing_generator_v_set_overscan_color_black,
    set_blank_color: dce110_timing_generator_v_program_blank_color,
    disable_vga: dce110_timing_generator_v_disable_vga,
    did_triggered_reset_occur: dce110_timing_generator_v_did_triggered_reset_occur,
    setup_global_swap_lock: dce110_timing_generator_v_setup_global_swap_lock,
    enable_reset_trigger: dce110_timing_generator_v_enable_reset_trigger,
    disable_reset_trigger: dce110_timing_generator_v_disable_reset_trigger,
    tear_down_global_swap_lock: dce110_timing_generator_v_tear_down_global_swap_lock,
    enable_advanced_request: dce110_timing_generator_v_enable_advanced_request,
    is_two_pixels_per_container: dce110_is_two_pixels_per_container,
};

pub unsafe fn dce110_timing_generator_v_construct(tg110: *mut dce110_timing_generator, ctx: *mut dc_context) {
    (*tg110).controller_id = CONTROLLER_ID_UNDERLAY0;
    (*tg110).base.funcs = &dce110_tg_v_funcs;
    (*tg110).base.ctx = ctx;
    (*tg110).base.bp = (*ctx).dc_bios;
    (*tg110).max_h_total = CRTC_H_TOTAL__CRTC_H_TOTAL_MASK + 1;
    (*tg110).max_v_total = CRTC_V_TOTAL__CRTC_V_TOTAL_MASK + 1;
    (*tg110).min_h_blank = 56;
    (*tg110).min_h_front_porch = 4;
    (*tg110).min_h_back_porch = 4;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
