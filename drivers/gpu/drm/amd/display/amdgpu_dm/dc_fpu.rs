// SPDX-License-Identifier: MIT
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

// Dependencies supplied by the surrounding kernel/display code.
extern "C" {
    fn in_task() -> bool;
    fn preempt_disable();
    fn preempt_enable();
    fn kernel_fpu_available() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn warn_on_once(condition: bool);
    fn bug_on(condition: bool);
    fn trace_dcn_fpu(enabled: bool, function_name: *const core::ffi::c_char, line: i32, depth: i32);
}

// DEFINE_PER_CPU(int, fpu_recursion_depth)
static mut FPU_RECURSION_DEPTH: i32 = 0;

/// Check if FPU protection is enabled.
#[inline]
pub unsafe fn dc_assert_fp_enabled() {
    let depth: i32 = FPU_RECURSION_DEPTH;

    // ASSERT(depth >= 1)
    debug_assert!(depth >= 1);
}

/// Check if FPU protection is enabled without asserting.
#[inline]
pub unsafe fn dc_is_fp_enabled() -> bool {
    let depth: i32 = FPU_RECURSION_DEPTH;

    depth >= 1
}

/// Enables FPU protection.
pub unsafe fn dc_fpu_begin(function_name: *const core::ffi::c_char, line: i32) {
    warn_on_once(!in_task());
    preempt_disable();
    FPU_RECURSION_DEPTH = FPU_RECURSION_DEPTH.wrapping_add(1);
    let depth: i32 = FPU_RECURSION_DEPTH;
    if depth == 1 {
        bug_on(!kernel_fpu_available());
        kernel_fpu_begin();
    }

    trace_dcn_fpu(true, function_name, line, depth);
}

/// Disable FPU protection.
pub unsafe fn dc_fpu_end(function_name: *const core::ffi::c_char, line: i32) {
    FPU_RECURSION_DEPTH = FPU_RECURSION_DEPTH.wrapping_sub(1);
    let depth: i32 = FPU_RECURSION_DEPTH;
    if depth == 0 {
        kernel_fpu_end();
    } else {
        warn_on_once(depth < 0);
    }

    trace_dcn_fpu(false, function_name, line, depth);
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
