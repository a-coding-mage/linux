/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_regs {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[repr(usize)]
pub enum cpuid_regs_idx {
    CPUID_EAX = 0,
    CPUID_EBX,
    CPUID_ECX,
    CPUID_EDX,
}

pub const CPUID_LEAF_MWAIT: u32 = 0x05;
pub const CPUID_LEAF_DCA: u32 = 0x09;
pub const CPUID_LEAF_XSTATE: u32 = 0x0d;
pub const CPUID_LEAF_TSC: u32 = 0x15;
pub const CPUID_LEAF_FREQ: u32 = 0x16;
pub const CPUID_LEAF_TILE: u32 = 0x1d;

#[inline]
pub const fn CPUID_RANGE(idx: u32) -> u32 { idx & 0xffff0000 }
#[inline]
pub const fn CPUID_RANGE_MAX(idx: u32) -> u32 { CPUID_RANGE(idx) + 0xffff }

pub const CPUID_BASE_START: u32 = 0x00000000;
pub const CPUID_BASE_END: u32 = CPUID_RANGE_MAX(CPUID_BASE_START);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_0x2_reg {
    pub value: u32,
}

#[repr(C)]
pub union leaf_0x2_regs {
    pub reg: [leaf_0x2_reg; 4],
    pub regv: [u32; 4],
    pub desc: [u8; 16],
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum _cache_table_type {
    CACHE_L1_INST = 1,
    CACHE_L1_DATA,
    CACHE_L2,
    CACHE_L3,
}

pub const __TLB_TABLE_TYPE_BEGIN: u8 = _cache_table_type::CACHE_L3 as u8 + 1;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum _tlb_table_type {
    TLB_INST_4K = __TLB_TABLE_TYPE_BEGIN,
    TLB_INST_4M,
    TLB_INST_2M_4M,
    TLB_INST_ALL,
    TLB_DATA_4K,
    TLB_DATA_4M,
    TLB_DATA_2M_4M,
    TLB_DATA_4K_4M,
    TLB_DATA_1G,
    TLB_DATA_1G_2M_4M,
    TLB_DATA0_4K,
    TLB_DATA0_4M,
    TLB_DATA0_2M_4M,
    STLB_4K,
    STLB_4K_2M,
}

#[repr(C)]
pub union leaf_0x2_table_type {
    pub c_type: _cache_table_type,
    pub t_type: _tlb_table_type,
}

#[repr(C)]
pub union leaf_0x2_table_size {
    pub c_size: i16,
    pub entries: i16,
}

#[repr(C)]
pub struct leaf_0x2_table {
    pub type_: leaf_0x2_table_type,
    pub size: leaf_0x2_table_size,
}

extern "C" {
    pub static cpuid_0x2_table: [leaf_0x2_table; 256];
}

pub const TLB_0x63_2M_4M_ENTRIES: i32 = 32;

#[repr(C)]
pub struct leaf_parse_info {
    pub nr_entries: u32,
}

#[repr(C)]
pub struct cpuid_leaves {
    pub leaf_0x0_0: [leaf_0x0_0; 1],
    pub leaf_0x0_0_info: leaf_parse_info,
    pub leaf_0x1_0: [leaf_0x1_0; 1],
    pub leaf_0x1_0_info: leaf_parse_info,
}

#[repr(C)]
pub struct cpuid_table {
    pub leaves: cpuid_leaves,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
