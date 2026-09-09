/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Synopsys, Inc. (www.synopsys.com)
 */

// C header guard: _ASM_HIGHMEM_H
// The following declarations are present only when CONFIG_HIGHMEM is enabled.
#[cfg(feature = "CONFIG_HIGHMEM")]
mod config_highmem {
    // C dependencies: <uapi/asm/page.h>, <asm/kmap_size.h>, and
    // <asm/cacheflush.h> provide these constants and functions.

    pub const FIXMAP_SIZE: usize = PGDIR_SIZE;
    pub const PKMAP_SIZE: usize = PGDIR_SIZE;

    /* start after vmalloc area */
    pub const FIXMAP_BASE: usize = PAGE_OFFSET - FIXMAP_SIZE - PKMAP_SIZE;

    pub const FIX_KMAP_SLOTS: usize = KM_MAX_IDX * NR_CPUS;
    pub const FIX_KMAP_BEGIN: usize = 0usize;
    pub const FIX_KMAP_END: usize = (FIX_KMAP_BEGIN + FIX_KMAP_SLOTS) - 1;

    pub const FIXADDR_TOP: usize = FIXMAP_BASE + (FIX_KMAP_END << PAGE_SHIFT);

    /*
     * This should be converted to the asm-generic version, but of course this
     * is needlessly different from all other architectures. Sigh - tglx
     */
    #[inline]
    pub const fn __fix_to_virt(x: usize) -> usize {
        FIXADDR_TOP - (x << PAGE_SHIFT)
    }

    #[inline]
    pub const fn __virt_to_fix(x: usize) -> usize {
        (FIXADDR_TOP - (x & PAGE_MASK)) >> PAGE_SHIFT
    }

    /* start after fixmap area */
    pub const PKMAP_BASE: usize = FIXMAP_BASE + FIXMAP_SIZE;
    pub const LAST_PKMAP: usize = PKMAP_SIZE >> PAGE_SHIFT;
    pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

    #[inline]
    pub const fn PKMAP_ADDR(nr: usize) -> usize {
        PKMAP_BASE + (nr << PAGE_SHIFT)
    }

    #[inline]
    pub const fn PKMAP_NR(virt: usize) -> usize {
        (virt - PKMAP_BASE) >> PAGE_SHIFT
    }

    unsafe extern "C" {
        pub fn kmap_init();
        pub fn local_flush_tlb_kernel_range(vaddr_start: usize, vaddr_end: usize);
        pub fn flush_cache_all();
    }

    #[inline]
    pub unsafe fn arch_kmap_local_post_unmap(vaddr: usize) {
        local_flush_tlb_kernel_range(vaddr, vaddr + PAGE_SIZE);
    }

    #[inline]
    pub unsafe fn flush_cache_kmaps() {
        flush_cache_all();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
