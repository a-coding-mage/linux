/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 ARM Ltd.
 */

/* Dependency: declarations supplied by linux/ptdump.h, linux/mm_types.h,
 * and linux/seq_file.h are expected to be available to the including crate.
 */

#[cfg(CONFIG_PTDUMP)]
#[repr(C)]
pub struct addr_marker {
    pub start_address: ::core::ffi::c_ulong,
    pub name: *mut ::core::ffi::c_char,
}

#[cfg(CONFIG_PTDUMP)]
#[repr(C)]
pub struct ptdump_info {
    pub mm: *mut mm_struct,
    pub markers: *const addr_marker,
    pub base_addr: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_PTDUMP)]
#[repr(C)]
pub struct ptdump_prot_bits {
    pub mask: ptval_t,
    pub val: ptval_t,
    pub set: *const ::core::ffi::c_char,
    pub clear: *const ::core::ffi::c_char,
}

#[cfg(CONFIG_PTDUMP)]
#[repr(C)]
pub struct ptdump_pg_level {
    pub bits: *const ptdump_prot_bits,
    pub name: [::core::ffi::c_char; 4],
    pub num: ::core::ffi::c_int,
    pub mask: ptval_t,
}

/*
 * The page dumper groups page table entries of the same type into a single
 * description. It uses pg_state to track the range information while
 * iterating over the pte entries. When the continuity is broken it then
 * dumps out a description of the range.
 */
#[cfg(CONFIG_PTDUMP)]
#[repr(C)]
pub struct ptdump_pg_state {
    pub ptdump: ptdump_state,
    pub pg_level: *mut ptdump_pg_level,
    pub seq: *mut seq_file,
    pub marker: *const addr_marker,
    pub mm: *const mm_struct,
    pub start_address: ::core::ffi::c_ulong,
    /* exclusive end, ULONG_MAX represents an end at 1 << 64 */
    pub end_address: ::core::ffi::c_ulong,
    pub level: ::core::ffi::c_int,
    pub current_prot: ptval_t,
    pub check_wx: bool,
    pub wx_pages: ::core::ffi::c_ulong,
    pub uxn_pages: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_PTDUMP)]
extern "C" {
    pub fn ptdump_walk(s: *mut seq_file, info: *mut ptdump_info);
    pub fn note_page(pt_st: *mut ptdump_state, addr: ::core::ffi::c_ulong,
                     level: ::core::ffi::c_int, val: pteval_t);
    pub fn note_page_pte(st: *mut ptdump_state, addr: ::core::ffi::c_ulong, pte: pte_t);
    pub fn note_page_pmd(st: *mut ptdump_state, addr: ::core::ffi::c_ulong, pmd: pmd_t);
    pub fn note_page_pud(st: *mut ptdump_state, addr: ::core::ffi::c_ulong, pud: pud_t);
    pub fn note_page_p4d(st: *mut ptdump_state, addr: ::core::ffi::c_ulong, p4d: p4d_t);
    pub fn note_page_pgd(st: *mut ptdump_state, addr: ::core::ffi::c_ulong, pgd: pgd_t);
    pub fn note_page_flush(st: *mut ptdump_state);
}

#[cfg(all(CONFIG_PTDUMP, CONFIG_PTDUMP_DEBUGFS))]
pub const EFI_RUNTIME_MAP_END: _ = DEFAULT_MAP_WINDOW_64;

#[cfg(all(CONFIG_PTDUMP, CONFIG_PTDUMP_DEBUGFS))]
extern "C" {
    pub fn ptdump_debugfs_register(info: *mut ptdump_info, name: *const ::core::ffi::c_char);
}

#[cfg(all(CONFIG_PTDUMP, not(CONFIG_PTDUMP_DEBUGFS)))]
#[inline]
pub unsafe fn ptdump_debugfs_register(_info: *mut ptdump_info,
                                      _name: *const ::core::ffi::c_char) {}

#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page(_pt_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong,
                        _level: ::core::ffi::c_int, _val: pteval_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_pte(_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong, _pte: pte_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_pmd(_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong, _pmd: pmd_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_pud(_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong, _pud: pud_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_p4d(_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong, _p4d: p4d_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_pgd(_st: *mut ptdump_state, _addr: ::core::ffi::c_ulong, _pgd: pgd_t) {}
#[cfg(not(CONFIG_PTDUMP))]
#[inline]
pub unsafe fn note_page_flush(_st: *mut ptdump_state) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
