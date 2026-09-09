/* SPDX-License-Identifier: GPL-2.0 */

// C forward declarations.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mx25_tsadc {
    pub regs: *mut regmap,
    pub domain: *mut irq_domain,
    pub clk: *mut clk,
}

pub const MX25_TSC_TGCR: u32 = 0x00;
pub const MX25_TSC_TGSR: u32 = 0x04;
pub const MX25_TSC_TICR: u32 = 0x08;

/* The same register layout for TC and GC queue */
pub const MX25_ADCQ_FIFO: u32 = 0x00;
pub const MX25_ADCQ_CR: u32 = 0x04;
pub const MX25_ADCQ_SR: u32 = 0x08;
pub const MX25_ADCQ_MR: u32 = 0x0c;
pub const MX25_ADCQ_ITEM_7_0: u32 = 0x20;
pub const MX25_ADCQ_ITEM_15_8: u32 = 0x24;
pub const fn MX25_ADCQ_CFG(n: u32) -> u32 { 0x40 + n * 0x4 }

pub const MX25_ADCQ_MR_MASK: u32 = 0xffffffff;

/* TGCR */
pub const fn MX25_TGCR_PDBTIME(x: u32) -> u32 { x << 25 }
pub const MX25_TGCR_PDBTIME_MASK: u32 = 0xfe000000;
pub const MX25_TGCR_PDBEN: u32 = 1 << 24;
pub const MX25_TGCR_PDEN: u32 = 1 << 23;
pub const fn MX25_TGCR_ADCCLKCFG(x: u32) -> u32 { x << 16 }
pub const fn MX25_TGCR_GET_ADCCLK(x: u32) -> u32 { (x >> 16) & 0x1f }
pub const MX25_TGCR_INTREFEN: u32 = 1 << 10;
pub const MX25_TGCR_POWERMODE_MASK: u32 = 0x00000300;
pub const MX25_TGCR_POWERMODE_SAVE: u32 = 1 << 8;
pub const MX25_TGCR_POWERMODE_ON: u32 = 2 << 8;
pub const MX25_TGCR_STLC: u32 = 1 << 5;
pub const MX25_TGCR_SLPC: u32 = 1 << 4;
pub const MX25_TGCR_FUNC_RST: u32 = 1 << 2;
pub const MX25_TGCR_TSC_RST: u32 = 1 << 1;
pub const MX25_TGCR_CLK_EN: u32 = 1;

/* TGSR */
pub const MX25_TGSR_SLP_INT: u32 = 1 << 2;
pub const MX25_TGSR_GCQ_INT: u32 = 1 << 1;
pub const MX25_TGSR_TCQ_INT: u32 = 1;

/* ADCQ_ITEM_* */
pub const fn _MX25_ADCQ_ITEM(item: u32, x: u32) -> u32 { x << (item * 4) }
pub const fn MX25_ADCQ_ITEM(item: u32, x: u32) -> u32 {
    if item >= 8 { _MX25_ADCQ_ITEM(item - 8, x) } else { _MX25_ADCQ_ITEM(item, x) }
}

/* ADCQ_FIFO (TCQFIFO and GCQFIFO) */
pub const fn MX25_ADCQ_FIFO_DATA(x: u32) -> u32 { (x >> 4) & 0xfff }
pub const fn MX25_ADCQ_FIFO_ID(x: u32) -> u32 { x & 0xf }

/* ADCQ_CR (TCQR and GCQR) */
pub const MX25_ADCQ_CR_PDCFG_LEVEL: u32 = 1 << 19;
pub const MX25_ADCQ_CR_PDMSK: u32 = 1 << 18;
pub const MX25_ADCQ_CR_FRST: u32 = 1 << 17;
pub const MX25_ADCQ_CR_QRST: u32 = 1 << 16;
pub const MX25_ADCQ_CR_RWAIT_MASK: u32 = 0x0000f000;
pub const fn MX25_ADCQ_CR_RWAIT(x: u32) -> u32 { x << 12 }
pub const MX25_ADCQ_CR_WMRK_MASK: u32 = 0x00000f00;
pub const fn MX25_ADCQ_CR_WMRK(x: u32) -> u32 { x << 8 }
pub const MX25_ADCQ_CR_LITEMID_MASK: u32 = 0xf << 4;
pub const fn MX25_ADCQ_CR_LITEMID(x: u32) -> u32 { x << 4 }
pub const MX25_ADCQ_CR_RPT: u32 = 1 << 3;
pub const MX25_ADCQ_CR_FQS: u32 = 1 << 2;
pub const MX25_ADCQ_CR_QSM_MASK: u32 = 0x3;
pub const MX25_ADCQ_CR_QSM_PD: u32 = 0x1;
pub const MX25_ADCQ_CR_QSM_FQS: u32 = 0x2;
pub const MX25_ADCQ_CR_QSM_FQS_PD: u32 = 0x3;

/* ADCQ_SR (TCQSR and GCQSR) */
pub const MX25_ADCQ_SR_FDRY: u32 = 1 << 15;
pub const MX25_ADCQ_SR_FULL: u32 = 1 << 14;
pub const MX25_ADCQ_SR_EMPT: u32 = 1 << 13;
pub const fn MX25_ADCQ_SR_FDN(x: u32) -> u32 { (x >> 8) & 0x1f }
pub const MX25_ADCQ_SR_FRR: u32 = 1 << 6;
pub const MX25_ADCQ_SR_FUR: u32 = 1 << 5;
pub const MX25_ADCQ_SR_FOR: u32 = 1 << 4;
pub const MX25_ADCQ_SR_EOQ: u32 = 1 << 1;
pub const MX25_ADCQ_SR_PD: u32 = 1;

/* ADCQ_MR (TCQMR and GCQMR) */
pub const MX25_ADCQ_MR_FDRY_DMA: u32 = 1 << 31;
pub const MX25_ADCQ_MR_FER_DMA: u32 = 1 << 22;
pub const MX25_ADCQ_MR_FUR_DMA: u32 = 1 << 21;
pub const MX25_ADCQ_MR_FOR_DMA: u32 = 1 << 20;
pub const MX25_ADCQ_MR_EOQ_DMA: u32 = 1 << 17;
pub const MX25_ADCQ_MR_PD_DMA: u32 = 1 << 16;
pub const MX25_ADCQ_MR_FDRY_IRQ: u32 = 1 << 15;
pub const MX25_ADCQ_MR_FER_IRQ: u32 = 1 << 6;
pub const MX25_ADCQ_MR_FUR_IRQ: u32 = 1 << 5;
pub const MX25_ADCQ_MR_FOR_IRQ: u32 = 1 << 4;
pub const MX25_ADCQ_MR_EOQ_IRQ: u32 = 1 << 1;
pub const MX25_ADCQ_MR_PD_IRQ: u32 = 1;

/* ADCQ_CFG (TICR, TCC0-7,GCC0-7) */
pub const fn MX25_ADCQ_CFG_SETTLING_TIME(x: u32) -> u32 { x << 24 }
pub const MX25_ADCQ_CFG_IGS: u32 = 1 << 20;
pub const MX25_ADCQ_CFG_NOS_MASK: u32 = 0x000f0000;
pub const fn MX25_ADCQ_CFG_NOS(x: u32) -> u32 { (x - 1) << 16 }
pub const MX25_ADCQ_CFG_WIPER: u32 = 1 << 15;
pub const MX25_ADCQ_CFG_YNLR: u32 = 1 << 14;
pub const MX25_ADCQ_CFG_YPLL_HIGH: u32 = 0 << 12;
pub const MX25_ADCQ_CFG_YPLL_OFF: u32 = 1 << 12;
pub const MX25_ADCQ_CFG_YPLL_LOW: u32 = 3 << 12;
pub const MX25_ADCQ_CFG_XNUR_HIGH: u32 = 0 << 10;
pub const MX25_ADCQ_CFG_XNUR_OFF: u32 = 1 << 10;
pub const MX25_ADCQ_CFG_XNUR_LOW: u32 = 3 << 10;
pub const MX25_ADCQ_CFG_XPUL_HIGH: u32 = 0 << 9;
pub const MX25_ADCQ_CFG_XPUL_OFF: u32 = 1 << 9;
pub const fn MX25_ADCQ_CFG_REFP(sel: u32) -> u32 { sel << 7 }
pub const MX25_ADCQ_CFG_REFP_YP: u32 = MX25_ADCQ_CFG_REFP(0);
pub const MX25_ADCQ_CFG_REFP_XP: u32 = MX25_ADCQ_CFG_REFP(1);
pub const MX25_ADCQ_CFG_REFP_EXT: u32 = MX25_ADCQ_CFG_REFP(2);
pub const MX25_ADCQ_CFG_REFP_INT: u32 = MX25_ADCQ_CFG_REFP(3);
pub const MX25_ADCQ_CFG_REFP_MASK: u32 = 0x00000180;
pub const fn MX25_ADCQ_CFG_IN(sel: u32) -> u32 { sel << 4 }
pub const MX25_ADCQ_CFG_IN_XP: u32 = MX25_ADCQ_CFG_IN(0);
pub const MX25_ADCQ_CFG_IN_YP: u32 = MX25_ADCQ_CFG_IN(1);
pub const MX25_ADCQ_CFG_IN_XN: u32 = MX25_ADCQ_CFG_IN(2);
pub const MX25_ADCQ_CFG_IN_YN: u32 = MX25_ADCQ_CFG_IN(3);
pub const MX25_ADCQ_CFG_IN_WIPER: u32 = MX25_ADCQ_CFG_IN(4);
pub const MX25_ADCQ_CFG_IN_AUX0: u32 = MX25_ADCQ_CFG_IN(5);
pub const MX25_ADCQ_CFG_IN_AUX1: u32 = MX25_ADCQ_CFG_IN(6);
pub const MX25_ADCQ_CFG_IN_AUX2: u32 = MX25_ADCQ_CFG_IN(7);
pub const fn MX25_ADCQ_CFG_REFN(sel: u32) -> u32 { sel << 2 }
pub const MX25_ADCQ_CFG_REFN_XN: u32 = MX25_ADCQ_CFG_REFN(0);
pub const MX25_ADCQ_CFG_REFN_YN: u32 = MX25_ADCQ_CFG_REFN(1);
pub const MX25_ADCQ_CFG_REFN_NGND: u32 = MX25_ADCQ_CFG_REFN(2);
pub const MX25_ADCQ_CFG_REFN_NGND2: u32 = MX25_ADCQ_CFG_REFN(3);
pub const MX25_ADCQ_CFG_REFN_MASK: u32 = 0x0000000c;
pub const MX25_ADCQ_CFG_PENIACK: u32 = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
