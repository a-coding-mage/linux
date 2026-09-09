/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 *  vineetg: May 2011: for Non-aliasing VIPT D-cache following can be NOPs
 *   -flush_cache_dup_mm (fork)
 *   -likewise for flush_cache_mm (exit/execve)
 *   -likewise for flush_cache_{range,page} (munmap, exit, COW-break)
 *
 *  vineetg: April 2008
 *   -Added a critical CacheLine flush to copy_to_user_page( ) which
 *     was causing gdbserver to not setup breakpoints consistently
 */

// Dependencies supplied by the surrounding translation unit:
// linux/mm.h and asm/shmparam.h

unsafe extern "C" {
    pub fn flush_cache_all();

    pub fn flush_icache_range(kstart: c_ulong, kend: c_ulong);
    pub fn __sync_icache_dcache(paddr: phys_addr_t, vaddr: c_ulong, len: c_int);
    pub fn __inv_icache_pages(paddr: phys_addr_t, vaddr: c_ulong, nr: c_uint);
    pub fn __flush_dcache_pages(paddr: phys_addr_t, vaddr: c_ulong, nr: c_uint);

    pub fn flush_dcache_page(page: *mut page);
    pub fn flush_dcache_folio(folio: *mut folio);

    pub fn dma_cache_wback_inv(start: phys_addr_t, sz: c_ulong);
    pub fn dma_cache_inv(start: phys_addr_t, sz: c_ulong);
    pub fn dma_cache_wback(start: phys_addr_t, sz: c_ulong);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

// C compatibility alias: flush_dcache_folio refers to the declaration above.

macro_rules! flush_dcache_mmap_lock {
    ($mapping:expr) => {{ let _ = &$mapping; }};
}

macro_rules! flush_dcache_mmap_unlock {
    ($mapping:expr) => {{ let _ = &$mapping; }};
}

/* TBD: optimize this */
macro_rules! flush_cache_vmap {
    ($start:expr, $end:expr) => {{ let _ = (&$start, &$end); unsafe { flush_cache_all() } }};
}

macro_rules! flush_cache_vmap_early {
    ($start:expr, $end:expr) => {{ let _ = (&$start, &$end); }};
}

macro_rules! flush_cache_vunmap {
    ($start:expr, $end:expr) => {{ let _ = (&$start, &$end); unsafe { flush_cache_all() } }};
}

macro_rules! flush_cache_dup_mm {
    ($mm:expr) => {{ let _ = &$mm; }};
}

macro_rules! flush_cache_mm {
    ($mm:expr) => {{ let _ = &$mm; }};
}

macro_rules! flush_cache_range {
    ($mm:expr, $u_vstart:expr, $u_vend:expr) => {{ let _ = (&$mm, &$u_vstart, &$u_vend); }};
}

macro_rules! flush_cache_page {
    ($vma:expr, $u_vaddr:expr, $pfn:expr) => {{ let _ = (&$vma, &$u_vaddr, &$pfn); }};
}

/*
 * A new pagecache page has PG_arch_1 clear - thus dcache dirty by default
 * This works around some PIO based drivers which don't call flush_dcache_page
 * to record that they dirtied the dcache
 */
pub const PG_dc_clean: _ = PG_arch_1;

macro_rules! copy_to_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {{
        unsafe { memcpy($dst, $src, $len); }
        if ($vma).vm_flags & VM_EXEC != 0 {
            unsafe { __sync_icache_dcache(($dst) as c_ulong, $vaddr, $len); }
        }
    }};
}

macro_rules! copy_from_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {{
        unsafe { memcpy($dst, $src, $len); }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
