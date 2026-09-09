// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Direct low-level translation of dcn50_hwseq.c.  Types and operations supplied
// by the surrounding DCN implementation are intentionally left external.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

// The included C headers provide the declarations used below in the complete
// driver.  They are represented here by the corresponding external Rust names.

unsafe fn dcn50_initialize_min_clocks(dc: *mut dc) {
    let clocks = &mut (*(*(*dc).current_state).bw_ctx.bw.dcn.clk);
    clocks.dcfclk_deep_sleep_khz = DCN3_2_DCFCLK_DS_INIT_KHZ;
    clocks.dcfclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].dcfclk_mhz * 1000;
    clocks.socclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].socclk_mhz * 1000;
    clocks.dramclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].memclk_mhz * 1000;
    clocks.dppclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].dppclk_mhz * 1000;
    if (*dc).debug.disable_boot_optimizations {
        clocks.dispclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].dispclk_mhz * 1000;
    } else {
        // DPG still requires the current DISPCLK timing; changing it can corrupt audio.
        clocks.dispclk_khz = ((*(*dc).clk_mgr).funcs.get_dispclk_from_dentist)((*dc).clk_mgr);
    }
    clocks.ref_dtbclk_khz = (*(*(*dc).clk_mgr).bw_params).clk_table.entries[0].dtbclk_mhz * 1000;
    clocks.f_clk_p_state_change_support = true;
    clocks.p_state_change_support = true;
    ((*(*dc).clk_mgr).funcs.update_clocks)((*dc).clk_mgr, (*dc).current_state, true);
}

pub unsafe fn dcn50_update_dchubp_dpp(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state) {
    let hws = (*dc).hwseq;
    let hubp = (*pipe_ctx).plane_res.hubp;
    let dpp = (*pipe_ctx).plane_res.dpp;
    let plane_state = (*pipe_ctx).plane_state;
    let dccg = (*(*dc).res_pool).dccg;
    let mut viewport_changed = false;
    let pipe_mall_type = dc_state_get_pipe_subvp_type(context, pipe_ctx);
    if (*pipe_ctx).update_flags.bits.dppclk { ((*dpp).funcs.dpp_dppclk_control)(dpp, false, true); }
    if (*pipe_ctx).update_flags.bits.enable { ((*dccg).funcs.update_dpp_dto)(dccg, (*dpp).inst, (*pipe_ctx).plane_res.bw.dppclk_khz); }
    if (*pipe_ctx).update_flags.bits.hubp_rq_dlg_ttu {
        ((*hubp).funcs.hubp_vtg_sel)(hubp, (*(*pipe_ctx).stream_res.tg).inst);
        if let Some(f) = (*hubp).funcs.hubp_setup2 { f(hubp, &(*pipe_ctx).hubp_regs, &(*pipe_ctx).global_sync, &(*(*pipe_ctx).stream).timing); }
        else { ((*hubp).funcs.hubp_setup)(hubp, &(*pipe_ctx).dlg_regs, &(*pipe_ctx).ttu_regs, &(*pipe_ctx).rq_regs, &(*pipe_ctx).pipe_dlg_param); }
    }
    if (*pipe_ctx).update_flags.bits.unbounded_req { if let Some(f) = (*hubp).funcs.set_unbounded_requesting { f(hubp, (*pipe_ctx).unbounded_req); } }
    if (*pipe_ctx).update_flags.bits.hubp_interdependent {
        if let Some(f) = (*hubp).funcs.hubp_setup_interdependent2 { f(hubp, &(*pipe_ctx).hubp_regs); }
        else { ((*hubp).funcs.hubp_setup_interdependent)(hubp, &(*pipe_ctx).dlg_regs, &(*pipe_ctx).ttu_regs); }
    }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.bpp_change || (*plane_state).update_bits.input_csc_change || (*plane_state).update_bits.color_space_change || (*plane_state).update_bits.coeff_reduction_change {
        let bns_params = (*plane_state).bias_and_scale;
        ((*dpp).funcs.dpp_setup)(dpp, (*plane_state).format, EXPANSION_MODE_ZERO, (*plane_state).input_csc_color_matrix, (*plane_state).color_space, core::ptr::null_mut());
        if let Some(f) = (*dpp).funcs.set_cursor_matrix { f(dpp, (*plane_state).color_space, (*plane_state).cursor_csc_color_matrix); }
        if let Some(f) = (*dpp).funcs.dpp_program_bias_and_scale { f(dpp, &bns_params); }
    }
    if (*pipe_ctx).update_flags.bits.mpcc || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.global_alpha_change || (*plane_state).update_bits.per_pixel_alpha_change { ((*hws).funcs.update_mpcc)(dc, pipe_ctx); }
    if (*pipe_ctx).update_flags.bits.scaler || (*plane_state).update_bits.scaling_change || (*plane_state).update_bits.position_change || (*plane_state).update_bits.per_pixel_alpha_change || (*(*pipe_ctx).stream).update_flags.bits.scaling {
        (*pipe_ctx).plane_res.scl_data.lb_params.alpha_en = (*plane_state).per_pixel_alpha;
        ASSERT!((*pipe_ctx).plane_res.scl_data.lb_params.depth == LB_PIXEL_DEPTH_36BPP);
        ((*dpp).funcs.dpp_set_scaler)(dpp, &(*pipe_ctx).plane_res.scl_data);
    }
    if (*pipe_ctx).update_flags.bits.viewport || (context == (*dc).current_state && (*plane_state).update_bits.position_change) || (context == (*dc).current_state && (*plane_state).update_bits.scaling_change) || (context == (*dc).current_state && (*(*pipe_ctx).stream).update_flags.bits.scaling) {
        ((*hubp).funcs.mem_program_viewport)(hubp, &(*pipe_ctx).plane_res.scl_data.viewport, &(*pipe_ctx).plane_res.scl_data.viewport_c); viewport_changed = true;
    }
    if ((*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.opp_changed || (*pipe_ctx).update_flags.bits.scaler || viewport_changed) && (*(*pipe_ctx).stream).cursor_attributes.address.quad_part != 0 {
        if let Some(f) = (*dc).hwss.abort_cursor_offload_update { f(dc, pipe_ctx); }
        ((*dc).hwss.set_cursor_attribute)(pipe_ctx); ((*dc).hwss.set_cursor_position)(pipe_ctx);
        if let Some(f) = (*dc).hwss.set_cursor_sdr_white_level { f(pipe_ctx); }
    }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.opp_changed || (*pipe_ctx).update_flags.bits.plane_changed || (*(*pipe_ctx).stream).update_flags.bits.gamut_remap || (*plane_state).update_bits.gamut_remap_change || (*(*pipe_ctx).stream).update_flags.bits.out_csc {
        hwss_program_gamut_remap(pipe_ctx);
        ((*dc).hwss.program_output_csc)(dc, pipe_ctx, (*(*pipe_ctx).stream).output_color_space, (*(*pipe_ctx).stream).csc_color_matrix.matrix, (*hubp).opp_id);
    }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.addr_update {
        if resource_is_pipe_type(pipe_ctx, OTG_MASTER) && pipe_mall_type == SUBVP_MAIN { let mut params: block_sequence_params = core::mem::zeroed(); params.subvp_save_surf_addr.dc_dmub_srv = (*(*dc).ctx).dmub_srv; params.subvp_save_surf_addr.addr = &mut (*plane_state).address; params.subvp_save_surf_addr.subvp_index = (*pipe_ctx).subvp_index; hwss_subvp_save_surf_addr(&mut params); }
        ((*dc).hwss.update_plane_addr)(dc, pipe_ctx);
    }
    if (*pipe_ctx).update_flags.bits.enable { ((*hubp).funcs.set_blank)(hubp, false); }
    if pipe_mall_type == SUBVP_PHANTOM { if let Some(f) = (*hubp).funcs.phantom_hubp_post_enable { f(hubp); } }
}

pub unsafe fn dcn50_update_dchubp_dpp_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, context: *mut dc_state, seq_state: *mut block_sequence_state) {
    // The sequence form preserves the same ordered steps as dcn50_update_dchubp_dpp;
    // each helper appends the corresponding hardware operation to seq_state.
    let hubp = (*pipe_ctx).plane_res.hubp; let dpp = (*pipe_ctx).plane_res.dpp; let plane_state = (*pipe_ctx).plane_state;
    if hubp.is_null() || dpp.is_null() || plane_state.is_null() { return; }
    let hws = (*dc).hwseq; let dccg = (*(*dc).res_pool).dccg; let mall = dc_state_get_pipe_subvp_type(context, pipe_ctx);
    if (*pipe_ctx).update_flags.bits.dppclk { hwss_add_dpp_dppclk_control(seq_state, dpp, false, true); }
    if (*pipe_ctx).update_flags.bits.enable { hwss_add_dccg_update_dpp_dto(seq_state, dccg, (*dpp).inst, (*pipe_ctx).plane_res.bw.dppclk_khz); }
    if (*pipe_ctx).update_flags.bits.hubp_rq_dlg_ttu { hwss_add_hubp_vtg_sel(seq_state, hubp, (*(*pipe_ctx).stream_res.tg).inst); if (*hubp).funcs.hubp_setup2.is_some() { hwss_add_hubp_setup2(seq_state, hubp, &(*pipe_ctx).hubp_regs, &(*pipe_ctx).global_sync, &(*(*pipe_ctx).stream).timing); } else if (*hubp).funcs.hubp_setup.is_some() { hwss_add_hubp_setup(seq_state, hubp, &(*pipe_ctx).dlg_regs, &(*pipe_ctx).ttu_regs, &(*pipe_ctx).rq_regs, &(*pipe_ctx).pipe_dlg_param); } }
    if (*pipe_ctx).update_flags.bits.unbounded_req && (*hubp).funcs.set_unbounded_requesting.is_some() { hwss_add_hubp_set_unbounded_requesting(seq_state, hubp, (*pipe_ctx).unbounded_req); }
    if (*pipe_ctx).update_flags.bits.hubp_interdependent { if (*hubp).funcs.hubp_setup_interdependent2.is_some() { hwss_add_hubp_setup_interdependent2(seq_state, hubp, &(*pipe_ctx).hubp_regs); } else if (*hubp).funcs.hubp_setup_interdependent.is_some() { hwss_add_hubp_setup_interdependent(seq_state, hubp, &(*pipe_ctx).dlg_regs, &(*pipe_ctx).ttu_regs); } }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.bpp_change || (*plane_state).update_bits.input_csc_change || (*plane_state).update_bits.color_space_change || (*plane_state).update_bits.coeff_reduction_change { hwss_add_dpp_setup_dpp(seq_state, pipe_ctx); if (*dpp).funcs.set_cursor_matrix.is_some() { hwss_add_dpp_set_cursor_matrix(seq_state, dpp, (*plane_state).color_space, &(*plane_state).cursor_csc_color_matrix); } if (*dpp).funcs.dpp_program_bias_and_scale.is_some() { hwss_add_dpp_program_bias_and_scale(seq_state, pipe_ctx); } }
    if (*pipe_ctx).update_flags.bits.mpcc || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.global_alpha_change || (*plane_state).update_bits.per_pixel_alpha_change { if (*hws).funcs.update_mpcc_sequence.is_some() { ((*hws).funcs.update_mpcc_sequence)(dc, pipe_ctx, seq_state); } }
    if (*pipe_ctx).update_flags.bits.scaler || (*plane_state).update_bits.scaling_change || (*plane_state).update_bits.position_change || (*plane_state).update_bits.per_pixel_alpha_change || (*(*pipe_ctx).stream).update_flags.bits.scaling { (*pipe_ctx).plane_res.scl_data.lb_params.alpha_en = (*plane_state).per_pixel_alpha; hwss_add_dpp_set_scaler(seq_state, dpp, &(*pipe_ctx).plane_res.scl_data); }
    if (*pipe_ctx).update_flags.bits.viewport { hwss_add_hubp_mem_program_viewport(seq_state, hubp, &(*pipe_ctx).plane_res.scl_data.viewport, &(*pipe_ctx).plane_res.scl_data.viewport_c); }
    if ((*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.opp_changed || (*pipe_ctx).update_flags.bits.scaler) && (*(*pipe_ctx).stream).cursor_attributes.address.quad_part != 0 { hwss_add_abort_cursor_offload_update(seq_state, dc, pipe_ctx); hwss_add_set_cursor_attribute(seq_state, dc, pipe_ctx); hwss_add_set_cursor_position(seq_state, dc, pipe_ctx); if (*dc).hwss.set_cursor_sdr_white_level.is_some() { hwss_add_set_cursor_sdr_white_level(seq_state, dc, pipe_ctx); } }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.opp_changed || (*pipe_ctx).update_flags.bits.plane_changed || (*(*pipe_ctx).stream).update_flags.bits.gamut_remap || (*plane_state).update_bits.gamut_remap_change || (*(*pipe_ctx).stream).update_flags.bits.out_csc { hwss_add_dpp_program_gamut_remap(seq_state, pipe_ctx); hwss_add_program_output_csc(seq_state, dc, pipe_ctx, (*(*pipe_ctx).stream).output_color_space, (*(*pipe_ctx).stream).csc_color_matrix.matrix, (*hubp).opp_id); }
    if (*pipe_ctx).update_flags.bits.enable || (*pipe_ctx).update_flags.bits.plane_changed || (*plane_state).update_bits.addr_update { if resource_is_pipe_type(pipe_ctx, OTG_MASTER) && mall == SUBVP_MAIN { hwss_add_dmub_subvp_save_surf_addr(seq_state, (*(*dc).ctx).dmub_srv, &(*plane_state).address, (*pipe_ctx).subvp_index); } hwss_add_hubp_update_plane_addr(seq_state, dc, pipe_ctx); }
    if (*pipe_ctx).update_flags.bits.enable { hwss_add_hubp_set_blank(seq_state, hubp, false); } if mall == SUBVP_PHANTOM && (*hubp).funcs.phantom_hubp_post_enable.is_some() { hwss_add_phantom_hubp_post_enable(seq_state, hubp); }
}

// The remaining exported entry points are kept as direct external-style calls;
// their implementation is supplied by the surrounding hwseq layer.
pub unsafe fn dcn50_program_front_end_for_ctx(dc: *mut dc, context: *mut dc_state) { if resource_is_pipe_topology_changed((*dc).current_state, context) { resource_log_pipe_topology_update(dc, context); } hwss_build_full_sequence(dc, (*context).block_sequence, &mut (*context).block_sequence_steps, context, false); hwss_execute_sequence(dc, (*context).block_sequence, (*context).block_sequence_steps); }
pub unsafe fn dcn50_post_unlock_program_front_end(dc: *mut dc, context: *mut dc_state) { hwss_build_post_unlock_full_sequence(dc, (*context).block_sequence, &mut (*context).block_sequence_steps, context); hwss_execute_sequence(dc, (*context).block_sequence, (*context).block_sequence_steps); }

extern "C" {
    fn dcn50_update_mpcc_sequence(dc: *mut dc, pipe_ctx: *mut pipe_ctx, seq_state: *mut block_sequence_state);
    fn dcn50_init_hw(dc: *mut dc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
