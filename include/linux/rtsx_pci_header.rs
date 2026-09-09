/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of rtsx_pci.h. External kernel types/functions remain dependencies. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Driver for Realtek PCI-Express card reader
 *
 * Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
 *
 * Author:
 *   Wei WANG <wei_wang@realsil.com.cn>
 */

// #ifndef __RTSX_PCI_H
#define __RTSX_PCI_H

// #include <linux/sched.h>
// #include <linux/pci.h>
// #include <linux/rtsx_common.h>

pub const MAX_RW_REG_CNT: u32 = 1024;

pub const RTSX_HCBAR: u32 = 0x00;
pub const RTSX_HCBCTLR: u32 = 0x04;
// C macro: #define   STOP_CMD			(0x01 << 28)
pub const READ_REG_CMD: u32 = 0;
pub const WRITE_REG_CMD: u32 = 1;
pub const CHECK_REG_CMD: u32 = 2;

pub const RTSX_HDBAR: u32 = 0x08;
pub const RTSX_SG_INT: u32 = 0x04;
pub const RTSX_SG_END: u32 = 0x02;
pub const RTSX_SG_VALID: u32 = 0x01;
pub const RTSX_SG_NO_OP: u32 = 0x00;
// C macro: #define   RTSX_SG_TRANS_DATA		(0x02 << 4)
// C macro: #define   RTSX_SG_LINK_DESC		(0x03 << 4)
pub const RTSX_HDBCTLR: u32 = 0x0C;
pub const SDMA_MODE: u32 = 0x00;
// C macro: #define   ADMA_MODE			(0x02 << 26)
// C macro: #define   STOP_DMA			(0x01 << 28)
// C macro: #define   TRIG_DMA			(0x01 << 31)

pub const RTSX_HAIMR: u32 = 0x10;
// C macro: #define   HAIMR_TRANS_START		(0x01 << 31)
pub const HAIMR_READ: u32 = 0x00;
// C macro: #define   HAIMR_WRITE			(0x01 << 30)
// C macro: #define   HAIMR_READ_START		(HAIMR_TRANS_START | HAIMR_READ)
// C macro: #define   HAIMR_WRITE_START		(HAIMR_TRANS_START | HAIMR_WRITE)
// C macro: #define   HAIMR_TRANS_END			(HAIMR_TRANS_START)

pub const RTSX_BIPR: u32 = 0x14;
// C macro: #define   CMD_DONE_INT			(1 << 31)
// C macro: #define   DATA_DONE_INT			(1 << 30)
// C macro: #define   TRANS_OK_INT			(1 << 29)
// C macro: #define   TRANS_FAIL_INT		(1 << 28)
// C macro: #define   XD_INT			(1 << 27)
// C macro: #define   MS_INT			(1 << 26)
// C macro: #define   SD_INT			(1 << 25)
// C macro: #define   GPIO0_INT			(1 << 24)
// C macro: #define   OC_INT			(1 << 23)
// C macro: #define   SD_WRITE_PROTECT		(1 << 19)
// C macro: #define   XD_EXIST			(1 << 18)
// C macro: #define   MS_EXIST			(1 << 17)
// C macro: #define   SD_EXIST			(1 << 16)
pub const DELINK_INT: u32 = GPIO0_INT;
// C macro: #define   MS_OC_INT			(1 << 23)
// C macro: #define   SD_OVP_INT		(1 << 23)
// C macro: #define   SD_OC_INT			(1 << 22)

// C macro: #define CARD_INT		(XD_INT | MS_INT | SD_INT)
// C macro: #define NEED_COMPLETE_INT	(DATA_DONE_INT | TRANS_OK_INT | TRANS_FAIL_INT)
// C macro: #define RTSX_INT		(CMD_DONE_INT | NEED_COMPLETE_INT | \
					CARD_INT | GPIO0_INT | OC_INT)
// C macro: #define CARD_EXIST		(XD_EXIST | MS_EXIST | SD_EXIST)

pub const RTSX_BIER: u32 = 0x18;
// C macro: #define   CMD_DONE_INT_EN		(1 << 31)
// C macro: #define   DATA_DONE_INT_EN		(1 << 30)
// C macro: #define   TRANS_OK_INT_EN		(1 << 29)
// C macro: #define   TRANS_FAIL_INT_EN		(1 << 28)
// C macro: #define   XD_INT_EN			(1 << 27)
// C macro: #define   MS_INT_EN			(1 << 26)
// C macro: #define   SD_INT_EN			(1 << 25)
// C macro: #define   GPIO0_INT_EN			(1 << 24)
// C macro: #define   OC_INT_EN			(1 << 23)
pub const DELINK_INT_EN: u32 = GPIO0_INT_EN;
// C macro: #define   MS_OC_INT_EN			(1 << 23)
// C macro: #define   SD_OVP_INT_EN			(1 << 23)
// C macro: #define   SD_OC_INT_EN			(1 << 22)

pub const RTSX_DUM_REG: u32 = 0x1C;

/*
 * macros for easy use
 */
#define rtsx_pci_writel(pcr, reg, value) \
	iowrite32(value, (pcr)->remap_addr + reg)
#define rtsx_pci_readl(pcr, reg) \
	ioread32((pcr)->remap_addr + reg)
#define rtsx_pci_writew(pcr, reg, value) \
	iowrite16(value, (pcr)->remap_addr + reg)
#define rtsx_pci_readw(pcr, reg) \
	ioread16((pcr)->remap_addr + reg)
#define rtsx_pci_writeb(pcr, reg, value) \
	iowrite8(value, (pcr)->remap_addr + reg)
#define rtsx_pci_readb(pcr, reg) \
	ioread8((pcr)->remap_addr + reg)

pub const STATE_TRANS_NONE: u32 = 0;
pub const STATE_TRANS_CMD: u32 = 1;
pub const STATE_TRANS_BUF: u32 = 2;
pub const STATE_TRANS_SG: u32 = 3;

pub const TRANS_NOT_READY: u32 = 0;
pub const TRANS_RESULT_OK: u32 = 1;
pub const TRANS_RESULT_FAIL: u32 = 2;
pub const TRANS_NO_DEVICE: u32 = 3;

pub const RTSX_RESV_BUF_LEN: u32 = 4096;
pub const HOST_CMDS_BUF_LEN: u32 = 1024;
// C macro: #define HOST_SG_TBL_BUF_LEN		(RTSX_RESV_BUF_LEN - HOST_CMDS_BUF_LEN)
// C macro: #define HOST_SG_TBL_ITEMS		(HOST_SG_TBL_BUF_LEN / 8)
pub const MAX_SG_ITEM_LEN: u32 = 0x80000;
pub const HOST_TO_DEVICE: u32 = 0;
pub const DEVICE_TO_HOST: u32 = 1;

pub const OUTPUT_3V3: u32 = 0;
pub const OUTPUT_1V8: u32 = 1;

pub const RTSX_PHASE_MAX: u32 = 32;
pub const RX_TUNING_CNT: u32 = 3;

pub const MS_CFG: u32 = 0xFD40;
pub const SAMPLE_TIME_RISING: u32 = 0x00;
pub const SAMPLE_TIME_FALLING: u32 = 0x80;
pub const PUSH_TIME_DEFAULT: u32 = 0x00;
pub const PUSH_TIME_ODD: u32 = 0x40;
pub const NO_EXTEND_TOGGLE: u32 = 0x00;
pub const EXTEND_TOGGLE_CHK: u32 = 0x20;
pub const MS_BUS_WIDTH_1: u32 = 0x00;
pub const MS_BUS_WIDTH_4: u32 = 0x10;
pub const MS_BUS_WIDTH_8: u32 = 0x18;
pub const MS_2K_SECTOR_MODE: u32 = 0x04;
pub const MS_512_SECTOR_MODE: u32 = 0x00;
pub const MS_TOGGLE_TIMEOUT_EN: u32 = 0x00;
pub const MS_TOGGLE_TIMEOUT_DISEN: u32 = 0x01;
pub const MS_NO_CHECK_INT: u32 = 0x02;
pub const MS_TPC: u32 = 0xFD41;
pub const MS_TRANS_CFG: u32 = 0xFD42;
pub const WAIT_INT: u32 = 0x80;
pub const NO_WAIT_INT: u32 = 0x00;
pub const NO_AUTO_READ_INT_REG: u32 = 0x00;
pub const AUTO_READ_INT_REG: u32 = 0x40;
pub const MS_CRC16_ERR: u32 = 0x20;
pub const MS_RDY_TIMEOUT: u32 = 0x10;
pub const MS_INT_CMDNK: u32 = 0x08;
pub const MS_INT_BREQ: u32 = 0x04;
pub const MS_INT_ERR: u32 = 0x02;
pub const MS_INT_CED: u32 = 0x01;
pub const MS_TRANSFER: u32 = 0xFD43;
pub const MS_TRANSFER_START: u32 = 0x80;
pub const MS_TRANSFER_END: u32 = 0x40;
pub const MS_TRANSFER_ERR: u32 = 0x20;
pub const MS_BS_STATE: u32 = 0x10;
pub const MS_TM_READ_BYTES: u32 = 0x00;
pub const MS_TM_NORMAL_READ: u32 = 0x01;
pub const MS_TM_WRITE_BYTES: u32 = 0x04;
pub const MS_TM_NORMAL_WRITE: u32 = 0x05;
pub const MS_TM_AUTO_READ: u32 = 0x08;
pub const MS_TM_AUTO_WRITE: u32 = 0x0C;
pub const MS_INT_REG: u32 = 0xFD44;
pub const MS_BYTE_CNT: u32 = 0xFD45;
pub const MS_SECTOR_CNT_L: u32 = 0xFD46;
pub const MS_SECTOR_CNT_H: u32 = 0xFD47;
pub const MS_DBUS_H: u32 = 0xFD48;

pub const SD_CFG1: u32 = 0xFDA0;
pub const SD_CLK_DIVIDE_0: u32 = 0x00;
pub const SD_CLK_DIVIDE_256: u32 = 0xC0;
pub const SD_CLK_DIVIDE_128: u32 = 0x80;
pub const SD_BUS_WIDTH_1BIT: u32 = 0x00;
pub const SD_BUS_WIDTH_4BIT: u32 = 0x01;
pub const SD_BUS_WIDTH_8BIT: u32 = 0x02;
pub const SD_ASYNC_FIFO_NOT_RST: u32 = 0x10;
pub const SD_20_MODE: u32 = 0x00;
pub const SD_DDR_MODE: u32 = 0x04;
pub const SD_30_MODE: u32 = 0x08;
pub const SD_CLK_DIVIDE_MASK: u32 = 0xC0;
pub const SD_MODE_SELECT_MASK: u32 = 0x0C;
pub const SD_CFG2: u32 = 0xFDA1;
pub const SD_CALCULATE_CRC7: u32 = 0x00;
pub const SD_NO_CALCULATE_CRC7: u32 = 0x80;
pub const SD_CHECK_CRC16: u32 = 0x00;
pub const SD_NO_CHECK_CRC16: u32 = 0x40;
pub const SD_NO_CHECK_WAIT_CRC_TO: u32 = 0x20;
pub const SD_WAIT_BUSY_END: u32 = 0x08;
pub const SD_NO_WAIT_BUSY_END: u32 = 0x00;
pub const SD_CHECK_CRC7: u32 = 0x00;
pub const SD_NO_CHECK_CRC7: u32 = 0x04;
pub const SD_RSP_LEN_0: u32 = 0x00;
pub const SD_RSP_LEN_6: u32 = 0x01;
pub const SD_RSP_LEN_17: u32 = 0x02;
pub const SD_RSP_TYPE_R0: u32 = 0x04;
pub const SD_RSP_TYPE_R1: u32 = 0x01;
pub const SD_RSP_TYPE_R1b: u32 = 0x09;
pub const SD_RSP_TYPE_R2: u32 = 0x02;
pub const SD_RSP_TYPE_R3: u32 = 0x05;
pub const SD_RSP_TYPE_R4: u32 = 0x05;
pub const SD_RSP_TYPE_R5: u32 = 0x01;
pub const SD_RSP_TYPE_R6: u32 = 0x01;
pub const SD_RSP_TYPE_R7: u32 = 0x01;
pub const SD_CFG3: u32 = 0xFDA2;
pub const SD30_CLK_END_EN: u32 = 0x10;
pub const SD_RSP_80CLK_TIMEOUT_EN: u32 = 0x01;

pub const SD_STAT1: u32 = 0xFDA3;
pub const SD_CRC7_ERR: u32 = 0x80;
pub const SD_CRC16_ERR: u32 = 0x40;
pub const SD_CRC_WRITE_ERR: u32 = 0x20;
pub const SD_CRC_WRITE_ERR_MASK: u32 = 0x1C;
pub const GET_CRC_TIME_OUT: u32 = 0x02;
pub const SD_TUNING_COMPARE_ERR: u32 = 0x01;
pub const SD_STAT2: u32 = 0xFDA4;
pub const SD_RSP_80CLK_TIMEOUT: u32 = 0x01;

pub const SD_BUS_STAT: u32 = 0xFDA5;
pub const SD_CLK_TOGGLE_EN: u32 = 0x80;
pub const SD_CLK_FORCE_STOP: u32 = 0x40;
pub const SD_DAT3_STATUS: u32 = 0x10;
pub const SD_DAT2_STATUS: u32 = 0x08;
pub const SD_DAT1_STATUS: u32 = 0x04;
pub const SD_DAT0_STATUS: u32 = 0x02;
pub const SD_CMD_STATUS: u32 = 0x01;
pub const SD_PAD_CTL: u32 = 0xFDA6;
pub const SD_IO_USING_1V8: u32 = 0x80;
pub const SD_IO_USING_3V3: u32 = 0x7F;
pub const TYPE_A_DRIVING: u32 = 0x00;
pub const TYPE_B_DRIVING: u32 = 0x01;
pub const TYPE_C_DRIVING: u32 = 0x02;
pub const TYPE_D_DRIVING: u32 = 0x03;
pub const SD_SAMPLE_POINT_CTL: u32 = 0xFDA7;
pub const DDR_FIX_RX_DAT: u32 = 0x00;
pub const DDR_VAR_RX_DAT: u32 = 0x80;
pub const DDR_FIX_RX_DAT_EDGE: u32 = 0x00;
pub const DDR_FIX_RX_DAT_14_DELAY: u32 = 0x40;
pub const DDR_FIX_RX_CMD: u32 = 0x00;
pub const DDR_VAR_RX_CMD: u32 = 0x20;
pub const DDR_FIX_RX_CMD_POS_EDGE: u32 = 0x00;
pub const DDR_FIX_RX_CMD_14_DELAY: u32 = 0x10;
pub const SD20_RX_POS_EDGE: u32 = 0x00;
pub const SD20_RX_14_DELAY: u32 = 0x08;
pub const SD20_RX_SEL_MASK: u32 = 0x08;
pub const SD_PUSH_POINT_CTL: u32 = 0xFDA8;
pub const DDR_FIX_TX_CMD_DAT: u32 = 0x00;
pub const DDR_VAR_TX_CMD_DAT: u32 = 0x80;
pub const DDR_FIX_TX_DAT_14_TSU: u32 = 0x00;
pub const DDR_FIX_TX_DAT_12_TSU: u32 = 0x40;
pub const DDR_FIX_TX_CMD_NEG_EDGE: u32 = 0x00;
pub const DDR_FIX_TX_CMD_14_AHEAD: u32 = 0x20;
pub const SD20_TX_NEG_EDGE: u32 = 0x00;
pub const SD20_TX_14_AHEAD: u32 = 0x10;
pub const SD20_TX_SEL_MASK: u32 = 0x10;
pub const DDR_VAR_SDCLK_POL_SWAP: u32 = 0x01;
pub const SD_CMD0: u32 = 0xFDA9;
pub const SD_CMD_START: u32 = 0x40;
pub const SD_CMD1: u32 = 0xFDAA;
pub const SD_CMD2: u32 = 0xFDAB;
pub const SD_CMD3: u32 = 0xFDAC;
pub const SD_CMD4: u32 = 0xFDAD;
pub const SD_CMD5: u32 = 0xFDAE;
pub const SD_BYTE_CNT_L: u32 = 0xFDAF;
pub const SD_BYTE_CNT_H: u32 = 0xFDB0;
pub const SD_BLOCK_CNT_L: u32 = 0xFDB1;
pub const SD_BLOCK_CNT_H: u32 = 0xFDB2;
pub const SD_TRANSFER: u32 = 0xFDB3;
pub const SD_TRANSFER_START: u32 = 0x80;
pub const SD_TRANSFER_END: u32 = 0x40;
pub const SD_STAT_IDLE: u32 = 0x20;
pub const SD_TRANSFER_ERR: u32 = 0x10;
pub const SD_TM_NORMAL_WRITE: u32 = 0x00;
pub const SD_TM_AUTO_WRITE_3: u32 = 0x01;
pub const SD_TM_AUTO_WRITE_4: u32 = 0x02;
pub const SD_TM_AUTO_READ_3: u32 = 0x05;
pub const SD_TM_AUTO_READ_4: u32 = 0x06;
pub const SD_TM_CMD_RSP: u32 = 0x08;
pub const SD_TM_AUTO_WRITE_1: u32 = 0x09;
pub const SD_TM_AUTO_WRITE_2: u32 = 0x0A;
pub const SD_TM_NORMAL_READ: u32 = 0x0C;
pub const SD_TM_AUTO_READ_1: u32 = 0x0D;
pub const SD_TM_AUTO_READ_2: u32 = 0x0E;
pub const SD_TM_AUTO_TUNING: u32 = 0x0F;
pub const SD_CMD_STATE: u32 = 0xFDB5;
pub const SD_CMD_IDLE: u32 = 0x80;

pub const SD_DATA_STATE: u32 = 0xFDB6;
pub const SD_DATA_IDLE: u32 = 0x80;
pub const REG_SD_STOP_SDCLK_CFG: u32 = 0xFDB8;
pub const SD30_CLK_STOP_CFG_EN: u32 = 0x04;
pub const SD30_CLK_STOP_CFG1: u32 = 0x02;
pub const SD30_CLK_STOP_CFG0: u32 = 0x01;
pub const REG_PRE_RW_MODE: u32 = 0xFD70;
pub const EN_INFINITE_MODE: u32 = 0x01;
pub const REG_CRC_DUMMY_0: u32 = 0xFD71;
// C macro: #define CFG_SD_POW_AUTO_PD		(1<<0)

pub const SRCTL: u32 = 0xFC13;

pub const DCM_DRP_CTL: u32 = 0xFC23;
pub const DCM_RESET: u32 = 0x08;
pub const DCM_LOCKED: u32 = 0x04;
pub const DCM_208M: u32 = 0x00;
pub const DCM_TX: u32 = 0x01;
pub const DCM_RX: u32 = 0x02;
pub const DCM_DRP_TRIG: u32 = 0xFC24;
pub const DRP_START: u32 = 0x80;
pub const DRP_DONE: u32 = 0x40;
pub const DCM_DRP_CFG: u32 = 0xFC25;
pub const DRP_WRITE: u32 = 0x80;
pub const DRP_READ: u32 = 0x00;
pub const DCM_WRITE_ADDRESS_50: u32 = 0x50;
pub const DCM_WRITE_ADDRESS_51: u32 = 0x51;
pub const DCM_READ_ADDRESS_00: u32 = 0x00;
pub const DCM_READ_ADDRESS_51: u32 = 0x51;
pub const DCM_DRP_WR_DATA_L: u32 = 0xFC26;
pub const DCM_DRP_WR_DATA_H: u32 = 0xFC27;
pub const DCM_DRP_RD_DATA_L: u32 = 0xFC28;
pub const DCM_DRP_RD_DATA_H: u32 = 0xFC29;
pub const SD_VPCLK0_CTL: u32 = 0xFC2A;
pub const SD_VPCLK1_CTL: u32 = 0xFC2B;
pub const PHASE_SELECT_MASK: u32 = 0x1F;
pub const SD_DCMPS0_CTL: u32 = 0xFC2C;
pub const SD_DCMPS1_CTL: u32 = 0xFC2D;
pub const SD_VPTX_CTL: u32 = SD_VPCLK0_CTL;
pub const SD_VPRX_CTL: u32 = SD_VPCLK1_CTL;
pub const PHASE_CHANGE: u32 = 0x80;
pub const PHASE_NOT_RESET: u32 = 0x40;
pub const SD_DCMPS_TX_CTL: u32 = SD_DCMPS0_CTL;
pub const SD_DCMPS_RX_CTL: u32 = SD_DCMPS1_CTL;
pub const DCMPS_CHANGE: u32 = 0x80;
pub const DCMPS_CHANGE_DONE: u32 = 0x40;
pub const DCMPS_ERROR: u32 = 0x20;
pub const DCMPS_CURRENT_PHASE: u32 = 0x1F;
pub const CARD_CLK_SOURCE: u32 = 0xFC2E;
// C macro: #define   CRC_FIX_CLK			(0x00 << 0)
// C macro: #define   CRC_VAR_CLK0			(0x01 << 0)
// C macro: #define   CRC_VAR_CLK1			(0x02 << 0)
// C macro: #define   SD30_FIX_CLK			(0x00 << 2)
// C macro: #define   SD30_VAR_CLK0			(0x01 << 2)
// C macro: #define   SD30_VAR_CLK1			(0x02 << 2)
// C macro: #define   SAMPLE_FIX_CLK		(0x00 << 4)
// C macro: #define   SAMPLE_VAR_CLK0		(0x01 << 4)
// C macro: #define   SAMPLE_VAR_CLK1		(0x02 << 4)
pub const CARD_PWR_CTL: u32 = 0xFD50;
pub const PMOS_STRG_MASK: u32 = 0x10;
pub const PMOS_STRG_800mA: u32 = 0x10;
pub const PMOS_STRG_400mA: u32 = 0x00;
pub const SD_POWER_OFF: u32 = 0x03;
pub const SD_PARTIAL_POWER_ON: u32 = 0x01;
pub const SD_POWER_ON: u32 = 0x00;
pub const SD_POWER_MASK: u32 = 0x03;
pub const MS_POWER_OFF: u32 = 0x0C;
pub const MS_PARTIAL_POWER_ON: u32 = 0x04;
pub const MS_POWER_ON: u32 = 0x00;
pub const MS_POWER_MASK: u32 = 0x0C;
pub const BPP_POWER_OFF: u32 = 0x0F;
pub const BPP_POWER_5_PERCENT_ON: u32 = 0x0E;
pub const BPP_POWER_10_PERCENT_ON: u32 = 0x0C;
pub const BPP_POWER_15_PERCENT_ON: u32 = 0x08;
pub const BPP_POWER_ON: u32 = 0x00;
pub const BPP_POWER_MASK: u32 = 0x0F;
pub const SD_VCC_PARTIAL_POWER_ON: u32 = 0x02;
pub const SD_VCC_POWER_ON: u32 = 0x00;
pub const CARD_CLK_SWITCH: u32 = 0xFD51;
pub const RTL8411B_PACKAGE_MODE: u32 = 0xFD51;
pub const CARD_SHARE_MODE: u32 = 0xFD52;
pub const CARD_SHARE_MASK: u32 = 0x0F;
pub const CARD_SHARE_MULTI_LUN: u32 = 0x00;
pub const CARD_SHARE_NORMAL: u32 = 0x00;
pub const CARD_SHARE_48_SD: u32 = 0x04;
pub const CARD_SHARE_48_MS: u32 = 0x08;
pub const CARD_SHARE_BAROSSA_SD: u32 = 0x01;
pub const CARD_SHARE_BAROSSA_MS: u32 = 0x02;
pub const CARD_DRIVE_SEL: u32 = 0xFD53;
// C macro: #define   MS_DRIVE_8mA			(0x01 << 6)
// C macro: #define   MMC_DRIVE_8mA			(0x01 << 4)
// C macro: #define   XD_DRIVE_8mA			(0x01 << 2)
pub const GPIO_DRIVE_8mA: u32 = 0x01;
// C macro: #define RTS5209_CARD_DRIVE_DEFAULT	(MS_DRIVE_8mA | MMC_DRIVE_8mA |\
					XD_DRIVE_8mA | GPIO_DRIVE_8mA)
// C macro: #define RTL8411_CARD_DRIVE_DEFAULT	(MS_DRIVE_8mA | MMC_DRIVE_8mA |\
					XD_DRIVE_8mA)
// C macro: #define RTSX_CARD_DRIVE_DEFAULT		(MS_DRIVE_8mA | GPIO_DRIVE_8mA)

pub const CARD_STOP: u32 = 0xFD54;
pub const SPI_STOP: u32 = 0x01;
pub const XD_STOP: u32 = 0x02;
pub const SD_STOP: u32 = 0x04;
pub const MS_STOP: u32 = 0x08;
pub const SPI_CLR_ERR: u32 = 0x10;
pub const XD_CLR_ERR: u32 = 0x20;
pub const SD_CLR_ERR: u32 = 0x40;
pub const MS_CLR_ERR: u32 = 0x80;
pub const CARD_OE: u32 = 0xFD55;
pub const SD_OUTPUT_EN: u32 = 0x04;
pub const MS_OUTPUT_EN: u32 = 0x08;
pub const CARD_AUTO_BLINK: u32 = 0xFD56;
pub const CARD_GPIO_DIR: u32 = 0xFD57;
pub const CARD_GPIO: u32 = 0xFD58;
pub const CARD_DATA_SOURCE: u32 = 0xFD5B;
pub const PINGPONG_BUFFER: u32 = 0x01;
pub const RING_BUFFER: u32 = 0x00;
pub const SD30_CLK_DRIVE_SEL: u32 = 0xFD5A;
pub const DRIVER_TYPE_A: u32 = 0x05;
pub const DRIVER_TYPE_B: u32 = 0x03;
pub const DRIVER_TYPE_C: u32 = 0x02;
pub const DRIVER_TYPE_D: u32 = 0x01;
pub const CARD_SELECT: u32 = 0xFD5C;
pub const SD_MOD_SEL: u32 = 2;
pub const MS_MOD_SEL: u32 = 3;
pub const SD30_DRIVE_SEL: u32 = 0xFD5E;
pub const CFG_DRIVER_TYPE_A: u32 = 0x02;
pub const CFG_DRIVER_TYPE_B: u32 = 0x03;
pub const CFG_DRIVER_TYPE_C: u32 = 0x01;
pub const CFG_DRIVER_TYPE_D: u32 = 0x00;
pub const SD30_CMD_DRIVE_SEL: u32 = 0xFD5E;
pub const SD30_DAT_DRIVE_SEL: u32 = 0xFD5F;
pub const CARD_CLK_EN: u32 = 0xFD69;
pub const SD_CLK_EN: u32 = 0x04;
pub const MS_CLK_EN: u32 = 0x08;
pub const SD40_CLK_EN: u32 = 0x10;
pub const SDIO_CTRL: u32 = 0xFD6B;
pub const CD_PAD_CTL: u32 = 0xFD73;
pub const CD_DISABLE_MASK: u32 = 0x07;
pub const MS_CD_DISABLE: u32 = 0x04;
pub const SD_CD_DISABLE: u32 = 0x02;
pub const XD_CD_DISABLE: u32 = 0x01;
pub const CD_DISABLE: u32 = 0x07;
pub const CD_ENABLE: u32 = 0x00;
pub const MS_CD_EN_ONLY: u32 = 0x03;
pub const SD_CD_EN_ONLY: u32 = 0x05;
pub const XD_CD_EN_ONLY: u32 = 0x06;
pub const FORCE_CD_LOW_MASK: u32 = 0x38;
pub const FORCE_CD_XD_LOW: u32 = 0x08;
pub const FORCE_CD_SD_LOW: u32 = 0x10;
pub const FORCE_CD_MS_LOW: u32 = 0x20;
pub const CD_AUTO_DISABLE: u32 = 0x40;
pub const FPDCTL: u32 = 0xFC00;
pub const SSC_POWER_DOWN: u32 = 0x01;
pub const SD_OC_POWER_DOWN: u32 = 0x02;
pub const ALL_POWER_DOWN: u32 = 0x03;
pub const OC_POWER_DOWN: u32 = 0x02;
pub const PDINFO: u32 = 0xFC01;

pub const CLK_CTL: u32 = 0xFC02;
pub const CHANGE_CLK: u32 = 0x01;
pub const CLK_LOW_FREQ: u32 = 0x01;

pub const CLK_DIV: u32 = 0xFC03;
pub const CLK_DIV_1: u32 = 0x01;
pub const CLK_DIV_2: u32 = 0x02;
pub const CLK_DIV_4: u32 = 0x03;
pub const CLK_DIV_8: u32 = 0x04;
pub const CLK_SEL: u32 = 0xFC04;

pub const SSC_DIV_N_0: u32 = 0xFC0F;
pub const SSC_DIV_N_1: u32 = 0xFC10;
pub const SSC_CTL1: u32 = 0xFC11;
pub const SSC_RSTB: u32 = 0x80;
pub const SSC_8X_EN: u32 = 0x40;
pub const SSC_FIX_FRAC: u32 = 0x20;
pub const SSC_SEL_1M: u32 = 0x00;
pub const SSC_SEL_2M: u32 = 0x08;
pub const SSC_SEL_4M: u32 = 0x10;
pub const SSC_SEL_8M: u32 = 0x18;
pub const SSC_CTL2: u32 = 0xFC12;
pub const SSC_DEPTH_MASK: u32 = 0x07;
pub const SSC_DEPTH_DISALBE: u32 = 0x00;
pub const SSC_DEPTH_4M: u32 = 0x01;
pub const SSC_DEPTH_2M: u32 = 0x02;
pub const SSC_DEPTH_1M: u32 = 0x03;
pub const SSC_DEPTH_500K: u32 = 0x04;
pub const SSC_DEPTH_250K: u32 = 0x05;
pub const RCCTL: u32 = 0xFC14;

pub const FPGA_PULL_CTL: u32 = 0xFC1D;
pub const OLT_LED_CTL: u32 = 0xFC1E;
pub const LED_SHINE_MASK: u32 = 0x08;
pub const LED_SHINE_EN: u32 = 0x08;
pub const LED_SHINE_DISABLE: u32 = 0x00;
pub const GPIO_CTL: u32 = 0xFC1F;

pub const LDO_CTL: u32 = 0xFC1E;
pub const BPP_ASIC_1V7: u32 = 0x00;
pub const BPP_ASIC_1V8: u32 = 0x01;
pub const BPP_ASIC_1V9: u32 = 0x02;
pub const BPP_ASIC_2V0: u32 = 0x03;
pub const BPP_ASIC_2V7: u32 = 0x04;
pub const BPP_ASIC_2V8: u32 = 0x05;
pub const BPP_ASIC_3V2: u32 = 0x06;
pub const BPP_ASIC_3V3: u32 = 0x07;
pub const BPP_REG_TUNED18: u32 = 0x07;
pub const BPP_TUNED18_SHIFT_8402: u32 = 5;
pub const BPP_TUNED18_SHIFT_8411: u32 = 4;
pub const BPP_PAD_MASK: u32 = 0x04;
pub const BPP_PAD_3V3: u32 = 0x04;
pub const BPP_PAD_1V8: u32 = 0x00;
pub const BPP_LDO_POWB: u32 = 0x03;
pub const BPP_LDO_ON: u32 = 0x00;
pub const BPP_LDO_SUSPEND: u32 = 0x02;
pub const BPP_LDO_OFF: u32 = 0x03;
pub const EFUSE_CTL: u32 = 0xFC30;
pub const EFUSE_ADD: u32 = 0xFC31;
pub const SYS_VER: u32 = 0xFC32;
pub const EFUSE_DATAL: u32 = 0xFC34;
pub const EFUSE_DATAH: u32 = 0xFC35;

pub const CARD_PULL_CTL1: u32 = 0xFD60;
pub const CARD_PULL_CTL2: u32 = 0xFD61;
pub const CARD_PULL_CTL3: u32 = 0xFD62;
pub const CARD_PULL_CTL4: u32 = 0xFD63;
pub const CARD_PULL_CTL5: u32 = 0xFD64;
pub const CARD_PULL_CTL6: u32 = 0xFD65;

/* PCI Express Related Registers */
pub const IRQEN0: u32 = 0xFE20;
pub const IRQSTAT0: u32 = 0xFE21;
pub const DMA_DONE_INT: u32 = 0x80;
pub const SUSPEND_INT: u32 = 0x40;
pub const LINK_RDY_INT: u32 = 0x20;
pub const LINK_DOWN_INT: u32 = 0x10;
pub const IRQEN1: u32 = 0xFE22;
pub const IRQSTAT1: u32 = 0xFE23;
pub const TLPRIEN: u32 = 0xFE24;
pub const TLPRISTAT: u32 = 0xFE25;
pub const TLPTIEN: u32 = 0xFE26;
pub const TLPTISTAT: u32 = 0xFE27;
pub const DMATC0: u32 = 0xFE28;
pub const DMATC1: u32 = 0xFE29;
pub const DMATC2: u32 = 0xFE2A;
pub const DMATC3: u32 = 0xFE2B;
pub const DMACTL: u32 = 0xFE2C;
pub const DMA_RST: u32 = 0x80;
pub const DMA_BUSY: u32 = 0x04;
pub const DMA_DIR_TO_CARD: u32 = 0x00;
pub const DMA_DIR_FROM_CARD: u32 = 0x02;
pub const DMA_EN: u32 = 0x01;
// C macro: #define   DMA_128			(0 << 4)
// C macro: #define   DMA_256			(1 << 4)
// C macro: #define   DMA_512			(2 << 4)
// C macro: #define   DMA_1024			(3 << 4)
pub const DMA_PACK_SIZE_MASK: u32 = 0x30;
pub const BCTL: u32 = 0xFE2D;
pub const RBBC0: u32 = 0xFE2E;
pub const RBBC1: u32 = 0xFE2F;
pub const RBDAT: u32 = 0xFE30;
pub const RBCTL: u32 = 0xFE34;
pub const U_AUTO_DMA_EN_MASK: u32 = 0x20;
pub const U_AUTO_DMA_DISABLE: u32 = 0x00;
pub const RB_FLUSH: u32 = 0x80;
pub const CFGADDR0: u32 = 0xFE35;
pub const CFGADDR1: u32 = 0xFE36;
pub const CFGDATA0: u32 = 0xFE37;
pub const CFGDATA1: u32 = 0xFE38;
pub const CFGDATA2: u32 = 0xFE39;
pub const CFGDATA3: u32 = 0xFE3A;
pub const CFGRWCTL: u32 = 0xFE3B;
pub const PHYRWCTL: u32 = 0xFE3C;
pub const PHYDATA0: u32 = 0xFE3D;
pub const PHYDATA1: u32 = 0xFE3E;
pub const PHYADDR: u32 = 0xFE3F;
pub const MSGRXDATA0: u32 = 0xFE40;
pub const MSGRXDATA1: u32 = 0xFE41;
pub const MSGRXDATA2: u32 = 0xFE42;
pub const MSGRXDATA3: u32 = 0xFE43;
pub const MSGTXDATA0: u32 = 0xFE44;
pub const MSGTXDATA1: u32 = 0xFE45;
pub const MSGTXDATA2: u32 = 0xFE46;
pub const MSGTXDATA3: u32 = 0xFE47;
pub const MSGTXCTL: u32 = 0xFE48;
pub const LTR_CTL: u32 = 0xFE4A;
pub const LTR_TX_EN_MASK: u32 = (1u32 << 7);
pub const LTR_TX_EN_1: u32 = (1u32 << 7);
pub const LTR_TX_EN_0: u32 = 0;
pub const LTR_LATENCY_MODE_MASK: u32 = (1u32 << 6);
pub const LTR_LATENCY_MODE_HW: u32 = 0;
pub const LTR_LATENCY_MODE_SW: u32 = (1u32 << 6);
pub const OBFF_CFG: u32 = 0xFE4C;
pub const OBFF_EN_MASK: u32 = 0x03;
pub const OBFF_DISABLE: u32 = 0x00;

pub const CDRESUMECTL: u32 = 0xFE52;
pub const CDGW: u32 = 0xFE53;
pub const WAKE_SEL_CTL: u32 = 0xFE54;
pub const PCLK_CTL: u32 = 0xFE55;
pub const PCLK_MODE_SEL: u32 = 0x20;
pub const PME_FORCE_CTL: u32 = 0xFE56;

pub const ASPM_FORCE_CTL: u32 = 0xFE57;
pub const FORCE_ASPM_CTL0: u32 = 0x10;
pub const FORCE_ASPM_CTL1: u32 = 0x20;
pub const FORCE_ASPM_VAL_MASK: u32 = 0x03;
pub const FORCE_ASPM_L1_EN: u32 = 0x02;
pub const FORCE_ASPM_L0_EN: u32 = 0x01;
pub const FORCE_ASPM_NO_ASPM: u32 = 0x00;
pub const PM_CLK_FORCE_CTL: u32 = 0xFE58;
pub const CLK_PM_EN: u32 = 0x01;
pub const FUNC_FORCE_CTL: u32 = 0xFE59;
pub const FUNC_FORCE_UPME_XMT_DBG: u32 = 0x02;
pub const PERST_GLITCH_WIDTH: u32 = 0xFE5C;
pub const CHANGE_LINK_STATE: u32 = 0xFE5B;
pub const RESET_LOAD_REG: u32 = 0xFE5E;
pub const EFUSE_CONTENT: u32 = 0xFE5F;
pub const HOST_SLEEP_STATE: u32 = 0xFE60;
pub const HOST_ENTER_S1: u32 = 1;
pub const HOST_ENTER_S3: u32 = 2;

pub const SDIO_CFG: u32 = 0xFE70;
pub const PM_EVENT_DEBUG: u32 = 0xFE71;
pub const PME_DEBUG_0: u32 = 0x08;
pub const NFTS_TX_CTRL: u32 = 0xFE72;

pub const PWR_GATE_CTRL: u32 = 0xFE75;
pub const PWR_GATE_EN: u32 = 0x01;
pub const LDO3318_PWR_MASK: u32 = 0x06;
pub const LDO_ON: u32 = 0x00;
pub const LDO_SUSPEND: u32 = 0x04;
pub const LDO_OFF: u32 = 0x06;
pub const PWD_SUSPEND_EN: u32 = 0xFE76;
pub const LDO_PWR_SEL: u32 = 0xFE78;

pub const L1SUB_CONFIG1: u32 = 0xFE8D;
pub const AUX_CLK_ACTIVE_SEL_MASK: u32 = 0x01;
pub const MAC_CKSW_DONE: u32 = 0x00;
pub const L1SUB_CONFIG2: u32 = 0xFE8E;
pub const L1SUB_AUTO_CFG: u32 = 0x02;
pub const L1SUB_CONFIG3: u32 = 0xFE8F;
pub const L1OFF_MBIAS2_EN_5250: u32 = (1u32 << 7);

pub const DUMMY_REG_RESET_0: u32 = 0xFE90;
pub const IC_VERSION_MASK: u32 = 0x0F;

pub const REG_VREF: u32 = 0xFE97;
pub const PWD_SUSPND_EN: u32 = 0x10;
pub const RTS5260_DMA_RST_CTL_0: u32 = 0xFEBF;
pub const RTS5260_DMA_RST: u32 = 0x80;
pub const RTS5260_ADMA3_RST: u32 = 0x40;
pub const AUTOLOAD_CFG_BASE: u32 = 0xFF00;
pub const RELINK_TIME_MASK: u32 = 0x01;
pub const PETXCFG: u32 = 0xFF03;
pub const FORCE_CLKREQ_DELINK_MASK: u32 = (1u32 << 7);
pub const FORCE_CLKREQ_LOW: u32 = 0x80;
pub const FORCE_CLKREQ_HIGH: u32 = 0x00;

pub const PM_CTRL1: u32 = 0xFF44;
pub const CD_RESUME_EN_MASK: u32 = 0xF0;

pub const PM_CTRL2: u32 = 0xFF45;
pub const PM_CTRL3: u32 = 0xFF46;
pub const SDIO_SEND_PME_EN: u32 = 0x80;
pub const FORCE_RC_MODE_ON: u32 = 0x40;
pub const FORCE_RX50_LINK_ON: u32 = 0x20;
pub const D3_DELINK_MODE_EN: u32 = 0x10;
pub const USE_PESRTB_CTL_DELINK: u32 = 0x08;
pub const DELAY_PIN_WAKE: u32 = 0x04;
pub const RESET_PIN_WAKE: u32 = 0x02;
pub const PM_WAKE_EN: u32 = 0x01;
pub const PM_CTRL4: u32 = 0xFF47;

/* FW config info register */
pub const RTS5261_FW_CFG_INFO0: u32 = 0xFF50;
// C macro: #define   RTS5261_FW_EXPRESS_TEST_MASK	(0x01 << 0)
// C macro: #define   RTS5261_FW_EA_MODE_MASK	(0x01 << 5)
pub const RTS5261_FW_CFG0: u32 = 0xFF54;
// C macro: #define   RTS5261_FW_ENTER_EXPRESS	(0x01 << 0)

pub const RTS5261_FW_CFG1: u32 = 0xFF55;
// C macro: #define   RTS5261_SYS_CLK_SEL_MCU_CLK	(0x01 << 7)
// C macro: #define   RTS5261_CRC_CLK_SEL_MCU_CLK	(0x01 << 6)
// C macro: #define   RTS5261_FAKE_MCU_CLOCK_GATING	(0x01 << 5)
// C macro: #define   RTS5261_MCU_BUS_SEL_MASK	(0x01 << 4)
// C macro: #define   RTS5261_MCU_CLOCK_SEL_MASK	(0x03 << 2)
// C macro: #define   RTS5261_MCU_CLOCK_SEL_16M	(0x01 << 2)
// C macro: #define   RTS5261_MCU_CLOCK_GATING	(0x01 << 1)
// C macro: #define   RTS5261_DRIVER_ENABLE_FW	(0x01 << 0)

pub const REG_CFG_OOBS_OFF_TIMER: u32 = 0xFEA6;
pub const REG_CFG_OOBS_ON_TIMER: u32 = 0xFEA7;
pub const REG_CFG_VCM_ON_TIMER: u32 = 0xFEA8;
pub const REG_CFG_OOBS_POLLING: u32 = 0xFEA9;

/* Memory mapping */
pub const SRAM_BASE: u32 = 0xE600;
pub const RBUF_BASE: u32 = 0xF400;
pub const PPBUF_BASE1: u32 = 0xF800;
pub const PPBUF_BASE2: u32 = 0xFA00;
pub const IMAGE_FLAG_ADDR0: u32 = 0xCE80;
pub const IMAGE_FLAG_ADDR1: u32 = 0xCE81;

pub const RREF_CFG: u32 = 0xFF6C;
pub const RREF_VBGSEL_MASK: u32 = 0x38;
pub const RREF_VBGSEL_1V25: u32 = 0x28;

pub const OOBS_CONFIG: u32 = 0xFF6E;
pub const OOBS_AUTOK_DIS: u32 = 0x80;
pub const OOBS_VAL_MASK: u32 = 0x1F;

pub const LDO_DV18_CFG: u32 = 0xFF70;
pub const LDO_DV18_SR_MASK: u32 = 0xC0;
pub const LDO_DV18_SR_DF: u32 = 0x40;
pub const DV331812_MASK: u32 = 0x70;
pub const DV331812_33: u32 = 0x70;
pub const DV331812_17: u32 = 0x30;

pub const LDO_CONFIG2: u32 = 0xFF71;
pub const LDO_D3318_MASK: u32 = 0x07;
pub const LDO_D3318_33V: u32 = 0x07;
pub const LDO_D3318_18V: u32 = 0x02;
pub const DV331812_VDD1: u32 = 0x04;
pub const DV331812_POWERON: u32 = 0x08;
pub const DV331812_POWEROFF: u32 = 0x00;

pub const LDO_VCC_CFG0: u32 = 0xFF72;
pub const LDO_VCC_LMTVTH_MASK: u32 = 0x30;
pub const LDO_VCC_LMTVTH_2A: u32 = 0x10;
/*RTS5260*/
pub const RTS5260_DVCC_TUNE_MASK: u32 = 0x70;
pub const RTS5260_DVCC_33: u32 = 0x70;

/*RTS5261*/
pub const RTS5261_LDO1_CFG0: u32 = 0xFF72;
// C macro: #define   RTS5261_LDO1_OCP_THD_MASK	(0x07 << 5)
// C macro: #define   RTS5261_LDO1_OCP_EN		(0x01 << 4)
// C macro: #define   RTS5261_LDO1_OCP_LMT_THD_MASK	(0x03 << 2)
// C macro: #define   RTS5261_LDO1_OCP_LMT_EN	(0x01 << 1)

pub const LDO_VCC_CFG1: u32 = 0xFF73;
pub const LDO_VCC_REF_TUNE_MASK: u32 = 0x30;
pub const LDO_VCC_REF_1V2: u32 = 0x20;
pub const LDO_VCC_TUNE_MASK: u32 = 0x07;
pub const LDO_VCC_1V8: u32 = 0x04;
pub const LDO_VCC_3V3: u32 = 0x07;
pub const LDO_VCC_LMT_EN: u32 = 0x08;
/*RTS5260*/
pub const LDO_POW_SDVDD1_MASK: u32 = 0x08;
pub const LDO_POW_SDVDD1_ON: u32 = 0x08;
pub const LDO_POW_SDVDD1_OFF: u32 = 0x00;

pub const LDO_VIO_CFG: u32 = 0xFF75;
pub const LDO_VIO_SR_MASK: u32 = 0xC0;
pub const LDO_VIO_SR_DF: u32 = 0x40;
pub const LDO_VIO_REF_TUNE_MASK: u32 = 0x30;
pub const LDO_VIO_REF_1V2: u32 = 0x20;
pub const LDO_VIO_TUNE_MASK: u32 = 0x07;
pub const LDO_VIO_1V7: u32 = 0x03;
pub const LDO_VIO_1V8: u32 = 0x04;
pub const LDO_VIO_3V3: u32 = 0x07;

pub const LDO_DV12S_CFG: u32 = 0xFF76;
pub const LDO_REF12_TUNE_MASK: u32 = 0x18;
pub const LDO_REF12_TUNE_DF: u32 = 0x10;
pub const LDO_D12_TUNE_MASK: u32 = 0x07;
pub const LDO_D12_TUNE_DF: u32 = 0x04;

pub const LDO_AV12S_CFG: u32 = 0xFF77;
pub const LDO_AV12S_TUNE_MASK: u32 = 0x07;
pub const LDO_AV12S_TUNE_DF: u32 = 0x04;

pub const SD40_LDO_CTL1: u32 = 0xFE7D;
pub const SD40_VIO_TUNE_MASK: u32 = 0x70;
pub const SD40_VIO_TUNE_1V7: u32 = 0x30;
pub const SD_VIO_LDO_1V8: u32 = 0x40;
pub const SD_VIO_LDO_3V3: u32 = 0x70;

pub const RTS5264_AUTOLOAD_CFG2: u32 = 0xFF7D;
// C macro: #define RTS5264_CHIP_RST_N_SEL		(1 << 6)

pub const RTS5260_AUTOLOAD_CFG4: u32 = 0xFF7F;
pub const RTS5260_MIMO_DISABLE: u32 = 0x8A;
/*RTS5261*/
// C macro: #define   RTS5261_AUX_CLK_16M_EN		(1 << 5)

pub const RTS5260_REG_GPIO_CTL0: u32 = 0xFC1A;
pub const RTS5260_REG_GPIO_MASK: u32 = 0x01;
pub const RTS5260_REG_GPIO_ON: u32 = 0x01;
pub const RTS5260_REG_GPIO_OFF: u32 = 0x00;

pub const PWR_GLOBAL_CTRL: u32 = 0xF200;
pub const PCIE_L1_2_EN: u32 = 0x0C;
pub const PCIE_L1_1_EN: u32 = 0x0A;
pub const PCIE_L1_0_EN: u32 = 0x09;
pub const PWR_FE_CTL: u32 = 0xF201;
pub const PCIE_L1_2_PD_FE_EN: u32 = 0x0C;
pub const PCIE_L1_1_PD_FE_EN: u32 = 0x0A;
pub const PCIE_L1_0_PD_FE_EN: u32 = 0x09;
pub const CFG_PCIE_APHY_OFF_0: u32 = 0xF204;
pub const CFG_PCIE_APHY_OFF_0_DEFAULT: u32 = 0xBF;
pub const CFG_PCIE_APHY_OFF_1: u32 = 0xF205;
pub const CFG_PCIE_APHY_OFF_1_DEFAULT: u32 = 0xFF;
pub const CFG_PCIE_APHY_OFF_2: u32 = 0xF206;
pub const CFG_PCIE_APHY_OFF_2_DEFAULT: u32 = 0x01;
pub const CFG_PCIE_APHY_OFF_3: u32 = 0xF207;
pub const CFG_PCIE_APHY_OFF_3_DEFAULT: u32 = 0x00;
pub const CFG_L1_0_PCIE_MAC_RET_VALUE: u32 = 0xF20C;
pub const CFG_L1_0_PCIE_DPHY_RET_VALUE: u32 = 0xF20E;
pub const CFG_L1_0_SYS_RET_VALUE: u32 = 0xF210;
pub const CFG_L1_0_CRC_MISC_RET_VALUE: u32 = 0xF212;
pub const CFG_L1_0_CRC_SD30_RET_VALUE: u32 = 0xF214;
pub const CFG_L1_0_CRC_SD40_RET_VALUE: u32 = 0xF216;
pub const CFG_LP_FPWM_VALUE: u32 = 0xF219;
pub const CFG_LP_FPWM_VALUE_DEFAULT: u32 = 0x18;
pub const PWC_CDR: u32 = 0xF253;
pub const PWC_CDR_DEFAULT: u32 = 0x03;
pub const CFG_L1_0_RET_VALUE_DEFAULT: u32 = 0x1B;
pub const CFG_L1_0_CRC_MISC_RET_VALUE_DEFAULT: u32 = 0x0C;

/* OCPCTL */
pub const SD_DETECT_EN: u32 = 0x08;
pub const SD_OCP_INT_EN: u32 = 0x04;
pub const SD_OCP_INT_CLR: u32 = 0x02;
pub const SD_OC_CLR: u32 = 0x01;

// C macro: #define SDVIO_DETECT_EN			(1 << 7)
// C macro: #define SDVIO_OCP_INT_EN		(1 << 6)
// C macro: #define SDVIO_OCP_INT_CLR		(1 << 5)
// C macro: #define SDVIO_OC_CLR			(1 << 4)

/* OCPSTAT */
pub const SD_OCP_DETECT: u32 = 0x08;
pub const SD_OC_NOW: u32 = 0x04;
pub const SD_OC_EVER: u32 = 0x02;

// C macro: #define SDVIO_OC_NOW			(1 << 6)
// C macro: #define SDVIO_OC_EVER			(1 << 5)

pub const REG_OCPCTL: u32 = 0xFD6A;
pub const REG_OCPSTAT: u32 = 0xFD6E;
pub const REG_OCPGLITCH: u32 = 0xFD6C;
pub const REG_OCPPARA1: u32 = 0xFD6B;
pub const REG_OCPPARA2: u32 = 0xFD6D;

/* rts5260 DV3318 OCP-related registers */
pub const REG_DV3318_OCPCTL: u32 = 0xFD89;
pub const DV3318_OCP_TIME_MASK: u32 = 0xF0;
pub const DV3318_DETECT_EN: u32 = 0x08;
pub const DV3318_OCP_INT_EN: u32 = 0x04;
pub const DV3318_OCP_INT_CLR: u32 = 0x02;
pub const DV3318_OCP_CLR: u32 = 0x01;

pub const REG_DV3318_OCPSTAT: u32 = 0xFD8A;
pub const DV3318_OCP_GlITCH_TIME_MASK: u32 = 0xF0;
pub const DV3318_OCP_DETECT: u32 = 0x08;
pub const DV3318_OCP_NOW: u32 = 0x04;
pub const DV3318_OCP_EVER: u32 = 0x02;

pub const SD_OCP_GLITCH_MASK: u32 = 0x0F;

/* OCPPARA1 */
pub const SDVIO_OCP_TIME_60: u32 = 0x00;
pub const SDVIO_OCP_TIME_100: u32 = 0x10;
pub const SDVIO_OCP_TIME_200: u32 = 0x20;
pub const SDVIO_OCP_TIME_400: u32 = 0x30;
pub const SDVIO_OCP_TIME_600: u32 = 0x40;
pub const SDVIO_OCP_TIME_800: u32 = 0x50;
pub const SDVIO_OCP_TIME_1100: u32 = 0x60;
pub const SDVIO_OCP_TIME_MASK: u32 = 0x70;

pub const SD_OCP_TIME_60: u32 = 0x00;
pub const SD_OCP_TIME_100: u32 = 0x01;
pub const SD_OCP_TIME_200: u32 = 0x02;
pub const SD_OCP_TIME_400: u32 = 0x03;
pub const SD_OCP_TIME_600: u32 = 0x04;
pub const SD_OCP_TIME_800: u32 = 0x05;
pub const SD_OCP_TIME_1100: u32 = 0x06;
pub const SD_OCP_TIME_MASK: u32 = 0x07;

/* OCPPARA2 */
pub const SDVIO_OCP_THD_190: u32 = 0x00;
pub const SDVIO_OCP_THD_250: u32 = 0x10;
pub const SDVIO_OCP_THD_320: u32 = 0x20;
pub const SDVIO_OCP_THD_380: u32 = 0x30;
pub const SDVIO_OCP_THD_440: u32 = 0x40;
pub const SDVIO_OCP_THD_500: u32 = 0x50;
pub const SDVIO_OCP_THD_570: u32 = 0x60;
pub const SDVIO_OCP_THD_630: u32 = 0x70;
pub const SDVIO_OCP_THD_MASK: u32 = 0x70;

pub const SD_OCP_THD_450: u32 = 0x00;
pub const SD_OCP_THD_550: u32 = 0x01;
pub const SD_OCP_THD_650: u32 = 0x02;
pub const SD_OCP_THD_750: u32 = 0x03;
pub const SD_OCP_THD_850: u32 = 0x04;
pub const SD_OCP_THD_950: u32 = 0x05;
pub const SD_OCP_THD_1050: u32 = 0x06;
pub const SD_OCP_THD_1150: u32 = 0x07;
pub const SD_OCP_THD_MASK: u32 = 0x07;

pub const SDVIO_OCP_GLITCH_MASK: u32 = 0xF0;
pub const SDVIO_OCP_GLITCH_NONE: u32 = 0x00;
pub const SDVIO_OCP_GLITCH_50U: u32 = 0x10;
pub const SDVIO_OCP_GLITCH_100U: u32 = 0x20;
pub const SDVIO_OCP_GLITCH_200U: u32 = 0x30;
pub const SDVIO_OCP_GLITCH_600U: u32 = 0x40;
pub const SDVIO_OCP_GLITCH_800U: u32 = 0x50;
pub const SDVIO_OCP_GLITCH_1M: u32 = 0x60;
pub const SDVIO_OCP_GLITCH_2M: u32 = 0x70;
pub const SDVIO_OCP_GLITCH_3M: u32 = 0x80;
pub const SDVIO_OCP_GLITCH_4M: u32 = 0x90;
pub const SDVIO_OCP_GLIVCH_5M: u32 = 0xA0;
pub const SDVIO_OCP_GLITCH_6M: u32 = 0xB0;
pub const SDVIO_OCP_GLITCH_7M: u32 = 0xC0;
pub const SDVIO_OCP_GLITCH_8M: u32 = 0xD0;
pub const SDVIO_OCP_GLITCH_9M: u32 = 0xE0;
pub const SDVIO_OCP_GLITCH_10M: u32 = 0xF0;

pub const SD_OCP_GLITCH_MASK: u32 = 0x0F;
pub const SD_OCP_GLITCH_NONE: u32 = 0x00;
pub const SD_OCP_GLITCH_50U: u32 = 0x01;
pub const SD_OCP_GLITCH_100U: u32 = 0x02;
pub const SD_OCP_GLITCH_200U: u32 = 0x03;
pub const SD_OCP_GLITCH_600U: u32 = 0x04;
pub const SD_OCP_GLITCH_800U: u32 = 0x05;
pub const SD_OCP_GLITCH_1M: u32 = 0x06;
pub const SD_OCP_GLITCH_2M: u32 = 0x07;
pub const SD_OCP_GLITCH_3M: u32 = 0x08;
pub const SD_OCP_GLITCH_4M: u32 = 0x09;
pub const SD_OCP_GLIVCH_5M: u32 = 0x0A;
pub const SD_OCP_GLITCH_6M: u32 = 0x0B;
pub const SD_OCP_GLITCH_7M: u32 = 0x0C;
pub const SD_OCP_GLITCH_8M: u32 = 0x0D;
pub const SD_OCP_GLITCH_9M: u32 = 0x0E;
pub const SD_OCP_GLITCH_10M: u32 = 0x0F;

/* Phy register */
pub const PHY_PCR: u32 = 0x00;
pub const PHY_PCR_FORCE_CODE: u32 = 0xB000;
pub const PHY_PCR_OOBS_CALI_50: u32 = 0x0800;
pub const PHY_PCR_OOBS_VCM_08: u32 = 0x0200;
pub const PHY_PCR_OOBS_SEN_90: u32 = 0x0040;
pub const PHY_PCR_RSSI_EN: u32 = 0x0002;
pub const PHY_PCR_RX10K: u32 = 0x0001;

pub const PHY_RCR0: u32 = 0x01;
pub const PHY_RCR1: u32 = 0x02;
pub const PHY_RCR1_ADP_TIME_4: u32 = 0x0400;
pub const PHY_RCR1_VCO_COARSE: u32 = 0x001F;
pub const PHY_RCR1_INIT_27S: u32 = 0x0A1F;
pub const PHY_SSCCR2: u32 = 0x02;
pub const PHY_SSCCR2_PLL_NCODE: u32 = 0x0A00;
pub const PHY_SSCCR2_TIME0: u32 = 0x001C;
pub const PHY_SSCCR2_TIME2_WIDTH: u32 = 0x0003;

pub const PHY_RCR2: u32 = 0x03;
pub const PHY_RCR2_EMPHASE_EN: u32 = 0x8000;
pub const PHY_RCR2_NADJR: u32 = 0x4000;
pub const PHY_RCR2_CDR_SR_2: u32 = 0x0100;
pub const PHY_RCR2_FREQSEL_12: u32 = 0x0040;
pub const PHY_RCR2_CDR_SC_12P: u32 = 0x0010;
pub const PHY_RCR2_CALIB_LATE: u32 = 0x0002;
pub const PHY_RCR2_INIT_27S: u32 = 0xC152;
pub const PHY_SSCCR3: u32 = 0x03;
pub const PHY_SSCCR3_STEP_IN: u32 = 0x2740;
pub const PHY_SSCCR3_CHECK_DELAY: u32 = 0x0008;
pub const _PHY_ANA03: u32 = 0x03;
pub const _PHY_ANA03_TIMER_MAX: u32 = 0x2700;
pub const _PHY_ANA03_OOBS_DEB_EN: u32 = 0x0040;
pub const _PHY_CMU_DEBUG_EN: u32 = 0x0008;

pub const PHY_RTCR: u32 = 0x04;
pub const PHY_RDR: u32 = 0x05;
pub const PHY_RDR_RXDSEL_1_9: u32 = 0x4000;
pub const PHY_SSC_AUTO_PWD: u32 = 0x0600;
pub const PHY_TCR0: u32 = 0x06;
pub const PHY_TCR1: u32 = 0x07;
pub const PHY_TUNE: u32 = 0x08;
pub const PHY_TUNE_TUNEREF_1_0: u32 = 0x4000;
pub const PHY_TUNE_VBGSEL_1252: u32 = 0x0C00;
pub const PHY_TUNE_SDBUS_33: u32 = 0x0200;
pub const PHY_TUNE_TUNED18: u32 = 0x01C0;
pub const PHY_TUNE_TUNED12: u32 = 0X0020;
pub const PHY_TUNE_TUNEA12: u32 = 0x0004;
pub const PHY_TUNE_VOLTAGE_MASK: u32 = 0xFC3F;
pub const PHY_TUNE_VOLTAGE_3V3: u32 = 0x03C0;
pub const PHY_TUNE_D18_1V8: u32 = 0x0100;
pub const PHY_TUNE_D18_1V7: u32 = 0x0080;
pub const PHY_ANA08: u32 = 0x08;
pub const PHY_ANA08_RX_EQ_DCGAIN: u32 = 0x5000;
pub const PHY_ANA08_SEL_RX_EN: u32 = 0x0400;
pub const PHY_ANA08_RX_EQ_VAL: u32 = 0x03C0;
pub const PHY_ANA08_SCP: u32 = 0x0020;
pub const PHY_ANA08_SEL_IPI: u32 = 0x0004;

pub const PHY_IMR: u32 = 0x09;
pub const PHY_BPCR: u32 = 0x0A;
pub const PHY_BPCR_IBRXSEL: u32 = 0x0400;
pub const PHY_BPCR_IBTXSEL: u32 = 0x0100;
pub const PHY_BPCR_IB_FILTER: u32 = 0x0080;
pub const PHY_BPCR_CMIRROR_EN: u32 = 0x0040;

pub const PHY_BIST: u32 = 0x0B;
pub const PHY_RAW_L: u32 = 0x0C;
pub const PHY_RAW_H: u32 = 0x0D;
pub const PHY_RAW_DATA: u32 = 0x0E;
pub const PHY_HOST_CLK_CTRL: u32 = 0x0F;
pub const PHY_DMR: u32 = 0x10;
pub const PHY_BACR: u32 = 0x11;
pub const PHY_BACR_BASIC_MASK: u32 = 0xFFF3;
pub const PHY_IER: u32 = 0x12;
pub const PHY_BCSR: u32 = 0x13;
pub const PHY_BPR: u32 = 0x14;
pub const PHY_BPNR2: u32 = 0x15;
pub const PHY_BPNR: u32 = 0x16;
pub const PHY_BRNR2: u32 = 0x17;
pub const PHY_BENR: u32 = 0x18;
pub const PHY_REV: u32 = 0x19;
pub const PHY_REV_RESV: u32 = 0xE000;
pub const PHY_REV_RXIDLE_LATCHED: u32 = 0x1000;
pub const PHY_REV_P1_EN: u32 = 0x0800;
pub const PHY_REV_RXIDLE_EN: u32 = 0x0400;
pub const PHY_REV_CLKREQ_TX_EN: u32 = 0x0200;
pub const PHY_REV_CLKREQ_RX_EN: u32 = 0x0100;
pub const PHY_REV_CLKREQ_DT_1_0: u32 = 0x0040;
pub const PHY_REV_STOP_CLKRD: u32 = 0x0020;
pub const PHY_REV_RX_PWST: u32 = 0x0008;
pub const PHY_REV_STOP_CLKWR: u32 = 0x0004;
pub const _PHY_REV0: u32 = 0x19;
pub const _PHY_REV0_FILTER_OUT: u32 = 0x3800;
pub const _PHY_REV0_CDR_BYPASS_PFD: u32 = 0x0100;
pub const _PHY_REV0_CDR_RX_IDLE_BYPASS: u32 = 0x0002;

pub const PHY_FLD0: u32 = 0x1A;
pub const PHY_ANA1A: u32 = 0x1A;
pub const PHY_ANA1A_TXR_LOOPBACK: u32 = 0x2000;
pub const PHY_ANA1A_RXT_BIST: u32 = 0x0500;
pub const PHY_ANA1A_TXR_BIST: u32 = 0x0040;
pub const PHY_ANA1A_REV: u32 = 0x0006;
pub const PHY_FLD0_INIT_27S: u32 = 0x2546;
pub const PHY_FLD1: u32 = 0x1B;
pub const PHY_FLD2: u32 = 0x1C;
pub const PHY_FLD3: u32 = 0x1D;
pub const PHY_FLD3_TIMER_4: u32 = 0x0800;
pub const PHY_FLD3_TIMER_6: u32 = 0x0020;
pub const PHY_FLD3_RXDELINK: u32 = 0x0004;
pub const PHY_FLD3_INIT_27S: u32 = 0x0004;
pub const PHY_ANA1D: u32 = 0x1D;
pub const PHY_ANA1D_DEBUG_ADDR: u32 = 0x0004;
pub const _PHY_FLD0: u32 = 0x1D;
pub const _PHY_FLD0_CLK_REQ_20C: u32 = 0x8000;
pub const _PHY_FLD0_RX_IDLE_EN: u32 = 0x1000;
pub const _PHY_FLD0_BIT_ERR_RSTN: u32 = 0x0800;
pub const _PHY_FLD0_BER_COUNT: u32 = 0x01E0;
pub const _PHY_FLD0_BER_TIMER: u32 = 0x001E;
pub const _PHY_FLD0_CHECK_EN: u32 = 0x0001;

pub const PHY_FLD4: u32 = 0x1E;
pub const PHY_FLD4_FLDEN_SEL: u32 = 0x4000;
pub const PHY_FLD4_REQ_REF: u32 = 0x2000;
pub const PHY_FLD4_RXAMP_OFF: u32 = 0x1000;
pub const PHY_FLD4_REQ_ADDA: u32 = 0x0800;
pub const PHY_FLD4_BER_COUNT: u32 = 0x00E0;
pub const PHY_FLD4_BER_TIMER: u32 = 0x000A;
pub const PHY_FLD4_BER_CHK_EN: u32 = 0x0001;
pub const PHY_FLD4_INIT_27S: u32 = 0x5C7F;
pub const PHY_DIG1E: u32 = 0x1E;
pub const PHY_DIG1E_REV: u32 = 0x4000;
pub const PHY_DIG1E_D0_X_D1: u32 = 0x1000;
pub const PHY_DIG1E_RX_ON_HOST: u32 = 0x0800;
pub const PHY_DIG1E_RCLK_REF_HOST: u32 = 0x0400;
pub const PHY_DIG1E_RCLK_TX_EN_KEEP: u32 = 0x0040;
pub const PHY_DIG1E_RCLK_TX_TERM_KEEP: u32 = 0x0020;
pub const PHY_DIG1E_RCLK_RX_EIDLE_ON: u32 = 0x0010;
pub const PHY_DIG1E_TX_TERM_KEEP: u32 = 0x0008;
pub const PHY_DIG1E_RX_TERM_KEEP: u32 = 0x0004;
pub const PHY_DIG1E_TX_EN_KEEP: u32 = 0x0002;
pub const PHY_DIG1E_RX_EN_KEEP: u32 = 0x0001;
pub const PHY_DUM_REG: u32 = 0x1F;

pub const PCR_SETTING_REG1: u32 = 0x724;
pub const PCR_SETTING_REG2: u32 = 0x814;
pub const PCR_SETTING_REG3: u32 = 0x747;
pub const PCR_SETTING_REG4: u32 = 0x818;
pub const PCR_SETTING_REG5: u32 = 0x81C;


#define rtsx_pci_init_cmd(pcr)		((pcr)->ci = 0)

pub const RTS5227_DEVICE_ID: u32 = 0x5227;
pub const RTS_MAX_TIMES_FREQ_REDUCTION: u32 = 8;

struct rtsx_pcr;

struct pcr_handle {
	struct rtsx_pcr			*pcr;
};

struct pcr_ops {
	int (*write_phy)(struct rtsx_pcr *pcr, u8 addr, u16 val);
	int (*read_phy)(struct rtsx_pcr *pcr, u8 addr, u16 *val);
	int		(*extra_init_hw)(struct rtsx_pcr *pcr);
	int		(*optimize_phy)(struct rtsx_pcr *pcr);
	int		(*turn_on_led)(struct rtsx_pcr *pcr);
	int		(*turn_off_led)(struct rtsx_pcr *pcr);
	int		(*enable_auto_blink)(struct rtsx_pcr *pcr);
	int		(*disable_auto_blink)(struct rtsx_pcr *pcr);
	int		(*card_power_on)(struct rtsx_pcr *pcr, int card);
	int		(*card_power_off)(struct rtsx_pcr *pcr, int card);
	int		(*switch_output_voltage)(struct rtsx_pcr *pcr,
						u8 voltage);
	unsigned int	(*cd_deglitch)(struct rtsx_pcr *pcr);
	int		(*conv_clk_and_div_n)(int clk, int dir);
	void		(*fetch_vendor_settings)(struct rtsx_pcr *pcr);
	void		(*force_power_down)(struct rtsx_pcr *pcr, u8 pm_state, bool runtime);
	void		(*stop_cmd)(struct rtsx_pcr *pcr);

	void (*set_aspm)(struct rtsx_pcr *pcr, bool enable);
	void (*set_l1off_cfg_sub_d0)(struct rtsx_pcr *pcr, int active);
	void (*enable_ocp)(struct rtsx_pcr *pcr);
	void (*disable_ocp)(struct rtsx_pcr *pcr);
	void (*init_ocp)(struct rtsx_pcr *pcr);
	void (*process_ocp)(struct rtsx_pcr *pcr);
	int (*get_ocpstat)(struct rtsx_pcr *pcr, u8 *val);
	void (*clear_ocpstat)(struct rtsx_pcr *pcr);
};

enum PDEV_STAT  {PDEV_STAT_IDLE, PDEV_STAT_RUN};
enum ASPM_MODE  {ASPM_MODE_CFG, ASPM_MODE_REG};

pub const ASPM_L1_1_EN: u32 = (1u32 << 0);
pub const ASPM_L1_2_EN: u32 = (1u32 << 1);
pub const PM_L1_1_EN: u32 = (1u32 << 2);
pub const PM_L1_2_EN: u32 = (1u32 << 3);
pub const LTR_L1SS_PWR_GATE_EN: u32 = (1u32 << 4);
pub const L1_SNOOZE_TEST_EN: u32 = (1u32 << 5);
pub const LTR_L1SS_PWR_GATE_CHECK_CARD_EN: u32 = (1u32 << 6);

/*
 * struct rtsx_cr_option  - card reader option
 * @dev_flags: device flags
 * @force_clkreq_0: force clock request
 * @ltr_en: enable ltr mode flag
 * @ltr_enabled: ltr mode in configure space flag
 * @ltr_active: ltr mode status
 * @ltr_active_latency: ltr mode active latency
 * @ltr_idle_latency: ltr mode idle latency
 * @ltr_l1off_latency: ltr mode l1off latency
 * @l1_snooze_delay: l1 snooze delay
 * @ltr_l1off_sspwrgate: ltr l1off sspwrgate
 * @ltr_l1off_snooze_sspwrgate: ltr l1off snooze sspwrgate
 * @ocp_en: enable ocp flag
 * @sd_400mA_ocp_thd: 400mA ocp thd
 * @sd_800mA_ocp_thd: 800mA ocp thd
 */
struct rtsx_cr_option {
	u32 dev_flags;
	bool force_clkreq_0;
	bool ltr_en;
	bool ltr_enabled;
	bool ltr_active;
	u32 ltr_active_latency;
	u32 ltr_idle_latency;
	u32 ltr_l1off_latency;
	u32 l1_snooze_delay;
	u8 ltr_l1off_sspwrgate;
	u8 ltr_l1off_snooze_sspwrgate;
	bool ocp_en;
	u8 sd_400mA_ocp_thd;
	u8 sd_800mA_ocp_thd;
	u8 sd_cd_reverse_en;
	u8 sd_wp_reverse_en;
};

/*
 * struct rtsx_hw_param  - card reader hardware param
 * @interrupt_en: indicate which interrutp enable
 * @ocp_glitch: ocp glitch time
 */
struct rtsx_hw_param {
	u32 interrupt_en;
	u8 ocp_glitch;
};

#define rtsx_set_dev_flag(cr, flag) \
	((cr)->option.dev_flags |= (flag))
#define rtsx_clear_dev_flag(cr, flag) \
	((cr)->option.dev_flags &= ~(flag))
#define rtsx_check_dev_flag(cr, flag) \
	((cr)->option.dev_flags & (flag))

struct rtsx_pcr {
	struct pci_dev			*pci;
	unsigned int			id;
	struct rtsx_cr_option	option;
	struct rtsx_hw_param hw_param;

	/* pci resources */
	unsigned long			addr;
	void __iomem			*remap_addr;
	int				irq;

	/* host reserved buffer */
	void				*rtsx_resv_buf;
	dma_addr_t			rtsx_resv_buf_addr;

	void				*host_cmds_ptr;
	dma_addr_t			host_cmds_addr;
	int				ci;

	void				*host_sg_tbl_ptr;
	dma_addr_t			host_sg_tbl_addr;
	int				sgi;

	u32				bier;
	char				trans_result;

	unsigned int			card_inserted;
	unsigned int			card_removed;
	unsigned int			card_exist;

	struct delayed_work		carddet_work;

	spinlock_t			lock;
	struct mutex			pcr_mutex;
	struct completion		*done;
	struct completion		*finish_me;

	unsigned int			cur_clock;
	bool				remove_pci;
	bool				msi_en;

// C macro: #define EXTRA_CAPS_SD_SDR50		(1 << 0)
// C macro: #define EXTRA_CAPS_SD_SDR104		(1 << 1)
// C macro: #define EXTRA_CAPS_SD_DDR50		(1 << 2)
// C macro: #define EXTRA_CAPS_MMC_HSDDR		(1 << 3)
// C macro: #define EXTRA_CAPS_MMC_HS200		(1 << 4)
// C macro: #define EXTRA_CAPS_MMC_8BIT		(1 << 5)
// C macro: #define EXTRA_CAPS_NO_MMC		(1 << 7)
// C macro: #define EXTRA_CAPS_SD_EXPRESS		(1 << 8)
	u32				extra_caps;

pub const IC_VER_A: u32 = 0;
pub const IC_VER_B: u32 = 1;
pub const IC_VER_C: u32 = 2;
pub const IC_VER_D: u32 = 3;
	u8				ic_version;

	u8				sd30_drive_sel_1v8;
	u8				sd30_drive_sel_3v3;
	u8				card_drive_sel;
pub const ASPM_L1_EN: u32 = 0x02;
	u8				aspm_en;
	enum ASPM_MODE			aspm_mode;
	bool				aspm_enabled;

// C macro: #define PCR_MS_PMOS			(1 << 0)
// C macro: #define PCR_REVERSE_SOCKET		(1 << 1)
	u32				flags;

	u32				tx_initial_phase;
	u32				rx_initial_phase;

	const u32			*sd_pull_ctl_enable_tbl;
	const u32			*sd_pull_ctl_disable_tbl;
	const u32			*ms_pull_ctl_enable_tbl;
	const u32			*ms_pull_ctl_disable_tbl;

	const struct pcr_ops		*ops;
	enum PDEV_STAT			state;

	u16				reg_pm_ctrl3;

	int				num_slots;
	struct rtsx_slot		*slots;

	u8				dma_error_count;
	u8			ocp_stat;
	u8			ocp_stat2;
	u8			ovp_stat;
	u8			rtd3_en;
};

pub const PID_524A: u32 = 0x524A;
pub const PID_5249: u32 = 0x5249;
pub const PID_5250: u32 = 0x5250;
pub const PID_525A: u32 = 0x525A;
pub const PID_5260: u32 = 0x5260;
pub const PID_5261: u32 = 0x5261;
pub const PID_5228: u32 = 0x5228;
pub const PID_5264: u32 = 0x5264;

#define CHK_PCI_PID(pcr, pid)		((pcr)->pci->device == (pid))
#define PCI_VID(pcr)			((pcr)->pci->vendor)
#define PCI_PID(pcr)			((pcr)->pci->device)
#define is_version(pcr, pid, ver)				\
	(CHK_PCI_PID(pcr, pid) && (pcr)->ic_version == (ver))
#define is_version_higher_than(pcr, pid, ver)			\
	(CHK_PCI_PID(pcr, pid) && (pcr)->ic_version > (ver))
#define pcr_dbg(pcr, fmt, arg...)				\
	dev_dbg(&(pcr)->pci->dev, fmt, ##arg)

#define SDR104_PHASE(val)		((val) & 0xFF)
#define SDR50_PHASE(val)		(((val) >> 8) & 0xFF)
#define DDR50_PHASE(val)		(((val) >> 16) & 0xFF)
#define SDR104_TX_PHASE(pcr)		SDR104_PHASE((pcr)->tx_initial_phase)
#define SDR50_TX_PHASE(pcr)		SDR50_PHASE((pcr)->tx_initial_phase)
#define DDR50_TX_PHASE(pcr)		DDR50_PHASE((pcr)->tx_initial_phase)
#define SDR104_RX_PHASE(pcr)		SDR104_PHASE((pcr)->rx_initial_phase)
#define SDR50_RX_PHASE(pcr)		SDR50_PHASE((pcr)->rx_initial_phase)
#define DDR50_RX_PHASE(pcr)		DDR50_PHASE((pcr)->rx_initial_phase)
#define SET_CLOCK_PHASE(sdr104, sdr50, ddr50)	\
				(((ddr50) << 16) | ((sdr50) << 8) | (sdr104))

void rtsx_pci_start_run(struct rtsx_pcr *pcr);
int rtsx_pci_write_register(struct rtsx_pcr *pcr, u16 addr, u8 mask, u8 data);
int rtsx_pci_read_register(struct rtsx_pcr *pcr, u16 addr, u8 *data);
int rtsx_pci_write_phy_register(struct rtsx_pcr *pcr, u8 addr, u16 val);
int rtsx_pci_read_phy_register(struct rtsx_pcr *pcr, u8 addr, u16 *val);
void rtsx_pci_stop_cmd(struct rtsx_pcr *pcr);
void rtsx_pci_add_cmd(struct rtsx_pcr *pcr,
		u8 cmd_type, u16 reg_addr, u8 mask, u8 data);
void rtsx_pci_send_cmd_no_wait(struct rtsx_pcr *pcr);
int rtsx_pci_send_cmd(struct rtsx_pcr *pcr, int timeout);
int rtsx_pci_dma_map_sg(struct rtsx_pcr *pcr, struct scatterlist *sglist,
		int num_sg, bool read);
void rtsx_pci_dma_unmap_sg(struct rtsx_pcr *pcr, struct scatterlist *sglist,
		int num_sg, bool read);
int rtsx_pci_dma_transfer(struct rtsx_pcr *pcr, struct scatterlist *sglist,
		int count, bool read, int timeout);
int rtsx_pci_read_ppbuf(struct rtsx_pcr *pcr, u8 *buf, int buf_len);
int rtsx_pci_write_ppbuf(struct rtsx_pcr *pcr, u8 *buf, int buf_len);
int rtsx_pci_card_pull_ctl_enable(struct rtsx_pcr *pcr, int card);
int rtsx_pci_card_pull_ctl_disable(struct rtsx_pcr *pcr, int card);
int rtsx_pci_switch_clock(struct rtsx_pcr *pcr, unsigned int card_clock,
		u8 ssc_depth, bool initial_mode, bool double_clk, bool vpclk);
int rtsx_pci_card_power_on(struct rtsx_pcr *pcr, int card);
int rtsx_pci_card_power_off(struct rtsx_pcr *pcr, int card);
int rtsx_pci_card_exclusive_check(struct rtsx_pcr *pcr, int card);
int rtsx_pci_switch_output_voltage(struct rtsx_pcr *pcr, u8 voltage);
unsigned int rtsx_pci_card_exist(struct rtsx_pcr *pcr);
void rtsx_pci_complete_unfinished_transfer(struct rtsx_pcr *pcr);

static inline u8 *rtsx_pci_get_cmd_data(struct rtsx_pcr *pcr)
{
	return (u8 *)(pcr->host_cmds_ptr);
}

static inline void rtsx_pci_write_be32(struct rtsx_pcr *pcr, u16 reg, u32 val)
{
	rtsx_pci_add_cmd(pcr, WRITE_REG_CMD, reg,     0xFF, val >> 24);
	rtsx_pci_add_cmd(pcr, WRITE_REG_CMD, reg + 1, 0xFF, val >> 16);
	rtsx_pci_add_cmd(pcr, WRITE_REG_CMD, reg + 2, 0xFF, val >> 8);
	rtsx_pci_add_cmd(pcr, WRITE_REG_CMD, reg + 3, 0xFF, val);
}

static inline int rtsx_pci_update_phy(struct rtsx_pcr *pcr, u8 addr,
	u16 mask, u16 append)
{
	int err;
	u16 val;

	err = rtsx_pci_read_phy_register(pcr, addr, &val);
	if (err < 0)
		return err;

	return rtsx_pci_write_phy_register(pcr, addr, (val & mask) | append);
}

// #endif

#[repr(C)]
pub struct rtsx_pcr;
#[repr(C)] pub struct pcr_handle { pub pcr: *mut rtsx_pcr }
#[repr(C)] pub struct pcr_ops {
    pub write_phy: Option<unsafe extern "C" fn(*mut rtsx_pcr, u8, u16) -> i32>,
    pub read_phy: Option<unsafe extern "C" fn(*mut rtsx_pcr, u8, *mut u16) -> i32>,
    pub extra_init_hw: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub optimize_phy: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub turn_on_led: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub turn_off_led: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub enable_auto_blink: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub disable_auto_blink: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> i32>,
    pub card_power_on: Option<unsafe extern "C" fn(*mut rtsx_pcr, i32) -> i32>,
    pub card_power_off: Option<unsafe extern "C" fn(*mut rtsx_pcr, i32) -> i32>,
    pub switch_output_voltage: Option<unsafe extern "C" fn(*mut rtsx_pcr, u8) -> i32>,
    pub cd_deglitch: Option<unsafe extern "C" fn(*mut rtsx_pcr) -> u32>,
    pub conv_clk_and_div_n: Option<unsafe extern "C" fn(i32, i32) -> i32>,
    pub fetch_vendor_settings: Option<unsafe extern "C" fn(*mut rtsx_pcr)>,
    pub force_power_down: Option<unsafe extern "C" fn(*mut rtsx_pcr, u8, bool)>,
    pub stop_cmd: Option<unsafe extern "C" fn(*mut rtsx_pcr)>,
}
#[repr(C)] pub struct rtsx_cr_option { pub dev_flags:u32, pub force_clkreq_0:bool, pub ltr_en:bool, pub ltr_enabled:bool, pub ltr_active:bool, pub ltr_active_latency:u32, pub ltr_idle_latency:u32, pub ltr_l1off_latency:u32, pub l1_snooze_delay:u32, pub ltr_l1off_sspwrgate:u8, pub ltr_l1off_snooze_sspwrgate:u8, pub ocp_en:bool, pub sd_400mA_ocp_thd:u8, pub sd_800mA_ocp_thd:u8, pub sd_cd_reverse_en:u8, pub sd_wp_reverse_en:u8 }
#[repr(C)] pub struct rtsx_hw_param { pub interrupt_en:u32, pub ocp_glitch:u8 }
extern "C" {
    pub fn rtsx_pci_start_run(pcr:*mut rtsx_pcr); pub fn rtsx_pci_write_register(pcr:*mut rtsx_pcr, addr:u16, mask:u8, data:u8)->i32; pub fn rtsx_pci_read_register(pcr:*mut rtsx_pcr, addr:u16, data:*mut u8)->i32;
    pub fn rtsx_pci_write_phy_register(pcr:*mut rtsx_pcr, addr:u8, val:u16)->i32; pub fn rtsx_pci_read_phy_register(pcr:*mut rtsx_pcr, addr:u8, val:*mut u16)->i32; pub fn rtsx_pci_stop_cmd(pcr:*mut rtsx_pcr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
