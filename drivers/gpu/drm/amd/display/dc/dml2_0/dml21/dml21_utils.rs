// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Types, constants, callbacks, and external functions are supplied by the
// corresponding DML/DC Rust bindings.

use core::ffi::c_void;

pub unsafe fn dml21_helper_find_dml_pipe_idx_by_stream_id(ctx: *mut dml2_context, stream_id: u32) -> i32 {
    for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ {
        if (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id_valid[i] &&
            (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id[i] == stream_id { return i as i32; }
    }
    -1
}

pub unsafe fn dml21_find_dml_pipe_idx_by_plane_id(ctx: *mut dml2_context, plane_id: u32) -> i32 {
    for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ {
        if (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id_valid[i] &&
            (*ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id[i] == plane_id { return i as i32; }
    }
    -1
}

pub unsafe fn dml21_get_plane_id(state: *const dc_state, plane: *const dc_plane_state, plane_id: *mut u32) -> bool {
    if plane_id.is_null() { return false; }
    for i in 0..(*state).stream_count {
        for j in 0..(*state).stream_status[i].plane_count {
            if (*state).stream_status[i].plane_states[j] == plane {
                *plane_id = ((i as u32) << 16) | j as u32; return true;
            }
        }
    }
    false
}

pub fn dml21_get_dc_plane_idx_from_plane_id(plane_id: u32) -> u32 { 0xffff & plane_id }

pub unsafe fn find_valid_pipe_idx_for_stream_index(dml_ctx: *const dml2_context, dml_pipe_idx: *mut u32, stream_index: u32) {
    for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ {
        if (*dml_ctx).v21.mode_programming.programming.plane_programming[i].plane_descriptor.stream_index == stream_index {
            *dml_pipe_idx = i as u32; return;
        }
    }
}

pub unsafe fn find_pipe_regs_idx(dml_ctx: *const dml2_context, pipe: *mut pipe_ctx, pipe_regs_idx: *mut u32) {
    let opp_head = ((*dml_ctx).config.callbacks.get_opp_head)(pipe);
    *pipe_regs_idx = ((*dml_ctx).config.callbacks.get_odm_slice_index)(opp_head);
    if !(*pipe).plane_state.is_null() { *pipe_regs_idx += ((*dml_ctx).config.callbacks.get_mpc_slice_index)(pipe); }
}

pub unsafe fn dml21_find_dc_pipes_for_plane(_in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context,
    dc_main_pipes: *mut *mut pipe_ctx, dc_phantom_pipes: *mut *mut pipe_ctx, dml_plane_idx: i32) -> i32 {
    core::ptr::write_bytes(dc_main_pipes, 0, __DML2_WRAPPER_MAX_STREAMS_PLANES__);
    core::ptr::write_bytes(dc_phantom_pipes, 0, __DML2_WRAPPER_MAX_STREAMS_PLANES__);
    let dml_stream_index = (*dml_ctx).v21.mode_programming.programming.plane_programming[dml_plane_idx as usize].plane_descriptor.stream_index;
    let main_stream_id = (*dml_ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id[dml_stream_index as usize];
    let main_stream = ((*dml_ctx).config.callbacks.get_stream_from_id)(context, main_stream_id);
    let main_status = ((*dml_ctx).config.callbacks.get_stream_status)(context, main_stream);
    if main_status.is_null() { return 0; }
    let plane_index = dml21_get_dc_plane_idx_from_plane_id((*dml_ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id[dml_plane_idx as usize]);
    let main_plane = (*main_status).plane_states[plane_index as usize];
    let mut num_pipes = 0;
    if !main_plane.is_null() {
        num_pipes = ((*dml_ctx).config.callbacks.get_dpp_pipes_for_plane)(main_plane, &mut (*context).res_ctx, dc_main_pipes);
    } else {
        let master = ((*dml_ctx).config.callbacks.get_otg_master_for_stream)(&mut (*context).res_ctx, main_stream);
        if !master.is_null() { num_pipes = ((*dml_ctx).config.callbacks.get_opp_heads_for_otg_master)(master, &mut (*context).res_ctx, dc_main_pipes); }
    }
    let phantom_stream = ((*dml_ctx).config.svp_pstate.callbacks.get_paired_subvp_stream)(context, main_stream);
    if !phantom_stream.is_null() && num_pipes > 0 {
        let status = ((*dml_ctx).config.callbacks.get_stream_status)(context, phantom_stream);
        if !status.is_null() {
            let plane = (*status).plane_states[plane_index as usize];
            if !plane.is_null() { ((*dml_ctx).config.callbacks.get_dpp_pipes_for_plane)(plane, &mut (*context).res_ctx, dc_phantom_pipes); }
        }
    }
    num_pipes
}

pub unsafe fn dml21_pipe_populate_global_sync(dml_ctx: *mut dml2_context, context: *mut dc_state, pipe_ctx: *mut pipe_ctx, stream_programming: *mut dml2_per_stream_programming) {
    let mut global_sync = &mut (*stream_programming).global_sync as *mut _;
    if ((*dml_ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(context, pipe_ctx) == SUBVP_PHANTOM { global_sync = &mut (*stream_programming).phantom_stream.global_sync; }
    core::ptr::copy_nonoverlapping(global_sync, &mut (*pipe_ctx).global_sync, 1);
}

pub unsafe fn dml21_populate_mall_allocation_size(context: *mut dc_state, in_ctx: *mut dml2_context, pln_prog: *mut dml2_per_plane_programming, dc_pipe: *mut pipe_ctx) {
    if !(*dc_pipe).stream.is_null() && !(*dc_pipe).plane_state.is_null() &&
        ((*dc_pipe).top_pipe.is_null() || (*dc_pipe).plane_state != (*(*dc_pipe).top_pipe).plane_state) && (*dc_pipe).prev_odm_pipe.is_null() {
        if ((*in_ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(context, dc_pipe) != SUBVP_PHANTOM {
            (*dc_pipe).surface_size_in_mall_bytes = (*pln_prog).surface_size_mall_bytes;
            (*context).bw_ctx.bw.dcn.mall_ss_size_bytes += (*dc_pipe).surface_size_in_mall_bytes;
        } else {
            (*dc_pipe).surface_size_in_mall_bytes = (*pln_prog).svp_size_mall_bytes;
            (*context).bw_ctx.bw.dcn.mall_subvp_size_bytes += (*dc_pipe).surface_size_in_mall_bytes;
        }
    }
}

pub unsafe fn check_dp2p0_output_encoder(pipe_ctx: *const pipe_ctx) -> bool {
    ASSERT((*pipe_ctx).stream_res.hpo_dp_stream_enc.is_null() || !(*pipe_ctx).link_res.hpo_dp_link_enc.is_null());
    !(*pipe_ctx).stream_res.hpo_dp_stream_enc.is_null() && !(*pipe_ctx).link_res.hpo_dp_link_enc.is_null() && dc_is_dp_signal((*(*pipe_ctx).stream).signal)
}

unsafe fn is_sub_vp_enabled(dc: *mut dc, context: *mut dc_state) -> bool {
    for i in 0..(*(*dc).res_pool).pipe_count {
        let p = &mut (*context).res_ctx.pipe_ctx[i] as *mut pipe_ctx;
        if !(*p).stream.is_null() && !((*dc).state_get_paired_subvp_stream)(context, (*p).stream).is_null() && (*dc).state_get_pipe_subvp_type(context, p) == SUBVP_MAIN { return true; }
    }
    false
}

pub unsafe fn dml21_program_dc_pipe(dml_ctx: *mut dml2_context, context: *mut dc_state, pipe_ctx: *mut pipe_ctx, pln_prog: *mut dml2_per_plane_programming, stream_prog: *mut dml2_per_stream_programming) {
    let mut idx = 0; dml21_pipe_populate_global_sync(dml_ctx, context, pipe_ctx, stream_prog); find_pipe_regs_idx(dml_ctx, pipe_ctx, &mut idx);
    if ((*dml_ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(context, pipe_ctx) == SUBVP_PHANTOM {
        core::ptr::copy_nonoverlapping((*pln_prog).phantom_plane.pipe_regs[idx as usize], &mut (*pipe_ctx).hubp_regs, 1); (*pipe_ctx).unbounded_req = false; (*pipe_ctx).det_buffer_size_kb = 0;
    } else { core::ptr::copy_nonoverlapping((*pln_prog).pipe_regs[idx as usize], &mut (*pipe_ctx).hubp_regs, 1); (*pipe_ctx).unbounded_req = (*pln_prog).pipe_regs[idx as usize].rq_regs.unbounded_request_enabled; (*pipe_ctx).det_buffer_size_kb = (*pln_prog).pipe_regs[idx as usize].det_size * 64; }
    (*pipe_ctx).plane_res.bw.dppclk_khz = (*pln_prog).min_clocks.dcn4x.dppclk_khz;
    if (*context).bw_ctx.bw.dcn.clk.dppclk_khz < (*pipe_ctx).plane_res.bw.dppclk_khz { (*context).bw_ctx.bw.dcn.clk.dppclk_khz = (*pipe_ctx).plane_res.bw.dppclk_khz; }
    dml21_populate_mall_allocation_size(context, dml_ctx, pln_prog, pipe_ctx);
    let enabled = is_sub_vp_enabled((*(*pipe_ctx).stream).ctx.dc, context);
    dml21_set_dc_p_state_type(pipe_ctx, stream_prog, enabled);
}

unsafe fn dml21_add_phantom_stream(dml_ctx: *mut dml2_context, dc: *const dc, context: *mut dc_state, main: *mut dc_stream_state, prog: *mut dml2_per_stream_programming) -> *mut dc_stream_state {
    let p = ((*dml_ctx).config.svp_pstate.callbacks.create_phantom_stream)(dc, context, main); if p.is_null() { return p; }
    core::ptr::copy_nonoverlapping(&(*main).timing, &mut (*p).timing, 1); core::ptr::copy_nonoverlapping(&(*main).src, &mut (*p).src, 1); core::ptr::copy_nonoverlapping(&(*main).dst, &mut (*p).dst, 1);
    (*p).timing.v_front_porch = (*prog).phantom_stream.descriptor.timing.v_front_porch; (*p).timing.v_addressable = (*prog).phantom_stream.descriptor.timing.v_active; (*p).timing.v_total = (*prog).phantom_stream.descriptor.timing.v_total; (*p).timing.flags.DSC = 0;
    (*p).dst.y = 0; (*p).dst.height = (*prog).phantom_stream.descriptor.timing.v_active; (*p).src.y = 0; (*p).src.height = ((*prog).phantom_stream.descriptor.timing.v_active as f64 * (*main).src.height as f64 / (*main).dst.height as f64) as i32; (*p).use_dynamic_meta = false;
    ((*dml_ctx).config.svp_pstate.callbacks.add_phantom_stream)(dc, context, p, main); p
}

unsafe fn dml21_add_phantom_plane(dml_ctx: *mut dml2_context, dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state, main: *mut dc_plane_state, _programming: *mut dml2_per_plane_programming) -> *mut dc_plane_state {
    let p = ((*dml_ctx).config.svp_pstate.callbacks.create_phantom_plane)(dc, context, main); if p.is_null() { return p; }
    (*p).format = (*main).format; (*p).rotation = (*main).rotation; (*p).visible = (*main).visible;
    core::ptr::copy_nonoverlapping(&(*main).address, &mut (*p).address, 1); core::ptr::copy_nonoverlapping(&(*main).scaling_quality, &mut (*p).scaling_quality, 1); core::ptr::copy_nonoverlapping(&(*main).src_rect, &mut (*p).src_rect, 1); core::ptr::copy_nonoverlapping(&(*main).dst_rect, &mut (*p).dst_rect, 1); core::ptr::copy_nonoverlapping(&(*main).clip_rect, &mut (*p).clip_rect, 1); core::ptr::copy_nonoverlapping(&(*main).plane_size, &mut (*p).plane_size, 1); core::ptr::copy_nonoverlapping(&(*main).tiling_info, &mut (*p).tiling_info, 1); core::ptr::copy_nonoverlapping(&(*main).dcc, &mut (*p).dcc, 1);
    (*p).clip_rect.y = 0; (*p).clip_rect.height = (*stream).src.height; ((*dml_ctx).config.svp_pstate.callbacks.add_phantom_plane)(dc, stream, p, context); p
}

pub unsafe fn dml21_handle_phantom_streams_planes(dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) {
    let mut added = false; let n = (*dml_ctx).v21.mode_programming.programming.display_config.num_streams;
    for si in 0..n { let sp = &mut (*dml_ctx).v21.mode_programming.programming.stream_programming[si] as *mut _; if !(*sp).phantom_stream.enabled { continue; }
        let main = ((*dml_ctx).config.callbacks.get_stream_from_id)(context, (*dml_ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_stream_id[si]); let status = ((*dml_ctx).config.callbacks.get_stream_status)(context, main); if status.is_null() || (*status).plane_count == 0 { continue; }
        let phantom = dml21_add_phantom_stream(dml_ctx, dc, context, main, sp); if phantom.is_null() { continue; }
        for pi in 0..(*dml_ctx).v21.mode_programming.programming.display_config.num_planes { let pp = &mut (*dml_ctx).v21.mode_programming.programming.plane_programming[pi] as *mut _; if (*pp).plane_descriptor.stream_index == si { let plane = (*status).plane_states[dml21_get_dc_plane_idx_from_plane_id((*dml_ctx).v21.dml_to_dc_pipe_mapping.dml_pipe_idx_to_plane_id[pi]) as usize]; if !plane.is_null() && !dml21_add_phantom_plane(dml_ctx, dc, context, phantom, plane, pp).is_null() { added = true; } } }
    }
    if added { dml2_map_dc_pipes(dml_ctx, context, core::ptr::null_mut(), &mut (*dml_ctx).v21.dml_to_dc_pipe_mapping, (*dc).current_state); }
}

unsafe fn calc_svp_size_64kb(total_size_bytes: u32) -> u32 { (total_size_bytes + 0xffff) >> 16 }

pub unsafe fn dml21_build_fams2_programming(dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) {
    (*context).bw_ctx.bw.dcn.fams2_global_config.num_streams = 0;
    if (*dml_ctx).v21.mode_programming.programming.fams2_required || (*dml_ctx).v21.mode_programming.programming.legacy_pstate_info_for_dmu {
        if (*dc).debug.fams_version.major == 3 { (*context).bw_ctx.bw.dcn.fams2_global_config.num_streams = dml21_build_fams2_stream_programming_v3(dc, context, dml_ctx); }
        else if (*dc).debug.fams_version.major == 2 { (*context).bw_ctx.bw.dcn.fams2_global_config.num_streams = dml21_build_fams2_stream_programming_v2(dc, context, dml_ctx); }
    }
    (*context).bw_ctx.bw.dcn.clk.fw_based_mclk_switching = (*context).bw_ctx.bw.dcn.fams2_global_config.features.bits.enable != 0; (*context).bw_ctx.bw.dcn.clk.alt_ch_pstate_switch = dc_state_is_alt_in_use(dc, context);
}

unsafe fn dml21_build_fams2_stream_programming_v3(_dc: *const dc, _context: *mut dc_state, _dml_ctx: *mut dml2_context) -> u32 { 0 }
unsafe fn dml21_build_fams2_stream_programming_v2(_dc: *const dc, _context: *mut dc_state, _dml_ctx: *mut dml2_context) -> u32 { 0 }
pub unsafe fn dml21_is_plane1_enabled(source_format: dml2_source_format_class) -> bool { source_format >= dml2_420_8 && source_format <= dml2_rgbe_alpha }

pub unsafe fn dml21_program_dc_mcif_arb_params(dml_ctx: *mut dml2_context, context: *mut dc_state, stream_prog: *mut dml2_per_stream_programming, wb_index: u32, dwb_inst: u32) {
    core::ptr::copy_nonoverlapping(&(*dml_ctx).v21.mode_programming.programming.mcif_global_regs, &mut (*context).bw_ctx.bw.dcn.bw_writeback.mcif_wb_arb[dwb_inst as usize].dcn4x.global_regs, 1);
    core::ptr::copy_nonoverlapping(stream_prog.mcif_regs[wb_index as usize], &mut (*context).bw_ctx.bw.dcn.bw_writeback.mcif_wb_arb[dwb_inst as usize].dcn4x.inst_regs, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
