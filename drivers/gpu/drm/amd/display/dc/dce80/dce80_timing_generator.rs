/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// C dependencies are supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_timing_generator_offsets {
    pub crtc: u32,
    pub dcp: u32,
}

#[repr(C)]
pub struct dce110_timing_generator {
    pub base: timing_generator,
    pub offsets: dce110_timing_generator_offsets,
    pub derived_offsets: dce110_timing_generator_offsets,
    pub controller_id: u32,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_front_porch: u32,
    pub min_h_back_porch: u32,
}

#[repr(C)]
pub struct timing_generator {
    pub funcs: *const timing_generator_funcs,
    pub ctx: *mut dc_context,
    pub bp: *mut dc_bios,
    pub inst: u32,
}

#[repr(C)]
pub struct dc_context { pub dc_bios: *mut dc_bios }
pub struct dc_bios;
pub struct dc_crtc_timing { pub pix_clk_100hz: u32, pub v_sync_width: u32, pub v_front_porch: u32 }
pub enum signal_type {}

#[repr(C)]
pub struct timing_generator_funcs {
    pub validate_timing: Option<unsafe extern "C" fn()>,
    pub program_timing: Option<unsafe extern "C" fn()>,
    pub enable_crtc: Option<unsafe extern "C" fn()>,
    pub disable_crtc: Option<unsafe extern "C" fn()>,
    pub is_counter_moving: Option<unsafe extern "C" fn()>,
    pub get_position: Option<unsafe extern "C" fn()>,
    pub get_frame_count: Option<unsafe extern "C" fn()>,
    pub get_scanoutpos: Option<unsafe extern "C" fn()>,
    pub set_early_control: Option<unsafe extern "C" fn()>,
    pub wait_for_state: Option<unsafe extern "C" fn()>,
    pub set_blank: Option<unsafe extern "C" fn()>,
    pub is_blanked: Option<unsafe extern "C" fn()>,
    pub set_colors: Option<unsafe extern "C" fn()>,
    pub set_overscan_blank_color: Option<unsafe extern "C" fn()>,
    pub set_blank_color: Option<unsafe extern "C" fn()>,
    pub disable_vga: Option<unsafe extern "C" fn()>,
    pub did_triggered_reset_occur: Option<unsafe extern "C" fn()>,
    pub setup_global_swap_lock: Option<unsafe extern "C" fn()>,
    pub enable_reset_trigger: Option<unsafe extern "C" fn()>,
    pub disable_reset_trigger: Option<unsafe extern "C" fn()>,
    pub tear_down_global_swap_lock: Option<unsafe extern "C" fn()>,
    pub set_drr: Option<unsafe extern "C" fn()>,
    pub get_last_used_drr_vtotal: Option<unsafe extern "C" fn()>,
    pub set_static_screen_control: Option<unsafe extern "C" fn()>,
    pub set_test_pattern: Option<unsafe extern "C" fn()>,
    pub arm_vert_intr: Option<unsafe extern "C" fn()>,
    pub enable_advanced_request: Option<unsafe extern "C" fn()>,
    pub configure_crc: Option<unsafe extern "C" fn()>,
    pub get_crc: Option<unsafe extern "C" fn()>,
    pub is_two_pixels_per_container: Option<unsafe extern "C" fn()>,
}

const NUMBER_OF_FRAME_TO_WAIT_ON_TRIGGERED_RESET: u32 = 10;
const MAX_H_TOTAL: u32 = CRTC_H_TOTAL__CRTC_H_TOTAL_MASK + 1;
const MAX_V_TOTAL: u32 = CRTC_V_TOTAL__CRTC_V_TOTAL_MASKhw + 1;

static REG_OFFSETS: [dce110_timing_generator_offsets; 6] = [
    dce110_timing_generator_offsets { crtc: mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP0_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC1_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP1_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC2_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP2_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC3_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP3_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC4_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP4_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC5_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP5_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
];

unsafe fn program_pix_dur(tg: *mut timing_generator, pix_clk_100hz: u32) {
    let tg110 = DCE110TG_FROM_TG(tg);
    let addr = mmDMIF_PG0_DPG_PIPE_ARBITRATION_CONTROL1 + (*tg110).offsets.dmif;
    let mut value = dm_read_reg((*tg).ctx, addr);
    if pix_clk_100hz == 0 { return; }
    let pix_dur = 10_000_000_000u64 / pix_clk_100hz as u64;
    set_reg_field_value(&mut value, pix_dur as u32, DPG_PIPE_ARBITRATION_CONTROL1, PIXEL_DURATION);
    dm_write_reg((*tg).ctx, addr, value);
}

unsafe fn dce80_timing_generator_program_timing(tg: *mut timing_generator, timing: *const dc_crtc_timing, _vready_offset: i32, _vstartup_start: i32, _vupdate_offset: i32, _vupdate_width: i32, _pstate_keepout: i32, _signal: signal_type, use_vbios: bool) {
    if !use_vbios { program_pix_dur(tg, (*timing).pix_clk_100hz); }
    dce110_tg_program_timing(tg, timing, 0, 0, 0, 0, 0, 0, use_vbios);
}

unsafe fn dce80_timing_generator_enable_advanced_request(tg: *mut timing_generator, enable: bool, timing: *const dc_crtc_timing) {
    let tg110 = DCE110TG_FROM_TG(tg);
    let addr = mmCRTC_START_LINE_CONTROL + (*tg110).offsets.crtc;
    let mut value = dm_read_reg((*tg).ctx, addr);
    set_reg_field_value(&mut value, if enable { 0 } else { 1 }, CRTC_START_LINE_CONTROL, CRTC_LEGACY_REQUESTOR_EN);
    if (*timing).v_sync_width + (*timing).v_front_porch <= 3 {
        set_reg_field_value(&mut value, 3, CRTC_START_LINE_CONTROL, CRTC_ADVANCED_START_LINE_POSITION);
        set_reg_field_value(&mut value, 0, CRTC_START_LINE_CONTROL, CRTC_PREFETCH_EN);
    } else {
        set_reg_field_value(&mut value, 4, CRTC_START_LINE_CONTROL, CRTC_ADVANCED_START_LINE_POSITION);
        set_reg_field_value(&mut value, 1, CRTC_START_LINE_CONTROL, CRTC_PREFETCH_EN);
    }
    set_reg_field_value(&mut value, 1, CRTC_START_LINE_CONTROL, CRTC_PROGRESSIVE_START_LINE_EARLY);
    set_reg_field_value(&mut value, 1, CRTC_START_LINE_CONTROL, CRTC_INTERLACE_START_LINE_EARLY);
    dm_write_reg((*tg).ctx, addr, value);
}

static DCE80_TG_FUNCS: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(dce110_tg_validate_timing), program_timing: Some(dce80_timing_generator_program_timing),
    enable_crtc: Some(dce110_timing_generator_enable_crtc), disable_crtc: Some(dce110_timing_generator_disable_crtc),
    is_counter_moving: Some(dce110_timing_generator_is_counter_moving), get_position: Some(dce110_timing_generator_get_position),
    get_frame_count: Some(dce110_timing_generator_get_vblank_counter), get_scanoutpos: Some(dce110_timing_generator_get_crtc_scanoutpos),
    set_early_control: Some(dce110_timing_generator_set_early_control), wait_for_state: Some(dce110_tg_wait_for_state),
    set_blank: Some(dce110_tg_set_blank), is_blanked: Some(dce110_tg_is_blanked), set_colors: Some(dce110_tg_set_colors),
    set_overscan_blank_color: Some(dce110_timing_generator_set_overscan_color_black), set_blank_color: Some(dce110_timing_generator_program_blank_color),
    disable_vga: Some(dce110_timing_generator_disable_vga), did_triggered_reset_occur: Some(dce110_timing_generator_did_triggered_reset_occur),
    setup_global_swap_lock: Some(dce110_timing_generator_setup_global_swap_lock), enable_reset_trigger: Some(dce110_timing_generator_enable_reset_trigger),
    disable_reset_trigger: Some(dce110_timing_generator_disable_reset_trigger), tear_down_global_swap_lock: Some(dce110_timing_generator_tear_down_global_swap_lock),
    set_drr: Some(dce110_timing_generator_set_drr), get_last_used_drr_vtotal: None, set_static_screen_control: Some(dce110_timing_generator_set_static_screen_control),
    set_test_pattern: Some(dce110_timing_generator_set_test_pattern), arm_vert_intr: Some(dce110_arm_vert_intr),
    enable_advanced_request: Some(dce80_timing_generator_enable_advanced_request), configure_crc: Some(dce110_configure_crc),
    get_crc: Some(dce110_get_crc), is_two_pixels_per_container: Some(dce110_is_two_pixels_per_container),
};

pub unsafe fn dce80_timing_generator_construct(tg110: *mut dce110_timing_generator, ctx: *mut dc_context, instance: u32, offsets: *const dce110_timing_generator_offsets) {
    (*tg110).controller_id = CONTROLLER_ID_D0 + instance;
    (*tg110).base.inst = instance;
    (*tg110).offsets = *offsets;
    (*tg110).derived_offsets = REG_OFFSETS[instance as usize];
    (*tg110).base.funcs = &DCE80_TG_FUNCS;
    (*tg110).base.ctx = ctx;
    (*tg110).base.bp = (*ctx).dc_bios;
    (*tg110).max_h_total = CRTC_H_TOTAL__CRTC_H_TOTAL_MASK + 1;
    (*tg110).max_v_total = CRTC_V_TOTAL__CRTC_V_TOTAL_MASK + 1;
    (*tg110).min_h_blank = 56;
    (*tg110).min_h_front_porch = 4;
    (*tg110).min_h_back_porch = 4;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
