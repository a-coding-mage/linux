/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn dc_assert_fp_enabled();
    pub fn dc_is_fp_enabled() -> bool;
    pub fn dc_fpu_begin(function_name: *const core::ffi::c_char, line: core::ffi::c_int);
    pub fn dc_fpu_end(function_name: *const core::ffi::c_char, line: core::ffi::c_int);
}

// C build condition: _LINUX_FPU_COMPILATION_UNIT.
// When enabled, DC_FP_START/DC_FP_END expand to BUILD_BUG().
#[cfg(not(feature = "linux_fpu_compilation_unit"))]
#[macro_export]
macro_rules! DC_FP_START {
    () => {
        unsafe {
            $crate::dc_fpu_begin(concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as core::ffi::c_int)
        }
    };
}

#[cfg(not(feature = "linux_fpu_compilation_unit"))]
#[macro_export]
macro_rules! DC_FP_END {
    () => {
        unsafe {
            $crate::dc_fpu_end(concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as core::ffi::c_int)
        }
    };
}

// C build condition: CONFIG_DRM_AMD_DC_FP.
#[cfg(all(not(feature = "linux_fpu_compilation_unit"), feature = "drm_amd_dc_fp"))]
#[macro_export]
macro_rules! DC_RUN_WITH_PREEMPTION_ENABLED {
    ($code:expr) => {{
        let dc_fp_enabled = unsafe { $crate::dc_is_fp_enabled() };
        if dc_fp_enabled {
            $crate::DC_FP_END!();
        }
        $code;
        if dc_fp_enabled {
            $crate::DC_FP_START!();
        }
    }};
}

#[cfg(all(not(feature = "linux_fpu_compilation_unit"), not(feature = "drm_amd_dc_fp")))]
#[macro_export]
macro_rules! DC_RUN_WITH_PREEMPTION_ENABLED {
    ($code:expr) => {{ $code }};
}

#[cfg(feature = "linux_fpu_compilation_unit")]
#[macro_export]
macro_rules! DC_FP_START {
    () => {
        compile_error!("BUILD_BUG")
    };
}

#[cfg(feature = "linux_fpu_compilation_unit")]
#[macro_export]
macro_rules! DC_FP_END {
    () => {
        compile_error!("BUILD_BUG")
    };
}

#[cfg(feature = "linux_fpu_compilation_unit")]
#[macro_export]
macro_rules! DC_RUN_WITH_PREEMPTION_ENABLED {
    ($code:expr) => {{
        compile_error!("BUILD_BUG");
        $code;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
