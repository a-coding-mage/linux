/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC Audio Layer - Rockchip I2S/TDM Controller driver
 *
 * Copyright (c) 2018 Rockchip Electronics Co. Ltd.
 * Author: Sugar Zhang <sugar.zhang@rock-chips.com>
 *
 */

/* C source included <linux/hw_bitfield.h> for BIT, GENMASK, and FIELD_PREP_WM16_CONST. */

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h >= 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !(BIT(l) - 1)
    }
}

pub const fn __bf_shf(x: u32) -> u32 {
    x.trailing_zeros()
}

pub const fn FIELD_PREP_WM16_CONST(mask: u32, val: u32) -> u32 {
    ((mask) << 16) | (((val) << __bf_shf(mask)) & (mask))
}

/*
 * TXCR
 * transmit operation control register
 */
pub const fn I2S_TXCR_PATH_SHIFT(x: u32) -> u32 { 23 + x * 2 }
pub const fn I2S_TXCR_PATH_MASK(x: u32) -> u32 { 0x3 << I2S_TXCR_PATH_SHIFT(x) }
pub const fn I2S_TXCR_PATH(x: u32, v: u32) -> u32 { v << I2S_TXCR_PATH_SHIFT(x) }
pub const I2S_TXCR_RCNT_SHIFT: u32 = 17;
pub const I2S_TXCR_RCNT_MASK: u32 = 0x3f << I2S_TXCR_RCNT_SHIFT;
pub const I2S_TXCR_CSR_SHIFT: u32 = 15;
pub const fn I2S_TXCR_CSR(x: u32) -> u32 { x << I2S_TXCR_CSR_SHIFT }
pub const I2S_TXCR_CSR_MASK: u32 = 3 << I2S_TXCR_CSR_SHIFT;
pub const I2S_TXCR_HWT: u32 = BIT(14);
pub const I2S_TXCR_SJM_SHIFT: u32 = 12;
pub const I2S_TXCR_SJM_R: u32 = 0 << I2S_TXCR_SJM_SHIFT;
pub const I2S_TXCR_SJM_L: u32 = 1 << I2S_TXCR_SJM_SHIFT;
pub const I2S_TXCR_FBM_SHIFT: u32 = 11;
pub const I2S_TXCR_FBM_MSB: u32 = 0 << I2S_TXCR_FBM_SHIFT;
pub const I2S_TXCR_FBM_LSB: u32 = 1 << I2S_TXCR_FBM_SHIFT;
pub const I2S_TXCR_IBM_SHIFT: u32 = 9;
pub const I2S_TXCR_IBM_NORMAL: u32 = 0 << I2S_TXCR_IBM_SHIFT;
pub const I2S_TXCR_IBM_LSJM: u32 = 1 << I2S_TXCR_IBM_SHIFT;
pub const I2S_TXCR_IBM_RSJM: u32 = 2 << I2S_TXCR_IBM_SHIFT;
pub const I2S_TXCR_IBM_MASK: u32 = 3 << I2S_TXCR_IBM_SHIFT;
pub const I2S_TXCR_PBM_SHIFT: u32 = 7;
pub const fn I2S_TXCR_PBM_MODE(x: u32) -> u32 { x << I2S_TXCR_PBM_SHIFT }
pub const I2S_TXCR_PBM_MASK: u32 = 3 << I2S_TXCR_PBM_SHIFT;
pub const I2S_TXCR_TFS_SHIFT: u32 = 5;
pub const I2S_TXCR_TFS_I2S: u32 = 0 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_PCM: u32 = 1 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_TDM_PCM: u32 = 2 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_TDM_I2S: u32 = 3 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_MASK: u32 = 3 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_VDW_SHIFT: u32 = 0;
pub const fn I2S_TXCR_VDW(x: u32) -> u32 { x.wrapping_sub(1) << I2S_TXCR_VDW_SHIFT }
pub const I2S_TXCR_VDW_MASK: u32 = 0x1f << I2S_TXCR_VDW_SHIFT;

/*
 * RXCR
 * receive operation control register
 */
pub const fn I2S_RXCR_PATH_SHIFT(x: u32) -> u32 { 17 + x * 2 }
pub const fn I2S_RXCR_PATH_MASK(x: u32) -> u32 { 0x3 << I2S_RXCR_PATH_SHIFT(x) }
pub const fn I2S_RXCR_PATH(x: u32, v: u32) -> u32 { v << I2S_RXCR_PATH_SHIFT(x) }
pub const I2S_RXCR_CSR_SHIFT: u32 = 15;
pub const fn I2S_RXCR_CSR(x: u32) -> u32 { x << I2S_RXCR_CSR_SHIFT }
pub const I2S_RXCR_CSR_MASK: u32 = 3 << I2S_RXCR_CSR_SHIFT;
pub const I2S_RXCR_HWT: u32 = BIT(14);
pub const I2S_RXCR_SJM_SHIFT: u32 = 12;
pub const I2S_RXCR_SJM_R: u32 = 0 << I2S_RXCR_SJM_SHIFT;
pub const I2S_RXCR_SJM_L: u32 = 1 << I2S_RXCR_SJM_SHIFT;
pub const I2S_RXCR_FBM_SHIFT: u32 = 11;
pub const I2S_RXCR_FBM_MSB: u32 = 0 << I2S_RXCR_FBM_SHIFT;
pub const I2S_RXCR_FBM_LSB: u32 = 1 << I2S_RXCR_FBM_SHIFT;
pub const I2S_RXCR_IBM_SHIFT: u32 = 9;
pub const I2S_RXCR_IBM_NORMAL: u32 = 0 << I2S_RXCR_IBM_SHIFT;
pub const I2S_RXCR_IBM_LSJM: u32 = 1 << I2S_RXCR_IBM_SHIFT;
pub const I2S_RXCR_IBM_RSJM: u32 = 2 << I2S_RXCR_IBM_SHIFT;
pub const I2S_RXCR_IBM_MASK: u32 = 3 << I2S_RXCR_IBM_SHIFT;
pub const I2S_RXCR_PBM_SHIFT: u32 = 7;
pub const fn I2S_RXCR_PBM_MODE(x: u32) -> u32 { x << I2S_RXCR_PBM_SHIFT }
pub const I2S_RXCR_PBM_MASK: u32 = 3 << I2S_RXCR_PBM_SHIFT;
pub const I2S_RXCR_TFS_SHIFT: u32 = 5;
pub const I2S_RXCR_TFS_I2S: u32 = 0 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_PCM: u32 = 1 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_TDM_PCM: u32 = 2 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_TDM_I2S: u32 = 3 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_MASK: u32 = 3 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_VDW_SHIFT: u32 = 0;
pub const fn I2S_RXCR_VDW(x: u32) -> u32 { x.wrapping_sub(1) << I2S_RXCR_VDW_SHIFT }
pub const I2S_RXCR_VDW_MASK: u32 = 0x1f << I2S_RXCR_VDW_SHIFT;

/*
 * CKR
 * clock generation register
 */
pub const I2S_CKR_TRCM_SHIFT: u32 = 28;
pub const fn I2S_CKR_TRCM(x: u32) -> u32 { x << I2S_CKR_TRCM_SHIFT }
pub const I2S_CKR_TRCM_TXRX: u32 = 0 << I2S_CKR_TRCM_SHIFT;
pub const I2S_CKR_TRCM_TXONLY: u32 = 1 << I2S_CKR_TRCM_SHIFT;
pub const I2S_CKR_TRCM_RXONLY: u32 = 2 << I2S_CKR_TRCM_SHIFT;
pub const I2S_CKR_TRCM_MASK: u32 = 3 << I2S_CKR_TRCM_SHIFT;
pub const I2S_CKR_MSS_SHIFT: u32 = 27;
pub const I2S_CKR_MSS_MASTER: u32 = 0 << I2S_CKR_MSS_SHIFT;
pub const I2S_CKR_MSS_SLAVE: u32 = 1 << I2S_CKR_MSS_SHIFT;
pub const I2S_CKR_MSS_MASK: u32 = 1 << I2S_CKR_MSS_SHIFT;
pub const I2S_CKR_CKP_SHIFT: u32 = 26;
pub const I2S_CKR_CKP_NORMAL: u32 = 0 << I2S_CKR_CKP_SHIFT;
pub const I2S_CKR_CKP_INVERTED: u32 = 1 << I2S_CKR_CKP_SHIFT;
pub const I2S_CKR_CKP_MASK: u32 = 1 << I2S_CKR_CKP_SHIFT;
pub const I2S_CKR_RLP_SHIFT: u32 = 25;
pub const I2S_CKR_RLP_NORMAL: u32 = 0 << I2S_CKR_RLP_SHIFT;
pub const I2S_CKR_RLP_INVERTED: u32 = 1 << I2S_CKR_RLP_SHIFT;
pub const I2S_CKR_RLP_MASK: u32 = 1 << I2S_CKR_RLP_SHIFT;
pub const I2S_CKR_TLP_SHIFT: u32 = 24;
pub const I2S_CKR_TLP_NORMAL: u32 = 0 << I2S_CKR_TLP_SHIFT;
pub const I2S_CKR_TLP_INVERTED: u32 = 1 << I2S_CKR_TLP_SHIFT;
pub const I2S_CKR_TLP_MASK: u32 = 1 << I2S_CKR_TLP_SHIFT;
pub const I2S_CKR_MDIV_SHIFT: u32 = 16;
pub const fn I2S_CKR_MDIV(x: u32) -> u32 { x.wrapping_sub(1) << I2S_CKR_MDIV_SHIFT }
pub const I2S_CKR_MDIV_MASK: u32 = 0xff << I2S_CKR_MDIV_SHIFT;
pub const I2S_CKR_RSD_SHIFT: u32 = 8;
pub const fn I2S_CKR_RSD(x: u32) -> u32 { x.wrapping_sub(1) << I2S_CKR_RSD_SHIFT }
pub const I2S_CKR_RSD_MASK: u32 = 0xff << I2S_CKR_RSD_SHIFT;
pub const I2S_CKR_TSD_SHIFT: u32 = 0;
pub const fn I2S_CKR_TSD(x: u32) -> u32 { x.wrapping_sub(1) << I2S_CKR_TSD_SHIFT }
pub const I2S_CKR_TSD_MASK: u32 = 0xff << I2S_CKR_TSD_SHIFT;

/*
 * FIFOLR
 * FIFO level register
 */
pub const I2S_FIFOLR_RFL_SHIFT: u32 = 24;
pub const I2S_FIFOLR_RFL_MASK: u32 = 0x3f << I2S_FIFOLR_RFL_SHIFT;
pub const I2S_FIFOLR_TFL3_SHIFT: u32 = 18;
pub const I2S_FIFOLR_TFL3_MASK: u32 = 0x3f << I2S_FIFOLR_TFL3_SHIFT;
pub const I2S_FIFOLR_TFL2_SHIFT: u32 = 12;
pub const I2S_FIFOLR_TFL2_MASK: u32 = 0x3f << I2S_FIFOLR_TFL2_SHIFT;
pub const I2S_FIFOLR_TFL1_SHIFT: u32 = 6;
pub const I2S_FIFOLR_TFL1_MASK: u32 = 0x3f << I2S_FIFOLR_TFL1_SHIFT;
pub const I2S_FIFOLR_TFL0_SHIFT: u32 = 0;
pub const I2S_FIFOLR_TFL0_MASK: u32 = 0x3f << I2S_FIFOLR_TFL0_SHIFT;

/*
 * DMACR
 * DMA control register
 */
pub const I2S_DMACR_RDE_SHIFT: u32 = 24;
pub const I2S_DMACR_RDE_DISABLE: u32 = 0 << I2S_DMACR_RDE_SHIFT;
pub const I2S_DMACR_RDE_ENABLE: u32 = 1 << I2S_DMACR_RDE_SHIFT;
pub const I2S_DMACR_RDL_SHIFT: u32 = 16;
pub const fn I2S_DMACR_RDL(x: u32) -> u32 { x.wrapping_sub(1) << I2S_DMACR_RDL_SHIFT }
pub const I2S_DMACR_RDL_MASK: u32 = 0x1f << I2S_DMACR_RDL_SHIFT;
pub const I2S_DMACR_TDE_SHIFT: u32 = 8;
pub const I2S_DMACR_TDE_DISABLE: u32 = 0 << I2S_DMACR_TDE_SHIFT;
pub const I2S_DMACR_TDE_ENABLE: u32 = 1 << I2S_DMACR_TDE_SHIFT;
pub const I2S_DMACR_TDL_SHIFT: u32 = 0;
pub const fn I2S_DMACR_TDL(x: u32) -> u32 { x << I2S_DMACR_TDL_SHIFT }
pub const I2S_DMACR_TDL_MASK: u32 = 0x1f << I2S_DMACR_TDL_SHIFT;

/*
 * INTCR
 * interrupt control register
 */
pub const I2S_INTCR_RFT_SHIFT: u32 = 20;
pub const fn I2S_INTCR_RFT(x: u32) -> u32 { x.wrapping_sub(1) << I2S_INTCR_RFT_SHIFT }
pub const I2S_INTCR_RXOIC: u32 = BIT(18);
pub const I2S_INTCR_RXOIE_SHIFT: u32 = 17;
pub const I2S_INTCR_RXOIE_DISABLE: u32 = 0 << I2S_INTCR_RXOIE_SHIFT;
pub const I2S_INTCR_RXOIE_ENABLE: u32 = 1 << I2S_INTCR_RXOIE_SHIFT;
pub const I2S_INTCR_RXFIE_SHIFT: u32 = 16;
pub const I2S_INTCR_RXFIE_DISABLE: u32 = 0 << I2S_INTCR_RXFIE_SHIFT;
pub const I2S_INTCR_RXFIE_ENABLE: u32 = 1 << I2S_INTCR_RXFIE_SHIFT;
pub const I2S_INTCR_TFT_SHIFT: u32 = 4;
pub const fn I2S_INTCR_TFT(x: u32) -> u32 { x.wrapping_sub(1) << I2S_INTCR_TFT_SHIFT }
pub const I2S_INTCR_TFT_MASK: u32 = 0x1f << I2S_INTCR_TFT_SHIFT;
pub const I2S_INTCR_TXUIC: u32 = BIT(2);
pub const I2S_INTCR_TXUIE_SHIFT: u32 = 1;
pub const I2S_INTCR_TXUIE_DISABLE: u32 = 0 << I2S_INTCR_TXUIE_SHIFT;
pub const I2S_INTCR_TXUIE_ENABLE: u32 = 1 << I2S_INTCR_TXUIE_SHIFT;

/*
 * INTSR
 * interrupt status register
 */
pub const I2S_INTSR_TXEIE_SHIFT: u32 = 0;
pub const I2S_INTSR_TXEIE_DISABLE: u32 = 0 << I2S_INTSR_TXEIE_SHIFT;
pub const I2S_INTSR_TXEIE_ENABLE: u32 = 1 << I2S_INTSR_TXEIE_SHIFT;
pub const I2S_INTSR_RXOI_SHIFT: u32 = 17;
pub const I2S_INTSR_RXOI_INA: u32 = 0 << I2S_INTSR_RXOI_SHIFT;
pub const I2S_INTSR_RXOI_ACT: u32 = 1 << I2S_INTSR_RXOI_SHIFT;
pub const I2S_INTSR_RXFI_SHIFT: u32 = 16;
pub const I2S_INTSR_RXFI_INA: u32 = 0 << I2S_INTSR_RXFI_SHIFT;
pub const I2S_INTSR_RXFI_ACT: u32 = 1 << I2S_INTSR_RXFI_SHIFT;
pub const I2S_INTSR_TXUI_SHIFT: u32 = 1;
pub const I2S_INTSR_TXUI_INA: u32 = 0 << I2S_INTSR_TXUI_SHIFT;
pub const I2S_INTSR_TXUI_ACT: u32 = 1 << I2S_INTSR_TXUI_SHIFT;
pub const I2S_INTSR_TXEI_SHIFT: u32 = 0;
pub const I2S_INTSR_TXEI_INA: u32 = 0 << I2S_INTSR_TXEI_SHIFT;
pub const I2S_INTSR_TXEI_ACT: u32 = 1 << I2S_INTSR_TXEI_SHIFT;

/*
 * XFER
 * Transfer start register
 */
pub const I2S_XFER_RXS_SHIFT: u32 = 1;
pub const I2S_XFER_RXS_STOP: u32 = 0 << I2S_XFER_RXS_SHIFT;
pub const I2S_XFER_RXS_START: u32 = 1 << I2S_XFER_RXS_SHIFT;
pub const I2S_XFER_TXS_SHIFT: u32 = 0;
pub const I2S_XFER_TXS_STOP: u32 = 0 << I2S_XFER_TXS_SHIFT;
pub const I2S_XFER_TXS_START: u32 = 1 << I2S_XFER_TXS_SHIFT;

/*
 * CLR
 * clear SCLK domain logic register
 */
pub const I2S_CLR_RXC: u32 = BIT(1);
pub const I2S_CLR_TXC: u32 = BIT(0);

/*
 * TXDR
 * Transimt FIFO data register, write only.
 */
pub const I2S_TXDR_MASK: u32 = 0xff;

/*
 * RXDR
 * Receive FIFO data register, write only.
 */
pub const I2S_RXDR_MASK: u32 = 0xff;

/*
 * TDM_CTRL
 * TDM ctrl register
 */
pub const TDM_FSYNC_WIDTH_SEL1_MSK: u32 = GENMASK(20, 18);
pub const fn TDM_FSYNC_WIDTH_SEL1(x: u32) -> u32 { x.wrapping_sub(1) << 18 }
pub const TDM_FSYNC_WIDTH_SEL0_MSK: u32 = BIT(17);
pub const TDM_FSYNC_WIDTH_HALF_FRAME: u32 = 0;
pub const TDM_FSYNC_WIDTH_ONE_FRAME: u32 = BIT(17);
pub const TDM_SHIFT_CTRL_MSK: u32 = GENMASK(16, 14);
pub const fn TDM_SHIFT_CTRL(x: u32) -> u32 { x << 14 }
pub const TDM_SLOT_BIT_WIDTH_MSK: u32 = GENMASK(13, 9);
pub const fn TDM_SLOT_BIT_WIDTH(x: u32) -> u32 { x.wrapping_sub(1) << 9 }
pub const TDM_FRAME_WIDTH_MSK: u32 = GENMASK(8, 0);
pub const fn TDM_FRAME_WIDTH(x: u32) -> u32 { x.wrapping_sub(1) << 0 }

/*
 * CLKDIV
 * Mclk div register
 */
pub const I2S_CLKDIV_TXM_SHIFT: u32 = 0;
pub const fn I2S_CLKDIV_TXM(x: u32) -> u32 { x.wrapping_sub(1) << I2S_CLKDIV_TXM_SHIFT }
pub const I2S_CLKDIV_TXM_MASK: u32 = 0xff << I2S_CLKDIV_TXM_SHIFT;
pub const I2S_CLKDIV_RXM_SHIFT: u32 = 8;
pub const fn I2S_CLKDIV_RXM(x: u32) -> u32 { x.wrapping_sub(1) << I2S_CLKDIV_RXM_SHIFT }
pub const I2S_CLKDIV_RXM_MASK: u32 = 0xff << I2S_CLKDIV_RXM_SHIFT;

/* Clock divider id */
pub const ROCKCHIP_DIV_MCLK: u32 = 0;
pub const ROCKCHIP_DIV_BCLK: u32 = 1;

/* channel select */
pub const I2S_CSR_SHIFT: u32 = 15;
pub const I2S_CHN_2: u32 = 0 << I2S_CSR_SHIFT;
pub const I2S_CHN_4: u32 = 1 << I2S_CSR_SHIFT;
pub const I2S_CHN_6: u32 = 2 << I2S_CSR_SHIFT;
pub const I2S_CHN_8: u32 = 3 << I2S_CSR_SHIFT;

/* io direction cfg register */
pub const I2S_IO_DIRECTION_MASK: u32 = 7;
pub const I2S_IO_8CH_OUT_2CH_IN: u32 = 7;
pub const I2S_IO_6CH_OUT_4CH_IN: u32 = 3;
pub const I2S_IO_4CH_OUT_6CH_IN: u32 = 1;
pub const I2S_IO_2CH_OUT_8CH_IN: u32 = 0;

/* I2S REGS */
pub const I2S_TXCR: u32 = 0x0000;
pub const I2S_RXCR: u32 = 0x0004;
pub const I2S_CKR: u32 = 0x0008;
pub const I2S_TXFIFOLR: u32 = 0x000c;
pub const I2S_DMACR: u32 = 0x0010;
pub const I2S_INTCR: u32 = 0x0014;
pub const I2S_INTSR: u32 = 0x0018;
pub const I2S_XFER: u32 = 0x001c;
pub const I2S_CLR: u32 = 0x0020;
pub const I2S_TXDR: u32 = 0x0024;
pub const I2S_RXDR: u32 = 0x0028;
pub const I2S_RXFIFOLR: u32 = 0x002c;
pub const I2S_TDM_TXCR: u32 = 0x0030;
pub const I2S_TDM_RXCR: u32 = 0x0034;
pub const I2S_CLKDIV: u32 = 0x0038;

pub const fn HIWORD_UPDATE(v: u32, h: u32, l: u32) -> u32 {
    FIELD_PREP_WM16_CONST(GENMASK(h, l), v)
}

/* PX30 GRF CONFIGS */
pub const PX30_I2S0_CLK_IN_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 13, 12);
pub const PX30_I2S0_CLK_IN_SRC_FROM_RX: u32 = HIWORD_UPDATE(2, 13, 12);
pub const PX30_I2S0_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 5, 5);
pub const PX30_I2S0_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 5, 5);

pub const PX30_I2S0_CLK_TXONLY: u32 =
    PX30_I2S0_MCLK_OUT_SRC_FROM_TX | PX30_I2S0_CLK_IN_SRC_FROM_TX;

pub const PX30_I2S0_CLK_RXONLY: u32 =
    PX30_I2S0_MCLK_OUT_SRC_FROM_RX | PX30_I2S0_CLK_IN_SRC_FROM_RX;

/* RK1808 GRF CONFIGS */
pub const RK1808_I2S0_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 2, 2);
pub const RK1808_I2S0_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 2, 2);
pub const RK1808_I2S0_CLK_IN_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 1, 0);
pub const RK1808_I2S0_CLK_IN_SRC_FROM_RX: u32 = HIWORD_UPDATE(2, 1, 0);

pub const RK1808_I2S0_CLK_TXONLY: u32 =
    RK1808_I2S0_MCLK_OUT_SRC_FROM_TX | RK1808_I2S0_CLK_IN_SRC_FROM_TX;

pub const RK1808_I2S0_CLK_RXONLY: u32 =
    RK1808_I2S0_MCLK_OUT_SRC_FROM_RX | RK1808_I2S0_CLK_IN_SRC_FROM_RX;

/* RK3308 GRF CONFIGS */
pub const RK3308_I2S0_8CH_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 10, 10);
pub const RK3308_I2S0_8CH_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 10, 10);
pub const RK3308_I2S0_8CH_CLK_IN_RX_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 9, 9);
pub const RK3308_I2S0_8CH_CLK_IN_RX_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 9, 9);
pub const RK3308_I2S0_8CH_CLK_IN_TX_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 8, 8);
pub const RK3308_I2S0_8CH_CLK_IN_TX_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 8, 8);
pub const RK3308_I2S1_8CH_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 2, 2);
pub const RK3308_I2S1_8CH_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 2, 2);
pub const RK3308_I2S1_8CH_CLK_IN_RX_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 1, 1);
pub const RK3308_I2S1_8CH_CLK_IN_RX_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 1, 1);
pub const RK3308_I2S1_8CH_CLK_IN_TX_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 0, 0);
pub const RK3308_I2S1_8CH_CLK_IN_TX_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 0, 0);

pub const RK3308_I2S0_CLK_TXONLY: u32 =
    RK3308_I2S0_8CH_MCLK_OUT_SRC_FROM_TX |
    RK3308_I2S0_8CH_CLK_IN_RX_SRC_FROM_TX |
    RK3308_I2S0_8CH_CLK_IN_TX_SRC_FROM_TX;

pub const RK3308_I2S0_CLK_RXONLY: u32 =
    RK3308_I2S0_8CH_MCLK_OUT_SRC_FROM_RX |
    RK3308_I2S0_8CH_CLK_IN_RX_SRC_FROM_RX |
    RK3308_I2S0_8CH_CLK_IN_TX_SRC_FROM_RX;

pub const RK3308_I2S1_CLK_TXONLY: u32 =
    RK3308_I2S1_8CH_MCLK_OUT_SRC_FROM_TX |
    RK3308_I2S1_8CH_CLK_IN_RX_SRC_FROM_TX |
    RK3308_I2S1_8CH_CLK_IN_TX_SRC_FROM_TX;

pub const RK3308_I2S1_CLK_RXONLY: u32 =
    RK3308_I2S1_8CH_MCLK_OUT_SRC_FROM_RX |
    RK3308_I2S1_8CH_CLK_IN_RX_SRC_FROM_RX |
    RK3308_I2S1_8CH_CLK_IN_TX_SRC_FROM_RX;

/* RK3568 GRF CONFIGS */
pub const RK3568_I2S1_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 5, 5);
pub const RK3568_I2S1_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 5, 5);

pub const RK3568_I2S1_CLK_TXONLY: u32 =
    RK3568_I2S1_MCLK_OUT_SRC_FROM_TX;

pub const RK3568_I2S1_CLK_RXONLY: u32 =
    RK3568_I2S1_MCLK_OUT_SRC_FROM_RX;

pub const RK3568_I2S3_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 15, 15);
pub const RK3568_I2S3_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 15, 15);
pub const RK3568_I2S3_SCLK_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 7, 7);
pub const RK3568_I2S3_SCLK_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 7, 7);
pub const RK3568_I2S3_LRCK_SRC_FROM_TX: u32 = HIWORD_UPDATE(1, 6, 6);
pub const RK3568_I2S3_LRCK_SRC_FROM_RX: u32 = HIWORD_UPDATE(0, 6, 6);

pub const RK3568_I2S3_MCLK_TXONLY: u32 =
    RK3568_I2S3_MCLK_OUT_SRC_FROM_TX;

pub const RK3568_I2S3_CLK_TXONLY: u32 =
    RK3568_I2S3_SCLK_SRC_FROM_TX |
    RK3568_I2S3_LRCK_SRC_FROM_TX;

pub const RK3568_I2S3_MCLK_RXONLY: u32 =
    RK3568_I2S3_MCLK_OUT_SRC_FROM_RX;

pub const RK3568_I2S3_CLK_RXONLY: u32 =
    RK3568_I2S3_SCLK_SRC_FROM_RX |
    RK3568_I2S3_LRCK_SRC_FROM_RX;

pub const RK3568_I2S3_MCLK_IE: u32 = HIWORD_UPDATE(0, 3, 3);
pub const RK3568_I2S3_MCLK_OE: u32 = HIWORD_UPDATE(1, 3, 3);
pub const RK3568_I2S2_MCLK_IE: u32 = HIWORD_UPDATE(0, 2, 2);
pub const RK3568_I2S2_MCLK_OE: u32 = HIWORD_UPDATE(1, 2, 2);
pub const RK3568_I2S1_MCLK_TX_IE: u32 = HIWORD_UPDATE(0, 1, 1);
pub const RK3568_I2S1_MCLK_TX_OE: u32 = HIWORD_UPDATE(1, 1, 1);
pub const RK3568_I2S1_MCLK_RX_IE: u32 = HIWORD_UPDATE(0, 0, 0);
pub const RK3568_I2S1_MCLK_RX_OE: u32 = HIWORD_UPDATE(1, 0, 0);

/* RV1126 GRF CONFIGS */
pub const RV1126_I2S0_MCLK_OUT_SRC_FROM_TX: u32 = HIWORD_UPDATE(0, 9, 9);
pub const RV1126_I2S0_MCLK_OUT_SRC_FROM_RX: u32 = HIWORD_UPDATE(1, 9, 9);

pub const RV1126_I2S0_CLK_TXONLY: u32 =
    RV1126_I2S0_MCLK_OUT_SRC_FROM_TX;

pub const RV1126_I2S0_CLK_RXONLY: u32 =
    RV1126_I2S0_MCLK_OUT_SRC_FROM_RX;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
