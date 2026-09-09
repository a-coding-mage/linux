/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 2001, 2003 by Ralf Baechle
 * Copyright (C) 1999, 2000, 2001 Silicon Graphics, Inc.
 */

// Originally guarded by _ASM_NIOS2_PGALLOC_H.
// Dependencies: linux/mm.h and asm-generic/pgalloc.h.

#[inline]
pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    let _ = mm;
    set_pmd(pmd, __pmd(pte as usize as ::core::ffi::c_ulong));
}

#[inline]
pub unsafe fn pmd_populate(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: pgtable_t,
) {
    let _ = mm;
    set_pmd(
        pmd,
        __pmd(page_address(pte) as usize as ::core::ffi::c_ulong),
    );
}

extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
}

// C macro: __pte_free_tlb(tlb, pte, addr)
#[macro_export]
macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $addr:expr) => {{
        let _ = $addr;
        tlb_remove_ptdesc($tlb, page_ptdesc($pte));
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
