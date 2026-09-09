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
 *
 */

// Dependency supplied by dmub_cmd.h is intentionally left external.

pub const LOAD_DMCU_FW: u32 = 1;
pub const LOAD_PHY_FW: u32 = 2;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dmucb_trace_code {
    DMCUB__UNKNOWN,
    DMCUB__MAIN_BEGIN,
    DMCUB__PHY_INIT_BEGIN,
    DMCUB__PHY_FW_SRAM_LOAD_BEGIN,
    DMCUB__PHY_FW_SRAM_LOAD_END,
    DMCUB__PHY_INIT_POLL_DONE,
    DMCUB__PHY_INIT_END,
    DMCUB__DMCU_ERAM_LOAD_BEGIN,
    DMCUB__DMCU_ERAM_LOAD_END,
    DMCUB__DMCU_ISR_LOAD_BEGIN,
    DMCUB__DMCU_ISR_LOAD_END,
    DMCUB__MAIN_IDLE,
    DMCUB__PERF_TRACE,
    DMCUB__PG_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcub_trace_buf_entry {
    pub trace_code: dmucb_trace_code,
    pub tick_count: u32,
    pub param0: u32,
    pub param1: u32,
}

pub const TRACE_BUF_SIZE: usize = 1024; // 1 kB
pub const PERF_TRACE_MAX_ENTRY: usize =
    (TRACE_BUF_SIZE - 8) / core::mem::size_of::<dmcub_trace_buf_entry>();

#[repr(C)]
pub struct dmcub_trace_buf {
    pub entry_count: u32,
    pub clk_freq: u32,
    pub entries: [dmcub_trace_buf_entry; PERF_TRACE_MAX_ENTRY],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
