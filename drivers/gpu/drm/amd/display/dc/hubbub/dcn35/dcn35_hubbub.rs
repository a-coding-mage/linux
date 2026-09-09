/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

const DCN35_CRB_SEGMENT_SIZE_KB: u32 = 64;

unsafe fn convert_and_clamp(wm_ns: u32, refclk_mhz: u32, clamp_value: u32) -> u32 {
    let mut ret_val = wm_ns.wrapping_mul(refclk_mhz);
    ret_val /= 1000;
    if ret_val > clamp_value { ret_val = clamp_value; }
    ret_val
}

pub unsafe fn dcn35_init_crb(hubbub: *mut hubbub) {
    let hubbub2 = TO_DCN20_HUBBUB(hubbub);
    REG_GET(DCHUBBUB_DET0_CTRL, DET0_SIZE_CURRENT, &mut (*hubbub2).det0_size);
    REG_GET(DCHUBBUB_DET1_CTRL, DET1_SIZE_CURRENT, &mut (*hubbub2).det1_size);
    REG_GET(DCHUBBUB_DET2_CTRL, DET2_SIZE_CURRENT, &mut (*hubbub2).det2_size);
    REG_GET(DCHUBBUB_DET3_CTRL, DET3_SIZE_CURRENT, &mut (*hubbub2).det3_size);
    REG_GET(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE_CURRENT, &mut (*hubbub2).compbuf_size_segments);
    REG_SET_2(COMPBUF_RESERVED_SPACE, 0, COMPBUF_RESERVED_SPACE_64B, (*hubbub2).pixel_chunk_size / 32,
        COMPBUF_RESERVED_SPACE_ZS, (*hubbub2).pixel_chunk_size / 128);
    REG_UPDATE(DCHUBBUB_DEBUG_CTRL_0, DET_DEPTH, 0x5ff);
}

pub unsafe fn dcn35_program_compbuf_size(hubbub: *mut hubbub, compbuf_size_kb: u32, safe_to_increase: bool) {
    let hubbub2 = TO_DCN20_HUBBUB(hubbub);
    let compbuf_size_segments = (compbuf_size_kb + DCN35_CRB_SEGMENT_SIZE_KB - 1) / DCN35_CRB_SEGMENT_SIZE_KB;
    if safe_to_increase || compbuf_size_segments <= (*hubbub2).compbuf_size_segments {
        if compbuf_size_segments > (*hubbub2).compbuf_size_segments {
            REG_WAIT(DCHUBBUB_DET0_CTRL, DET0_SIZE_CURRENT, (*hubbub2).det0_size, 1, 100);
            REG_WAIT(DCHUBBUB_DET1_CTRL, DET1_SIZE_CURRENT, (*hubbub2).det1_size, 1, 100);
            REG_WAIT(DCHUBBUB_DET2_CTRL, DET2_SIZE_CURRENT, (*hubbub2).det2_size, 1, 100);
            REG_WAIT(DCHUBBUB_DET3_CTRL, DET3_SIZE_CURRENT, (*hubbub2).det3_size, 1, 100);
        }
        ASSERT((*hubbub2).det0_size + (*hubbub2).det1_size + (*hubbub2).det2_size + (*hubbub2).det3_size + compbuf_size_segments <= (*hubbub2).crb_size_segs);
        REG_UPDATE(DCHUBBUB_COMPBUF_CTRL, COMPBUF_SIZE, compbuf_size_segments);
        (*hubbub2).compbuf_size_segments = compbuf_size_segments;
        let mut config_error = compbuf_size_segments;
        ASSERT(REG_GET(DCHUBBUB_COMPBUF_CTRL, CONFIG_ERROR, &mut config_error) && config_error == 0);
    }
}

unsafe fn hubbub35_program_stutter_z8_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: u32, safe_to_lower: bool) -> bool {
    let h = TO_DCN20_HUBBUB(hubbub);
    let mut pending = false;
    macro_rules! wm { ($s:ident, $e:ident, $r:ident) => {
        if safe_to_lower || (*watermarks).$s.cstate_pstate.$e > (*h).watermarks.$s.cstate_pstate.$e {
            (*h).watermarks.$s.cstate_pstate.$e = (*watermarks).$s.cstate_pstate.$e;
            let v = convert_and_clamp((*watermarks).$s.cstate_pstate.$e, refclk_mhz, 0xfffff);
            REG_SET($r, 0, $r, v);
        } else if (*watermarks).$s.cstate_pstate.$e < (*h).watermarks.$s.cstate_pstate.$e { pending = true; }
    }};
    wm!(a, cstate_enter_plus_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A);
    wm!(a, cstate_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A);
    wm!(b, cstate_enter_plus_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B);
    wm!(b, cstate_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B);
    wm!(c, cstate_enter_plus_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C);
    wm!(c, cstate_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C);
    wm!(d, cstate_enter_plus_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D);
    wm!(d, cstate_exit_z8_ns, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D);
    pending
}

pub unsafe fn hubbub35_get_dchub_ref_freq(hubbub: *mut hubbub, _dccg_ref_freq_in_khz: u32, out: *mut u32) {
    let h = TO_DCN20_HUBBUB(hubbub); let mut div = 0; let mut en = 0; let refclk = 24000;
    REG_GET_2(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, &mut div, DCHUBBUB_GLOBAL_TIMER_ENABLE, &mut en);
    if en { *out = if div == 2 { refclk / 2 } else { refclk }; if *out < 20000 || *out > 50000 { ASSERT_CRITICAL(false); } }
    else { *out = refclk; REG_UPDATE_2(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, 1, DCHUBBUB_GLOBAL_TIMER_ENABLE, 1); ASSERT_CRITICAL(false); }
    let _ = h;
}

pub unsafe fn hubbub35_program_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: u32, safe_to_lower: bool) -> bool {
    let mut pending = false;
    if hubbub32_program_urgent_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { pending = true; }
    if hubbub32_program_stutter_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { pending = true; }
    if hubbub32_program_pstate_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { pending = true; }
    if hubbub32_program_usr_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { pending = true; }
    if hubbub35_program_stutter_z8_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { pending = true; }
    REG_SET(DCHUBBUB_ARB_SAT_LEVEL, 0, DCHUBBUB_ARB_SAT_LEVEL, 60 * refclk_mhz);
    REG_UPDATE_2(DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MIN_REQ_OUTSTAND, 0xff, DCHUBBUB_ARB_MIN_REQ_OUTSTAND_COMMIT_THRESHOLD, 0xa);
    REG_UPDATE(DCHUBBUB_ARB_HOSTVM_CNTL, DCHUBBUB_ARB_MAX_QOS_COMMIT_THRESHOLD, 0xf);
    if safe_to_lower || (*hubbub).ctx.dc.debug.disable_stutter { hubbub1_allow_self_refresh_control(hubbub, !(*hubbub).ctx.dc.debug.disable_stutter); }
    hubbub32_force_usr_retraining_allow(hubbub, (*hubbub).ctx.dc.debug.force_usr_allow); pending
}

pub unsafe fn hubbub35_init_watermarks(hubbub: *mut hubbub) {
    let _h = TO_DCN20_HUBBUB(hubbub); let mut reg;
    macro_rules! copy { ($a:ident, $b:ident, $c:ident, $d:ident) => { reg = REG_READ($a); REG_WRITE($b, reg); REG_WRITE($c, reg); REG_WRITE($d, reg); }; }
    copy!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D);
    copy!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D);
    copy!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, DCHUBBUB_ARB_FRAC_URG_BW_NOM_C, DCHUBBUB_ARB_FRAC_URG_BW_NOM_D);
    copy!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D);
    copy!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D);
    copy!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D);
    copy!(DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_C, DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_D);
    copy!(DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_C, DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_D);
    copy!(DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_C, DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_D);
    copy!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D);
    copy!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D);
}

/* Register-state reads and initialization retain the source's external register-helper dependencies. */
pub unsafe fn hubbub35_wm_read_state(hubbub: *mut hubbub, wm: *mut dcn_hubbub_wm) {
    let _h = TO_DCN20_HUBBUB(hubbub); memset(wm, 0, core::mem::size_of::<dcn_hubbub_wm>());
    macro_rules! read_set { ($i:expr, $x:ident, $z:ident) => { (*wm).sets[$i].wm_set = $i; REG_GET($x, $x, &mut (*wm).sets[$i].data_urgent); REG_GET($z, $z, &mut (*wm).sets[$i].sr_enter_exit_Z8); }; }
    read_set!(0, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A);
    read_set!(1, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B);
    read_set!(2, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C);
    read_set!(3, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D);
}

unsafe fn hubbub35_set_fgcg(hubbub2: *mut dcn20_hubbub, enable: bool) { let _ = hubbub2; REG_UPDATE(DCHUBBUB_CLOCK_CNTL, DCHUBBUB_FGCG_REP_DIS, !enable); }

pub unsafe fn hubbub35_init(hubbub: *mut hubbub) {
    let h = TO_DCN20_HUBBUB(hubbub);
    if (*hubbub).ctx.dc.debug.disable_clock_gate { REG_UPDATE_2(DCHUBBUB_CLOCK_CNTL, DISPCLK_R_DCHUBBUB_GATE_DIS, 1, DCFCLK_R_DCHUBBUB_GATE_DIS, 1); }
    hubbub35_set_fgcg(h, (*hubbub).ctx.dc.debug.enable_fine_grain_clock_gating.bits.dchubbub);
    REG_UPDATE(DCHUBBUB_SDPIF_CFG0, SDPIF_PORT_CONTROL, 1); REG_UPDATE(DCHUBBUB_SDPIF_CFG1, SDPIF_MAX_NUM_OUTSTANDING, 0);
    REG_UPDATE_2(DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MAX_REQ_OUTSTAND, 256, DCHUBBUB_ARB_MIN_REQ_OUTSTAND, 256);
    memset(&mut (*h).watermarks.a.cstate_pstate, 0, core::mem::size_of_val(&(*h).watermarks.a.cstate_pstate));
}

pub unsafe fn dcn35_dchvm_init(hubbub: *mut hubbub) {
    REG_UPDATE(DCHVM_CTRL0, HOSTVM_INIT_REQ, 1); let mut active = 0;
    for _i in 0..100 { REG_GET(DCHVM_RIOMMU_STAT0, RIOMMU_ACTIVE, &mut active); if active != 0 { break; } else { udelay(5); } }
    if active != 0 { REG_UPDATE_2(DCHVM_MEM_CTRL, HVM_GPUVMRET_PWR_REQ_DIS, 1, HVM_GPUVMRET_FORCE_REQ, 0); REG_UPDATE_4(DCHVM_CLK_CTRL, HVM_DISPCLK_R_GATE_DIS, 1, HVM_DISPCLK_G_GATE_DIS, 1, HVM_DCFCLK_R_GATE_DIS, 1, HVM_DCFCLK_G_GATE_DIS, 1); REG_UPDATE(DCHVM_RIOMMU_CTRL0, HOSTVM_POWERSTATUS, 1); REG_UPDATE(DCHVM_RIOMMU_CTRL0, HOSTVM_PREFETCH_REQ, 1); REG_WAIT(DCHVM_RIOMMU_STAT0, HOSTVM_PREFETCH_DONE, 1, 5, 100); REG_UPDATE(DCHVM_MEM_CTRL, HVM_GPUVMRET_PWR_REQ_DIS, 0); REG_UPDATE_4(DCHVM_CLK_CTRL, HVM_DISPCLK_R_GATE_DIS, 0, HVM_DISPCLK_G_GATE_DIS, 0, HVM_DCFCLK_R_GATE_DIS, 0, HVM_DCFCLK_G_GATE_DIS, 0); (*hubbub).riommu_active = true; }
}

pub unsafe fn hubbub35_construct(hubbub2: *mut dcn20_hubbub, ctx: *mut dc_context, regs: *const dcn_hubbub_registers, shift: *const dcn_hubbub_shift, mask: *const dcn_hubbub_mask, det_size_kb: i32, pixel_chunk_size_kb: i32, config_return_buffer_size_kb: i32) {
    (*hubbub2).base.ctx = ctx; (*hubbub2).regs = regs; (*hubbub2).shifts = shift; (*hubbub2).masks = mask;
    (*hubbub2).debug_test_index_pstate = 0xb; (*hubbub2).detile_buf_size = det_size_kb * 1024; (*hubbub2).pixel_chunk_size = pixel_chunk_size_kb * 1024; (*hubbub2).crb_size_segs = config_return_buffer_size_kb / DCN35_CRB_SEGMENT_SIZE_KB as i32;
    (*hubbub2).base.funcs = &hubbub35_funcs;
}

static hubbub35_funcs: hubbub_funcs = hubbub_funcs { update_dchub: hubbub2_update_dchub, init_dchub_sys_ctx: hubbub31_init_dchub_sys_ctx, init_vm_ctx: hubbub2_init_vm_ctx, dcc_support_swizzle: hubbub3_dcc_support_swizzle, dcc_support_pixel_format: hubbub2_dcc_support_pixel_format, get_dcc_compression_cap: hubbub3_get_dcc_compression_cap, wm_read_state: hubbub35_wm_read_state, get_dchub_ref_freq: hubbub35_get_dchub_ref_freq, program_watermarks: hubbub35_program_watermarks, allow_self_refresh_control: hubbub1_allow_self_refresh_control, is_allow_self_refresh_enabled: hubbub1_is_allow_self_refresh_enabled, verify_allow_pstate_change_high: hubbub1_verify_allow_pstate_change_high, force_wm_propagate_to_pipes: hubbub32_force_wm_propagate_to_pipes, force_pstate_change_control: hubbub3_force_pstate_change_control, init_watermarks: hubbub35_init_watermarks, program_det_size: dcn32_program_det_size, program_compbuf_size: dcn35_program_compbuf_size, init_crb: dcn35_init_crb, hubbub_read_state: hubbub2_read_state, force_usr_retraining_allow: hubbub32_force_usr_retraining_allow, dchubbub_init: hubbub35_init, hubbub_read_reg_state: hubbub3_read_reg_state, dchvm_init: dcn35_dchvm_init };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
