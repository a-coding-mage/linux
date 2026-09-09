/*
 * Copyright (C) 2020  Advanced Micro Devices, Inc.
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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
 * OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// SMUIO_GFX_MISC_CNTL
pub const SMUIO_GFX_MISC_CNTL__PWR_GFXOFF_STATUS_MASK: u32 = 0x00000006;
pub const SMUIO_GFX_MISC_CNTL__PWR_GFXOFF_STATUS__SHIFT: u32 = 0x1;

// PWR_MISC_CNTL_STATUS
pub const PWR_MISC_CNTL_STATUS__PWR_GFX_RLC_CGPG_EN__SHIFT: u32 = 0x0;
pub const PWR_MISC_CNTL_STATUS__PWR_GFXOFF_STATUS__SHIFT: u32 = 0x1;
pub const PWR_MISC_CNTL_STATUS__PWR_GFX_RLC_CGPG_EN_MASK: u32 = 0x00000001;
pub const PWR_MISC_CNTL_STATUS__PWR_GFXOFF_STATUS_MASK: u32 = 0x00000006;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
