// SPDX-License-Identifier: GPL-2.0-only
/*
 * PGD allocation/freeing
 *
 * Copyright (C) 2012 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 */

// Dependencies supplied by the corresponding Linux and ARM headers:
// linux/mm.h, linux/gfp.h, linux/highmem.h, linux/slab.h,
// asm/pgalloc.h, asm/page.h, asm/tlbflush.h

static mut pgd_cache: *mut kmem_cache = core::ptr::null_mut(); // __ro_after_init

unsafe fn pgdir_is_page_size() -> bool {
    if PGD_SIZE == PAGE_SIZE {
        return true;
    }
    if CONFIG_PGTABLE_LEVELS == 4 {
        return !pgtable_l4_enabled();
    }
    if CONFIG_PGTABLE_LEVELS == 5 {
        return !pgtable_l5_enabled();
    }
    false
}

unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let gfp: gfp_t = GFP_PGTABLE_USER;

    if pgdir_is_page_size() {
        __pgd_alloc(mm, 0)
    } else {
        kmem_cache_alloc(pgd_cache, gfp)
    }
}

unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    if pgdir_is_page_size() {
        __pgd_free(mm, pgd);
    } else {
        kmem_cache_free(pgd_cache, pgd);
    }
}

unsafe fn pgtable_cache_init() {
    if pgdir_is_page_size() {
        return;
    }

    // CONFIG_ARM64_PA_BITS_52
    // With 52-bit physical addresses, the architecture requires the
    // top-level table to be aligned to at least 64 bytes.
    #[cfg(CONFIG_ARM64_PA_BITS_52)]
    {
        BUILD_BUG_ON(!IS_ALIGNED(PGD_SIZE, 64));
    }

    // Naturally aligned pgds required by the architecture.
    pgd_cache = kmem_cache_create(
        b"pgd_cache\0".as_ptr() as *const core::ffi::c_char,
        PGD_SIZE,
        PGD_SIZE,
        SLAB_PANIC,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
