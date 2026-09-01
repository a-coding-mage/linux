/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * sound/soc/rockchip/rockchip_i2s.h
 *
 * ALSA SoC Audio Layer - Rockchip I2S Controller driver
 *
 * Copyright (c) 2014 Rockchip Electronics Co. Ltd.
 * Author: Jianqun xu <jay.xu@rock-chips.com>
 */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

/*
 * TXCR
 * transmit operation control register
*/
pub const I2S_TXCR_RCNT_SHIFT: u32 = 17;
pub const I2S_TXCR_RCNT_MASK: u32 = 0x3f << I2S_TXCR_RCNT_SHIFT;
pub const I2S_TXCR_CSR_SHIFT: u32 = 15;
pub const fn I2S_TXCR_CSR(x: u32) -> u32 {
    x << I2S_TXCR_CSR_SHIFT
}
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
pub const fn I2S_TXCR_PBM_MODE(x: u32) -> u32 {
    x << I2S_TXCR_PBM_SHIFT
}
pub const I2S_TXCR_PBM_MASK: u32 = 3 << I2S_TXCR_PBM_SHIFT;
pub const I2S_TXCR_TFS_SHIFT: u32 = 5;
pub const I2S_TXCR_TFS_I2S: u32 = 0 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_PCM: u32 = 1 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_TFS_MASK: u32 = 1 << I2S_TXCR_TFS_SHIFT;
pub const I2S_TXCR_VDW_SHIFT: u32 = 0;
pub const fn I2S_TXCR_VDW(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_TXCR_VDW_SHIFT
}
pub const I2S_TXCR_VDW_MASK: u32 = 0x1f << I2S_TXCR_VDW_SHIFT;

/*
 * RXCR
 * receive operation control register
*/
pub const I2S_RXCR_CSR_SHIFT: u32 = 15;
pub const fn I2S_RXCR_CSR(x: u32) -> u32 {
    x << I2S_RXCR_CSR_SHIFT
}
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
pub const fn I2S_RXCR_PBM_MODE(x: u32) -> u32 {
    x << I2S_RXCR_PBM_SHIFT
}
pub const I2S_RXCR_PBM_MASK: u32 = 3 << I2S_RXCR_PBM_SHIFT;
pub const I2S_RXCR_TFS_SHIFT: u32 = 5;
pub const I2S_RXCR_TFS_I2S: u32 = 0 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_PCM: u32 = 1 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_TFS_MASK: u32 = 1 << I2S_RXCR_TFS_SHIFT;
pub const I2S_RXCR_VDW_SHIFT: u32 = 0;
pub const fn I2S_RXCR_VDW(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_RXCR_VDW_SHIFT
}
pub const I2S_RXCR_VDW_MASK: u32 = 0x1f << I2S_RXCR_VDW_SHIFT;

/*
 * CKR
 * clock generation register
*/
pub const I2S_CKR_TRCM_SHIFT: u32 = 28;
pub const fn I2S_CKR_TRCM(x: u32) -> u32 {
    x << I2S_CKR_TRCM_SHIFT
}
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
pub const fn I2S_CKR_MDIV(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_CKR_MDIV_SHIFT
}
pub const I2S_CKR_MDIV_MASK: u32 = 0xff << I2S_CKR_MDIV_SHIFT;
pub const I2S_CKR_RSD_SHIFT: u32 = 8;
pub const fn I2S_CKR_RSD(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_CKR_RSD_SHIFT
}
pub const I2S_CKR_RSD_MASK: u32 = 0xff << I2S_CKR_RSD_SHIFT;
pub const I2S_CKR_TSD_SHIFT: u32 = 0;
pub const fn I2S_CKR_TSD(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_CKR_TSD_SHIFT
}
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
pub const fn I2S_DMACR_RDL(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_DMACR_RDL_SHIFT
}
pub const I2S_DMACR_RDL_MASK: u32 = 0x1f << I2S_DMACR_RDL_SHIFT;
pub const I2S_DMACR_TDE_SHIFT: u32 = 8;
pub const I2S_DMACR_TDE_DISABLE: u32 = 0 << I2S_DMACR_TDE_SHIFT;
pub const I2S_DMACR_TDE_ENABLE: u32 = 1 << I2S_DMACR_TDE_SHIFT;
pub const I2S_DMACR_TDL_SHIFT: u32 = 0;
pub const fn I2S_DMACR_TDL(x: u32) -> u32 {
    x << I2S_DMACR_TDL_SHIFT
}
pub const I2S_DMACR_TDL_MASK: u32 = 0x1f << I2S_DMACR_TDL_SHIFT;

/*
 * INTCR
 * interrupt control register
*/
pub const I2S_INTCR_RFT_SHIFT: u32 = 20;
pub const fn I2S_INTCR_RFT(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_INTCR_RFT_SHIFT
}
pub const I2S_INTCR_RXOIC: u32 = BIT(18);
pub const I2S_INTCR_RXOIE_SHIFT: u32 = 17;
pub const I2S_INTCR_RXOIE_DISABLE: u32 = 0 << I2S_INTCR_RXOIE_SHIFT;
pub const I2S_INTCR_RXOIE_ENABLE: u32 = 1 << I2S_INTCR_RXOIE_SHIFT;
pub const I2S_INTCR_RXFIE_SHIFT: u32 = 16;
pub const I2S_INTCR_RXFIE_DISABLE: u32 = 0 << I2S_INTCR_RXFIE_SHIFT;
pub const I2S_INTCR_RXFIE_ENABLE: u32 = 1 << I2S_INTCR_RXFIE_SHIFT;
pub const I2S_INTCR_TFT_SHIFT: u32 = 4;
pub const fn I2S_INTCR_TFT(x: u32) -> u32 {
    x.wrapping_sub(1) << I2S_INTCR_TFT_SHIFT
}
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

/* Clock divider id */
pub const ROCKCHIP_DIV_MCLK: u32 = 0;
pub const ROCKCHIP_DIV_BCLK: u32 = 1;

/* channel select */
pub const I2S_CSR_SHIFT: u32 = 15;
pub const I2S_CHN_2: u32 = 0 << I2S_CSR_SHIFT;
pub const I2S_CHN_4: u32 = 1 << I2S_CSR_SHIFT;
pub const I2S_CHN_6: u32 = 2 << I2S_CSR_SHIFT;
pub const I2S_CHN_8: u32 = 3 << I2S_CSR_SHIFT;

/* I2S REGS */
pub const I2S_TXCR: u32 = 0x0000;
pub const I2S_RXCR: u32 = 0x0004;
pub const I2S_CKR: u32 = 0x0008;
pub const I2S_FIFOLR: u32 = 0x000c;
pub const I2S_DMACR: u32 = 0x0010;
pub const I2S_INTCR: u32 = 0x0014;
pub const I2S_INTSR: u32 = 0x0018;
pub const I2S_XFER: u32 = 0x001c;
pub const I2S_CLR: u32 = 0x0020;
pub const I2S_TXDR: u32 = 0x0024;
pub const I2S_RXDR: u32 = 0x0028;

/* io direction cfg register */
pub const I2S_IO_DIRECTION_MASK: u32 = 7;
pub const I2S_IO_8CH_OUT_2CH_IN: u32 = 0;
pub const I2S_IO_6CH_OUT_4CH_IN: u32 = 4;
pub const I2S_IO_4CH_OUT_6CH_IN: u32 = 6;
pub const I2S_IO_2CH_OUT_8CH_IN: u32 = 7;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
