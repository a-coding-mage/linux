/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm-generic/tlb.h.

// Equivalent to the C preprocessor condition:
// #if CONFIG_PGTABLE_LEVELS == 3
#[cfg(CONFIG_PGTABLE_LEVELS_3)]
macro_rules! __pmd_free_tlb {
    ($tlb:expr, $pmd:expr, $addr:expr) => {
        tlb_remove_ptdesc(($tlb), virt_to_ptdesc($pmd))
    };
}

macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $addr:expr) => {
        tlb_remove_ptdesc(($tlb), page_ptdesc($pte))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
