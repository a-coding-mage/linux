/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/hugetlb.h
 *
 * Copyright (C) 2012 ARM Ltd.
 *
 * Based on arch/x86/include/asm/hugetlb.h
 */

// Dependencies supplied by the corresponding architecture and generic headers:
// asm/cacheflush.h, asm/page.h, asm/hugetlb-3level.h, asm-generic/hugetlb.h

pub unsafe fn arch_clear_hugetlb_flags(folio: *mut folio) {
    clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
}

// C macro alias: arch_clear_hugetlb_flags

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
