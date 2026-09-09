/* Copyright 2023 Advanced Micro Devices, Inc.
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

// C dependencies: dc.h, dc_types.h, and hw_shared.h.

use core::ffi::c_char;

#[repr(C)]
pub struct pg_cntl {
    pub ctx: *mut dc_context,
    pub funcs: *const pg_cntl_funcs,
    pub pg_pipe_res_enable: [[bool; MAX_PIPES]; PG_HW_PIPE_RESOURCES_NUM_ELEMENT],
    pub pg_res_enable: [bool; PG_HW_RESOURCES_NUM_ELEMENT],
}

#[repr(C)]
pub struct pg_cntl_funcs {
    pub dsc_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, dsc_inst: u32, power_on: bool)>,
    pub hubp_dpp_pg_control: Option<unsafe extern "C" fn(
        pg_cntl: *mut pg_cntl,
        hubp_dpp_inst: u32,
        power_on: bool,
    )>,
    pub hpo_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub io_clk_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub plane_otg_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub mpcc_pg_control: Option<unsafe extern "C" fn(
        pg_cntl: *mut pg_cntl,
        mpcc_inst: u32,
        power_on: bool,
    )>,
    pub opp_pg_control: Option<unsafe extern "C" fn(
        pg_cntl: *mut pg_cntl,
        opp_inst: u32,
        power_on: bool,
    )>,
    pub optc_pg_control: Option<unsafe extern "C" fn(
        pg_cntl: *mut pg_cntl,
        optc_inst: u32,
        power_on: bool,
    )>,
    pub dwb_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub mem_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub dio_pg_control:
        Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl, power_on: bool)>,
    pub init_pg_status: Option<unsafe extern "C" fn(pg_cntl: *mut pg_cntl)>,
    pub print_pg_status: Option<unsafe extern "C" fn(
        pg_cntl: *mut pg_cntl,
        debug_func: *const c_char,
        debug_log: *const c_char,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
