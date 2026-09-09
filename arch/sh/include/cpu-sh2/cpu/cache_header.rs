/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh2/cache.h
 *
 * Copyright (C) 2003 Paul Mundt
 */

pub const L1_CACHE_SHIFT: u32 = 4;

pub const SH_CACHE_VALID: u32 = 1;
pub const SH_CACHE_UPDATED: u32 = 2;
pub const SH_CACHE_COMBINED: u32 = 4;
pub const SH_CACHE_ASSOC: u32 = 8;

/* Build-time condition: defined(CONFIG_CPU_SUBTYPE_SH7619). */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const SH_CCR: u32 = 0xffffffec;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_CE: u32 = 0x01; /* Cache enable */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_WT: u32 = 0x02; /* CCR[bit1=1,bit2=1] */
/* 0x00000000-0x7fffffff: Write-through */
/* 0x80000000-0x9fffffff: Write-back */
/* 0xc0000000-0xdfffffff: Write-through */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_CB: u32 = 0x04; /* CCR[bit1=0,bit2=0] */
/* 0x00000000-0x7fffffff: Write-back */
/* 0x80000000-0x9fffffff: Write-through */
/* 0xc0000000-0xdfffffff: Write-back */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_CF: u32 = 0x08; /* Cache invalidate */

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CACHE_OC_ADDRESS_ARRAY: u32 = 0xf0000000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CACHE_OC_DATA_ARRAY: u32 = 0xf1000000;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_ENABLE: u32 = CCR_CACHE_CE;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CCR_CACHE_INVALIDATE: u32 = CCR_CACHE_CF;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7619")]
pub const CACHE_PHYSADDR_MASK: u32 = 0x1ffffc00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
