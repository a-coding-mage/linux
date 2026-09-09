// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/proc-syms.c
 *
 *  Copyright (C) 2000-2002 Russell King
 */

// The following exports correspond to the kernel's EXPORT_SYMBOL macro.
// Build-time configuration conditions are preserved with cfg feature names.

#[cfg(not(feature = "MULTI_CPU"))]
EXPORT_SYMBOL!(cpu_dcache_clean_area);

#[cfg(all(not(feature = "MULTI_CPU"), feature = "CONFIG_MMU"))]
EXPORT_SYMBOL!(cpu_set_pte_ext);

#[cfg(feature = "MULTI_CPU")]
EXPORT_SYMBOL!(processor);

#[cfg(not(feature = "MULTI_CACHE"))]
EXPORT_SYMBOL!(__cpuc_flush_kern_all);

#[cfg(not(feature = "MULTI_CACHE"))]
EXPORT_SYMBOL!(__cpuc_flush_user_all);

#[cfg(not(feature = "MULTI_CACHE"))]
EXPORT_SYMBOL!(__cpuc_flush_user_range);

#[cfg(not(feature = "MULTI_CACHE"))]
EXPORT_SYMBOL!(__cpuc_coherent_kern_range);

#[cfg(not(feature = "MULTI_CACHE"))]
EXPORT_SYMBOL!(__cpuc_flush_dcache_area);

#[cfg(feature = "MULTI_CACHE")]
EXPORT_SYMBOL!(cpu_cache);

#[cfg(all(feature = "CONFIG_MMU", not(feature = "MULTI_USER")))]
EXPORT_SYMBOL!(__cpu_clear_user_highpage);

#[cfg(all(feature = "CONFIG_MMU", not(feature = "MULTI_USER")))]
EXPORT_SYMBOL!(__cpu_copy_user_highpage);

#[cfg(all(feature = "CONFIG_MMU", feature = "MULTI_USER"))]
EXPORT_SYMBOL!(cpu_user);

/*
 * No module should need to touch the TLB (and currently
 * no modules do.  We export this for "loadkernel" support
 * (booting a new kernel from within a running kernel.)
 */
#[cfg(feature = "MULTI_TLB")]
EXPORT_SYMBOL!(cpu_tlb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
