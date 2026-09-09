/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency corresponding to: #include "link_service.h"

pub struct dc_link;
pub struct link_hwss;

extern "C" {
    pub fn dp_dio_fixed_vs_pe_retimer_get_lttpr_write_address(
        link: *mut dc_link,
    ) -> u32;
    pub fn dp_dio_fixed_vs_pe_retimer_lane_cfg_to_hw_cfg(link: *mut dc_link) -> u8;
    pub fn dp_dio_fixed_vs_pe_retimer_exit_manual_automation(link: *mut dc_link);
    pub fn enable_dio_fixed_vs_pe_retimer_program_4lane_output(link: *mut dc_link);
    pub fn requires_fixed_vs_pe_retimer_dio_link_hwss(link: *const dc_link) -> bool;
    pub fn get_dio_fixed_vs_pe_retimer_link_hwss() -> *const link_hwss;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
