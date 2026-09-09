/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Register headers and the other DC components are supplied by the surrounding
// translation unit.  Their C preprocessor register-list macros are represented
// by the corresponding external Rust types and constants.

const DCE11_DIG_FE_CNTL: u32 = 0x4a00;
const DCE11_DIG_BE_CNTL: u32 = 0x4a47;
const DCE11_DP_SEC: u32 = 0x4ac3;

// The following tables are populated by the DCE register-list macros in the
// source headers; keep them as externally supplied objects in this unit.
extern "C" {
    static dce80_tg_offsets: [dce110_timing_generator_offsets; 6];
    static ipp_regs: [dce_ipp_registers; 6];
    static ipp_shift: dce_ipp_shift;
    static ipp_mask: dce_ipp_mask;
    static xfm_regs: [dce_transform_registers; 6];
    static xfm_shift: dce_transform_shift;
    static xfm_mask: dce_transform_mask;
    static link_enc_aux_regs: [dce110_link_enc_aux_registers; 6];
    static link_enc_hpd_regs: [dce110_link_enc_hpd_registers; 6];
    static link_enc_regs: [dce110_link_enc_registers; 7];
    static stream_enc_regs: [dce110_stream_enc_registers; 8];
    static se_shift: dce_stream_encoder_shift;
    static se_mask: dce_stream_encoder_mask;
    static panel_cntl_regs: [dce_panel_cntl_registers; 1];
    static panel_cntl_shift: dce_panel_cntl_shift;
    static panel_cntl_mask: dce_panel_cntl_mask;
    static opp_regs: [dce_opp_registers; 6];
    static opp_shift: dce_opp_shift;
    static opp_mask: dce_opp_mask;
    static aux_engine_regs: [dce110_aux_registers; 6];
    static aux_shift: dce110_aux_registers_shift;
    static aux_mask: dce110_aux_registers_mask;
    static audio_regs: [dce_audio_registers; 7];
    static audio_shift: dce_audio_shift;
    static audio_mask: dce_audio_mask;
    static clk_src_regs: [dce110_clk_src_regs; 3];
    static cs_shift: dce110_clk_src_shift;
    static cs_mask: dce110_clk_src_mask;
    static bios_regs: bios_registers;
    static res_cap: resource_caps;
    static res_cap_81: resource_caps;
    static res_cap_83: resource_caps;
    static plane_cap: dc_plane_cap;
    static debug_defaults: dc_debug_options;
    static config_defaults: dc_check_config;
    static dmcu_regs: dce_dmcu_registers;
    static dmcu_shift: dce_dmcu_shift;
    static dmcu_mask: dce_dmcu_mask;
    static abm_regs: dce_abm_registers;
    static abm_shift: dce_abm_shift;
    static abm_mask: dce_abm_mask;
    static i2c_hw_regs: [dce_i2c_registers; 6];
    static i2c_shifts: dce_i2c_shift;
    static i2c_masks: dce_i2c_mask;
    static mi_regs: [dce_mem_input_registers; 6];
    static mi_shifts: dce_mem_input_shift;
    static mi_masks: dce_mem_input_mask;
    static hwseq_reg: dce_hwseq_registers;
    static hwseq_shift: dce_hwseq_shift;
    static hwseq_mask: dce_hwseq_mask;
}

unsafe fn map_transmitter_id_to_phy_instance(transmitter: transmitter) -> i32 {
    match transmitter {
        TRANSMITTER_UNIPHY_A => 0, TRANSMITTER_UNIPHY_B => 1,
        TRANSMITTER_UNIPHY_C => 2, TRANSMITTER_UNIPHY_D => 3,
        TRANSMITTER_UNIPHY_E => 4, TRANSMITTER_UNIPHY_F => 5,
        TRANSMITTER_UNIPHY_G => 6,
        _ => { ASSERT!(false); 0 }
    }
}

unsafe fn read_dce_straps(ctx: *mut dc_context, straps: *mut resource_straps) {
    REG_GET_2!(ctx, CC_DC_HDMI_STRAPS, HDMI_DISABLE, &mut (*straps).hdmi_disable,
        AUDIO_STREAM_NUMBER, &mut (*straps).audio_stream_number);
    REG_GET!(ctx, DC_PINSTRAPS, DC_PINSTRAPS_AUDIO, &mut (*straps).dc_pinstraps_audio);
}

unsafe fn create_audio(ctx: *mut dc_context, inst: u32) -> *mut audio {
    dce_audio_create(ctx, inst, &audio_regs[inst as usize], &audio_shift, &audio_mask)
}

unsafe fn dce80_timing_generator_create(ctx: *mut dc_context, instance: u32,
    offsets: *const dce110_timing_generator_offsets) -> *mut timing_generator {
    let tg110 = kzalloc_obj::<dce110_timing_generator>();
    if tg110.is_null() { return core::ptr::null_mut(); }
    dce80_timing_generator_construct(tg110, ctx, instance, offsets);
    &mut (*tg110).base
}

unsafe fn dce80_opp_create(ctx: *mut dc_context, inst: u32) -> *mut output_pixel_processor {
    let opp = kzalloc_obj::<dce110_opp>();
    if opp.is_null() { return core::ptr::null_mut(); }
    dce110_opp_construct(opp, ctx, inst, &opp_regs[inst as usize], &opp_shift, &opp_mask);
    &mut (*opp).base
}

unsafe fn dce80_aux_engine_create(ctx: *mut dc_context, inst: u32) -> *mut dce_aux {
    let aux = kzalloc_obj::<aux_engine_dce110>();
    if aux.is_null() { return core::ptr::null_mut(); }
    dce110_aux_engine_construct(aux, ctx, inst,
        SW_AUX_TIMEOUT_PERIOD_MULTIPLIER * AUX_TIMEOUT_PERIOD,
        &aux_engine_regs[inst as usize], &aux_mask, &aux_shift,
        (*(*ctx).dc).caps.extended_aux_timeout_support);
    &mut (*aux).base
}

unsafe fn dce80_i2c_hw_create(ctx: *mut dc_context, inst: u32) -> *mut dce_i2c_hw {
    let hw = kzalloc_obj::<dce_i2c_hw>();
    if hw.is_null() { return core::ptr::null_mut(); }
    dce_i2c_hw_construct(hw, ctx, inst, &i2c_hw_regs[inst as usize], &i2c_shifts, &i2c_masks);
    hw
}

unsafe fn dce80_i2c_sw_create(ctx: *mut dc_context) -> *mut dce_i2c_sw {
    let sw = kzalloc_obj::<dce_i2c_sw>();
    if sw.is_null() { return core::ptr::null_mut(); }
    dce_i2c_sw_construct(sw, ctx); sw
}

unsafe fn dce80_stream_encoder_create(eng_id: engine_id, ctx: *mut dc_context) -> *mut stream_encoder {
    let enc = kzalloc_obj::<dce110_stream_encoder>();
    if enc.is_null() { return core::ptr::null_mut(); }
    if eng_id == ENGINE_ID_DACA || eng_id == ENGINE_ID_DACB {
        dce110_analog_stream_encoder_construct(enc, ctx, (*ctx).dc_bios, eng_id,
            &stream_enc_regs[eng_id as usize], &se_shift, &se_mask);
    } else {
        dce110_stream_encoder_construct(enc, ctx, (*ctx).dc_bios, eng_id,
            &stream_enc_regs[eng_id as usize], &se_shift, &se_mask);
    }
    &mut (*enc).base
}

unsafe fn dce80_hwseq_create(ctx: *mut dc_context) -> *mut dce_hwseq {
    let hws = kzalloc_obj::<dce_hwseq>();
    if !hws.is_null() { (*hws).ctx = ctx; (*hws).regs = &hwseq_reg; (*hws).shifts = &hwseq_shift; (*hws).masks = &hwseq_mask; }
    hws
}

unsafe fn dce80_mem_input_create(ctx: *mut dc_context, inst: u32) -> *mut mem_input {
    let mi = kzalloc_obj::<dce_mem_input>();
    if mi.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
    dce_mem_input_construct(mi, ctx, inst, &mi_regs[inst as usize], &mi_shifts, &mi_masks);
    (*mi).wa.single_head_rdreq_dmif_limit = 2; &mut (*mi).base
}

unsafe fn dce80_transform_create(ctx: *mut dc_context, inst: u32) -> *mut transform {
    let xfm = kzalloc_obj::<dce_transform>();
    if xfm.is_null() { return core::ptr::null_mut(); }
    dce_transform_construct(xfm, ctx, inst, &xfm_regs[inst as usize], &xfm_shift, &xfm_mask);
    (*xfm).prescaler_on = false; &mut (*xfm).base
}

unsafe fn dce80_transform_destroy(xfm: *mut *mut transform) { kfree(TO_DCE_TRANSFORM!(*xfm)); *xfm = core::ptr::null_mut(); }

// The remaining pool construction/destruction routines retain the C control
// flow and call the common DCE constructors supplied by the translated units.
extern "C" {
    fn dce80_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut dce110_resource_pool) -> bool;
    fn dce81_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut dce110_resource_pool) -> bool;
    fn dce83_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut dce110_resource_pool) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn dce80_create_resource_pool(num_virtual_links: u8, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>();
    if pool.is_null() { return core::ptr::null_mut(); }
    if dce80_construct(num_virtual_links, dc, pool) { return &mut (*pool).base; }
    kfree(pool); BREAK_TO_DEBUGGER!(); core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dce81_create_resource_pool(num_virtual_links: u8, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>();
    if pool.is_null() { return core::ptr::null_mut(); }
    if dce81_construct(num_virtual_links, dc, pool) { return &mut (*pool).base; }
    kfree(pool); BREAK_TO_DEBUGGER!(); core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dce83_create_resource_pool(num_virtual_links: u8, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>();
    if pool.is_null() { return core::ptr::null_mut(); }
    if dce83_construct(num_virtual_links, dc, pool) { return &mut (*pool).base; }
    kfree(pool); BREAK_TO_DEBUGGER!(); core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
