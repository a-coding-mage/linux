// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.
// Translated from dcn50_hubp.c. External types, functions, and register macros
// are supplied by the surrounding driver crate.

#[allow(unused_variables, dead_code)]
unsafe fn hubp50_program_surface_flip_and_addr(
    hubp: *mut hubp,
    address: *const dc_plane_address,
    flip_immediate: bool,
) -> bool {
    let hubp2 = TO_DCN20_HUBP(hubp);
    REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_TYPE, flip_immediate);
    if !flip_immediate { REG_UPDATE!(hubp2, VMID_SETTINGS_0, VMID, (*address).vmid); }
    if (*address).type_ == PLN_ADDR_TYPE_GRPH_STEREO {
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_MODE_FOR_STEREOSYNC, 0);
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_IN_STEREOSYNC, 1);
    } else {
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_MODE_FOR_STEREOSYNC, 0);
        REG_UPDATE!(hubp2, DCSURF_FLIP_CONTROL, SURFACE_FLIP_IN_STEREOSYNC, 0);
    }
    match (*address).type_ {
        PLN_ADDR_TYPE_GRAPHICS => {
            if (*address).grph.addr.quad_part != 0 {
                REG_UPDATE!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).grph.addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).grph.addr.low_part);
            }
        }
        PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => {
            if (*address).video_progressive.luma_addr.quad_part != 0 && (*address).video_progressive.chroma_addr.quad_part != 0 {
                REG_UPDATE_2!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, (*address).video_progressive.chroma_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, (*address).video_progressive.chroma_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, (*address).video_progressive.luma_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, (*address).video_progressive.luma_addr.low_part);
            }
        }
        PLN_ADDR_TYPE_GRPH_STEREO => {
            let a = &(*address).grph_stereo;
            if a.left_addr.quad_part != 0 && a.right_addr.quad_part != 0 {
                REG_UPDATE_4!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface, SECONDARY_SURFACE_TMZ, (*address).tmz_surface, SECONDARY_SURFACE_TMZ_C, (*address).tmz_surface);
                REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH_C, 0, SECONDARY_SURFACE_ADDRESS_HIGH_C, a.right_alpha_addr.high_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_C, 0, SECONDARY_SURFACE_ADDRESS_C, a.right_alpha_addr.low_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH, 0, SECONDARY_SURFACE_ADDRESS_HIGH, a.right_addr.high_part);
                REG_SET!(hubp2, DCSURF_SECONDARY_SURFACE_ADDRESS, 0, SECONDARY_SURFACE_ADDRESS, a.right_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, a.left_alpha_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, a.left_alpha_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, a.left_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, a.left_addr.low_part);
            }
        }
        PLN_ADDR_TYPE_RGBEA => {
            let a = &(*address).rgbea;
            if a.addr.quad_part != 0 && a.alpha_addr.quad_part != 0 {
                REG_UPDATE_2!(hubp2, DCSURF_SURFACE_CONTROL, PRIMARY_SURFACE_TMZ, (*address).tmz_surface, PRIMARY_SURFACE_TMZ_C, (*address).tmz_surface);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C, 0, PRIMARY_SURFACE_ADDRESS_HIGH_C, a.alpha_addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_C, 0, PRIMARY_SURFACE_ADDRESS_C, a.alpha_addr.low_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH, 0, PRIMARY_SURFACE_ADDRESS_HIGH, a.addr.high_part);
                REG_SET!(hubp2, DCSURF_PRIMARY_SURFACE_ADDRESS, 0, PRIMARY_SURFACE_ADDRESS, a.addr.low_part);
            }
        }
        _ => BREAK_TO_DEBUGGER!(),
    }
    (*hubp).request_address = *address;
    true
}

unsafe fn hubp50_addr3_to_swizzle_mode_mapping(v: swizzle_mode_addr3_values) -> swizzle_mode_values {
    match v {
        DC_ADDR3_SW_LINEAR => DC_SW_LINEAR,
        DC_ADDR3_SW_256B_2D => DC_SW_256_R,
        DC_ADDR3_SW_4KB_2D => DC_SW_4KB_R,
        DC_ADDR3_SW_64KB_2D | DC_ADDR3_SW_64KB_2D_Z => DC_SW_64KB_R,
        DC_ADDR3_SW_256KB_2D | DC_ADDR3_SW_256KB_2D_Z => DC_SW_VAR_R,
        DC_ADDR3_SW_4KB_3D | DC_ADDR3_SW_64KB_3D | DC_ADDR3_SW_256KB_3D => { BREAK_TO_DEBUGGER!(); DC_SW_UNKNOWN },
        _ => { BREAK_TO_DEBUGGER!(); DC_SW_UNKNOWN },
    }
}

unsafe fn hubp50_program_tiling(hubp2: *mut dcn20_hubp, info: *const dc_tiling_info, _pixel_format: surface_pixel_format) {
    let mut compat_level = 0;
    match (*info).gfxversion {
        DcGfxVersion7 | DcGfxVersion8 => {
            REG_UPDATE_8!(hubp2, DCSURF_LEGACY_ADDR_CONFIG, LEGACY_NUM_BANKS, (*info).gfx8.num_banks, BANK_WIDTH, (*info).gfx8.bank_width, BANK_HEIGHT, (*info).gfx8.bank_height, MACRO_TILE_ASPECT, (*info).gfx8.tile_aspect, TILE_SPLIT, (*info).gfx8.tile_split, MICRO_TILE_MODE_NEW, (*info).gfx8.tile_mode, PIPE_CONFIG, (*info).gfx8.pipe_config, ARRAY_MODE, (*info).gfx8.array_mode);
            if (*info).gfx8.bank_height == 1 { compat_level = 0; } else if (*info).gfx8.bank_height == 2 { compat_level = 1; }
        }
        DcGfxVersion9 | DcGfxVersion10 | DcGfxVersion11 => {
            REG_UPDATE_4!(hubp2, DCSURF_ADDR_CONFIG, NUM_PIPES, log_2((*info).gfx9.num_pipes), PIPE_INTERLEAVE, (*info).gfx9.pipe_interleave, MAX_COMPRESSED_FRAGS, log_2((*info).gfx9.max_compressed_frags), NUM_PKRS, log_2((*info).gfx9.num_pkrs));
            REG_UPDATE!(hubp2, DCSURF_TILING_CONFIG, SW_MODE, (*info).gfx9.swizzle); compat_level = 2;
        }
        DcGfxAddr3 => { REG_UPDATE!(hubp2, DCSURF_TILING_CONFIG, SW_MODE, hubp50_addr3_to_swizzle_mode_mapping((*info).gfx_addr3.swizzle)); compat_level = 5; }
        _ => {}
    }
    REG_UPDATE!(hubp2, DCSURF_TILING_CONFIG, COMPAT_LEVEL, compat_level);
}

unsafe fn hubp50_program_pixel_format(hubp: *mut hubp, format: surface_pixel_format) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let (red_bar, blue_bar) = match format { SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => (2, 3), _ => (3, 2) };
    REG_UPDATE_2!(hubp2, HUBPRET_CONTROL, CROSSBAR_SRC_CB_B, blue_bar, CROSSBAR_SRC_CR_R, red_bar);
    let value = match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => 1, SURFACE_PIXEL_FORMAT_GRPH_RGB565 => 3,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => 8,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS => 10,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => 26,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => 24,
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P208 => 65,
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb | SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P208 => 64,
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P210 => 67,
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb | SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P210 => 66,
        SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P212 => 68, SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P212 => 69,
        SURFACE_PIXEL_FORMAT_VIDEO_422_YCrYCb => 72, SURFACE_PIXEL_FORMAT_VIDEO_422_YCbYCr => 73, SURFACE_PIXEL_FORMAT_VIDEO_422_CrYCbY => 74, SURFACE_PIXEL_FORMAT_VIDEO_422_CbYCrY => 75,
        SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCrYCb => 76, SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCbYCr => 77, SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CrYCbY => 78, SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CbYCrY => 79,
        SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCbYCr => 80, SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCrYCb => 81, SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CrYCbY => 82, SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CbYCrY => 83,
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => 12, SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => 112, SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => 113, SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => 114,
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => 118, SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => 119,
        SURFACE_PIXEL_FORMAT_GRPH_RGBE | SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => 116,
        _ => { BREAK_TO_DEBUGGER!(); 0 },
    };
    REG_UPDATE!(hubp2, DCSURF_SURFACE_CONFIG, SURFACE_PIXEL_FORMAT, value);
    if format == SURFACE_PIXEL_FORMAT_GRPH_RGBE { REG_UPDATE!(hubp2, DCSURF_SURFACE_CONFIG, ALPHA_PLANE_EN, 0); }
    if format == SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA { REG_UPDATE!(hubp2, DCSURF_SURFACE_CONFIG, ALPHA_PLANE_EN, 1); }
}

unsafe fn hubp50_program_surface_config(hubp: *mut hubp, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, plane_size: *mut plane_size, rotation: dc_rotation_angle, dcc: *mut dc_plane_dcc_param, horizontal_mirror: bool, _compat_level: u32) {
    hubp401_dcc_control(hubp, dcc); hubp50_program_tiling(TO_DCN20_HUBP(hubp), tiling_info, format); hubp401_program_size(hubp, format, plane_size, dcc); hubp2_program_rotation(hubp, rotation, horizontal_mirror); hubp50_program_pixel_format(hubp, format);
}
unsafe fn hubp50_read_state(hubp: *mut hubp) { hubp401_read_state(hubp); }

static mut dcn50_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer), hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled), hubp_program_surface_flip_and_addr: Some(hubp50_program_surface_flip_and_addr), hubp_program_surface_config: Some(hubp50_program_surface_config), hubp_is_flip_pending: Some(hubp2_is_flip_pending), hubp_setup2: Some(hubp401_setup), hubp_setup_interdependent2: Some(hubp401_setup_interdependent), hubp_set_vm_system_aperture_settings: Some(hubp3_set_vm_system_aperture_settings), set_blank: Some(hubp2_set_blank), set_blank_regs: Some(hubp2_set_blank_regs), hubp_reset: Some(hubp_reset), mem_program_viewport: Some(hubp401_set_viewport), set_cursor_attributes: Some(hubp32_cursor_set_attributes), set_cursor_position: Some(hubp401_cursor_set_position), hubp_clk_cntl: Some(hubp2_clk_cntl), hubp_vtg_sel: Some(hubp2_vtg_sel), dmdata_set_attributes: Some(hubp3_dmdata_set_attributes), dmdata_load: Some(hubp2_dmdata_load), dmdata_status_done: Some(hubp2_dmdata_status_done), hubp_read_state: Some(hubp50_read_state), hubp_clear_underflow: Some(hubp2_clear_underflow), hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl), hubp_init: Some(hubp401_init), set_unbounded_requesting: Some(hubp401_set_unbounded_requesting), hubp_soft_reset: Some(hubp31_soft_reset), hubp_set_flip_int: Some(hubp401_set_flip_int), hubp_in_blank: Some(hubp401_in_blank), phantom_hubp_post_enable: Some(hubp32_phantom_hubp_post_enable), hubp_update_mall_sel: Some(hubp401_update_mall_sel), hubp_prepare_subvp_buffering: Some(hubp32_prepare_subvp_buffering), hubp_program_mcache_id_and_split_coordinate: Some(hubp401_program_mcache_id_and_split_coordinate), hubp_program_3dlut_fl_addr: Some(hubp401_program_3dlut_fl_addr), hubp_program_3dlut_fl_config: Some(hubp42_program_3dlut_fl_config), hubp_program_3dlut_fl_dlg_param: Some(hubp401_program_3dlut_fl_dlg_param), hubp_enable_3dlut_fl: Some(hubp401_enable_3dlut_fl), hubp_program_3dlut_fl_crossbar: Some(hubp42_program_3dlut_fl_crossbar), hubp_get_3dlut_fl_done: Some(hubp401_get_3dlut_fl_done), hubp_clear_tiling: Some(hubp401_clear_tiling), hubp_read_reg_state: Some(hubp3_read_reg_state),
};

unsafe fn hubp50_construct(hubp2: *mut dcn20_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_hubp2_registers, hubp_shift: *const dcn_hubp2_shift, hubp_mask: *const dcn_hubp2_mask) -> bool {
    (*hubp2).base.funcs = &raw mut dcn50_hubp_funcs; (*hubp2).base.ctx = ctx; (*hubp2).hubp_regs = hubp_regs; (*hubp2).hubp_shift = hubp_shift; (*hubp2).hubp_mask = hubp_mask; (*hubp2).base.inst = inst; (*hubp2).base.opp_id = OPP_ID_INVALID; (*hubp2).base.mpcc_id = 0xf; true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
