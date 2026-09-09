// Translated from arm64/include/asm/vmalloc.h.
// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not implemented in this header.

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub fn arch_vmap_pud_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    unsafe { pud_sect_supported() }
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub fn arch_vmap_pmd_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    true
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub fn arch_vmap_pte_range_map_size(
    addr: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
    pfn: u64,
    max_page_shift: ::core::ffi::c_uint,
) -> ::core::ffi::c_ulong {
    /*
     * If the block is at least CONT_PTE_SIZE in size, and is naturally
     * aligned in both virtual and physical space, then we can pte-map the
     * block using the PTE_CONT bit for more efficient use of the TLB.
     */
    if max_page_shift < CONT_PTE_SHIFT {
        return PAGE_SIZE;
    }

    if end.wrapping_sub(addr) < CONT_PTE_SIZE {
        return PAGE_SIZE;
    }

    if !IS_ALIGNED(addr, CONT_PTE_SIZE) {
        return PAGE_SIZE;
    }

    if !IS_ALIGNED(PFN_PHYS(pfn), CONT_PTE_SIZE) {
        return PAGE_SIZE;
    }

    CONT_PTE_SIZE
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub unsafe fn arch_vmap_pte_range_unmap_size(
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> ::core::ffi::c_ulong {
    /*
     * The caller handles alignment so it's sufficient just to check
     * PTE_CONT.
     */
    let _ = addr;
    if pte_valid_cont(__ptep_get(ptep)) {
        CONT_PTE_SIZE
    } else {
        PAGE_SIZE
    }
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub fn arch_vmap_pte_supported_shift(size: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    if size >= CONT_PTE_SIZE {
        CONT_PTE_SHIFT
    } else {
        PAGE_SHIFT
    }
}

#[inline]
pub fn arch_vmap_pgprot_tagged(prot: pgprot_t) -> pgprot_t {
    pgprot_tagged(prot)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
