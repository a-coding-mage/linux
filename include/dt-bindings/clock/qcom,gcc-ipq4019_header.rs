/* Copyright (c) 2015 The Linux Foundation. All rights reserved.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

pub const GCC_DUMMY_CLK: u32 = 0;
pub const AUDIO_CLK_SRC: u32 = 1;
pub const BLSP1_QUP1_I2C_APPS_CLK_SRC: u32 = 2;
pub const BLSP1_QUP1_SPI_APPS_CLK_SRC: u32 = 3;
pub const BLSP1_QUP2_I2C_APPS_CLK_SRC: u32 = 4;
pub const BLSP1_QUP2_SPI_APPS_CLK_SRC: u32 = 5;
pub const BLSP1_UART1_APPS_CLK_SRC: u32 = 6;
pub const BLSP1_UART2_APPS_CLK_SRC: u32 = 7;
pub const GCC_USB3_MOCK_UTMI_CLK_SRC: u32 = 8;
pub const GCC_APPS_CLK_SRC: u32 = 9;
pub const GCC_APPS_AHB_CLK_SRC: u32 = 10;
pub const GP1_CLK_SRC: u32 = 11;
pub const GP2_CLK_SRC: u32 = 12;
pub const GP3_CLK_SRC: u32 = 13;
pub const SDCC1_APPS_CLK_SRC: u32 = 14;
pub const FEPHY_125M_DLY_CLK_SRC: u32 = 15;
pub const WCSS2G_CLK_SRC: u32 = 16;
pub const WCSS5G_CLK_SRC: u32 = 17;
pub const GCC_APSS_AHB_CLK: u32 = 18;
pub const GCC_AUDIO_AHB_CLK: u32 = 19;
pub const GCC_AUDIO_PWM_CLK: u32 = 20;
pub const GCC_BLSP1_AHB_CLK: u32 = 21;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK: u32 = 22;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK: u32 = 23;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK: u32 = 24;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK: u32 = 25;
pub const GCC_BLSP1_UART1_APPS_CLK: u32 = 26;
pub const GCC_BLSP1_UART2_APPS_CLK: u32 = 27;
pub const GCC_DCD_XO_CLK: u32 = 28;
pub const GCC_GP1_CLK: u32 = 29;
pub const GCC_GP2_CLK: u32 = 30;
pub const GCC_GP3_CLK: u32 = 31;
pub const GCC_BOOT_ROM_AHB_CLK: u32 = 32;
pub const GCC_CRYPTO_AHB_CLK: u32 = 33;
pub const GCC_CRYPTO_AXI_CLK: u32 = 34;
pub const GCC_CRYPTO_CLK: u32 = 35;
pub const GCC_ESS_CLK: u32 = 36;
pub const GCC_IMEM_AXI_CLK: u32 = 37;
pub const GCC_IMEM_CFG_AHB_CLK: u32 = 38;
pub const GCC_PCIE_AHB_CLK: u32 = 39;
pub const GCC_PCIE_AXI_M_CLK: u32 = 40;
pub const GCC_PCIE_AXI_S_CLK: u32 = 41;
pub const GCC_PCNOC_AHB_CLK: u32 = 42;
pub const GCC_PRNG_AHB_CLK: u32 = 43;
pub const GCC_QPIC_AHB_CLK: u32 = 44;
pub const GCC_QPIC_CLK: u32 = 45;
pub const GCC_SDCC1_AHB_CLK: u32 = 46;
pub const GCC_SDCC1_APPS_CLK: u32 = 47;
pub const GCC_SNOC_PCNOC_AHB_CLK: u32 = 48;
pub const GCC_SYS_NOC_125M_CLK: u32 = 49;
pub const GCC_SYS_NOC_AXI_CLK: u32 = 50;
pub const GCC_TCSR_AHB_CLK: u32 = 51;
pub const GCC_TLMM_AHB_CLK: u32 = 52;
pub const GCC_USB2_MASTER_CLK: u32 = 53;
pub const GCC_USB2_SLEEP_CLK: u32 = 54;
pub const GCC_USB2_MOCK_UTMI_CLK: u32 = 55;
pub const GCC_USB3_MASTER_CLK: u32 = 56;
pub const GCC_USB3_SLEEP_CLK: u32 = 57;
pub const GCC_USB3_MOCK_UTMI_CLK: u32 = 58;
pub const GCC_WCSS2G_CLK: u32 = 59;
pub const GCC_WCSS2G_REF_CLK: u32 = 60;
pub const GCC_WCSS2G_RTC_CLK: u32 = 61;
pub const GCC_WCSS5G_CLK: u32 = 62;
pub const GCC_WCSS5G_REF_CLK: u32 = 63;
pub const GCC_WCSS5G_RTC_CLK: u32 = 64;
pub const GCC_APSS_DDRPLL_VCO: u32 = 65;
pub const GCC_SDCC_PLLDIV_CLK: u32 = 66;
pub const GCC_FEPLL_VCO: u32 = 67;
pub const GCC_FEPLL125_CLK: u32 = 68;
pub const GCC_FEPLL125DLY_CLK: u32 = 69;
pub const GCC_FEPLL200_CLK: u32 = 70;
pub const GCC_FEPLL500_CLK: u32 = 71;
pub const GCC_FEPLL_WCSS2G_CLK: u32 = 72;
pub const GCC_FEPLL_WCSS5G_CLK: u32 = 73;
pub const GCC_APSS_CPU_PLLDIV_CLK: u32 = 74;
pub const GCC_PCNOC_AHB_CLK_SRC: u32 = 75;

pub const WIFI0_CPU_INIT_RESET: u32 = 0;
pub const WIFI0_RADIO_SRIF_RESET: u32 = 1;
pub const WIFI0_RADIO_WARM_RESET: u32 = 2;
pub const WIFI0_RADIO_COLD_RESET: u32 = 3;
pub const WIFI0_CORE_WARM_RESET: u32 = 4;
pub const WIFI0_CORE_COLD_RESET: u32 = 5;
pub const WIFI1_CPU_INIT_RESET: u32 = 6;
pub const WIFI1_RADIO_SRIF_RESET: u32 = 7;
pub const WIFI1_RADIO_WARM_RESET: u32 = 8;
pub const WIFI1_RADIO_COLD_RESET: u32 = 9;
pub const WIFI1_CORE_WARM_RESET: u32 = 10;
pub const WIFI1_CORE_COLD_RESET: u32 = 11;
pub const USB3_UNIPHY_PHY_ARES: u32 = 12;
pub const USB3_HSPHY_POR_ARES: u32 = 13;
pub const USB3_HSPHY_S_ARES: u32 = 14;
pub const USB2_HSPHY_POR_ARES: u32 = 15;
pub const USB2_HSPHY_S_ARES: u32 = 16;
pub const PCIE_PHY_AHB_ARES: u32 = 17;
pub const PCIE_AHB_ARES: u32 = 18;
pub const PCIE_PWR_ARES: u32 = 19;
pub const PCIE_PIPE_STICKY_ARES: u32 = 20;
pub const PCIE_AXI_M_STICKY_ARES: u32 = 21;
pub const PCIE_PHY_ARES: u32 = 22;
pub const PCIE_PARF_XPU_ARES: u32 = 23;
pub const PCIE_AXI_S_XPU_ARES: u32 = 24;
pub const PCIE_AXI_M_VMIDMT_ARES: u32 = 25;
pub const PCIE_PIPE_ARES: u32 = 26;
pub const PCIE_AXI_S_ARES: u32 = 27;
pub const PCIE_AXI_M_ARES: u32 = 28;
pub const ESS_RESET: u32 = 29;
pub const GCC_BLSP1_BCR: u32 = 30;
pub const GCC_BLSP1_QUP1_BCR: u32 = 31;
pub const GCC_BLSP1_UART1_BCR: u32 = 32;
pub const GCC_BLSP1_QUP2_BCR: u32 = 33;
pub const GCC_BLSP1_UART2_BCR: u32 = 34;
pub const GCC_BIMC_BCR: u32 = 35;
pub const GCC_TLMM_BCR: u32 = 36;
pub const GCC_IMEM_BCR: u32 = 37;
pub const GCC_ESS_BCR: u32 = 38;
pub const GCC_PRNG_BCR: u32 = 39;
pub const GCC_BOOT_ROM_BCR: u32 = 40;
pub const GCC_CRYPTO_BCR: u32 = 41;
pub const GCC_SDCC1_BCR: u32 = 42;
pub const GCC_SEC_CTRL_BCR: u32 = 43;
pub const GCC_AUDIO_BCR: u32 = 44;
pub const GCC_QPIC_BCR: u32 = 45;
pub const GCC_PCIE_BCR: u32 = 46;
pub const GCC_USB2_BCR: u32 = 47;
pub const GCC_USB2_PHY_BCR: u32 = 48;
pub const GCC_USB3_BCR: u32 = 49;
pub const GCC_USB3_PHY_BCR: u32 = 50;
pub const GCC_SYSTEM_NOC_BCR: u32 = 51;
pub const GCC_PCNOC_BCR: u32 = 52;
pub const GCC_DCD_BCR: u32 = 53;
pub const GCC_SNOC_BUS_TIMEOUT0_BCR: u32 = 54;
pub const GCC_SNOC_BUS_TIMEOUT1_BCR: u32 = 55;
pub const GCC_SNOC_BUS_TIMEOUT2_BCR: u32 = 56;
pub const GCC_SNOC_BUS_TIMEOUT3_BCR: u32 = 57;
pub const GCC_PCNOC_BUS_TIMEOUT0_BCR: u32 = 58;
pub const GCC_PCNOC_BUS_TIMEOUT1_BCR: u32 = 59;
pub const GCC_PCNOC_BUS_TIMEOUT2_BCR: u32 = 60;
pub const GCC_PCNOC_BUS_TIMEOUT3_BCR: u32 = 61;
pub const GCC_PCNOC_BUS_TIMEOUT4_BCR: u32 = 62;
pub const GCC_PCNOC_BUS_TIMEOUT5_BCR: u32 = 63;
pub const GCC_PCNOC_BUS_TIMEOUT6_BCR: u32 = 64;
pub const GCC_PCNOC_BUS_TIMEOUT7_BCR: u32 = 65;
pub const GCC_PCNOC_BUS_TIMEOUT8_BCR: u32 = 66;
pub const GCC_PCNOC_BUS_TIMEOUT9_BCR: u32 = 67;
pub const GCC_TCSR_BCR: u32 = 68;
pub const GCC_QDSS_BCR: u32 = 69;
pub const GCC_MPM_BCR: u32 = 70;
pub const GCC_SPDM_BCR: u32 = 71;
pub const ESS_MAC1_ARES: u32 = 72;
pub const ESS_MAC2_ARES: u32 = 73;
pub const ESS_MAC3_ARES: u32 = 74;
pub const ESS_MAC4_ARES: u32 = 75;
pub const ESS_MAC5_ARES: u32 = 76;
pub const ESS_PSGMII_ARES: u32 = 77;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
