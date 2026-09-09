/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies supplied by clk_mgr_internal.h, dcn30/dcn30m_clk_mgr.h, and
// dcn30m_clk_mgr_smu_msg.h remain external to this translation unit.

pub unsafe fn dcn30m_set_smartmux_switch(
    clk_mgr_base: *mut clk_mgr,
    pins_to_set: u32,
) -> u32 {
    let clk_mgr: *mut clk_mgr_internal = TO_CLK_MGR_INTERNAL(clk_mgr_base);

    dcn30m_smu_set_smart_mux_switch(clk_mgr, pins_to_set)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
