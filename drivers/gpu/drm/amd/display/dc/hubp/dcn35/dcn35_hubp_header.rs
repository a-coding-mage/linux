/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by dcn31/dcn31_hubp.h and dcn32/dcn32_hubp.h.

/*
 * C macro translation.  HUBP_MASK_SH_LIST_DCN32 and HUBP_SF are supplied by
 * the dependent headers.
 */
#[macro_export]
macro_rules! HUBP_MASK_SH_LIST_DCN35 {
    ($mask_sh:ident) => {
        HUBP_MASK_SH_LIST_DCN32!($mask_sh),
        HUBP_SF!(HUBP0_HUBP_CLK_CNTL, HUBP_FGCG_REP_DIS, $mask_sh)
    };
}

/*
 * DCN32_HUBP_REG_FIELD_VARIABLE_LIST(type) is a C field-list macro from the
 * dependent header.  Its expanded fields precede HUBP_FGCG_REP_DIS here.
 */
#[repr(C)]
pub struct dcn35_hubp2_shift {
    // DCN32_HUBP_REG_FIELD_VARIABLE_LIST(u8) expanded here.
    pub HUBP_FGCG_REP_DIS: u8,
}

#[repr(C)]
pub struct dcn35_hubp2_mask {
    // DCN32_HUBP_REG_FIELD_VARIABLE_LIST(u32) expanded here.
    pub HUBP_FGCG_REP_DIS: u32,
}

extern "C" {
    pub fn hubp35_construct(
        hubp2: *mut dcn20_hubp,
        ctx: *mut dc_context,
        inst: u32,
        hubp_regs: *const dcn_hubp2_registers,
        hubp_shift: *const dcn35_hubp2_shift,
        hubp_mask: *const dcn35_hubp2_mask,
    ) -> bool;

    pub fn hubp35_set_fgcg(hubp: *mut hubp, enable: bool);

    pub fn hubp35_program_pixel_format(
        hubp: *mut hubp,
        format: surface_pixel_format,
    );

    pub fn hubp35_program_surface_config(
        hubp: *mut hubp,
        format: surface_pixel_format,
        tiling_info: *mut dc_tiling_info,
        plane_size: *mut plane_size,
        rotation: dc_rotation_angle,
        dcc: *mut dc_plane_dcc_param,
        horizontal_mirror: bool,
        compat_level: ::core::ffi::c_uint,
    );

    pub fn hubp35_init(hubp: *mut hubp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
