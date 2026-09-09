/* SPDX-License-Identifier: MIT */
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

// Dependencies supplied by the corresponding headers and register helpers.

#[allow(non_snake_case)]
pub unsafe fn dcn35_mmhubbub_construct(
    mcif_wb30: *mut dcn30_mmhubbub,
    ctx: *mut dc_context,
    mcif_wb_regs: *const dcn35_mmhubbub_registers,
    mcif_wb_shift: *const dcn35_mmhubbub_shift,
    mcif_wb_mask: *const dcn35_mmhubbub_mask,
    inst: ::core::ffi::c_int,
) {
    dcn32_mmhubbub_construct(
        mcif_wb30,
        ctx,
        mcif_wb_regs as *const dcn30_mmhubbub_registers,
        mcif_wb_shift as *const dcn30_mmhubbub_shift,
        mcif_wb_mask as *const dcn30_mmhubbub_mask,
        inst,
    );
}

#[allow(non_snake_case)]
pub unsafe fn dcn35_mmhubbub_set_fgcg(
    mcif_wb30: *mut dcn30_mmhubbub,
    enabled: bool,
) {
    REG_UPDATE!(MMHUBBUB_CLOCK_CNTL, MMHUBBUB_FGCG_REP_DIS, !enabled);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
