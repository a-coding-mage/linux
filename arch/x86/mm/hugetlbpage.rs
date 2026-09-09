// SPDX-License-Identifier: GPL-2.0
/*
 * IA-32 Huge TLB Page Support for Kernel.
 *
 * Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
 */

// C headers omitted; symbols supplied by the surrounding kernel translation are
// referenced directly below.

#[cfg(target_pointer_width = "64")]
pub unsafe fn arch_hugetlb_valid_size(size: usize) -> bool {
    if size == PMD_SIZE {
        true
    } else if size == PUD_SIZE && boot_cpu_has(X86_FEATURE_GBPAGES) {
        true
    } else {
        false
    }
}

#[cfg(all(target_pointer_width = "64", feature = "CONFIG_CONTIG_ALLOC"))]
unsafe fn gigantic_pages_init() -> i32 {
    /* With compaction or CMA we can allocate gigantic pages at runtime */
    if boot_cpu_has(X86_FEATURE_GBPAGES) {
        hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT);
    }
    0
}

// arch_initcall(gigantic_pages_init);

pub unsafe fn arch_hugetlb_cma_order() -> u32 {
    if boot_cpu_has(X86_FEATURE_GBPAGES) {
        return PUD_SHIFT - PAGE_SHIFT;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
