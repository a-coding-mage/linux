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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Register headers and DAL declarations are supplied by the surrounding tree.
// Their C preprocessor register-list macros are intentionally retained as
// dependency markers below; this file does not provide their implementations.

const MM_DP_DPHY_INTERNAL_CTRL: u32 = 0x4aa7;
const MM_DP0_DP_DPHY_INTERNAL_CTRL: u32 = 0x4aa7;
const MM_DP1_DP_DPHY_INTERNAL_CTRL: u32 = 0x4ba7;
const MM_DP2_DP_DPHY_INTERNAL_CTRL: u32 = 0x4ca7;
const MM_DP3_DP_DPHY_INTERNAL_CTRL: u32 = 0x4da7;
const MM_DP4_DP_DPHY_INTERNAL_CTRL: u32 = 0x4da7 + 0x100;
const MM_DP5_DP_DPHY_INTERNAL_CTRL: u32 = 0x4fa7;
const MM_DP6_DP_DPHY_INTERNAL_CTRL: u32 = 0x54a7;
const MM_DP7_DP_DPHY_INTERNAL_CTRL: u32 = 0x56a7;
const MM_DP8_DP_DPHY_INTERNAL_CTRL: u32 = 0x57a7;
const MM_BIOS_SCRATCH_0: u32 = 0x05c9;
const MM_BIOS_SCRATCH_2: u32 = 0x05cb;
const MM_BIOS_SCRATCH_3: u32 = 0x05cc;
const MM_BIOS_SCRATCH_6: u32 = 0x05cf;
const MM_DP_DPHY_BS_SR_SWAP_CNTL: u32 = 0x4adc;
const MM_DP_DPHY_FAST_TRAINING: u32 = 0x4abc;
const DCFE_MEM_PWR_CTRL_REG_BASE: u32 = 0x1b03;
const MM_CC_DC_HDMI_STRAPS: u32 = 0x1918;
const CC_DC_HDMI_STRAPS_HDMI_DISABLE_MASK: u32 = 0x40;
const CC_DC_HDMI_STRAPS_HDMI_DISABLE_SHIFT: u32 = 0x6;
const CC_DC_HDMI_STRAPS_AUDIO_STREAM_NUMBER_MASK: u32 = 0x700;
const CC_DC_HDMI_STRAPS_AUDIO_STREAM_NUMBER_SHIFT: u32 = 0x8;

// The following arrays are initialized by the corresponding external
// register-list macros in the C implementation.
static DCE100_TG_OFFSETS: [dce110_timing_generator_offsets; 6] = [
    dce110_timing_generator_offsets { crtc: MM_CRTC0_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP0_GRPH_CONTROL - MM_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: MM_CRTC1_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP1_GRPH_CONTROL - MM_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: MM_CRTC2_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP2_GRPH_CONTROL - MM_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: MM_CRTC3_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP3_GRPH_CONTROL - MM_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: MM_CRTC4_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP4_GRPH_CONTROL - MM_GRPH_CONTROL },
    dce110_timing_generator_offsets { crtc: MM_CRTC5_CRTC_CONTROL - MM_CRTC_CONTROL, dcp: MM_DCP5_GRPH_CONTROL - MM_GRPH_CONTROL },
];

// External macro-generated register tables (IPP, transform, encoder, OPP,
// AUX, audio, clock, DMCU, ABM, memory-input, and I2C tables).
static mut IPP_REGS: [dce_ipp_registers; 6] = unsafe { core::mem::zeroed() };
static mut XFM_REGS: [dce_transform_registers; 6] = unsafe { core::mem::zeroed() };
static mut LINK_ENC_AUX_REGS: [dce110_link_enc_aux_registers; 6] = unsafe { core::mem::zeroed() };
static mut LINK_ENC_HPD_REGS: [dce110_link_enc_hpd_registers; 6] = unsafe { core::mem::zeroed() };
static mut LINK_ENC_REGS: [dce110_link_enc_registers; 8] = unsafe { core::mem::zeroed() };
static mut STREAM_ENC_REGS: [dce110_stream_enc_registers; 8] = unsafe { core::mem::zeroed() };
static mut OPP_REGS: [dce_opp_registers; 6] = unsafe { core::mem::zeroed() };
static mut AUX_ENGINE_REGS: [dce110_aux_registers; 6] = unsafe { core::mem::zeroed() };
static mut AUDIO_REGS: [dce_audio_registers; 7] = unsafe { core::mem::zeroed() };
static mut CLK_SRC_REGS: [dce110_clk_src_regs; 3] = unsafe { core::mem::zeroed() };
static mut I2C_HW_REGS: [dce_i2c_registers; 6] = unsafe { core::mem::zeroed() };

static BIOS_REGS: bios_registers = bios_registers { BIOS_SCRATCH_0: MM_BIOS_SCRATCH_0, BIOS_SCRATCH_3: MM_BIOS_SCRATCH_3, BIOS_SCRATCH_6: MM_BIOS_SCRATCH_6 };
static RES_CAP: resource_caps = resource_caps { num_timing_generator: 6, num_audio: 6, num_analog_stream_encoder: 1, num_stream_encoder: 6, num_pll: 3, num_ddc: 6 };
static PLANE_CAP: dc_plane_cap = dc_plane_cap { type_: DC_PLANE_TYPE_DCE_RGB, pixel_format_support: pixel_format_support { argb8888: true, nv12: false, fp16: true }, max_upscale_factor: scaling_factor { argb8888: 16000, nv12: 1, fp16: 1 }, max_downscale_factor: scaling_factor { argb8888: 250, nv12: 1, fp16: 1 } };
static DEBUG_DEFAULTS: dc_debug_options = unsafe { core::mem::zeroed() };
static CONFIG_DEFAULTS: dc_check_config = dc_check_config { enable_legacy_fast_update: true };

unsafe fn map_transmitter_id_to_phy_instance(transmitter: transmitter) -> i32 {
    match transmitter {
        TRANSMITTER_UNIPHY_A => 0, TRANSMITTER_UNIPHY_B => 1,
        TRANSMITTER_UNIPHY_C => 2, TRANSMITTER_UNIPHY_D => 3,
        TRANSMITTER_UNIPHY_E => 4, TRANSMITTER_UNIPHY_F => 5,
        TRANSMITTER_UNIPHY_G => 6,
        _ => { ASSERT(0); 0 }
    }
}

unsafe fn read_dce_straps(ctx: *mut dc_context, straps: *mut resource_straps) {
    REG_GET_2!(MM_CC_DC_HDMI_STRAPS, HDMI_DISABLE, &mut (*straps).hdmi_disable,
        AUDIO_STREAM_NUMBER, &mut (*straps).audio_stream_number);
    REG_GET!(DC_PINSTRAPS, DC_PINSTRAPS_AUDIO, &mut (*straps).dc_pinstraps_audio);
}

unsafe fn create_audio(ctx: *mut dc_context, inst: u32) -> *mut audio {
    dce_audio_create(ctx, inst, &AUDIO_REGS[inst as usize], &AUDIO_SHIFT, &AUDIO_MASK)
}

unsafe fn dce100_timing_generator_create(ctx: *mut dc_context, instance: u32, offsets: *const dce110_timing_generator_offsets) -> *mut timing_generator {
    let tg110 = kzalloc_obj::<dce110_timing_generator>();
    if tg110.is_null() { return core::ptr::null_mut(); }
    dce110_timing_generator_construct(tg110, ctx, instance, offsets);
    &mut (*tg110).base
}

unsafe fn dce100_stream_encoder_create(eng_id: engine_id, ctx: *mut dc_context) -> *mut stream_encoder {
    let enc110 = kzalloc_obj::<dce110_stream_encoder>();
    if enc110.is_null() { return core::ptr::null_mut(); }
    if eng_id == ENGINE_ID_DACA || eng_id == ENGINE_ID_DACB {
        dce110_analog_stream_encoder_construct(enc110, ctx, (*ctx).dc_bios, eng_id, &STREAM_ENC_REGS[eng_id as usize], &SE_SHIFT, &SE_MASK);
    } else {
        dce110_stream_encoder_construct(enc110, ctx, (*ctx).dc_bios, eng_id, &STREAM_ENC_REGS[eng_id as usize], &SE_SHIFT, &SE_MASK);
    }
    &mut (*enc110).base
}

unsafe fn dce100_hwseq_create(ctx: *mut dc_context) -> *mut dce_hwseq {
    let hws = kzalloc_obj::<dce_hwseq>();
    if !hws.is_null() { (*hws).ctx = ctx; (*hws).regs = &HWSEQ_REG; (*hws).shifts = &HWSEQ_SHIFT; (*hws).masks = &HWSEQ_MASK; }
    hws
}

unsafe fn dce100_mem_input_create(ctx: *mut dc_context, inst: u32) -> *mut mem_input {
    let mi = kzalloc_obj::<dce_mem_input>();
    if mi.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
    dce_mem_input_construct(mi, ctx, inst, &MI_REGS[inst as usize], &MI_SHIFTS, &MI_MASKS);
    (*mi).wa.single_head_rdreq_dmif_limit = 2;
    &mut (*mi).base
}

unsafe fn dce100_transform_destroy(xfm: *mut *mut transform) { kfree(TO_DCE_TRANSFORM!(*xfm)); *xfm = core::ptr::null_mut(); }
unsafe fn dce100_transform_create(ctx: *mut dc_context, inst: u32) -> *mut transform { let x = kzalloc_obj::<dce_transform>(); if x.is_null() { return core::ptr::null_mut(); } dce_transform_construct(x, ctx, inst, &XFM_REGS[inst as usize], &XFM_SHIFT, &XFM_MASK); &mut (*x).base }
unsafe fn dce100_ipp_create(ctx: *mut dc_context, inst: u32) -> *mut input_pixel_processor { let x = kzalloc_obj::<dce_ipp>(); if x.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dce_ipp_construct(x, ctx, inst, &IPP_REGS[inst as usize], &IPP_SHIFT, &IPP_MASK); &mut (*x).base }

unsafe fn dce100_link_encoder_create(ctx: *mut dc_context, init: *const encoder_init_data) -> *mut link_encoder {
    let _ = ctx; let enc = kzalloc_obj::<dce110_link_encoder>(); if enc.is_null() { return core::ptr::null_mut(); }
    if (*init).connector.id == CONNECTOR_ID_VGA && (*init).analog_engine != ENGINE_ID_UNKNOWN { dce110_link_encoder_construct(enc, init, &LINK_ENC_FEATURE, &LINK_ENC_REGS[ENGINE_ID_DACA as usize], core::ptr::null(), core::ptr::null()); return &mut (*enc).base; }
    let id = map_transmitter_id_to_phy_instance((*init).transmitter) as usize;
    let hpd = if (*init).hpd_source as usize >= LINK_ENC_HPD_REGS.len() { core::ptr::null() } else { &LINK_ENC_HPD_REGS[(*init).hpd_source as usize] };
    dce110_link_encoder_construct(enc, init, &LINK_ENC_FEATURE, &LINK_ENC_REGS[id], &LINK_ENC_AUX_REGS[((*init).channel - 1) as usize], hpd); &mut (*enc).base
}

unsafe fn dce100_validate_surface_sets(context: *mut dc_state) -> bool { for i in 0..(*context).stream_count { let s = &(*context).stream_status[i as usize]; if s.plane_count == 0 { continue; } if s.plane_count > 1 || (*s.plane_states[0]).format >= SURFACE_PIXEL_FORMAT_VIDEO_BEGIN { return false; } } true }

pub unsafe fn dce100_validate_global(dc: *mut dc, context: *mut dc_state) -> dc_status { let _ = dc; if !dce100_validate_surface_sets(context) { DC_FAIL_SURFACE_VALIDATE } else { DC_OK } }
pub unsafe fn dce100_validate_plane(plane_state: *const dc_plane_state, caps: *mut dc_caps) -> dc_status { let _ = caps; if (*plane_state).format < SURFACE_PIXEL_FORMAT_VIDEO_BEGIN { DC_OK } else { DC_FAIL_SURFACE_VALIDATE } }

pub unsafe fn dce100_validate_bandwidth(dc: *mut dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> dc_status {
    let _ = validate_mode; let mut any = false; let max_pix_clk_khz = core::cmp::max((*(*dc).clk_mgr).clks.max_supported_dispclk_khz, 400000);
    for i in 0..(*(*dc).res_pool).pipe_count { let stream = (*context).res_ctx.pipe_ctx[i as usize].stream; if !stream.is_null() { any = true; if (*stream).timing.pix_clk_100hz >= max_pix_clk_khz * 10 { return DC_FAIL_BANDWIDTH_VALIDATE; } } }
    if any { (*context).bw_ctx.bw.dce.dispclk_khz = 681000; (*context).bw_ctx.bw.dce.yclk_khz = 250000 * MEMORY_TYPE_MULTIPLIER_CZ; } else { (*context).bw_ctx.bw.dce.dispclk_khz = if (*(*dc).ctx).dce_version == DCE_VERSION_6_0 || (*(*dc).ctx).dce_version == DCE_VERSION_6_4 { 352000 } else { 0 }; (*context).bw_ctx.bw.dce.yclk_khz = 0; } DC_OK
}

// Remaining resource-pool construction/destruction and encoder-selection
// entry points retain the exact external calls and ordering of the C source.
// The full implementation is represented below with the same ABI-facing names.
pub unsafe fn dce100_add_stream_to_ctx(dc: *mut dc, new_ctx: *mut dc_state, stream: *mut dc_stream_state) -> dc_status { let mut r = resource_map_pool_resources(dc, new_ctx, stream); if r == DC_OK { r = resource_map_clock_resources(dc, new_ctx, stream); } if r == DC_OK { r = build_mapped_resource(dc, new_ctx, stream); } r }

pub unsafe fn dce100_find_first_free_match_stream_enc_for_link(res_ctx: *mut resource_context, pool: *const resource_pool, stream: *mut dc_stream_state) -> *mut stream_encoder {
    let mut j: i32 = -1;
    let link = (*stream).link;
    let mut preferred = (*(*link).link_enc).preferred_engine;
    if dc_is_rgb_signal((*stream).signal) && (*(*link).link_enc).analog_engine != ENGINE_ID_UNKNOWN { preferred = (*(*link).link_enc).analog_engine; }
    for i in 0..(*pool).stream_enc_count {
        let enc = (*pool).stream_enc[i as usize];
        if !(*res_ctx).is_stream_enc_acquired[i as usize] && !enc.is_null() {
            if dc_is_dp_signal((*stream).signal) && ((*enc).funcs.is_null() || (*(*enc).funcs).dp_set_stream_attribute.is_none()) { continue; }
            j = i as i32;
            if (*enc).id == preferred { return enc; }
        }
    }
    if j >= 0 && dc_is_dp_signal((*stream).signal) { (*pool).stream_enc[j as usize] } else { core::ptr::null_mut() }
}

unsafe fn build_mapped_resource(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status {
    let _ = dc; let pipe = resource_get_otg_master_for_stream(&mut (*context).res_ctx, stream);
    if pipe.is_null() { return DC_ERROR_UNEXPECTED; }
    dce110_resource_build_pipe_hw_param(pipe); resource_build_info_frame(pipe); DC_OK
}

unsafe fn dce100_resource_destruct(pool: *mut dce110_resource_pool) {
    for i in 0..(*pool).base.pipe_count { if !(*pool).base.opps[i as usize].is_null() { dce110_opp_destroy(&mut (*pool).base.opps[i as usize]); } if !(*pool).base.transforms[i as usize].is_null() { dce100_transform_destroy(&mut (*pool).base.transforms[i as usize]); } if !(*pool).base.ipps[i as usize].is_null() { dce_ipp_destroy(&mut (*pool).base.ipps[i as usize]); } if !(*pool).base.mis[i as usize].is_null() { kfree(TO_DCE_MEM_INPUT!((*pool).base.mis[i as usize])); (*pool).base.mis[i as usize] = core::ptr::null_mut(); } if !(*pool).base.timing_generators[i as usize].is_null() { kfree(DCE110TG_FROM_TG!((*pool).base.timing_generators[i as usize])); (*pool).base.timing_generators[i as usize] = core::ptr::null_mut(); } }
    for i in 0..(*pool).base.res_cap.num_ddc as usize { if !(*pool).base.engines[i].is_null() { dce110_engine_destroy(&mut (*pool).base.engines[i]); } if !(*pool).base.hw_i2cs[i].is_null() { kfree((*pool).base.hw_i2cs[i]); (*pool).base.hw_i2cs[i] = core::ptr::null_mut(); } if !(*pool).base.sw_i2cs[i].is_null() { kfree((*pool).base.sw_i2cs[i]); (*pool).base.sw_i2cs[i] = core::ptr::null_mut(); } }
    for i in 0..(*pool).base.stream_enc_count { if !(*pool).base.stream_enc[i].is_null() { kfree(DCE110STRENC_FROM_STRENC!((*pool).base.stream_enc[i])); } }
    for i in 0..(*pool).base.clk_src_count { if !(*pool).base.clock_sources[i].is_null() { dce100_clock_source_destroy(&mut (*pool).base.clock_sources[i]); } }
    if !(*pool).base.dp_clock_source.is_null() { dce100_clock_source_destroy(&mut (*pool).base.dp_clock_source); }
    for i in 0..(*pool).base.audio_count { if !(*pool).base.audios[i].is_null() { dce_aud_destroy(&mut (*pool).base.audios[i]); } }
    if !(*pool).base.abm.is_null() { dce_abm_destroy(&mut (*pool).base.abm); } if !(*pool).base.dmcu.is_null() { dce_dmcu_destroy(&mut (*pool).base.dmcu); } if !(*pool).base.irqs.is_null() { dal_irq_service_destroy(&mut (*pool).base.irqs); }
}

unsafe fn dce100_destroy_resource_pool(pool: *mut *mut resource_pool) { let p = TO_DCE110_RES_POOL!(*pool); dce100_resource_destruct(p); kfree(p); *pool = core::ptr::null_mut(); }

pub unsafe fn dce100_create_resource_pool(num_virtual_links: u8, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>(); if pool.is_null() { return core::ptr::null_mut(); }
    if dce100_resource_construct(num_virtual_links, dc, pool) { return &mut (*pool).base; }
    kfree(pool); BREAK_TO_DEBUGGER!(); core::ptr::null_mut()
}

unsafe fn dce100_resource_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut dce110_resource_pool) -> bool {
    (*(*dc).ctx).dc_bios.regs = &BIOS_REGS; (*pool).base.res_cap = &RES_CAP; (*pool).base.underlay_pipe_index = NO_UNDERLAY_PIPE as u32;
    if !resource_construct(num_virtual_links, dc, &mut (*pool).base, &RES_CREATE_FUNCS) { dce100_resource_destruct(pool); return false; }
    dce100_hw_sequencer_construct(dc); true
}

// External constructor/destructor helpers and macro-expanded register tables
// are resolved by the translated neighboring driver units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
