// Translated from pte-walk.h.
// Dependency: <linux/sched.h>

/* Don't use this directly */
extern "C" {
    pub fn __find_linux_pte(
        pgdir: *mut pgd_t,
        ea: ::core::ffi::c_ulong,
        is_thp: *mut bool,
        hshift: *mut ::core::ffi::c_uint,
    ) -> *mut pte_t;
}

pub unsafe fn find_linux_pte(
    pgdir: *mut pgd_t,
    ea: ::core::ffi::c_ulong,
    is_thp: *mut bool,
    hshift: *mut ::core::ffi::c_uint,
) -> *mut pte_t {
    let pte: *mut pte_t;

    VM_WARN!(!arch_irqs_disabled(), "%s called with irq enabled\n", "find_linux_pte");
    pte = __find_linux_pte(pgdir, ea, is_thp, hshift);

    // CONFIG_DEBUG_VM && !(CONFIG_HUGETLB_PAGE || CONFIG_TRANSPARENT_HUGEPAGE)
    // We should not find huge page if these configs are not enabled.
    if hshift != ::core::ptr::null_mut() {
        WARN_ON!(*hshift);
    }

    pte
}

pub unsafe fn find_init_mm_pte(
    ea: ::core::ffi::c_ulong,
    hshift: *mut ::core::ffi::c_uint,
) -> *mut pte_t {
    let pgdir: *mut pgd_t = init_mm.pgd;
    __find_linux_pte(pgdir, ea, ::core::ptr::null_mut(), hshift)
}

/*
 * Convert a kernel vmap virtual address (vmalloc or ioremap space) to a
 * physical address, without taking locks. This can be used in real-mode.
 */
pub unsafe fn ppc_find_vmap_phys(addr: ::core::ffi::c_ulong) -> phys_addr_t {
    let ptep: *mut pte_t;
    let mut pa: phys_addr_t;
    let mut hugepage_shift: ::core::ffi::c_int;

    /*
     * init_mm does not free page tables, and does not do THP. It may
     * have huge pages from huge vmalloc / ioremap etc.
     */
    ptep = find_init_mm_pte(addr, &mut hugepage_shift as *mut _ as *mut ::core::ffi::c_uint);
    if WARN_ON!(ptep.is_null()) {
        return 0;
    }

    pa = PFN_PHYS!(pte_pfn!(*ptep));

    if hugepage_shift == 0 {
        hugepage_shift = PAGE_SHIFT;
    }

    pa |= addr & ((1 as phys_addr_t << hugepage_shift) - 1);

    pa
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
