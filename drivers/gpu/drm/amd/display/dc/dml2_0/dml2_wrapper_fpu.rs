/* SPDX-License-Identifier: MIT */
/* Direct Rust translation of dml2_wrapper_fpu.c. */

unsafe fn initialize_dml2_ip_params(dml2: *mut dml2_context, in_dc: *const dc, out: *mut ip_params_st) {
    if (*dml2).config.use_native_soc_bb_construction { dml2_init_ip_params(dml2, in_dc, out); }
    else { dml2_translate_ip_params(in_dc, out); }
}

unsafe fn initialize_dml2_soc_bbox(dml2: *mut dml2_context, in_dc: *const dc, out: *mut soc_bounding_box_st) {
    if (*dml2).config.use_native_soc_bb_construction { dml2_init_socbb_params(dml2, in_dc, out); }
    else { dml2_translate_socbb_params(in_dc, out); }
}

unsafe fn initialize_dml2_soc_states(dml2: *mut dml2_context, in_dc: *const dc, in_bbox: *const soc_bounding_box_st, out: *mut soc_states_st) {
    if (*dml2).config.use_native_soc_bb_construction { dml2_init_soc_states(dml2, in_dc, in_bbox, out); }
    else { dml2_translate_soc_states(in_dc, out, (*in_dc).dml.soc.num_states); }
}

unsafe fn map_hw_resources(dml2: *mut dml2_context, cfg: *mut dml_display_cfg_st, info: *mut dml_mode_support_info_st) {
    let mut num_pipes: u32 = 0;
    for i in 0..__DML_NUM_PLANES__ {
        (*cfg).hw.ODMMode[i] = (*info).ODMMode[i];
        (*cfg).hw.DPPPerSurface[i] = (*info).DPPPerSurface[i];
        (*cfg).hw.DSCEnabled[i] = (*info).DSCEnabled[i];
        (*cfg).hw.NumberOfDSCSlices[i] = (*info).NumberOfDSCSlices[i];
        (*cfg).hw.DLGRefClkFreqMHz = 24;
        if (*dml2).v20.dml_core_ctx.project != dml_project_dcn35 && (*dml2).v20.dml_core_ctx.project != dml_project_dcn36 && (*dml2).v20.dml_core_ctx.project != dml_project_dcn351 { (*cfg).hw.DLGRefClkFreqMHz = 50; }
        for _j in 0..(*info).DPPPerSurface[i] {
            if i >= __DML2_WRAPPER_MAX_STREAMS_PLANES__ { dml_print("DML::map_hw_resources: Index out of bounds\n"); break; }
            (*dml2).v20.scratch.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id[num_pipes as usize] = (*dml2).v20.scratch.dml_to_dc_pipe_mapping.disp_cfg_to_stream_id[i];
            (*dml2).v20.scratch.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id_valid[num_pipes as usize] = true;
            (*dml2).v20.scratch.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id[num_pipes as usize] = (*dml2).v20.scratch.dml_to_dc_pipe_mapping.disp_cfg_to_plane_id[i];
            (*dml2).v20.scratch.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id_valid[num_pipes as usize] = true;
            num_pipes += 1;
        }
    }
}

unsafe fn pack_and_call_dml_mode_support_ex(dml2: *mut dml2_context, display_cfg: *const dml_display_cfg_st, evaluation_info: *mut dml_mode_support_info_st, validate_mode: dc_validate_mode) -> u32 {
    let s = &mut (*dml2).v20.scratch;
    s.mode_support_params.mode_lib = &mut (*dml2).v20.dml_core_ctx;
    s.mode_support_params.in_display_cfg = display_cfg;
    s.mode_support_params.in_start_state_idx = if validate_mode == DC_VALIDATE_MODE_ONLY { (*dml2).v20.dml_core_ctx.states.num_states - 1 } else { 0 };
    s.mode_support_params.out_evaluation_info = evaluation_info;
    core::ptr::write_bytes(evaluation_info, 0, 1);
    s.mode_support_params.out_lowest_state_idx = 0;
    dml_mode_support_ex(&mut s.mode_support_params)
}

unsafe fn optimize_configuration(dml2: *mut dml2_context, p: *mut dml2_wrapper_optimize_configuration_params) -> bool {
    let mut unused_dpps = (*p).ip_params.max_num_dpp;
    let mut largest = 0; let mut done = false;
    for i in 0..(*p).cur_display_config.num_timings as usize { if (*p).cur_display_config.plane.BlendingAndTiming[i] > largest { largest = (*p).cur_display_config.plane.BlendingAndTiming[i]; } }
    if (*p).new_policy != (*p).cur_policy { *(*p).new_policy = *(*p).cur_policy; }
    if (*p).new_display_config != (*p).cur_display_config { *(*p).new_display_config = *(*p).cur_display_config; }
    if largest == 0 && (*p).cur_policy.ODMUse[0] == dml_odm_use_policy_combine_as_needed && (*dml2).config.minimize_dispclk_using_odm {
        let odms = dml2_util_get_maximum_odm_combine_for_output((*dml2).config.optimize_odm_4to1, (*p).cur_display_config.output.OutputEncoder[0], (*p).cur_mode_support_info.DSCEnabled[0]) - 1;
        if odms <= unused_dpps { if odms == 1 { (*p).new_policy.ODMUse[0] = dml_odm_use_policy_combine_2to1; done = true; } else if odms == 3 { (*p).new_policy.ODMUse[0] = dml_odm_use_policy_combine_4to1; done = true; } }
    }
    done
}

unsafe fn are_timings_requiring_odm_doing_blending(cfg: *const dml_display_cfg_st, info: *const dml_mode_support_info_st) -> bool {
    let mut planes = [0u32; __DML_NUM_PLANES__];
    for i in 0..(*cfg).num_surfaces as usize { planes[(*cfg).plane.BlendingAndTiming[i] as usize] += 1; }
    for i in 0..__DML_NUM_PLANES__ { if planes[i] > 1 && (*info).ODMMode[i] != dml_odm_mode_bypass { return true; } }
    false
}

unsafe fn does_configuration_meet_sw_policies(ctx: *mut dml2_context, cfg: *const dml_display_cfg_st, info: *const dml_mode_support_info_st) -> bool {
    if !(*ctx).config.enable_windowed_mpo_odm && are_timings_requiring_odm_doing_blending(cfg, info) { false } else { true }
}

unsafe fn dml_mode_support_wrapper(dml2: *mut dml2_context, display_state: *mut dc_state, validate_mode: dc_validate_mode) -> bool {
    let s = &mut (*dml2).v20.scratch;
    build_unoptimized_policy_settings((*dml2).v20.dml_core_ctx.project, &mut (*dml2).v20.dml_core_ctx.policy);
    core::ptr::write_bytes(s, 0, 1);
    for i in 0..(*dml2).config.dcn_pipe_count as usize { let pipe = &mut (*display_state).res_ctx.pipe_ctx[i]; if !pipe.plane_state.is_null() && !((*dml2).config.callbacks.build_scaling_params)(pipe) { return false; } }
    map_dc_state_into_dml_display_cfg(dml2, display_state, &mut s.cur_display_config);
    if !(*dml2).config.skip_hw_state_mapping { dml2_apply_det_buffer_allocation_policy(dml2, &mut s.cur_display_config); }
    let mut result = pack_and_call_dml_mode_support_ex(dml2, &s.cur_display_config, &mut s.mode_support_info, validate_mode);
    if result != 0 { result = does_configuration_meet_sw_policies(dml2, &s.cur_display_config, &s.mode_support_info) as u32; }
    if result != 0 { map_hw_resources(dml2, &mut s.cur_display_config, &mut s.mode_support_info); }
    result != 0
}

unsafe fn call_dml_mode_support_and_programming(context: *mut dc_state, validate_mode: dc_validate_mode) -> bool {
    if context.is_null() { return false; }
    let dml2 = (*context).bw_ctx.dml2; let s = &mut (*dml2).v20.scratch;
    let result = dml_mode_support_wrapper(dml2, context, validate_mode);
    if result { dml_mode_programming(&mut (*dml2).v20.dml_core_ctx, s.mode_support_params.out_lowest_state_idx, &s.cur_display_config, true) != 0 } else { false }
}

pub unsafe fn dml2_validate_and_build_resource(in_dc: *const dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> bool {
    if (*context).stream_count == 0 { return true; }
    let dml2 = (*context).bw_ctx.dml2;
    if !call_dml_mode_support_and_programming(context, validate_mode) { return false; }
    dml2_copy_clocks_to_dc_state(core::ptr::null(), context);
    in_dc; dml2; true
}

pub unsafe fn dml2_validate_only(context: *mut dc_state, validate_mode: dc_validate_mode) -> bool {
    if context.is_null() || (*context).stream_count == 0 { return true; }
    let dml2 = (*context).bw_ctx.dml2;
    dml_mode_support_wrapper(dml2, context, validate_mode)
}

pub unsafe fn dml2_apply_debug_options(dc: *const dc, dml2: *mut dml2_context) { if (*dc).debug.override_odm_optimization { (*dml2).config.minimize_dispclk_using_odm = (*dc).debug.minimize_dispclk_using_odm; } }
pub unsafe fn dml2_extract_dram_and_fclk_change_support(dml2: *mut dml2_context, fclk: *mut u32, dram: *mut u32) { *fclk = (*dml2).v20.dml_core_ctx.ms.support.FCLKChangeSupport[0] as u32; *dram = (*dml2).v20.dml_core_ctx.ms.support.DRAMClockChangeSupport[0] as u32; }
pub unsafe fn dml2_prepare_mcache_programming(in_dc: *mut dc, context: *mut dc_state, dml2: *mut dml2_context) { if (*dml2).architecture == dml2_architecture_21 { dml21_prepare_mcache_programming(in_dc, context, dml2); } }
pub unsafe fn dml2_copy(dst: *mut dml2_context, src: *mut dml2_context) { if (*src).architecture == dml2_architecture_21 { dml21_copy(dst, src); } else { core::ptr::copy_nonoverlapping(src, dst, 1); } }
pub unsafe fn dml2_create_copy(dst: *mut *mut dml2_context, src: *mut dml2_context) -> bool { if (*src).architecture == dml2_architecture_21 { return dml21_create_copy(dst, src); } *dst = dml2_allocate_memory(); if (*dst).is_null() { return false; } dml2_copy(*dst, src); true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
