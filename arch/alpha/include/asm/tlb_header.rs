/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm-generic/tlb.h>

macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $address:expr) => {
        tlb_remove_ptdesc(($tlb), page_ptdesc($pte))
    };
}

macro_rules! __pmd_free_tlb {
    ($tlb:expr, $pmd:expr, $address:expr) => {
        tlb_remove_ptdesc(($tlb), virt_to_ptdesc($pmd))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
