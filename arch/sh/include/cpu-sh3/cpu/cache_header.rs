/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh3/cache.h
 *
 * Copyright (C) 1999 Niibe Yutaka
 */

pub const L1_CACHE_SHIFT: u32 = 4;

pub const SH_CACHE_VALID: u32 = 1;
pub const SH_CACHE_UPDATED: u32 = 2;
pub const SH_CACHE_COMBINED: u32 = 4;
pub const SH_CACHE_ASSOC: u32 = 8;

pub const SH_CCR: u32 = 0xffffffec; /* Address of Cache Control Register */

pub const CCR_CACHE_CE: u32 = 0x01; /* Cache Enable */
pub const CCR_CACHE_WT: u32 = 0x02; /* Write-Through (for P0,U0,P3) (else writeback) */
pub const CCR_CACHE_CB: u32 = 0x04; /* Write-Back (for P1) (else writethrough) */
pub const CCR_CACHE_CF: u32 = 0x08; /* Cache Flush */
pub const CCR_CACHE_ORA: u32 = 0x20; /* RAM mode */

pub const CACHE_OC_ADDRESS_ARRAY: u32 = 0xf0000000;
pub const CACHE_PHYSADDR_MASK: u32 = 0x1ffffc00;

pub const CCR_CACHE_ENABLE: u32 = CCR_CACHE_CE;
pub const CCR_CACHE_INVALIDATE: u32 = CCR_CACHE_CF;

/* C build-time conditions: CONFIG_CPU_SUBTYPE_SH7705,
 * CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7720, or
 * CONFIG_CPU_SUBTYPE_SH7721.
 */
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7705",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
))]
pub const CCR3_REG: u32 = 0xa40000b4;

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7705",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
))]
pub const CCR_CACHE_16KB: u32 = 0x00010000;

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7705",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
))]
pub const CCR_CACHE_32KB: u32 = 0x00020000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
