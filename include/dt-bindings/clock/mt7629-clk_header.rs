/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 MediaTek Inc.
 */

/* TOPCKGEN */
pub const CLK_TOP_TO_U2_PHY: u32 = 0;
pub const CLK_TOP_TO_U2_PHY_1P: u32 = 1;
pub const CLK_TOP_PCIE0_PIPE_EN: u32 = 2;
pub const CLK_TOP_PCIE1_PIPE_EN: u32 = 3;
pub const CLK_TOP_SSUSB_TX250M: u32 = 4;
pub const CLK_TOP_SSUSB_EQ_RX250M: u32 = 5;
pub const CLK_TOP_SSUSB_CDR_REF: u32 = 6;
pub const CLK_TOP_SSUSB_CDR_FB: u32 = 7;
pub const CLK_TOP_SATA_ASIC: u32 = 8;
pub const CLK_TOP_SATA_RBC: u32 = 9;
pub const CLK_TOP_TO_USB3_SYS: u32 = 10;
pub const CLK_TOP_P1_1MHZ: u32 = 11;
pub const CLK_TOP_4MHZ: u32 = 12;
pub const CLK_TOP_P0_1MHZ: u32 = 13;
pub const CLK_TOP_ETH_500M: u32 = 14;
pub const CLK_TOP_TXCLK_SRC_PRE: u32 = 15;
pub const CLK_TOP_RTC: u32 = 16;
pub const CLK_TOP_PWM_QTR_26M: u32 = 17;
pub const CLK_TOP_CPUM_TCK_IN: u32 = 18;
pub const CLK_TOP_TO_USB3_DA_TOP: u32 = 19;
pub const CLK_TOP_MEMPLL: u32 = 20;
pub const CLK_TOP_DMPLL: u32 = 21;
pub const CLK_TOP_DMPLL_D4: u32 = 22;
pub const CLK_TOP_DMPLL_D8: u32 = 23;
pub const CLK_TOP_SYSPLL_D2: u32 = 24;
pub const CLK_TOP_SYSPLL1_D2: u32 = 25;
pub const CLK_TOP_SYSPLL1_D4: u32 = 26;
pub const CLK_TOP_SYSPLL1_D8: u32 = 27;
pub const CLK_TOP_SYSPLL1_D16: u32 = 28;
pub const CLK_TOP_SYSPLL2_D2: u32 = 29;
pub const CLK_TOP_SYSPLL2_D4: u32 = 30;
pub const CLK_TOP_SYSPLL2_D8: u32 = 31;
pub const CLK_TOP_SYSPLL_D5: u32 = 32;
pub const CLK_TOP_SYSPLL3_D2: u32 = 33;
pub const CLK_TOP_SYSPLL3_D4: u32 = 34;
pub const CLK_TOP_SYSPLL_D7: u32 = 35;
pub const CLK_TOP_SYSPLL4_D2: u32 = 36;
pub const CLK_TOP_SYSPLL4_D4: u32 = 37;
pub const CLK_TOP_SYSPLL4_D16: u32 = 38;
pub const CLK_TOP_UNIVPLL: u32 = 39;
pub const CLK_TOP_UNIVPLL1_D2: u32 = 40;
pub const CLK_TOP_UNIVPLL1_D4: u32 = 41;
pub const CLK_TOP_UNIVPLL1_D8: u32 = 42;
pub const CLK_TOP_UNIVPLL_D3: u32 = 43;
pub const CLK_TOP_UNIVPLL2_D2: u32 = 44;
pub const CLK_TOP_UNIVPLL2_D4: u32 = 45;
pub const CLK_TOP_UNIVPLL2_D8: u32 = 46;
pub const CLK_TOP_UNIVPLL2_D16: u32 = 47;
pub const CLK_TOP_UNIVPLL_D5: u32 = 48;
pub const CLK_TOP_UNIVPLL3_D2: u32 = 49;
pub const CLK_TOP_UNIVPLL3_D4: u32 = 50;
pub const CLK_TOP_UNIVPLL3_D16: u32 = 51;
pub const CLK_TOP_UNIVPLL_D7: u32 = 52;
pub const CLK_TOP_UNIVPLL_D80_D4: u32 = 53;
pub const CLK_TOP_UNIV48M: u32 = 54;
pub const CLK_TOP_SGMIIPLL_D2: u32 = 55;
pub const CLK_TOP_CLKXTAL_D4: u32 = 56;
pub const CLK_TOP_HD_FAXI: u32 = 57;
pub const CLK_TOP_FAXI: u32 = 58;
pub const CLK_TOP_F_FAUD_INTBUS: u32 = 59;
pub const CLK_TOP_AP2WBHIF_HCLK: u32 = 60;
pub const CLK_TOP_10M_INFRAO: u32 = 61;
pub const CLK_TOP_MSDC30_1: u32 = 62;
pub const CLK_TOP_SPI: u32 = 63;
pub const CLK_TOP_SF: u32 = 64;
pub const CLK_TOP_FLASH: u32 = 65;
pub const CLK_TOP_TO_USB3_REF: u32 = 66;
pub const CLK_TOP_TO_USB3_MCU: u32 = 67;
pub const CLK_TOP_TO_USB3_DMA: u32 = 68;
pub const CLK_TOP_FROM_TOP_AHB: u32 = 69;
pub const CLK_TOP_FROM_TOP_AXI: u32 = 70;
pub const CLK_TOP_PCIE1_MAC_EN: u32 = 71;
pub const CLK_TOP_PCIE0_MAC_EN: u32 = 72;
pub const CLK_TOP_AXI_SEL: u32 = 73;
pub const CLK_TOP_MEM_SEL: u32 = 74;
pub const CLK_TOP_DDRPHYCFG_SEL: u32 = 75;
pub const CLK_TOP_ETH_SEL: u32 = 76;
pub const CLK_TOP_PWM_SEL: u32 = 77;
pub const CLK_TOP_F10M_REF_SEL: u32 = 78;
pub const CLK_TOP_NFI_INFRA_SEL: u32 = 79;
pub const CLK_TOP_FLASH_SEL: u32 = 80;
pub const CLK_TOP_UART_SEL: u32 = 81;
pub const CLK_TOP_SPI0_SEL: u32 = 82;
pub const CLK_TOP_SPI1_SEL: u32 = 83;
pub const CLK_TOP_MSDC50_0_SEL: u32 = 84;
pub const CLK_TOP_MSDC30_0_SEL: u32 = 85;
pub const CLK_TOP_MSDC30_1_SEL: u32 = 86;
pub const CLK_TOP_AP2WBMCU_SEL: u32 = 87;
pub const CLK_TOP_AP2WBHIF_SEL: u32 = 88;
pub const CLK_TOP_AUDIO_SEL: u32 = 89;
pub const CLK_TOP_AUD_INTBUS_SEL: u32 = 90;
pub const CLK_TOP_PMICSPI_SEL: u32 = 91;
pub const CLK_TOP_SCP_SEL: u32 = 92;
pub const CLK_TOP_ATB_SEL: u32 = 93;
pub const CLK_TOP_HIF_SEL: u32 = 94;
pub const CLK_TOP_SATA_SEL: u32 = 95;
pub const CLK_TOP_U2_SEL: u32 = 96;
pub const CLK_TOP_AUD1_SEL: u32 = 97;
pub const CLK_TOP_AUD2_SEL: u32 = 98;
pub const CLK_TOP_IRRX_SEL: u32 = 99;
pub const CLK_TOP_IRTX_SEL: u32 = 100;
pub const CLK_TOP_SATA_MCU_SEL: u32 = 101;
pub const CLK_TOP_PCIE0_MCU_SEL: u32 = 102;
pub const CLK_TOP_PCIE1_MCU_SEL: u32 = 103;
pub const CLK_TOP_SSUSB_MCU_SEL: u32 = 104;
pub const CLK_TOP_CRYPTO_SEL: u32 = 105;
pub const CLK_TOP_SGMII_REF_1_SEL: u32 = 106;
pub const CLK_TOP_10M_SEL: u32 = 107;
pub const CLK_TOP_NR_CLK: u32 = 108;

/* INFRACFG */
pub const CLK_INFRA_MUX1_SEL: u32 = 0;
pub const CLK_INFRA_DBGCLK_PD: u32 = 1;
pub const CLK_INFRA_TRNG_PD: u32 = 2;
pub const CLK_INFRA_DEVAPC_PD: u32 = 3;
pub const CLK_INFRA_APXGPT_PD: u32 = 4;
pub const CLK_INFRA_SEJ_PD: u32 = 5;
pub const CLK_INFRA_NR_CLK: u32 = 6;

/* PERICFG */
pub const CLK_PERIBUS_SEL: u32 = 0;
pub const CLK_PERI_PWM1_PD: u32 = 1;
pub const CLK_PERI_PWM2_PD: u32 = 2;
pub const CLK_PERI_PWM3_PD: u32 = 3;
pub const CLK_PERI_PWM4_PD: u32 = 4;
pub const CLK_PERI_PWM5_PD: u32 = 5;
pub const CLK_PERI_PWM6_PD: u32 = 6;
pub const CLK_PERI_PWM7_PD: u32 = 7;
pub const CLK_PERI_PWM_PD: u32 = 8;
pub const CLK_PERI_AP_DMA_PD: u32 = 9;
pub const CLK_PERI_MSDC30_1_PD: u32 = 10;
pub const CLK_PERI_UART0_PD: u32 = 11;
pub const CLK_PERI_UART1_PD: u32 = 12;
pub const CLK_PERI_UART2_PD: u32 = 13;
pub const CLK_PERI_UART3_PD: u32 = 14;
pub const CLK_PERI_BTIF_PD: u32 = 15;
pub const CLK_PERI_I2C0_PD: u32 = 16;
pub const CLK_PERI_SPI0_PD: u32 = 17;
pub const CLK_PERI_SNFI_PD: u32 = 18;
pub const CLK_PERI_NFI_PD: u32 = 19;
pub const CLK_PERI_NFIECC_PD: u32 = 20;
pub const CLK_PERI_FLASH_PD: u32 = 21;
pub const CLK_PERI_NR_CLK: u32 = 22;

/* APMIXEDSYS */
pub const CLK_APMIXED_ARMPLL: u32 = 0;
pub const CLK_APMIXED_MAINPLL: u32 = 1;
pub const CLK_APMIXED_UNIV2PLL: u32 = 2;
pub const CLK_APMIXED_ETH1PLL: u32 = 3;
pub const CLK_APMIXED_ETH2PLL: u32 = 4;
pub const CLK_APMIXED_SGMIPLL: u32 = 5;
pub const CLK_APMIXED_MAIN_CORE_EN: u32 = 6;
pub const CLK_APMIXED_NR_CLK: u32 = 7;

/* SSUSBSYS */
pub const CLK_SSUSB_U2_PHY_1P_EN: u32 = 0;
pub const CLK_SSUSB_U2_PHY_EN: u32 = 1;
pub const CLK_SSUSB_REF_EN: u32 = 2;
pub const CLK_SSUSB_SYS_EN: u32 = 3;
pub const CLK_SSUSB_MCU_EN: u32 = 4;
pub const CLK_SSUSB_DMA_EN: u32 = 5;
pub const CLK_SSUSB_NR_CLK: u32 = 6;

/* PCIESYS */
pub const CLK_PCIE_P1_AUX_EN: u32 = 0;
pub const CLK_PCIE_P1_OBFF_EN: u32 = 1;
pub const CLK_PCIE_P1_AHB_EN: u32 = 2;
pub const CLK_PCIE_P1_AXI_EN: u32 = 3;
pub const CLK_PCIE_P1_MAC_EN: u32 = 4;
pub const CLK_PCIE_P1_PIPE_EN: u32 = 5;
pub const CLK_PCIE_P0_AUX_EN: u32 = 6;
pub const CLK_PCIE_P0_OBFF_EN: u32 = 7;
pub const CLK_PCIE_P0_AHB_EN: u32 = 8;
pub const CLK_PCIE_P0_AXI_EN: u32 = 9;
pub const CLK_PCIE_P0_MAC_EN: u32 = 10;
pub const CLK_PCIE_P0_PIPE_EN: u32 = 11;
pub const CLK_PCIE_NR_CLK: u32 = 12;

/* ETHSYS */
pub const CLK_ETH_FE_EN: u32 = 0;
pub const CLK_ETH_GP2_EN: u32 = 1;
pub const CLK_ETH_GP1_EN: u32 = 2;
pub const CLK_ETH_GP0_EN: u32 = 3;
pub const CLK_ETH_ESW_EN: u32 = 4;
pub const CLK_ETH_NR_CLK: u32 = 5;

/* SGMIISYS */
pub const CLK_SGMII_TX_EN: u32 = 0;
pub const CLK_SGMII_RX_EN: u32 = 1;
pub const CLK_SGMII_CDR_REF: u32 = 2;
pub const CLK_SGMII_CDR_FB: u32 = 3;
pub const CLK_SGMII_NR_CLK: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
