/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap-dmic.h  --  OMAP Digital Microphone Controller
 */

pub const OMAP_DMIC_REVISION_REG: u32 = 0x00;
pub const OMAP_DMIC_SYSCONFIG_REG: u32 = 0x10;
pub const OMAP_DMIC_IRQSTATUS_RAW_REG: u32 = 0x24;
pub const OMAP_DMIC_IRQSTATUS_REG: u32 = 0x28;
pub const OMAP_DMIC_IRQENABLE_SET_REG: u32 = 0x2C;
pub const OMAP_DMIC_IRQENABLE_CLR_REG: u32 = 0x30;
pub const OMAP_DMIC_IRQWAKE_EN_REG: u32 = 0x34;
pub const OMAP_DMIC_DMAENABLE_SET_REG: u32 = 0x38;
pub const OMAP_DMIC_DMAENABLE_CLR_REG: u32 = 0x3C;
pub const OMAP_DMIC_DMAWAKEEN_REG: u32 = 0x40;
pub const OMAP_DMIC_CTRL_REG: u32 = 0x44;
pub const OMAP_DMIC_DATA_REG: u32 = 0x48;
pub const OMAP_DMIC_FIFO_CTRL_REG: u32 = 0x4C;
pub const OMAP_DMIC_FIFO_DMIC1R_DATA_REG: u32 = 0x50;
pub const OMAP_DMIC_FIFO_DMIC1L_DATA_REG: u32 = 0x54;
pub const OMAP_DMIC_FIFO_DMIC2R_DATA_REG: u32 = 0x58;
pub const OMAP_DMIC_FIFO_DMIC2L_DATA_REG: u32 = 0x5C;
pub const OMAP_DMIC_FIFO_DMIC3R_DATA_REG: u32 = 0x60;
pub const OMAP_DMIC_FIFO_DMIC3L_DATA_REG: u32 = 0x64;

/* IRQSTATUS_RAW, IRQSTATUS, IRQENABLE_SET, IRQENABLE_CLR bit fields */
pub const OMAP_DMIC_IRQ: u32 = 1 << 0;
pub const OMAP_DMIC_IRQ_FULL: u32 = 1 << 1;
pub const OMAP_DMIC_IRQ_ALMST_EMPTY: u32 = 1 << 2;
pub const OMAP_DMIC_IRQ_EMPTY: u32 = 1 << 3;
pub const OMAP_DMIC_IRQ_MASK: u32 = 0x07;

/* DMIC_DMAENABLE bit fields */
pub const OMAP_DMIC_DMA_ENABLE: u32 = 0x1;

/* DMIC_CTRL bit fields */
pub const OMAP_DMIC_UP1_ENABLE: u32 = 1 << 0;
pub const OMAP_DMIC_UP2_ENABLE: u32 = 1 << 1;
pub const OMAP_DMIC_UP3_ENABLE: u32 = 1 << 2;
pub const OMAP_DMIC_UP_ENABLE_MASK: u32 = 0x7;
pub const OMAP_DMIC_FORMAT: u32 = 1 << 3;
pub const OMAP_DMIC_POLAR1: u32 = 1 << 4;
pub const OMAP_DMIC_POLAR2: u32 = 1 << 5;
pub const OMAP_DMIC_POLAR3: u32 = 1 << 6;
pub const OMAP_DMIC_POLAR_MASK: u32 = 0x7 << 4;
pub const fn OMAP_DMIC_CLK_DIV(x: u32) -> u32 {
    (x & 0x7) << 7
}
pub const OMAP_DMIC_CLK_DIV_MASK: u32 = 0x7 << 7;
pub const OMAP_DMIC_RESET: u32 = 1 << 10;

pub const OMAP_DMICOUTFORMAT_LJUST: u32 = 0 << 3;
pub const OMAP_DMICOUTFORMAT_RJUST: u32 = 1 << 3;

/* DMIC_FIFO_CTRL bit fields */
pub const OMAP_DMIC_THRES_MAX: u32 = 0xF;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum omap_dmic_clk {
    OMAP_DMIC_SYSCLK_PAD_CLKS,      /* PAD_CLKS */
    OMAP_DMIC_SYSCLK_SLIMBLUS_CLKS, /* SLIMBUS_CLK */
    OMAP_DMIC_SYSCLK_SYNC_MUX_CLKS, /* DMIC_SYNC_MUX_CLK */
    OMAP_DMIC_ABE_DMIC_CLK,         /* abe_dmic_clk */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
