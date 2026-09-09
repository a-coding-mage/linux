/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 */

/* APMIXEDSYS */
pub const CLK_APMIXED_ARMPLL: u32 = 0;
pub const CLK_APMIXED_NET2PLL: u32 = 1;
pub const CLK_APMIXED_MMPLL: u32 = 2;
pub const CLK_APMIXED_SGMPLL: u32 = 3;
pub const CLK_APMIXED_WEDMCUPLL: u32 = 4;
pub const CLK_APMIXED_NET1PLL: u32 = 5;
pub const CLK_APMIXED_MPLL: u32 = 6;
pub const CLK_APMIXED_APLL2: u32 = 7;

/* TOPCKGEN */
pub const CLK_TOP_XTAL: u32 = 0;
pub const CLK_TOP_XTAL_D2: u32 = 1;
pub const CLK_TOP_RTC_32K: u32 = 2;
pub const CLK_TOP_RTC_32P7K: u32 = 3;
pub const CLK_TOP_MPLL_D2: u32 = 4;
pub const CLK_TOP_MPLL_D4: u32 = 5;
pub const CLK_TOP_MPLL_D8: u32 = 6;
pub const CLK_TOP_MPLL_D8_D2: u32 = 7;
pub const CLK_TOP_MPLL_D3_D2: u32 = 8;
pub const CLK_TOP_MMPLL_D2: u32 = 9;
pub const CLK_TOP_MMPLL_D4: u32 = 10;
pub const CLK_TOP_MMPLL_D8: u32 = 11;
pub const CLK_TOP_MMPLL_D8_D2: u32 = 12;
pub const CLK_TOP_MMPLL_D3_D8: u32 = 13;
pub const CLK_TOP_MMPLL_U2PHY: u32 = 14;
pub const CLK_TOP_APLL2_D4: u32 = 15;
pub const CLK_TOP_NET1PLL_D4: u32 = 16;
pub const CLK_TOP_NET1PLL_D5: u32 = 17;
pub const CLK_TOP_NET1PLL_D5_D2: u32 = 18;
pub const CLK_TOP_NET1PLL_D5_D4: u32 = 19;
pub const CLK_TOP_NET1PLL_D8_D2: u32 = 20;
pub const CLK_TOP_NET1PLL_D8_D4: u32 = 21;
pub const CLK_TOP_NET2PLL_D4: u32 = 22;
pub const CLK_TOP_NET2PLL_D4_D2: u32 = 23;
pub const CLK_TOP_NET2PLL_D3_D2: u32 = 24;
pub const CLK_TOP_WEDMCUPLL_D5_D2: u32 = 25;
pub const CLK_TOP_NFI1X_SEL: u32 = 26;
pub const CLK_TOP_SPINFI_SEL: u32 = 27;
pub const CLK_TOP_SPI_SEL: u32 = 28;
pub const CLK_TOP_SPIM_MST_SEL: u32 = 29;
pub const CLK_TOP_UART_SEL: u32 = 30;
pub const CLK_TOP_PWM_SEL: u32 = 31;
pub const CLK_TOP_I2C_SEL: u32 = 32;
pub const CLK_TOP_PEXTP_TL_SEL: u32 = 33;
pub const CLK_TOP_EMMC_250M_SEL: u32 = 34;
pub const CLK_TOP_EMMC_416M_SEL: u32 = 35;
pub const CLK_TOP_F_26M_ADC_SEL: u32 = 36;
pub const CLK_TOP_DRAMC_SEL: u32 = 37;
pub const CLK_TOP_DRAMC_MD32_SEL: u32 = 38;
pub const CLK_TOP_SYSAXI_SEL: u32 = 39;
pub const CLK_TOP_SYSAPB_SEL: u32 = 40;
pub const CLK_TOP_ARM_DB_MAIN_SEL: u32 = 41;
pub const CLK_TOP_ARM_DB_JTSEL: u32 = 42;
pub const CLK_TOP_NETSYS_SEL: u32 = 43;
pub const CLK_TOP_NETSYS_500M_SEL: u32 = 44;
pub const CLK_TOP_NETSYS_MCU_SEL: u32 = 45;
pub const CLK_TOP_NETSYS_2X_SEL: u32 = 46;
pub const CLK_TOP_SGM_325M_SEL: u32 = 47;
pub const CLK_TOP_SGM_REG_SEL: u32 = 48;
pub const CLK_TOP_A1SYS_SEL: u32 = 49;
pub const CLK_TOP_CONN_MCUSYS_SEL: u32 = 50;
pub const CLK_TOP_EIP_B_SEL: u32 = 51;
pub const CLK_TOP_PCIE_PHY_SEL: u32 = 52;
pub const CLK_TOP_USB3_PHY_SEL: u32 = 53;
pub const CLK_TOP_F26M_SEL: u32 = 54;
pub const CLK_TOP_AUD_L_SEL: u32 = 55;
pub const CLK_TOP_A_TUNER_SEL: u32 = 56;
pub const CLK_TOP_U2U3_SEL: u32 = 57;
pub const CLK_TOP_U2U3_SYS_SEL: u32 = 58;
pub const CLK_TOP_U2U3_XHCI_SEL: u32 = 59;
pub const CLK_TOP_DA_U2_REFSEL: u32 = 60;
pub const CLK_TOP_DA_U2_CK_1P_SEL: u32 = 61;
pub const CLK_TOP_AP2CNN_HOST_SEL: u32 = 62;
pub const CLK_TOP_JTAG: u32 = 63;

/* INFRACFG */
pub const CLK_INFRA_SYSAXI_D2: u32 = 0;
pub const CLK_INFRA_UART0_SEL: u32 = 1;
pub const CLK_INFRA_UART1_SEL: u32 = 2;
pub const CLK_INFRA_UART2_SEL: u32 = 3;
pub const CLK_INFRA_SPI0_SEL: u32 = 4;
pub const CLK_INFRA_SPI1_SEL: u32 = 5;
pub const CLK_INFRA_PWM1_SEL: u32 = 6;
pub const CLK_INFRA_PWM2_SEL: u32 = 7;
pub const CLK_INFRA_PWM_BSEL: u32 = 8;
pub const CLK_INFRA_PCIE_SEL: u32 = 9;
pub const CLK_INFRA_GPT_STA: u32 = 10;
pub const CLK_INFRA_PWM_HCK: u32 = 11;
pub const CLK_INFRA_PWM_STA: u32 = 12;
pub const CLK_INFRA_PWM1_CK: u32 = 13;
pub const CLK_INFRA_PWM2_CK: u32 = 14;
pub const CLK_INFRA_CQ_DMA_CK: u32 = 15;
pub const CLK_INFRA_EIP97_CK: u32 = 16;
pub const CLK_INFRA_AUD_BUS_CK: u32 = 17;
pub const CLK_INFRA_AUD_26M_CK: u32 = 18;
pub const CLK_INFRA_AUD_L_CK: u32 = 19;
pub const CLK_INFRA_AUD_AUD_CK: u32 = 20;
pub const CLK_INFRA_AUD_EG2_CK: u32 = 21;
pub const CLK_INFRA_DRAMC_26M_CK: u32 = 22;
pub const CLK_INFRA_DBG_CK: u32 = 23;
pub const CLK_INFRA_AP_DMA_CK: u32 = 24;
pub const CLK_INFRA_SEJ_CK: u32 = 25;
pub const CLK_INFRA_SEJ_13M_CK: u32 = 26;
pub const CLK_INFRA_THERM_CK: u32 = 27;
pub const CLK_INFRA_I2C0_CK: u32 = 28;
pub const CLK_INFRA_UART0_CK: u32 = 29;
pub const CLK_INFRA_UART1_CK: u32 = 30;
pub const CLK_INFRA_UART2_CK: u32 = 31;
pub const CLK_INFRA_NFI1_CK: u32 = 32;
pub const CLK_INFRA_SPINFI1_CK: u32 = 33;
pub const CLK_INFRA_NFI_HCK_CK: u32 = 34;
pub const CLK_INFRA_SPI0_CK: u32 = 35;
pub const CLK_INFRA_SPI1_CK: u32 = 36;
pub const CLK_INFRA_SPI0_HCK_CK: u32 = 37;
pub const CLK_INFRA_SPI1_HCK_CK: u32 = 38;
pub const CLK_INFRA_FRTC_CK: u32 = 39;
pub const CLK_INFRA_MSDC_CK: u32 = 40;
pub const CLK_INFRA_MSDC_HCK_CK: u32 = 41;
pub const CLK_INFRA_MSDC_133M_CK: u32 = 42;
pub const CLK_INFRA_MSDC_66M_CK: u32 = 43;
pub const CLK_INFRA_ADC_26M_CK: u32 = 44;
pub const CLK_INFRA_ADC_FRC_CK: u32 = 45;
pub const CLK_INFRA_FBIST2FPC_CK: u32 = 46;
pub const CLK_INFRA_IUSB_133_CK: u32 = 47;
pub const CLK_INFRA_IUSB_66M_CK: u32 = 48;
pub const CLK_INFRA_IUSB_SYS_CK: u32 = 49;
pub const CLK_INFRA_IUSB_CK: u32 = 50;
pub const CLK_INFRA_IPCIE_CK: u32 = 51;
pub const CLK_INFRA_IPCIE_PIPE_CK: u32 = 52;
pub const CLK_INFRA_IPCIER_CK: u32 = 53;
pub const CLK_INFRA_IPCIEB_CK: u32 = 54;
pub const CLK_INFRA_TRNG_CK: u32 = 55;

/* SGMIISYS_0 */
pub const CLK_SGMII0_TX250M_EN: u32 = 0;
pub const CLK_SGMII0_RX250M_EN: u32 = 1;
pub const CLK_SGMII0_CDR_REF: u32 = 2;
pub const CLK_SGMII0_CDR_FB: u32 = 3;

/* SGMIISYS_1 */
pub const CLK_SGMII1_TX250M_EN: u32 = 0;
pub const CLK_SGMII1_RX250M_EN: u32 = 1;
pub const CLK_SGMII1_CDR_REF: u32 = 2;
pub const CLK_SGMII1_CDR_FB: u32 = 3;

/* ETHSYS */
pub const CLK_ETH_FE_EN: u32 = 0;
pub const CLK_ETH_GP2_EN: u32 = 1;
pub const CLK_ETH_GP1_EN: u32 = 2;
pub const CLK_ETH_WOCPU1_EN: u32 = 3;
pub const CLK_ETH_WOCPU0_EN: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
