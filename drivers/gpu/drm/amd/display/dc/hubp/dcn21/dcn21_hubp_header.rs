/*
* Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies: ../dcn20/dcn20_hubp.h and ../dcn10/dcn10_hubp.h

macro_rules! TO_DCN21_HUBP { ($hubp:expr) => { container_of!($hubp, dcn21_hubp, base) }; }

macro_rules! HUBP_REG_LIST_DCN21 {
    ($id:expr) => {
        HUBP_REG_LIST_DCN2_COMMON!($id),
        SRI!(FLIP_PARAMETERS_3, HUBPREQ, $id),
        SRI!(FLIP_PARAMETERS_4, HUBPREQ, $id),
        SRI!(FLIP_PARAMETERS_5, HUBPREQ, $id),
        SRI!(FLIP_PARAMETERS_6, HUBPREQ, $id),
        SRI!(VBLANK_PARAMETERS_5, HUBPREQ, $id),
        SRI!(VBLANK_PARAMETERS_6, HUBPREQ, $id)
    };
}

macro_rules! HUBP_MASK_SH_LIST_DCN21_COMMON {
    ($mask_sh:expr) => {
        HUBP_MASK_SH_LIST_DCN_SHARE_COMMON!($mask_sh),
        HUBP_MASK_SH_LIST_DCN_VM!($mask_sh),
        HUBP_SF!(HUBP0_DCSURF_SURFACE_CONFIG, ROTATION_ANGLE, $mask_sh), HUBP_SF!(HUBP0_DCSURF_SURFACE_CONFIG, H_MIRROR_EN, $mask_sh),
        HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, DST_Y_PREFETCH, $mask_sh), HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, VRATIO_PREFETCH, $mask_sh), HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS_C, VRATIO_PREFETCH_C, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_VM_SYSTEM_APERTURE_LOW_ADDR, MC_VM_SYSTEM_APERTURE_LOW_ADDR, $mask_sh), HUBP_SF!(HUBPREQ0_DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, MC_VM_SYSTEM_APERTURE_HIGH_ADDR, $mask_sh),
        HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_DST_Y_OFFSET, $mask_sh), HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_CHUNK_HDL_ADJUST, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SURFACE_ADDRESS_HIGH, CURSOR_SURFACE_ADDRESS_HIGH, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_SURFACE_ADDRESS, CURSOR_SURFACE_ADDRESS, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SIZE, CURSOR_WIDTH, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_SIZE, CURSOR_HEIGHT, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_MODE, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_2X_MAGNIFY, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_PITCH, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_LINES_PER_CHUNK, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_ENABLE, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_POSITION, CURSOR_X_POSITION, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_POSITION, CURSOR_Y_POSITION, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_HOT_SPOT, CURSOR_HOT_SPOT_X, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_HOT_SPOT, CURSOR_HOT_SPOT_Y, $mask_sh), HUBP_SF!(CURSOR0_0_CURSOR_DST_OFFSET, CURSOR_DST_X_OFFSET, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_ADDRESS_HIGH, DMDATA_ADDRESS_HIGH, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_MODE, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_UPDATED, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_REPEAT, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_SIZE, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_UPDATED, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_REPEAT, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_SIZE, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_QOS_MODE, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_QOS_LEVEL, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_DL_DELTA, $mask_sh), HUBP_SF!(CURSOR0_0_DMDATA_STATUS, DMDATA_DONE, $mask_sh),
        HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_0, DST_Y_PER_VM_FLIP, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_0, DST_Y_PER_ROW_FLIP, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_1, REFCYC_PER_PTE_GROUP_FLIP_L, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_2, REFCYC_PER_META_CHUNK_FLIP_L, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_VREADY_AT_OR_AFTER_VSYNC, $mask_sh), HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_DISABLE_STOP_DATA_DURING_VM, $mask_sh), HUBP_SF!(HUBPREQ0_DCSURF_FLIP_CONTROL, HUBPREQ_MASTER_UPDATE_LOCK_STATUS, $mask_sh), HUBP_SF!(HUBPREQ0_DCSURF_FLIP_CONTROL2, SURFACE_GSL_ENABLE, $mask_sh), HUBP_SF!(HUBPREQ0_DCSURF_FLIP_CONTROL2, SURFACE_TRIPLE_BUFFER_ENABLE, $mask_sh),
        HUBP_SF!(HUBPREQ0_VMID_SETTINGS_0, VMID, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_3, REFCYC_PER_VM_GROUP_FLIP, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_4, REFCYC_PER_VM_REQ_FLIP, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_5, REFCYC_PER_PTE_GROUP_FLIP_C, $mask_sh), HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_6, REFCYC_PER_META_CHUNK_FLIP_C, $mask_sh), HUBP_SF!(HUBPREQ0_VBLANK_PARAMETERS_5, REFCYC_PER_VM_GROUP_VBLANK, $mask_sh), HUBP_SF!(HUBPREQ0_VBLANK_PARAMETERS_6, REFCYC_PER_VM_REQ_VBLANK, $mask_sh), HUBP_SF!(HUBP0_DCHUBP_REQ_SIZE_CONFIG, VM_GROUP_SIZE, $mask_sh)
    };
}

macro_rules! HUBP_MASK_SH_LIST_DCN21 { ($mask_sh:expr) => { HUBP_MASK_SH_LIST_DCN21_COMMON!($mask_sh), HUBP_SF!(HUBP0_DCSURF_TILING_CONFIG, RB_ALIGNED, $mask_sh) }; }

#[repr(C)]
pub struct dcn21_hubp {
    pub base: hubp,
    pub state: dcn_hubp_state,
    pub hubp_regs: *const dcn_hubp2_registers,
    pub hubp_shift: *const dcn_hubp2_shift,
    pub hubp_mask: *const dcn_hubp2_mask,
    pub PLAT_54186_wa_chroma_addr_offset: ::core::ffi::c_int,
}

extern "C" {
    pub fn hubp21_construct(hubp21: *mut dcn21_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_hubp2_registers, hubp_shift: *const dcn_hubp2_shift, hubp_mask: *const dcn_hubp2_mask) -> bool;
    pub fn apply_DEDCN21_142_wa_for_hostvm_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st);
    pub fn hubp21_program_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp21_program_requestor(hubp: *mut hubp, rq_regs: *mut _vcs_dpi_display_rq_regs_st);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
