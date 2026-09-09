/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies: dcn20/dcn20_hubp.h and dcn21/dcn21_hubp.h.
// The register-list macros below intentionally remain dependent on those
// externally supplied definitions.

macro_rules! HUBP_REG_LIST_DCN30 {
    ($id:expr) => {
        HUBP_REG_LIST_DCN21!($id),
        SRI!(DCN_DMDATA_VM_CNTL, HUBPREQ, $id)
    };
}

macro_rules! HUBP_MASK_SH_LIST_DCN30_BASE {
    ($mask_sh:expr) => {
        HUBP_MASK_SH_LIST_DCN21_COMMON!($mask_sh),
        HUBP_SF!(HUBP0_DCSURF_SURFACE_CONFIG, ALPHA_PLANE_EN, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, REFCYC_PER_VM_DMDATA, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_FAULT_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_FAULT_STATUS_CLEAR, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_UNDERFLOW_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_LATE_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_UNDERFLOW_STATUS_CLEAR, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_DONE, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_ADDR_CONFIG, NUM_PKRS, $mask_sh)
    };
}

// The C macro is a register-field expansion.  Keep its complete field list
// as a Rust macro invocation interface; the field-building macros are defined
// by the register dependency headers.
macro_rules! HUBP_MASK_SH_LIST_DCN30 {
    ($mask_sh:expr) => {
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, REFCYC_PER_VM_DMDATA, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_FAULT_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_FAULT_STATUS_CLEAR, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_UNDERFLOW_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_LATE_STATUS, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_UNDERFLOW_STATUS_CLEAR, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCN_DMDATA_VM_CNTL, DMDATA_VM_DONE, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_BLANK_EN, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_TTU_DISABLE, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_UNDERFLOW_STATUS, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_UNDERFLOW_CLEAR, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_NO_OUTSTANDING_REQ, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_VTG_SEL, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_DISABLE, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_IN_BLANK, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_ADDR_CONFIG, NUM_PIPES, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_ADDR_CONFIG, PIPE_INTERLEAVE, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_ADDR_CONFIG, MAX_COMPRESSED_FRAGS, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_ADDR_CONFIG, NUM_PKRS, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_TILING_CONFIG, SW_MODE, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_TILING_CONFIG, META_LINEAR, $mask_sh),
        HUBP_SF!(HUBP0_DCSURF_TILING_CONFIG, PIPE_ALIGNED, $mask_sh),
        HUBP_MASK_SH_LIST_DCN_VM!($mask_sh)
    };
}

extern "C" {
    pub fn hubp3_construct(
        hubp2: *mut dcn20_hubp,
        ctx: *mut dc_context,
        inst: u32,
        hubp_regs: *const dcn_hubp2_registers,
        hubp_shift: *const dcn_hubp2_shift,
        hubp_mask: *const dcn_hubp2_mask,
    ) -> bool;
    pub fn hubp3_set_vm_system_aperture_settings(hubp: *mut hubp, apt: *mut vm_system_aperture_param);
    pub fn hubp3_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool;
    pub fn hubp3_program_surface_config(hubp: *mut hubp, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, plane_size: *mut plane_size, rotation: dc_rotation_angle, dcc: *mut dc_plane_dcc_param, horizontal_mirror: bool, compat_level: c_uint);
    pub fn hubp3_setup(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st, rq_regs: *mut _vcs_dpi_display_rq_regs_st, pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st);
    pub fn hubp3_program_tiling(hubp2: *mut dcn20_hubp, info: *const dc_tiling_info, pixel_format: surface_pixel_format);
    pub fn hubp3_dcc_control(hubp: *mut hubp, enable: bool, blk_size: hubp_ind_block_size);
    pub fn hubp3_dcc_control_sienna_cichlid(hubp: *mut hubp, dcc: *mut dc_plane_dcc_param);
    pub fn hubp3_dmdata_set_attributes(hubp: *mut hubp, attr: *const dc_dmdata_attributes);
    pub fn hubp3_read_state(hubp: *mut hubp);
    pub fn hubp3_read_reg_state(hubp: *mut hubp, reg_state: *mut dcn_hubp_reg_state);
    pub fn hubp3_init(hubp: *mut hubp);
    pub fn hubp3_clear_tiling(hubp: *mut hubp);
    pub fn hubp3_get_current_read_line(hubp: *mut hubp) -> u32;
    pub fn hubp3_get_underflow_status(hubp: *mut hubp) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
