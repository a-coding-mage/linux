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

// C dependencies: resource.h, dwb.h, and dcn30/dcn30_dwb.h.

// DWBC_COMMON_MASK_SH_LIST_DCN35(mask_sh)
// Expands the DCN30 common mask list and adds the DCN35 DWB_FGCG_REP_DIS
// field through SF_DWB2(DWB_ENABLE_CLK_CTRL, DWB_TOP, 0,
// DWB_FGCG_REP_DIS, mask_sh).

// DWBC_REG_FIELD_LIST_DCN3_5(type)
// Expands DWBC_REG_FIELD_LIST_DCN3_0(type), then appends `type
// DWB_FGCG_REP_DIS`.

#[repr(C)]
pub struct dcn35_dwbc_mask {
    // Fields supplied by DWBC_REG_FIELD_LIST_DCN3_0(uint32_t).
    pub DWB_FGCG_REP_DIS: u32,
}

#[repr(C)]
pub struct dcn35_dwbc_shift {
    // Fields supplied by DWBC_REG_FIELD_LIST_DCN3_0(uint8_t).
    pub DWB_FGCG_REP_DIS: u8,
}

extern "C" {
    pub fn dcn35_dwbc_construct(
        dwbc30: *mut dcn30_dwbc,
        ctx: *mut dc_context,
        dwbc_regs: *const dcn30_dwbc_registers,
        dwbc_shift: *const dcn35_dwbc_shift,
        dwbc_mask: *const dcn35_dwbc_mask,
        inst: ::core::ffi::c_int,
    );

    pub fn dcn35_dwbc_set_fgcg(dwbc30: *mut dcn30_dwbc, enable: bool);
}

// External types are provided by the included C dependencies:
// dcn30_dwbc, dc_context, and dcn30_dwbc_registers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
