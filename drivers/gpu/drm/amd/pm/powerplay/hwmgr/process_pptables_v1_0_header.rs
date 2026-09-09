/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied by the translated hwmgr interface.

unsafe extern "C" {
    pub static pptable_v1_0_funcs: pp_table_func;

    pub fn get_number_of_powerplay_table_entries_v1_0(
        hwmgr: *mut pp_hwmgr,
    ) -> i32;

    pub fn get_powerplay_table_entry_v1_0(
        hwmgr: *mut pp_hwmgr,
        entry_index: u32,
        power_state: *mut pp_power_state,
        call_back_func: Option<
            unsafe extern "C" fn(
                *mut pp_hwmgr,
                *mut core::ffi::c_void,
                *mut pp_power_state,
                *mut core::ffi::c_void,
                u32,
            ) -> i32,
        >,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
