/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 iopsys Software Solutions AB.
 * Copyright (C) 2025 Genexis AB.
 *
 * Author: Mikhail Kshevetskiy <mikhail.kshevetskiy@iopsys.eu>
 *
 * based on
 *   include/dt-bindings/reset/airoha,en7581-reset.h
 * by Lorenzo Bianconi <lorenzo@kernel.org>
 */

/* RST_CTRL2 */
pub const EN7523_XPON_PHY_RST: u32 = 0;
pub const EN7523_XSI_MAC_RST: u32 = 1;
pub const EN7523_XSI_PHY_RST: u32 = 2;
pub const EN7523_NPU_RST: u32 = 3;
pub const EN7523_I2S_RST: u32 = 4;
pub const EN7523_TRNG_RST: u32 = 5;
pub const EN7523_TRNG_MSTART_RST: u32 = 6;
pub const EN7523_DUAL_HSI0_RST: u32 = 7;
pub const EN7523_DUAL_HSI1_RST: u32 = 8;
pub const EN7523_HSI_RST: u32 = 9;
pub const EN7523_DUAL_HSI0_MAC_RST: u32 = 10;
pub const EN7523_DUAL_HSI1_MAC_RST: u32 = 11;
pub const EN7523_HSI_MAC_RST: u32 = 12;
pub const EN7523_WDMA_RST: u32 = 13;
pub const EN7523_WOE0_RST: u32 = 14;
pub const EN7523_WOE1_RST: u32 = 15;
pub const EN7523_HSDMA_RST: u32 = 16;
pub const EN7523_I2C2RBUS_RST: u32 = 17;
pub const EN7523_TDMA_RST: u32 = 18;
/* RST_CTRL1 */
pub const EN7523_PCM1_ZSI_ISI_RST: u32 = 19;
pub const EN7523_FE_PDMA_RST: u32 = 20;
pub const EN7523_FE_QDMA_RST: u32 = 21;
pub const EN7523_PCM_SPIWP_RST: u32 = 22;
pub const EN7523_CRYPTO_RST: u32 = 23;
pub const EN7523_TIMER_RST: u32 = 24;
pub const EN7523_PCM1_RST: u32 = 25;
pub const EN7523_UART_RST: u32 = 26;
pub const EN7523_GPIO_RST: u32 = 27;
pub const EN7523_GDMA_RST: u32 = 28;
pub const EN7523_I2C_MASTER_RST: u32 = 29;
pub const EN7523_PCM2_ZSI_ISI_RST: u32 = 30;
pub const EN7523_SFC_RST: u32 = 31;
pub const EN7523_UART2_RST: u32 = 32;
pub const EN7523_GDMP_RST: u32 = 33;
pub const EN7523_FE_RST: u32 = 34;
pub const EN7523_USB_HOST_P0_RST: u32 = 35;
pub const EN7523_GSW_RST: u32 = 36;
pub const EN7523_SFC2_PCM_RST: u32 = 37;
pub const EN7523_PCIE0_RST: u32 = 38;
pub const EN7523_PCIE1_RST: u32 = 39;
pub const EN7523_PCIE_HB_RST: u32 = 40;
pub const EN7523_XPON_MAC_RST: u32 = 41;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
