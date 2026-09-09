/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
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

// Dependency: dcn35/dcn35_hubp.h

macro_rules! HUBP_MASK_SH_LIST_DCN42 {
    ($mask_sh:expr) => {
        HUBP_MASK_SH_LIST_DCN35!($mask_sh),
        HUBP_SF!(HUBP0_3DLUT_FL_CONFIG, HUBP0_3DLUT_FL_MODE, $mask_sh),
        HUBP_SF!(HUBP0_3DLUT_FL_CONFIG, HUBP0_3DLUT_FL_FORMAT, $mask_sh),
        HUBP_SF!(HUBP0_3DLUT_FL_BIAS_SCALE, HUBP0_3DLUT_FL_BIAS, $mask_sh),
        HUBP_SF!(HUBP0_3DLUT_FL_BIAS_SCALE, HUBP0_3DLUT_FL_SCALE, $mask_sh),
        HUBP_SF!(HUBPREQ0_DST_Y_DELTA_DRQ_LIMIT, DST_Y_DELTA_DRQ_LIMIT, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_ENABLE, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_DONE, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_ADDRESSING_MODE, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_WIDTH, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_MPC_WIDTH, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_TMZ, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_CROSSBAR_SEL_B, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_CROSSBAR_SEL_G, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_CONTROL, HUBP_3DLUT_CROSSBAR_SEL_R, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_ADDRESS_HIGH, HUBP_3DLUT_ADDRESS_HIGH, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_ADDRESS_LOW, HUBP_3DLUT_ADDRESS_LOW, $mask_sh),
        HUBP_SF!(CURSOR0_0_HUBP_3DLUT_DLG_PARAM, REFCYC_PER_3DLUT_GROUP, $mask_sh)
    };
}

macro_rules! HUBP_MASK_SH_LIST_DCN42B {
    ($mask_sh:expr) => {
        HUBP_MASK_SH_LIST_DCN42!($mask_sh)
    };
}

#[repr(C)]
pub struct dml2_display_rq_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn hubp42_construct(
        hubp2: *mut dcn20_hubp,
        ctx: *mut dc_context,
        inst: u32,
        hubp_regs: *const dcn_hubp2_registers,
        hubp_shift: *const dcn_hubp2_shift,
        hubp_mask: *const dcn_hubp2_mask,
    ) -> bool;

    pub fn hubp42_program_3dlut_fl_crossbar(
        hubp: *mut hubp,
        format: dc_cm_lut_pixel_format,
    );
    pub fn hubp42_program_3dlut_fl_config(hubp: *mut hubp, config: *const dc_3dlut_dma);
    pub fn hubp42_read_state(hubp: *mut hubp);
    pub fn hubp42_program_requestor(hubp: *mut hubp, rq_regs: *mut dml2_display_rq_regs);
    pub fn hubp42_setup(
        hubp: *mut hubp,
        pipe_regs: *mut dml2_dchub_per_pipe_register_set,
        pipe_global_sync: *mut dml2_global_sync_programming,
        timing: *mut dc_crtc_timing,
    );
    pub fn hubp42_setup_interdependent(
        hubp: *mut hubp,
        pipe_regs: *mut dml2_dchub_per_pipe_register_set,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
