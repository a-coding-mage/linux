/* SPDX-License-Identifier: GPL-2.0 */
// Translated from sparc/include/asm/cacheflush_32.h.
// The included kernel declarations are supplied by other translation units.

#[allow(non_camel_case_types)]
pub struct page;
#[allow(non_camel_case_types)]
pub struct folio;

#[macro_export]
macro_rules! flush_cache_all {
    () => {{ unsafe { sparc32_cachetlb_ops.cache_all() } }};
}

#[macro_export]
macro_rules! flush_cache_mm {
    ($mm:expr) => {{ unsafe { sparc32_cachetlb_ops.cache_mm($mm) } }};
}

#[macro_export]
macro_rules! flush_cache_dup_mm {
    ($mm:expr) => {{ unsafe { sparc32_cachetlb_ops.cache_mm($mm) } }};
}

#[macro_export]
macro_rules! flush_cache_range {
    ($vma:expr, $start:expr, $end:expr) => {{
        unsafe { sparc32_cachetlb_ops.cache_range($vma, $start, $end) }
    }};
}

#[macro_export]
macro_rules! flush_cache_page {
    ($vma:expr, $addr:expr, $pfn:expr) => {{
        unsafe { sparc32_cachetlb_ops.cache_page($vma, $addr) }
    }};
}

#[macro_export]
macro_rules! flush_icache_range {
    ($start:expr, $end:expr) => {{
        let _ = ($start, $end);
    }};
}

#[macro_export]
macro_rules! copy_to_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {{
        $crate::flush_cache_page!($vma, $vaddr, unsafe { page_to_pfn($page) });
        unsafe { memcpy($dst, $src, $len) };
    }};
}

#[macro_export]
macro_rules! copy_from_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {{
        $crate::flush_cache_page!($vma, $vaddr, unsafe { page_to_pfn($page) });
        unsafe { memcpy($dst, $src, $len) };
    }};
}

#[macro_export]
macro_rules! __flush_page_to_ram {
    ($addr:expr) => {{ unsafe { sparc32_cachetlb_ops.page_to_ram($addr) } }};
}

#[macro_export]
macro_rules! flush_sig_insns {
    ($mm:expr, $insn_addr:expr) => {{
        unsafe { sparc32_cachetlb_ops.sig_insns($mm, $insn_addr) }
    }};
}

#[macro_export]
macro_rules! flush_page_for_dma {
    ($addr:expr) => {{ unsafe { sparc32_cachetlb_ops.page_for_dma($addr) } }};
}

unsafe extern "C" {
    pub fn sparc_flush_page_to_ram(page: *mut page);
    pub fn sparc_flush_folio_to_ram(folio: *mut folio);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[macro_export]
macro_rules! flush_dcache_folio {
    ($folio:expr) => {{ unsafe { sparc_flush_folio_to_ram($folio) } }};
}

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio!(page_folio(page));
}

#[macro_export]
macro_rules! flush_dcache_mmap_lock {
    ($mapping:expr) => {{ let _ = $mapping; }};
}

#[macro_export]
macro_rules! flush_dcache_mmap_unlock {
    ($mapping:expr) => {{ let _ = $mapping; }};
}

#[macro_export]
macro_rules! flush_cache_vmap {
    ($start:expr, $end:expr) => {{ $crate::flush_cache_all!() }};
}

#[macro_export]
macro_rules! flush_cache_vmap_early {
    ($start:expr, $end:expr) => {{ let _ = ($start, $end); }};
}

#[macro_export]
macro_rules! flush_cache_vunmap {
    ($start:expr, $end:expr) => {{ $crate::flush_cache_all!() }};
}

/* When a context switch happens we must flush all user windows so that
 * the windows of the current process are flushed onto its stack. This
 * way the windows are all clean for the next process and the stack
 * frames are up to date.
 */
unsafe extern "C" {
    pub fn flush_user_windows();
    pub fn kill_user_windows();
    pub fn flushw_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
