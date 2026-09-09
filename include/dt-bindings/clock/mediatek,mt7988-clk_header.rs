/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Xiufeng Li <Xiufeng.Li@mediatek.com>
 */


/* APMIXEDSYS */

pub const CLK_APMIXED_NETSYSPLL: u32 = 0;
pub const CLK_APMIXED_MPLL: u32 = 1;
pub const CLK_APMIXED_MMPLL: u32 = 2;
pub const CLK_APMIXED_APLL2: u32 = 3;
pub const CLK_APMIXED_NET1PLL: u32 = 4;
pub const CLK_APMIXED_NET2PLL: u32 = 5;
pub const CLK_APMIXED_WEDMCUPLL: u32 = 6;
pub const CLK_APMIXED_SGMPLL: u32 = 7;
pub const CLK_APMIXED_ARM_B: u32 = 8;
pub const CLK_APMIXED_CCIPLL2_B: u32 = 9;
pub const CLK_APMIXED_USXGMIIPLL: u32 = 10;
pub const CLK_APMIXED_MSDCPLL: u32 = 11;

/* TOPCKGEN */

pub const CLK_TOP_XTAL: u32 = 0;
pub const CLK_TOP_XTAL_D2: u32 = 1;
pub const CLK_TOP_RTC_32K: u32 = 2;
pub const CLK_TOP_RTC_32P7K: u32 = 3;
pub const CLK_TOP_MPLL_D2: u32 = 4;
pub const CLK_TOP_MPLL_D3_D2: u32 = 5;
pub const CLK_TOP_MPLL_D4: u32 = 6;
pub const CLK_TOP_MPLL_D8: u32 = 7;
pub const CLK_TOP_MPLL_D8_D2: u32 = 8;
pub const CLK_TOP_MMPLL_D2: u32 = 9;
pub const CLK_TOP_MMPLL_D3_D5: u32 = 10;
pub const CLK_TOP_MMPLL_D4: u32 = 11;
pub const CLK_TOP_MMPLL_D6_D2: u32 = 12;
pub const CLK_TOP_MMPLL_D8: u32 = 13;
pub const CLK_TOP_APLL2_D4: u32 = 14;
pub const CLK_TOP_NET1PLL_D4: u32 = 15;
pub const CLK_TOP_NET1PLL_D5: u32 = 16;
pub const CLK_TOP_NET1PLL_D5_D2: u32 = 17;
pub const CLK_TOP_NET1PLL_D5_D4: u32 = 18;
pub const CLK_TOP_NET1PLL_D8: u32 = 19;
pub const CLK_TOP_NET1PLL_D8_D2: u32 = 20;
pub const CLK_TOP_NET1PLL_D8_D4: u32 = 21;
pub const CLK_TOP_NET1PLL_D8_D8: u32 = 22;
pub const CLK_TOP_NET1PLL_D8_D16: u32 = 23;
pub const CLK_TOP_NET2PLL_D2: u32 = 24;
pub const CLK_TOP_NET2PLL_D4: u32 = 25;
pub const CLK_TOP_NET2PLL_D4_D4: u32 = 26;
pub const CLK_TOP_NET2PLL_D4_D8: u32 = 27;
pub const CLK_TOP_NET2PLL_D6: u32 = 28;
pub const CLK_TOP_NET2PLL_D8: u32 = 29;
pub const CLK_TOP_NETSYS_SEL: u32 = 30;
pub const CLK_TOP_NETSYS_500M_SEL: u32 = 31;
pub const CLK_TOP_NETSYS_2X_SEL: u32 = 32;
pub const CLK_TOP_NETSYS_GSW_SEL: u32 = 33;
pub const CLK_TOP_ETH_GMII_SEL: u32 = 34;
pub const CLK_TOP_NETSYS_MCU_SEL: u32 = 35;
pub const CLK_TOP_NETSYS_PAO_2X_SEL: u32 = 36;
pub const CLK_TOP_EIP197_SEL: u32 = 37;
pub const CLK_TOP_AXI_INFRA_SEL: u32 = 38;
pub const CLK_TOP_UART_SEL: u32 = 39;
pub const CLK_TOP_EMMC_250M_SEL: u32 = 40;
pub const CLK_TOP_EMMC_400M_SEL: u32 = 41;
pub const CLK_TOP_SPI_SEL: u32 = 42;
pub const CLK_TOP_SPIM_MST_SEL: u32 = 43;
pub const CLK_TOP_NFI1X_SEL: u32 = 44;
pub const CLK_TOP_SPINFI_SEL: u32 = 45;
pub const CLK_TOP_PWM_SEL: u32 = 46;
pub const CLK_TOP_I2C_SEL: u32 = 47;
pub const CLK_TOP_PCIE_MBIST_250M_SEL: u32 = 48;
pub const CLK_TOP_PEXTP_TL_SEL: u32 = 49;
pub const CLK_TOP_PEXTP_TL_P1_SEL: u32 = 50;
pub const CLK_TOP_PEXTP_TL_P2_SEL: u32 = 51;
pub const CLK_TOP_PEXTP_TL_P3_SEL: u32 = 52;
pub const CLK_TOP_USB_SYS_SEL: u32 = 53;
pub const CLK_TOP_USB_SYS_P1_SEL: u32 = 54;
pub const CLK_TOP_USB_XHCI_SEL: u32 = 55;
pub const CLK_TOP_USB_XHCI_P1_SEL: u32 = 56;
pub const CLK_TOP_USB_FRMCNT_SEL: u32 = 57;
pub const CLK_TOP_USB_FRMCNT_P1_SEL: u32 = 58;
pub const CLK_TOP_AUD_SEL: u32 = 59;
pub const CLK_TOP_A1SYS_SEL: u32 = 60;
pub const CLK_TOP_AUD_L_SEL: u32 = 61;
pub const CLK_TOP_A_TUNER_SEL: u32 = 62;
pub const CLK_TOP_SSPXTP_SEL: u32 = 63;
pub const CLK_TOP_USB_PHY_SEL: u32 = 64;
pub const CLK_TOP_USXGMII_SBUS_0_SEL: u32 = 65;
pub const CLK_TOP_USXGMII_SBUS_1_SEL: u32 = 66;
pub const CLK_TOP_SGM_0_SEL: u32 = 67;
pub const CLK_TOP_SGM_SBUS_0_SEL: u32 = 68;
pub const CLK_TOP_SGM_1_SEL: u32 = 69;
pub const CLK_TOP_SGM_SBUS_1_SEL: u32 = 70;
pub const CLK_TOP_XFI_PHY_0_XTAL_SEL: u32 = 71;
pub const CLK_TOP_XFI_PHY_1_XTAL_SEL: u32 = 72;
pub const CLK_TOP_SYSAXI_SEL: u32 = 73;
pub const CLK_TOP_SYSAPB_SEL: u32 = 74;
pub const CLK_TOP_ETH_REFCK_50M_SEL: u32 = 75;
pub const CLK_TOP_ETH_SYS_200M_SEL: u32 = 76;
pub const CLK_TOP_ETH_SYS_SEL: u32 = 77;
pub const CLK_TOP_ETH_XGMII_SEL: u32 = 78;
pub const CLK_TOP_BUS_TOPS_SEL: u32 = 79;
pub const CLK_TOP_NPU_TOPS_SEL: u32 = 80;
pub const CLK_TOP_DRAMC_SEL: u32 = 81;
pub const CLK_TOP_DRAMC_MD32_SEL: u32 = 82;
pub const CLK_TOP_INFRA_F26M_SEL: u32 = 83;
pub const CLK_TOP_PEXTP_P0_SEL: u32 = 84;
pub const CLK_TOP_PEXTP_P1_SEL: u32 = 85;
pub const CLK_TOP_PEXTP_P2_SEL: u32 = 86;
pub const CLK_TOP_PEXTP_P3_SEL: u32 = 87;
pub const CLK_TOP_DA_XTP_GLB_P0_SEL: u32 = 88;
pub const CLK_TOP_DA_XTP_GLB_P1_SEL: u32 = 89;
pub const CLK_TOP_DA_XTP_GLB_P2_SEL: u32 = 90;
pub const CLK_TOP_DA_XTP_GLB_P3_SEL: u32 = 91;
pub const CLK_TOP_CKM_SEL: u32 = 92;
pub const CLK_TOP_DA_SEL: u32 = 93;
pub const CLK_TOP_PEXTP_SEL: u32 = 94;
pub const CLK_TOP_TOPS_P2_26M_SEL: u32 = 95;
pub const CLK_TOP_MCUSYS_BACKUP_625M_SEL: u32 = 96;
pub const CLK_TOP_NETSYS_SYNC_250M_SEL: u32 = 97;
pub const CLK_TOP_MACSEC_SEL: u32 = 98;
pub const CLK_TOP_NETSYS_TOPS_400M_SEL: u32 = 99;
pub const CLK_TOP_NETSYS_PPEFB_250M_SEL: u32 = 100;
pub const CLK_TOP_NETSYS_WARP_SEL: u32 = 101;
pub const CLK_TOP_ETH_MII_SEL: u32 = 102;
pub const CLK_TOP_NPU_SEL: u32 = 103;
pub const CLK_TOP_AUD_I2S_M: u32 = 104;

/* MCUSYS */

pub const CLK_MCU_BUS_DIV_SEL: u32 = 0;
pub const CLK_MCU_ARM_DIV_SEL: u32 = 1;

/* INFRACFG_AO */

pub const CLK_INFRA_MUX_UART0_SEL: u32 = 0;
pub const CLK_INFRA_MUX_UART1_SEL: u32 = 1;
pub const CLK_INFRA_MUX_UART2_SEL: u32 = 2;
pub const CLK_INFRA_MUX_SPI0_SEL: u32 = 3;
pub const CLK_INFRA_MUX_SPI1_SEL: u32 = 4;
pub const CLK_INFRA_MUX_SPI2_SEL: u32 = 5;
pub const CLK_INFRA_PWM_SEL: u32 = 6;
pub const CLK_INFRA_PWM_CK1_SEL: u32 = 7;
pub const CLK_INFRA_PWM_CK2_SEL: u32 = 8;
pub const CLK_INFRA_PWM_CK3_SEL: u32 = 9;
pub const CLK_INFRA_PWM_CK4_SEL: u32 = 10;
pub const CLK_INFRA_PWM_CK5_SEL: u32 = 11;
pub const CLK_INFRA_PWM_CK6_SEL: u32 = 12;
pub const CLK_INFRA_PWM_CK7_SEL: u32 = 13;
pub const CLK_INFRA_PWM_CK8_SEL: u32 = 14;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P0_SEL: u32 = 15;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P1_SEL: u32 = 16;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P2_SEL: u32 = 17;
pub const CLK_INFRA_PCIE_GFMUX_TL_O_P3_SEL: u32 = 18;

/* INFRACFG */

pub const CLK_INFRA_PCIE_PERI_26M_CK_P0: u32 = 19;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P1: u32 = 20;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P2: u32 = 21;
pub const CLK_INFRA_PCIE_PERI_26M_CK_P3: u32 = 22;
pub const CLK_INFRA_66M_GPT_BCK: u32 = 23;
pub const CLK_INFRA_66M_PWM_HCK: u32 = 24;
pub const CLK_INFRA_66M_PWM_BCK: u32 = 25;
pub const CLK_INFRA_66M_PWM_CK1: u32 = 26;
pub const CLK_INFRA_66M_PWM_CK2: u32 = 27;
pub const CLK_INFRA_66M_PWM_CK3: u32 = 28;
pub const CLK_INFRA_66M_PWM_CK4: u32 = 29;
pub const CLK_INFRA_66M_PWM_CK5: u32 = 30;
pub const CLK_INFRA_66M_PWM_CK6: u32 = 31;
pub const CLK_INFRA_66M_PWM_CK7: u32 = 32;
pub const CLK_INFRA_66M_PWM_CK8: u32 = 33;
pub const CLK_INFRA_133M_CQDMA_BCK: u32 = 34;
pub const CLK_INFRA_66M_AUD_SLV_BCK: u32 = 35;
pub const CLK_INFRA_AUD_26M: u32 = 36;
pub const CLK_INFRA_AUD_L: u32 = 37;
pub const CLK_INFRA_AUD_AUD: u32 = 38;
pub const CLK_INFRA_AUD_EG2: u32 = 39;
pub const CLK_INFRA_DRAMC_F26M: u32 = 40;
pub const CLK_INFRA_133M_DBG_ACKM: u32 = 41;
pub const CLK_INFRA_66M_AP_DMA_BCK: u32 = 42;
pub const CLK_INFRA_66M_SEJ_BCK: u32 = 43;
pub const CLK_INFRA_PRE_CK_SEJ_F13M: u32 = 44;
pub const CLK_INFRA_26M_THERM_SYSTEM: u32 = 45;
pub const CLK_INFRA_I2C_BCK: u32 = 46;
pub const CLK_INFRA_52M_UART0_CK: u32 = 47;
pub const CLK_INFRA_52M_UART1_CK: u32 = 48;
pub const CLK_INFRA_52M_UART2_CK: u32 = 49;
pub const CLK_INFRA_NFI: u32 = 50;
pub const CLK_INFRA_SPINFI: u32 = 51;
pub const CLK_INFRA_66M_NFI_HCK: u32 = 52;
pub const CLK_INFRA_104M_SPI0: u32 = 53;
pub const CLK_INFRA_104M_SPI1: u32 = 54;
pub const CLK_INFRA_104M_SPI2_BCK: u32 = 55;
pub const CLK_INFRA_66M_SPI0_HCK: u32 = 56;
pub const CLK_INFRA_66M_SPI1_HCK: u32 = 57;
pub const CLK_INFRA_66M_SPI2_HCK: u32 = 58;
pub const CLK_INFRA_66M_FLASHIF_AXI: u32 = 59;
pub const CLK_INFRA_RTC: u32 = 60;
pub const CLK_INFRA_26M_ADC_BCK: u32 = 61;
pub const CLK_INFRA_RC_ADC: u32 = 62;
pub const CLK_INFRA_MSDC400: u32 = 63;
pub const CLK_INFRA_MSDC2_HCK: u32 = 64;
pub const CLK_INFRA_133M_MSDC_0_HCK: u32 = 65;
pub const CLK_INFRA_66M_MSDC_0_HCK: u32 = 66;
pub const CLK_INFRA_133M_CPUM_BCK: u32 = 67;
pub const CLK_INFRA_BIST2FPC: u32 = 68;
pub const CLK_INFRA_I2C_X16W_MCK_CK_P1: u32 = 69;
pub const CLK_INFRA_I2C_X16W_PCK_CK_P1: u32 = 70;
pub const CLK_INFRA_133M_USB_HCK: u32 = 71;
pub const CLK_INFRA_133M_USB_HCK_CK_P1: u32 = 72;
pub const CLK_INFRA_66M_USB_HCK: u32 = 73;
pub const CLK_INFRA_66M_USB_HCK_CK_P1: u32 = 74;
pub const CLK_INFRA_USB_SYS: u32 = 75;
pub const CLK_INFRA_USB_SYS_CK_P1: u32 = 76;
pub const CLK_INFRA_USB_REF: u32 = 77;
pub const CLK_INFRA_USB_CK_P1: u32 = 78;
pub const CLK_INFRA_USB_FRMCNT: u32 = 79;
pub const CLK_INFRA_USB_FRMCNT_CK_P1: u32 = 80;
pub const CLK_INFRA_USB_PIPE: u32 = 81;
pub const CLK_INFRA_USB_PIPE_CK_P1: u32 = 82;
pub const CLK_INFRA_USB_UTMI: u32 = 83;
pub const CLK_INFRA_USB_UTMI_CK_P1: u32 = 84;
pub const CLK_INFRA_USB_XHCI: u32 = 85;
pub const CLK_INFRA_USB_XHCI_CK_P1: u32 = 86;
pub const CLK_INFRA_PCIE_GFMUX_TL_P0: u32 = 87;
pub const CLK_INFRA_PCIE_GFMUX_TL_P1: u32 = 88;
pub const CLK_INFRA_PCIE_GFMUX_TL_P2: u32 = 89;
pub const CLK_INFRA_PCIE_GFMUX_TL_P3: u32 = 90;
pub const CLK_INFRA_PCIE_PIPE_P0: u32 = 91;
pub const CLK_INFRA_PCIE_PIPE_P1: u32 = 92;
pub const CLK_INFRA_PCIE_PIPE_P2: u32 = 93;
pub const CLK_INFRA_PCIE_PIPE_P3: u32 = 94;
pub const CLK_INFRA_133M_PCIE_CK_P0: u32 = 95;
pub const CLK_INFRA_133M_PCIE_CK_P1: u32 = 96;
pub const CLK_INFRA_133M_PCIE_CK_P2: u32 = 97;
pub const CLK_INFRA_133M_PCIE_CK_P3: u32 = 98;

/* ETHDMA */

pub const CLK_ETHDMA_XGP1_EN: u32 = 0;
pub const CLK_ETHDMA_XGP2_EN: u32 = 1;
pub const CLK_ETHDMA_XGP3_EN: u32 = 2;
pub const CLK_ETHDMA_FE_EN: u32 = 3;
pub const CLK_ETHDMA_GP2_EN: u32 = 4;
pub const CLK_ETHDMA_GP1_EN: u32 = 5;
pub const CLK_ETHDMA_GP3_EN: u32 = 6;
pub const CLK_ETHDMA_ESW_EN: u32 = 7;
pub const CLK_ETHDMA_CRYPT0_EN: u32 = 8;
pub const CLK_ETHDMA_NR_CLK: u32 = 9;

/* SGMIISYS_0 */

pub const CLK_SGM0_TX_EN: u32 = 0;
pub const CLK_SGM0_RX_EN: u32 = 1;
pub const CLK_SGMII0_NR_CLK: u32 = 2;

/* SGMIISYS_1 */

pub const CLK_SGM1_TX_EN: u32 = 0;
pub const CLK_SGM1_RX_EN: u32 = 1;
pub const CLK_SGMII1_NR_CLK: u32 = 2;

/* ETHWARP */

pub const CLK_ETHWARP_WOCPU2_EN: u32 = 0;
pub const CLK_ETHWARP_WOCPU1_EN: u32 = 1;
pub const CLK_ETHWARP_WOCPU0_EN: u32 = 2;
pub const CLK_ETHWARP_NR_CLK: u32 = 3;

/* XFIPLL */
pub const CLK_XFIPLL_PLL: u32 = 0;
pub const CLK_XFIPLL_PLL_EN: u32 = 1;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
