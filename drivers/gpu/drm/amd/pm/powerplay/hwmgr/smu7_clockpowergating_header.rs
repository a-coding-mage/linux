/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency supplied by smu7_hwmgr.h.

extern "C" {
    pub fn smu7_powergate_vce(hwmgr: *mut pp_hwmgr, bgate: bool);
    pub fn smu7_powergate_uvd(hwmgr: *mut pp_hwmgr, bgate: bool);
    pub fn smu7_powergate_acp(hwmgr: *mut pp_hwmgr, bgate: bool) -> i32;
    pub fn smu7_disable_clock_power_gating(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_update_clock_gatings(hwmgr: *mut pp_hwmgr, msg_id: *const u32) -> i32;
    pub fn smu7_powergate_gfx(hwmgr: *mut pp_hwmgr, enable: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
