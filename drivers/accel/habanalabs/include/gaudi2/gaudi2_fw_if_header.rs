/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2019-2021 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

pub const GAUDI2_EVENT_QUEUE_MSIX_IDX: u32 = 0;

pub const UBOOT_FW_OFFSET: u32 = 0x100000; // 1MB in SRAM
pub const LINUX_FW_OFFSET: u32 = 0x800000; // 8BM in DDR

pub const GAUDI2_PLL_FREQ_LOW: u32 = 200000000; // 200 MHz

pub const GAUDI2_SP_SRAM_BASE_ADDR: u32 = 0x27FE0000;
pub const GAUDI2_MAILBOX_BASE_ADDR: u32 = 0x27FE1800;

pub const GAUDI2_NUM_MME: usize = 4;

pub const NUM_OF_GPIOS_PER_PORT: u32 = 16;
pub const GAUDI2_WD_GPIO: u32 = 62 % NUM_OF_GPIOS_PER_PORT;

pub const GAUDI2_ARCPID_TX_MB_SIZE: u32 = 0x1000;
pub const GAUDI2_ARCPID_RX_MB_SIZE: u32 = 0x400;
pub const GAUDI2_ARM_TX_MB_SIZE: u32 = 0x400;
pub const GAUDI2_ARM_RX_MB_SIZE: u32 = 0x1800;

pub const GAUDI2_DCCM_BASE_ADDR: u32 = 0x27020000;

pub const GAUDI2_ARM_TX_MB_ADDR: u32 = GAUDI2_MAILBOX_BASE_ADDR;

pub const GAUDI2_ARM_RX_MB_ADDR: u32 =
    GAUDI2_ARM_TX_MB_ADDR + GAUDI2_ARM_TX_MB_SIZE;

pub const GAUDI2_ARCPID_TX_MB_ADDR: u32 =
    GAUDI2_ARM_RX_MB_ADDR + GAUDI2_ARM_RX_MB_SIZE;

pub const GAUDI2_ARCPID_RX_MB_ADDR: u32 =
    GAUDI2_ARCPID_TX_MB_ADDR + GAUDI2_ARCPID_TX_MB_SIZE;

pub const GAUDI2_ARM_TX_MB_OFFSET: u32 =
    GAUDI2_ARM_TX_MB_ADDR - GAUDI2_SP_SRAM_BASE_ADDR;

pub const GAUDI2_ARM_RX_MB_OFFSET: u32 =
    GAUDI2_ARM_RX_MB_ADDR - GAUDI2_SP_SRAM_BASE_ADDR;

pub const POWER_MODE_LEVELS: [u32; 3] = [
    150000, // 00
    250000, // 01
    400000, // 10
    // 11: Normal mode
];

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gaudi2_fw_status {
    GAUDI2_PID_STATUS_UP = 0x1, // PID on ARC0 is up
    GAUDI2_ARM_STATUS_UP = 0x2, // ARM Linux Boot complete
    GAUDI2_MGMT_STATUS_UP = 0x3, // ARC1 Mgmt is up
    GAUDI2_STATUS_LAST = 0xFF,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gaudi2_rst_src {
    HL_COLD_RST = 1,
    HL_MANUAL_RST = 2,
    HL_PRSTN_RST = 4,
    HL_SOFT_RST = 8,
    HL_WD_RST = 16,
    HL_FW_ALL_RST = 32,
    HL_SW_ALL_RST = 64,
    HL_FLR_RST = 128,
    HL_ECC_DERR_RST = 256,
}

#[repr(C, packed)]
pub struct gaudi2_redundancy_ctx {
    pub redundant_hbm: u32,
    pub redundant_edma: u32,
    pub redundant_tpc: u32,
    pub redundant_vdec: u32,
    pub hbm_mask: u64,
    pub edma_mask: u64,
    pub tpc_mask: u64,
    pub vdec_mask: u64,
    pub mme_mask: u64,
    pub nic_mask: u64,
    pub rtr_mask: u64,
    pub hmmu_hif_iso: u64,
    pub xbar_edge_iso: u64,
    pub hmmu_hif_mask: u64,
    pub xbar_edge_mask: u64,
    pub mme_pe_iso: [u8; GAUDI2_NUM_MME],
    pub full_hbm_mode: u32, // true on full (non binning hbm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
