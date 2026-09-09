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
 * Interface Functions related to the BIOS PowerPlay Tables.
 *
 */

// Forward declarations corresponding to incomplete C struct types.
pub enum pp_hwmgr {}
pub enum pp_power_state {}
pub enum pp_hw_power_state {}
pub enum pp_table_func {}

extern "C" {
    pub static pptable_funcs: pp_table_func;
}

pub type pp_tables_hw_clock_info_callback = unsafe extern "C" fn(
    hwmgr: *mut pp_hwmgr,
    hw_ps: *mut pp_hw_power_state,
    index: ::core::ffi::c_uint,
    clock_info: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;

extern "C" {
    pub fn pp_tables_get_num_of_entries(
        hwmgr: *mut pp_hwmgr,
        num_of_entries: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn pp_tables_get_entry(
        hwmgr: *mut pp_hwmgr,
        entry_index: ::core::ffi::c_ulong,
        ps: *mut pp_power_state,
        func: pp_tables_hw_clock_info_callback,
    ) -> ::core::ffi::c_int;

    pub fn pp_tables_get_response_times(
        hwmgr: *mut pp_hwmgr,
        vol_rep_time: *mut u32,
        bb_rep_time: *mut u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
