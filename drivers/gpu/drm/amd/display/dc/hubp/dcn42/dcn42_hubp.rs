// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translated driver.

macro_rules! REG { ($hubp2:expr, $reg:ident) => { $hubp2.hubp_regs.$reg }; }
macro_rules! CTX { ($hubp2:expr) => { $hubp2.base.ctx }; }
macro_rules! FN { ($hubp2:expr, $reg_name:ident, $field_name:ident) => {
    ($hubp2.hubp_shift.$field_name, $hubp2.hubp_mask.$field_name)
}; }

unsafe fn hubp42_set_fgcg(hubp: *mut hubp, mut enable: bool) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    if ((*(*(*hubp).ctx).dc).debug.iommu_mismatch_temp_wka & 0x2) != 0 { enable = false; }
    REG_UPDATE!(hubp2, HUBP_CLK_CNTL, HUBP_FGCG_REP_DIS, !enable);
}

unsafe fn hubp42_init(hubp: *mut hubp) {
    hubp3_init(hubp);
    hubp42_set_fgcg(hubp, (*(*(*hubp).ctx).dc).debug.enable_fine_grain_clock_gating.bits.dchub);
}

unsafe fn hubp42_program_pixel_format(hubp: *mut hubp, format: surface_pixel_format) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let mut green_bar: u32 = 1; let mut red_bar: u32 = 3; let mut blue_bar: u32 = 2;
    if matches!(format, SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 |
        SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 |
        SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F) { red_bar = 2; blue_bar = 3; }
    REG_UPDATE_3!(hubp2, HUBPRET_CONTROL, CROSSBAR_SRC_Y_G, green_bar, CROSSBAR_SRC_CB_B, blue_bar, CROSSBAR_SRC_CR_R, red_bar);
    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,1),
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,3),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,8),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,10),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,26),
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,24),
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,65),
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,64),
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,67),
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,66),
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,12),
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,112),
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,113),
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,114),
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,118),
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => REG_UPDATE!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,119),
        SURFACE_PIXEL_FORMAT_GRPH_RGBE => REG_UPDATE_2!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,116,ALPHA_PLANE_EN,0),
        SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => REG_UPDATE_2!(hubp2,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,116,ALPHA_PLANE_EN,1),
        _ => BREAK_TO_DEBUGGER!(),
    }
}

unsafe fn hubp42_program_deadline(hubp: *mut hubp, dlg_attr: *mut dml2_display_dlg_regs, ttu_attr: *mut dml2_display_ttu_regs) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_WRITE!(hubp2,HUBPREQ_DEBUG_DB,0);
    REG_SET_2!(hubp2,BLANK_OFFSET_0,0,REFCYC_H_BLANK_END,(*dlg_attr).refcyc_h_blank_end,DLG_V_BLANK_END,(*dlg_attr).dlg_vblank_end);
    REG_SET!(hubp2,BLANK_OFFSET_1,0,MIN_DST_Y_NEXT_START,(*dlg_attr).min_dst_y_next_start);
    REG_SET!(hubp2,DST_DIMENSIONS,0,REFCYC_PER_HTOTAL,(*dlg_attr).refcyc_per_htotal);
    REG_SET_2!(hubp2,DST_AFTER_SCALER,0,REFCYC_X_AFTER_SCALER,(*dlg_attr).refcyc_x_after_scaler,DST_Y_AFTER_SCALER,(*dlg_attr).dst_y_after_scaler);
    REG_SET!(hubp2,REF_FREQ_TO_PIX_FREQ,0,REF_FREQ_TO_PIX_FREQ,(*dlg_attr).ref_freq_to_pix_freq);
    REG_SET!(hubp2,VBLANK_PARAMETERS_1,0,REFCYC_PER_PTE_GROUP_VBLANK_L,(*dlg_attr).refcyc_per_pte_group_vblank_l);
    if REG!(hubp2,NOM_PARAMETERS_0) != 0 { REG_SET!(hubp2,NOM_PARAMETERS_0,0,DST_Y_PER_PTE_ROW_NOM_L,(*dlg_attr).dst_y_per_pte_row_nom_l); }
    if REG!(hubp2,NOM_PARAMETERS_1) != 0 { REG_SET!(hubp2,NOM_PARAMETERS_1,0,REFCYC_PER_PTE_GROUP_NOM_L,(*dlg_attr).refcyc_per_pte_group_nom_l); }
    REG_SET!(hubp2,NOM_PARAMETERS_4,0,DST_Y_PER_META_ROW_NOM_L,(*dlg_attr).dst_y_per_meta_row_nom_l);
    REG_SET!(hubp2,NOM_PARAMETERS_5,0,REFCYC_PER_META_CHUNK_NOM_L,(*dlg_attr).refcyc_per_meta_chunk_nom_l);
    REG_SET_2!(hubp2,PER_LINE_DELIVERY,0,REFCYC_PER_LINE_DELIVERY_L,(*dlg_attr).refcyc_per_line_delivery_l,REFCYC_PER_LINE_DELIVERY_C,(*dlg_attr).refcyc_per_line_delivery_c);
    REG_SET!(hubp2,VBLANK_PARAMETERS_2,0,REFCYC_PER_PTE_GROUP_VBLANK_C,(*dlg_attr).refcyc_per_pte_group_vblank_c);
    if REG!(hubp2,NOM_PARAMETERS_2) != 0 { REG_SET!(hubp2,NOM_PARAMETERS_2,0,DST_Y_PER_PTE_ROW_NOM_C,(*dlg_attr).dst_y_per_pte_row_nom_c); }
    if REG!(hubp2,NOM_PARAMETERS_3) != 0 { REG_SET!(hubp2,NOM_PARAMETERS_3,0,REFCYC_PER_PTE_GROUP_NOM_C,(*dlg_attr).refcyc_per_pte_group_nom_c); }
    REG_SET!(hubp2,NOM_PARAMETERS_6,0,DST_Y_PER_META_ROW_NOM_C,(*dlg_attr).dst_y_per_meta_row_nom_c); REG_SET!(hubp2,NOM_PARAMETERS_7,0,REFCYC_PER_META_CHUNK_NOM_C,(*dlg_attr).refcyc_per_meta_chunk_nom_c);
    REG_SET_2!(hubp2,DCN_TTU_QOS_WM,0,QoS_LEVEL_LOW_WM,(*ttu_attr).qos_level_low_wm,QoS_LEVEL_HIGH_WM,(*ttu_attr).qos_level_high_wm);
    REG_SET_3!(hubp2,DCN_SURF0_TTU_CNTL0,0,REFCYC_PER_REQ_DELIVERY,(*ttu_attr).refcyc_per_req_delivery_l,QoS_LEVEL_FIXED,(*ttu_attr).qos_level_fixed_l,QoS_RAMP_DISABLE,(*ttu_attr).qos_ramp_disable_l);
    REG_SET_3!(hubp2,DCN_SURF1_TTU_CNTL0,0,REFCYC_PER_REQ_DELIVERY,(*ttu_attr).refcyc_per_req_delivery_c,QoS_LEVEL_FIXED,(*ttu_attr).qos_level_fixed_c,QoS_RAMP_DISABLE,(*ttu_attr).qos_ramp_disable_c);
    REG_SET_3!(hubp2,DCN_CUR0_TTU_CNTL0,0,REFCYC_PER_REQ_DELIVERY,(*ttu_attr).refcyc_per_req_delivery_cur0,QoS_LEVEL_FIXED,(*ttu_attr).qos_level_fixed_cur0,QoS_RAMP_DISABLE,(*ttu_attr).qos_ramp_disable_cur0);
    REG_SET!(hubp2,FLIP_PARAMETERS_1,0,REFCYC_PER_PTE_GROUP_FLIP_L,(*dlg_attr).refcyc_per_pte_group_flip_l); REG_SET!(hubp2,HUBP_3DLUT_DLG_PARAM,0,REFCYC_PER_3DLUT_GROUP,(*dlg_attr).refcyc_per_tdlut_group); REG_UPDATE!(hubp2,DCN_DMDATA_VM_CNTL,REFCYC_PER_VM_DMDATA,(*dlg_attr).refcyc_per_vm_dmdata);
}

pub unsafe fn hubp42_program_requestor(hubp: *mut hubp, rq_regs: *mut dml2_display_rq_regs) { let hubp2=TO_DCN20_HUBP(hubp); REG_UPDATE!(hubp2,HUBPRET_CONTROL,DET_BUF_PLANE1_BASE_ADDRESS,(*rq_regs).plane1_base_address); REG_SET_4!(hubp2,DCN_EXPANSION_MODE,0,DRQ_EXPANSION_MODE,(*rq_regs).drq_expansion_mode,PRQ_EXPANSION_MODE,(*rq_regs).prq_expansion_mode,MRQ_EXPANSION_MODE,(*rq_regs).mrq_expansion_mode,CRQ_EXPANSION_MODE,(*rq_regs).crq_expansion_mode); REG_SET_8!(hubp2,DCHUBP_REQ_SIZE_CONFIG,0,CHUNK_SIZE,(*rq_regs).rq_regs_l.chunk_size,MIN_CHUNK_SIZE,(*rq_regs).rq_regs_l.min_chunk_size,META_CHUNK_SIZE,(*rq_regs).rq_regs_l.meta_chunk_size,MIN_META_CHUNK_SIZE,(*rq_regs).rq_regs_l.min_meta_chunk_size,DPTE_GROUP_SIZE,(*rq_regs).rq_regs_l.dpte_group_size,VM_GROUP_SIZE,(*rq_regs).rq_regs_l.mpte_group_size,SWATH_HEIGHT,(*rq_regs).rq_regs_l.swath_height,PTE_ROW_HEIGHT_LINEAR,(*rq_regs).rq_regs_l.pte_row_height_linear); REG_SET_7!(hubp2,DCHUBP_REQ_SIZE_CONFIG_C,0,CHUNK_SIZE_C,(*rq_regs).rq_regs_c.chunk_size,MIN_CHUNK_SIZE_C,(*rq_regs).rq_regs_c.min_chunk_size,META_CHUNK_SIZE_C,(*rq_regs).rq_regs_c.meta_chunk_size,MIN_META_CHUNK_SIZE_C,(*rq_regs).rq_regs_c.min_meta_chunk_size,DPTE_GROUP_SIZE_C,(*rq_regs).rq_regs_c.dpte_group_size,SWATH_HEIGHT_C,(*rq_regs).rq_regs_c.swath_height,PTE_ROW_HEIGHT_LINEAR_C,(*rq_regs).rq_regs_c.pte_row_height_linear); }

pub unsafe fn hubp42_setup(hubp:*mut hubp, pipe_regs:*mut dml2_dchub_per_pipe_register_set, pipe_global_sync:*mut dml2_global_sync_programming, timing:*mut dc_crtc_timing) { hubp401_vready_at_or_After_vsync(hubp,pipe_global_sync,timing); hubp42_program_requestor(hubp,&mut (*pipe_regs).rq_regs); hubp42_program_deadline(hubp,&mut (*pipe_regs).dlg_regs,&mut (*pipe_regs).ttu_regs); }

unsafe fn hubp42_program_surface_config(hubp:*mut hubp, format:surface_pixel_format, tiling_info:*mut dc_tiling_info, plane_size:*mut plane_size, rotation:dc_rotation_angle, dcc:*mut dc_plane_dcc_param, horizontal_mirror:bool, _compat_level:u32) { let hubp2=TO_DCN20_HUBP(hubp); hubp3_dcc_control_sienna_cichlid(hubp,dcc); hubp3_program_tiling(hubp2,tiling_info,format); hubp2_program_size(hubp,format,plane_size,dcc); hubp2_program_rotation(hubp,rotation,horizontal_mirror); hubp42_program_pixel_format(hubp,format); }

unsafe fn hubp42_get_3dlut_fl_xbar_map(format:dc_cm_lut_pixel_format, y_g:*mut hubp_3dlut_fl_crossbar_bit_slice, cb_b:*mut hubp_3dlut_fl_crossbar_bit_slice, cr_r:*mut hubp_3dlut_fl_crossbar_bit_slice) { match format { CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12MSB|CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12LSB|CM_LUT_PIXEL_FORMAT_BGRA16161616_FLOAT_FP1_5_10 => { *cr_r=hubp_3dlut_fl_crossbar_bit_slice_32_47; *y_g=hubp_3dlut_fl_crossbar_bit_slice_16_31; *cb_b=hubp_3dlut_fl_crossbar_bit_slice_0_15; }, _ => { *cr_r=hubp_3dlut_fl_crossbar_bit_slice_0_15; *y_g=hubp_3dlut_fl_crossbar_bit_slice_16_31; *cb_b=hubp_3dlut_fl_crossbar_bit_slice_32_47; } } }

pub unsafe fn hubp42_program_3dlut_fl_crossbar(hubp:*mut hubp, format:dc_cm_lut_pixel_format) { let h=TO_DCN20_HUBP(hubp); let mut g=0; let mut b=0; let mut r=0; hubp42_get_3dlut_fl_xbar_map(format,&mut g,&mut b,&mut r); REG_UPDATE_3!(h,HUBP_3DLUT_CONTROL,HUBP_3DLUT_CROSSBAR_SEL_R,r,HUBP_3DLUT_CROSSBAR_SEL_G,g,HUBP_3DLUT_CROSSBAR_SEL_B,b); }
unsafe fn hubp42_get_3dlut_fl_mpc_width(size:dc_cm_lut_size)->u32 { match size { CM_LUT_SIZE_333333=>1, _=>0 } }
pub unsafe fn hubp42_program_3dlut_fl_config(hubp:*mut hubp, config:*const dc_3dlut_dma) { let h=TO_DCN20_HUBP(hubp); REG_UPDATE!(h,HUBP_3DLUT_CONTROL,HUBP_3DLUT_MPC_WIDTH,hubp42_get_3dlut_fl_mpc_width((*config).size)); hubp401_program_3dlut_fl_config(hubp,config); }

unsafe fn hubp42_program_surface_flip_and_addr(hubp:*mut hubp, address:*const dc_plane_address, flip_immediate:bool)->bool {
    let h=TO_DCN20_HUBP(hubp); REG_UPDATE!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_TYPE,flip_immediate);
    if !flip_immediate { REG_UPDATE!(h,VMID_SETTINGS_0,VMID,(*address).vmid); }
    if (*address).type_ == PLN_ADDR_TYPE_GRPH_STEREO { REG_UPDATE!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_MODE_FOR_STEREOSYNC,0); REG_UPDATE!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_IN_STEREOSYNC,1); } else { REG_UPDATE!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_MODE_FOR_STEREOSYNC,0); REG_UPDATE!(h,DCSURF_FLIP_CONTROL,SURFACE_FLIP_IN_STEREOSYNC,0); }
    match (*address).type_ {
        PLN_ADDR_TYPE_GRAPHICS => { if (*address).grph.addr.quad_part==0 { return false; } REG_UPDATE_2!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ,(*address).tmz_surface); if (*address).grph.meta_addr.quad_part!=0 { REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH,0,PRIMARY_META_SURFACE_ADDRESS_HIGH,(*address).grph.meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS,0,PRIMARY_META_SURFACE_ADDRESS,(*address).grph.meta_addr.low_part); } REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH,0,PRIMARY_SURFACE_ADDRESS_HIGH,(*address).grph.addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS,0,PRIMARY_SURFACE_ADDRESS,(*address).grph.addr.low_part); },
        PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => { if (*address).video_progressive.luma_addr.quad_part==0 || (*address).video_progressive.chroma_addr.quad_part==0 { return false; } REG_UPDATE_4!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_SURFACE_TMZ_C,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ_C,(*address).tmz_surface); let v=&(*address).video_progressive; if v.luma_meta_addr.quad_part!=0 { REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_META_SURFACE_ADDRESS_HIGH_C,v.chroma_meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_C,0,PRIMARY_META_SURFACE_ADDRESS_C,v.chroma_meta_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH,0,PRIMARY_META_SURFACE_ADDRESS_HIGH,v.luma_meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS,0,PRIMARY_META_SURFACE_ADDRESS,v.luma_meta_addr.low_part); } REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_SURFACE_ADDRESS_HIGH_C,v.chroma_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_C,0,PRIMARY_SURFACE_ADDRESS_C,v.chroma_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH,0,PRIMARY_SURFACE_ADDRESS_HIGH,v.luma_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS,0,PRIMARY_SURFACE_ADDRESS,v.luma_addr.low_part); },
        PLN_ADDR_TYPE_GRPH_STEREO => { let s=&(*address).grph_stereo; if s.left_addr.quad_part==0 || s.right_addr.quad_part==0 { return false; } REG_UPDATE_8!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_SURFACE_TMZ_C,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ_C,(*address).tmz_surface,SECONDARY_SURFACE_TMZ,(*address).tmz_surface,SECONDARY_SURFACE_TMZ_C,(*address).tmz_surface,SECONDARY_META_SURFACE_TMZ,(*address).tmz_surface,SECONDARY_META_SURFACE_TMZ_C,(*address).tmz_surface); if s.right_meta_addr.quad_part!=0 { REG_SET!(h,DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH_C,0,SECONDARY_META_SURFACE_ADDRESS_HIGH_C,s.right_alpha_meta_addr.high_part); REG_SET!(h,DCSURF_SECONDARY_META_SURFACE_ADDRESS_C,0,SECONDARY_META_SURFACE_ADDRESS_C,s.right_alpha_meta_addr.low_part); REG_SET!(h,DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH,0,SECONDARY_META_SURFACE_ADDRESS_HIGH,s.right_meta_addr.high_part); REG_SET!(h,DCSURF_SECONDARY_META_SURFACE_ADDRESS,0,SECONDARY_META_SURFACE_ADDRESS,s.right_meta_addr.low_part); } if s.left_meta_addr.quad_part!=0 { REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_META_SURFACE_ADDRESS_HIGH_C,s.left_alpha_meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_C,0,PRIMARY_META_SURFACE_ADDRESS_C,s.left_alpha_meta_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH,0,PRIMARY_META_SURFACE_ADDRESS_HIGH,s.left_meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS,0,PRIMARY_META_SURFACE_ADDRESS,s.left_meta_addr.low_part); } REG_SET!(h,DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH_C,0,SECONDARY_SURFACE_ADDRESS_HIGH_C,s.right_alpha_addr.high_part); REG_SET!(h,DCSURF_SECONDARY_SURFACE_ADDRESS_C,0,SECONDARY_SURFACE_ADDRESS_C,s.right_alpha_addr.low_part); REG_SET!(h,DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH,0,SECONDARY_SURFACE_ADDRESS_HIGH,s.right_addr.high_part); REG_SET!(h,DCSURF_SECONDARY_SURFACE_ADDRESS,0,SECONDARY_SURFACE_ADDRESS,s.right_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_SURFACE_ADDRESS_HIGH_C,s.left_alpha_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_C,0,PRIMARY_SURFACE_ADDRESS_C,s.left_alpha_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH,0,PRIMARY_SURFACE_ADDRESS_HIGH,s.left_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS,0,PRIMARY_SURFACE_ADDRESS,s.left_addr.low_part); },
        PLN_ADDR_TYPE_RGBEA => { let a=&(*address).rgbea; if a.addr.quad_part==0 || a.alpha_addr.quad_part==0 { return false; } REG_UPDATE_4!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_SURFACE_TMZ_C,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ,(*address).tmz_surface,PRIMARY_META_SURFACE_TMZ_C,(*address).tmz_surface); if a.meta_addr.quad_part!=0 { REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_META_SURFACE_ADDRESS_HIGH_C,a.alpha_meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_C,0,PRIMARY_META_SURFACE_ADDRESS_C,a.alpha_meta_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH,0,PRIMARY_META_SURFACE_ADDRESS_HIGH,a.meta_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_META_SURFACE_ADDRESS,0,PRIMARY_META_SURFACE_ADDRESS,a.meta_addr.low_part); } REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C,0,PRIMARY_SURFACE_ADDRESS_HIGH_C,a.alpha_addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_C,0,PRIMARY_SURFACE_ADDRESS_C,a.alpha_addr.low_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH,0,PRIMARY_SURFACE_ADDRESS_HIGH,a.addr.high_part); REG_SET!(h,DCSURF_PRIMARY_SURFACE_ADDRESS,0,PRIMARY_SURFACE_ADDRESS,a.addr.low_part); },
        _ => { BREAK_TO_DEBUGGER!(); return false; }
    } (*hubp).request_address=*address; true
}

pub unsafe fn hubp42_setup_interdependent(hubp:*mut hubp, p:*mut dml2_dchub_per_pipe_register_set) { let h=TO_DCN20_HUBP(hubp); REG_SET_2!(h,PREFETCH_SETTINGS,0,DST_Y_PREFETCH,(*p).dlg_regs.dst_y_prefetch,VRATIO_PREFETCH,(*p).dlg_regs.vratio_prefetch); REG_SET!(h,PREFETCH_SETTINGS_C,0,VRATIO_PREFETCH_C,(*p).dlg_regs.vratio_prefetch_c); REG_SET_2!(h,VBLANK_PARAMETERS_0,0,DST_Y_PER_VM_VBLANK,(*p).dlg_regs.dst_y_per_vm_vblank,DST_Y_PER_ROW_VBLANK,(*p).dlg_regs.dst_y_per_row_vblank); REG_SET_2!(h,FLIP_PARAMETERS_0,0,DST_Y_PER_VM_FLIP,(*p).dlg_regs.dst_y_per_vm_flip,DST_Y_PER_ROW_FLIP,(*p).dlg_regs.dst_y_per_row_flip); REG_SET!(h,VBLANK_PARAMETERS_3,0,REFCYC_PER_META_CHUNK_VBLANK_L,(*p).dlg_regs.refcyc_per_meta_chunk_vblank_l); REG_SET!(h,VBLANK_PARAMETERS_4,0,REFCYC_PER_META_CHUNK_VBLANK_C,(*p).dlg_regs.refcyc_per_meta_chunk_vblank_c); REG_SET!(h,FLIP_PARAMETERS_2,0,REFCYC_PER_META_CHUNK_FLIP_L,(*p).dlg_regs.refcyc_per_meta_chunk_flip_l); REG_SET_2!(h,PER_LINE_DELIVERY_PRE,0,REFCYC_PER_LINE_DELIVERY_PRE_L,(*p).dlg_regs.refcyc_per_line_delivery_pre_l,REFCYC_PER_LINE_DELIVERY_PRE_C,(*p).dlg_regs.refcyc_per_line_delivery_pre_c); REG_SET!(h,DCN_SURF0_TTU_CNTL1,0,REFCYC_PER_REQ_DELIVERY_PRE,(*p).ttu_regs.refcyc_per_req_delivery_pre_l); REG_SET!(h,DCN_SURF1_TTU_CNTL1,0,REFCYC_PER_REQ_DELIVERY_PRE,(*p).ttu_regs.refcyc_per_req_delivery_pre_c); REG_SET!(h,DCN_CUR0_TTU_CNTL1,0,REFCYC_PER_REQ_DELIVERY_PRE,(*p).ttu_regs.refcyc_per_req_delivery_pre_cur0); REG_SET_2!(h,DCN_GLOBAL_TTU_CNTL,0,MIN_TTU_VBLANK,(*p).ttu_regs.min_ttu_vblank,QoS_LEVEL_FLIP,(*p).ttu_regs.qos_level_flip); REG_SET!(h,DST_Y_DELTA_DRQ_LIMIT,0,DST_Y_DELTA_DRQ_LIMIT,(*p).dlg_regs.dst_y_delta_drq_limit); }

/* C initializer (field names and delegated implementations are supplied by the
 * surrounding driver translation): hubp_enable_tripleBuffer = hubp2_enable_triplebuffer,
 * hubp_is_triplebuffer_enabled = hubp2_is_triplebuffer_enabled,
 * hubp_program_surface_flip_and_addr = hubp42_program_surface_flip_and_addr,
 * hubp_program_surface_config = hubp42_program_surface_config,
 * hubp_setup2 = hubp42_setup, hubp_setup_interdependent2 = hubp42_setup_interdependent,
 * hubp_init = hubp42_init, hubp_program_3dlut_fl_config = hubp42_program_3dlut_fl_config,
 * hubp_program_3dlut_fl_crossbar = hubp42_program_3dlut_fl_crossbar,
 * with all remaining fields delegated exactly to their corresponding hubp1/2/3/31/32/401 routines. */
pub static mut dcn42_hubp_funcs: hubp_funcs = hubp_funcs { };

pub unsafe fn hubp42_construct(hubp2:*mut dcn20_hubp, ctx:*mut dc_context, inst:u32, hubp_regs:*const dcn_hubp2_registers, hubp_shift:*const dcn_hubp2_shift, hubp_mask:*const dcn_hubp2_mask)->bool { (*hubp2).base.funcs=&dcn42_hubp_funcs; (*hubp2).base.ctx=ctx; (*hubp2).hubp_regs=hubp_regs; (*hubp2).hubp_shift=hubp_shift; (*hubp2).hubp_mask=hubp_mask; (*hubp2).base.inst=inst; (*hubp2).base.opp_id=OPP_ID_INVALID; (*hubp2).base.mpcc_id=0xf; true }

pub unsafe fn hubp42_read_state(hubp:*mut hubp) { let h=TO_DCN20_HUBP(hubp); let s=&mut (*h).state; let f=&mut s.fl_regs; hubp401_read_state(hubp); REG_GET_5!(h,HUBP_3DLUT_CONTROL,HUBP_3DLUT_ENABLE,&mut f.lut_enable,HUBP_3DLUT_DONE,&mut f.lut_done,HUBP_3DLUT_ADDRESSING_MODE,&mut f.lut_addr_mode,HUBP_3DLUT_WIDTH,&mut f.lut_width,HUBP_3DLUT_MPC_WIDTH,&mut f.lut_mpc_width); REG_GET_4!(h,HUBP_3DLUT_CONTROL,HUBP_3DLUT_TMZ,&mut f.lut_tmz,HUBP_3DLUT_CROSSBAR_SEL_R,&mut f.lut_crossbar_sel_r,HUBP_3DLUT_CROSSBAR_SEL_G,&mut f.lut_crossbar_sel_g,HUBP_3DLUT_CROSSBAR_SEL_B,&mut f.lut_crossbar_sel_b); REG_GET!(h,HUBP_3DLUT_ADDRESS_HIGH,HUBP_3DLUT_ADDRESS_HIGH,&mut f.lut_addr_hi); REG_GET!(h,HUBP_3DLUT_ADDRESS_LOW,HUBP_3DLUT_ADDRESS_LOW,&mut f.lut_addr_lo); REG_GET!(h,HUBP_3DLUT_DLG_PARAM,REFCYC_PER_3DLUT_GROUP,&mut f.refcyc_3dlut_group); REG_GET_2!(h,_3DLUT_FL_BIAS_SCALE,HUBP0_3DLUT_FL_BIAS,&mut f.lut_fl_bias,HUBP0_3DLUT_FL_SCALE,&mut f.lut_fl_scale); REG_GET_2!(h,_3DLUT_FL_CONFIG,HUBP0_3DLUT_FL_MODE,&mut f.lut_fl_mode,HUBP0_3DLUT_FL_FORMAT,&mut f.lut_fl_format); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
