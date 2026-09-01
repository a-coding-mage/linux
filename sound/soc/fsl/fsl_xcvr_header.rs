/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NXP XCVR ALSA SoC Digital Audio Interface (DAI) driver
 *
 * Copyright 2019 NXP
 */

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const FSL_XCVR_MODE_SPDIF: u32 = 0;
pub const FSL_XCVR_MODE_ARC: u32 = 1;
pub const FSL_XCVR_MODE_EARC: u32 = 2;

/* XCVR Registers */
pub const FSL_XCVR_REG_OFFSET: u32 = 0x800; /* regs offset */
pub const FSL_XCVR_FIFO_SIZE: u32 = 0x80; /* 128 */
pub const FSL_XCVR_FIFO_WMK_RX: u32 = FSL_XCVR_FIFO_SIZE >> 1; /* 64 */
pub const FSL_XCVR_FIFO_WMK_TX: u32 = FSL_XCVR_FIFO_SIZE >> 1; /* 64 */
pub const FSL_XCVR_MAXBURST_RX: u32 = FSL_XCVR_FIFO_WMK_RX >> 2; /* 16 */
pub const FSL_XCVR_MAXBURST_TX: u32 = FSL_XCVR_FIFO_WMK_TX >> 2; /* 16 */

pub const FSL_XCVR_RX_FIFO_ADDR: u32 = 0x0C00;
pub const FSL_XCVR_TX_FIFO_ADDR: u32 = 0x0E00;

pub const FSL_XCVR_VERSION: u32 = 0x00; /* Version */
pub const FSL_XCVR_EXT_CTRL: u32 = 0x10; /* Control */
pub const FSL_XCVR_EXT_STATUS: u32 = 0x20; /* Status */
pub const FSL_XCVR_EXT_IER0: u32 = 0x30; /* Interrupt en 0 */
pub const FSL_XCVR_EXT_IER1: u32 = 0x40; /* Interrupt en 1 */
pub const FSL_XCVR_EXT_ISR: u32 = 0x50; /* Interrupt status */
pub const FSL_XCVR_EXT_ISR_SET: u32 = 0x54; /* Interrupt status */
pub const FSL_XCVR_EXT_ISR_CLR: u32 = 0x58; /* Interrupt status */
pub const FSL_XCVR_EXT_ISR_TOG: u32 = 0x5C; /* Interrupt status */
pub const FSL_XCVR_IER: u32 = 0x70; /* Interrupt en for M0+ */
pub const FSL_XCVR_ISR: u32 = 0x80; /* Interrupt status */
pub const FSL_XCVR_ISR_SET: u32 = 0x84; /* Interrupt status set */
pub const FSL_XCVR_ISR_CLR: u32 = 0x88; /* Interrupt status clear */
pub const FSL_XCVR_ISR_TOG: u32 = 0x8C; /* Interrupt status toggle */
pub const FSL_XCVR_PHY_AI_CTRL: u32 = 0x90;
pub const FSL_XCVR_PHY_AI_CTRL_SET: u32 = 0x94;
pub const FSL_XCVR_PHY_AI_CTRL_CLR: u32 = 0x98;
pub const FSL_XCVR_PHY_AI_CTRL_TOG: u32 = 0x9C;
pub const FSL_XCVR_PHY_AI_WDATA: u32 = 0xA0;
pub const FSL_XCVR_PHY_AI_RDATA: u32 = 0xA4;
pub const FSL_XCVR_CLK_CTRL: u32 = 0xB0;
pub const FSL_XCVR_RX_DPTH_CTRL: u32 = 0x180; /* RX datapath ctrl reg */
pub const FSL_XCVR_RX_DPTH_CTRL_SET: u32 = 0x184;
pub const FSL_XCVR_RX_DPTH_CTRL_CLR: u32 = 0x188;
pub const FSL_XCVR_RX_DPTH_CTRL_TOG: u32 = 0x18c;

pub const FSL_XCVR_RX_CS_DATA_0: u32 = 0x190;
pub const FSL_XCVR_RX_CS_DATA_1: u32 = 0x194;
pub const FSL_XCVR_RX_CS_DATA_2: u32 = 0x198;
pub const FSL_XCVR_RX_CS_DATA_3: u32 = 0x19C;
pub const FSL_XCVR_RX_CS_DATA_4: u32 = 0x1A0;
pub const FSL_XCVR_RX_CS_DATA_5: u32 = 0x1A4;

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL: u32 = 0x1C0;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_SET: u32 = 0x1C4;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR: u32 = 0x1C8;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG: u32 = 0x1CC;

pub const FSL_XCVR_RX_DPTH_TSCR: u32 = 0x1D0;
pub const FSL_XCVR_RX_DPTH_BCR: u32 = 0x1D4;
pub const FSL_XCVR_RX_DPTH_BCTR: u32 = 0x1D8;
pub const FSL_XCVR_RX_DPTH_BCRR: u32 = 0x1DC;

pub const FSL_XCVR_TX_DPTH_CTRL: u32 = 0x220; /* TX datapath ctrl reg */
pub const FSL_XCVR_TX_DPTH_CTRL_SET: u32 = 0x224;
pub const FSL_XCVR_TX_DPTH_CTRL_CLR: u32 = 0x228;
pub const FSL_XCVR_TX_DPTH_CTRL_TOG: u32 = 0x22C;
pub const FSL_XCVR_TX_CS_DATA_0: u32 = 0x230; /* TX channel status bits regs */
pub const FSL_XCVR_TX_CS_DATA_1: u32 = 0x234;
pub const FSL_XCVR_TX_CS_DATA_2: u32 = 0x238;
pub const FSL_XCVR_TX_CS_DATA_3: u32 = 0x23C;
pub const FSL_XCVR_TX_CS_DATA_4: u32 = 0x240;
pub const FSL_XCVR_TX_CS_DATA_5: u32 = 0x244;

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL: u32 = 0x260;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_SET: u32 = 0x264;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR: u32 = 0x268;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG: u32 = 0x26C;

pub const FSL_XCVR_TX_DPTH_TSCR: u32 = 0x270;
pub const FSL_XCVR_TX_DPTH_BCR: u32 = 0x274;
pub const FSL_XCVR_TX_DPTH_BCTR: u32 = 0x278;
pub const FSL_XCVR_TX_DPTH_BCRR: u32 = 0x27C;

pub const FSL_XCVR_DEBUG_REG_0: u32 = 0x2E0;
pub const FSL_XCVR_DEBUG_REG_1: u32 = 0x2F0;

pub const FSL_XCVR_MAX_REG: u32 = FSL_XCVR_DEBUG_REG_1;

pub const FSL_XCVR_EXT_CTRL_CORE_RESET: u32 = BIT(31);

pub const FSL_XCVR_EXT_CTRL_RX_CMDC_RESET: u32 = BIT(30);
pub const FSL_XCVR_EXT_CTRL_TX_CMDC_RESET: u32 = BIT(29);
pub const fn FSL_XCVR_EXT_CTRL_CMDC_RESET(t: u32) -> u32 {
    if t != 0 { BIT(29) } else { BIT(30) }
}

pub const FSL_XCVR_EXT_CTRL_RX_DPTH_RESET: u32 = BIT(28);
pub const FSL_XCVR_EXT_CTRL_TX_DPTH_RESET: u32 = BIT(27);
pub const fn FSL_XCVR_EXT_CTRL_DPTH_RESET(t: u32) -> u32 {
    if t != 0 { BIT(27) } else { BIT(28) }
}

pub const FSL_XCVR_EXT_CTRL_TX_RX_MODE: u32 = BIT(26);
pub const FSL_XCVR_EXT_CTRL_DMA_RD_DIS: u32 = BIT(25);
pub const FSL_XCVR_EXT_CTRL_DMA_WR_DIS: u32 = BIT(24);
pub const fn FSL_XCVR_EXT_CTRL_DMA_DIS(t: u32) -> u32 {
    if t != 0 { BIT(24) } else { BIT(25) }
}
pub const FSL_XCVR_EXT_CTRL_SPDIF_MODE: u32 = BIT(23);
pub const FSL_XCVR_EXT_CTRL_SLEEP_MODE: u32 = BIT(21);

pub const FSL_XCVR_EXT_CTRL_TX_FWM_SHFT: u32 = 0;
pub const FSL_XCVR_EXT_CTRL_TX_FWM_MASK: u32 = GENMASK(6, 0);
pub const fn FSL_XCVR_EXT_CTRL_TX_FWM(i: u32) -> u32 {
    (i << FSL_XCVR_EXT_CTRL_TX_FWM_SHFT) & FSL_XCVR_EXT_CTRL_TX_FWM_MASK
}
pub const FSL_XCVR_EXT_CTRL_RX_FWM_SHFT: u32 = 8;
pub const FSL_XCVR_EXT_CTRL_RX_FWM_MASK: u32 = GENMASK(14, 8);
pub const fn FSL_XCVR_EXT_CTRL_RX_FWM(i: u32) -> u32 {
    (i << FSL_XCVR_EXT_CTRL_RX_FWM_SHFT) & FSL_XCVR_EXT_CTRL_RX_FWM_MASK
}
pub const FSL_XCVR_EXT_CTRL_PAGE_SHFT: u32 = 16;
pub const FSL_XCVR_EXT_CTRL_PAGE_MASK: u32 = GENMASK(19, 16);
pub const fn FSL_XCVR_EXT_CTRL_PAGE(i: u32) -> u32 {
    (i << FSL_XCVR_EXT_CTRL_PAGE_SHFT) & FSL_XCVR_EXT_CTRL_PAGE_MASK
}

pub const FSL_XCVR_EXT_STUS_NT_FIFO_ENTR: u32 = GENMASK(7, 0);
pub const FSL_XCVR_EXT_STUS_NR_FIFO_ENTR: u32 = GENMASK(15, 8);
pub const FSL_XCVR_EXT_STUS_CM0_SLEEPING: u32 = BIT(16);
pub const FSL_XCVR_EXT_STUS_CM0_DEEP_SLP: u32 = BIT(17);
pub const FSL_XCVR_EXT_STUS_CM0_SLP_HACK: u32 = BIT(18);
pub const FSL_XCVR_EXT_STUS_RX_CMDC_RSTO: u32 = BIT(23);
pub const FSL_XCVR_EXT_STUS_TX_CMDC_RSTO: u32 = BIT(24);
pub const FSL_XCVR_EXT_STUS_RX_CMDC_COTO: u32 = BIT(25);
pub const FSL_XCVR_EXT_STUS_TX_CMDC_COTO: u32 = BIT(26);
pub const FSL_XCVR_EXT_STUS_HB_STATUS: u32 = BIT(27);
pub const FSL_XCVR_EXT_STUS_NEW_UD4_REC: u32 = BIT(28);
pub const FSL_XCVR_EXT_STUS_NEW_UD5_REC: u32 = BIT(29);
pub const FSL_XCVR_EXT_STUS_NEW_UD6_REC: u32 = BIT(30);
pub const FSL_XCVR_EXT_STUS_HPD_INPUT: u32 = BIT(31);

pub const FSL_XCVR_IRQ_NEW_CS: u32 = BIT(0);
pub const FSL_XCVR_IRQ_NEW_UD: u32 = BIT(1);
pub const FSL_XCVR_IRQ_MUTE: u32 = BIT(2);
pub const FSL_XCVR_IRQ_CMDC_RESP_TO: u32 = BIT(3);
pub const FSL_XCVR_IRQ_ECC_ERR: u32 = BIT(4);
pub const FSL_XCVR_IRQ_PREAMBLE_MISMATCH: u32 = BIT(5);
pub const FSL_XCVR_IRQ_FIFO_UOFL_ERR: u32 = BIT(6);
pub const FSL_XCVR_IRQ_HOST_WAKEUP: u32 = BIT(7);
pub const FSL_XCVR_IRQ_HOST_OHPD: u32 = BIT(8);
pub const FSL_XCVR_IRQ_DMAC_NO_DATA_REC: u32 = BIT(9);
pub const FSL_XCVR_IRQ_DMAC_FMT_CHG_DET: u32 = BIT(10);
pub const FSL_XCVR_IRQ_HB_STATE_CHG: u32 = BIT(11);
pub const FSL_XCVR_IRQ_CMDC_STATUS_UPD: u32 = BIT(12);
pub const FSL_XCVR_IRQ_TEMP_UPD: u32 = BIT(13);
pub const FSL_XCVR_IRQ_DMA_RD_REQ: u32 = BIT(14);
pub const FSL_XCVR_IRQ_DMA_WR_REQ: u32 = BIT(15);
pub const FSL_XCVR_IRQ_DMAC_BME_BIT_ERR: u32 = BIT(16);
pub const FSL_XCVR_IRQ_PREAMBLE_MATCH: u32 = BIT(17);
pub const FSL_XCVR_IRQ_M_W_PRE_MISMATCH: u32 = BIT(18);
pub const FSL_XCVR_IRQ_B_PRE_MISMATCH: u32 = BIT(19);
pub const FSL_XCVR_IRQ_UNEXP_PRE_REC: u32 = BIT(20);
pub const FSL_XCVR_IRQ_ARC_MODE: u32 = BIT(21);
pub const FSL_XCVR_IRQ_CH_UD_OFLOW: u32 = BIT(22);
pub const FSL_XCVR_IRQ_EARC_ALL: u32 = FSL_XCVR_IRQ_NEW_CS
    | FSL_XCVR_IRQ_NEW_UD
    | FSL_XCVR_IRQ_MUTE
    | FSL_XCVR_IRQ_FIFO_UOFL_ERR
    | FSL_XCVR_IRQ_HOST_WAKEUP
    | FSL_XCVR_IRQ_CMDC_STATUS_UPD
    | FSL_XCVR_IRQ_B_PRE_MISMATCH
    | FSL_XCVR_IRQ_M_W_PRE_MISMATCH
    | FSL_XCVR_IRQ_PREAMBLE_MISMATCH
    | FSL_XCVR_IRQ_UNEXP_PRE_REC
    | FSL_XCVR_IRQ_ARC_MODE;

pub const FSL_XCVR_ISR_CMDC_TX_EN: u32 = BIT(3);
pub const FSL_XCVR_ISR_HPD_TGL: u32 = BIT(15);
pub const FSL_XCVR_ISR_DMAC_SPARE_INT: u32 = BIT(19);
pub const FSL_XCVR_ISR_SET_SPDIF_RX_INT: u32 = BIT(20);
pub const FSL_XCVR_ISR_SET_SPDIF_TX_INT: u32 = BIT(21);
pub const fn FSL_XCVR_ISR_SET_SPDIF_MODE(t: u32) -> u32 {
    if t != 0 { BIT(21) } else { BIT(20) }
}
pub const FSL_XCVR_ISR_SET_ARC_CM_INT: u32 = BIT(22);
pub const FSL_XCVR_ISR_SET_ARC_SE_INT: u32 = BIT(23);

pub const FSL_XCVR_PHY_AI_ADDR_MASK: u32 = GENMASK(7, 0);
pub const FSL_XCVR_PHY_AI_RESETN: u32 = BIT(15);
pub const FSL_XCVR_PHY_AI_TOG_PLL: u32 = BIT(24);
pub const FSL_XCVR_PHY_AI_TOG_DONE_PLL: u32 = BIT(25);
pub const FSL_XCVR_PHY_AI_TOG_PHY: u32 = BIT(26);
pub const FSL_XCVR_PHY_AI_TOG_DONE_PHY: u32 = BIT(27);
pub const FSL_XCVR_PHY_AI_RW_MASK: u32 = BIT(31);

pub const FSL_XCVR_RX_DPTH_CTRL_PAPB_FIFO_STATUS: u32 = BIT(0);
pub const FSL_XCVR_RX_DPTH_CTRL_DIS_PRE_ERR_CHK: u32 = BIT(1);
pub const FSL_XCVR_RX_DPTH_CTRL_DIS_NOD_REC_CHK: u32 = BIT(2);
pub const FSL_XCVR_RX_DPTH_CTRL_ECC_VUC_BIT_CHK: u32 = BIT(3);
pub const FSL_XCVR_RX_DPTH_CTRL_EN_CMP_PAR_CALC: u32 = BIT(4);
pub const FSL_XCVR_RX_DPTH_CTRL_RST_PKT_CNT_FIFO: u32 = BIT(5);
pub const FSL_XCVR_RX_DPTH_CTRL_STORE_FMT: u32 = BIT(6);
pub const FSL_XCVR_RX_DPTH_CTRL_EN_PAR_CALC: u32 = BIT(7);
pub const FSL_XCVR_RX_DPTH_CTRL_UDR: u32 = BIT(8);
pub const FSL_XCVR_RX_DPTH_CTRL_CSR: u32 = BIT(9);
pub const FSL_XCVR_RX_DPTH_CTRL_UDA: u32 = BIT(10);
pub const FSL_XCVR_RX_DPTH_CTRL_CSA: u32 = BIT(11);
pub const FSL_XCVR_RX_DPTH_CTRL_CLR_RX_FIFO: u32 = BIT(12);
pub const FSL_XCVR_RX_DPTH_CTRL_DIS_B_PRE_ERR_CHK: u32 = BIT(13);
pub const FSL_XCVR_RX_DPTH_CTRL_PABS: u32 = BIT(19);
pub const FSL_XCVR_RX_DPTH_CTRL_DTS_CDS: u32 = BIT(20);
pub const FSL_XCVR_RX_DPTH_CTRL_BLKC: u32 = BIT(21);
pub const FSL_XCVR_RX_DPTH_CTRL_MUTE_CTRL: u32 = BIT(22);
pub const FSL_XCVR_RX_DPTH_CTRL_MUTE_MODE: u32 = BIT(23);
pub const FSL_XCVR_RX_DPTH_CTRL_FMT_CHG_CTRL: u32 = BIT(24);
pub const FSL_XCVR_RX_DPTH_CTRL_FMT_CHG_MODE: u32 = BIT(25);
pub const FSL_XCVR_RX_DPTH_CTRL_LAYB_CTRL: u32 = BIT(26);
pub const FSL_XCVR_RX_DPTH_CTRL_LAYB_MODE: u32 = BIT(27);
pub const FSL_XCVR_RX_DPTH_CTRL_PRC: u32 = BIT(28);
pub const FSL_XCVR_RX_DPTH_CTRL_COMP: u32 = BIT(29);
pub const FSL_XCVR_RX_DPTH_CTRL_FSM: u32 = GENMASK(31, 30);

pub const FSL_XCVR_TX_DPTH_CTRL_CS_ACK: u32 = BIT(0);
pub const FSL_XCVR_TX_DPTH_CTRL_UD_ACK: u32 = BIT(1);
pub const FSL_XCVR_TX_DPTH_CTRL_CS_MOD: u32 = BIT(2);
pub const FSL_XCVR_TX_DPTH_CTRL_UD_MOD: u32 = BIT(3);
pub const FSL_XCVR_TX_DPTH_CTRL_VLD_MOD: u32 = BIT(4);
pub const FSL_XCVR_TX_DPTH_CTRL_FRM_VLD: u32 = BIT(5);
pub const FSL_XCVR_TX_DPTH_CTRL_EN_PARITY: u32 = BIT(6);
pub const FSL_XCVR_TX_DPTH_CTRL_EN_PREAMBLE: u32 = BIT(7);
pub const FSL_XCVR_TX_DPTH_CTRL_EN_ECC_INTER: u32 = BIT(8);
pub const FSL_XCVR_TX_DPTH_CTRL_BYPASS_FEM: u32 = BIT(10);
pub const FSL_XCVR_TX_DPTH_CTRL_FRM_FMT: u32 = BIT(11);
pub const FSL_XCVR_TX_DPTH_CTRL_STRT_DATA_TX: u32 = BIT(14);
pub const FSL_XCVR_TX_DPTH_CTRL_ADD_CYC_TX_OE_STR: u32 = BIT(15);
pub const FSL_XCVR_TX_DPTH_CTRL_ADD_CYC_TX_OE_END: u32 = BIT(16);
pub const FSL_XCVR_TX_DPTH_CTRL_CLK_RATIO: u32 = BIT(29);
pub const FSL_XCVR_TX_DPTH_CTRL_TM_NO_PRE_BME: u32 = GENMASK(31, 30);

pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSEN_SHIFT: u32 = 0;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSEN: u32 = BIT(0);
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSINC_SHIFT: u32 = 1;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_TSINC: u32 = BIT(1);
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RBC_SHIFT: u32 = 8;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RBC: u32 = BIT(8);
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RTSC_SHIFT: u32 = 9;
pub const FSL_XCVR_RX_DPTH_CNTR_CTRL_RTSC: u32 = BIT(9);

pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSEN_SHIFT: u32 = 0;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSEN: u32 = BIT(0);
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSINC_SHIFT: u32 = 1;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_TSINC: u32 = BIT(1);
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RBC_SHIFT: u32 = 8;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RBC: u32 = BIT(8);
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RTSC_SHIFT: u32 = 9;
pub const FSL_XCVR_TX_DPTH_CNTR_CTRL_RTSC: u32 = BIT(9);

pub const FSL_XCVR_PHY_AI_CTRL_AI_RESETN: u32 = BIT(15);
pub const FSL_XCVR_PHY_AI_CTRL_AI_RWB: u32 = BIT(31);

pub const FSL_XCVR_PLL_CTRL0: u32 = 0x00;
pub const FSL_XCVR_PLL_CTRL0_SET: u32 = 0x04;
pub const FSL_XCVR_PLL_CTRL0_CLR: u32 = 0x08;
pub const FSL_XCVR_PLL_NUM: u32 = 0x20;
pub const FSL_XCVR_PLL_DEN: u32 = 0x30;
pub const FSL_XCVR_PLL_PDIV: u32 = 0x40;
pub const FSL_XCVR_PLL_BANDGAP: u32 = 0x50;
pub const FSL_XCVR_PLL_BANDGAP_SET: u32 = 0x54;
pub const FSL_XCVR_PLL_STAT0: u32 = 0x60;
pub const FSL_XCVR_PLL_STAT0_TOG: u32 = 0x6c;

pub const FSL_XCVR_PHY_CTRL: u32 = 0x00;
pub const FSL_XCVR_PHY_CTRL_SET: u32 = 0x04;
pub const FSL_XCVR_PHY_CTRL_CLR: u32 = 0x08;
pub const FSL_XCVR_PHY_CTRL_TOG: u32 = 0x0c;
pub const FSL_XCVR_PHY_STATUS: u32 = 0x10;
pub const FSL_XCVR_PHY_ANALOG_TRIM: u32 = 0x20;
pub const FSL_XCVR_PHY_SLEW_RATE_TRIM: u32 = 0x30;
pub const FSL_XCVR_PHY_DATA_TEST_DELAY: u32 = 0x40;
pub const FSL_XCVR_PHY_TEST_CTRL: u32 = 0x50;
pub const FSL_XCVR_PHY_DIFF_CDR_CTRL: u32 = 0x60;
pub const FSL_XCVR_PHY_CTRL2: u32 = 0x70;
pub const FSL_XCVR_PHY_CTRL2_SET: u32 = 0x74;
pub const FSL_XCVR_PHY_CTRL2_CLR: u32 = 0x78;
pub const FSL_XCVR_PHY_CTRL2_TOG: u32 = 0x7c;

pub const FSL_XCVR_PLL_BANDGAP_EN_VBG: u32 = BIT(0);
pub const FSL_XCVR_PLL_CTRL0_HROFF: u32 = BIT(13);
pub const FSL_XCVR_PLL_CTRL0_PWP: u32 = BIT(14);
pub const FSL_XCVR_PLL_CTRL0_CM0_EN: u32 = BIT(24);
pub const FSL_XCVR_PLL_CTRL0_CM1_EN: u32 = BIT(25);
pub const FSL_XCVR_PLL_CTRL0_CM2_EN: u32 = BIT(26);
pub const fn FSL_XCVR_PLL_PDIVx(v: u32, i: u32) -> u32 {
    (v & 0x7) << (4 * i)
}

pub const FSL_XCVR_PHY_CTRL_PHY_EN: u32 = BIT(0);
pub const FSL_XCVR_PHY_CTRL_RX_CM_EN: u32 = BIT(1);
pub const FSL_XCVR_PHY_CTRL_TSDIFF_OE: u32 = BIT(5);
pub const FSL_XCVR_PHY_CTRL_SPDIF_EN: u32 = BIT(8);
pub const FSL_XCVR_PHY_CTRL_ARC_MODE_SE_EN: u32 = BIT(9);
pub const FSL_XCVR_PHY_CTRL_ARC_MODE_CM_EN: u32 = BIT(10);
pub const FSL_XCVR_PHY_CTRL_TX_CLK_MASK: u32 = GENMASK(26, 25);
pub const FSL_XCVR_PHY_CTRL_TX_CLK_HDMI_SS: u32 = BIT(25);
pub const FSL_XCVR_PHY_CTRL_TX_CLK_AUD_SS: u32 = BIT(26);
pub const FSL_XCVR_PHY_CTRL2_EARC_TXMS: u32 = BIT(14);

pub const FSL_XCVR_CS_DATA_0_FS_MASK: u32 = GENMASK(31, 24);
pub const FSL_XCVR_CS_DATA_0_FS_32000: u32 = 0x3000000;
pub const FSL_XCVR_CS_DATA_0_FS_44100: u32 = 0x0000000;
pub const FSL_XCVR_CS_DATA_0_FS_48000: u32 = 0x2000000;
pub const FSL_XCVR_CS_DATA_0_FS_64000: u32 = 0xB000000;
pub const FSL_XCVR_CS_DATA_0_FS_88200: u32 = 0x8000000;
pub const FSL_XCVR_CS_DATA_0_FS_96000: u32 = 0xA000000;
pub const FSL_XCVR_CS_DATA_0_FS_176400: u32 = 0xC000000;
pub const FSL_XCVR_CS_DATA_0_FS_192000: u32 = 0xE000000;

pub const FSL_XCVR_CS_DATA_0_CH_MASK: u32 = 0x3A;
pub const FSL_XCVR_CS_DATA_0_CH_U2LPCM: u32 = 0x00;
pub const FSL_XCVR_CS_DATA_0_CH_UMLPCM: u32 = 0x20;
pub const FSL_XCVR_CS_DATA_0_CH_U1BAUD: u32 = 0x30;

pub const FSL_XCVR_CS_DATA_1_CH_MASK: u32 = 0xF000;
pub const FSL_XCVR_CS_DATA_1_CH_2: u32 = 0x0000;
pub const FSL_XCVR_CS_DATA_1_CH_8: u32 = 0x7000;
pub const FSL_XCVR_CS_DATA_1_CH_16: u32 = 0xB000;
pub const FSL_XCVR_CS_DATA_1_CH_32: u32 = 0x3000;

/* Data memory structures */
pub const FSL_XCVR_RX_CS_CTRL_0: u32 = 0x20; /* First  RX CS control register */
pub const FSL_XCVR_RX_CS_CTRL_1: u32 = 0x24; /* Second RX CS control register */
pub const FSL_XCVR_RX_CS_BUFF_0: u32 = 0x80; /* First  RX CS buffer */
pub const FSL_XCVR_RX_CS_BUFF_1: u32 = 0xA0; /* Second RX CS buffer */
pub const FSL_XCVR_CAP_DATA_STR: u32 = 0x300; /* Capabilities data structure */

/* GP PLL Registers */
pub const FSL_XCVR_GP_PLL_CTRL: u32 = 0x00;
pub const FSL_XCVR_GP_PLL_CTRL_SET: u32 = 0x04;
pub const FSL_XCVR_GP_PLL_CTRL_CLR: u32 = 0x08;
pub const FSL_XCVR_GP_PLL_CTRL_TOG: u32 = 0x0C;
pub const FSL_XCVR_GP_PLL_ANA_PRG: u32 = 0x10;
pub const FSL_XCVR_GP_PLL_ANA_PRG_SET: u32 = 0x14;
pub const FSL_XCVR_GP_PLL_ANA_PRG_CLR: u32 = 0x18;
pub const FSL_XCVR_GP_PLL_ANA_PRG_TOG: u32 = 0x1C;
pub const FSL_XCVR_GP_PLL_TEST: u32 = 0x20;
pub const FSL_XCVR_GP_PLL_TEST_SET: u32 = 0x24;
pub const FSL_XCVR_GP_PLL_TEST_CLR: u32 = 0x28;
pub const FSL_XCVR_GP_PLL_TEST_TOG: u32 = 0x2C;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM: u32 = 0x30;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_SET: u32 = 0x34;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_CLR: u32 = 0x38;
pub const FSL_XCVR_GP_PLL_SPREAD_SPECTRUM_TOG: u32 = 0x3C;
pub const FSL_XCVR_GP_PLL_NUMERATOR: u32 = 0x40;
pub const FSL_XCVR_GP_PLL_NUMERATOR_SET: u32 = 0x44;
pub const FSL_XCVR_GP_PLL_NUMERATOR_CLR: u32 = 0x48;
pub const FSL_XCVR_GP_PLL_NUMERATOR_TOG: u32 = 0x4C;
pub const FSL_XCVR_GP_PLL_DENOMINATOR: u32 = 0x50;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_SET: u32 = 0x54;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_CLR: u32 = 0x58;
pub const FSL_XCVR_GP_PLL_DENOMINATOR_TOG: u32 = 0x5C;
pub const FSL_XCVR_GP_PLL_DIV: u32 = 0x60;
pub const FSL_XCVR_GP_PLL_DIV_SET: u32 = 0x64;
pub const FSL_XCVR_GP_PLL_DIV_CLR: u32 = 0x68;
pub const FSL_XCVR_GP_PLL_DIV_TOG: u32 = 0x6C;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0: u32 = 0x70;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_SET: u32 = 0x74;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_CLR: u32 = 0x78;
pub const FSL_XCVR_GP_PLL_DFS_CTRL0_TOG: u32 = 0x7C;
pub const FSL_XCVR_GP_PLL_DFS_DIV0: u32 = 0x80;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_SET: u32 = 0x84;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_CLR: u32 = 0x88;
pub const FSL_XCVR_GP_PLL_DFS_DIV0_TOG: u32 = 0x8C;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1: u32 = 0x90;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_SET: u32 = 0x94;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_CLR: u32 = 0x98;
pub const FSL_XCVR_GP_PLL_DFS_CTRL1_TOG: u32 = 0x9C;
pub const FSL_XCVR_GP_PLL_DFS_DIV1: u32 = 0xA0;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_SET: u32 = 0xA4;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_CLR: u32 = 0xA8;
pub const FSL_XCVR_GP_PLL_DFS_DIV1_TOG: u32 = 0xAC;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2: u32 = 0xB0;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_SET: u32 = 0xB4;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_CLR: u32 = 0xB8;
pub const FSL_XCVR_GP_PLL_DFS_CTRL2_TOG: u32 = 0xBC;
pub const FSL_XCVR_GP_PLL_DFS_DIV2: u32 = 0xC0;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_SET: u32 = 0xC4;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_CLR: u32 = 0xC8;
pub const FSL_XCVR_GP_PLL_DFS_DIV2_TOG: u32 = 0xCC;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3: u32 = 0xD0;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_SET: u32 = 0xD4;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_CLR: u32 = 0xD8;
pub const FSL_XCVR_GP_PLL_DFS_CTRL3_TOG: u32 = 0xDC;
pub const FSL_XCVR_GP_PLL_DFS_DIV3: u32 = 0xE0;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_SET: u32 = 0xE4;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_CLR: u32 = 0xE8;
pub const FSL_XCVR_GP_PLL_DFS_DIV3_TOG: u32 = 0xEC;
pub const FSL_XCVR_GP_PLL_STATUS: u32 = 0xF0;
pub const FSL_XCVR_GP_PLL_STATUS_SET: u32 = 0xF4;
pub const FSL_XCVR_GP_PLL_STATUS_CLR: u32 = 0xF8;
pub const FSL_XCVR_GP_PLL_STATUS_TOG: u32 = 0xFC;

/* GP PLL Control Register */
pub const FSL_XCVR_GP_PLL_CTRL_LBYPASS: u32 = BIT(31);
pub const FSL_XCVR_GP_PLL_CTRL_HCS: u32 = BIT(16);
pub const FSL_XCVR_GP_PLL_CTRL_MSD: u32 = BIT(12);
pub const FSL_XCVR_GP_PLL_CTRL_DITHER_EN3: u32 = BIT(11);
pub const FSL_XCVR_GP_PLL_CTRL_DITHER_EN2: u32 = BIT(10);
pub const FSL_XCVR_GP_PLL_CTRL_DITHER_EN1: u32 = BIT(9);
pub const FSL_XCVR_GP_PLL_CTRL_SPREADCTL: u32 = BIT(8);
pub const FSL_XCVR_GP_PLL_CTRL_CLKMUX_BYPASS: u32 = BIT(2);
pub const FSL_XCVR_GP_PLL_CTRL_CLKMUX_EN: u32 = BIT(1);
pub const FSL_XCVR_GP_PLL_CTRL_POWERUP: u32 = BIT(0);

/* GP PLL Numerator Register */
pub const FSL_XCVR_GP_PLL_NUMERATOR_MFN_SHIFT: u32 = 2;
pub const FSL_XCVR_GP_PLL_NUMERATOR_MFN: u32 = GENMASK(31, 2);

/* GP PLL Denominator Register */
pub const FSL_XCVR_GP_PLL_DENOMINATOR_MFD: u32 = GENMASK(29, 0);

/* GP PLL Dividers Register */
pub const FSL_XCVR_GP_PLL_DIV_MFI_SHIFT: u32 = 16;
pub const FSL_XCVR_GP_PLL_DIV_MFI: u32 = GENMASK(24, 16);
pub const FSL_XCVR_GP_PLL_DIV_RDIV: u32 = GENMASK(15, 13);
pub const FSL_XCVR_GP_PLL_DIV_ODIV: u32 = GENMASK(7, 0);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
