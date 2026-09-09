/* SPDX-License-Identifier: GPL-2.0-only */
/* include/linux/amba/pl080.h
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      http://armlinux.simtec.co.uk/
 *      Ben Dooks <ben@simtec.co.uk>
 *
 * ARM PrimeCell PL080 DMA controller
 */

/* Note, there are some Samsung updates to this controller block which
 * make it not entierly compatible with the PL080 specification from ARM. When in doubt, check the Samsung documentation first.
 *
 * The Samsung defines are PL080S, and add an extra control register,
 * the ability to move more than 2^11 counts of data and some extra
 * OneNAND features.
 */

pub const PL080_INT_STATUS: u32 = 0x00;
pub const PL080_TC_STATUS: u32 = 0x04;
pub const PL080_TC_CLEAR: u32 = 0x08;
pub const PL080_ERR_STATUS: u32 = 0x0C;
pub const PL080_ERR_CLEAR: u32 = 0x10;
pub const PL080_RAW_TC_STATUS: u32 = 0x14;
pub const PL080_RAW_ERR_STATUS: u32 = 0x18;
pub const PL080_EN_CHAN: u32 = 0x1c;
pub const PL080_SOFT_BREQ: u32 = 0x20;
pub const PL080_SOFT_SREQ: u32 = 0x24;
pub const PL080_SOFT_LBREQ: u32 = 0x28;
pub const PL080_SOFT_LSREQ: u32 = 0x2C;
pub const PL080_CONFIG: u32 = 0x30;
pub const PL080_CONFIG_M2_BE: u32 = BIT(2);
pub const PL080_CONFIG_M1_BE: u32 = BIT(1);
pub const PL080_CONFIG_ENABLE: u32 = BIT(0);
pub const PL080_SYNC: u32 = 0x34;

pub const FTDMAC020_CH_BUSY: u32 = 0x20;
pub const FTDMAC020_CSR: u32 = 0x24;
pub const FTDMAC020_SYNC: u32 = 0x2C;
pub const FTDMAC020_REVISION: u32 = 0x30;
pub const FTDMAC020_FEATURE: u32 = 0x34;

#[inline]
pub const fn PL080_Cx_BASE(x: u32) -> u32 { 0x100 + (x * 0x20) }
pub const PL080_CH_SRC_ADDR: u32 = 0x00;
pub const PL080_CH_DST_ADDR: u32 = 0x04;
pub const PL080_CH_LLI: u32 = 0x08;
pub const PL080_CH_CONTROL: u32 = 0x0C;
pub const PL080_CH_CONFIG: u32 = 0x10;
pub const PL080S_CH_CONTROL2: u32 = 0x10;
pub const PL080S_CH_CONFIG: u32 = 0x14;
pub const FTDMAC020_CH_CSR: u32 = 0x00;
pub const FTDMAC020_CH_CFG: u32 = 0x04;
pub const FTDMAC020_CH_SRC_ADDR: u32 = 0x08;
pub const FTDMAC020_CH_DST_ADDR: u32 = 0x0C;
pub const FTDMAC020_CH_LLP: u32 = 0x10;
pub const FTDMAC020_CH_SIZE: u32 = 0x14;

pub const PL080_LLI_ADDR_MASK: u32 = GENMASK(31, 2);
pub const PL080_LLI_ADDR_SHIFT: u32 = 2;
pub const PL080_LLI_LM_AHB2: u32 = BIT(0);
pub const PL080_CONTROL_TC_IRQ_EN: u32 = BIT(31);
pub const PL080_CONTROL_PROT_MASK: u32 = GENMASK(30, 28);
pub const PL080_CONTROL_PROT_SHIFT: u32 = 28;
pub const PL080_CONTROL_PROT_CACHE: u32 = BIT(30);
pub const PL080_CONTROL_PROT_BUFF: u32 = BIT(29);
pub const PL080_CONTROL_PROT_SYS: u32 = BIT(28);
pub const PL080_CONTROL_DST_INCR: u32 = BIT(27);
pub const PL080_CONTROL_SRC_INCR: u32 = BIT(26);
pub const PL080_CONTROL_DST_AHB2: u32 = BIT(25);
pub const PL080_CONTROL_SRC_AHB2: u32 = BIT(24);
pub const PL080_CONTROL_DWIDTH_MASK: u32 = GENMASK(23, 21);
pub const PL080_CONTROL_DWIDTH_SHIFT: u32 = 21;
pub const PL080_CONTROL_SWIDTH_MASK: u32 = GENMASK(20, 18);
pub const PL080_CONTROL_SWIDTH_SHIFT: u32 = 18;
pub const PL080_CONTROL_DB_SIZE_MASK: u32 = GENMASK(17, 15);
pub const PL080_CONTROL_DB_SIZE_SHIFT: u32 = 15;
pub const PL080_CONTROL_SB_SIZE_MASK: u32 = GENMASK(14, 12);
pub const PL080_CONTROL_SB_SIZE_SHIFT: u32 = 12;
pub const PL080_CONTROL_TRANSFER_SIZE_MASK: u32 = GENMASK(11, 0);
pub const PL080S_CONTROL_TRANSFER_SIZE_MASK: u32 = GENMASK(24, 0);
pub const PL080_CONTROL_TRANSFER_SIZE_SHIFT: u32 = 0;

pub const PL080_BSIZE_1: u32 = 0x0;
pub const PL080_BSIZE_4: u32 = 0x1;
pub const PL080_BSIZE_8: u32 = 0x2;
pub const PL080_BSIZE_16: u32 = 0x3;
pub const PL080_BSIZE_32: u32 = 0x4;
pub const PL080_BSIZE_64: u32 = 0x5;
pub const PL080_BSIZE_128: u32 = 0x6;
pub const PL080_BSIZE_256: u32 = 0x7;
pub const PL080_WIDTH_8BIT: u32 = 0x0;
pub const PL080_WIDTH_16BIT: u32 = 0x1;
pub const PL080_WIDTH_32BIT: u32 = 0x2;

pub const PL080N_CONFIG_ITPROT: u32 = BIT(20);
pub const PL080N_CONFIG_SECPROT: u32 = BIT(19);
pub const PL080_CONFIG_HALT: u32 = BIT(18);
pub const PL080_CONFIG_ACTIVE: u32 = BIT(17); /* RO */
pub const PL080_CONFIG_LOCK: u32 = BIT(16);
pub const PL080_CONFIG_TC_IRQ_MASK: u32 = BIT(15);
pub const PL080_CONFIG_ERR_IRQ_MASK: u32 = BIT(14);
pub const PL080_CONFIG_FLOW_CONTROL_MASK: u32 = GENMASK(13, 11);
pub const PL080_CONFIG_FLOW_CONTROL_SHIFT: u32 = 11;
pub const PL080_CONFIG_DST_SEL_MASK: u32 = GENMASK(9, 6);
pub const PL080_CONFIG_DST_SEL_SHIFT: u32 = 6;
pub const PL080_CONFIG_SRC_SEL_MASK: u32 = GENMASK(4, 1);
pub const PL080_CONFIG_SRC_SEL_SHIFT: u32 = 1;
/* PL080_CONFIG_ENABLE is defined identically above in the original header. */

pub const PL080_FLOW_MEM2MEM: u32 = 0x0;
pub const PL080_FLOW_MEM2PER: u32 = 0x1;
pub const PL080_FLOW_PER2MEM: u32 = 0x2;
pub const PL080_FLOW_SRC2DST: u32 = 0x3;
pub const PL080_FLOW_SRC2DST_DST: u32 = 0x4;
pub const PL080_FLOW_MEM2PER_PER: u32 = 0x5;
pub const PL080_FLOW_PER2MEM_PER: u32 = 0x6;
pub const PL080_FLOW_SRC2DST_SRC: u32 = 0x7;

pub const FTDMAC020_CH_CSR_TC_MSK: u32 = BIT(31);
pub const FTDMAC020_CH_CSR_FIFOTH_MSK: u32 = GENMASK(26, 24);
pub const FTDMAC020_CH_CSR_FIFOTH_SHIFT: u32 = 24;
pub const FTDMAC020_CH_CSR_CHPR1_MSK: u32 = GENMASK(23, 22);
pub const FTDMAC020_CH_CSR_PROT3: u32 = BIT(21);
pub const FTDMAC020_CH_CSR_PROT2: u32 = BIT(20);
pub const FTDMAC020_CH_CSR_PROT1: u32 = BIT(19);
pub const FTDMAC020_CH_CSR_SRC_SIZE_MSK: u32 = GENMASK(18, 16);
pub const FTDMAC020_CH_CSR_SRC_SIZE_SHIFT: u32 = 16;
pub const FTDMAC020_CH_CSR_ABT: u32 = BIT(15);
pub const FTDMAC020_CH_CSR_SRC_WIDTH_MSK: u32 = GENMASK(13, 11);
pub const FTDMAC020_CH_CSR_SRC_WIDTH_SHIFT: u32 = 11;
pub const FTDMAC020_CH_CSR_DST_WIDTH_MSK: u32 = GENMASK(10, 8);
pub const FTDMAC020_CH_CSR_DST_WIDTH_SHIFT: u32 = 8;
pub const FTDMAC020_CH_CSR_MODE: u32 = BIT(7);
/* 00 = increase, 01 = decrease, 10 = fix */
pub const FTDMAC020_CH_CSR_SRCAD_CTL_MSK: u32 = GENMASK(6, 5);
pub const FTDMAC020_CH_CSR_SRCAD_CTL_SHIFT: u32 = 5;
pub const FTDMAC020_CH_CSR_DSTAD_CTL_MSK: u32 = GENMASK(4, 3);
pub const FTDMAC020_CH_CSR_DSTAD_CTL_SHIFT: u32 = 3;
pub const FTDMAC020_CH_CSR_SRC_SEL: u32 = BIT(2);
pub const FTDMAC020_CH_CSR_DST_SEL: u32 = BIT(1);
pub const FTDMAC020_CH_CSR_EN: u32 = BIT(0);
pub const FTDMAC020_CH_CSR_FIFOTH_1: u32 = 0x0;
pub const FTDMAC020_CH_CSR_FIFOTH_2: u32 = 0x1;
pub const FTDMAC020_CH_CSR_FIFOTH_4: u32 = 0x2;
pub const FTDMAC020_CH_CSR_FIFOTH_8: u32 = 0x3;
pub const FTDMAC020_CH_CSR_FIFOTH_16: u32 = 0x4;
pub const FTDMAC020_WIDTH_64BIT: u32 = 0x3;
pub const FTDMAC020_CH_CSR_SRCAD_CTL_INC: u32 = 0x0;
pub const FTDMAC020_CH_CSR_SRCAD_CTL_DEC: u32 = 0x1;
pub const FTDMAC020_CH_CSR_SRCAD_CTL_FIXED: u32 = 0x2;

pub const FTDMAC020_CH_CFG_LLP_CNT_MASK: u32 = GENMASK(19, 16);
pub const FTDMAC020_CH_CFG_LLP_CNT_SHIFT: u32 = 16;
pub const FTDMAC020_CH_CFG_BUSY: u32 = BIT(8);
pub const FTDMAC020_CH_CFG_INT_ABT_MASK: u32 = BIT(2);
pub const FTDMAC020_CH_CFG_INT_ERR_MASK: u32 = BIT(1);
pub const FTDMAC020_CH_CFG_INT_TC_MASK: u32 = BIT(0);
pub const FTDMAC020_LLI_TC_MSK: u32 = BIT(28);
pub const FTDMAC020_LLI_SRC_WIDTH_MSK: u32 = GENMASK(27, 25);
pub const FTDMAC020_LLI_SRC_WIDTH_SHIFT: u32 = 25;
pub const FTDMAC020_LLI_DST_WIDTH_MSK: u32 = GENMASK(24, 22);
pub const FTDMAC020_LLI_DST_WIDTH_SHIFT: u32 = 22;
pub const FTDMAC020_LLI_SRCAD_CTL_MSK: u32 = GENMASK(21, 20);
pub const FTDMAC020_LLI_SRCAD_CTL_SHIFT: u32 = 20;
pub const FTDMAC020_LLI_DSTAD_CTL_MSK: u32 = GENMASK(19, 18);
pub const FTDMAC020_LLI_DSTAD_CTL_SHIFT: u32 = 18;
pub const FTDMAC020_LLI_SRC_SEL: u32 = BIT(17);
pub const FTDMAC020_LLI_DST_SEL: u32 = BIT(16);
pub const FTDMAC020_LLI_TRANSFER_SIZE_MASK: u32 = GENMASK(11, 0);
pub const FTDMAC020_LLI_TRANSFER_SIZE_SHIFT: u32 = 0;
pub const FTDMAC020_CFG_LLP_CNT_MASK: u32 = GENMASK(19, 16);
pub const FTDMAC020_CFG_LLP_CNT_SHIFT: u32 = 16;
pub const FTDMAC020_CFG_BUSY: u32 = BIT(8);
pub const FTDMAC020_CFG_INT_ABT_MSK: u32 = BIT(2);
pub const FTDMAC020_CFG_INT_ERR_MSK: u32 = BIT(1);
pub const FTDMAC020_CFG_INT_TC_MSK: u32 = BIT(0);

/* DMA linked list chain structure */
#[repr(C)]
pub struct pl080_lli {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub next_lli: u32,
    pub control0: u32,
}

#[repr(C)]
pub struct pl080s_lli {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub next_lli: u32,
    pub control0: u32,
    pub control1: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
