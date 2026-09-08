/* SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause) */
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 *
 * Rust translation of libfdt_internal.h.  Names supplied by fdt.h and other
 * libfdt translation units are intentionally left as external dependencies.
 */

#[allow(improper_ctypes)]
extern "C" {
    pub fn fdt_ro_probe_(fdt: *const core::ffi::c_void) -> i32;
    pub fn fdt_check_node_offset_(fdt: *const core::ffi::c_void, offset: i32) -> i32;
    pub fn fdt_check_prop_offset_(fdt: *const core::ffi::c_void, offset: i32) -> i32;
    pub fn fdt_find_string_len_(
        strtab: *const core::ffi::c_char,
        tabsize: i32,
        s: *const core::ffi::c_char,
        s_len: usize,
    ) -> *const core::ffi::c_char;
    pub fn fdt_node_end_offset_(fdt: *mut core::ffi::c_void, nodeoffset: i32) -> i32;
    pub fn strlen(s: *const core::ffi::c_char) -> usize;
}

#[inline]
pub unsafe fn fdt_find_string_(
    strtab: *const core::ffi::c_char,
    tabsize: i32,
    s: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    fdt_find_string_len_(strtab, tabsize, s, strlen(s))
}

#[inline]
pub unsafe fn fdt_offset_ptr_(fdt: *const core::ffi::c_void, offset: isize) -> *const core::ffi::c_char {
    (fdt as *const core::ffi::c_char)
        .add(fdt_off_dt_struct(fdt) as usize)
        .offset(offset)
}

#[inline]
pub unsafe fn fdt_offset_ptr_w_(fdt: *mut core::ffi::c_void, offset: isize) -> *mut core::ffi::c_void {
    fdt_offset_ptr_(fdt as *const core::ffi::c_void, offset) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn fdt_mem_rsv_(fdt: *const core::ffi::c_void, n: isize) -> *const fdt_reserve_entry {
    let rsv_table = (fdt as *const core::ffi::c_char)
        .add(fdt_off_mem_rsvmap(fdt) as usize) as *const fdt_reserve_entry;
    rsv_table.offset(n)
}

#[inline]
pub unsafe fn fdt_mem_rsv_w_(fdt: *mut core::ffi::c_void, n: isize) -> *mut fdt_reserve_entry {
    fdt_mem_rsv_(fdt as *const core::ffi::c_void, n) as *mut fdt_reserve_entry
}

/* Structural accesses assume naturally aligned or gracefully unaligned data. */
#[inline]
pub unsafe fn fdt32_ld_(p: *const fdt32_t) -> u32 { fdt32_to_cpu(*p) }

#[inline]
pub unsafe fn fdt64_ld_(p: *const fdt64_t) -> u64 { fdt64_to_cpu(*p) }

pub const FDT_SW_MAGIC: u32 = !FDT_MAGIC;

pub const FDT_ASSUME_MASK: i32 = 0;

pub const ASSUME_PERFECT: i32 = 0xff;
pub const ASSUME_VALID_DTB: i32 = 1 << 0;
pub const ASSUME_VALID_INPUT: i32 = 1 << 1;
pub const ASSUME_LATEST: i32 = 1 << 2;
pub const ASSUME_NO_ROLLBACK: i32 = 1 << 3;
pub const ASSUME_LIBFDT_ORDER: i32 = 1 << 4;
pub const ASSUME_LIBFDT_FLAWLESS: i32 = 1 << 5;

#[inline]
pub const fn can_assume_(mask: i32) -> bool {
    (FDT_ASSUME_MASK & mask) != 0
}

/* The C FDT_RO_PROBE macro performs an early return from its caller. */
#[macro_export]
macro_rules! FDT_RO_PROBE {
    ($fdt:expr) => {{
        if !$crate::can_assume_(ASSUME_VALID_DTB) {
            let totalsize_ = unsafe { $crate::fdt_ro_probe_($fdt) };
            if totalsize_ < 0 { return totalsize_; }
        }
    }};
}

#[macro_export]
macro_rules! FDT_ALIGN { ($x:expr, $a:expr) => {
    (($x + $a - 1) & !($a - 1))
} }

#[macro_export]
macro_rules! FDT_TAGALIGN { ($x:expr) => { FDT_ALIGN!($x, FDT_TAGSIZE) } }

extern "C" {
    fn fdt_off_dt_struct(fdt: *const core::ffi::c_void) -> u32;
    fn fdt_off_mem_rsvmap(fdt: *const core::ffi::c_void) -> u32;
    fn fdt32_to_cpu(value: fdt32_t) -> u32;
    fn fdt64_to_cpu(value: fdt64_t) -> u64;
}

/* fdt32_t, fdt64_t, fdt_reserve_entry, FDT_MAGIC, and FDT_TAGSIZE are
 * supplied by the translated fdt.h dependency. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
