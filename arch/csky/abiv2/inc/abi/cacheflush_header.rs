/* SPDX-License-Identifier: GPL-2.0 */

/* Keep includes the same across arches. */
/* Dependency intent: definitions from <linux/mm.h> are supplied externally. */

/*
 * The cache doesn't need to be flushed when TLB entries change when
 * the cache is mapped to physical memory, not virtual memory
 */
#[macro_export]
macro_rules! flush_cache_all { () => {}; }
#[macro_export]
macro_rules! flush_cache_mm { ($mm:expr) => {}; }
#[macro_export]
macro_rules! flush_cache_dup_mm { ($mm:expr) => {}; }
#[macro_export]
macro_rules! flush_cache_range { ($vma:expr, $start:expr, $end:expr) => {}; }
#[macro_export]
macro_rules! flush_cache_page { ($vma:expr, $vmaddr:expr, $pfn:expr) => {}; }

pub const PG_dcache_clean: usize = PG_arch_1;

#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    if test_bit(PG_dcache_clean, &(*folio).flags.f) != 0 {
        clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

/* Compatibility macro corresponding to the C self-referential macro. */
#[macro_export]
macro_rules! flush_dcache_folio { ($folio:expr) => { $crate::flush_dcache_folio($folio) }; }

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

#[macro_export]
macro_rules! flush_dcache_mmap_lock { ($mapping:expr) => {}; }
#[macro_export]
macro_rules! flush_dcache_mmap_unlock { ($mapping:expr) => {}; }

#[macro_export]
macro_rules! flush_icache_range { ($start:expr, $end:expr) => { cache_wbinv_range($start, $end) }; }

extern "C" {
    pub fn flush_icache_mm_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong);
    pub fn flush_icache_deferred(mm: *mut mm_struct);
}

#[macro_export]
macro_rules! flush_cache_vmap { ($start:expr, $end:expr) => {}; }
#[macro_export]
macro_rules! flush_cache_vmap_early { ($start:expr, $end:expr) => {}; }
#[macro_export]
macro_rules! flush_cache_vunmap { ($start:expr, $end:expr) => {}; }

#[macro_export]
macro_rules! copy_to_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {{
        core::ptr::copy_nonoverlapping($src as *const u8, $dst as *mut u8, $len);
        if ($vma).vm_flags & VM_EXEC != 0 {
            dcache_wb_range($dst as c_ulong, $dst as c_ulong + $len as c_ulong);
            flush_icache_mm_range((*current).mm, $dst as c_ulong,
                                  $dst as c_ulong + $len as c_ulong);
        }
    }};
}

#[macro_export]
macro_rules! copy_from_user_page {
    ($vma:expr, $page:expr, $vaddr:expr, $dst:expr, $src:expr, $len:expr) => {
        core::ptr::copy_nonoverlapping($src as *const u8, $dst as *mut u8, $len)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
