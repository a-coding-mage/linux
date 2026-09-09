/*
 * Copyright (C) 2019  Advanced Micro Devices, Inc.
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

// C header guard: _nbio_7_0_SMN_HEADER

pub const smnCPM_CONTROL: u32 = 0x11180460;
pub const smnPCIE_CNTL2: u32 = 0x11180070;

pub const smnPCIE_PERF_COUNT_CNTL: u32 = 0x11180200;
pub const smnPCIE_PERF_CNTL_TXCLK: u32 = 0x11180204;
pub const smnPCIE_PERF_COUNT0_TXCLK: u32 = 0x11180208;
pub const smnPCIE_PERF_COUNT1_TXCLK: u32 = 0x1118020c;
pub const smnPCIE_PERF_CNTL_MST_R_CLK: u32 = 0x11180210;
pub const smnPCIE_PERF_COUNT0_MST_R_CLK: u32 = 0x11180214;
pub const smnPCIE_PERF_COUNT1_MST_R_CLK: u32 = 0x11180218;
pub const smnPCIE_PERF_CNTL_MST_C_CLK: u32 = 0x1118021c;
pub const smnPCIE_PERF_COUNT0_MST_C_CLK: u32 = 0x11180220;
pub const smnPCIE_PERF_COUNT1_MST_C_CLK: u32 = 0x11180224;
pub const smnPCIE_PERF_CNTL_SLV_R_CLK: u32 = 0x11180228;
pub const smnPCIE_PERF_COUNT0_SLV_R_CLK: u32 = 0x1118022c;
pub const smnPCIE_PERF_COUNT1_SLV_R_CLK: u32 = 0x11180230;
pub const smnPCIE_PERF_CNTL_SLV_S_C_CLK: u32 = 0x11180234;
pub const smnPCIE_PERF_COUNT0_SLV_S_C_CLK: u32 = 0x11180238;
pub const smnPCIE_PERF_COUNT1_SLV_S_C_CLK: u32 = 0x1118023c;
pub const smnPCIE_PERF_CNTL_SLV_NS_C_CLK: u32 = 0x11180240;
pub const smnPCIE_PERF_COUNT0_SLV_NS_C_CLK: u32 = 0x11180244;
pub const smnPCIE_PERF_COUNT1_SLV_NS_C_CLK: u32 = 0x11180248;
pub const smnPCIE_PERF_CNTL_EVENT0_PORT_SEL: u32 = 0x1118024c;
pub const smnPCIE_PERF_CNTL_EVENT1_PORT_SEL: u32 = 0x11180250;
pub const smnPCIE_PERF_CNTL_TXCLK2: u32 = 0x11180254;
pub const smnPCIE_PERF_COUNT0_TXCLK2: u32 = 0x11180258;
pub const smnPCIE_PERF_COUNT1_TXCLK2: u32 = 0x1118025c;
pub const smnPCIE_PERF_CNTL_TXCLK3: u32 = 0x1118021c;
pub const smnPCIE_PERF_COUNT0_TXCLK3: u32 = 0x11180220;
pub const smnPCIE_PERF_COUNT1_TXCLK3: u32 = 0x11180224;
pub const smnPCIE_PERF_CNTL_TXCLK4: u32 = 0x11180228;
pub const smnPCIE_PERF_COUNT0_TXCLK4: u32 = 0x1118022c;
pub const smnPCIE_PERF_COUNT1_TXCLK4: u32 = 0x11180230;

pub const smnPCIE_RX_NUM_NAK: u32 = 0x11180038;
pub const smnPCIE_RX_NUM_NAK_GENERATED: u32 = 0x1118003c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
