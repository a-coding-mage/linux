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

// Dependency supplied by display_rq_dlg_helpers.h.

#[repr(C)]
pub struct display_mode_lib {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_rq_regs_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_rq_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_pipe_source_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_dlg_regs_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_ttu_regs_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_rq_dlg_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_dlg_sys_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_e2e_pipe_params_st {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub fn dml1_extract_rq_regs(
        mode_lib: *mut display_mode_lib,
        rq_regs: *mut _vcs_dpi_display_rq_regs_st,
        rq_param: *const _vcs_dpi_display_rq_params_st,
    );

    /* Function: dml_rq_dlg_get_rq_params
     *  Calculate requestor related parameters that register definition agnostic
     *  (i.e. this layer does try to separate real values from register definition)
     * Input:
     *  pipe_src_param - pipe source configuration (e.g. vp, pitch, etc.)
     * Output:
     *  rq_param - values that can be used to setup RQ (e.g. swath_height, plane1_addr, etc.)
     */
    pub fn dml1_rq_dlg_get_rq_params(
        mode_lib: *mut display_mode_lib,
        rq_param: *mut _vcs_dpi_display_rq_params_st,
        pipe_src_param: *const _vcs_dpi_display_pipe_source_params_st,
    );

    /* Function: dml_rq_dlg_get_dlg_params
     *  Calculate deadline related parameters
     */
    pub fn dml1_rq_dlg_get_dlg_params(
        mode_lib: *mut display_mode_lib,
        dlg_regs: *mut _vcs_dpi_display_dlg_regs_st,
        ttu_regs: *mut _vcs_dpi_display_ttu_regs_st,
        rq_dlg_param: *const _vcs_dpi_display_rq_dlg_params_st,
        dlg_sys_param: *const _vcs_dpi_display_dlg_sys_params_st,
        e2e_pipe_param: *const _vcs_dpi_display_e2e_pipe_params_st,
        cstate_en: bool,
        pstate_en: bool,
        vm_en: bool,
        iflip_en: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
