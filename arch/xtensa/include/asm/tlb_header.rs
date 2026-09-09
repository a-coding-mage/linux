/*
 * include/asm-xtensa/tlb.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C dependencies: <asm/cache.h>, <asm/page.h>, and <asm-generic/tlb.h>.

/// Equivalent of the C macro `__pte_free_tlb(tlb, pte, address)`.
///
/// The `address` argument is intentionally unused, matching the source macro.
#[macro_export]
macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $address:expr) => {
        pte_free(($tlb).mm, $pte)
    };
}

extern "C" {
    pub fn check_tlb_sanity();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
