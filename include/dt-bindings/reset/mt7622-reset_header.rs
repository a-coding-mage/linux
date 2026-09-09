/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Sean Wang <sean.wang@mediatek.com>
 */

/* INFRACFG resets */
pub const MT7622_INFRA_EMI_REG_RST: u32 = 0;
pub const MT7622_INFRA_DRAMC0_A0_RST: u32 = 1;
pub const MT7622_INFRA_APCIRQ_EINT_RST: u32 = 3;
pub const MT7622_INFRA_APXGPT_RST: u32 = 4;
pub const MT7622_INFRA_SCPSYS_RST: u32 = 5;
pub const MT7622_INFRA_PMIC_WRAP_RST: u32 = 7;
pub const MT7622_INFRA_IRRX_RST: u32 = 9;
pub const MT7622_INFRA_EMI_RST: u32 = 16;
pub const MT7622_INFRA_WED0_RST: u32 = 17;
pub const MT7622_INFRA_DRAMC_RST: u32 = 18;
pub const MT7622_INFRA_CCI_INTF_RST: u32 = 19;
pub const MT7622_INFRA_TRNG_RST: u32 = 21;
pub const MT7622_INFRA_SYSIRQ_RST: u32 = 22;
pub const MT7622_INFRA_WED1_RST: u32 = 25;

/* PERICFG Subsystem resets */
pub const MT7622_PERI_UART0_SW_RST: u32 = 0;
pub const MT7622_PERI_UART1_SW_RST: u32 = 1;
pub const MT7622_PERI_UART2_SW_RST: u32 = 2;
pub const MT7622_PERI_UART3_SW_RST: u32 = 3;
pub const MT7622_PERI_UART4_SW_RST: u32 = 4;
pub const MT7622_PERI_BTIF_SW_RST: u32 = 6;
pub const MT7622_PERI_PWM_SW_RST: u32 = 8;
pub const MT7622_PERI_AUXADC_SW_RST: u32 = 10;
pub const MT7622_PERI_DMA_SW_RST: u32 = 11;
pub const MT7622_PERI_IRTX_SW_RST: u32 = 13;
pub const MT7622_PERI_NFI_SW_RST: u32 = 14;
pub const MT7622_PERI_THERM_SW_RST: u32 = 16;
pub const MT7622_PERI_MSDC0_SW_RST: u32 = 19;
pub const MT7622_PERI_MSDC1_SW_RST: u32 = 20;
pub const MT7622_PERI_I2C0_SW_RST: u32 = 22;
pub const MT7622_PERI_I2C1_SW_RST: u32 = 23;
pub const MT7622_PERI_I2C2_SW_RST: u32 = 24;
pub const MT7622_PERI_SPI0_SW_RST: u32 = 33;
pub const MT7622_PERI_SPI1_SW_RST: u32 = 34;
pub const MT7622_PERI_FLASHIF_SW_RST: u32 = 36;

/* TOPRGU resets */
pub const MT7622_TOPRGU_INFRA_RST: u32 = 0;
pub const MT7622_TOPRGU_ETHDMA_RST: u32 = 1;
pub const MT7622_TOPRGU_DDRPHY_RST: u32 = 6;
pub const MT7622_TOPRGU_INFRA_AO_RST: u32 = 8;
pub const MT7622_TOPRGU_CONN_RST: u32 = 9;
pub const MT7622_TOPRGU_APMIXED_RST: u32 = 10;
pub const MT7622_TOPRGU_CONN_MCU_RST: u32 = 12;

/* PCIe/SATA Subsystem resets */
pub const MT7622_SATA_PHY_REG_RST: u32 = 12;
pub const MT7622_SATA_PHY_SW_RST: u32 = 13;
pub const MT7622_SATA_AXI_BUS_RST: u32 = 15;
pub const MT7622_PCIE1_CORE_RST: u32 = 19;
pub const MT7622_PCIE1_MMIO_RST: u32 = 20;
pub const MT7622_PCIE1_HRST: u32 = 21;
pub const MT7622_PCIE1_USER_RST: u32 = 22;
pub const MT7622_PCIE1_PIPE_RST: u32 = 23;
pub const MT7622_PCIE0_CORE_RST: u32 = 27;
pub const MT7622_PCIE0_MMIO_RST: u32 = 28;
pub const MT7622_PCIE0_HRST: u32 = 29;
pub const MT7622_PCIE0_USER_RST: u32 = 30;
pub const MT7622_PCIE0_PIPE_RST: u32 = 31;

/* SSUSB Subsystem resets */
pub const MT7622_SSUSB_PHY_PWR_RST: u32 = 3;
pub const MT7622_SSUSB_MAC_PWR_RST: u32 = 4;

/* ETHSYS Subsystem resets */
pub const MT7622_ETHSYS_SYS_RST: u32 = 0;
pub const MT7622_ETHSYS_MCM_RST: u32 = 2;
pub const MT7622_ETHSYS_HSDMA_RST: u32 = 5;
pub const MT7622_ETHSYS_FE_RST: u32 = 6;
pub const MT7622_ETHSYS_GMAC_RST: u32 = 23;
pub const MT7622_ETHSYS_EPHY_RST: u32 = 24;
pub const MT7622_ETHSYS_CRYPTO_RST: u32 = 29;
pub const MT7622_ETHSYS_PPE_RST: u32 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
