/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2020-2022 Advanced Micro Devices, Inc.
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

/*--------------------MES_MAP_PROCESS (PER DEBUG VMID)--------------------*/

/* C header guard: PM4_MES_MAP_PROCESS_PER_DEBUG_VMID_DEFINED */

#[repr(C)]
pub union pm4_mes_map_process_aldebaran_header {
    pub header: std::mem::ManuallyDrop<PM4_MES_TYPE_3_HEADER>,
    pub ordinal1: u32,
}

#[repr(C)]
pub struct pm4_mes_map_process_aldebaran_bitfields2 {
    pub value: u32,
}

impl pm4_mes_map_process_aldebaran_bitfields2 {
    pub const fn pasid(self) -> u32 { self.value & 0xffff }
    pub const fn single_memops(self) -> u32 { (self.value >> 16) & 0x1 }
    pub const fn exec_cleaner_shader(self) -> u32 { (self.value >> 17) & 0x1 }
    pub const fn debug_vmid(self) -> u32 { (self.value >> 18) & 0xf }
    pub const fn new_debug(self) -> u32 { (self.value >> 22) & 0x1 }
    pub const fn tmz(self) -> u32 { (self.value >> 23) & 0x1 }
    pub const fn diq_enable(self) -> u32 { (self.value >> 24) & 0x1 }
    pub const fn process_quantum(self) -> u32 { (self.value >> 25) & 0x7f }
}

#[repr(C)]
pub union pm4_mes_map_process_aldebaran_ordinal2_union {
    pub bitfields2: pm4_mes_map_process_aldebaran_bitfields2,
    pub ordinal2: u32,
}

#[repr(C)]
pub struct pm4_mes_map_process_aldebaran {
    pub header: pm4_mes_map_process_aldebaran_header,
    pub ordinal2: pm4_mes_map_process_aldebaran_ordinal2_union,
    pub vm_context_page_table_base_addr_lo32: u32,
    pub vm_context_page_table_base_addr_hi32: u32,
    pub sh_mem_bases: u32,
    pub sh_mem_config: u32,
    pub sq_shader_tba_lo: u32,
    pub sq_shader_tba_hi: u32,
    pub sq_shader_tma_lo: u32,
    pub sq_shader_tma_hi: u32,
    pub reserved6: u32,
    pub gds_addr_lo: u32,
    pub gds_addr_hi: u32,
    pub bitfields14: pm4_mes_map_process_aldebaran_bitfields14_union,
    pub spi_gdbg_per_vmid_cntl: u32,
    pub tcp_watch_cntl: [u32; 4],
    pub completion_signal_lo: u32,
    pub completion_signal_hi: u32,
}

#[repr(C)]
pub struct pm4_mes_map_process_aldebaran_bitfields14 {
    pub value: u32,
}

#[repr(C)]
pub union pm4_mes_map_process_aldebaran_bitfields14_union {
    pub bitfields14: pm4_mes_map_process_aldebaran_bitfields14,
    pub ordinal14: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
