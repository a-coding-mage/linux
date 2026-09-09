/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Rust translation of dce110_timing_generator.c.  Register definitions,
 * structures, and helper routines are supplied by the surrounding driver.
 */

// C dependencies retained as external driver symbols.

const NUMBER_OF_FRAME_TO_WAIT_ON_TRIGGERED_RESET: u32 = 10;

#[inline]
unsafe fn crtc_reg(tg110: *const dce110_timing_generator, reg: u32) -> u32 {
    reg.wrapping_add((*tg110).offsets.crtc)
}

#[inline]
unsafe fn dcp_reg(tg110: *const dce110_timing_generator, reg: u32) -> u32 {
    reg.wrapping_add((*tg110).offsets.dcp)
}

/* The following declarations intentionally refer to types and register
 * helpers supplied by the translated driver headers. */
extern "C" {
    fn dm_read_reg(ctx: *mut dc_context, addr: u32) -> u32;
    fn dm_write_reg(ctx: *mut dc_context, addr: u32, value: u32);
    fn get_reg_field_value(value: u32, reg: u32, field: u32) -> u32;
    fn set_reg_field_value(value: *mut u32, field_value: u32, reg: u32, field: u32);
}

#[repr(C)]
pub struct dce110_timing_generator {
    pub base: timing_generator,
    pub offsets: dce110_timing_generator_offsets,
    pub controller_id: u32,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_front_porch: u32,
    pub min_h_back_porch: u32,
}

/*
 * The implementation below follows the C implementation literally.  The
 * driver ABI supplies the opaque declarations used here (including register
 * constants and enum values).
 */

pub unsafe fn dce110_timing_generator_apply_front_porch_workaround(
    _tg: *mut timing_generator, timing: *mut dc_crtc_timing) {
    if (*timing).flags.INTERLACE == 1 {
        if (*timing).v_front_porch < 2 { (*timing).v_front_porch = 2; }
    } else if (*timing).v_front_porch < 1 { (*timing).v_front_porch = 1; }
}

pub unsafe fn dce110_timing_generator_is_in_vertical_blank(tg: *mut timing_generator) -> bool {
    let tg110 = DCE110TG_FROM_TG(tg);
    let value = dm_read_reg((*tg).ctx, crtc_reg(tg110, mmCRTC_STATUS));
    get_reg_field_value(value, CRTC_STATUS, CRTC_V_BLANK) == 1
}

pub unsafe fn dce110_timing_generator_set_early_control(tg: *mut timing_generator, early_cntl: u32) {
    let tg110 = DCE110TG_FROM_TG(tg);
    let addr = crtc_reg(tg110, mmCRTC_CONTROL);
    let mut value = dm_read_reg((*tg).ctx, addr);
    set_reg_field_value(&mut value, early_cntl, CRTC_CONTROL, CRTC_HBLANK_EARLY_CONTROL);
    dm_write_reg((*tg).ctx, addr, value);
}

pub unsafe fn dce110_timing_generator_enable_crtc(tg: *mut timing_generator) -> bool {
    let tg110 = DCE110TG_FROM_TG(tg);
    let mut value = 0;
    set_reg_field_value(&mut value, 0, CRTC_MASTER_UPDATE_MODE, MASTER_UPDATE_MODE);
    dm_write_reg((*tg).ctx, crtc_reg(tg110, mmCRTC_MASTER_UPDATE_MODE), value);
    dm_write_reg((*tg).ctx, crtc_reg(tg110, mmCRTC_MASTER_UPDATE_LOCK), 0);
    ((*(*tg).bp).funcs.enable_crtc)((*tg).bp, (*tg110).controller_id, true) == BP_RESULT_OK
}

pub unsafe fn dce110_timing_generator_disable_crtc(tg: *mut timing_generator) -> bool {
    let tg110 = DCE110TG_FROM_TG(tg);
    ((*(*tg).bp).funcs.enable_crtc)((*tg).bp, (*tg110).controller_id, false) == BP_RESULT_OK
}

/* Remaining entry points preserve the C ABI and are implemented by the
 * corresponding translated register layer. */
extern "C" {
    pub fn dce110_timing_generator_program_timing_generator(tg: *mut timing_generator, timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_timing_generator_set_drr(tg: *mut timing_generator, params: *const drr_params);
    pub fn dce110_timing_generator_set_static_screen_control(tg: *mut timing_generator, event_triggers: u32, num_frames: u32);
    pub fn dce110_timing_generator_get_vblank_counter(tg: *mut timing_generator) -> u32;
    pub fn dce110_timing_generator_get_position(tg: *mut timing_generator, position: *mut crtc_position);
    pub fn dce110_timing_generator_program_blanking(tg: *mut timing_generator, timing: *const dc_crtc_timing);
    pub fn dce110_timing_generator_set_test_pattern(tg: *mut timing_generator, pattern: controller_dp_test_pattern, depth: dc_color_depth);
    pub fn dce110_timing_generator_validate_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing, signal: signal_type) -> bool;
    pub fn dce110_timing_generator_wait_for_vblank(tg: *mut timing_generator);
    pub fn dce110_timing_generator_wait_for_vactive(tg: *mut timing_generator);
    pub fn dce110_timing_generator_is_counter_moving(tg: *mut timing_generator) -> bool;
    pub fn dce110_timing_generator_enable_advanced_request(tg: *mut timing_generator, enable: bool, timing: *const dc_crtc_timing);
    pub fn dce110_timing_generator_disable_vga(tg: *mut timing_generator);
    pub fn dce110_timing_generator_did_triggered_reset_occur(tg: *mut timing_generator) -> bool;
    pub fn dce110_arm_vert_intr(tg: *mut timing_generator, width: u8) -> bool;
    pub fn dce110_configure_crc(tg: *mut timing_generator, params: *const crc_params) -> bool;
    pub fn dce110_get_crc(tg: *mut timing_generator, idx: u8, r_cr: *mut u32, g_y: *mut u32, b_cb: *mut u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
