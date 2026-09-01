/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ALSA SoC Audio Layer - Rockchip SAI Controller driver
 *
 * Copyright (c) 2022 Rockchip Electronics Co. Ltd.
 */

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

const fn genmask(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* XCR Transmit / Receive Control Register */
pub const SAI_XCR_START_SEL_MASK: u32 = bit(23);
pub const SAI_XCR_START_SEL_CHAINED: u32 = bit(23);
pub const SAI_XCR_START_SEL_STANDALONE: u32 = 0;
pub const SAI_XCR_EDGE_SHIFT_MASK: u32 = bit(22);
pub const SAI_XCR_EDGE_SHIFT_1: u32 = bit(22);
pub const SAI_XCR_EDGE_SHIFT_0: u32 = 0;
pub const SAI_XCR_CSR_MASK: u32 = genmask(21, 20);
pub const fn SAI_XCR_CSR(x: u32) -> u32 {
    (x - 1) << 20
}
pub const fn SAI_XCR_CSR_V(v: u32) -> u32 {
    (((v) & SAI_XCR_CSR_MASK) >> 20) + 1
}
pub const SAI_XCR_SJM_MASK: u32 = bit(19);
pub const SAI_XCR_SJM_L: u32 = bit(19);
pub const SAI_XCR_SJM_R: u32 = 0;
pub const SAI_XCR_FBM_MASK: u32 = bit(18);
pub const SAI_XCR_FBM_LSB: u32 = bit(18);
pub const SAI_XCR_FBM_MSB: u32 = 0;
pub const SAI_XCR_SNB_MASK: u32 = genmask(17, 11);
pub const fn SAI_XCR_SNB(x: u32) -> u32 {
    (x - 1) << 11
}
pub const SAI_XCR_VDJ_MASK: u32 = bit(10);
pub const SAI_XCR_VDJ_L: u32 = bit(10);
pub const SAI_XCR_VDJ_R: u32 = 0;
pub const SAI_XCR_SBW_MASK: u32 = genmask(9, 5);
pub const fn SAI_XCR_SBW(x: u32) -> u32 {
    (x - 1) << 5
}
pub const fn SAI_XCR_SBW_V(v: u32) -> u32 {
    (((v) & SAI_XCR_SBW_MASK) >> 5) + 1
}
pub const SAI_XCR_VDW_MASK: u32 = genmask(4, 0);
pub const fn SAI_XCR_VDW(x: u32) -> u32 {
    (x - 1) << 0
}

/* FSCR Frame Sync Control Register */
pub const SAI_FSCR_EDGE_MASK: u32 = bit(24);
pub const SAI_FSCR_EDGE_DUAL: u32 = bit(24);
pub const SAI_FSCR_EDGE_RISING: u32 = 0;
pub const SAI_FSCR_FPW_MASK: u32 = genmask(23, 12);
pub const fn SAI_FSCR_FPW(x: u32) -> u32 {
    (x - 1) << 12
}
pub const SAI_FSCR_FW_MASK: u32 = genmask(11, 0);
pub const fn SAI_FSCR_FW(x: u32) -> u32 {
    (x - 1) << 0
}
pub const fn SAI_FSCR_FW_V(v: u32) -> u32 {
    (((v) & SAI_FSCR_FW_MASK) >> 0) + 1
}

/* MONO_CR Mono Control Register */
pub const SAI_MCR_RX_MONO_SLOT_MASK: u32 = genmask(8, 2);
pub const fn SAI_MCR_RX_MONO_SLOT_SEL(x: u32) -> u32 {
    (x - 1) << 2
}
pub const SAI_MCR_RX_MONO_MASK: u32 = bit(1);
pub const SAI_MCR_RX_MONO_EN: u32 = bit(1);
pub const SAI_MCR_RX_MONO_DIS: u32 = 0;
pub const SAI_MCR_TX_MONO_MASK: u32 = bit(0);
pub const SAI_MCR_TX_MONO_EN: u32 = bit(0);
pub const SAI_MCR_TX_MONO_DIS: u32 = 0;

/* XFER Transfer Start Register */
pub const SAI_XFER_RX_IDLE: u32 = bit(8);
pub const SAI_XFER_TX_IDLE: u32 = bit(7);
pub const SAI_XFER_FS_IDLE: u32 = bit(6);
/*
 * Used for TX only (VERSION >= SAI_VER_2311)
 *
 * SCLK/FSYNC auto gated when TX FIFO empty.
 */
pub const SAI_XFER_TX_AUTO_MASK: u32 = bit(6);
pub const SAI_XFER_TX_AUTO_EN: u32 = bit(6);
pub const SAI_XFER_TX_AUTO_DIS: u32 = 0;
pub const SAI_XFER_RX_CNT_MASK: u32 = bit(5);
pub const SAI_XFER_RX_CNT_EN: u32 = bit(5);
pub const SAI_XFER_RX_CNT_DIS: u32 = 0;
pub const SAI_XFER_TX_CNT_MASK: u32 = bit(4);
pub const SAI_XFER_TX_CNT_EN: u32 = bit(4);
pub const SAI_XFER_TX_CNT_DIS: u32 = 0;
pub const SAI_XFER_RXS_MASK: u32 = bit(3);
pub const SAI_XFER_RXS_EN: u32 = bit(3);
pub const SAI_XFER_RXS_DIS: u32 = 0;
pub const SAI_XFER_TXS_MASK: u32 = bit(2);
pub const SAI_XFER_TXS_EN: u32 = bit(2);
pub const SAI_XFER_TXS_DIS: u32 = 0;
pub const SAI_XFER_FSS_MASK: u32 = bit(1);
pub const SAI_XFER_FSS_EN: u32 = bit(1);
pub const SAI_XFER_FSS_DIS: u32 = 0;
pub const SAI_XFER_CLK_MASK: u32 = bit(0);
pub const SAI_XFER_CLK_EN: u32 = bit(0);
pub const SAI_XFER_CLK_DIS: u32 = 0;

/* CLR Clear Logic Register */
pub const SAI_CLR_FCR: u32 = bit(3); /* TODO: what is this? */
pub const SAI_CLR_FSC: u32 = bit(2);
pub const SAI_CLR_RXC: u32 = bit(1);
pub const SAI_CLR_TXC: u32 = bit(0);

/* CKR Clock Generation Register */
pub const SAI_CKR_MDIV_MASK: u32 = genmask(14, 3);
pub const fn SAI_CKR_MDIV(x: u32) -> u32 {
    (x - 1) << 3
}
pub const SAI_CKR_MSS_MASK: u32 = bit(2);
pub const SAI_CKR_MSS_SLAVE: u32 = bit(2);
pub const SAI_CKR_MSS_MASTER: u32 = 0;
pub const SAI_CKR_CKP_MASK: u32 = bit(1);
pub const SAI_CKR_CKP_INVERTED: u32 = bit(1);
pub const SAI_CKR_CKP_NORMAL: u32 = 0;
pub const SAI_CKR_FSP_MASK: u32 = bit(0);
pub const SAI_CKR_FSP_INVERTED: u32 = bit(0);
pub const SAI_CKR_FSP_NORMAL: u32 = 0;

/* DMACR DMA Control Register */
pub const SAI_DMACR_RDE_MASK: u32 = bit(24);
pub const fn SAI_DMACR_RDE(x: u32) -> u32 {
    (x) << 24
}
pub const SAI_DMACR_RDL_MASK: u32 = genmask(20, 16);
pub const fn SAI_DMACR_RDL(x: u32) -> u32 {
    (x - 1) << 16
}
pub const fn SAI_DMACR_RDL_V(v: u32) -> u32 {
    (((v) & SAI_DMACR_RDL_MASK) >> 16) + 1
}
pub const SAI_DMACR_TDE_MASK: u32 = bit(8);
pub const fn SAI_DMACR_TDE(x: u32) -> u32 {
    (x) << 8
}
pub const SAI_DMACR_TDL_MASK: u32 = genmask(4, 0);
pub const fn SAI_DMACR_TDL(x: u32) -> u32 {
    (x) << 0
}
pub const fn SAI_DMACR_TDL_V(v: u32) -> u32 {
    ((v) & SAI_DMACR_TDL_MASK) >> 0
}

/* INTCR Interrupt Ctrl Register */
pub const SAI_INTCR_FSLOSTC: u32 = bit(28);
pub const SAI_INTCR_FSLOST_MASK: u32 = bit(27);
pub const fn SAI_INTCR_FSLOST(x: u32) -> u32 {
    (x) << 27
}
pub const SAI_INTCR_FSERRC: u32 = bit(26);
pub const SAI_INTCR_FSERR_MASK: u32 = bit(25);
pub const fn SAI_INTCR_FSERR(x: u32) -> u32 {
    (x) << 25
}
pub const SAI_INTCR_RXOIC: u32 = bit(18);
pub const SAI_INTCR_RXOIE_MASK: u32 = bit(17);
pub const fn SAI_INTCR_RXOIE(x: u32) -> u32 {
    (x) << 17
}
pub const SAI_INTCR_TXUIC: u32 = bit(2);
pub const SAI_INTCR_TXUIE_MASK: u32 = bit(1);
pub const fn SAI_INTCR_TXUIE(x: u32) -> u32 {
    (x) << 1
}

/* INTSR Interrupt Status Register */
pub const SAI_INTSR_FSLOSTI_INA: u32 = 0;
pub const SAI_INTSR_FSLOSTI_ACT: u32 = bit(19);
pub const SAI_INTSR_FSERRI_INA: u32 = 0;
pub const SAI_INTSR_FSERRI_ACT: u32 = bit(18);
pub const SAI_INTSR_RXOI_INA: u32 = 0;
pub const SAI_INTSR_RXOI_ACT: u32 = bit(17);
pub const SAI_INTSR_TXUI_INA: u32 = 0;
pub const SAI_INTSR_TXUI_ACT: u32 = bit(1);

/* PATH_SEL: Transfer / Receive Path Select Register */
pub const fn SAI_RX_PATH_SHIFT(x: u32) -> u32 {
    8 + (x) * 2
}
pub const fn SAI_RX_PATH_MASK(x: u32) -> u32 {
    0x3 << SAI_RX_PATH_SHIFT(x)
}
pub const fn SAI_RX_PATH(x: u32, v: u32) -> u32 {
    (v) << SAI_RX_PATH_SHIFT(x)
}
pub const fn SAI_TX_PATH_SHIFT(x: u32) -> u32 {
    0 + (x) * 2
}
pub const fn SAI_TX_PATH_MASK(x: u32) -> u32 {
    0x3 << SAI_TX_PATH_SHIFT(x)
}
pub const fn SAI_TX_PATH(x: u32, v: u32) -> u32 {
    (v) << SAI_TX_PATH_SHIFT(x)
}

/* XSHIFT: Transfer / Receive Frame Sync Shift Register */

/*
 * TX-ONLY: LEFT Direction Feature
 * +------------------------------------------------+
 * | DATA LEFTx (step: 0.5 cycle) | FSYNC Edge      |
 * +------------------------------------------------+
 */
pub const SAI_XSHIFT_LEFT_MASK: u32 = genmask(25, 24);
pub const fn SAI_XSHIFT_LEFT(x: u32) -> u32 {
    (x) << 24
}
/*
 * +------------------------------------------------+
 * | FSYNC Edge | DATA RIGHTx (step: 0.5 cycle)     |
 * +------------------------------------------------+
 */
pub const SAI_XSHIFT_RIGHT_MASK: u32 = genmask(23, 0);
pub const fn SAI_XSHIFT_RIGHT(x: u32) -> u32 {
    x
}

/* XFIFOLR: Transfer / Receive FIFO Level Register */
pub const SAI_FIFOLR_XFL3_SHIFT: u32 = 18;
pub const SAI_FIFOLR_XFL3_MASK: u32 = genmask(23, 18);
pub const SAI_FIFOLR_XFL2_SHIFT: u32 = 12;
pub const SAI_FIFOLR_XFL2_MASK: u32 = genmask(17, 12);
pub const SAI_FIFOLR_XFL1_SHIFT: u32 = 6;
pub const SAI_FIFOLR_XFL1_MASK: u32 = genmask(11, 6);
pub const SAI_FIFOLR_XFL0_SHIFT: u32 = 0;
pub const SAI_FIFOLR_XFL0_MASK: u32 = genmask(5, 0);

/* STATUS Status Register (VERSION >= SAI_VER_2307) */
pub const SAI_STATUS_RX_IDLE: u32 = bit(3);
pub const SAI_STATUS_TX_IDLE: u32 = bit(2);
pub const SAI_STATUS_FS_IDLE: u32 = bit(1);

/* VERSION */
/*
 * Updates:
 *
 * VERSION >= SAI_VER_2311
 *
 * Support Frame Sync xN (FSXN)
 * Support Frame Sync Error Detect (FSE)
 * Support Frame Sync Lost Detect (FSLOST)
 * Support Force Clear (FCR)
 * Support SAIn-Chained (e.g. SAI0-CLK-DATA + SAI3-DATA +...)
 * Support Transmit Auto Gate Mode
 * Support Timing Shift Left for TX
 *
 * Optimize SCLK/FSYNC Timing Alignment
 *
 * VERSION >= SAI_VER_2403
 *
 * Support Loopback LR Select (e.g. L:MIC R:LP)
 *
 */
pub const SAI_VER_2307: u32 = 0x23073576;
pub const SAI_VER_2311: u32 = 0x23112118;
pub const SAI_VER_2401: u32 = 0x24013506;
pub const SAI_VER_2403: u32 = 0x24031103;

/* FS_TIMEOUT: Frame Sync Timeout Register */
pub const SAI_FS_TIMEOUT_VAL_MASK: u32 = genmask(31, 1);
pub const fn SAI_FS_TIMEOUT_VAL(x: u32) -> u32 {
    (x) << 1
}
pub const SAI_FS_TIMEOUT_EN_MASK: u32 = bit(0);
pub const fn SAI_FS_TIMEOUT_EN(x: u32) -> u32 {
    (x) << 0
}

/* SAI Registers */
pub const SAI_TXCR: u32 = 0x0000;
pub const SAI_FSCR: u32 = 0x0004;
pub const SAI_RXCR: u32 = 0x0008;
pub const SAI_MONO_CR: u32 = 0x000c;
pub const SAI_XFER: u32 = 0x0010;
pub const SAI_CLR: u32 = 0x0014;
pub const SAI_CKR: u32 = 0x0018;
pub const SAI_TXFIFOLR: u32 = 0x001c;
pub const SAI_RXFIFOLR: u32 = 0x0020;
pub const SAI_DMACR: u32 = 0x0024;
pub const SAI_INTCR: u32 = 0x0028;
pub const SAI_INTSR: u32 = 0x002c;
pub const SAI_TXDR: u32 = 0x0030;
pub const SAI_RXDR: u32 = 0x0034;
pub const SAI_PATH_SEL: u32 = 0x0038;
pub const SAI_TX_SLOT_MASK0: u32 = 0x003c;
pub const SAI_TX_SLOT_MASK1: u32 = 0x0040;
pub const SAI_TX_SLOT_MASK2: u32 = 0x0044;
pub const SAI_TX_SLOT_MASK3: u32 = 0x0048;
pub const SAI_RX_SLOT_MASK0: u32 = 0x004c;
pub const SAI_RX_SLOT_MASK1: u32 = 0x0050;
pub const SAI_RX_SLOT_MASK2: u32 = 0x0054;
pub const SAI_RX_SLOT_MASK3: u32 = 0x0058;
pub const SAI_TX_DATA_CNT: u32 = 0x005c;
pub const SAI_RX_DATA_CNT: u32 = 0x0060;
pub const SAI_TX_SHIFT: u32 = 0x0064;
pub const SAI_RX_SHIFT: u32 = 0x0068;
pub const SAI_STATUS: u32 = 0x006c;
pub const SAI_VERSION: u32 = 0x0070;
pub const SAI_FSXN: u32 = 0x0074;
pub const SAI_FS_TIMEOUT: u32 = 0x0078;
pub const SAI_LOOPBACK_LR: u32 = 0x007c;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
