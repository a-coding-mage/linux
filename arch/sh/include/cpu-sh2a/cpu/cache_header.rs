/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh2a/cache.h
 *
 * Copyright (C) 2004 Paul Mundt
 */

pub const L1_CACHE_SHIFT: u32 = 4;

pub const SH_CACHE_VALID: u32 = 1;
pub const SH_CACHE_UPDATED: u32 = 2;
pub const SH_CACHE_COMBINED: u32 = 4;
pub const SH_CACHE_ASSOC: u32 = 8;

pub const SH_CCR: u32 = 0xfffc1000; /* CCR1 */
pub const SH_CCR2: u32 = 0xfffc1004;

/*
 * Most of the SH-2A CCR1 definitions resemble the SH-4 ones. All others not
 * listed here are reserved.
 */
pub const CCR_CACHE_CB: u32 = 0x0000; /* Hack */
pub const CCR_CACHE_OCE: u32 = 0x0001;
pub const CCR_CACHE_WT: u32 = 0x0002;
pub const CCR_CACHE_OCI: u32 = 0x0008; /* OCF */
pub const CCR_CACHE_ICE: u32 = 0x0100;
pub const CCR_CACHE_ICI: u32 = 0x0800; /* ICF */

pub const CACHE_IC_ADDRESS_ARRAY: u32 = 0xf0000000;
pub const CACHE_OC_ADDRESS_ARRAY: u32 = 0xf0800000;

pub const CCR_CACHE_ENABLE: u32 = CCR_CACHE_OCE | CCR_CACHE_ICE;
pub const CCR_CACHE_INVALIDATE: u32 = CCR_CACHE_OCI | CCR_CACHE_ICI;
pub const CCR_ICACHE_INVALIDATE: u32 = CCR_CACHE_ICI;
pub const CCR_OCACHE_INVALIDATE: u32 = CCR_CACHE_OCI;
pub const CACHE_PHYSADDR_MASK: u32 = 0x1ffffc00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
