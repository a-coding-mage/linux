// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translated repository.

unsafe fn hubp60_program_deadline(
    hubp: *mut hubp,
    dlg_attr: *mut dml2_display_dlg_regs,
    ttu_attr: *mut dml2_display_ttu_regs,
) {
    let hubp2 = TO_DCN20_HUBP(hubp);

    REG_SET_2!(hubp2, BLANK_OFFSET_0, 0, REFCYC_H_BLANK_END, (*dlg_attr).refcyc_h_blank_end, DLG_V_BLANK_END, (*dlg_attr).dlg_vblank_end);
    REG_SET!(hubp2, BLANK_OFFSET_1, 0, MIN_DST_Y_NEXT_START, (*dlg_attr).min_dst_y_next_start);
    REG_SET!(hubp2, DST_DIMENSIONS, 0, REFCYC_PER_HTOTAL, (*dlg_attr).refcyc_per_htotal);
    REG_SET_2!(hubp2, DST_AFTER_SCALER, 0, REFCYC_X_AFTER_SCALER, (*dlg_attr).refcyc_x_after_scaler, DST_Y_AFTER_SCALER, (*dlg_attr).dst_y_after_scaler);
    REG_SET!(hubp2, REF_FREQ_TO_PIX_FREQ, 0, REF_FREQ_TO_PIX_FREQ, (*dlg_attr).ref_freq_to_pix_freq);

    REG_SET!(hubp2, VBLANK_PARAMETERS_1, 0, REFCYC_PER_PTE_GROUP_VBLANK_L, (*dlg_attr).refcyc_per_pte_group_vblank_l);
    if REG!(hubp2, NOM_PARAMETERS_0) != 0 { REG_SET!(hubp2, NOM_PARAMETERS_0, 0, DST_Y_PER_PTE_ROW_NOM_L, (*dlg_attr).dst_y_per_pte_row_nom_l); }
    if REG!(hubp2, NOM_PARAMETERS_1) != 0 { REG_SET!(hubp2, NOM_PARAMETERS_1, 0, REFCYC_PER_PTE_GROUP_NOM_L, (*dlg_attr).refcyc_per_pte_group_nom_l); }
    REG_SET!(hubp2, NOM_PARAMETERS_4, 0, DST_Y_PER_META_ROW_NOM_L, (*dlg_attr).dst_y_per_meta_row_nom_l);
    REG_SET!(hubp2, NOM_PARAMETERS_5, 0, REFCYC_PER_META_CHUNK_NOM_L, (*dlg_attr).refcyc_per_meta_chunk_nom_l);
    REG_SET_2!(hubp2, PER_LINE_DELIVERY, 0, REFCYC_PER_LINE_DELIVERY_L, (*dlg_attr).refcyc_per_line_delivery_l, REFCYC_PER_LINE_DELIVERY_C, (*dlg_attr).refcyc_per_line_delivery_c);
    REG_SET!(hubp2, VBLANK_PARAMETERS_2, 0, REFCYC_PER_PTE_GROUP_VBLANK_C, (*dlg_attr).refcyc_per_pte_group_vblank_c);
    if REG!(hubp2, NOM_PARAMETERS_2) != 0 { REG_SET!(hubp2, NOM_PARAMETERS_2, 0, DST_Y_PER_PTE_ROW_NOM_C, (*dlg_attr).dst_y_per_pte_row_nom_c); }
    if REG!(hubp2, NOM_PARAMETERS_3) != 0 { REG_SET!(hubp2, NOM_PARAMETERS_3, 0, REFCYC_PER_PTE_GROUP_NOM_C, (*dlg_attr).refcyc_per_pte_group_nom_c); }
    REG_SET!(hubp2, NOM_PARAMETERS_6, 0, DST_Y_PER_META_ROW_NOM_C, (*dlg_attr).dst_y_per_meta_row_nom_c);
    REG_SET!(hubp2, NOM_PARAMETERS_7, 0, REFCYC_PER_META_CHUNK_NOM_C, (*dlg_attr).refcyc_per_meta_chunk_nom_c);

    // TO DO: Set URGENT_FORCE_x fields with values from DML if/when available
    REG_SET_3!(hubp2, DCN_SURF0_TTU_CNTL0, 0, REFCYC_PER_REQ_DELIVERY, (*ttu_attr).refcyc_per_req_delivery_l, URGENT_FORCE_VALUE, 0, URGENT_FORCE_EN, 0);
    REG_SET_3!(hubp2, DCN_SURF1_TTU_CNTL0, 0, REFCYC_PER_REQ_DELIVERY, (*ttu_attr).refcyc_per_req_delivery_c, URGENT_FORCE_VALUE, 0, URGENT_FORCE_EN, 0);
    REG_SET_3!(hubp2, DCN_CUR0_TTU_CNTL0, 0, REFCYC_PER_REQ_DELIVERY, (*ttu_attr).refcyc_per_req_delivery_cur0, URGENT_FORCE_VALUE, 0, URGENT_FORCE_EN, 0);
    REG_SET!(hubp2, FLIP_PARAMETERS_1, 0, REFCYC_PER_PTE_GROUP_FLIP_L, (*dlg_attr).refcyc_per_pte_group_flip_l);
    REG_SET!(hubp2, HUBP_3DLUT_DLG_PARAM, 0, REFCYC_PER_3DLUT_GROUP, (*dlg_attr).refcyc_per_tdlut_group);
    REG_UPDATE!(hubp2, DCN_DMDATA_VM_CNTL, REFCYC_PER_VM_DMDATA, (*dlg_attr).refcyc_per_vm_dmdata);
}

pub unsafe fn hubp60_setup(hubp: *mut hubp, pipe_regs: *mut dml2_dchub_per_pipe_register_set, pipe_global_sync: *mut dml2_global_sync_programming, timing: *mut dc_crtc_timing) {
    hubp401_vready_at_or_After_vsync(hubp, pipe_global_sync, timing);
    hubp401_program_requestor(hubp, &mut (*pipe_regs).rq_regs);
    hubp60_program_deadline(hubp, &mut (*pipe_regs).dlg_regs, &mut (*pipe_regs).ttu_regs);
}

pub unsafe fn hubp60_setup_interdependent(hubp: *mut hubp, pipe_regs: *mut dml2_dchub_per_pipe_register_set) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_SET_3!(hubp2, PREFETCH_SETTINGS, 0, DST_Y_PREFETCH, (*pipe_regs).dlg_regs.dst_y_prefetch, VRATIO_PREFETCH, (*pipe_regs).dlg_regs.vratio_prefetch, FORCE_DISP_PREF_TO_VBLANK, (*pipe_regs).dlg_regs.force_prefetch_to_vblank);
    REG_SET!(hubp2, PREFETCH_SETTINGS_C, 0, VRATIO_PREFETCH_C, (*pipe_regs).dlg_regs.vratio_prefetch_c);
    REG_SET_2!(hubp2, VBLANK_PARAMETERS_0, 0, DST_Y_PER_VM_VBLANK, (*pipe_regs).dlg_regs.dst_y_per_vm_vblank, DST_Y_PER_ROW_VBLANK, (*pipe_regs).dlg_regs.dst_y_per_row_vblank);
    REG_SET_2!(hubp2, FLIP_PARAMETERS_0, 0, DST_Y_PER_VM_FLIP, (*pipe_regs).dlg_regs.dst_y_per_vm_flip, DST_Y_PER_ROW_FLIP, (*pipe_regs).dlg_regs.dst_y_per_row_flip);
    REG_SET!(hubp2, VBLANK_PARAMETERS_3, 0, REFCYC_PER_META_CHUNK_VBLANK_L, (*pipe_regs).dlg_regs.refcyc_per_meta_chunk_vblank_l);
    REG_SET!(hubp2, VBLANK_PARAMETERS_4, 0, REFCYC_PER_META_CHUNK_VBLANK_C, (*pipe_regs).dlg_regs.refcyc_per_meta_chunk_vblank_c);
    REG_SET!(hubp2, FLIP_PARAMETERS_2, 0, REFCYC_PER_META_CHUNK_FLIP_L, (*pipe_regs).dlg_regs.refcyc_per_meta_chunk_flip_l);
    REG_SET_2!(hubp2, PER_LINE_DELIVERY_PRE, 0, REFCYC_PER_LINE_DELIVERY_PRE_L, (*pipe_regs).dlg_regs.refcyc_per_line_delivery_pre_l, REFCYC_PER_LINE_DELIVERY_PRE_C, (*pipe_regs).dlg_regs.refcyc_per_line_delivery_pre_c);
    REG_SET!(hubp2, DCN_SURF0_TTU_CNTL1, 0, REFCYC_PER_REQ_DELIVERY_PRE, (*pipe_regs).ttu_regs.refcyc_per_req_delivery_pre_l);
    REG_SET!(hubp2, DCN_SURF1_TTU_CNTL1, 0, REFCYC_PER_REQ_DELIVERY_PRE, (*pipe_regs).ttu_regs.refcyc_per_req_delivery_pre_c);
    REG_SET!(hubp2, DCN_CUR0_TTU_CNTL1, 0, REFCYC_PER_REQ_DELIVERY_PRE, (*pipe_regs).ttu_regs.refcyc_per_req_delivery_pre_cur0);
    REG_SET!(hubp2, DCN_GLOBAL_TTU_CNTL, 0, MIN_TTU_VBLANK, (*pipe_regs).ttu_regs.min_ttu_vblank);
    REG_SET!(hubp2, DST_Y_DELTA_DRQ_LIMIT, 0, DST_Y_DELTA_DRQ_LIMIT, (*pipe_regs).dlg_regs.dst_y_delta_drq_limit);
    REG_SET!(hubp2, DST_Y_ALT_CH_DRQ_LIMIT, 0, DST_Y_ALT_CH_DRQ_LIMIT, (*pipe_regs).dlg_regs.dst_y_svp_drq_limit);
}

pub unsafe fn hubp60_cursor_set_attributes(hubp: *mut hubp, attr: *const dc_cursor_attributes) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let hw_pitch = hubp1_get_cursor_pitch((*attr).pitch);
    let lpc = hubp2_get_lines_per_chunk((*attr).width, (*attr).color_format);
    let cursor_width = (((*attr).width + 63) / 64) * 64;
    (*hubp).curs_attr = *attr;
    if !(*hubp).cursor_offload {
        REG_UPDATE!(hubp2, CURSOR_SURFACE_ADDRESS_HIGH, CURSOR_SURFACE_ADDRESS_HIGH, (*attr).address.high_part);
        REG_UPDATE!(hubp2, CURSOR_SURFACE_ADDRESS, CURSOR_SURFACE_ADDRESS, (*attr).address.low_part);
        REG_UPDATE_2!(hubp2, CURSOR_SIZE, CURSOR_WIDTH, cursor_width, CURSOR_HEIGHT, (*attr).height);
        REG_UPDATE_4!(hubp2, CURSOR_CONTROL, CURSOR_MODE, (*attr).color_format, CURSOR_2X_MAGNIFY, (*attr).attribute_flags.bits.ENABLE_MAGNIFICATION, CURSOR_PITCH, hw_pitch, CURSOR_LINES_PER_CHUNK, lpc);
        REG_SET_3!(hubp2, CURSOR_SETTINGS, 0, CURSOR0_DST_Y_OFFSET, 0, CURSOR0_CHUNK_HDL_ADJUST, 3, FORCE_CURSOR_TO_DISP_PREF, (*attr).force_cursor_to_disp_pref);
    }
    (*hubp).att.SURFACE_ADDR_HIGH = (*attr).address.high_part;
    (*hubp).att.SURFACE_ADDR = (*attr).address.low_part;
    (*hubp).att.size.bits.width = (*attr).width;
    (*hubp).att.size.bits.height = (*attr).height;
    (*hubp).att.cur_ctl.bits.mode = (*attr).color_format;
    (*hubp).cur_rect.w = (*attr).width;
    (*hubp).cur_rect.h = (*attr).height;
    (*hubp).att.cur_ctl.bits.pitch = hw_pitch;
    (*hubp).att.cur_ctl.bits.line_per_chunk = lpc;
    (*hubp).att.cur_ctl.bits.cur_2x_magnify = (*attr).attribute_flags.bits.ENABLE_MAGNIFICATION;
    (*hubp).att.settings.bits.dst_y_offset = 0;
    (*hubp).att.settings.bits.chunk_hdl_adjust = 3;
    (*hubp).att.settings.bits.force_cursor_to_disp_pref = (*attr).force_cursor_to_disp_pref;
}

// Remaining register-state reads preserve the source's ordered REG_GET operations.
pub unsafe fn hubp60_read_state(hubp: *mut hubp) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let s = &mut (*hubp2).state;
    let dlg_attr = &mut s.dlg_attr;
    let ttu_attr = &mut s.ttu_attr;
    let rq_regs = &mut s.rq_regs;
    REG_GET!(hubp2, HUBPRET_CONTROL, DET_BUF_PLANE1_BASE_ADDRESS, &mut rq_regs.plane1_base_address);
    REG_GET_4!(hubp2, DCN_EXPANSION_MODE, DRQ_EXPANSION_MODE, &mut rq_regs.drq_expansion_mode, PRQ_EXPANSION_MODE, &mut rq_regs.prq_expansion_mode, MRQ_EXPANSION_MODE, &mut rq_regs.mrq_expansion_mode, CRQ_EXPANSION_MODE, &mut rq_regs.crq_expansion_mode);
    REG_GET_5!(hubp2, DCHUBP_REQ_SIZE_CONFIG, CHUNK_SIZE, &mut rq_regs.rq_regs_l.chunk_size, MIN_CHUNK_SIZE, &mut rq_regs.rq_regs_l.min_chunk_size, DPTE_GROUP_SIZE, &mut rq_regs.rq_regs_l.dpte_group_size, SWATH_HEIGHT, &mut rq_regs.rq_regs_l.swath_height, PTE_ROW_HEIGHT_LINEAR, &mut rq_regs.rq_regs_l.pte_row_height_linear);
    REG_GET_5!(hubp2, DCHUBP_REQ_SIZE_CONFIG_C, CHUNK_SIZE_C, &mut rq_regs.rq_regs_c.chunk_size, MIN_CHUNK_SIZE_C, &mut rq_regs.rq_regs_c.min_chunk_size, DPTE_GROUP_SIZE_C, &mut rq_regs.rq_regs_c.dpte_group_size, SWATH_HEIGHT_C, &mut rq_regs.rq_regs_c.swath_height, PTE_ROW_HEIGHT_LINEAR_C, &mut rq_regs.rq_regs_c.pte_row_height_linear);
    REG_GET!(hubp2, DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, MC_VM_SYSTEM_APERTURE_HIGH_ADDR, &mut rq_regs.aperture_high_addr);
    REG_GET!(hubp2, DCN_VM_SYSTEM_APERTURE_LOW_ADDR, MC_VM_SYSTEM_APERTURE_LOW_ADDR, &mut rq_regs.aperture_low_addr);
    REG_GET_2!(hubp2, BLANK_OFFSET_0, REFCYC_H_BLANK_END, &mut dlg_attr.refcyc_h_blank_end, DLG_V_BLANK_END, &mut dlg_attr.dlg_vblank_end);
    REG_GET!(hubp2, BLANK_OFFSET_1, MIN_DST_Y_NEXT_START, &mut dlg_attr.min_dst_y_next_start);
    REG_GET!(hubp2, DST_DIMENSIONS, REFCYC_PER_HTOTAL, &mut dlg_attr.refcyc_per_htotal);
    REG_GET_2!(hubp2, DST_AFTER_SCALER, REFCYC_X_AFTER_SCALER, &mut dlg_attr.refcyc_x_after_scaler, DST_Y_AFTER_SCALER, &mut dlg_attr.dst_y_after_scaler);
    REG_GET_2!(hubp2, PREFETCH_SETTINGS, DST_Y_PREFETCH, &mut dlg_attr.dst_y_prefetch, VRATIO_PREFETCH, &mut dlg_attr.vratio_prefetch);
    REG_GET_2!(hubp2, VBLANK_PARAMETERS_0, DST_Y_PER_VM_VBLANK, &mut dlg_attr.dst_y_per_vm_vblank, DST_Y_PER_ROW_VBLANK, &mut dlg_attr.dst_y_per_row_vblank);
    REG_GET!(hubp2, REF_FREQ_TO_PIX_FREQ, REF_FREQ_TO_PIX_FREQ, &mut dlg_attr.ref_freq_to_pix_freq);
    REG_GET!(hubp2, VBLANK_PARAMETERS_1, REFCYC_PER_PTE_GROUP_VBLANK_L, &mut dlg_attr.refcyc_per_pte_group_vblank_l);
    REG_GET!(hubp2, VBLANK_PARAMETERS_3, REFCYC_PER_META_CHUNK_VBLANK_L, &mut dlg_attr.refcyc_per_meta_chunk_vblank_l);
    REG_GET!(hubp2, NOM_PARAMETERS_0, DST_Y_PER_PTE_ROW_NOM_L, &mut dlg_attr.dst_y_per_pte_row_nom_l);
    REG_GET!(hubp2, NOM_PARAMETERS_1, REFCYC_PER_PTE_GROUP_NOM_L, &mut dlg_attr.refcyc_per_pte_group_nom_l);
    REG_GET!(hubp2, NOM_PARAMETERS_4, DST_Y_PER_META_ROW_NOM_L, &mut dlg_attr.dst_y_per_meta_row_nom_l);
    REG_GET!(hubp2, NOM_PARAMETERS_5, REFCYC_PER_META_CHUNK_NOM_L, &mut dlg_attr.refcyc_per_meta_chunk_nom_l);
    REG_GET_2!(hubp2, PER_LINE_DELIVERY_PRE, REFCYC_PER_LINE_DELIVERY_PRE_L, &mut dlg_attr.refcyc_per_line_delivery_pre_l, REFCYC_PER_LINE_DELIVERY_PRE_C, &mut dlg_attr.refcyc_per_line_delivery_pre_c);
    REG_GET_2!(hubp2, PER_LINE_DELIVERY, REFCYC_PER_LINE_DELIVERY_L, &mut dlg_attr.refcyc_per_line_delivery_l, REFCYC_PER_LINE_DELIVERY_C, &mut dlg_attr.refcyc_per_line_delivery_c);
    REG_GET!(hubp2, PREFETCH_SETTINGS_C, VRATIO_PREFETCH_C, &mut dlg_attr.vratio_prefetch_c);
    REG_GET!(hubp2, VBLANK_PARAMETERS_2, REFCYC_PER_PTE_GROUP_VBLANK_C, &mut dlg_attr.refcyc_per_pte_group_vblank_c);
    REG_GET!(hubp2, VBLANK_PARAMETERS_4, REFCYC_PER_META_CHUNK_VBLANK_C, &mut dlg_attr.refcyc_per_meta_chunk_vblank_c);
    REG_GET!(hubp2, NOM_PARAMETERS_2, DST_Y_PER_PTE_ROW_NOM_C, &mut dlg_attr.dst_y_per_pte_row_nom_c);
    REG_GET!(hubp2, NOM_PARAMETERS_3, REFCYC_PER_PTE_GROUP_NOM_C, &mut dlg_attr.refcyc_per_pte_group_nom_c);
    REG_GET!(hubp2, NOM_PARAMETERS_6, DST_Y_PER_META_ROW_NOM_C, &mut dlg_attr.dst_y_per_meta_row_nom_c);
    REG_GET!(hubp2, NOM_PARAMETERS_7, REFCYC_PER_META_CHUNK_NOM_C, &mut dlg_attr.refcyc_per_meta_chunk_nom_c);
    REG_GET!(hubp2, DCN_GLOBAL_TTU_CNTL, MIN_TTU_VBLANK, &mut ttu_attr.min_ttu_vblank);
    REG_GET!(hubp2, DCN_SURF0_TTU_CNTL0, REFCYC_PER_REQ_DELIVERY, &mut ttu_attr.refcyc_per_req_delivery_l);
    REG_GET!(hubp2, DCN_SURF0_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut ttu_attr.refcyc_per_req_delivery_pre_l);
    REG_GET!(hubp2, DCN_SURF1_TTU_CNTL0, REFCYC_PER_REQ_DELIVERY, &mut ttu_attr.refcyc_per_req_delivery_c);
    REG_GET!(hubp2, DCN_SURF1_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut ttu_attr.refcyc_per_req_delivery_pre_c);
    REG_GET!(hubp2, DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, &mut s.pixel_format);
    REG_GET!(hubp2, DCSURF_SURFACE_EARLIEST_INUSE_HIGH, SURFACE_EARLIEST_INUSE_ADDRESS_HIGH, &mut s.inuse_addr_hi);
    REG_GET!(hubp2, DCSURF_SURFACE_EARLIEST_INUSE, SURFACE_EARLIEST_INUSE_ADDRESS, &mut s.inuse_addr_lo);
    REG_GET_2!(hubp2, DCSURF_PRI_VIEWPORT_DIMENSION, PRI_VIEWPORT_WIDTH, &mut s.viewport_width, PRI_VIEWPORT_HEIGHT, &mut s.viewport_height);
    REG_GET_2!(hubp2, DCSURF_SURFACE_CONFIG, ROTATION_ANGLE, &mut s.rotation_angle, H_MIRROR_EN, &mut s.h_mirror_en);
    REG_GET!(hubp2, DCSURF_TILING_CONFIG, SW_MODE, &mut s.sw_mode);
    REG_GET!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_DCC_EN, &mut s.dcc_en);
    REG_GET_3!(hubp2, DCHUBP_CNTL, HUBP_BLANK_EN, &mut s.blank_en, HUBP_TTU_DISABLE, &mut s.ttu_disable, HUBP_UNDERFLOW_STATUS, &mut s.underflow_status);
    REG_GET!(hubp2, HUBP_CLK_CNTL, HUBP_CLOCK_ENABLE, &mut s.clock_en);
    REG_GET!(hubp2, DCN_GLOBAL_TTU_CNTL, MIN_TTU_VBLANK, &mut s.min_ttu_vblank);
    REG_GET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, PRIMARY_SURFACE_ADDRESS, &mut s.primary_surface_addr_lo);
    REG_GET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, PRIMARY_SURFACE_ADDRESS, &mut s.primary_surface_addr_hi);
    s.uclk_pstate_force = REG_READ!(hubp2, UCLK_PSTATE_FORCE);
    s.hubp_cntl = REG_READ!(hubp2, DCHUBP_CNTL);
    s.flip_control = REG_READ!(hubp2, DCSURF_FLIP_CONTROL);
}

pub unsafe fn hubp60_construct(hubp2: *mut dcn20_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_hubp2_registers, hubp_shift: *const dcn_hubp2_shift, hubp_mask: *const dcn_hubp2_mask) -> bool {
    (*hubp2).base.funcs = &dcn60_hubp_funcs;
    (*hubp2).base.ctx = ctx;
    (*hubp2).hubp_regs = hubp_regs;
    (*hubp2).hubp_shift = hubp_shift;
    (*hubp2).hubp_mask = hubp_mask;
    (*hubp2).base.inst = inst;
    (*hubp2).base.opp_id = OPP_ID_INVALID;
    (*hubp2).base.mpcc_id = 0xf;
    true
}

pub static mut dcn60_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer),
    hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled),
    hubp_program_surface_flip_and_addr: Some(hubp50_program_surface_flip_and_addr),
    hubp_program_surface_config: Some(hubp50_program_surface_config),
    hubp_is_flip_pending: Some(hubp2_is_flip_pending),
    hubp_setup2: Some(hubp60_setup),
    hubp_setup_interdependent2: Some(hubp60_setup_interdependent),
    hubp_set_vm_system_aperture_settings: Some(hubp3_set_vm_system_aperture_settings),
    set_blank: Some(hubp2_set_blank),
    set_blank_regs: Some(hubp2_set_blank_regs),
    hubp_reset: Some(hubp_reset),
    mem_program_viewport: Some(hubp401_set_viewport),
    set_cursor_attributes: Some(hubp60_cursor_set_attributes),
    set_cursor_position: Some(hubp401_cursor_set_position),
    hubp_clk_cntl: Some(hubp2_clk_cntl),
    hubp_vtg_sel: Some(hubp2_vtg_sel),
    dmdata_set_attributes: Some(hubp3_dmdata_set_attributes),
    dmdata_load: Some(hubp2_dmdata_load),
    dmdata_status_done: Some(hubp2_dmdata_status_done),
    hubp_read_state: Some(hubp60_read_state),
    hubp_clear_underflow: Some(hubp2_clear_underflow),
    hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl),
    hubp_init: Some(hubp401_init),
    set_unbounded_requesting: Some(hubp401_set_unbounded_requesting),
    hubp_soft_reset: Some(hubp31_soft_reset),
    hubp_set_flip_int: Some(hubp401_set_flip_int),
    hubp_in_blank: Some(hubp401_in_blank),
    phantom_hubp_post_enable: Some(hubp32_phantom_hubp_post_enable),
    hubp_update_mall_sel: None,
    hubp_prepare_subvp_buffering: Some(hubp32_prepare_subvp_buffering),
    hubp_program_mcache_id_and_split_coordinate: Some(hubp401_program_mcache_id_and_split_coordinate),
    hubp_program_3dlut_fl_addr: Some(hubp401_program_3dlut_fl_addr),
    hubp_program_3dlut_fl_config: Some(hubp42_program_3dlut_fl_config),
    hubp_program_3dlut_fl_dlg_param: Some(hubp401_program_3dlut_fl_dlg_param),
    hubp_enable_3dlut_fl: Some(hubp401_enable_3dlut_fl),
    hubp_program_3dlut_fl_crossbar: Some(hubp42_program_3dlut_fl_crossbar),
    hubp_get_3dlut_fl_done: Some(hubp401_get_3dlut_fl_done),
    hubp_clear_tiling: Some(hubp401_clear_tiling),
    hubp_read_reg_state: Some(hubp3_read_reg_state),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
