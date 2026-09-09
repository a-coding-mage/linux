/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
 */

// Dependencies corresponding to the original C includes:
// dcn20/dcn20_hubp.h
// dcn21/dcn21_hubp.h
// dcn30/dcn30_hubp.h
// dcn31/dcn31_hubp.h
// dcn32/dcn32_hubp.h
// dcn401/dcn401_hubp.h
// dml2_0/dml21/inc/dml_top_dchub_registers.h

#[macro_export]
macro_rules! HUBP_MASK_SH_LIST_DCN50 {
    ($mask_sh:ident) => {
        HUBP_MASK_SH_LIST_DCN401!($mask_sh);
    };
}

unsafe extern "C" {
    pub fn hubp50_program_surface_flip_and_addr(
        hubp: *mut hubp,
        address: *const dc_plane_address,
        flip_immediate: bool,
    ) -> bool;

    pub fn hubp50_program_surface_config(
        hubp: *mut hubp,
        format: surface_pixel_format,
        tiling_info: *mut dc_tiling_info,
        plane_size: *mut plane_size,
        rotation: dc_rotation_angle,
        dcc: *mut dc_plane_dcc_param,
        horizontal_mirror: bool,
        compat_level: ::core::ffi::c_uint,
    );

    pub fn hubp50_read_state(hubp: *mut hubp);

    pub fn hubp50_construct(
        hubp2: *mut dcn20_hubp,
        ctx: *mut dc_context,
        inst: u32,
        hubp_regs: *const dcn_hubp2_registers,
        hubp_shift: *const dcn_hubp2_shift,
        hubp_mask: *const dcn_hubp2_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
