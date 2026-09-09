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
 */

// Dependency supplied by the corresponding hwmgr translation.
use crate::{pp_hwmgr, pp_power_state, PP_StateUILabel};

extern "C" {
    pub fn psm_init_power_state_table(hwmgr: *mut pp_hwmgr) -> ::core::ffi::c_int;
    pub fn psm_fini_power_state_table(hwmgr: *mut pp_hwmgr) -> ::core::ffi::c_int;
    pub fn psm_set_boot_states(hwmgr: *mut pp_hwmgr) -> ::core::ffi::c_int;
    pub fn psm_set_performance_states(hwmgr: *mut pp_hwmgr) -> ::core::ffi::c_int;
    pub fn psm_set_user_performance_state(
        hwmgr: *mut pp_hwmgr,
        label_id: PP_StateUILabel,
        state: *mut *mut pp_power_state,
    ) -> ::core::ffi::c_int;
    pub fn psm_adjust_power_state_dynamic(
        hwmgr: *mut pp_hwmgr,
        skip_display_settings: bool,
        new_ps: *mut pp_power_state,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
