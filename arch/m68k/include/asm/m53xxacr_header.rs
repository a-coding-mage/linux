/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * m53xxacr.h -- ColdFire version 3 core cache support
 *
 * (C) Copyright 2010, Greg Ungerer <gerg@snapgear.com>
 */

/****************************************************************************/

/*
 * All varients of the ColdFire using version 3 cores have a similar
 * cache setup. They have a unified instruction and data cache, with
 * configurable write-through or copy-back operation.
 */

/* Define the Cache Control register flags. */
pub const CACR_EC: u32 = 0x8000_0000; // Enable cache
pub const CACR_ESB: u32 = 0x2000_0000; // Enable store buffer
pub const CACR_DPI: u32 = 0x1000_0000; // Disable invalidation by CPUSHL
pub const CACR_HLCK: u32 = 0x0800_0000; // Half cache lock mode
pub const CACR_CINVA: u32 = 0x0100_0000; // Invalidate cache
pub const CACR_DNFB: u32 = 0x0000_0400; // Inhibited fill buffer
pub const CACR_DCM_WT: u32 = 0x0000_0000; // Cacheable write-through
pub const CACR_DCM_CB: u32 = 0x0000_0100; // Cacheable copy-back
pub const CACR_DCM_PRE: u32 = 0x0000_0200; // Cache inhibited, precise
pub const CACR_DCM_IMPRE: u32 = 0x0000_0300; // Cache inhibited, imprecise
pub const CACR_WPROTECT: u32 = 0x0000_0020; // Write protect
pub const CACR_EUSP: u32 = 0x0000_0010; // Eanble separate user a7

/* Define the Access Control register flags. */
pub const ACR_BASE_POS: u32 = 24; // Address Base (upper 8 bits)
pub const ACR_MASK_POS: u32 = 16; // Address Mask (next 8 bits)
pub const ACR_ENABLE: u32 = 0x0000_8000; // Enable this ACR
pub const ACR_USER: u32 = 0x0000_0000; // Allow only user accesses
pub const ACR_SUPER: u32 = 0x0000_2000; // Allow supervisor access only
pub const ACR_ANY: u32 = 0x0000_4000; // Allow any access type
pub const ACR_CM_WT: u32 = 0x0000_0000; // Cacheable, write-through
pub const ACR_CM_CB: u32 = 0x0000_0020; // Cacheable, copy-back
pub const ACR_CM_PRE: u32 = 0x0000_0040; // Cache inhibited, precise
pub const ACR_CM_IMPRE: u32 = 0x0000_0060; // Cache inhibited, imprecise
pub const ACR_WPROTECT: u32 = 0x0000_0004; // Write protect region

/* Cache arrangement depends on the corresponding build configuration. */
#[cfg(feature = "CONFIG_M5307")]
pub const CACHE_SIZE: u32 = 0x2000; // 8k of unified cache
#[cfg(feature = "CONFIG_M5307")]
pub const ICACHE_SIZE: u32 = CACHE_SIZE;
#[cfg(feature = "CONFIG_M5307")]
pub const DCACHE_SIZE: u32 = CACHE_SIZE;

#[cfg(feature = "CONFIG_M53xx")]
pub const CACHE_SIZE: u32 = 0x4000; // 16k of unified cache
#[cfg(feature = "CONFIG_M53xx")]
pub const ICACHE_SIZE: u32 = CACHE_SIZE;
#[cfg(feature = "CONFIG_M53xx")]
pub const DCACHE_SIZE: u32 = CACHE_SIZE;

pub const CACHE_LINE_SIZE: u32 = 16; // 16 byte line size
pub const CACHE_WAYS: u32 = 4; // 4 ways - set associative

/* Cache controller settings selected by the build configuration. */
#[cfg(feature = "CONFIG_CACHE_COPYBACK")]
pub const CACHE_TYPE: u32 = ACR_CM_CB;
/* CONFIG_CACHE_COPYBACK also defines the C marker CACHE_PUSH. */
#[cfg(not(feature = "CONFIG_CACHE_COPYBACK"))]
pub const CACHE_TYPE: u32 = ACR_CM_WT;

#[cfg(feature = "CONFIG_COLDFIRE_SW_A7")]
pub const CACHE_MODE: u32 = CACR_EC + CACR_ESB + CACR_DCM_PRE;
#[cfg(not(feature = "CONFIG_COLDFIRE_SW_A7"))]
pub const CACHE_MODE: u32 = CACR_EC + CACR_ESB + CACR_DCM_PRE + CACR_EUSP;

pub const CACHE_INIT: u32 = CACHE_MODE + CACR_CINVA - CACR_EC;
pub const CACHE_INVALIDATE: u32 = CACHE_MODE + CACR_CINVA;
pub const CACHE_INVALIDATED: u32 = CACHE_MODE + CACR_CINVA;

/* CONFIG_RAMBASE is supplied by the surrounding build configuration. */
pub const ACR0_MODE: u32 =
    ((CONFIG_RAMBASE & 0xff00_0000) + 0x000f_0000 + (ACR_ENABLE + ACR_ANY + CACHE_TYPE));
pub const ACR1_MODE: u32 = 0;

/****************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
