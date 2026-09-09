/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2018 Oleksij Rempel <linux@rempel-privat.de>
 *
 * Driver for Alcor Micro AU6601 and AU6621 controllers
 */

pub const ALCOR_SD_CARD: i32 = 0;
pub const ALCOR_MS_CARD: i32 = 1;

pub const DRV_NAME_ALCOR_PCI: &str = "alcor_pci";
pub const DRV_NAME_ALCOR_PCI_SDMMC: &str = "alcor_sdmmc";
pub const DRV_NAME_ALCOR_PCI_MS: &str = "alcor_ms";

pub const PCI_ID_ALCOR_MICRO: u32 = 0x1AEA;
pub const PCI_ID_AU6601: u32 = 0x6601;
pub const PCI_ID_AU6621: u32 = 0x6621;
pub const PCI_ID_AU6625: u32 = 0x6625;

#[inline]
pub const fn mhz_to_hz(freq: u32) -> u32 { freq.wrapping_mul(1000).wrapping_mul(1000) }

pub const AU6601_BASE_CLOCK: u32 = 31000000;
pub const AU6601_MIN_CLOCK: u32 = 150000;
pub const AU6601_MAX_CLOCK: u32 = 208000000;
pub const AU6601_MAX_DMA_SEGMENTS: u32 = 64;
pub const AU6601_MAX_PIO_SEGMENTS: u32 = 1;
pub const AU6601_MAX_DMA_BLOCK_SIZE: u32 = 0x1000;
pub const AU6601_MAX_PIO_BLOCK_SIZE: u32 = 0x200;
pub const AU6601_MAX_DMA_BLOCKS: u32 = 1;
pub const AU6601_DMA_LOCAL_SEGMENTS: u32 = 1;

/* Registers spotted by reverse engineering but still with unknown functionality. */
/* 0x10 - ADMA phy address. AU6621 only? */
/* 0x51 - LED ctrl? */
/* 0x52 - unknown */
/* 0x61 - LED related? Always toggled BIT0 */
/* 0x63 - Same as 0x61? */
/* 0x77 - unknown */

/* SDMA phy address. Higher then 0x0800.0000? */
/* The au6601 and au6621 have different DMA engines with different issues. */
/* For example au6621 engine is triggered by addr change. No other interaction is needed. */
/* This means, if we get two buffers with same address, then engine will stall. */
pub const AU6601_REG_SDMA_ADDR: u32 = 0x00;
pub const AU6601_SDMA_MASK: u32 = 0xffffffff;
pub const AU6601_DMA_BOUNDARY: u32 = 0x05;
pub const AU6621_DMA_PAGE_CNT: u32 = 0x05;
pub const AU6601_REG_BUFFER: u32 = 0x08;
pub const AU6621_DMA_CTRL: u32 = 0x0c;
pub const AU6621_DMA_ENABLE: u32 = 1 << 0;
pub const AU6601_REG_CMD_OPCODE: u32 = 0x23;
pub const AU6601_REG_CMD_ARG: u32 = 0x24;
pub const AU6601_REG_CMD_RSP0: u32 = 0x30;
pub const AU6601_REG_CMD_RSP1: u32 = 0x34;
pub const AU6601_REG_CMD_RSP2: u32 = 0x38;
pub const AU6601_REG_CMD_RSP3: u32 = 0x3C;
pub const AU6601_TIME_OUT_CTRL: u32 = 0x69;
pub const AU6601_REG_BLOCK_SIZE: u32 = 0x6c;
pub const AU6601_POWER_CONTROL: u32 = 0x70;
pub const AU6601_CLK_SELECT: u32 = 0x72;
pub const AU6601_CLK_OVER_CLK: u32 = 0x80;
pub const AU6601_CLK_384_MHZ: u32 = 0x30;
pub const AU6601_CLK_125_MHZ: u32 = 0x20;
pub const AU6601_CLK_48_MHZ: u32 = 0x10;
pub const AU6601_CLK_EXT_PLL: u32 = 0x04;
pub const AU6601_CLK_X2_MODE: u32 = 0x02;
pub const AU6601_CLK_ENABLE: u32 = 0x01;
pub const AU6601_CLK_31_25_MHZ: u32 = 0x00;
pub const AU6601_CLK_DIVIDER: u32 = 0x73;
pub const AU6601_INTERFACE_MODE_CTRL: u32 = 0x74;
pub const AU6601_DLINK_MODE: u32 = 0x80;
pub const AU6601_INTERRUPT_DELAY_TIME: u32 = 0x40;
pub const AU6601_SIGNAL_REQ_CTRL: u32 = 0x30;
pub const AU6601_MS_CARD_WP: u32 = 1 << 3;
pub const AU6601_SD_CARD_WP: u32 = 1 << 0;
pub const AU6601_ACTIVE_CTRL: u32 = 0x75;
pub const AU6601_XD_CARD: u32 = 1 << 4;
pub const AU6601_MS_CARD: u32 = 1 << 3;
pub const AU6601_SD_CARD: u32 = 1 << 0;
pub const AU6601_DETECT_STATUS: u32 = 0x76;
pub const AU6601_DETECT_EN: u32 = 1 << 7;
pub const AU6601_MS_DETECTED: u32 = 1 << 3;
pub const AU6601_SD_DETECTED: u32 = 1 << 0;
pub const AU6601_DETECT_STATUS_M: u32 = 0xf;
pub const AU6601_REG_SW_RESET: u32 = 0x79;
pub const AU6601_BUF_CTRL_RESET: u32 = 1 << 7;
pub const AU6601_RESET_DATA: u32 = 1 << 3;
pub const AU6601_RESET_CMD: u32 = 1 << 0;
pub const AU6601_OUTPUT_ENABLE: u32 = 0x7a;
pub const AU6601_PAD_DRIVE0: u32 = 0x7b;
pub const AU6601_PAD_DRIVE1: u32 = 0x7c;
pub const AU6601_PAD_DRIVE2: u32 = 0x7d;
pub const AU6601_FUNCTION: u32 = 0x7f;
pub const AU6601_CMD_XFER_CTRL: u32 = 0x81;
pub const AU6601_CMD_17_BYTE_CRC: u32 = 0xc0;
pub const AU6601_CMD_6_BYTE_WO_CRC: u32 = 0x80;
pub const AU6601_CMD_6_BYTE_CRC: u32 = 0x40;
pub const AU6601_CMD_START_XFER: u32 = 0x20;
pub const AU6601_CMD_STOP_WAIT_RDY: u32 = 0x10;
pub const AU6601_CMD_NO_RESP: u32 = 0x00;
pub const AU6601_REG_BUS_CTRL: u32 = 0x82;
pub const AU6601_BUS_WIDTH_4BIT: u32 = 0x20;
pub const AU6601_BUS_WIDTH_8BIT: u32 = 0x10;
pub const AU6601_BUS_WIDTH_1BIT: u32 = 0x00;
pub const AU6601_DATA_XFER_CTRL: u32 = 0x83;
pub const AU6601_DATA_WRITE: u32 = 1 << 7;
pub const AU6601_DATA_DMA_MODE: u32 = 1 << 6;
pub const AU6601_DATA_START_XFER: u32 = 1 << 0;
pub const AU6601_DATA_PIN_STATE: u32 = 0x84;
pub const AU6601_BUS_STAT_CMD: u32 = 1 << 15;
pub const AU6601_BUS_STAT_DAT3: u32 = 1 << 3;
pub const AU6601_BUS_STAT_DAT2: u32 = 1 << 2;
pub const AU6601_BUS_STAT_DAT1: u32 = 1 << 1;
pub const AU6601_BUS_STAT_DAT0: u32 = 1 << 0;
pub const AU6601_BUS_STAT_DAT_MASK: u32 = 0xf;
pub const AU6601_OPT: u32 = 0x85;
pub const AU6601_OPT_CMD_LINE_LEVEL: u32 = 0x80;
pub const AU6601_OPT_NCRC_16_CLK: u32 = 1 << 4;
pub const AU6601_OPT_CMD_NWT: u32 = 1 << 3;
pub const AU6601_OPT_STOP_CLK: u32 = 1 << 2;
pub const AU6601_OPT_DDR_MODE: u32 = 1 << 1;
pub const AU6601_OPT_SD_18V: u32 = 1 << 0;
pub const AU6601_CLK_DELAY: u32 = 0x86;
pub const AU6601_CLK_DATA_POSITIVE_EDGE: u32 = 0x80;
pub const AU6601_CLK_CMD_POSITIVE_EDGE: u32 = 0x40;
pub const AU6601_CLK_POSITIVE_EDGE_ALL: u32 = AU6601_CLK_CMD_POSITIVE_EDGE | AU6601_CLK_DATA_POSITIVE_EDGE;

pub const AU6601_REG_INT_STATUS: u32 = 0x90;
pub const AU6601_REG_INT_ENABLE: u32 = 0x94;
pub const AU6601_INT_DATA_END_BIT_ERR: u32 = 1 << 22;
pub const AU6601_INT_DATA_CRC_ERR: u32 = 1 << 21;
pub const AU6601_INT_DATA_TIMEOUT_ERR: u32 = 1 << 20;
pub const AU6601_INT_CMD_INDEX_ERR: u32 = 1 << 19;
pub const AU6601_INT_CMD_END_BIT_ERR: u32 = 1 << 18;
pub const AU6601_INT_CMD_CRC_ERR: u32 = 1 << 17;
pub const AU6601_INT_CMD_TIMEOUT_ERR: u32 = 1 << 16;
pub const AU6601_INT_ERROR: u32 = 1 << 15;
pub const AU6601_INT_OVER_CURRENT_ERR: u32 = 1 << 8;
pub const AU6601_INT_CARD_INSERT: u32 = 1 << 7;
pub const AU6601_INT_CARD_REMOVE: u32 = 1 << 6;
pub const AU6601_INT_READ_BUF_RDY: u32 = 1 << 5;
pub const AU6601_INT_WRITE_BUF_RDY: u32 = 1 << 4;
pub const AU6601_INT_DMA_END: u32 = 1 << 3;
pub const AU6601_INT_DATA_END: u32 = 1 << 1;
pub const AU6601_INT_CMD_END: u32 = 1 << 0;
pub const AU6601_INT_NORMAL_MASK: u32 = 0x00007FFF;
pub const AU6601_INT_ERROR_MASK: u32 = 0xFFFF8000;
pub const AU6601_INT_CMD_MASK: u32 = AU6601_INT_CMD_END | AU6601_INT_CMD_TIMEOUT_ERR | AU6601_INT_CMD_CRC_ERR | AU6601_INT_CMD_END_BIT_ERR | AU6601_INT_CMD_INDEX_ERR;
pub const AU6601_INT_DATA_MASK: u32 = AU6601_INT_DATA_END | AU6601_INT_DMA_END | AU6601_INT_READ_BUF_RDY | AU6601_INT_WRITE_BUF_RDY | AU6601_INT_DATA_TIMEOUT_ERR | AU6601_INT_DATA_CRC_ERR | AU6601_INT_DATA_END_BIT_ERR;
pub const AU6601_INT_ALL_MASK: u32 = u32::MAX;

/* MS_CARD mode registers */
pub const AU6601_MS_STATUS: u32 = 0xa0;
pub const AU6601_MS_BUS_MODE_CTRL: u32 = 0xa1;
pub const AU6601_MS_BUS_8BIT_MODE: u32 = 0x03;
pub const AU6601_MS_BUS_4BIT_MODE: u32 = 0x01;
pub const AU6601_MS_BUS_1BIT_MODE: u32 = 0x00;
pub const AU6601_MS_TPC_CMD: u32 = 0xa2;
pub const AU6601_MS_TPC_READ_PAGE_DATA: u32 = 0x02;
pub const AU6601_MS_TPC_READ_REG: u32 = 0x04;
pub const AU6601_MS_TPC_GET_INT: u32 = 0x07;
pub const AU6601_MS_TPC_WRITE_PAGE_DATA: u32 = 0x0D;
pub const AU6601_MS_TPC_WRITE_REG: u32 = 0x0B;
pub const AU6601_MS_TPC_SET_RW_REG_ADRS: u32 = 0x08;
pub const AU6601_MS_TPC_SET_CMD: u32 = 0x0E;
pub const AU6601_MS_TPC_EX_SET_CMD: u32 = 0x09;
pub const AU6601_MS_TPC_READ_SHORT_DATA: u32 = 0x03;
pub const AU6601_MS_TPC_WRITE_SHORT_DATA: u32 = 0x0C;
pub const AU6601_MS_TRANSFER_MODE: u32 = 0xa3;
pub const AU6601_MS_XFER_INT_TIMEOUT_CHK: u32 = 1 << 2;
pub const AU6601_MS_XFER_DMA_ENABLE: u32 = 1 << 1;
pub const AU6601_MS_XFER_START: u32 = 1 << 0;
pub const AU6601_MS_DATA_PIN_STATE: u32 = 0xa4;
pub const AU6601_MS_INT_STATUS: u32 = 0xb0;
pub const AU6601_MS_INT_ENABLE: u32 = 0xb4;
pub const AU6601_MS_INT_OVER_CURRENT_ERROR: u32 = 1 << 23;
pub const AU6601_MS_INT_DATA_CRC_ERROR: u32 = 1 << 21;
pub const AU6601_MS_INT_INT_TIMEOUT: u32 = 1 << 20;
pub const AU6601_MS_INT_INT_RESP_ERROR: u32 = 1 << 19;
pub const AU6601_MS_INT_CED_ERROR: u32 = 1 << 18;
pub const AU6601_MS_INT_TPC_TIMEOUT: u32 = 1 << 16;
pub const AU6601_MS_INT_ERROR: u32 = 1 << 15;
pub const AU6601_MS_INT_CARD_INSERT: u32 = 1 << 7;
pub const AU6601_MS_INT_CARD_REMOVE: u32 = 1 << 6;
pub const AU6601_MS_INT_BUF_READ_RDY: u32 = 1 << 5;
pub const AU6601_MS_INT_BUF_WRITE_RDY: u32 = 1 << 4;
pub const AU6601_MS_INT_DMA_END: u32 = 1 << 3;
pub const AU6601_MS_INT_TPC_END: u32 = 1 << 1;
pub const AU6601_MS_INT_DATA_MASK: u32 = 0x00000038;
pub const AU6601_MS_INT_TPC_MASK: u32 = 0x003d8002;
pub const AU6601_MS_INT_TPC_ERROR: u32 = 0x003d0000;

pub const ALCOR_PCIE_LINK_CTRL_OFFSET: u32 = 0x10;
pub const ALCOR_PCIE_LINK_CAP_OFFSET: u32 = 0x0c;
pub const ALCOR_CAP_START_OFFSET: u32 = 0x34;

#[repr(C)]
pub struct alcor_dev_cfg {
    pub dma: u8,
}

#[repr(C)]
pub struct alcor_pci_priv {
    pub pdev: *mut pci_dev,
    pub parent_pdev: *mut pci_dev,
    pub dev: *mut device,
    pub iobase: *mut core::ffi::c_void,
    pub irq: u32,
    pub id: usize,
    pub cfg: *mut alcor_dev_cfg,
}

unsafe extern "C" {
    pub fn alcor_write8(priv_: *mut alcor_pci_priv, val: u8, addr: u32);
    pub fn alcor_write16(priv_: *mut alcor_pci_priv, val: u16, addr: u32);
    pub fn alcor_write32(priv_: *mut alcor_pci_priv, val: u32, addr: u32);
    pub fn alcor_write32be(priv_: *mut alcor_pci_priv, val: u32, addr: u32);
    pub fn alcor_read8(priv_: *mut alcor_pci_priv, addr: u32) -> u8;
    pub fn alcor_read32(priv_: *mut alcor_pci_priv, addr: u32) -> u32;
    pub fn alcor_read32be(priv_: *mut alcor_pci_priv, addr: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
