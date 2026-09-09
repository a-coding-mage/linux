/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2018-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

pub const SRAM_BAR_ID: u32 = 0;
pub const CFG_BAR_ID: u32 = 2;
pub const HBM_BAR_ID: u32 = 4;

pub const SRAM_BAR_SIZE: u64 = 0x4000000u64; // 64MB
pub const CFG_BAR_SIZE: u64 = 0x8000000u64; // 128MB

pub const CFG_BASE: u64 = 0x7FFC000000u64;
pub const CFG_SIZE: u32 = 0x400000; // 32MB CFG + 32MB DBG

pub const SRAM_BASE_ADDR: u64 = 0x7FF0000000u64;
pub const SRAM_SIZE: u32 = 0x1400000; // 20MB

pub const SPI_FLASH_BASE_ADDR: u64 = 0x7FF8000000u64;

pub const PSOC_SCRATCHPAD_ADDR: u64 = 0x7FFBFE0000u64;
pub const PSOC_SCRATCHPAD_SIZE: u32 = 0x10000; // 64KB

pub const PCIE_FW_SRAM_ADDR: u64 = 0x7FFBFF0000u64;
pub const PCIE_FW_SRAM_SIZE: u32 = 0x8000; // 32KB

pub const DRAM_PHYS_BASE: u64 = 0x0u64;

pub const HOST_PHYS_BASE: u64 = 0x8000000000u64; // 0.5TB
pub const HOST_PHYS_SIZE: u64 = 0x1000000000000u64; // 0.25PB (48 bits)

pub const GAUDI_MSI_ENTRIES: u32 = 32;

pub const QMAN_PQ_ENTRY_SIZE: u32 = 16; // Bytes

pub const MAX_ASID: u32 = 2;

pub const PROT_BITS_OFFS: u32 = 0xF80;

pub const MME_NUMBER_OF_MASTER_ENGINES: u32 = 2;

pub const MME_NUMBER_OF_SLAVE_ENGINES: u32 = 2;

pub const TPC_NUMBER_OF_ENGINES: u32 = 8;

pub const DMA_NUMBER_OF_CHANNELS: u32 = 8;

pub const NIC_NUMBER_OF_MACROS: u32 = 5;

pub const NIC_NUMBER_OF_ENGINES: u32 = NIC_NUMBER_OF_MACROS * 2;

pub const NUMBER_OF_IF: u32 = 8;

pub const DEVICE_CACHE_LINE_SIZE: u32 = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
