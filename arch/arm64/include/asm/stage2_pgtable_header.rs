/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 - ARM Ltd
 *
 * stage2 page table helpers
 */

// Dependency supplied by the translated pgtable header.

/*
 * The hardware supports concatenation of up to 16 tables at stage2 entry
 * level and we use the feature whenever possible, which means we resolve 4
 * additional bits of address at the entry level.
 *
 * This implies, the total number of page table levels required for
 * IPA_SHIFT at stage2 expected by the hardware can be calculated using the
 * same logic used for the (non-collapsable) stage1 page tables but for
 * (IPA_SHIFT - 4).
 */
macro_rules! stage2_pgtable_levels {
    ($ipa:expr) => {
        ARM64_HW_PGTABLE_LEVELS!(($ipa) - 4)
    };
}

macro_rules! kvm_stage2_levels {
    ($mmu:expr) => {
        VTCR_EL2_LVLS!(($mmu).vtcr)
    };
}

/*
 * kvm_mmmu_cache_min_pages() is the number of pages required to install
 * a stage-2 translation. We pre-allocate the entry level page table at the
 * VM creation.
 */
macro_rules! kvm_mmu_cache_min_pages {
    ($mmu:expr) => {
        kvm_stage2_levels!($mmu) - 1
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
