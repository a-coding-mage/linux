/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

/*
 * For radix we want generic code to handle hugetlb. But then if we want
 * both hash and radix to be enabled together we need to workaround the
 * limitations.
 */
unsafe extern "C" {
    pub fn radix__flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
    pub fn radix__local_flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
    pub fn radix__huge_ptep_modify_prot_commit(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        old_pte: pte_t,
        pte: pte_t,
    );
}

#[inline]
pub unsafe fn hstate_get_psize(hstate: *mut hstate) -> c_int {
    let shift: c_ulong;

    shift = huge_page_shift(hstate);
    if shift == mmu_psize_defs[MMU_PAGE_2M].shift {
        MMU_PAGE_2M
    } else if shift == mmu_psize_defs[MMU_PAGE_1G].shift {
        MMU_PAGE_1G
    } else if shift == mmu_psize_defs[MMU_PAGE_16M].shift {
        MMU_PAGE_16M
    } else if shift == mmu_psize_defs[MMU_PAGE_16G].shift {
        MMU_PAGE_16G
    } else {
        WARN(1, "Wrong huge page shift\n");
        mmu_virtual_psize
    }
}

/* __HAVE_ARCH_GIGANTIC_PAGE_RUNTIME_SUPPORTED */
#[inline]
pub unsafe fn gigantic_page_runtime_supported() -> bool {
    /*
     * We used gigantic page reservation with hypervisor assist in some case.
     * We cannot use runtime allocation of gigantic pages in those platforms
     * This is hash translation mode LPARs.
     */
    if firmware_has_feature(FW_FEATURE_LPAR) && !radix_enabled() {
        return false;
    }

    true
}

/* huge_ptep_modify_prot_start huge_ptep_modify_prot_start */
unsafe extern "C" {
    pub fn huge_ptep_modify_prot_start(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;
}

/* huge_ptep_modify_prot_commit huge_ptep_modify_prot_commit */
unsafe extern "C" {
    pub fn huge_ptep_modify_prot_commit(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        old_pte: pte_t,
        new_pte: pte_t,
    );
}

#[inline]
pub unsafe fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    if radix_enabled() {
        return radix__flush_hugetlb_page(vma, vmaddr);
    }
}

unsafe extern "C" {
    pub fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
}

#[inline]
pub unsafe fn check_and_get_huge_psize(shift: c_int) -> c_int {
    let mmu_psize: c_int;

    if shift > SLICE_HIGH_SHIFT {
        return -EINVAL;
    }

    mmu_psize = shift_to_mmu_psize(shift);

    /*
     * We need to make sure that for different page sizes reported by
     * firmware we only add hugetlb support for page sizes that can be
     * supported by linux page table layout.
     * For now we have
     * Radix: 2M and 1G
     * Hash: 16M and 16G
     */
    if radix_enabled() {
        if mmu_psize != MMU_PAGE_2M && mmu_psize != MMU_PAGE_1G {
            return -EINVAL;
        }
    } else if mmu_psize != MMU_PAGE_16M && mmu_psize != MMU_PAGE_16G {
        return -EINVAL;
    }
    mmu_psize
}

/* arch_has_huge_bootmem_alloc arch_has_huge_bootmem_alloc */
#[inline]
pub unsafe fn arch_has_huge_bootmem_alloc() -> bool {
    firmware_has_feature(FW_FEATURE_LPAR) && !radix_enabled()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
