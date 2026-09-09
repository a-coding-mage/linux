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
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// CGTT_ROM_CLK_CTRL0
pub const CGTT_ROM_CLK_CTRL0__ON_DELAY__SHIFT: u32 = 0x0;
pub const CGTT_ROM_CLK_CTRL0__OFF_HYSTERESIS__SHIFT: u32 = 0x4;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1__SHIFT: u32 = 0x1e;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0__SHIFT: u32 = 0x1f;
pub const CGTT_ROM_CLK_CTRL0__ON_DELAY_MASK: u32 = 0x0000000f;
pub const CGTT_ROM_CLK_CTRL0__OFF_HYSTERESIS_MASK: u32 = 0x00000ff0;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK: u32 = 0x40000000;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK: u32 = 0x80000000;

// ROM_INDEX
pub const ROM_INDEX__ROM_INDEX__SHIFT: u32 = 0x0;
pub const ROM_INDEX__ROM_INDEX_MASK: u32 = 0x01ffffff;

// ROM_DATA
pub const ROM_DATA__ROM_DATA__SHIFT: u32 = 0x0;
pub const ROM_DATA__ROM_DATA_MASK: u32 = 0xffffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
