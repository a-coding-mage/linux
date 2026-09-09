/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding translated headers:
// core_types.h and dchubbub.h

#[repr(C)]
pub struct dcn_vmid_registers {
    pub CNTL: u32,
    pub PAGE_TABLE_BASE_ADDR_HI32: u32,
    pub PAGE_TABLE_BASE_ADDR_LO32: u32,
    pub PAGE_TABLE_START_ADDR_HI32: u32,
    pub PAGE_TABLE_START_ADDR_LO32: u32,
    pub PAGE_TABLE_END_ADDR_HI32: u32,
    pub PAGE_TABLE_END_ADDR_LO32: u32,
}

#[repr(C)]
pub struct dcn_vmid_page_table_config {
    pub page_table_start_addr: u64,
    pub page_table_end_addr: u64,
    pub depth: dcn_hubbub_page_table_depth,
    pub block_size: dcn_hubbub_page_table_block_size,
    pub page_table_base_addr: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
