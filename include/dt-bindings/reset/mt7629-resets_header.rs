/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 MediaTek Inc.
 */

/* INFRACFG resets */
pub const MT7629_INFRA_EMI_MPU_RST: u32 = 0;
pub const MT7629_INFRA_UART5_RST: u32 = 2;
pub const MT7629_INFRA_CIRQ_EINT_RST: u32 = 3;
pub const MT7629_INFRA_APXGPT_RST: u32 = 4;
pub const MT7629_INFRA_SCPSYS_RST: u32 = 5;
pub const MT7629_INFRA_KP_RST: u32 = 6;
pub const MT7629_INFRA_SPI1_RST: u32 = 7;
pub const MT7629_INFRA_SPI4_RST: u32 = 8;
pub const MT7629_INFRA_SYSTIMER_RST: u32 = 9;
pub const MT7629_INFRA_IRRX_RST: u32 = 10;
pub const MT7629_INFRA_AO_BUS_RST: u32 = 16;
pub const MT7629_INFRA_EMI_RST: u32 = 32;
pub const MT7629_INFRA_APMIXED_RST: u32 = 35;
pub const MT7629_INFRA_MIPI_RST: u32 = 36;
pub const MT7629_INFRA_TRNG_RST: u32 = 37;
pub const MT7629_INFRA_SYSCIRQ_RST: u32 = 38;
pub const MT7629_INFRA_MIPI_CSI_RST: u32 = 39;
pub const MT7629_INFRA_GCE_FAXI_RST: u32 = 40;
pub const MT7629_INFRA_I2C_SRAM_RST: u32 = 41;
pub const MT7629_INFRA_IOMMU_RST: u32 = 47;

/* PERICFG resets */
pub const MT7629_PERI_UART0_SW_RST: u32 = 0;
pub const MT7629_PERI_UART1_SW_RST: u32 = 1;
pub const MT7629_PERI_UART2_SW_RST: u32 = 2;
pub const MT7629_PERI_BTIF_SW_RST: u32 = 6;
pub const MT7629_PERI_PWN_SW_RST: u32 = 8;
pub const MT7629_PERI_DMA_SW_RST: u32 = 11;
pub const MT7629_PERI_NFI_SW_RST: u32 = 14;
pub const MT7629_PERI_I2C0_SW_RST: u32 = 22;
pub const MT7629_PERI_SPI0_SW_RST: u32 = 33;
pub const MT7629_PERI_SPI1_SW_RST: u32 = 34;
pub const MT7629_PERI_FLASHIF_SW_RST: u32 = 36;

/* PCIe Subsystem resets */
pub const MT7629_PCIE1_CORE_RST: u32 = 19;
pub const MT7629_PCIE1_MMIO_RST: u32 = 20;
pub const MT7629_PCIE1_HRST: u32 = 21;
pub const MT7629_PCIE1_USER_RST: u32 = 22;
pub const MT7629_PCIE1_PIPE_RST: u32 = 23;
pub const MT7629_PCIE0_CORE_RST: u32 = 27;
pub const MT7629_PCIE0_MMIO_RST: u32 = 28;
pub const MT7629_PCIE0_HRST: u32 = 29;
pub const MT7629_PCIE0_USER_RST: u32 = 30;
pub const MT7629_PCIE0_PIPE_RST: u32 = 31;

/* SSUSB Subsystem resets */
pub const MT7629_SSUSB_PHY_PWR_RST: u32 = 3;
pub const MT7629_SSUSB_MAC_PWR_RST: u32 = 4;

/* ETH Subsystem resets */
pub const MT7629_ETHSYS_SYS_RST: u32 = 0;
pub const MT7629_ETHSYS_MCM_RST: u32 = 2;
pub const MT7629_ETHSYS_HSDMA_RST: u32 = 5;
pub const MT7629_ETHSYS_FE_RST: u32 = 6;
pub const MT7629_ETHSYS_ESW_RST: u32 = 16;
pub const MT7629_ETHSYS_GMAC_RST: u32 = 23;
pub const MT7629_ETHSYS_EPHY_RST: u32 = 24;
pub const MT7629_ETHSYS_CRYPTO_RST: u32 = 29;
pub const MT7629_ETHSYS_PPE_RST: u32 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
