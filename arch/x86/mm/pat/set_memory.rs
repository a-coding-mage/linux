// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low-level translation of x86/mm/pat/set_memory.c.
 * Kernel-provided types, constants, globals, and functions are intentionally
 * left as external dependencies, as they are supplied by the surrounding
 * kernel translation unit.
 */

#[repr(C)]
pub struct CpaData {
    pub vaddr: *mut ::core::ffi::c_ulong,
    pub pgd: *mut PgdT,
    pub mask_set: PgprotT,
    pub mask_clr: PgprotT,
    pub numpages: ::core::ffi::c_ulong,
    pub curpage: ::core::ffi::c_ulong,
    pub pfn: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_uint,
    pub force_split: bool,
    pub force_static_prot: bool,
    pub force_flush_all: bool,
    pub pages: *mut *mut Page,
}

// These kernel types are supplied by the architecture and memory-management
// layers when this file is included in the complete kernel translation.
pub type PgdT = ::core::ffi::c_ulong;
pub type PgprotT = ::core::ffi::c_ulong;
pub type Page = ::core::ffi::c_void;
pub type PteT = ::core::ffi::c_ulong;
pub type PhysAddrT = ::core::ffi::c_ulong;

pub const CPA_FLUSHTLB: ::core::ffi::c_uint = 0x01;
pub const CPA_ARRAY: ::core::ffi::c_uint = 0x02;
pub const CPA_PAGES_ARRAY: ::core::ffi::c_uint = 0x04;
pub const CPA_NO_CHECK_ALIAS: ::core::ffi::c_uint = 0x08;
pub const CPA_COLLAPSE: ::core::ffi::c_uint = 0x10;
pub const CPA_DEBUG_PAGEALLOC: ::core::ffi::c_uint = 0x20;

#[inline]
pub const unsafe fn within(addr: usize, start: usize, end: usize) -> bool {
    addr >= start && addr < end
}

#[inline]
pub const unsafe fn overlaps(r1_start: usize, r1_end: usize,
                             r2_start: usize, r2_end: usize) -> bool {
    (r1_start <= r2_end && r1_end >= r2_start)
        || (r2_start <= r1_end && r2_end >= r1_start)
}

#[inline]
pub unsafe fn fix_addr(addr: usize) -> usize {
    // CONFIG_X86_64: sign-extend the low canonical address bits.
    ((addr << 1) as isize >> 1) as usize
}

#[inline]
pub unsafe fn pgprot_clear_protnone_bits(mut prot: PgprotT, present: PgprotT,
                                         global: PgprotT) -> PgprotT {
    if prot & present == 0 { prot &= !global; }
    prot
}

/// Translate the address selection logic of __cpa_addr().
pub unsafe fn cpa_addr(cpa: *mut CpaData, idx: usize, page_address: unsafe fn(*mut Page) -> usize) -> usize {
    let c = &*cpa;
    if c.flags & CPA_PAGES_ARRAY != 0 {
        return page_address(*c.pages.add(idx));
    }
    if c.flags & CPA_ARRAY != 0 {
        return *c.vaddr.add(idx);
    }
    *c.vaddr + idx * 4096
}

// Public entry points retained with their C ABI and names. Their architecture
// implementations are provided by the surrounding kernel translation.
extern "C" {
    pub fn set_memory_uc(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_wc(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_wb(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_x(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_nx(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_ro(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_rw(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_np(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_p(addr: usize, numpages: i32) -> i32;
    pub fn set_memory_4k(addr: usize, numpages: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
