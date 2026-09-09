/*
 * PPC Huge TLB Page Support for Kernel.
 *
 * Copyright (C) 2003 David Gibson, IBM Corporation.
 * Copyright (C) 2011 Becky Bruce, Freescale Semiconductor
 *
 * Based on the IA-32 version:
 * Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
 */

// Linux kernel dependencies supplied by other translation units.

pub static mut hugetlb_disabled: bool = false;

// #define PTE_T_ORDER (__builtin_ffs(sizeof(pte_basic_t)) - __builtin_ffs(sizeof(void *)))
pub const PTE_T_ORDER: i32 = 0;

pub unsafe fn huge_pte_offset(
    mm: *mut mm_struct,
    addr: c_ulong,
    _sz: c_ulong,
) -> *mut pte_t {
    /*
     * Only called for hugetlbfs pages, hence can ignore THP and the
     * irq disabled walk.
     */
    __find_linux_pte((*mm).pgd, addr, core::ptr::null_mut(), core::ptr::null_mut())
}

pub unsafe fn huge_pte_alloc(
    mm: *mut mm_struct,
    _vma: *mut vm_area_struct,
    mut addr: c_ulong,
    sz: c_ulong,
) -> *mut pte_t {
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;

    addr &= !(sz - 1);

    p4d = p4d_offset(pgd_offset(mm, addr), addr);
    if !mm_pud_folded(mm) && sz >= P4D_SIZE {
        return p4d as *mut pte_t;
    }

    pud = pud_alloc(mm, p4d, addr);
    if pud.is_null() {
        return core::ptr::null_mut();
    }
    if !mm_pmd_folded(mm) && sz >= PUD_SIZE {
        return pud as *mut pte_t;
    }

    pmd = pmd_alloc(mm, pud, addr);
    if pmd.is_null() {
        return core::ptr::null_mut();
    }

    if sz >= PMD_SIZE {
        /* On 8xx, all hugepages are handled as contiguous PTEs */
        // CONFIG_PPC_8xx conditional: retained as the source conditional.
        if cfg!(feature = "CONFIG_PPC_8xx") {
            let mut i: usize = 0;
            while i < (sz / PMD_SIZE) as usize {
                if !pte_alloc_huge(mm, pmd.add(i), addr) {
                    return core::ptr::null_mut();
                }
                i += 1;
            }
        }
        return pmd as *mut pte_t;
    }

    pte_alloc_huge(mm, pmd, addr)
}

// CONFIG_PPC_BOOK3S_64 conditional.
/*
 * Tracks gpages after the device tree is scanned and before the
 * huge_boot_pages list is ready on pseries.
 */
pub const MAX_NUMBER_GPAGES: usize = 1024;
pub static mut gpage_freearray: [u64; MAX_NUMBER_GPAGES] = [0; MAX_NUMBER_GPAGES];
pub static mut nr_gpages: c_uint = 0;

/*
 * Build list of addresses of gigantic pages.  This function is used in early
 * boot before the buddy allocator is setup.
 */
pub unsafe fn pseries_add_gpage(mut addr: u64, page_size: u64, mut number_of_pages: c_ulong) {
    if addr == 0 {
        return;
    }
    while number_of_pages > 0 {
        gpage_freearray[nr_gpages as usize] = addr;
        nr_gpages += 1;
        number_of_pages -= 1;
        addr += page_size;
    }
}

unsafe fn pseries_alloc_bootmem_huge_page(_hstate: *mut hstate) -> *mut core::ffi::c_void {
    if nr_gpages == 0 {
        return core::ptr::null_mut();
    }
    nr_gpages -= 1;
    let m = phys_to_virt(gpage_freearray[nr_gpages as usize]);
    gpage_freearray[nr_gpages as usize] = 0;
    m
}

pub unsafe fn hugetlb_node_alloc_supported() -> bool {
    false
}

pub unsafe fn arch_alloc_bootmem_huge_page(
    h: *mut hstate,
    nid: c_int,
) -> *mut core::ffi::c_void {
    // CONFIG_PPC_BOOK3S_64 conditional.
    if cfg!(feature = "CONFIG_PPC_BOOK3S_64")
        && firmware_has_feature(FW_FEATURE_LPAR)
        && !radix_enabled()
    {
        return pseries_alloc_bootmem_huge_page(h);
    }
    __alloc_bootmem_huge_page(h, nid)
}

pub unsafe fn arch_hugetlb_valid_size(size: c_ulong) -> bool {
    let shift = __ffs(size);
    let mmu_psize: c_int;

    /* Check that it is a page size supported by the hardware and
     * that it fits within pagetable and slice limits. */
    if size <= PAGE_SIZE || !is_power_of_2(size) {
        return false;
    }

    mmu_psize = check_and_get_huge_psize(shift);
    if mmu_psize < 0 {
        return false;
    }

    BUG_ON(mmu_psize_defs[mmu_psize as usize].shift != shift);
    true
}

unsafe fn add_huge_page_size(size: c_ulonglong) -> c_int {
    let shift = __ffs(size as c_ulong);

    if !arch_hugetlb_valid_size(size as c_ulong) {
        return -EINVAL;
    }

    hugetlb_add_hstate(shift - PAGE_SHIFT);
    0
}

unsafe fn hugetlbpage_init() -> c_int {
    let mut configured = false;
    let mut psize: c_int = 0;

    if hugetlb_disabled {
        pr_info("HugeTLB support is disabled!\n");
        return 0;
    }

    if cfg!(feature = "CONFIG_PPC_BOOK3S_64")
        && !radix_enabled()
        && !mmu_has_feature(MMU_FTR_16M_PAGE)
    {
        return -ENODEV;
    }

    while psize < MMU_PAGE_COUNT {
        if mmu_psize_defs[psize as usize].shift == 0 {
            psize += 1;
            continue;
        }

        let shift = mmu_psize_to_shift(psize);
        if add_huge_page_size(1u64 << shift) < 0 {
            psize += 1;
            continue;
        }

        configured = true;
        psize += 1;
    }

    if !configured {
        pr_info("Failed to initialize. Disabling HugeTLB");
    }

    0
}

// arch_initcall(hugetlbpage_init);

pub unsafe fn arch_hugetlb_cma_order() -> c_uint {
    if radix_enabled() {
        return PUD_SHIFT - PAGE_SHIFT;
    } else if !firmware_has_feature(FW_FEATURE_LPAR)
        && mmu_psize_defs[MMU_PAGE_16G as usize].shift != 0
    {
        /*
         * For pseries we do use ibm,expected#pages for reserving 16G pages.
         */
        return mmu_psize_to_shift(MMU_PAGE_16G) - PAGE_SHIFT;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
