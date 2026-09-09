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

// Dependency: display_mode_lib.h

/* Function: Printer functions
 *  Print various struct
 */

#[repr(C)]
pub struct display_mode_lib {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_rq_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_data_rq_sizing_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_data_rq_dlg_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_data_rq_misc_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_dlg_sys_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_data_rq_regs_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_rq_regs_st {
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

unsafe extern "C" {
    pub fn print__rq_params_st(
        mode_lib: *mut display_mode_lib,
        rq_param: *const _vcs_dpi_display_rq_params_st,
    );
    pub fn print__data_rq_sizing_params_st(
        mode_lib: *mut display_mode_lib,
        rq_sizing: *const _vcs_dpi_display_data_rq_sizing_params_st,
    );
    pub fn print__data_rq_dlg_params_st(
        mode_lib: *mut display_mode_lib,
        rq_dlg_param: *const _vcs_dpi_display_data_rq_dlg_params_st,
    );
    pub fn print__data_rq_misc_params_st(
        mode_lib: *mut display_mode_lib,
        rq_misc_param: *const _vcs_dpi_display_data_rq_misc_params_st,
    );
    pub fn print__dlg_sys_params_st(
        mode_lib: *mut display_mode_lib,
        dlg_sys_param: *const _vcs_dpi_display_dlg_sys_params_st,
    );

    pub fn print__data_rq_regs_st(
        mode_lib: *mut display_mode_lib,
        rq_regs: *const _vcs_dpi_display_data_rq_regs_st,
    );
    pub fn print__rq_regs_st(
        mode_lib: *mut display_mode_lib,
        rq_regs: *const _vcs_dpi_display_rq_regs_st,
    );
    pub fn print__dlg_regs_st(
        mode_lib: *mut display_mode_lib,
        dlg_regs: *const _vcs_dpi_display_dlg_regs_st,
    );
    pub fn print__ttu_regs_st(
        mode_lib: *mut display_mode_lib,
        ttu_regs: *const _vcs_dpi_display_ttu_regs_st,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
