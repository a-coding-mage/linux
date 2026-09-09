/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 * based on v850 version which was
 * Copyright (C) 2001,02,03 NEC Electronics Corporation
 * Copyright (C) 2001,02,03 Miles Bader <miles@gnu.org>
 */

/* Dependencies: linux/mm.h, linux/io.h, and asm-generic/cacheflush.h. */

/* Look at Documentation/core-api/cachetlb.rst */

/*
 * Cache handling functions.
 * Microblaze has a write-through data cache, meaning that the data cache
 * never needs to be flushed.  The only flushing operations that are
 * implemented are to invalidate the instruction cache.  These are called
 * after loading a user application into memory, we must invalidate the
 * instruction cache to make sure we don't fetch old, bad code.
 */

/* struct cache, d=dcache, i=icache, fl = flush, iv = invalidate,
 * suffix r = range */
#[repr(C)]
pub struct scache {
    /* icache */
    pub ie: Option<unsafe extern "C" fn()>, /* enable */
    pub id: Option<unsafe extern "C" fn()>, /* disable */
    pub ifl: Option<unsafe extern "C" fn()>, /* flush */
    pub iflr: Option<unsafe extern "C" fn(a: u64, b: u64)>,
    pub iin: Option<unsafe extern "C" fn()>, /* invalidate */
    pub iinr: Option<unsafe extern "C" fn(a: u64, b: u64)>,
    /* dcache */
    pub de: Option<unsafe extern "C" fn()>, /* enable */
    pub dd: Option<unsafe extern "C" fn()>, /* disable */
    pub dfl: Option<unsafe extern "C" fn()>, /* flush */
    pub dflr: Option<unsafe extern "C" fn(a: u64, b: u64)>,
    pub din: Option<unsafe extern "C" fn()>, /* invalidate */
    pub dinr: Option<unsafe extern "C" fn(a: u64, b: u64)>,
}

/* microblaze cache */
extern "C" {
    pub static mut mbc: *mut scache;
    pub fn microblaze_cache_init();
}

pub unsafe fn enable_icache() { ((*mbc).ie.unwrap())(); }
pub unsafe fn disable_icache() { ((*mbc).id.unwrap())(); }
pub unsafe fn flush_icache() { ((*mbc).ifl.unwrap())(); }
pub unsafe fn flush_icache_range(start: u64, end: u64) { ((*mbc).iflr.unwrap())(start, end); }
pub unsafe fn invalidate_icache() { ((*mbc).iin.unwrap())(); }
pub unsafe fn invalidate_icache_range(start: u64, end: u64) { ((*mbc).iinr.unwrap())(start, end); }

pub unsafe fn enable_dcache() { ((*mbc).de.unwrap())(); }
pub unsafe fn disable_dcache() { ((*mbc).dd.unwrap())(); }
/* FIXME for LL-temac driver */
pub unsafe fn invalidate_dcache() { ((*mbc).din.unwrap())(); }
pub unsafe fn invalidate_dcache_range(start: u64, end: u64) { ((*mbc).dinr.unwrap())(start, end); }
pub unsafe fn flush_dcache() { ((*mbc).dfl.unwrap())(); }
pub unsafe fn flush_dcache_range(start: u64, end: u64) { ((*mbc).dflr.unwrap())(start, end); }

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

/* MS: We have to implement it because of rootfs-jffs2 issue on WB */
pub unsafe fn flush_dcache_page(page: *mut page) {
    let mut addr: u64 = page_address(page) as u64; /* virtual */
    addr = virt_to_phys(addr as *mut core::ffi::c_void) as u32 as u64;
    flush_dcache_range(addr as u32 as u64, (addr as u32 as u64).wrapping_add(PAGE_SIZE as u64));
}

pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    let addr: u64 = folio_pfn(folio) << PAGE_SHIFT;
    flush_dcache_range(addr, addr.wrapping_add(folio_size(folio)));
}

pub unsafe fn flush_cache_page(_vma: *mut vm_area_struct, _vmaddr: u64, pfn: u64) {
    flush_dcache_range(pfn << PAGE_SHIFT, (pfn << PAGE_SHIFT).wrapping_add(PAGE_SIZE as u64));
}

pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: u64,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: i32,
) {
    let addr: u32 = virt_to_phys(dst) as u32;
    memcpy(dst, src, len as usize);
    if ((*vma).vm_flags & VM_EXEC) != 0 {
        invalidate_icache_range(addr as u64, (addr as u64).wrapping_add(PAGE_SIZE as u64));
        flush_dcache_range(addr as u64, (addr as u64).wrapping_add(PAGE_SIZE as u64));
    }
}

/* External declarations supplied by the included kernel headers. */
extern "C" {
    pub fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    pub fn virt_to_phys(addr: *mut core::ffi::c_void) -> u64;
    pub fn folio_pfn(folio: *mut folio) -> u64;
    pub fn folio_size(folio: *mut folio) -> u64;
    pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
}

/* Types and constants are supplied by linux/mm.h and linux/io.h. */
extern "C" {
    pub type page;
    pub type folio;
    pub type vm_area_struct;
    pub static PAGE_SIZE: u32;
    pub static PAGE_SHIFT: u32;
    pub static VM_EXEC: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
