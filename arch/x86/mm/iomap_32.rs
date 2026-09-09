// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright © 2008 Ingo Molnar
 */

// Dependencies supplied by the corresponding architecture and kernel modules
// are intentionally left external, as in the original C translation unit.

unsafe fn is_io_mapping_possible(base: resource_size_t, size: libc::c_ulong) -> libc::c_int {
    // There is no way to map greater than 1 << 32 address without PAE.
    // This preserves the source conditional for builds with 64-bit physical
    // addresses and without x86 PAE.
    #[cfg(all(not(CONFIG_X86_PAE), CONFIG_PHYS_ADDR_T_64BIT))]
    {
        if base.wrapping_add(size as resource_size_t) > 0x1_0000_0000u64 as resource_size_t {
            return 0;
        }
    }
    1
}

pub unsafe fn iomap_create_wc(
    base: resource_size_t,
    size: libc::c_ulong,
    prot: *mut pgprot_t,
) -> libc::c_int {
    let mut pcm: page_cache_mode = _PAGE_CACHE_MODE_WC;
    let ret: libc::c_int;

    if is_io_mapping_possible(base, size) == 0 {
        return -EINVAL;
    }

    ret = memtype_reserve_io(
        base,
        base.wrapping_add(size as resource_size_t),
        &mut pcm,
    );
    if ret != 0 {
        return ret;
    }

    *prot = __pgprot(__PAGE_KERNEL | cachemode2protval(pcm));
    // Filter out unsupported __PAGE_KERNEL* bits:
    *prot = pgprot_t(pgprot_val(*prot) & __default_kernel_pte_mask);

    0
}

// EXPORT_SYMBOL_GPL(iomap_create_wc);

pub unsafe fn iomap_free(base: resource_size_t, size: libc::c_ulong) {
    memtype_free_io(base, base.wrapping_add(size as resource_size_t));
}

// EXPORT_SYMBOL_GPL(iomap_free);

pub unsafe fn __iomap_local_pfn_prot(pfn: libc::c_ulong, mut prot: pgprot_t) -> *mut core::ffi::c_void {
    /*
     * For non-PAT systems, translate non-WB request to UC- just in
     * case the caller set the PWT bit to prot directly without using
     * pgprot_writecombine(). UC- translates to uncached if the MTRR
     * is UC or WC. UC- gets the real intention, of the user, which is
     * "WC if the MTRR is WC, UC if you can't do that."
     */
    if !pat_enabled() && pgprot2cachemode(prot) != _PAGE_CACHE_MODE_WB {
        prot = __pgprot(__PAGE_KERNEL | cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS));
    }

    // Filter out unsupported __PAGE_KERNEL* bits:
    prot = pgprot_t(pgprot_val(prot) & __default_kernel_pte_mask);

    __kmap_local_pfn_prot(pfn, prot) as *mut core::ffi::c_void
}

// EXPORT_SYMBOL_GPL(__iomap_local_pfn_prot);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
