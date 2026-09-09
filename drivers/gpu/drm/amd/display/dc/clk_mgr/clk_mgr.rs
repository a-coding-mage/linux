/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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

// Declarations supplied by the surrounding driver translation.

pub unsafe fn clk_mgr_helper_get_active_display_cnt(dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut display_count: i32 = 0;
    for i in 0..(*context).stream_count {
        let stream = (*context).streams[i as usize];
        let stream_status = &(*context).stream_status[i as usize];

        /* Don't count SubVP phantom pipes as part of active display count */
        if dc_state_get_stream_subvp_type(context, stream) == SUBVP_PHANTOM {
            continue;
        }

        if !(*stream).dpms_off || (*dc).is_switch_in_progress_dest || ((*stream_status).plane_count != 0) {
            display_count += 1;
        }
    }
    display_count
}

pub unsafe fn clk_mgr_helper_get_active_plane_cnt(_dc: *mut dc, context: *mut dc_state) -> i32 {
    let mut total_plane_count: i32 = 0;
    for i in 0..(*context).stream_count {
        let stream_status = (*context).stream_status[i as usize];
        /* Sum up plane_count for all streams ( active and virtual ). */
        total_plane_count += stream_status.plane_count;
    }
    total_plane_count
}

pub unsafe fn clk_mgr_exit_optimized_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr) {
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
    let mut edp_num: u32 = 0;
    dc_get_edp_links(dc, edp_links.as_mut_ptr(), &mut edp_num);
    if let Some(f) = (*dc).hwss.exit_optimized_pwr_state {
        f(dc, (*dc).current_state);
    }
    for panel_inst in 0..edp_num {
        let edp_link = edp_links[panel_inst as usize];
        let mut allow_active = false;
        if !(*edp_link).psr_settings.psr_feature_enabled { continue; }
        (*clk_mgr).psr_allow_active_cache = (*edp_link).psr_settings.psr_allow_active;
        (*dc).link_srv.edp_set_psr_allow_active(edp_link, &mut allow_active, false, false, core::ptr::null_mut());
        (*dc).link_srv.edp_set_replay_allow_active(edp_link, &mut allow_active, false, false, core::ptr::null_mut());
    }
}

pub unsafe fn clk_mgr_optimize_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr) {
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP] = [core::ptr::null_mut(); MAX_NUM_EDP];
    let mut edp_num: u32 = 0;
    dc_get_edp_links(dc, edp_links.as_mut_ptr(), &mut edp_num);
    for panel_inst in 0..edp_num {
        let edp_link = edp_links[panel_inst as usize];
        if !(*edp_link).psr_settings.psr_feature_enabled { continue; }
        (*dc).link_srv.edp_set_psr_allow_active(edp_link, &mut (*clk_mgr).psr_allow_active_cache, false, false, core::ptr::null_mut());
        (*dc).link_srv.edp_set_replay_allow_active(edp_link, &mut (*clk_mgr).psr_allow_active_cache, false, false, core::ptr::null_mut());
    }
    if let Some(f) = (*dc).hwss.optimize_pwr_state { f(dc, (*dc).current_state); }
}

pub unsafe fn dc_clk_mgr_create(ctx: *mut dc_context, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) -> *mut clk_mgr {
    let asic_id = (*ctx).asic_id;
    match asic_id.chip_family {
        FAMILY_SI | FAMILY_CI | FAMILY_KV => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            dce_clk_mgr_construct(ctx, clk_mgr); &mut (*clk_mgr).base
        }
        FAMILY_CZ => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            dce110_clk_mgr_construct(ctx, clk_mgr); &mut (*clk_mgr).base
        }
        FAMILY_VI => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            if ASIC_REV_IS_TONGA_P(asic_id.hw_internal_rev) || ASIC_REV_IS_FIJI_P(asic_id.hw_internal_rev) { dce_clk_mgr_construct(ctx, clk_mgr); }
            else if ASIC_REV_IS_POLARIS10_P(asic_id.hw_internal_rev) || ASIC_REV_IS_POLARIS11_M(asic_id.hw_internal_rev) || ASIC_REV_IS_POLARIS12_V(asic_id.hw_internal_rev) || ASIC_REV_IS_VEGAM(asic_id.hw_internal_rev) { dce112_clk_mgr_construct(ctx, clk_mgr); }
            &mut (*clk_mgr).base
        }
        FAMILY_AI => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            if ASICREV_IS_VEGA20_P(asic_id.hw_internal_rev) { dce121_clk_mgr_construct(ctx, clk_mgr); } else { dce120_clk_mgr_construct(ctx, clk_mgr); }
            &mut (*clk_mgr).base
        }
        // The following constructors are enabled by CONFIG_DRM_AMD_DC_FP in the C source.
        FAMILY_RV => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            if ASICREV_IS_RENOIR(asic_id.hw_internal_rev) || ASICREV_IS_GREEN_SARDINE(asic_id.hw_internal_rev) { rn_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); }
            else if ASICREV_IS_RAVEN2(asic_id.hw_internal_rev) { rv2_clk_mgr_construct(ctx, clk_mgr, pp_smu); }
            else if ASICREV_IS_RAVEN(asic_id.hw_internal_rev) || ASICREV_IS_PICASSO(asic_id.hw_internal_rev) { rv1_clk_mgr_construct(ctx, clk_mgr, pp_smu); }
            &mut (*clk_mgr).base
        }
        FAMILY_NV => {
            let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
            if (*ctx).dce_version == DCN_VERSION_2_01 { dcn201_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); }
            else if ASICREV_IS_SIENNA_CICHLID_P(asic_id.hw_internal_rev) || ASICREV_IS_DIMGREY_CAVEFISH_P(asic_id.hw_internal_rev) || ASICREV_IS_BEIGE_GOBY_P(asic_id.hw_internal_rev) { dcn3_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); }
            else { dcn20_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); }
            &mut (*clk_mgr).base
        }
        FAMILY_VGH => {
            if ASICREV_IS_VANGOGH(asic_id.hw_internal_rev) { let clk_mgr = kzalloc_obj::<clk_mgr_vgh>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } vg_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); return &mut (*clk_mgr).base.base; }
            core::ptr::null_mut()
        }
        FAMILY_YELLOW_CARP => { let clk_mgr = kzalloc_obj::<clk_mgr_dcn31>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn31_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base }
        AMDGPU_FAMILY_GC_10_3_6 => { let clk_mgr = kzalloc_obj::<clk_mgr_dcn315>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn315_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base }
        AMDGPU_FAMILY_GC_10_3_7 => { let clk_mgr = kzalloc_obj::<clk_mgr_dcn316>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn316_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base }
        AMDGPU_FAMILY_GC_11_0_0 => { let clk_mgr = kzalloc_obj::<clk_mgr_internal>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn32_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base }
        AMDGPU_FAMILY_GC_11_0_1 => { let clk_mgr = kzalloc_obj::<clk_mgr_dcn314>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn314_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base }
        AMDGPU_FAMILY_GC_11_5_0 => { if (*ctx).dce_version == DCN_VERSION_4_2B { let clk_mgr = kzalloc_obj::<clk_mgr_dcn42>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn42b_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base } else { let clk_mgr = kzalloc_obj::<clk_mgr_dcn35>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } if (*ctx).dce_version == DCN_VERSION_3_51 { dcn351_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); } else { dcn35_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); } &mut (*clk_mgr).base.base } }
        AMDGPU_FAMILY_GC_12_0_0 => { let clk_mgr = dcn401_clk_mgr_construct(ctx, dccg); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } &mut (*clk_mgr).base }
        AMDGPU_FAMILY_GC_11_5_4 => { let clk_mgr = kzalloc_obj::<clk_mgr_dcn42>(); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } dcn42_clk_mgr_construct(ctx, clk_mgr, pp_smu, dccg); &mut (*clk_mgr).base.base }
        AMDGPU_FAMILY_GC_13_0_1 => { let clk_mgr = dcn60_clk_mgr_construct(ctx, dccg); if clk_mgr.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); } &mut (*clk_mgr).base }
        _ => { ASSERT!(false); core::ptr::null_mut() }
    }
}

pub unsafe fn dc_destroy_clk_mgr(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    // CONFIG_DRM_AMD_DC_FP-gated destruction dispatch is supplied by the build configuration.
    match (*clk_mgr_base).ctx.asic_id.chip_family {
        FAMILY_NV => { if ASICREV_IS_SIENNA_CICHLID_P((*clk_mgr_base).ctx.asic_id.hw_internal_rev) || ASICREV_IS_DIMGREY_CAVEFISH_P((*clk_mgr_base).ctx.asic_id.hw_internal_rev) || ASICREV_IS_BEIGE_GOBY_P((*clk_mgr_base).ctx.asic_id.hw_internal_rev) { dcn3_clk_mgr_destroy(clk_mgr); } }
        FAMILY_VGH => { if ASICREV_IS_VANGOGH((*clk_mgr_base).ctx.asic_id.hw_internal_rev) { vg_clk_mgr_destroy(clk_mgr); } }
        FAMILY_YELLOW_CARP => dcn31_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_10_3_6 => dcn315_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_10_3_7 => dcn316_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_11_0_0 => dcn32_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_11_0_1 => dcn314_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_11_5_0 => { if (*clk_mgr_base).ctx.dce_version == DCN_VERSION_4_2B { dcn42_clk_mgr_destroy(clk_mgr); } else { dcn35_clk_mgr_destroy(clk_mgr); } }
        AMDGPU_FAMILY_GC_12_0_0 => dcn401_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_11_5_4 => dcn42_clk_mgr_destroy(clk_mgr),
        AMDGPU_FAMILY_GC_13_0_1 => dcn60_clk_mgr_destroy(clk_mgr),
        _ => {}
    }
    kfree(clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
