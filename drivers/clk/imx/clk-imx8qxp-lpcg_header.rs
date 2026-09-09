/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2018 NXP
 *   Dong Aisheng <aisheng.dong@nxp.com>
 */

/* LSIO SS */
pub const LSIO_PWM_0_LPCG: u32 = 0x00000;
pub const LSIO_PWM_1_LPCG: u32 = 0x10000;
pub const LSIO_PWM_2_LPCG: u32 = 0x20000;
pub const LSIO_PWM_3_LPCG: u32 = 0x30000;
pub const LSIO_PWM_4_LPCG: u32 = 0x40000;
pub const LSIO_PWM_5_LPCG: u32 = 0x50000;
pub const LSIO_PWM_6_LPCG: u32 = 0x60000;
pub const LSIO_PWM_7_LPCG: u32 = 0x70000;
pub const LSIO_GPIO_0_LPCG: u32 = 0x80000;
pub const LSIO_GPIO_1_LPCG: u32 = 0x90000;
pub const LSIO_GPIO_2_LPCG: u32 = 0xa0000;
pub const LSIO_GPIO_3_LPCG: u32 = 0xb0000;
pub const LSIO_GPIO_4_LPCG: u32 = 0xc0000;
pub const LSIO_GPIO_5_LPCG: u32 = 0xd0000;
pub const LSIO_GPIO_6_LPCG: u32 = 0xe0000;
pub const LSIO_GPIO_7_LPCG: u32 = 0xf0000;
pub const LSIO_FSPI_0_LPCG: u32 = 0x120000;
pub const LSIO_FSPI_1_LPCG: u32 = 0x130000;
pub const LSIO_GPT_0_LPCG: u32 = 0x140000;
pub const LSIO_GPT_1_LPCG: u32 = 0x150000;
pub const LSIO_GPT_2_LPCG: u32 = 0x160000;
pub const LSIO_GPT_3_LPCG: u32 = 0x170000;
pub const LSIO_GPT_4_LPCG: u32 = 0x180000;
pub const LSIO_OCRAM_LPCG: u32 = 0x190000;
pub const LSIO_KPP_LPCG: u32 = 0x1a0000;
pub const LSIO_ROMCP_LPCG: u32 = 0x100000;

/* Connectivity SS */
pub const CONN_USDHC_0_LPCG: u32 = 0x00000;
pub const CONN_USDHC_1_LPCG: u32 = 0x10000;
pub const CONN_USDHC_2_LPCG: u32 = 0x20000;
pub const CONN_ENET_0_LPCG: u32 = 0x30000;
pub const CONN_ENET_1_LPCG: u32 = 0x40000;
pub const CONN_DTCP_LPCG: u32 = 0x50000;
pub const CONN_USB_2_LPCG: u32 = 0x70000;
pub const CONN_USB_3_LPCG: u32 = 0x80000;
pub const CONN_NAND_LPCG: u32 = 0x90000;
pub const CONN_EDMA_LPCG: u32 = 0xa0000;

/* ADMA SS */
pub const ADMA_ASRC_0_LPCG: u32 = 0x400000;
pub const ADMA_ESAI_0_LPCG: u32 = 0x410000;
pub const ADMA_SPDIF_0_LPCG: u32 = 0x420000;
pub const ADMA_SAI_0_LPCG: u32 = 0x440000;
pub const ADMA_SAI_1_LPCG: u32 = 0x450000;
pub const ADMA_SAI_2_LPCG: u32 = 0x460000;
pub const ADMA_SAI_3_LPCG: u32 = 0x470000;
pub const ADMA_GPT_5_LPCG: u32 = 0x4b0000;
pub const ADMA_GPT_6_LPCG: u32 = 0x4c0000;
pub const ADMA_GPT_7_LPCG: u32 = 0x4d0000;
pub const ADMA_GPT_8_LPCG: u32 = 0x4e0000;
pub const ADMA_GPT_9_LPCG: u32 = 0x4f0000;
pub const ADMA_GPT_10_LPCG: u32 = 0x500000;
pub const ADMA_HIFI_LPCG: u32 = 0x580000;
pub const ADMA_OCRAM_LPCG: u32 = 0x590000;
pub const ADMA_EDMA_0_LPCG: u32 = 0x5f0000;
pub const ADMA_ASRC_1_LPCG: u32 = 0xc00000;
pub const ADMA_SAI_4_LPCG: u32 = 0xc20000;
pub const ADMA_SAI_5_LPCG: u32 = 0xc30000;
pub const ADMA_AMIX_LPCG: u32 = 0xc40000;
pub const ADMA_MQS_LPCG: u32 = 0xc50000;
pub const ADMA_ACM_LPCG: u32 = 0xc60000;
pub const ADMA_REC_CLK0_LPCG: u32 = 0xd00000;
pub const ADMA_REC_CLK1_LPCG: u32 = 0xd10000;
pub const ADMA_PLL_CLK0_LPCG: u32 = 0xd20000;
pub const ADMA_PLL_CLK1_LPCG: u32 = 0xd30000;
pub const ADMA_MCLKOUT0_LPCG: u32 = 0xd50000;
pub const ADMA_MCLKOUT1_LPCG: u32 = 0xd60000;
pub const ADMA_EDMA_1_LPCG: u32 = 0xdf0000;
pub const ADMA_LPSPI_0_LPCG: u32 = 0x1400000;
pub const ADMA_LPSPI_1_LPCG: u32 = 0x1410000;
pub const ADMA_LPSPI_2_LPCG: u32 = 0x1420000;
pub const ADMA_LPSPI_3_LPCG: u32 = 0x1430000;
pub const ADMA_LPUART_0_LPCG: u32 = 0x1460000;
pub const ADMA_LPUART_1_LPCG: u32 = 0x1470000;
pub const ADMA_LPUART_2_LPCG: u32 = 0x1480000;
pub const ADMA_LPUART_3_LPCG: u32 = 0x1490000;
pub const ADMA_LCD_LPCG: u32 = 0x1580000;
pub const ADMA_PWM_LPCG: u32 = 0x1590000;
pub const ADMA_LPI2C_0_LPCG: u32 = 0x1c00000;
pub const ADMA_LPI2C_1_LPCG: u32 = 0x1c10000;
pub const ADMA_LPI2C_2_LPCG: u32 = 0x1c20000;
pub const ADMA_LPI2C_3_LPCG: u32 = 0x1c30000;
pub const ADMA_ADC_0_LPCG: u32 = 0x1c80000;
pub const ADMA_FTM_0_LPCG: u32 = 0x1ca0000;
pub const ADMA_FTM_1_LPCG: u32 = 0x1cb0000;
pub const ADMA_FLEXCAN_0_LPCG: u32 = 0x1cd0000;
pub const ADMA_FLEXCAN_1_LPCG: u32 = 0x1ce0000;
pub const ADMA_FLEXCAN_2_LPCG: u32 = 0x1cf0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
