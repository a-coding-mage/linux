/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

pub const SRAM_CFG_BAR_ID: u32 = 0;
pub const MSIX_BAR_ID: u32 = 2;
pub const DDR_BAR_ID: u32 = 4;

pub const CFG_BAR_SIZE: u64 = 0x10000000u64; /* 256MB */
pub const MSIX_BAR_SIZE: u64 = 0x1000u64; /* 4KB */

pub const CFG_BASE: u64 = 0x7FFC000000u64;
pub const CFG_SIZE: u32 = 0x4000000; /* 32MB CFG + 32MB DBG */

pub const SRAM_BASE_ADDR: u64 = 0x7FF0000000u64;
pub const SRAM_SIZE: u32 = 0x32A0000; /* 50.625MB */

pub const DRAM_PHYS_BASE: u64 = 0x0u64;

pub const HOST_PHYS_BASE: u64 = 0x8000000000u64; /* 0.5TB */
pub const HOST_PHYS_SIZE: u64 = 0x1000000000000u64; /* 0.25PB (48 bits) */

pub const GOYA_MSIX_ENTRIES: u32 = 8;

pub const QMAN_PQ_ENTRY_SIZE: u32 = 16; /* Bytes */

pub const MAX_ASID: u32 = 2;

pub const PROT_BITS_OFFS: u32 = 0xF80;

pub const DMA_MAX_NUM: u32 = 5;

pub const TPC_MAX_NUM: u32 = 8;

pub const MME_MAX_NUM: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
