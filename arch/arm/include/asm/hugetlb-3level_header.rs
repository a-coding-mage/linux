/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/hugetlb-3level.h
 *
 * Copyright (C) 2012 ARM Ltd.
 *
 * Based on arch/x86/include/asm/hugetlb.h.
 */

/*
 * Dependency intent from the original header: pte_t, mm_struct, pte_val,
 * and L_PTE_VALID are supplied by the surrounding architecture code.
 */

/*
 * If our huge pte is non-zero then mark the valid bit.
 * This allows pte_present(huge_ptep_get(mm,addr,ptep)) to return true for non-zero
 * ptes.
 * (The valid bit is automatically cleared by set_pte_at for PROT_NONE ptes).
 */
pub const __HAVE_ARCH_HUGE_PTEP_GET: bool = true;

pub unsafe fn huge_ptep_get(
    _mm: *mut mm_struct,
    _addr: libc::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    let mut retval = core::ptr::read(ptep);
    if retval.pte != 0 {
        retval.pte |= L_PTE_VALID;
    }
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
