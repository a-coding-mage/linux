/* SPDX-License-Identifier: GPL-2.0 */

/* Generic cache-flush interfaces.  C preprocessor conditions are represented
 * by the unconditional generic declarations supplied by this header. */

use core::ffi::c_void;

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
pub struct address_space {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn flush_cache_all() {}

#[inline]
pub unsafe fn flush_cache_mm(_mm: *mut mm_struct) {}

#[inline]
pub unsafe fn flush_cache_dup_mm(_mm: *mut mm_struct) {}

#[inline]
pub unsafe fn flush_cache_range(
    _vma: *mut vm_area_struct,
    _start: c_ulong,
    _end: c_ulong,
) {
}

#[inline]
pub unsafe fn flush_cache_page(
    _vma: *mut vm_area_struct,
    _vmaddr: c_ulong,
    _pfn: c_ulong,
) {
}

#[inline]
pub unsafe fn flush_dcache_page(_page: *mut page) {}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 0;

#[inline]
pub unsafe fn flush_dcache_mmap_lock(_mapping: *mut address_space) {}

#[inline]
pub unsafe fn flush_dcache_mmap_unlock(_mapping: *mut address_space) {}

#[inline]
pub unsafe fn flush_icache_range(_start: c_ulong, _end: c_ulong) {}

/* C macro alias: flush_icache_user_range expands to flush_icache_range. */
#[inline]
pub unsafe fn flush_icache_user_range(start: c_ulong, end: c_ulong) {
    flush_icache_range(start, end);
}

#[inline]
pub unsafe fn flush_icache_user_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _addr: c_ulong,
    _len: i32,
) {
}

#[inline]
pub unsafe fn flush_cache_vmap(_start: c_ulong, _end: c_ulong) {}

#[inline]
pub unsafe fn flush_cache_vmap_early(_start: c_ulong, _end: c_ulong) {}

#[inline]
pub unsafe fn flush_cache_vunmap(_start: c_ulong, _end: c_ulong) {}

extern "C" {
    fn instrument_copy_to_user(to: *mut c_void, from: *const c_void, len: usize);
    fn instrument_copy_from_user_before(to: *mut c_void, from: *const c_void, len: usize);
    fn instrument_copy_from_user_after(
        to: *mut c_void,
        from: *const c_void,
        len: usize,
        left: usize,
    );
}

/* Translation of copy_to_user_page(vma, page, vaddr, dst, src, len). */
#[inline]
pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    page: *mut page,
    vaddr: c_ulong,
    dst: *mut c_void,
    src: *const c_void,
    len: usize,
) {
    instrument_copy_to_user(dst, src, len);
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    flush_icache_user_page(vma, page, vaddr, len as i32);
}

/* Translation of copy_from_user_page(vma, page, vaddr, dst, src, len). */
#[inline]
pub unsafe fn copy_from_user_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: c_ulong,
    dst: *mut c_void,
    src: *const c_void,
    len: usize,
) {
    instrument_copy_from_user_before(dst, src, len);
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    instrument_copy_from_user_after(dst, src, len, 0);
}

type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
