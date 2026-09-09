/* Translated from dcn10_dpp.c. External types, functions, and register helpers
 * are supplied by the surrounding translated repository. */

pub const NUM_PHASES: u32 = 64;
pub const HORZ_MAX_TAPS: u32 = 8;
pub const VERT_MAX_TAPS: u32 = 8;
pub const BLACK_OFFSET_RGB_Y: u32 = 0x0;
pub const BLACK_OFFSET_CBCR: u32 = 0x8000;

#[repr(C)]
pub enum pixel_format_description { PIXEL_FORMAT_FIXED = 0, PIXEL_FORMAT_FIXED16, PIXEL_FORMAT_FLOAT }
#[repr(C)]
pub enum dcn10_coef_filter_type_sel {
    SCL_COEF_LUMA_VERT_FILTER = 0, SCL_COEF_LUMA_HORZ_FILTER,
    SCL_COEF_CHROMA_VERT_FILTER, SCL_COEF_CHROMA_HORZ_FILTER,
    SCL_COEF_ALPHA_VERT_FILTER, SCL_COEF_ALPHA_HORZ_FILTER,
}
#[repr(C)]
pub enum dscl_autocal_mode {
    AUTOCAL_MODE_OFF = 0, AUTOCAL_MODE_AUTOSCALE = 1,
    AUTOCAL_MODE_AUTOCENTER = 2, AUTOCAL_MODE_AUTOREPLICATE = 3,
}
#[repr(C)]
pub enum dscl_mode_sel {
    DSCL_MODE_SCALING_444_BYPASS = 0, DSCL_MODE_SCALING_444_RGB_ENABLE = 1,
    DSCL_MODE_SCALING_444_YCBCR_ENABLE = 2, DSCL_MODE_SCALING_420_YCBCR_ENABLE = 3,
    DSCL_MODE_SCALING_420_LUMA_BYPASS = 4, DSCL_MODE_SCALING_420_CHROMA_BYPASS = 5,
    DSCL_MODE_DSCL_BYPASS = 6,
}

pub unsafe fn dpp_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state) {
    let dpp = TO_DCN10_DPP(dpp_base);
    REG_GET!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, &mut (*s).is_enabled);
    REG_GET!(dpp, CM_IGAM_CONTROL, CM_IGAM_LUT_MODE, &mut (*s).igam_lut_mode);
    REG_GET!(dpp, CM_IGAM_CONTROL, CM_IGAM_INPUT_FORMAT, &mut (*s).igam_input_format);
    REG_GET!(dpp, CM_DGAM_CONTROL, CM_DGAM_LUT_MODE, &mut (*s).dgam_lut_mode);
    REG_GET!(dpp, CM_RGAM_CONTROL, CM_RGAM_LUT_MODE, &mut (*s).rgam_lut_mode);
    REG_GET!(dpp, CM_GAMUT_REMAP_CONTROL, CM_GAMUT_REMAP_MODE, &mut (*s).gamut_remap_mode);
    if (*s).gamut_remap_mode != 0 {
        (*s).gamut_remap_c11_c12 = REG_READ!(dpp, CM_GAMUT_REMAP_C11_C12);
        (*s).gamut_remap_c13_c14 = REG_READ!(dpp, CM_GAMUT_REMAP_C13_C14);
        (*s).gamut_remap_c21_c22 = REG_READ!(dpp, CM_GAMUT_REMAP_C21_C22);
        (*s).gamut_remap_c23_c24 = REG_READ!(dpp, CM_GAMUT_REMAP_C23_C24);
        (*s).gamut_remap_c31_c32 = REG_READ!(dpp, CM_GAMUT_REMAP_C31_C32);
        (*s).gamut_remap_c33_c34 = REG_READ!(dpp, CM_GAMUT_REMAP_C33_C34);
    }
}

pub unsafe fn dpp1_get_optimal_number_of_taps(dpp: *mut dpp, scl_data: *mut scaler_data, in_taps: *const scaling_taps) -> bool {
    if (*scl_data).format == PIXEL_FORMAT_FP16 && (*dpp).caps.dscl_data_proc_format == DSCL_DATA_PRCESSING_FIXED_FORMAT && (*scl_data).ratios.horz.value != dc_fixpt_one.value && (*scl_data).ratios.vert.value != dc_fixpt_one.value { return false; }
    if (*scl_data).viewport.width > (*scl_data).h_active && (*dpp).ctx.dc.debug.max_downscale_src_width != 0 && (*scl_data).viewport.width > (*dpp).ctx.dc.debug.max_downscale_src_width { return false; }
    if (*scl_data).ratios.horz.value == 4i64 << 32 { (*scl_data).ratios.horz.value -= 1; }
    if (*scl_data).ratios.vert.value == 4i64 << 32 { (*scl_data).ratios.vert.value -= 1; }
    if (*scl_data).ratios.horz_c.value == 4i64 << 32 { (*scl_data).ratios.horz_c.value -= 1; }
    if (*scl_data).ratios.vert_c.value == 4i64 << 32 { (*scl_data).ratios.vert_c.value -= 1; }
    (*scl_data).taps.h_taps = if (*in_taps).h_taps == 0 { 4 } else { (*in_taps).h_taps };
    (*scl_data).taps.v_taps = if (*in_taps).v_taps == 0 { 4 } else { (*in_taps).v_taps };
    (*scl_data).taps.v_taps_c = if (*in_taps).v_taps_c == 0 { 2 } else { (*in_taps).v_taps_c };
    (*scl_data).taps.h_taps_c = if (*in_taps).h_taps_c == 0 { 2 } else if (*in_taps).h_taps_c % 2 != 0 && (*in_taps).h_taps_c != 1 { (*in_taps).h_taps_c - 1 } else { (*in_taps).h_taps_c };
    if !(*dpp).ctx.dc.debug.always_scale {
        if dc_fixpt_u2d19((*scl_data).ratios.horz) == (1 << 19) { (*scl_data).taps.h_taps = 1; (*scl_data).taps.h_taps_c = 1; }
        if dc_fixpt_u2d19((*scl_data).ratios.vert) == (1 << 19) { (*scl_data).taps.v_taps = 1; (*scl_data).taps.v_taps_c = 1; }
        if dc_fixpt_u2d19((*scl_data).ratios.horz_c) == (1 << 19) { (*scl_data).taps.h_taps_c = 1; }
        if dc_fixpt_u2d19((*scl_data).ratios.vert_c) == (1 << 19) { (*scl_data).taps.v_taps_c = 1; }
    }
    true
}

pub unsafe fn dpp_reset(dpp_base: *mut dpp) {
    let dpp = TO_DCN10_DPP(dpp_base);
    (*dpp).filter_h_c = core::ptr::null_mut(); (*dpp).filter_v_c = core::ptr::null_mut();
    (*dpp).filter_h = core::ptr::null_mut(); (*dpp).filter_v = core::ptr::null_mut();
    core::ptr::write_bytes(&mut (*dpp_base).pos as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*dpp_base).att as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*dpp).scl_data as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*dpp).pwl_data as *mut _, 0, 1);
    (*dpp_base).cursor_offload = false;
}

// The remaining implementation is a direct register-programming translation.
// These calls intentionally retain their external C-derived interfaces.
pub unsafe fn dpp1_set_cursor_position(dpp_base: *mut dpp, pos: *const dc_cursor_position, param: *const dc_cursor_mi_param, width: u32, height: u32) {
    let dpp = TO_DCN10_DPP(dpp_base); let x_pos = (*pos).x - (*param).viewport.x; let y_pos = (*pos).y - (*param).viewport.y;
    let mut x_hotspot = (*pos).x_hotspot; let mut y_hotspot = (*pos).y_hotspot; let mut src_x_offset = x_pos - x_hotspot; let mut src_y_offset = y_pos - y_hotspot;
    let mut cursor_height = height as i32; let mut cursor_width = width as i32; let mut cur_en = if (*pos).enable { 1 } else { 0 };
    if (*param).rotation == ROTATION_ANGLE_90 || (*param).rotation == ROTATION_ANGLE_270 { core::mem::swap(&mut cursor_height, &mut cursor_width); core::mem::swap(&mut x_hotspot, &mut y_hotspot); if (*param).rotation == ROTATION_ANGLE_90 { src_x_offset = x_pos - (cursor_width - x_hotspot); src_y_offset = y_pos - y_hotspot; } else { src_x_offset = x_pos - x_hotspot; src_y_offset = y_pos - (cursor_height - y_hotspot); } } else if (*param).rotation == ROTATION_ANGLE_180 { if !(*param).mirror { src_x_offset = x_pos - (cursor_width - x_hotspot); } src_y_offset = y_pos - (cursor_height - y_hotspot); }
    if src_x_offset >= (*param).viewport.width as i32 || src_x_offset + cursor_width <= 0 || src_y_offset >= (*param).viewport.height as i32 || src_y_offset + cursor_height <= 0 { cur_en = 0; }
    if (*dpp_base).pos.cur0_ctl.bits.cur0_enable != cur_en && !(*dpp_base).cursor_offload { REG_UPDATE!(dpp, CURSOR0_CONTROL, CUR0_ENABLE, cur_en); }
    (*dpp_base).pos.cur0_ctl.bits.cur0_enable = cur_en; (*dpp_base).att.cur0_ctl.bits.cur0_enable = cur_en;
}

pub unsafe fn dpp_force_disable_cursor(dpp_base: *mut dpp) { let dpp = TO_DCN10_DPP(dpp_base); REG_UPDATE!(dpp, CURSOR0_CONTROL, CUR0_ENABLE, 0); (*dpp_base).pos.cur0_ctl.bits.cur0_enable = 0; }

pub unsafe fn dpp1_cm_set_regamma_pwl(dpp_base: *mut dpp, params: *const pwl_params, mode: opp_regamma) {
    let dpp = TO_DCN10_DPP(dpp_base); let mut re_mode = 0;
    match mode { OPP_REGAMMA_BYPASS => re_mode = 0, OPP_REGAMMA_SRGB => re_mode = 1, OPP_REGAMMA_XVYCC => re_mode = 2, OPP_REGAMMA_USER => { re_mode = if (*dpp).is_write_to_ram_a_safe { 4 } else { 3 }; if core::ptr::eq(&(*dpp).pwl_data, params) { return; } dpp1_cm_power_on_regamma_lut(dpp_base, true); dpp1_cm_configure_regamma_lut(dpp_base, (*dpp).is_write_to_ram_a_safe); if (*dpp).is_write_to_ram_a_safe { dpp1_cm_program_regamma_luta_settings(dpp_base, params); } else { dpp1_cm_program_regamma_lutb_settings(dpp_base, params); } dpp1_cm_program_regamma_lut(dpp_base, (*params).rgb_resulted, (*params).hw_points_num); (*dpp).pwl_data = *params; re_mode = if (*dpp).is_write_to_ram_a_safe { 3 } else { 4 }; (*dpp).is_write_to_ram_a_safe = !(*dpp).is_write_to_ram_a_safe; }, _ => {} }
    REG_SET!(dpp, CM_RGAM_CONTROL, CM_RGAM_LUT_MODE, re_mode);
}

pub unsafe fn dpp1_set_degamma_format_float(dpp_base: *mut dpp, is_float: bool) { let dpp = TO_DCN10_DPP(dpp_base); if is_float { REG_UPDATE!(dpp, CM_IGAM_CONTROL, CM_IGAM_INPUT_FORMAT, 3); REG_UPDATE!(dpp, CM_IGAM_CONTROL, CM_IGAM_LUT_MODE, 1); } else { REG_UPDATE!(dpp, CM_IGAM_CONTROL, CM_IGAM_INPUT_FORMAT, 2); REG_UPDATE!(dpp, CM_IGAM_CONTROL, CM_IGAM_LUT_MODE, 0); } }

pub unsafe fn dpp1_cnv_set_optional_cursor_attributes(dpp_base: *mut dpp, attr: *const dpp_cursor_attributes) { let dpp = TO_DCN10_DPP(dpp_base); if !attr.is_null() { if !(*dpp_base).cursor_offload { REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS, CUR0_FP_BIAS, (*attr).bias); REG_UPDATE!(dpp, CURSOR0_FP_SCALE_BIAS, CUR0_FP_SCALE, (*attr).scale); } (*dpp_base).att.fp_scale_bias.bits.fp_bias = (*attr).bias; (*dpp_base).att.fp_scale_bias.bits.fp_scale = (*attr).scale; } }

pub unsafe fn dpp1_dppclk_control(dpp_base: *mut dpp, dppclk_div: bool, enable: bool) { let dpp = TO_DCN10_DPP(dpp_base); if enable { if (*dpp).tf_mask.DPPCLK_RATE_CONTROL != 0 { REG_UPDATE_2!(dpp, DPP_CONTROL, DPPCLK_RATE_CONTROL, dppclk_div, DPP_CLOCK_ENABLE, 1); } else { REG_UPDATE!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 1); } } else { REG_UPDATE!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 0); } }

pub unsafe fn dpp1_construct(dpp: *mut dcn10_dpp, ctx: *mut dc_context, inst: u32, tf_regs: *const dcn_dpp_registers, tf_shift: *const dcn_dpp_shift, tf_mask: *const dcn_dpp_mask) { (*dpp).base.ctx = ctx; (*dpp).base.inst = inst; (*dpp).tf_regs = tf_regs; (*dpp).tf_shift = tf_shift; (*dpp).tf_mask = tf_mask; (*dpp).lb_pixel_depth_supported = LB_PIXEL_DEPTH_18BPP | LB_PIXEL_DEPTH_24BPP | LB_PIXEL_DEPTH_30BPP | LB_PIXEL_DEPTH_36BPP; (*dpp).lb_bits_per_entry = LB_BITS_PER_ENTRY; (*dpp).lb_memory_size = LB_TOTAL_NUMBER_OF_ENTRIES; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
