/*
 * Copyright (C) 2003 Microtronix Datacom Ltd.
 * Copyright (C) 2000-2002 Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Translated from the NIOS2 cacheflush header. The Linux memory types and
// related symbols are supplied by other translated dependencies.

use std::os::raw::{c_int, c_uint, c_ulong, c_void};

pub const PG_dcache_clean: usize = PG_arch_1;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

extern "C" {
    pub fn flush_cache_all();
    pub fn flush_cache_mm(mm: *mut mm_struct);
    pub fn flush_cache_dup_mm(mm: *mut mm_struct);
    pub fn flush_cache_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    pub fn flush_cache_page(vma: *mut vm_area_struct, vmaddr: c_ulong, pfn: c_ulong);
    pub fn flush_dcache_page(page: *mut page);
    pub fn flush_dcache_folio(folio: *mut folio);
    pub fn flush_icache_range(start: c_ulong, end: c_ulong);
    pub fn flush_icache_pages(vma: *mut vm_area_struct, page: *mut page, nr: c_uint);
    pub fn copy_to_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        user_vaddr: c_ulong,
        dst: *mut c_void,
        src: *mut c_void,
        len: c_int,
    );
    pub fn copy_from_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        user_vaddr: c_ulong,
        dst: *mut c_void,
        src: *mut c_void,
        len: c_int,
    );
    pub fn flush_dcache_range(start: c_ulong, end: c_ulong);
    pub fn invalidate_dcache_range(start: c_ulong, end: c_ulong);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

// The following macro aliases preserve the corresponding C preprocessor names.
#[macro_export]
macro_rules! flush_dcache_folio {
    ($folio:expr) => { flush_dcache_folio($folio) };
}

#[macro_export]
macro_rules! flush_icache_pages {
    ($vma:expr, $page:expr, $nr:expr) => { flush_icache_pages($vma, $page, $nr) };
}

#[macro_export]
macro_rules! flush_cache_vmap {
    ($start:expr, $end:expr) => { flush_dcache_range($start, $end) };
}

#[macro_export]
macro_rules! flush_cache_vmap_early {
    ($start:expr, $end:expr) => {{ let _ = ($start, $end); }};
}

#[macro_export]
macro_rules! flush_cache_vunmap {
    ($start:expr, $end:expr) => { flush_dcache_range($start, $end) };
}

#[macro_export]
macro_rules! flush_dcache_mmap_lock {
    ($mapping:expr) => { xa_lock_irq(&($mapping).i_pages) };
}

#[macro_export]
macro_rules! flush_dcache_mmap_unlock {
    ($mapping:expr) => { xa_unlock_irq(&($mapping).i_pages) };
}

#[macro_export]
macro_rules! flush_dcache_mmap_lock_irqsave {
    ($mapping:expr, $flags:expr) => { xa_lock_irqsave(&($mapping).i_pages, $flags) };
}

#[macro_export]
macro_rules! flush_dcache_mmap_unlock_irqrestore {
    ($mapping:expr, $flags:expr) => { xa_unlock_irqrestore(&($mapping).i_pages, $flags) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
