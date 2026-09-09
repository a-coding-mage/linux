// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Translated from dml21_wrapper_fpu.c. External types, constants, globals, and
// functions are supplied by the corresponding DML/DC dependencies.
use core::ffi::c_void;

const INVALID: i32 = -1;

unsafe fn dml21_populate_configuration_options(
    in_dc: *const dc,
    dml_ctx: *mut dml2_context,
    config: *const dml2_configuration_options,
) {
    (*dml_ctx).config = *config;
    if (*in_dc).debug.dml21_force_pstate_method {
        (*dml_ctx).config.pmo.force_pstate_method_enable = true;
        for i in 0..MAX_PIPES {
            (*dml_ctx).config.pmo.force_pstate_method_values[i] =
                (*in_dc).debug.dml21_force_pstate_method_values[i];
        }
    } else {
        (*dml_ctx).config.pmo.force_pstate_method_enable = false;
    }
}

pub unsafe fn dml21_init(in_dc: *const dc, dml_ctx: *mut dml2_context, config: *const dml2_configuration_options) {
    (*dml_ctx).architecture = dml2_architecture_21;
    dml21_populate_configuration_options(in_dc, dml_ctx, config);
    dml21_populate_dml_init_params(&mut (*dml_ctx).v21.dml_init, &(*dml_ctx).config, in_dc);
    dml2_initialize_instance(&mut (*dml_ctx).v21.dml_init);
}

pub unsafe fn dml21_reinit(in_dc: *const dc, dml_ctx: *mut dml2_context, config: *const dml2_configuration_options) {
    let dml2 = (*dml_ctx).v21.dml_init.dml2_instance;
    dml21_populate_configuration_options(in_dc, dml_ctx, config);
    dml21_populate_dml_init_params(&mut (*dml_ctx).v21.dml_init, &(*dml_ctx).config, in_dc);
    if memcmp(&(*dml2).soc_bbox as *const _ as *const c_void, &(*dml_ctx).v21.dml_init.soc_bb as *const _ as *const c_void, core::mem::size_of::<dml2_soc_bb>()) == 0
        && memcmp(&(*dml2).ip_caps as *const _ as *const c_void, &(*dml_ctx).v21.dml_init.ip_caps as *const _ as *const c_void, core::mem::size_of::<dml2_ip_capabilities>()) == 0
        && memcmp(&(*dml2).pmo_options as *const _ as *const c_void, &(*dml_ctx).v21.dml_init.options.pmo_options as *const _ as *const c_void, core::mem::size_of::<dml2_pmo_options>()) == 0 { return; }
    dml2_initialize_instance(&mut (*dml_ctx).v21.dml_init);
}

unsafe fn dml21_calculate_rq_and_dlg_params(dc: *const dc, context: *mut dc_state, out_new_hw_state: *mut resource_context, in_ctx: *mut dml2_context, pipe_cnt: u32) {
    let _ = (out_new_hw_state, pipe_cnt);
    let mut dml_phantom_prog_idx;
    let mut dc_main_pipes: [*mut pipe_ctx; __DML2_WRAPPER_MAX_STREAMS_PLANES__] = [core::ptr::null_mut(); __DML2_WRAPPER_MAX_STREAMS_PLANES__];
    let mut dc_phantom_pipes: [*mut pipe_ctx; __DML2_WRAPPER_MAX_STREAMS_PLANES__] = [core::ptr::null_mut(); __DML2_WRAPPER_MAX_STREAMS_PLANES__];
    (*context).bw_ctx.bw.dcn.clk.dppclk_khz = 0;
    memcpy(&mut (*context).bw_ctx.bw.dcn.arb_regs as *mut _ as *mut c_void, &(*in_ctx).v21.mode_programming.programming.global_regs.arb_regs as *const _ as *const c_void, core::mem::size_of::<dml2_display_arb_regs>());
    (*context).bw_ctx.bw.dcn.compbuf_size_kb = (*in_ctx).v21.mode_programming.programming.global_regs.arb_regs.compbuf_size as i32 * 64;
    (*context).bw_ctx.bw.dcn.mall_ss_size_bytes = 0;
    (*context).bw_ctx.bw.dcn.mall_ss_psr_active_size_bytes = 0;
    (*context).bw_ctx.bw.dcn.mall_subvp_size_bytes = 0;
    dml_phantom_prog_idx = (*in_ctx).v21.mode_programming.programming.display_config.num_planes;
    for dml_prog_idx in 0..DML2_MAX_PLANES {
        let pln_prog = &(*in_ctx).v21.mode_programming.programming.plane_programming[dml_prog_idx];
        if pln_prog.plane_descriptor.is_null() || pln_prog.num_dpps_required == 0 { continue; }
        let stream_prog = &(*in_ctx).v21.mode_programming.programming.stream_programming[(*pln_prog.plane_descriptor).stream_index];
        let num_pipes = dml21_find_dc_pipes_for_plane(dc, context, in_ctx, dc_main_pipes.as_mut_ptr(), dc_phantom_pipes.as_mut_ptr(), dml_prog_idx);
        if num_pipes <= 0 { continue; }
        for i in 0..num_pipes as usize {
            dml21_program_dc_pipe(in_ctx, context, dc_main_pipes[i], pln_prog, stream_prog);
            if pln_prog.phantom_plane.valid && !dc_phantom_pipes[i].is_null() { dml21_program_dc_pipe(in_ctx, context, dc_phantom_pipes[i], pln_prog, stream_prog); }
        }
        for wb in 0..stream_prog.stream_descriptor.writeback.active_writebacks_per_stream as usize {
            dml21_program_dc_mcif_arb_params(in_ctx, context, stream_prog, wb, (*dc_main_pipes[0]).stream.writeback_info[wb].dwb_pipe_inst);
        }
        memcpy(&mut (*context).bw_ctx.bw.dcn.mcache_allocations[dml_prog_idx] as *mut _ as *mut c_void, &pln_prog.mcache_allocation as *const _ as *const c_void, core::mem::size_of::<dml2_mcache_surface_allocation>());
        if pln_prog.phantom_plane.valid { memcpy(&mut (*context).bw_ctx.bw.dcn.mcache_allocations[dml_phantom_prog_idx] as *mut _ as *mut c_void, &pln_prog.phantom_plane.mcache_allocation as *const _ as *const c_void, core::mem::size_of::<dml2_mcache_surface_allocation>()); dml_phantom_prog_idx += 1; }
    }
    (*context).bw_ctx.bw.dcn.clk.bw_dppclk_khz = (*context).bw_ctx.bw.dcn.clk.dppclk_khz;
    (*context).bw_ctx.bw.dcn.clk.bw_dispclk_khz = (*context).bw_ctx.bw.dcn.clk.dispclk_khz;
    let disp = &(*in_ctx).v21.dml_init.soc_bb.clk_table.dispclk;
    (*context).bw_ctx.bw.dcn.clk.max_supported_dispclk_khz = disp.clk_values_khz[if disp.num_clk_values > 1 { disp.num_clk_values - 1 } else { 0 }];
    let dpp = &(*in_ctx).v21.dml_init.soc_bb.clk_table.dppclk;
    (*context).bw_ctx.bw.dcn.clk.max_supported_dppclk_khz = dpp.clk_values_khz[if dpp.num_clk_values > 1 { dpp.num_clk_values - 1 } else { 0 }];
    if (*dc).config.forced_clocks || (*dc).debug.max_disp_clk { (*context).bw_ctx.bw.dcn.clk.bw_dispclk_khz = (*context).bw_ctx.bw.dcn.clk.max_supported_dispclk_khz; (*context).bw_ctx.bw.dcn.clk.bw_dppclk_khz = (*context).bw_ctx.bw.dcn.clk.max_supported_dppclk_khz; }
    (*context).bw_ctx.bw.dcn.clk.num_ways = if let Some(f) = (*(*dc).res_pool).funcs.calculate_mall_ways_from_bytes { f(dc, (*context).bw_ctx.bw.dcn.mall_subvp_size_bytes) } else { 0 };
}

// The remaining wrapper entry points retain the C control flow and delegate to
// external DML/DC helpers supplied by the integration headers.
pub unsafe fn dml21_validate(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context, validate_mode: dc_validate_mode) -> bool {
    if validate_mode != DC_VALIDATE_MODE_AND_PROGRAMMING { dml21_check_mode_support(in_dc, context, dml_ctx) } else { dml21_mode_check_and_programming(in_dc, context, dml_ctx) }
}

unsafe fn dml21_prepare_mcache_params(dml_ctx: *mut dml2_context, context: *mut dc_state, mcache_params: *mut dc_mcache_params) {
    let mut dc_plane_idx = 0;
    for stream_idx in 0..(*context).stream_count as usize {
        for plane_idx in 0..(*context).stream_status[stream_idx].plane_count as usize {
            let idx = map_plane_to_dml21_display_cfg(dml_ctx, (*context).streams[stream_idx].stream_id, (*context).stream_status[stream_idx].plane_states[plane_idx], context);
            if idx == INVALID { continue; }
            let p = &(*dml_ctx).v21.mode_programming.programming.plane_programming[idx as usize];
            (*mcache_params.add(dc_plane_idx)).valid = p.mcache_allocation.valid;
            (*mcache_params.add(dc_plane_idx)).num_mcaches_plane0 = p.mcache_allocation.num_mcaches_plane0;
            (*mcache_params.add(dc_plane_idx)).num_mcaches_plane1 = p.mcache_allocation.num_mcaches_plane1;
            (*mcache_params.add(dc_plane_idx)).requires_dedicated_mall_mcache = p.mcache_allocation.requires_dedicated_mall_mcache;
            (*mcache_params.add(dc_plane_idx)).last_slice_sharing.plane0_plane1 = p.mcache_allocation.last_slice_sharing.plane0_plane1;
            memcpy((*mcache_params.add(dc_plane_idx)).mcache_x_offsets_plane0.as_mut_ptr() as *mut c_void, p.mcache_allocation.mcache_x_offsets_plane0.as_ptr() as *const c_void, core::mem::size_of::<i32>() * (DML2_MAX_MCACHES + 1));
            memcpy((*mcache_params.add(dc_plane_idx)).mcache_x_offsets_plane1.as_mut_ptr() as *mut c_void, p.mcache_allocation.mcache_x_offsets_plane1.as_ptr() as *const c_void, core::mem::size_of::<i32>() * (DML2_MAX_MCACHES + 1));
            dc_plane_idx += 1;
        }
    }
}

unsafe fn dml21_check_mode_support(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) -> bool {
    memset(&mut (*dml_ctx).v21.display_config as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_display_cfg>());
    memset(&mut (*dml_ctx).v21.dml_to_dc_pipe_mapping as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_dml_to_dc_pipe_mapping>());
    if context.is_null() || (*context).stream_count == 0 { return true; }
    (*dml_ctx).config.svp_pstate.callbacks.remove_phantom_streams_and_planes(in_dc, context);
    (*dml_ctx).config.svp_pstate.callbacks.release_phantom_streams_and_planes(in_dc, context);
    (*dml_ctx).v21.mode_support.dml2_instance = (*dml_ctx).v21.dml_init.dml2_instance;
    dml21_map_dc_state_into_dml_display_cfg(in_dc, context, dml_ctx);
    (*dml_ctx).v21.mode_programming.dml2_instance.scratch.build_mode_programming_locals.mode_programming_params.programming = (*dml_ctx).v21.mode_programming.programming;
    dml2_check_mode_supported(&mut (*dml_ctx).v21.mode_support)
}

unsafe fn dml21_mode_check_and_programming(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) -> bool {
    let mut mcache_params: [dc_mcache_params; MAX_PLANES] = core::mem::zeroed();
    memset(&mut (*dml_ctx).v21.display_config as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_display_cfg>());
    memset(&mut (*dml_ctx).v21.dml_to_dc_pipe_mapping as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_dml_to_dc_pipe_mapping>());
    if context.is_null() { return true; }
    if (*context).stream_count == 0 { dml21_init_min_clocks_for_dc_state(dml_ctx, context); dml21_build_fams2_programming(in_dc, context, dml_ctx); return true; }
    (*dml_ctx).config.svp_pstate.callbacks.remove_phantom_streams_and_planes(in_dc, context);
    (*dml_ctx).config.svp_pstate.callbacks.release_phantom_streams_and_planes(in_dc, context);
    if !dml21_map_dc_state_into_dml_display_cfg(in_dc, context, dml_ctx) { return false; }
    if !dml2_build_mode_programming(&mut (*dml_ctx).v21.mode_programming) { return false; }
    if !(*dml_ctx).config.skip_hw_state_mapping { dml21_map_hw_resources(dml_ctx); dml2_map_dc_pipes(dml_ctx, context, core::ptr::null_mut(), &mut (*dml_ctx).v21.dml_to_dc_pipe_mapping, (*in_dc).current_state); dml21_handle_phantom_streams_planes(in_dc, context, dml_ctx); if (*(*in_dc).res_pool).funcs.program_mcache_pipe_config.is_some() { dml21_prepare_mcache_params(dml_ctx, context, mcache_params.as_mut_ptr()); (*dml_ctx).config.callbacks.allocate_mcache(context, mcache_params.as_mut_ptr()); } }
    if !(*dml_ctx).config.skip_hw_state_mapping { dml21_calculate_rq_and_dlg_params(in_dc, context, &mut (*context).res_ctx, dml_ctx, (*(*in_dc).res_pool).pipe_count); dml21_copy_clocks_to_dc_state(dml_ctx, context); dml21_extract_watermark_sets(in_dc, &mut (*context).bw_ctx.bw.dcn.watermarks, dml_ctx); dml21_build_fams2_programming(in_dc, context, dml_ctx); }
    true
}

pub unsafe fn dml21_prepare_mcache_programming(in_dc: *mut dc, context: *mut dc_state, dml_ctx: *mut dml2_context) {
    if (*context).stream_count == 0 { return; }
    let l = &mut (*dml_ctx).v21.scratch.prepare_mcache_locals;
    memset(&mut l.build_mcache_programming_params as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_build_mcache_programming_in_out>());
    l.build_mcache_programming_params.dml2_instance = (*dml_ctx).v21.dml_init.dml2_instance;
    let mut main: [*mut pipe_ctx; __DML2_WRAPPER_MAX_STREAMS_PLANES__] = [core::ptr::null_mut(); __DML2_WRAPPER_MAX_STREAMS_PLANES__];
    let mut phantom: [*mut pipe_ctx; __DML2_WRAPPER_MAX_STREAMS_PLANES__] = [core::ptr::null_mut(); __DML2_WRAPPER_MAX_STREAMS_PLANES__];
    let mut phantom_idx = (*dml_ctx).v21.mode_programming.programming.display_config.num_planes;
    for i in 0..(*dml_ctx).v21.mode_programming.programming.display_config.num_planes as usize {
        let p = &(*dml_ctx).v21.mode_programming.programming.plane_programming[i];
        let c = &mut l.build_mcache_programming_params.mcache_configurations[i];
        memset(c as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_plane_mcache_configuration_descriptor>());
        c.plane_descriptor = p.plane_descriptor; c.mcache_allocation = &mut (*context).bw_ctx.bw.dcn.mcache_allocations[i]; c.num_pipes = p.num_dpps_required as i8; l.build_mcache_programming_params.num_configurations += 1;
        if p.num_dpps_required == 0 { continue; }
        let n = dml21_find_dc_pipes_for_plane(in_dc, context, dml_ctx, main.as_mut_ptr(), phantom.as_mut_ptr(), i);
        if n <= 0 || (*main[0]).stream.is_null() || (*main[0]).plane_state.is_null() { continue; }
        for j in 0..n as usize { dml21_get_pipe_mcache_config(context, main[j], p, &mut c.pipe_configurations[j]); }
        if p.phantom_plane.valid && !phantom[0].is_null() && !(*main[0]).stream.is_null() && !(*phantom[0]).plane_state.is_null() { let pc = &mut l.build_mcache_programming_params.mcache_configurations[phantom_idx]; memset(pc as *mut _ as *mut c_void, 0, core::mem::size_of::<dml2_plane_mcache_configuration_descriptor>()); pc.plane_descriptor = p.plane_descriptor; pc.mcache_allocation = &mut (*context).bw_ctx.bw.dcn.mcache_allocations[phantom_idx]; pc.num_pipes = p.num_dpps_required as i8; l.build_mcache_programming_params.num_configurations += 1; for j in 0..n as usize { dml21_get_pipe_mcache_config(context, phantom[j], p, &mut pc.pipe_configurations[j]); } phantom_idx += 1; }
    }
    dml2_build_mcache_programming(&mut l.build_mcache_programming_params);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
