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

// C header dependencies: dml_depedencies.h, dml_logging.h, dml_assert.h

pub const DCN_DML__DML_STANDALONE: i32 = 1;
pub const DCN_DML__DML_STANDALONE__1: i32 = 1;
pub const DCN_DML__PRESENT: i32 = 1;
pub const DCN_DML__PRESENT__1: i32 = 1;
pub const DCN_DML__NUM_PLANE: i32 = 8;
pub const DCN_DML__NUM_PLANE__8: i32 = 1;
pub const DCN_DML__NUM_CURSOR: i32 = 1;
pub const DCN_DML__NUM_CURSOR__1: i32 = 1;
pub const DCN_DML__NUM_PWR_STATE: i32 = 30;
pub const DCN_DML__NUM_PWR_STATE__30: i32 = 1;
pub const DCN_DML__VM_PRESENT: i32 = 1;
pub const DCN_DML__VM_PRESENT__1: i32 = 1;
pub const DCN_DML__HOST_VM_PRESENT: i32 = 1;
pub const DCN_DML__HOST_VM_PRESENT__1: i32 = 1;
pub const DCN_DML__DWB: i32 = 1;

// To enable a lot of debug msg
pub const __DML_VBA_DEBUG__: bool = true;
pub const __DML_VBA_ENABLE_INLINE_CHECK_: i32 = 0;
pub const __DML_VBA_MIN_VSTARTUP__: i32 = 9; // <brief At which vstartup the DML start to try if the mode can be supported
pub const __DML_ARB_TO_RET_DELAY__: i32 = 7 + 95; // <brief Delay in DCFCLK from ARB to DET (1st num is ARB to SDPIF, 2nd number is SDPIF to DET)
pub const __DML_MIN_DCFCLK_FACTOR__: f64 = 1.15; // <brief fudge factor for min dcfclk calclation
pub const __DML_MAX_VRATIO_PRE__: f64 = 4.0; // <brief Prefetch schedule max vratio
pub const __DML_MAX_VRATIO_PRE_OTO__: f64 = 4.0; // <brief Prefetch schedule max vratio for one to one scheduling calculation for prefetch
pub const __DML_MAX_VRATIO_PRE_ENHANCE_PREFETCH_ACC__: f64 = 6.0; // <brief Prefetch schedule max vratio when enhance prefetch schedule acceleration is enabled and vstartup is earliest possible already
pub const __DML_NUM_PLANES__: i32 = DCN_DML__NUM_PLANE;
pub const __DML_NUM_CURSORS__: i32 = DCN_DML__NUM_CURSOR;
pub const __DML_DPP_INVALID__: i32 = 0;
pub const __DML_NUM_DMB__: i32 = DCN_DML__DWB;
pub const __DML_PIPE_NO_PLANE__: i32 = 99;
pub const __DML_MAX_STATE_ARRAY_SIZE__: i32 = DCN_DML__NUM_PWR_STATE;

// Compilation define: __DML_DLL_EXPORT__

// int is 32-bit in C/C++, but Integer datatype is 16-bit in VBA. this should map to Long in VBA
pub type dml_int_t = i32;
pub type dml_uint_t = u32;
pub type dml_float_t = f64;

// bool is 8-bit in C/C++, but Boolean is 16-bit in VBA, use "short" in C/C++ DLL so the struct work when vba uses DLL
// Or the VBA side don't use Boolean, just use "Byte", then C side can use bool
pub type dml_bool_t = bool;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
