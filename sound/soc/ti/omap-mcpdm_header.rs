/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap-mcpdm.h
 *
 * Copyright (C) 2009 - 2011 Texas Instruments
 *
 * Contact: Misael Lopez Cruz <misael.lopez@ti.com>
 */

pub const MCPDM_REG_REVISION: u32 = 0x00;
pub const MCPDM_REG_SYSCONFIG: u32 = 0x10;
pub const MCPDM_REG_IRQSTATUS_RAW: u32 = 0x24;
pub const MCPDM_REG_IRQSTATUS: u32 = 0x28;
pub const MCPDM_REG_IRQENABLE_SET: u32 = 0x2C;
pub const MCPDM_REG_IRQENABLE_CLR: u32 = 0x30;
pub const MCPDM_REG_IRQWAKE_EN: u32 = 0x34;
pub const MCPDM_REG_DMAENABLE_SET: u32 = 0x38;
pub const MCPDM_REG_DMAENABLE_CLR: u32 = 0x3C;
pub const MCPDM_REG_DMAWAKEEN: u32 = 0x40;
pub const MCPDM_REG_CTRL: u32 = 0x44;
pub const MCPDM_REG_DN_DATA: u32 = 0x48;
pub const MCPDM_REG_UP_DATA: u32 = 0x4C;
pub const MCPDM_REG_FIFO_CTRL_DN: u32 = 0x50;
pub const MCPDM_REG_FIFO_CTRL_UP: u32 = 0x54;
pub const MCPDM_REG_DN_OFFSET: u32 = 0x58;

/*
 * MCPDM_IRQ bit fields
 * IRQSTATUS_RAW, IRQSTATUS, IRQENABLE_SET, IRQENABLE_CLR
 */

pub const MCPDM_DN_IRQ: u32 = 1 << 0;
pub const MCPDM_DN_IRQ_EMPTY: u32 = 1 << 1;
pub const MCPDM_DN_IRQ_ALMST_EMPTY: u32 = 1 << 2;
pub const MCPDM_DN_IRQ_FULL: u32 = 1 << 3;

pub const MCPDM_UP_IRQ: u32 = 1 << 8;
pub const MCPDM_UP_IRQ_EMPTY: u32 = 1 << 9;
pub const MCPDM_UP_IRQ_ALMST_FULL: u32 = 1 << 10;
pub const MCPDM_UP_IRQ_FULL: u32 = 1 << 11;

pub const MCPDM_DOWNLINK_IRQ_MASK: u32 = 0x00F;
pub const MCPDM_UPLINK_IRQ_MASK: u32 = 0xF00;

/*
 * MCPDM_DMAENABLE bit fields
 */

pub const MCPDM_DMA_DN_ENABLE: u32 = 1 << 0;
pub const MCPDM_DMA_UP_ENABLE: u32 = 1 << 1;

/*
 * MCPDM_CTRL bit fields
 */

pub const fn MCPDM_PDM_UPLINK_EN(x: u32) -> u32 {
    1 << (x - 1)
} /* ch1 is at bit 0 */

pub const fn MCPDM_PDM_DOWNLINK_EN(x: u32) -> u32 {
    1 << (x + 2)
} /* ch1 is at bit 3 */

pub const MCPDM_PDMOUTFORMAT: u32 = 1 << 8;
pub const MCPDM_CMD_INT: u32 = 1 << 9;
pub const MCPDM_STATUS_INT: u32 = 1 << 10;
pub const MCPDM_SW_UP_RST: u32 = 1 << 11;
pub const MCPDM_SW_DN_RST: u32 = 1 << 12;
pub const MCPDM_WD_EN: u32 = 1 << 14;
pub const MCPDM_PDM_UP_MASK: u32 = 0x7;
pub const MCPDM_PDM_DN_MASK: u32 = 0x1f << 3;

pub const MCPDM_PDMOUTFORMAT_LJUST: u32 = 0 << 8;
pub const MCPDM_PDMOUTFORMAT_RJUST: u32 = 1 << 8;

/*
 * MCPDM_FIFO_CTRL bit fields
 */

pub const MCPDM_UP_THRES_MAX: u32 = 0xF;
pub const MCPDM_DN_THRES_MAX: u32 = 0xF;

/*
 * MCPDM_DN_OFFSET bit fields
 */

pub const MCPDM_DN_OFST_RX1_EN: u32 = 1 << 0;

pub const fn MCPDM_DNOFST_RX1(x: u32) -> u32 {
    (x & 0x1f) << 1
}

pub const MCPDM_DN_OFST_RX2_EN: u32 = 1 << 8;

pub const fn MCPDM_DNOFST_RX2(x: u32) -> u32 {
    (x & 0x1f) << 9
}

unsafe extern "C" {
    pub fn omap_mcpdm_configure_dn_offsets(rtd: *mut snd_soc_pcm_runtime, rx1: u8, rx2: u8);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
