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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies: reg_helper.h and dcn35_dwb.h.
// REG(reg) expands to dwbc30->dwbc_regs->reg.
// CTX expands to dwbc30->base.ctx.
// FN(reg_name, field_name) selects the dcn35 shift and mask fields.
// DC_LOGGER expands to dwbc30->base.ctx->logger.

pub unsafe fn dcn35_dwbc_construct(
    dwbc30: *mut dcn30_dwbc,
    ctx: *mut dc_context,
    dwbc_regs: *const dcn30_dwbc_registers,
    dwbc_shift: *const dcn35_dwbc_shift,
    dwbc_mask: *const dcn35_dwbc_mask,
    inst: i32,
) {
    dcn30_dwbc_construct(
        dwbc30,
        ctx,
        dwbc_regs,
        dwbc_shift as *const dcn30_dwbc_shift,
        dwbc_mask as *const dcn30_dwbc_mask,
        inst,
    );
}

pub unsafe fn dcn35_dwbc_set_fgcg(dwbc30: *mut dcn30_dwbc, enable: bool) {
    // C: REG_UPDATE(DWB_ENABLE_CLK_CTRL, DWB_FGCG_REP_DIS, !enable);
    REG_UPDATE!(dwbc30, DWB_ENABLE_CLK_CTRL, DWB_FGCG_REP_DIS, !enable);
}

extern "C" {
    fn dcn30_dwbc_construct(
        dwbc30: *mut dcn30_dwbc,
        ctx: *mut dc_context,
        dwbc_regs: *const dcn30_dwbc_registers,
        dwbc_shift: *const dcn30_dwbc_shift,
        dwbc_mask: *const dcn30_dwbc_mask,
        inst: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
