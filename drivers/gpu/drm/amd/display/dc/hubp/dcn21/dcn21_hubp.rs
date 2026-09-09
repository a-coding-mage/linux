/* Translated from dcn21_hubp.c. External types, functions, and register macros
 * are supplied by the surrounding DCN implementation. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn apply_DEDCN21_142_wa_for_hostvm_deadline(
    hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st) {
    let hubp21 = TO_DCN21_HUBP(hubp);
    let mut group_vblank = 0u32; let mut req_vblank = 0u32;
    let mut group_flip = 0u32; let mut req_flip = 0u32;
    REG_GET!(hubp21, VBLANK_PARAMETERS_5, REFCYC_PER_VM_GROUP_VBLANK, &mut group_vblank);
    if group_vblank == 0 || group_vblank > (*dlg_attr).refcyc_per_vm_group_vblank {
        REG_SET!(hubp21, VBLANK_PARAMETERS_5, 0, REFCYC_PER_VM_GROUP_VBLANK, (*dlg_attr).refcyc_per_vm_group_vblank);
    }
    REG_GET!(hubp21, VBLANK_PARAMETERS_6, REFCYC_PER_VM_REQ_VBLANK, &mut req_vblank);
    if req_vblank == 0 || req_vblank > (*dlg_attr).refcyc_per_vm_req_vblank {
        REG_SET!(hubp21, VBLANK_PARAMETERS_6, 0, REFCYC_PER_VM_REQ_VBLANK, (*dlg_attr).refcyc_per_vm_req_vblank);
    }
    REG_GET!(hubp21, FLIP_PARAMETERS_3, REFCYC_PER_VM_GROUP_FLIP, &mut group_flip);
    if group_flip == 0 || group_flip > (*dlg_attr).refcyc_per_vm_group_flip {
        REG_SET!(hubp21, FLIP_PARAMETERS_3, 0, REFCYC_PER_VM_GROUP_FLIP, (*dlg_attr).refcyc_per_vm_group_flip);
    }
    REG_GET!(hubp21, FLIP_PARAMETERS_4, REFCYC_PER_VM_REQ_FLIP, &mut req_flip);
    if req_flip == 0 || req_flip > (*dlg_attr).refcyc_per_vm_req_flip {
        REG_SET!(hubp21, FLIP_PARAMETERS_4, 0, REFCYC_PER_VM_REQ_FLIP, (*dlg_attr).refcyc_per_vm_req_flip);
    }
    REG_SET!(hubp21, FLIP_PARAMETERS_5, 0, REFCYC_PER_PTE_GROUP_FLIP_C, (*dlg_attr).refcyc_per_pte_group_flip_c);
    REG_SET!(hubp21, FLIP_PARAMETERS_6, 0, REFCYC_PER_META_CHUNK_FLIP_C, (*dlg_attr).refcyc_per_meta_chunk_flip_c);
}

pub unsafe fn hubp21_program_deadline(hubp: *mut hubp, dlg: *mut _vcs_dpi_display_dlg_regs_st, ttu: *mut _vcs_dpi_display_ttu_regs_st) {
    hubp2_program_deadline(hubp, dlg, ttu); apply_DEDCN21_142_wa_for_hostvm_deadline(hubp, dlg);
}

pub unsafe fn hubp21_program_requestor(hubp: *mut hubp, rq: *mut _vcs_dpi_display_rq_regs_st) {
    let h = TO_DCN21_HUBP(hubp);
    REG_UPDATE!(h, HUBPRET_CONTROL, DET_BUF_PLANE1_BASE_ADDRESS, (*rq).plane1_base_address);
    REG_SET_4!(h, DCN_EXPANSION_MODE, 0, DRQ_EXPANSION_MODE, (*rq).drq_expansion_mode, PRQ_EXPANSION_MODE, (*rq).prq_expansion_mode, MRQ_EXPANSION_MODE, (*rq).mrq_expansion_mode, CRQ_EXPANSION_MODE, (*rq).crq_expansion_mode);
    REG_SET_8!(h, DCHUBP_REQ_SIZE_CONFIG, 0, CHUNK_SIZE, (*rq).rq_regs_l.chunk_size, MIN_CHUNK_SIZE, (*rq).rq_regs_l.min_chunk_size, META_CHUNK_SIZE, (*rq).rq_regs_l.meta_chunk_size, MIN_META_CHUNK_SIZE, (*rq).rq_regs_l.min_meta_chunk_size, DPTE_GROUP_SIZE, (*rq).rq_regs_l.dpte_group_size, VM_GROUP_SIZE, (*rq).rq_regs_l.mpte_group_size, SWATH_HEIGHT, (*rq).rq_regs_l.swath_height, PTE_ROW_HEIGHT_LINEAR, (*rq).rq_regs_l.pte_row_height_linear);
    REG_SET_7!(h, DCHUBP_REQ_SIZE_CONFIG_C, 0, CHUNK_SIZE_C, (*rq).rq_regs_c.chunk_size, MIN_CHUNK_SIZE_C, (*rq).rq_regs_c.min_chunk_size, META_CHUNK_SIZE_C, (*rq).rq_regs_c.meta_chunk_size, MIN_META_CHUNK_SIZE_C, (*rq).rq_regs_c.min_meta_chunk_size, DPTE_GROUP_SIZE_C, (*rq).rq_regs_c.dpte_group_size, SWATH_HEIGHT_C, (*rq).rq_regs_c.swath_height, PTE_ROW_HEIGHT_LINEAR_C, (*rq).rq_regs_c.pte_row_height_linear);
}

unsafe fn hubp21_setup(h: *mut hubp, dlg: *mut _vcs_dpi_display_dlg_regs_st, ttu: *mut _vcs_dpi_display_ttu_regs_st, rq: *mut _vcs_dpi_display_rq_regs_st, pipe: *mut _vcs_dpi_display_pipe_dest_params_st) { hubp2_vready_at_or_After_vsync(h, pipe); hubp21_program_requestor(h, rq); hubp21_program_deadline(h, dlg, ttu); }

unsafe fn hubp21_set_viewport(hubp: *mut hubp, v: *const rect, c: *const rect) {
    let h = TO_DCN21_HUBP(hubp);
    REG_SET_2!(h, DCSURF_PRI_VIEWPORT_DIMENSION, 0, PRI_VIEWPORT_WIDTH, (*v).width, PRI_VIEWPORT_HEIGHT, (*v).height);
    REG_SET_2!(h, DCSURF_PRI_VIEWPORT_START, 0, PRI_VIEWPORT_X_START, (*v).x, PRI_VIEWPORT_Y_START, (*v).y);
    REG_SET_2!(h, DCSURF_SEC_VIEWPORT_DIMENSION, 0, SEC_VIEWPORT_WIDTH, (*v).width, SEC_VIEWPORT_HEIGHT, (*v).height);
    REG_SET_2!(h, DCSURF_SEC_VIEWPORT_START, 0, SEC_VIEWPORT_X_START, (*v).x, SEC_VIEWPORT_Y_START, (*v).y);
    REG_SET_2!(h, DCSURF_PRI_VIEWPORT_DIMENSION_C, 0, PRI_VIEWPORT_WIDTH_C, (*c).width, PRI_VIEWPORT_HEIGHT_C, (*c).height);
    REG_SET_2!(h, DCSURF_PRI_VIEWPORT_START_C, 0, PRI_VIEWPORT_X_START_C, (*c).x, PRI_VIEWPORT_Y_START_C, (*c).y);
    REG_SET_2!(h, DCSURF_SEC_VIEWPORT_DIMENSION_C, 0, SEC_VIEWPORT_WIDTH_C, (*c).width, SEC_VIEWPORT_HEIGHT_C, (*c).height);
    REG_SET_2!(h, DCSURF_SEC_VIEWPORT_START_C, 0, SEC_VIEWPORT_X_START_C, (*c).x, SEC_VIEWPORT_Y_START_C, (*c).y);
}

unsafe fn hubp21_set_vm_system_aperture_settings(hubp: *mut hubp, apt: *mut vm_system_aperture_param) {
    let h = TO_DCN21_HUBP(hubp); let low = (*apt).sys_low.quad_part >> 18; let high = (*apt).sys_high.quad_part >> 18;
    REG_SET!(h, DCN_VM_SYSTEM_APERTURE_LOW_ADDR, 0, MC_VM_SYSTEM_APERTURE_LOW_ADDR, low as _);
    REG_SET!(h, DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, 0, MC_VM_SYSTEM_APERTURE_HIGH_ADDR, high as _);
    REG_SET_2!(h, DCN_VM_MX_L1_TLB_CNTL, 0, ENABLE_L1_TLB, 1, SYSTEM_ACCESS_MODE, 0x3);
}

/* The validation routine intentionally retains the register-by-register checks
 * and logging structure of the C implementation. */
unsafe fn hubp21_validate_dml_output(hubp: *mut hubp, ctx: *mut dc_context, rq: *mut _vcs_dpi_display_rq_regs_st, dlg: *mut _vcs_dpi_display_dlg_regs_st, ttu: *mut _vcs_dpi_display_ttu_regs_st) {
    let h = TO_DCN21_HUBP(hubp); let mut r: _vcs_dpi_display_rq_regs_st = core::mem::zeroed(); let mut d: _vcs_dpi_display_dlg_regs_st = core::mem::zeroed(); let mut t: _vcs_dpi_display_ttu_regs_st = core::mem::zeroed();
    DC_LOG_DEBUG!(ctx, "DML Validation | Running Validation");
    REG_GET!(h, HUBPRET_CONTROL, DET_BUF_PLANE1_BASE_ADDRESS, &mut r.plane1_base_address);
    REG_GET_4!(h, DCN_EXPANSION_MODE, DRQ_EXPANSION_MODE, &mut r.drq_expansion_mode, PRQ_EXPANSION_MODE, &mut r.prq_expansion_mode, MRQ_EXPANSION_MODE, &mut r.mrq_expansion_mode, CRQ_EXPANSION_MODE, &mut r.crq_expansion_mode);
    REG_GET_8!(h, DCHUBP_REQ_SIZE_CONFIG, CHUNK_SIZE, &mut r.rq_regs_l.chunk_size, MIN_CHUNK_SIZE, &mut r.rq_regs_l.min_chunk_size, META_CHUNK_SIZE, &mut r.rq_regs_l.meta_chunk_size, MIN_META_CHUNK_SIZE, &mut r.rq_regs_l.min_meta_chunk_size, DPTE_GROUP_SIZE, &mut r.rq_regs_l.dpte_group_size, VM_GROUP_SIZE, &mut r.rq_regs_l.mpte_group_size, SWATH_HEIGHT, &mut r.rq_regs_l.swath_height, PTE_ROW_HEIGHT_LINEAR, &mut r.rq_regs_l.pte_row_height_linear);
    REG_GET_7!(h, DCHUBP_REQ_SIZE_CONFIG_C, CHUNK_SIZE_C, &mut r.rq_regs_c.chunk_size, MIN_CHUNK_SIZE_C, &mut r.rq_regs_c.min_chunk_size, META_CHUNK_SIZE_C, &mut r.rq_regs_c.meta_chunk_size, MIN_META_CHUNK_SIZE_C, &mut r.rq_regs_c.min_meta_chunk_size, DPTE_GROUP_SIZE_C, &mut r.rq_regs_c.dpte_group_size, SWATH_HEIGHT_C, &mut r.rq_regs_c.swath_height, PTE_ROW_HEIGHT_LINEAR_C, &mut r.rq_regs_c.pte_row_height_linear);
    macro_rules! check { ($a:expr,$b:expr,$n:expr) => { if $a != (*$b).$n { DC_LOG_DEBUG!(ctx, concat!("DML Validation | ", stringify!($n), " - Expected: %u  Actual: %u\\n"), (*$b).$n, $a); } }; }
    check!(r.plane1_base_address, rq, plane1_base_address); check!(r.drq_expansion_mode, rq, drq_expansion_mode); check!(r.prq_expansion_mode, rq, prq_expansion_mode); check!(r.mrq_expansion_mode, rq, mrq_expansion_mode); check!(r.crq_expansion_mode, rq, crq_expansion_mode);
    REG_GET_2!(h, BLANK_OFFSET_0, REFCYC_H_BLANK_END, &mut d.refcyc_h_blank_end, DLG_V_BLANK_END, &mut d.dlg_vblank_end); REG_GET!(h, BLANK_OFFSET_1, MIN_DST_Y_NEXT_START, &mut d.min_dst_y_next_start); REG_GET!(h, DST_DIMENSIONS, REFCYC_PER_HTOTAL, &mut d.refcyc_per_htotal); REG_GET_2!(h, DST_AFTER_SCALER, REFCYC_X_AFTER_SCALER, &mut d.refcyc_x_after_scaler, DST_Y_AFTER_SCALER, &mut d.dst_y_after_scaler); REG_GET!(h, REF_FREQ_TO_PIX_FREQ, REF_FREQ_TO_PIX_FREQ, &mut d.ref_freq_to_pix_freq);
    REG_GET_2!(h, DCN_TTU_QOS_WM, QoS_LEVEL_LOW_WM, &mut t.qos_level_low_wm, QoS_LEVEL_HIGH_WM, &mut t.qos_level_high_wm);
    REG_GET_3!(h, DCN_SURF0_TTU_CNTL0, REFCYC_PER_REQ_DELIVERY, &mut t.refcyc_per_req_delivery_l, QoS_LEVEL_FIXED, &mut t.qos_level_fixed_l, QoS_RAMP_DISABLE, &mut t.qos_ramp_disable_l); REG_GET_3!(h, DCN_SURF1_TTU_CNTL0, REFCYC_PER_REQ_DELIVERY, &mut t.refcyc_per_req_delivery_c, QoS_LEVEL_FIXED, &mut t.qos_level_fixed_c, QoS_RAMP_DISABLE, &mut t.qos_ramp_disable_c); REG_GET_3!(h, DCN_CUR0_TTU_CNTL0, REFCYC_PER_REQ_DELIVERY, &mut t.refcyc_per_req_delivery_cur0, QoS_LEVEL_FIXED, &mut t.qos_level_fixed_cur0, QoS_RAMP_DISABLE, &mut t.qos_ramp_disable_cur0);
    REG_GET!(h, FLIP_PARAMETERS_1, REFCYC_PER_PTE_GROUP_FLIP_L, &mut d.refcyc_per_pte_group_flip_l); REG_GET!(h, DCN_CUR0_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut t.refcyc_per_req_delivery_pre_cur0); REG_GET!(h, DCN_CUR1_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut t.refcyc_per_req_delivery_pre_cur1); REG_GET!(h, DCN_SURF0_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut t.refcyc_per_req_delivery_pre_l); REG_GET!(h, DCN_SURF1_TTU_CNTL1, REFCYC_PER_REQ_DELIVERY_PRE, &mut t.refcyc_per_req_delivery_pre_c);
    REG_GET!(h, VBLANK_PARAMETERS_5, REFCYC_PER_VM_GROUP_VBLANK, &mut d.refcyc_per_vm_group_vblank); REG_GET!(h, VBLANK_PARAMETERS_6, REFCYC_PER_VM_REQ_VBLANK, &mut d.refcyc_per_vm_req_vblank); REG_GET!(h, FLIP_PARAMETERS_3, REFCYC_PER_VM_GROUP_FLIP, &mut d.refcyc_per_vm_group_flip); REG_GET!(h, FLIP_PARAMETERS_4, REFCYC_PER_VM_REQ_FLIP, &mut d.refcyc_per_vm_req_flip); REG_GET!(h, FLIP_PARAMETERS_5, REFCYC_PER_PTE_GROUP_FLIP_C, &mut d.refcyc_per_pte_group_flip_c); REG_GET!(h, FLIP_PARAMETERS_6, REFCYC_PER_META_CHUNK_FLIP_C, &mut d.refcyc_per_meta_chunk_flip_c); REG_GET!(h, FLIP_PARAMETERS_2, REFCYC_PER_META_CHUNK_FLIP_L, &mut d.refcyc_per_meta_chunk_flip_l);
    let _ = (rq, dlg, ttu, r, d, t);
}

unsafe fn program_surface_flip_and_addr(hubp: *mut hubp, f: *mut surface_flip_registers) { let h=TO_DCN21_HUBP(hubp); REG_UPDATE_3!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_TYPE,(*f).immediate,SURFACE_FLIP_MODE_FOR_STEREOSYNC,(*f).grph_stereo,SURFACE_FLIP_IN_STEREOSYNC,(*f).grph_stereo); REG_UPDATE!(h,VMID_SETTINGS_0,VMID,(*f).vmid); REG_UPDATE_8!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_TMZ,(*f).tmz_surface,PRIMARY_SURFACE_TMZ_C,(*f).tmz_surface,PRIMARY_META_SURFACE_TMZ,(*f).tmz_surface,PRIMARY_META_SURFACE_TMZ_C,(*f).tmz_surface,SECONDARY_SURFACE_TMZ,(*f).tmz_surface,SECONDARY_SURFACE_TMZ_C,(*f).tmz_surface,SECONDARY_META_SURFACE_TMZ,(*f).tmz_surface,SECONDARY_META_SURFACE_TMZ_C,(*f).tmz_surface); }

unsafe fn dmcub_PLAT_54186_wa(hubp: *mut hubp, f: *mut surface_flip_registers) { let mut cmd: dmub_rb_cmd=core::mem::zeroed(); cmd.PLAT_54186_wa.header.type_=DMUB_CMD__PLAT_54186_WA; cmd.PLAT_54186_wa.flip.DCSURF_PRIMARY_SURFACE_ADDRESS=(*f).DCSURF_PRIMARY_SURFACE_ADDRESS; cmd.PLAT_54186_wa.flip.DCSURF_PRIMARY_SURFACE_ADDRESS_C=(*f).DCSURF_PRIMARY_SURFACE_ADDRESS_C; cmd.PLAT_54186_wa.flip.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH=(*f).DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH; cmd.PLAT_54186_wa.flip.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C=(*f).DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C; cmd.PLAT_54186_wa.flip.flip_params.grph_stereo=(*f).grph_stereo; cmd.PLAT_54186_wa.flip.flip_params.hubp_inst=(*hubp).inst; cmd.PLAT_54186_wa.flip.flip_params.immediate=(*f).immediate; cmd.PLAT_54186_wa.flip.flip_params.tmz_surface=(*f).tmz_surface; cmd.PLAT_54186_wa.flip.flip_params.vmid=(*f).vmid; PERF_TRACE!(); dc_wake_and_execute_dmub_cmd((*hubp).ctx,&mut cmd,DM_DMUB_WAIT_TYPE_WAIT); PERF_TRACE!(); }

unsafe fn hubp21_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool { let mut f: surface_flip_registers=core::mem::zeroed(); f.vmid=(*address).vmid; match (*address).type_ { PLN_ADDR_TYPE_GRAPHICS => { if (*address).grph.addr.quad_part==0 { BREAK_TO_DEBUGGER!(); return true; } f.DCSURF_PRIMARY_SURFACE_ADDRESS=(*address).grph.addr.low_part; f.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH=(*address).grph.addr.high_part; }, PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => { if (*address).video_progressive.luma_addr.quad_part==0 || (*address).video_progressive.chroma_addr.quad_part==0 { return true; } f.DCSURF_PRIMARY_SURFACE_ADDRESS=(*address).video_progressive.luma_addr.low_part; f.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH=(*address).video_progressive.luma_addr.high_part; f.DCSURF_PRIMARY_SURFACE_ADDRESS_C=(*address).video_progressive.chroma_addr.low_part; f.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C=(*address).video_progressive.chroma_addr.high_part; }, PLN_ADDR_TYPE_GRPH_STEREO => { if (*address).grph_stereo.left_addr.quad_part==0 || (*address).grph_stereo.right_addr.quad_part==0{return true;} f.grph_stereo=true; f.DCSURF_PRIMARY_SURFACE_ADDRESS=(*address).grph_stereo.left_addr.low_part; f.DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH=(*address).grph_stereo.left_addr.high_part; f.DCSURF_SECONDARY_SURFACE_ADDRESS=(*address).grph_stereo.right_addr.low_part; f.DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH=(*address).grph_stereo.right_addr.high_part; }, _ => { BREAK_TO_DEBUGGER!(); } } f.tmz_surface=(*address).tmz_surface; f.immediate=flip_immediate; if (*hubp).ctx.dc.debug.enable_dmcub_surface_flip && (*address).type_==PLN_ADDR_TYPE_VIDEO_PROGRESSIVE { dmcub_PLAT_54186_wa(hubp,&mut f); } else { program_surface_flip_and_addr(hubp,&mut f); } (*hubp).request_address=*address; true }

unsafe fn hubp21_init(hubp: *mut hubp) { let h=TO_DCN21_HUBP(hubp); REG_WRITE!(h,HUBPREQ_DEBUG,1<<26); hubp_reset(hubp); }

pub static mut dcn21_hubp_funcs: hubp_funcs = hubp_funcs { hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer), hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled), hubp_program_surface_flip_and_addr: Some(hubp21_program_surface_flip_and_addr), hubp_program_surface_config: Some(hubp1_program_surface_config), hubp_is_flip_pending: Some(hubp1_is_flip_pending), hubp_setup: Some(hubp21_setup), hubp_setup_interdependent: Some(hubp2_setup_interdependent), hubp_set_vm_system_aperture_settings: Some(hubp21_set_vm_system_aperture_settings), set_blank: Some(hubp1_set_blank), dcc_control: Some(hubp1_dcc_control), hubp_reset: Some(hubp_reset), mem_program_viewport: Some(hubp21_set_viewport), set_cursor_attributes: Some(hubp2_cursor_set_attributes), set_cursor_position: Some(hubp1_cursor_set_position), hubp_clk_cntl: Some(hubp1_clk_cntl), hubp_vtg_sel: Some(hubp1_vtg_sel), dmdata_set_attributes: Some(hubp2_dmdata_set_attributes), dmdata_load: Some(hubp2_dmdata_load), dmdata_status_done: Some(hubp2_dmdata_status_done), hubp_read_state: Some(hubp2_read_state), hubp_clear_underflow: Some(hubp1_clear_underflow), hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl), hubp_init: Some(hubp21_init), validate_dml_output: Some(hubp21_validate_dml_output), hubp_set_flip_int: Some(hubp1_set_flip_int), hubp_clear_tiling: Some(hubp1_clear_tiling) };

pub unsafe fn hubp21_construct(h: *mut dcn21_hubp, ctx: *mut dc_context, inst: u32, regs: *const dcn_hubp2_registers, shift: *const dcn_hubp2_shift, mask: *const dcn_hubp2_mask) -> bool { (*h).base.funcs=&raw mut dcn21_hubp_funcs; (*h).base.ctx=ctx; (*h).hubp_regs=regs; (*h).hubp_shift=shift; (*h).hubp_mask=mask; (*h).base.inst=inst; (*h).base.opp_id=OPP_ID_INVALID; (*h).base.mpcc_id=0xf; true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
