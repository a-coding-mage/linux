/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * m52xxacr.h -- ColdFire version 2 core cache support
 *
 * (C) Copyright 2010, Greg Ungerer <gerg@snapgear.com>
 */

/****************************************************************************/

/*
 * All varients of the ColdFire using version 2 cores have a similar
 * cache setup. Although not absolutely identical the cache register
 * definitions are compatible for all of them. Mostly they support a
 * configurable cache memory that can be instruction only, data only,
 * or split instruction and data. The exception is the very old version 2
 * core based parts, like the 5206(e), 5249 and 5272, which are instruction
 * cache only. Cache size varies from 2k up to 16k.
 */

/*
 * Define the Cache Control register flags.
 */
pub const CACR_CENB: u32 = 0x80000000; /* Enable cache */
pub const CACR_CDPI: u32 = 0x10000000; /* Disable invalidation by CPUSHL */
pub const CACR_CFRZ: u32 = 0x08000000; /* Cache freeze mode */
pub const CACR_CINV: u32 = 0x01000000; /* Invalidate cache */
pub const CACR_DISI: u32 = 0x00800000; /* Disable instruction cache */
pub const CACR_DISD: u32 = 0x00400000; /* Disable data cache */
pub const CACR_INVI: u32 = 0x00200000; /* Invalidate instruction cache */
pub const CACR_INVD: u32 = 0x00100000; /* Invalidate data cache */
pub const CACR_CEIB: u32 = 0x00000400; /* Non-cachable instruction burst */
pub const CACR_DCM: u32 = 0x00000200; /* Default cache mode */
pub const CACR_DBWE: u32 = 0x00000100; /* Buffered write enable */
pub const CACR_DWP: u32 = 0x00000020; /* Write protection */
pub const CACR_EUSP: u32 = 0x00000010; /* Enable separate user a7 */

/*
 * Define the Access Control register flags.
 */
pub const ACR_BASE_POS: u32 = 24; /* Address Base (upper 8 bits) */
pub const ACR_MASK_POS: u32 = 16; /* Address Mask (next 8 bits) */
pub const ACR_ENABLE: u32 = 0x00008000; /* Enable this ACR */
pub const ACR_USER: u32 = 0x00000000; /* Allow only user accesses */
pub const ACR_SUPER: u32 = 0x00002000; /* Allow supervisor access only */
pub const ACR_ANY: u32 = 0x00004000; /* Allow any access type */
pub const ACR_CENB: u32 = 0x00000000; /* Caching of region enabled */
pub const ACR_CDIS: u32 = 0x00000040; /* Caching of region disabled */
pub const ACR_BWE: u32 = 0x00000020; /* Write buffer enabled */
pub const ACR_WPROTECT: u32 = 0x00000004; /* Write protect region */

/*
 * Set the cache controller settings we will use. On the cores that support
 * a split cache configuration we allow all the combinations at Kconfig
 * time. For those cores that only have an instruction cache we just set
 * that as on.
 *
 * The following cfg features represent the original CONFIG_CACHE_* build
 * conditions.
 */
#[cfg(feature = "CONFIG_CACHE_I")]
pub const CACHE_TYPE: u32 = CACR_DISD + CACR_EUSP;
#[cfg(feature = "CONFIG_CACHE_I")]
pub const CACHE_INVTYPEI: u32 = 0;

#[cfg(feature = "CONFIG_CACHE_D")]
pub const CACHE_TYPE: u32 = CACR_DISI + CACR_EUSP;
#[cfg(feature = "CONFIG_CACHE_D")]
pub const CACHE_INVTYPED: u32 = 0;

#[cfg(feature = "CONFIG_CACHE_BOTH")]
pub const CACHE_TYPE: u32 = CACR_EUSP;
#[cfg(feature = "CONFIG_CACHE_BOTH")]
pub const CACHE_INVTYPEI: u32 = CACR_INVI;
#[cfg(feature = "CONFIG_CACHE_BOTH")]
pub const CACHE_INVTYPED: u32 = CACR_INVD;

/* This is the instruction cache only devices (no split cache, no eusp). */
#[cfg(not(any(
    feature = "CONFIG_CACHE_I",
    feature = "CONFIG_CACHE_D",
    feature = "CONFIG_CACHE_BOTH"
)))]
pub const CACHE_TYPE: u32 = 0;
#[cfg(not(any(
    feature = "CONFIG_CACHE_I",
    feature = "CONFIG_CACHE_D",
    feature = "CONFIG_CACHE_BOTH"
)))]
pub const CACHE_INVTYPEI: u32 = 0;

pub const CACHE_INIT: u32 = CACR_CINV + CACHE_TYPE;
pub const CACHE_MODE: u32 = CACR_CENB + CACHE_TYPE + CACR_DCM;

pub const CACHE_INVALIDATE: u32 = CACHE_MODE + CACR_CINV;
#[cfg(any(feature = "CONFIG_CACHE_I", feature = "CONFIG_CACHE_BOTH"))]
pub const CACHE_INVALIDATEI: u32 = CACHE_MODE + CACR_CINV + CACHE_INVTYPEI;
#[cfg(feature = "CONFIG_CACHE_BOTH")]
pub const CACHE_INVALIDATED: u32 = CACHE_MODE + CACR_CINV + CACHE_INVTYPED;

/* CONFIG_RAMBASE is supplied by the external build configuration. */
pub const ACR0_MODE: u32 =
    ((CONFIG_RAMBASE & 0xff000000) + 0x000f0000)
        + (ACR_ENABLE + ACR_ANY + ACR_CENB + ACR_BWE);
pub const ACR1_MODE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
