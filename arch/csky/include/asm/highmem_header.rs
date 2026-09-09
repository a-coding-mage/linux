/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header's declarations are active only when building the kernel.
 * The C preprocessor condition is preserved here as source-level intent.
 */

/* undef for production */
pub const HIGHMEM_DEBUG: i32 = 1;

/* declarations for highmem.c */
extern "C" {
    pub static mut highstart_pfn: ::core::ffi::c_ulong;
    pub static mut highend_pfn: ::core::ffi::c_ulong;

    pub static mut pkmap_page_table: *mut pte_t;
}

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.
 */
pub const LAST_PKMAP: usize = 1024;
pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline(always)]
pub const unsafe fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline(always)]
pub const unsafe fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

/* ARCH_HAS_KMAP_FLUSH_TLB */
extern "C" {
    pub fn kmap_flush_tlb(addr: ::core::ffi::c_ulong);

    pub fn kmap_init();
}

#[macro_export]
macro_rules! flush_cache_kmaps {
    () => {{
    }};
}

#[macro_export]
macro_rules! arch_kmap_local_post_map {
    ($vaddr:expr, $pteval:expr) => {{
        let _ = $pteval;
        unsafe { kmap_flush_tlb($vaddr) }
    }};
}

#[macro_export]
macro_rules! arch_kmap_local_post_unmap {
    ($vaddr:expr) => {{
        unsafe { kmap_flush_tlb($vaddr) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
