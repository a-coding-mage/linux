/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh4/cache.h
 *
 * Copyright (C) 1999 Niibe Yutaka
 */

pub const L1_CACHE_SHIFT: u32 = 5;

pub const SH_CACHE_VALID: u32 = 1;
pub const SH_CACHE_UPDATED: u32 = 2;
pub const SH_CACHE_COMBINED: u32 = 4;
pub const SH_CACHE_ASSOC: u32 = 8;

pub const SH_CCR: u32 = 0xff00001c; /* Address of Cache Control Register */
pub const CCR_CACHE_OCE: u32 = 0x0001; /* Operand Cache Enable */
pub const CCR_CACHE_WT: u32 = 0x0002; /* Write-Through (for P0,U0,P3) (else writeback) */
pub const CCR_CACHE_CB: u32 = 0x0004; /* Copy-Back (for P1) (else writethrough) */
pub const CCR_CACHE_OCI: u32 = 0x0008; /* OC Invalidate */
pub const CCR_CACHE_ORA: u32 = 0x0020; /* OC RAM Mode */
pub const CCR_CACHE_OIX: u32 = 0x0080; /* OC Index Enable */
pub const CCR_CACHE_ICE: u32 = 0x0100; /* Instruction Cache Enable */
pub const CCR_CACHE_ICI: u32 = 0x0800; /* IC Invalidate */
pub const CCR_CACHE_IIX: u32 = 0x8000; /* IC Index Enable */

/* Preserved from the C build-time condition: define when CONFIG_CPU_SH4A is not enabled. */
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const CCR_CACHE_EMODE: u32 = 0x80000000; /* EMODE Enable */

/* Default CCR setup: 8k+16k-byte cache,P1-wb,enable */
pub const CCR_CACHE_ENABLE: u32 = CCR_CACHE_OCE | CCR_CACHE_ICE;
pub const CCR_CACHE_INVALIDATE: u32 = CCR_CACHE_OCI | CCR_CACHE_ICI;

pub const CACHE_IC_ADDRESS_ARRAY: u32 = 0xf0000000;
pub const CACHE_OC_ADDRESS_ARRAY: u32 = 0xf4000000;

pub const RAMCR: u32 = 0xFF000074;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
