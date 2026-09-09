/* Translated from dcn30_hubp.c; external types, functions, and register macros
 * are supplied by the surrounding display driver. */

pub unsafe fn hubp3_set_vm_system_aperture_settings(hubp: *mut hubp, apt: *mut vm_system_aperture_param) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let mut mc_vm_apt_low: PHYSICAL_ADDRESS_LOC = core::mem::zeroed();
    let mut mc_vm_apt_high: PHYSICAL_ADDRESS_LOC = core::mem::zeroed();
    // The format of high/low are 48:18 of the 48 bit addr
    (*mc_vm_apt_low).quad_part = (*apt).sys_low.quad_part >> 18;
    (*mc_vm_apt_high).quad_part = (*apt).sys_high.quad_part >> 18;
    REG_SET!(hubp2, DCN_VM_SYSTEM_APERTURE_LOW_ADDR, 0, MC_VM_SYSTEM_APERTURE_LOW_ADDR, (*mc_vm_apt_low).low_part);
    REG_SET!(hubp2, DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, 0, MC_VM_SYSTEM_APERTURE_HIGH_ADDR, (*mc_vm_apt_high).low_part);
    REG_SET_2!(hubp2, DCN_VM_MX_L1_TLB_CNTL, 0, ENABLE_L1_TLB, 1, SYSTEM_ACCESS_MODE, 0x3);
}

pub unsafe fn hubp3_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_TYPE, flip_immediate);
    if !flip_immediate { REG_UPDATE!(hubp2, VMID_SETTINGS_0, VMID, (*address).vmid); }
    if (*address).type_ == PLN_ADDR_TYPE_GRPH_STEREO {
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_MODE_FOR_STEREOSYNC, 0);
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_IN_STEREOSYNC, 0x1);
    } else {
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_MODE_FOR_STEREOSYNC, 0x0);
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_IN_STEREOSYNC, 0x0);
    }
    match (*address).type_ {
        PLN_ADDR_TYPE_GRAPHICS => {
            if (*address).grph.addr.quad_part == 0 { return false; }
            REG_UPDATE_2!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ, (*address).tmz_surface);
            if (*address).grph.meta_addr.quad_part != 0 {
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH, (*address).grph.meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS, 0, PRIMARY_META_SURFACE_ADDRESS, (*address).grph.meta_addr.low_part);
            }
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).grph.addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).grph.addr.low_part);
        }
        PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => {
            if (*address).video_progressive.luma_addr.quad_part == 0 || (*address).video_progressive.chroma_addr.quad_part == 0 { return false; }
            REG_UPDATE_4!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ_C, (*address).tmz_surface);
            if (*address).video_progressive.luma_meta_addr.quad_part != 0 {
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH_C, (*address).video_progressive.chroma_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_C, 0, PRIMARY_META_SURFACE_ADDRESS_C, (*address).video_progressive.chroma_meta_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH, (*address).video_progressive.luma_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS, 0, PRIMARY_META_SURFACE_ADDRESS, (*address).video_progressive.luma_meta_addr.low_part);
            }
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, (*address).video_progressive.chroma_addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, (*address).video_progressive.chroma_addr.low_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).video_progressive.luma_addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).video_progressive.luma_addr.low_part);
        }
        PLN_ADDR_TYPE_GRPH_STEREO => {
            if (*address).grph_stereo.left_addr.quad_part == 0 || (*address).grph_stereo.right_addr.quad_part == 0 { return false; }
            REG_UPDATE_8!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ_C, (*address).tmz_surface, SECONDARY_SURFACE_TMZ, (*address).tmz_surface, SECONDARY_SURFACE_TMZ_C, (*address).tmz_surface, SECONDARY_META_SURFACE_TMZ, (*address).tmz_surface, SECONDARY_META_SURFACE_TMZ_C, (*address).tmz_surface);
            if (*address).grph_stereo.right_meta_addr.quad_part != 0 {
                REG_SET!(hubp2, DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH_C, 0, SECONDARY_META_SURFACE_ADDRESS_HIGH_C, (*address).grph_stereo.right_alpha_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_META_SURFACE_ADDRESS_C, 0, SECONDARY_META_SURFACE_ADDRESS_C, (*address).grph_stereo.right_alpha_meta_addr.low_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH, 0, SECONDARY_META_SURFACE_ADDRESS_HIGH, (*address).grph_stereo.right_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_META_SURFACE_ADDRESS, 0, SECONDARY_META_SURFACE_ADDRESS, (*address).grph_stereo.right_meta_addr.low_part);
            }
            if (*address).grph_stereo.left_meta_addr.quad_part != 0 {
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH_C, (*address).grph_stereo.left_alpha_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_C, 0, PRIMARY_META_SURFACE_ADDRESS_C, (*address).grph_stereo.left_alpha_meta_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH, (*address).grph_stereo.left_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS, 0, PRIMARY_META_SURFACE_ADDRESS, (*address).grph_stereo.left_meta_addr.low_part);
            }
            REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH_C, 0, SECONDARY_SURFACE_ADDRESS_HIGH_C, (*address).grph_stereo.right_alpha_addr.high_part);
            REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_C, 0, SECONDARY_SURFACE_ADDRESS_C, (*address).grph_stereo.right_alpha_addr.low_part);
            REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH, 0, SECONDARY_SURFACE_ADDRESS_HIGH, (*address).grph_stereo.right_addr.high_part);
            REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS, 0, SECONDARY_SURFACE_ADDRESS, (*address).grph_stereo.right_addr.low_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, (*address).grph_stereo.left_alpha_addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, (*address).grph_stereo.left_alpha_addr.low_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).grph_stereo.left_addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).grph_stereo.left_addr.low_part);
        }
        PLN_ADDR_TYPE_RGBEA => {
            if (*address).rgbea.addr.quad_part == 0 || (*address).rgbea.alpha_addr.quad_part == 0 { return false; }
            REG_UPDATE_4!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_META_SURFACE_TMZ_C, (*address).tmz_surface);
            if (*address).rgbea.meta_addr.quad_part != 0 {
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH_C, (*address).rgbea.alpha_meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_C, 0, PRIMARY_META_SURFACE_ADDRESS_C, (*address).rgbea.alpha_meta_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH, 0, PRIMARY_META_SURFACE_ADDRESS_HIGH, (*address).rgbea.meta_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_META_SURFACE_ADDRESS, 0, PRIMARY_META_SURFACE_ADDRESS, (*address).rgbea.meta_addr.low_part);
            }
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, (*address).rgbea.alpha_addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, (*address).rgbea.alpha_addr.low_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).rgbea.addr.high_part);
            REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).rgbea.addr.low_part);
        }
        _ => { BREAK_TO_DEBUGGER!(); return false; }
    }
    (*hubp).request_address = *address;
    true
}

pub unsafe fn hubp3_program_tiling(hubp2: *mut dcn20_hubp, info: *const dc_tiling_info, _pixel_format: surface_pixel_format) {
    REG_UPDATE_4!(hubp2, DCSURF_ADDR_CONFIG, NUM_PIPES, log_2((*info).gfx9.num_pipes), PIPE_INTERLEAVE, (*info).gfx9.pipe_interleave, MAX_COMPRESSED_FRAGS, log_2((*info).gfx9.max_compressed_frags), NUM_PKRS, log_2((*info).gfx9.num_pkrs));
    REG_UPDATE_3!(hubp2, DCSURF_TILING_CONFIG, SW_MODE, (*info).gfx9.swizzle, META_LINEAR, (*info).gfx9.meta_linear, PIPE_ALIGNED, (*info).gfx9.pipe_aligned);
}

pub unsafe fn hubp3_clear_tiling(hubp: *mut hubp) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_UPDATE!(hubp2, DCHUBP_REQ_SIZE_CONFIG, SWATH_HEIGHT, 0);
    REG_UPDATE!(hubp2, DCSURF_TILING_CONFIG, SW_MODE, DC_SW_LINEAR);
    REG_UPDATE_6!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_DCC_EN, 0, PRIMARY_SURFACE_DCC_IND_BLK, 0, PRIMARY_SURFACE_DCC_IND_BLK_C, 0, SECONDARY_SURFACE_DCC_EN, 0, SECONDARY_SURFACE_DCC_IND_BLK, 0, SECONDARY_SURFACE_DCC_IND_BLK_C, 0);
}

pub unsafe fn hubp3_dcc_control(hubp: *mut hubp, enable: bool, blk_size: hubp_ind_block_size) {
    let hubp2 = TO_DCN20_HUBP(hubp); let dcc_en: u32 = if enable { 1 } else { 0 };
    REG_UPDATE_4!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_DCC_EN, dcc_en, PRIMARY_SURFACE_DCC_IND_BLK, blk_size, SECONDARY_SURFACE_DCC_EN, dcc_en, SECONDARY_SURFACE_DCC_IND_BLK, blk_size);
}

pub unsafe fn hubp3_dcc_control_sienna_cichlid(hubp: *mut hubp, dcc: *const dc_plane_dcc_param) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_UPDATE_6!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_DCC_EN, (*dcc).enable, PRIMARY_SURFACE_DCC_IND_BLK, (*dcc).dcc_ind_blk, PRIMARY_SURFACE_DCC_IND_BLK_C, (*dcc).dcc_ind_blk_c, SECONDARY_SURFACE_DCC_EN, (*dcc).enable, SECONDARY_SURFACE_DCC_IND_BLK, (*dcc).dcc_ind_blk, SECONDARY_SURFACE_DCC_IND_BLK_C, (*dcc).dcc_ind_blk_c);
}

pub unsafe fn hubp3_dmdata_set_attributes(hubp: *mut hubp, attr: *const dc_dmdata_attributes) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_UPDATE!(hubp2, DMDATA_CNTL, DMDATA_MODE, 1);
    REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_UPDATE_LOCK, 1);
    REG_UPDATE!(hubp2, DMDATA_CNTL, DMDATA_UPDATED, 0);
    REG_UPDATE_3!(hubp2, DMDATA_CNTL, DMDATA_UPDATED, 1, DMDATA_REPEAT, (*attr).dmdata_repeat, DMDATA_SIZE, (*attr).dmdata_size);
    REG_WRITE!(hubp2, DMDATA_ADDRESS_LOW, (*attr).address.low_part);
    REG_UPDATE!(hubp2, DMDATA_ADDRESS_HIGH, DMDATA_ADDRESS_HIGH, (*attr).address.high_part);
    REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_UPDATE_LOCK, 0);
}

pub unsafe fn hubp3_program_surface_config(hubp: *mut hubp, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, plane_size: *mut plane_size, rotation: dc_rotation_angle, dcc: *mut dc_plane_dcc_param, horizontal_mirror: bool, _compat_level: u32) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    hubp3_dcc_control_sienna_cichlid(hubp, dcc);
    hubp3_program_tiling(hubp2, tiling_info, format);
    hubp2_program_size(hubp, format, plane_size, dcc);
    hubp2_program_rotation(hubp, rotation, horizontal_mirror);
    hubp2_program_pixel_format(hubp, format);
}

unsafe fn hubp3_program_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st) {
    let hubp2 = TO_DCN20_HUBP(hubp); hubp2_program_deadline(hubp, dlg_attr, ttu_attr);
    REG_UPDATE!(hubp2, DCN_DMDATA_VM_CNTL, REFCYC_PER_VM_DMDATA, (*dlg_attr).refcyc_per_vm_dmdata);
}

pub unsafe fn hubp3_setup(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st, rq_regs: *mut _vcs_dpi_display_rq_regs_st, pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st) {
    /* otg is locked when this func is called. Register are double buffered. */
    hubp2_vready_at_or_After_vsync(hubp, pipe_dest); hubp21_program_requestor(hubp, rq_regs); hubp3_program_deadline(hubp, dlg_attr, ttu_attr);
}

pub unsafe fn hubp3_init(hubp: *mut hubp) {
    // DEDCN21-133: Inconsistent row starting line for flip between DPTE and Meta
    // This is a chicken bit to enable the ECO fix.
    let hubp2 = TO_DCN20_HUBP(hubp); REG_WRITE!(hubp2, HUBPREQ_DEBUG, 1 << 26); REG_UPDATE!(hubp2, DCHUBP_CNTL, HUBP_TTU_DISABLE, 0); hubp_reset(hubp);
}

pub unsafe fn hubp3_read_state(hubp: *mut hubp) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    hubp2_read_state_common(hubp);
    let s = &mut (*hubp2).state;
    let rq = &mut s.rq_regs;
    REG_GET_7!(hubp2, DCHUBP_REQ_SIZE_CONFIG, CHUNK_SIZE, rq.rq_regs_l.chunk_size, MIN_CHUNK_SIZE, rq.rq_regs_l.min_chunk_size, META_CHUNK_SIZE, rq.rq_regs_l.meta_chunk_size, MIN_META_CHUNK_SIZE, rq.rq_regs_l.min_meta_chunk_size, DPTE_GROUP_SIZE, rq.rq_regs_l.dpte_group_size, SWATH_HEIGHT, rq.rq_regs_l.swath_height, PTE_ROW_HEIGHT_LINEAR, rq.rq_regs_l.pte_row_height_linear);
    REG_GET_7!(hubp2, DCHUBP_REQ_SIZE_CONFIG_C, CHUNK_SIZE_C, rq.rq_regs_c.chunk_size, MIN_CHUNK_SIZE_C, rq.rq_regs_c.min_chunk_size, META_CHUNK_SIZE_C, rq.rq_regs_c.meta_chunk_size, MIN_META_CHUNK_SIZE_C, rq.rq_regs_c.min_meta_chunk_size, DPTE_GROUP_SIZE_C, rq.rq_regs_c.dpte_group_size, SWATH_HEIGHT_C, rq.rq_regs_c.swath_height, PTE_ROW_HEIGHT_LINEAR_C, rq.rq_regs_c.pte_row_height_linear);
    if REG!(hubp2, UCLK_PSTATE_FORCE) != 0 { s.uclk_pstate_force = REG_READ!(hubp2, UCLK_PSTATE_FORCE); }
    if REG!(hubp2, DCHUBP_CNTL) != 0 { s.hubp_cntl = REG_READ!(hubp2, DCHUBP_CNTL); }
    if REG!(hubp2, DCSURF_FLIP_CONTROL) != 0 { s.flip_control = REG_READ!(hubp2, DCSURF_FLIP_CONTROL); }
}

pub unsafe fn hubp3_read_reg_state(hubp: *mut hubp, reg_state: *mut dcn_hubp_reg_state) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    macro_rules! R { ($f:ident, $r:ident) => { (*reg_state).$f = REG_READ!(hubp2, $r); }; }
    R!(hubp_cntl,DCHUBP_CNTL); R!(mall_config,DCHUBP_MALL_CONFIG); R!(mall_sub_vp,DCHUBP_MALL_SUB_VP); R!(hubp_req_size_config,DCHUBP_REQ_SIZE_CONFIG); R!(hubp_req_size_config_c,DCHUBP_REQ_SIZE_CONFIG_C); R!(vmpg_config,DCHUBP_VMPG_CONFIG); R!(addr_config,DCSURF_ADDR_CONFIG); R!(pri_viewport_dimension,DCSURF_PRI_VIEWPORT_DIMENSION); R!(pri_viewport_dimension_c,DCSURF_PRI_VIEWPORT_DIMENSION_C); R!(pri_viewport_start,DCSURF_PRI_VIEWPORT_START); R!(pri_viewport_start_c,DCSURF_PRI_VIEWPORT_START_C); R!(sec_viewport_dimension,DCSURF_SEC_VIEWPORT_DIMENSION); R!(sec_viewport_dimension_c,DCSURF_SEC_VIEWPORT_DIMENSION_C); R!(sec_viewport_start,DCSURF_SEC_VIEWPORT_START); R!(sec_viewport_start_c,DCSURF_SEC_VIEWPORT_START_C); R!(surface_config,DCSURF_SURFACE_CONFIG); R!(tiling_config,DCSURF_TILING_CONFIG); R!(clk_cntl,HUBP_CLK_CNTL); R!(mall_status,HUBP_MALL_STATUS); R!(measure_win_ctrl_dcfclk,HUBP_MEASURE_WIN_CTRL_DCFCLK); R!(measure_win_ctrl_dppclk,HUBP_MEASURE_WIN_CTRL_DPPCLK);
    R!(blank_offset_0,BLANK_OFFSET_0); R!(blank_offset_1,BLANK_OFFSET_1); R!(cursor_settings,CURSOR_SETTINGS); R!(dcn_cur0_ttu_cntl0,DCN_CUR0_TTU_CNTL0); R!(dcn_cur0_ttu_cntl1,DCN_CUR0_TTU_CNTL1); R!(dcn_cur1_ttu_cntl0,DCN_CUR1_TTU_CNTL0); R!(dcn_cur1_ttu_cntl1,DCN_CUR1_TTU_CNTL1); R!(dcn_dmdat_vm_cntl,DCN_DMDATA_VM_CNTL); R!(dcn_expansion_mode,DCN_EXPANSION_MODE); R!(dcn_global_ttu_cntl,DCN_GLOBAL_TTU_CNTL); R!(dcn_surf0_ttu_cntl0,DCN_SURF0_TTU_CNTL0); R!(dcn_surf0_ttu_cntl1,DCN_SURF0_TTU_CNTL1); R!(dcn_surf1_ttu_cntl0,DCN_SURF1_TTU_CNTL0); R!(dcn_surf1_ttu_cntl1,DCN_SURF1_TTU_CNTL1); R!(dcn_ttu_qos_wm,DCN_TTU_QOS_WM); R!(dcn_vm_mx_l1_tlb_cntl,DCN_VM_MX_L1_TLB_CNTL); R!(dcn_vm_system_aperture_high_addr,DCN_VM_SYSTEM_APERTURE_HIGH_ADDR); R!(dcn_vm_system_aperture_low_addr,DCN_VM_SYSTEM_APERTURE_LOW_ADDR); R!(dcsurf_flip_control,DCSURF_FLIP_CONTROL); R!(dcsurf_flip_control2,DCSURF_FLIP_CONTROL2);
    R!(dcsurf_primary_meta_surface_address,DCSURF_PRIMARY_META_SURFACE_ADDRESS); R!(dcsurf_primary_meta_surface_address_c,DCSURF_PRIMARY_META_SURFACE_ADDRESS_C); R!(dcsurf_primary_meta_surface_address_high,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH); R!(dcsurf_primary_meta_surface_address_high_c,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C); R!(dcsurf_primary_surface_address,DCSURF_PRIMARY_SURFACE_ADDRESS); R!(dcsurf_primary_surface_address_c,DCSURF_PRIMARY_SURFACE_ADDRESS_C); R!(dcsurf_primary_surface_address_high,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH); R!(dcsurf_primary_surface_address_high_c,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C); R!(dcsurf_secondary_meta_surface_address,DCSURF_SECONDARY_META_SURFACE_ADDRESS); R!(dcsurf_secondary_meta_surface_address_c,DCSURF_SECONDARY_META_SURFACE_ADDRESS_C); R!(dcsurf_secondary_meta_surface_address_high,DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH); R!(dcsurf_secondary_meta_surface_address_high_c,DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH_C); R!(dcsurf_secondary_surface_address,DCSURF_SECONDARY_SURFACE_ADDRESS); R!(dcsurf_secondary_surface_address_c,DCSURF_SECONDARY_SURFACE_ADDRESS_C); R!(dcsurf_secondary_surface_address_high,DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH); R!(dcsurf_secondary_surface_address_high_c,DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH_C); R!(dcsurf_surface_control,DCSURF_SURFACE_CONTROL);
    R!(dcsurf_surface_earliest_inuse,DCSURF_SURFACE_EARLIEST_INUSE); R!(dcsurf_surface_earliest_inuse_c,DCSURF_SURFACE_EARLIEST_INUSE_C); R!(dcsurf_surface_earliest_inuse_high,DCSURF_SURFACE_EARLIEST_INUSE_HIGH); R!(dcsurf_surface_earliest_inuse_high_c,DCSURF_SURFACE_EARLIEST_INUSE_HIGH_C); R!(dcsurf_surface_flip_interrupt,DCSURF_SURFACE_FLIP_INTERRUPT); R!(dcsurf_surface_inuse,DCSURF_SURFACE_INUSE); R!(dcsurf_surface_inuse_c,DCSURF_SURFACE_INUSE_C); R!(dcsurf_surface_inuse_high,DCSURF_SURFACE_INUSE_HIGH); R!(dcsurf_surface_inuse_high_c,DCSURF_SURFACE_INUSE_HIGH_C); R!(dcsurf_surface_pitch,DCSURF_SURFACE_PITCH); R!(dcsurf_surface_pitch_c,DCSURF_SURFACE_PITCH_C); R!(dst_after_scaler,DST_AFTER_SCALER); R!(dst_dimensions,DST_DIMENSIONS); R!(dst_y_delta_drq_limit,DST_Y_DELTA_DRQ_LIMIT);
    R!(flip_parameters_0,FLIP_PARAMETERS_0); R!(flip_parameters_1,FLIP_PARAMETERS_1); R!(flip_parameters_2,FLIP_PARAMETERS_2); R!(flip_parameters_3,FLIP_PARAMETERS_3); R!(flip_parameters_4,FLIP_PARAMETERS_4); R!(flip_parameters_5,FLIP_PARAMETERS_5); R!(flip_parameters_6,FLIP_PARAMETERS_6); R!(hubpreq_mem_pwr_ctrl,HUBPREQ_MEM_PWR_CTRL); R!(hubpreq_mem_pwr_status,HUBPREQ_MEM_PWR_STATUS); R!(nom_parameters_0,NOM_PARAMETERS_0); R!(nom_parameters_1,NOM_PARAMETERS_1); R!(nom_parameters_2,NOM_PARAMETERS_2); R!(nom_parameters_3,NOM_PARAMETERS_3); R!(nom_parameters_4,NOM_PARAMETERS_4); R!(nom_parameters_5,NOM_PARAMETERS_5); R!(nom_parameters_6,NOM_PARAMETERS_6); R!(nom_parameters_7,NOM_PARAMETERS_7); R!(per_line_delivery,PER_LINE_DELIVERY); R!(per_line_delivery_pre,PER_LINE_DELIVERY_PRE); R!(prefetch_settings,PREFETCH_SETTINGS); R!(prefetch_settings_c,PREFETCH_SETTINGS_C); R!(ref_freq_to_pix_freq,REF_FREQ_TO_PIX_FREQ); R!(uclk_pstate_force,UCLK_PSTATE_FORCE);
    R!(vblank_parameters_0,VBLANK_PARAMETERS_0); R!(vblank_parameters_1,VBLANK_PARAMETERS_1); R!(vblank_parameters_2,VBLANK_PARAMETERS_2); R!(vblank_parameters_3,VBLANK_PARAMETERS_3); R!(vblank_parameters_4,VBLANK_PARAMETERS_4); R!(vblank_parameters_5,VBLANK_PARAMETERS_5); R!(vblank_parameters_6,VBLANK_PARAMETERS_6); R!(vmid_settings_0,VMID_SETTINGS_0); R!(hubpret_control,HUBPRET_CONTROL); R!(hubpret_interrupt,HUBPRET_INTERRUPT); R!(hubpret_mem_pwr_ctrl,HUBPRET_MEM_PWR_CTRL); R!(hubpret_mem_pwr_status,HUBPRET_MEM_PWR_STATUS); R!(hubpret_read_line_ctrl0,HUBPRET_READ_LINE_CTRL0); R!(hubpret_read_line_ctrl1,HUBPRET_READ_LINE_CTRL1); R!(hubpret_read_line_status,HUBPRET_READ_LINE_STATUS); R!(hubpret_read_line_value,HUBPRET_READ_LINE_VALUE); R!(hubpret_read_line0,HUBPRET_READ_LINE0); R!(hubpret_read_line1,HUBPRET_READ_LINE1);
}

static mut dcn30_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer), hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled), hubp_program_surface_flip_and_addr: Some(hubp3_program_surface_flip_and_addr), hubp_program_surface_config: Some(hubp3_program_surface_config), hubp_is_flip_pending: Some(hubp2_is_flip_pending), hubp_setup: Some(hubp3_setup), hubp_setup_interdependent: Some(hubp2_setup_interdependent), hubp_set_vm_system_aperture_settings: Some(hubp3_set_vm_system_aperture_settings), set_blank: Some(hubp2_set_blank), set_blank_regs: Some(hubp2_set_blank_regs), dcc_control: Some(hubp3_dcc_control), hubp_reset: Some(hubp_reset), mem_program_viewport: Some(min_set_viewport), set_cursor_attributes: Some(hubp2_cursor_set_attributes), set_cursor_position: Some(hubp2_cursor_set_position), hubp_clk_cntl: Some(hubp2_clk_cntl), hubp_vtg_sel: Some(hubp2_vtg_sel), dmdata_set_attributes: Some(hubp3_dmdata_set_attributes), dmdata_load: Some(hubp2_dmdata_load), dmdata_status_done: Some(hubp2_dmdata_status_done), hubp_read_state: Some(hubp3_read_state), hubp_clear_underflow: Some(hubp2_clear_underflow), hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl), hubp_init: Some(hubp3_init), hubp_in_blank: Some(hubp1_in_blank), hubp_soft_reset: Some(hubp1_soft_reset), hubp_set_flip_int: Some(hubp1_set_flip_int), hubp_clear_tiling: Some(hubp3_clear_tiling), hubp_read_reg_state: Some(hubp3_read_reg_state)
};

pub unsafe fn hubp3_construct(hubp2: *mut dcn20_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_hubp2_registers, hubp_shift: *const dcn_hubp2_shift, hubp_mask: *const dcn_hubp2_mask) -> bool {
    (*hubp2).base.funcs = &dcn30_hubp_funcs; (*hubp2).base.ctx = ctx; (*hubp2).hubp_regs = hubp_regs; (*hubp2).hubp_shift = hubp_shift; (*hubp2).hubp_mask = hubp_mask; (*hubp2).base.inst = inst; (*hubp2).base.opp_id = OPP_ID_INVALID; (*hubp2).base.mpcc_id = 0xf; true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
