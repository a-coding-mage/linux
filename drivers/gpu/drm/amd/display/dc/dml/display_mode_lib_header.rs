/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 *
 */

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, dc_features.h, display_mode_structs.h,
// display_mode_enums.h, display_mode_vba.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_project {
    DML_PROJECT_UNDEFINED,
    DML_PROJECT_RAVEN1,
    DML_PROJECT_NAVI10,
    DML_PROJECT_NAVI10v2,
    DML_PROJECT_DCN201,
    DML_PROJECT_DCN21,
    DML_PROJECT_DCN30,
    DML_PROJECT_DCN31,
    DML_PROJECT_DCN315,
    DML_PROJECT_DCN314,
    DML_PROJECT_DCN32,
}

#[repr(C)]
pub struct dml_funcs {
    pub rq_dlg_get_dlg_reg: Option<unsafe extern "C" fn(
        mode_lib: *mut display_mode_lib,
        dlg_regs: *mut display_dlg_regs_st,
        ttu_regs: *mut display_ttu_regs_st,
        e2e_pipe_param: *const display_e2e_pipe_params_st,
        num_pipes: ::core::ffi::c_uint,
        pipe_idx: ::core::ffi::c_uint,
        cstate_en: bool,
        pstate_en: bool,
        vm_en: bool,
        ignore_viewport_pos: bool,
        immediate_flip_support: bool,
    )>,
    pub rq_dlg_get_rq_reg: Option<unsafe extern "C" fn(
        mode_lib: *mut display_mode_lib,
        rq_regs: *mut display_rq_regs_st,
        pipe_param: *const display_pipe_params_st,
    )>,
    // DLG interfaces have different function parameters in DCN32.
    // Create new function pointers to address the changes
    pub rq_dlg_get_dlg_reg_v2: Option<unsafe extern "C" fn(
        mode_lib: *mut display_mode_lib,
        dlg_regs: *mut display_dlg_regs_st,
        ttu_regs: *mut display_ttu_regs_st,
        e2e_pipe_param: *mut display_e2e_pipe_params_st,
        num_pipes: ::core::ffi::c_uint,
        pipe_idx: ::core::ffi::c_uint,
    )>,
    pub rq_dlg_get_rq_reg_v2: Option<unsafe extern "C" fn(
        rq_regs: *mut display_rq_regs_st,
        mode_lib: *mut display_mode_lib,
        e2e_pipe_param: *const display_e2e_pipe_params_st,
        num_pipes: ::core::ffi::c_uint,
        pipe_idx: ::core::ffi::c_uint,
    )>,
    pub recalculate: Option<unsafe extern "C" fn(mode_lib: *mut display_mode_lib)>,
    pub validate: Option<unsafe extern "C" fn(mode_lib: *mut display_mode_lib)>,
}

#[repr(C)]
pub struct display_mode_lib {
    pub ip: _vcs_dpi_ip_params_st,
    pub soc: _vcs_dpi_soc_bounding_box_st,
    pub project: dml_project,
    pub vba: vba_vars_st,
    pub logger: *mut dal_logger,
    pub funcs: dml_funcs,
    pub dml_pipe_state: [_vcs_dpi_display_e2e_pipe_params_st; 6],
    pub validate_max_state: bool,
}

extern "C" {
    pub fn dml_init_instance(
        lib: *mut display_mode_lib,
        soc_bb: *const _vcs_dpi_soc_bounding_box_st,
        ip_params: *const _vcs_dpi_ip_params_st,
        project: dml_project,
    );

    pub fn dml_get_status_message(status: dm_validation_status) -> *const ::core::ffi::c_char;

    pub fn dml_log_pipe_params(
        mode_lib: *mut display_mode_lib,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
    );

    pub fn dml_log_mode_support_params(mode_lib: *mut display_mode_lib);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
