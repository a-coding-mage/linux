// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/mm/flush.c
 *
 * Copyright (C) 1995-2002 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn sync_icache_aliases(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong) {
    if icache_is_aliasing() {
        dcache_clean_pou(start, end);
        icache_inval_all_pou();
    } else {
        /*
         * Don't issue kick_all_cpus_sync() after I-cache invalidation
         * for user mappings.
         */
        caches_clean_inval_pou(start, end);
    }
}

unsafe fn flush_ptrace_access(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        sync_icache_aliases(start, end);
    }
}

/*
 * Copy user data from/to a page which is mapped into a different processes
 * address space.  Really, we want to allow our "user space" model to handle
 * this.
 */
pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    _page: *mut page,
    _uaddr: ::core::ffi::c_ulong,
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    len: ::core::ffi::c_ulong,
) {
    ::core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len as usize);
    flush_ptrace_access(vma, dst as ::core::ffi::c_ulong, dst as ::core::ffi::c_ulong + len);
}

pub unsafe fn __sync_icache_dcache(pte: pte_t) {
    let folio = page_folio(pte_page(pte));

    if !test_bit(PG_dcache_clean, &mut (*folio).flags.f) {
        sync_icache_aliases(
            folio_address(folio) as ::core::ffi::c_ulong,
            folio_address(folio) as ::core::ffi::c_ulong + folio_size(folio),
        );
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

// EXPORT_SYMBOL_GPL(__sync_icache_dcache);

/*
 * This function is called when a page has been modified by the kernel. Mark
 * it as dirty for later flushing when mapped in user space (if executable,
 * see __sync_icache_dcache).
 */
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    if test_bit(PG_dcache_clean, &mut (*folio).flags.f) {
        clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

// EXPORT_SYMBOL(flush_dcache_folio);

pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

// EXPORT_SYMBOL(flush_dcache_page);

/*
 * Additional functions defined in assembly.
 */
// EXPORT_SYMBOL(caches_clean_inval_pou);

// Preserved from CONFIG_ARCH_HAS_PMEM_API; the surrounding build selects this condition.
#[cfg(CONFIG_ARCH_HAS_PMEM_API)]
pub unsafe fn arch_wb_cache_pmem(addr: *mut ::core::ffi::c_void, size: usize) {
    /* Ensure order against any prior non-cacheable writes */
    dmb(osh);
    dcache_clean_pop(addr as ::core::ffi::c_ulong, addr as ::core::ffi::c_ulong + size as ::core::ffi::c_ulong);
}

// EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);

#[cfg(CONFIG_ARCH_HAS_PMEM_API)]
pub unsafe fn arch_invalidate_pmem(addr: *mut ::core::ffi::c_void, size: usize) {
    dcache_inval_poc(addr as ::core::ffi::c_ulong, addr as ::core::ffi::c_ulong + size as ::core::ffi::c_ulong);
}

// EXPORT_SYMBOL_GPL(arch_invalidate_pmem);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
