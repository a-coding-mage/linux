/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Chip register definitions for PCILynx chipset. Based on pcilynx.h
 * from the Linux 1394 drivers, but modified a bit so the names here
 * match the specification exactly (even though they have weird names,
 * like xxx_OVER_FLOW, or arbitrary abbreviations like SNTRJ for "sent
 * reject" etc.)
 */

pub const PCILYNX_MAX_REGISTER: u32 = 0xfff;
pub const PCILYNX_MAX_MEMORY: u32 = 0xffff;

pub const PCI_LATENCY_CACHELINE: u32 = 0x0c;

pub const MISC_CONTROL: u32 = 0x40;
pub const MISC_CONTROL_SWRESET: u32 = 1 << 0;

pub const SERIAL_EEPROM_CONTROL: u32 = 0x44;

pub const PCI_INT_STATUS: u32 = 0x48;
pub const PCI_INT_ENABLE: u32 = 0x4c;
/* status and enable have identical bit numbers */
pub const PCI_INT_INT_PEND: u32 = 1 << 31;
pub const PCI_INT_FRC_INT: u32 = 1 << 30;
pub const PCI_INT_SLV_ADR_PERR: u32 = 1 << 28;
pub const PCI_INT_SLV_DAT_PERR: u32 = 1 << 27;
pub const PCI_INT_MST_DAT_PERR: u32 = 1 << 26;
pub const PCI_INT_MST_DEV_TO: u32 = 1 << 25;
pub const PCI_INT_INT_SLV_TO: u32 = 1 << 23;
pub const PCI_INT_AUX_TO: u32 = 1 << 18;
pub const PCI_INT_AUX_INT: u32 = 1 << 17;
pub const PCI_INT_P1394_INT: u32 = 1 << 16;
pub const PCI_INT_DMA4_PCL: u32 = 1 << 9;
pub const PCI_INT_DMA4_HLT: u32 = 1 << 8;
pub const PCI_INT_DMA3_PCL: u32 = 1 << 7;
pub const PCI_INT_DMA3_HLT: u32 = 1 << 6;
pub const PCI_INT_DMA2_PCL: u32 = 1 << 5;
pub const PCI_INT_DMA2_HLT: u32 = 1 << 4;
pub const PCI_INT_DMA1_PCL: u32 = 1 << 3;
pub const PCI_INT_DMA1_HLT: u32 = 1 << 2;
pub const PCI_INT_DMA0_PCL: u32 = 1 << 1;
pub const PCI_INT_DMA0_HLT: u32 = 1 << 0;
/* all DMA interrupts combined: */
pub const PCI_INT_DMA_ALL: u32 = 0x3ff;

pub const fn PCI_INT_DMA_HLT(chan: u32) -> u32 { 1 << (chan * 2) }
pub const fn PCI_INT_DMA_PCL(chan: u32) -> u32 { 1 << (chan * 2 + 1) }

pub const LBUS_ADDR: u32 = 0xb4;
pub const LBUS_ADDR_SEL_RAM: u32 = 0x0 << 16;
pub const LBUS_ADDR_SEL_ROM: u32 = 0x1 << 16;
pub const LBUS_ADDR_SEL_AUX: u32 = 0x2 << 16;
pub const LBUS_ADDR_SEL_ZV: u32 = 0x3 << 16;

pub const GPIO_CTRL_A: u32 = 0xb8;
pub const GPIO_CTRL_B: u32 = 0xbc;
pub const GPIO_DATA_BASE: u32 = 0xc0;

pub const fn DMA_BREG(base: u32, chan: u32) -> u32 { base + chan * 0x20 }
pub const fn DMA_SREG(base: u32, chan: u32) -> u32 { base + chan * 0x10 }

pub const PCL_NEXT_INVALID: u32 = 1 << 0;

/* transfer commands */
pub const PCL_CMD_RCV: u32 = 0x1 << 24;
pub const PCL_CMD_RCV_AND_UPDATE: u32 = 0xa << 24;
pub const PCL_CMD_XMT: u32 = 0x2 << 24;
pub const PCL_CMD_UNFXMT: u32 = 0xc << 24;
pub const PCL_CMD_PCI_TO_LBUS: u32 = 0x8 << 24;
pub const PCL_CMD_LBUS_TO_PCI: u32 = 0x9 << 24;

/* aux commands */
pub const PCL_CMD_NOP: u32 = 0x0 << 24;
pub const PCL_CMD_LOAD: u32 = 0x3 << 24;
pub const PCL_CMD_STOREQ: u32 = 0x4 << 24;
pub const PCL_CMD_STORED: u32 = 0xb << 24;
pub const PCL_CMD_STORE0: u32 = 0x5 << 24;
pub const PCL_CMD_STORE1: u32 = 0x6 << 24;
pub const PCL_CMD_COMPARE: u32 = 0xe << 24;
pub const PCL_CMD_SWAP_COMPARE: u32 = 0xf << 24;
pub const PCL_CMD_ADD: u32 = 0xd << 24;
pub const PCL_CMD_BRANCH: u32 = 0x7 << 24;

/* BRANCH condition codes */
pub const PCL_COND_DMARDY_SET: u32 = 0x1 << 20;
pub const PCL_COND_DMARDY_CLEAR: u32 = 0x2 << 20;

pub const PCL_GEN_INTR: u32 = 1 << 19;
pub const PCL_LAST_BUFF: u32 = 1 << 18;
pub const PCL_LAST_CMD: u32 = PCL_LAST_BUFF;
pub const PCL_WAITSTAT: u32 = 1 << 17;
pub const PCL_BIGENDIAN: u32 = 1 << 16;
pub const PCL_ISOMODE: u32 = 1 << 12;

pub const DMA0_PREV_PCL: u32 = 0x100;
pub const DMA1_PREV_PCL: u32 = 0x120;
pub const DMA2_PREV_PCL: u32 = 0x140;
pub const DMA3_PREV_PCL: u32 = 0x160;
pub const DMA4_PREV_PCL: u32 = 0x180;
pub const fn DMA_PREV_PCL(chan: u32) -> u32 { DMA_BREG(DMA0_PREV_PCL, chan) }

pub const DMA0_CURRENT_PCL: u32 = 0x104;
pub const DMA1_CURRENT_PCL: u32 = 0x124;
pub const DMA2_CURRENT_PCL: u32 = 0x144;
pub const DMA3_CURRENT_PCL: u32 = 0x164;
pub const DMA4_CURRENT_PCL: u32 = 0x184;
pub const fn DMA_CURRENT_PCL(chan: u32) -> u32 { DMA_BREG(DMA0_CURRENT_PCL, chan) }

pub const DMA0_CHAN_STAT: u32 = 0x10c;
pub const DMA1_CHAN_STAT: u32 = 0x12c;
pub const DMA2_CHAN_STAT: u32 = 0x14c;
pub const DMA3_CHAN_STAT: u32 = 0x16c;
pub const DMA4_CHAN_STAT: u32 = 0x18c;
pub const fn DMA_CHAN_STAT(chan: u32) -> u32 { DMA_BREG(DMA0_CHAN_STAT, chan) }
/* CHAN_STATUS registers share bits */
pub const DMA_CHAN_STAT_SELFID: u32 = 1 << 31;
pub const DMA_CHAN_STAT_ISOPKT: u32 = 1 << 30;
pub const DMA_CHAN_STAT_PCIERR: u32 = 1 << 29;
pub const DMA_CHAN_STAT_PKTERR: u32 = 1 << 28;
pub const DMA_CHAN_STAT_PKTCMPL: u32 = 1 << 27;
pub const DMA_CHAN_STAT_SPECIALACK: u32 = 1 << 14;

pub const DMA0_CHAN_CTRL: u32 = 0x110;
pub const DMA1_CHAN_CTRL: u32 = 0x130;
pub const DMA2_CHAN_CTRL: u32 = 0x150;
pub const DMA3_CHAN_CTRL: u32 = 0x170;
pub const DMA4_CHAN_CTRL: u32 = 0x190;
pub const fn DMA_CHAN_CTRL(chan: u32) -> u32 { DMA_BREG(DMA0_CHAN_CTRL, chan) }
/* CHAN_CTRL registers share bits */
pub const DMA_CHAN_CTRL_ENABLE: u32 = 1 << 31;
pub const DMA_CHAN_CTRL_BUSY: u32 = 1 << 30;
pub const DMA_CHAN_CTRL_LINK: u32 = 1 << 29;

pub const DMA0_READY: u32 = 0x114;
pub const DMA1_READY: u32 = 0x134;
pub const DMA2_READY: u32 = 0x154;
pub const DMA3_READY: u32 = 0x174;
pub const DMA4_READY: u32 = 0x194;
pub const fn DMA_READY(chan: u32) -> u32 { DMA_BREG(DMA0_READY, chan) }

pub const DMA_GLOBAL_REGISTER: u32 = 0x908;
pub const FIFO_SIZES: u32 = 0xa00;
pub const FIFO_CONTROL: u32 = 0xa10;
pub const FIFO_CONTROL_GRF_FLUSH: u32 = 1 << 4;
pub const FIFO_CONTROL_ITF_FLUSH: u32 = 1 << 3;
pub const FIFO_CONTROL_ATF_FLUSH: u32 = 1 << 2;
pub const FIFO_XMIT_THRESHOLD: u32 = 0xa14;

pub const DMA0_WORD0_CMP_VALUE: u32 = 0xb00;
pub const DMA1_WORD0_CMP_VALUE: u32 = 0xb10;
pub const DMA2_WORD0_CMP_VALUE: u32 = 0xb20;
pub const DMA3_WORD0_CMP_VALUE: u32 = 0xb30;
pub const DMA4_WORD0_CMP_VALUE: u32 = 0xb40;
pub const fn DMA_WORD0_CMP_VALUE(chan: u32) -> u32 { DMA_SREG(DMA0_WORD0_CMP_VALUE, chan) }
pub const DMA0_WORD0_CMP_ENABLE: u32 = 0xb04;
pub const DMA1_WORD0_CMP_ENABLE: u32 = 0xb14;
pub const DMA2_WORD0_CMP_ENABLE: u32 = 0xb24;
pub const DMA3_WORD0_CMP_ENABLE: u32 = 0xb34;
pub const DMA4_WORD0_CMP_ENABLE: u32 = 0xb44;
pub const fn DMA_WORD0_CMP_ENABLE(chan: u32) -> u32 { DMA_SREG(DMA0_WORD0_CMP_ENABLE, chan) }
pub const DMA0_WORD1_CMP_VALUE: u32 = 0xb08;
pub const DMA1_WORD1_CMP_VALUE: u32 = 0xb18;
pub const DMA2_WORD1_CMP_VALUE: u32 = 0xb28;
pub const DMA3_WORD1_CMP_VALUE: u32 = 0xb38;
pub const DMA4_WORD1_CMP_VALUE: u32 = 0xb48;
pub const fn DMA_WORD1_CMP_VALUE(chan: u32) -> u32 { DMA_SREG(DMA0_WORD1_CMP_VALUE, chan) }
pub const DMA0_WORD1_CMP_ENABLE: u32 = 0xb0c;
pub const DMA1_WORD1_CMP_ENABLE: u32 = 0xb1c;
pub const DMA2_WORD1_CMP_ENABLE: u32 = 0xb2c;
pub const DMA3_WORD1_CMP_ENABLE: u32 = 0xb3c;
pub const DMA4_WORD1_CMP_ENABLE: u32 = 0xb4c;
pub const fn DMA_WORD1_CMP_ENABLE(chan: u32) -> u32 { DMA_SREG(DMA0_WORD1_CMP_ENABLE, chan) }
/* word 1 compare enable flags */
pub const DMA_WORD1_CMP_MATCH_OTHERBUS: u32 = 1 << 15;
pub const DMA_WORD1_CMP_MATCH_BROADCAST: u32 = 1 << 14;
pub const DMA_WORD1_CMP_MATCH_BUS_BCAST: u32 = 1 << 13;
pub const DMA_WORD1_CMP_MATCH_LOCAL_NODE: u32 = 1 << 12;
pub const DMA_WORD1_CMP_MATCH_EXACT: u32 = 1 << 11;
pub const DMA_WORD1_CMP_ENABLE_SELF_ID: u32 = 1 << 10;
pub const DMA_WORD1_CMP_ENABLE_MASTER: u32 = 1 << 8;

pub const LINK_ID: u32 = 0xf00;
pub const fn LINK_ID_BUS(id: u32) -> u32 { id << 22 }
pub const fn LINK_ID_NODE(id: u32) -> u32 { id << 16 }
pub const LINK_CONTROL: u32 = 0xf04;
pub const LINK_CONTROL_BUSY: u32 = 1 << 29;
pub const LINK_CONTROL_TX_ISO_EN: u32 = 1 << 26;
pub const LINK_CONTROL_RX_ISO_EN: u32 = 1 << 25;
pub const LINK_CONTROL_TX_ASYNC_EN: u32 = 1 << 24;
pub const LINK_CONTROL_RX_ASYNC_EN: u32 = 1 << 23;
pub const LINK_CONTROL_RESET_TX: u32 = 1 << 21;
pub const LINK_CONTROL_RESET_RX: u32 = 1 << 20;
pub const LINK_CONTROL_CYCMASTER: u32 = 1 << 11;
pub const LINK_CONTROL_CYCSOURCE: u32 = 1 << 10;
pub const LINK_CONTROL_CYCTIMEREN: u32 = 1 << 9;
pub const LINK_CONTROL_RCV_CMP_VALID: u32 = 1 << 7;
pub const LINK_CONTROL_SNOOP_ENABLE: u32 = 1 << 6;
pub const CYCLE_TIMER: u32 = 0xf08;
pub const LINK_PHY: u32 = 0xf0c;
pub const LINK_PHY_READ: u32 = 1 << 31;
pub const LINK_PHY_WRITE: u32 = 1 << 30;
pub const fn LINK_PHY_ADDR(addr: u32) -> u32 { addr << 24 }
pub const fn LINK_PHY_WDATA(data: u32) -> u32 { data << 16 }
pub const fn LINK_PHY_RADDR(addr: u32) -> u32 { addr << 8 }
pub const LINK_INT_STATUS: u32 = 0xf14;
pub const LINK_INT_ENABLE: u32 = 0xf18;
/* status and enable have identical bit numbers */
pub const LINK_INT_LINK_INT: u32 = 1 << 31;
pub const LINK_INT_PHY_TIME_OUT: u32 = 1 << 30;
pub const LINK_INT_PHY_REG_RCVD: u32 = 1 << 29;
pub const LINK_INT_PHY_BUSRESET: u32 = 1 << 28;
pub const LINK_INT_TX_RDY: u32 = 1 << 26;
pub const LINK_INT_RX_DATA_RDY: u32 = 1 << 25;
pub const LINK_INT_IT_STUCK: u32 = 1 << 20;
pub const LINK_INT_AT_STUCK: u32 = 1 << 19;
pub const LINK_INT_SNTRJ: u32 = 1 << 17;
pub const LINK_INT_HDR_ERR: u32 = 1 << 16;
pub const LINK_INT_TC_ERR: u32 = 1 << 15;
pub const LINK_INT_CYC_SEC: u32 = 1 << 11;
pub const LINK_INT_CYC_STRT: u32 = 1 << 10;
pub const LINK_INT_CYC_DONE: u32 = 1 << 9;
pub const LINK_INT_CYC_PEND: u32 = 1 << 8;
pub const LINK_INT_CYC_LOST: u32 = 1 << 7;
pub const LINK_INT_CYC_ARB_FAILED: u32 = 1 << 6;
pub const LINK_INT_GRF_OVER_FLOW: u32 = 1 << 5;
pub const LINK_INT_ITF_UNDER_FLOW: u32 = 1 << 4;
pub const LINK_INT_ATF_UNDER_FLOW: u32 = 1 << 3;
pub const LINK_INT_IARB_FAILED: u32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
