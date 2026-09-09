/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 95, 96, 97, 98, 99, 2000, 01, 02, 03 by Ralf Baechle
 * Copyright (C) 1999, 2000, 2001 Silicon Graphics, Inc.
 */

/* Keep includes the same across arches.  C dependencies are supplied externally. */

/* Cache flushing declarations and MIPS-specific flush operations. */

/* This flag indicates that the page pointed to by a pte is dirty and requires
 * cleaning before returning it to the user.
 */
pub const PG_dcache_dirty: _ = PG_arch_1;

#[inline]
pub unsafe fn folio_test_dcache_dirty(folio: *const folio) -> bool {
    test_bit(PG_dcache_dirty, &(*folio).flags.f)
}

#[inline]
pub unsafe fn folio_set_dcache_dirty(folio: *mut folio) {
    set_bit(PG_dcache_dirty, &mut (*folio).flags.f);
}

#[inline]
pub unsafe fn folio_clear_dcache_dirty(folio: *mut folio) {
    clear_bit(PG_dcache_dirty, &mut (*folio).flags.f);
}

extern "C" {
    pub static mut flush_cache_all: Option<unsafe extern "C" fn()>;
    pub static mut __flush_cache_all: Option<unsafe extern "C" fn()>;
    pub static mut flush_cache_mm: Option<unsafe extern "C" fn(mm: *mut mm_struct)>;
    pub static mut flush_cache_range:
        Option<unsafe extern "C" fn(vma: *mut vm_area_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong)>;
    pub static mut flush_cache_page:
        Option<unsafe extern "C" fn(vma: *mut vm_area_struct, page: ::core::ffi::c_ulong, pfn: ::core::ffi::c_ulong)>;
    pub fn __flush_dcache_folio_pages(folio: *mut folio, page: *mut page, nr: ::core::ffi::c_uint);
    pub static mut flush_icache_range: Option<unsafe extern "C" fn(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong)>;
    pub static mut local_flush_icache_range: Option<unsafe extern "C" fn(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong)>;
    pub static mut __flush_icache_user_range: Option<unsafe extern "C" fn(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong)>;
    pub static mut __local_flush_icache_user_range: Option<unsafe extern "C" fn(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong)>;
    pub static mut __flush_cache_vmap: Option<unsafe extern "C" fn()>;
    pub static mut __flush_cache_vunmap: Option<unsafe extern "C" fn()>;
    pub fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: ::core::ffi::c_ulong, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, len: ::core::ffi::c_ulong);
    pub fn copy_from_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: ::core::ffi::c_ulong, dst: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, len: ::core::ffi::c_ulong);
    pub static mut flush_icache_all: Option<unsafe extern "C" fn()>;
    pub static mut flush_data_cache_page: Option<unsafe extern "C" fn(addr: ::core::ffi::c_ulong)>;
    pub fn run_uncached(func: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    pub fn kmap_coherent(page: *mut page, addr: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn kunmap_coherent();
    pub fn kmap_noncoherent(page: *mut page, addr: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub static mut __flush_kernel_vmap_range: Option<unsafe extern "C" fn(vaddr: ::core::ffi::c_ulong, size: ::core::ffi::c_int)>;
    pub fn __flush_anon_page(page: *mut page, vmaddr: ::core::ffi::c_ulong);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    if cpu_has_dc_aliases {
        __flush_dcache_folio_pages(folio, folio_page(folio, 0), folio_nr_pages(folio));
    } else if !cpu_has_ic_fills_f_dc {
        folio_set_dcache_dirty(folio);
    }
}

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    let folio = page_folio(page);
    if cpu_has_dc_aliases {
        __flush_dcache_folio_pages(folio, page, 1);
    } else if !cpu_has_ic_fills_f_dc {
        folio_set_dcache_dirty(folio);
    }
}

pub const ARCH_HAS_FLUSH_ANON_PAGE: bool = true;

#[inline]
pub unsafe fn flush_anon_page(_vma: *mut vm_area_struct, page: *mut page, vmaddr: ::core::ffi::c_ulong) {
    if cpu_has_dc_aliases && PageAnon(page) {
        __flush_anon_page(page, vmaddr);
    }
}

#[inline]
pub unsafe fn flush_cache_vmap(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {
    if cpu_has_dc_aliases {
        if let Some(f) = __flush_cache_vmap { f(); }
    }
}

#[inline]
pub unsafe fn flush_cache_vunmap(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {
    if cpu_has_dc_aliases {
        if let Some(f) = __flush_cache_vunmap { f(); }
    }
}

#[inline]
pub unsafe fn flush_cache_dup_mm<T>(_mm: T) {}
#[inline]
pub unsafe fn flush_dcache_mmap_lock<T>(_mapping: T) {}
#[inline]
pub unsafe fn flush_dcache_mmap_unlock<T>(_mapping: T) {}
#[inline]
pub unsafe fn flush_cache_vmap_early<T, U>(_start: T, _end: U) {}

#[inline]
pub unsafe fn kunmap_noncoherent() { kunmap_coherent(); }

pub const ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE: i32 = 1;

/* For now both operations write back and invalidate the cache. */
#[inline]
pub unsafe fn flush_kernel_vmap_range(vaddr: *mut ::core::ffi::c_void, size: ::core::ffi::c_int) {
    if cpu_has_dc_aliases {
        if let Some(f) = __flush_kernel_vmap_range { f(vaddr as ::core::ffi::c_ulong, size); }
    }
}

#[inline]
pub unsafe fn invalidate_kernel_vmap_range(vaddr: *mut ::core::ffi::c_void, size: ::core::ffi::c_int) {
    if cpu_has_dc_aliases {
        if let Some(f) = __flush_kernel_vmap_range { f(vaddr as ::core::ffi::c_ulong, size); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
