/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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
 */

/* C header guards and conditional definition guards are represented by this
 * Rust translation unit's item definitions. */

#[repr(C)]
#[derive(Copy, Clone)]
pub union PM4_MES_TYPE_3_HEADER {
    pub bits: PM4_MES_TYPE_3_HEADER_BITS,
    pub u32all: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PM4_MES_TYPE_3_HEADER_BITS {
    pub bits: u32,
}

impl PM4_MES_TYPE_3_HEADER_BITS {
    pub const fn new(reserved1: u32, opcode: u32, count: u32, type_: u32) -> Self {
        Self { bits: (reserved1 & 0xff) | ((opcode & 0xff) << 8) | ((count & 0x3fff) << 16) | ((type_ & 0x3) << 30) }
    }
    pub const fn reserved1(self) -> u32 { self.bits & 0xff }
    pub const fn opcode(self) -> u32 { (self.bits >> 8) & 0xff }
    pub const fn count(self) -> u32 { (self.bits >> 16) & 0x3fff }
    pub const fn type_(self) -> u32 { (self.bits >> 30) & 0x3 }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PM4_MAP_PROCESS_WORD {
    pub bits: PM4_MAP_PROCESS_BITS,
    pub ordinal: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PM4_MAP_PROCESS_BITS { pub bits: u32 }

#[repr(C)]
pub struct pm4_map_process {
    pub header: PM4_MAP_PROCESS_WORD,
    pub process: PM4_MAP_PROCESS_WORD,
    pub page_table: PM4_MAP_PROCESS_WORD,
    pub sh_mem_bases: u32,
    pub sh_mem_ape1_base: u32,
    pub sh_mem_ape1_limit: u32,
    pub sh_mem_config: u32,
    pub gds_addr_lo: u32,
    pub gds_addr_hi: u32,
    pub resources: PM4_MAP_PROCESS_WORD,
}

#[repr(C)]
pub struct pm4_map_process_scratch_kv {
    pub header: PM4_MAP_PROCESS_WORD,
    pub process: PM4_MAP_PROCESS_WORD,
    pub page_table: PM4_MAP_PROCESS_WORD,
    pub reserved3: u32,
    pub sh_mem_bases: u32,
    pub sh_mem_config: u32,
    pub sh_mem_ape1_base: u32,
    pub sh_mem_ape1_limit: u32,
    pub sh_hidden_private_base_vmid: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub gds_addr_lo: u32,
    pub gds_addr_hi: u32,
    pub resources: PM4_MAP_PROCESS_WORD,
    pub completion_signal_lo32: u32,
    pub completion_signal_hi32: u32,
}

pub const CACHE_FLUSH_AND_INV_TS_EVENT: u32 = 0x00000014;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
