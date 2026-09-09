/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// External C headers and build-time register definitions are supplied by dependencies.

#[repr(C)]
pub enum BlackColorFormat {
    BlackColorFormatRgbFullrange = 0,
    BlackColorFormatRgbLimited,
    BlackColorFormatYuvTv,
    BlackColorFormatYuvCv,
    BlackColorFormatYuvSuperAa,
    BlackColorFormatCount,
}

const NUMBER_OF_FRAME_TO_WAIT_ON_TRIGGERED_RESET: u32 = 10;
const MAX_H_TOTAL: u32 = CRTC_H_TOTAL__CRTC_H_TOTAL_MASK + 1;
const MAX_V_TOTAL: u32 = CRTC_V_TOTAL__CRTC_V_TOTAL_MASKhw + 1;

static REG_OFFSETS: [Dce110TimingGeneratorOffsets; 6] = [
    Dce110TimingGeneratorOffsets { crtc: mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP0_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    Dce110TimingGeneratorOffsets { crtc: mmCRTC1_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP1_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    Dce110TimingGeneratorOffsets { crtc: mmCRTC2_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP2_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    Dce110TimingGeneratorOffsets { crtc: mmCRTC3_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP3_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    Dce110TimingGeneratorOffsets { crtc: mmCRTC4_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP4_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
    Dce110TimingGeneratorOffsets { crtc: mmCRTC5_DCFE_MEM_LIGHT_SLEEP_CNTL - mmCRTC0_DCFE_MEM_LIGHT_SLEEP_CNTL, dcp: mmDCP5_GRPH_CONTROL - mmDCP0_GRPH_CONTROL },
];

unsafe fn program_pix_dur(tg: *mut TimingGenerator, pix_clk_100hz: u32) {
    let addr = mmDMIF_PG0_DPG_PIPE_ARBITRATION_CONTROL1 + (*DCE110TG_FROM_TG(tg)).offsets.dmif;
    let mut value = dm_read_reg((*tg).ctx, addr);
    if pix_clk_100hz == 0 { return; }
    let pix_dur = 10_000_000_000u64 / pix_clk_100hz as u64;
    set_reg_field_value(&mut value, pix_dur, DPG_PIPE_ARBITRATION_CONTROL1, PIXEL_DURATION);
    dm_write_reg((*tg).ctx, addr, value);
}

unsafe fn program_timing(tg: *mut TimingGenerator, timing: *const DcCrtcTiming,
    _vready_offset: i32, _vstartup_start: i32, _vupdate_offset: i32,
    _vupdate_width: i32, _pstate_keepout: i32, _signal: SignalType, use_vbios: bool) {
    if !use_vbios { program_pix_dur(tg, (*timing).pix_clk_100hz); }
    dce110_tg_program_timing(tg, timing, 0, 0, 0, 0, 0, 0, use_vbios);
}

unsafe fn dce60_timing_generator_enable_advanced_request(
    tg: *mut TimingGenerator, _enable: bool, timing: *const DcCrtcTiming) {
    let addr = mmCRTC_START_LINE_CONTROL + (*DCE110TG_FROM_TG(tg)).offsets.crtc;
    let mut value = dm_read_reg((*tg).ctx, addr);
    let addr2 = mmCRTC_CONTROL + (*DCE110TG_FROM_TG(tg)).offsets.crtc;
    let mut value2 = dm_read_reg((*tg).ctx, addr2);
    if (*timing).v_sync_width + (*timing).v_front_porch <= 3 {
        set_reg_field_value(&mut value, 3, CRTC_START_LINE_CONTROL, CRTC_ADVANCED_START_LINE_POSITION);
        set_reg_field_value(&mut value2, 0, CRTC_CONTROL, CRTC_PREFETCH_EN);
    } else {
        set_reg_field_value(&mut value, 4, CRTC_START_LINE_CONTROL, CRTC_ADVANCED_START_LINE_POSITION);
        set_reg_field_value(&mut value2, 1, CRTC_CONTROL, CRTC_PREFETCH_EN);
    }
    set_reg_field_value(&mut value, 1, CRTC_START_LINE_CONTROL, CRTC_PROGRESSIVE_START_LINE_EARLY);
    set_reg_field_value(&mut value, 1, CRTC_START_LINE_CONTROL, CRTC_INTERLACE_START_LINE_EARLY);
    dm_write_reg((*tg).ctx, addr, value);
    dm_write_reg((*tg).ctx, addr2, value2);
}

unsafe fn dce60_is_tg_enabled(tg: *mut TimingGenerator) -> bool {
    let addr = mmCRTC_CONTROL + (*DCE110TG_FROM_TG(tg)).offsets.crtc;
    let value = dm_read_reg((*tg).ctx, addr);
    get_reg_field_value(value, CRTC_CONTROL, CRTC_CURRENT_MASTER_EN_STATE) == 1
}

unsafe fn dce60_configure_crc(tg: *mut TimingGenerator, _params: *const CrcParams) -> bool {
    if !dce60_is_tg_enabled(tg) { return false; }
    true
}

static DCE60_TG_FUNCS: TimingGeneratorFuncs = TimingGeneratorFuncs {
    validate_timing: dce110_tg_validate_timing,
    program_timing,
    enable_crtc: dce110_timing_generator_enable_crtc,
    disable_crtc: dce110_timing_generator_disable_crtc,
    is_counter_moving: dce110_timing_generator_is_counter_moving,
    get_position: dce110_timing_generator_get_position,
    get_frame_count: dce110_timing_generator_get_vblank_counter,
    get_scanoutpos: dce110_timing_generator_get_crtc_scanoutpos,
    set_early_control: dce110_timing_generator_set_early_control,
    wait_for_state: dce110_tg_wait_for_state,
    set_blank: dce110_tg_set_blank,
    is_blanked: dce110_tg_is_blanked,
    set_colors: dce110_tg_set_colors,
    set_overscan_blank_color: dce110_timing_generator_set_overscan_color_black,
    set_blank_color: dce110_timing_generator_program_blank_color,
    disable_vga: dce110_timing_generator_disable_vga,
    did_triggered_reset_occur: dce110_timing_generator_did_triggered_reset_occur,
    setup_global_swap_lock: dce110_timing_generator_setup_global_swap_lock,
    enable_reset_trigger: dce110_timing_generator_enable_reset_trigger,
    disable_reset_trigger: dce110_timing_generator_disable_reset_trigger,
    tear_down_global_swap_lock: dce110_timing_generator_tear_down_global_swap_lock,
    set_drr: dce110_timing_generator_set_drr,
    set_static_screen_control: dce110_timing_generator_set_static_screen_control,
    set_test_pattern: dce110_timing_generator_set_test_pattern,
    arm_vert_intr: dce110_arm_vert_intr,
    enable_advanced_request: dce60_timing_generator_enable_advanced_request,
    configure_crc: dce60_configure_crc,
    get_crc: dce110_get_crc,
    is_two_pixels_per_container: dce110_is_two_pixels_per_container,
};

pub unsafe fn dce60_timing_generator_construct(
    tg110: *mut Dce110TimingGenerator, ctx: *mut DcContext, instance: u32,
    offsets: *const Dce110TimingGeneratorOffsets) {
    (*tg110).controller_id = CONTROLLER_ID_D0 + instance;
    (*tg110).base.inst = instance;
    (*tg110).offsets = *offsets;
    (*tg110).derived_offsets = REG_OFFSETS[instance as usize];
    (*tg110).base.funcs = &DCE60_TG_FUNCS;
    (*tg110).base.ctx = ctx;
    (*tg110).base.bp = (*ctx).dc_bios;
    (*tg110).max_h_total = CRTC_H_TOTAL__CRTC_H_TOTAL_MASK + 1;
    (*tg110).max_v_total = CRTC_V_TOTAL__CRTC_V_TOTAL_MASK + 1;
    (*tg110).min_h_blank = 56;
    (*tg110).min_h_front_porch = 4;
    (*tg110).min_h_back_porch = 4;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
