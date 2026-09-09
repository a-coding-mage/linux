/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Hisilicon Limited.
 *
 * Author: Bintian Wang <bintian.wang@huawei.com>
 */

/* clk in Hi6220 AO (always on) controller */
pub const HI6220_NONE_CLOCK: u32 = 0;

/* fixed rate clocks */
pub const HI6220_REF32K: u32 = 1;
pub const HI6220_CLK_TCXO: u32 = 2;
pub const HI6220_MMC1_PAD: u32 = 3;
pub const HI6220_MMC2_PAD: u32 = 4;
pub const HI6220_MMC0_PAD: u32 = 5;
pub const HI6220_PLL_BBP: u32 = 6;
pub const HI6220_PLL_GPU: u32 = 7;
pub const HI6220_PLL1_DDR: u32 = 8;
pub const HI6220_PLL_SYS: u32 = 9;
pub const HI6220_PLL_SYS_MEDIA: u32 = 10;
pub const HI6220_DDR_SRC: u32 = 11;
pub const HI6220_PLL_MEDIA: u32 = 12;
pub const HI6220_PLL_DDR: u32 = 13;

/* fixed factor clocks */
pub const HI6220_300M: u32 = 14;
pub const HI6220_150M: u32 = 15;
pub const HI6220_PICOPHY_SRC: u32 = 16;
pub const HI6220_MMC0_SRC_SEL: u32 = 17;
pub const HI6220_MMC1_SRC_SEL: u32 = 18;
pub const HI6220_MMC2_SRC_SEL: u32 = 19;
pub const HI6220_VPU_CODEC: u32 = 20;
pub const HI6220_MMC0_SMP: u32 = 21;
pub const HI6220_MMC1_SMP: u32 = 22;
pub const HI6220_MMC2_SMP: u32 = 23;

/* gate clocks */
pub const HI6220_WDT0_PCLK: u32 = 24;
pub const HI6220_WDT1_PCLK: u32 = 25;
pub const HI6220_WDT2_PCLK: u32 = 26;
pub const HI6220_TIMER0_PCLK: u32 = 27;
pub const HI6220_TIMER1_PCLK: u32 = 28;
pub const HI6220_TIMER2_PCLK: u32 = 29;
pub const HI6220_TIMER3_PCLK: u32 = 30;
pub const HI6220_TIMER4_PCLK: u32 = 31;
pub const HI6220_TIMER5_PCLK: u32 = 32;
pub const HI6220_TIMER6_PCLK: u32 = 33;
pub const HI6220_TIMER7_PCLK: u32 = 34;
pub const HI6220_TIMER8_PCLK: u32 = 35;
pub const HI6220_UART0_PCLK: u32 = 36;
pub const HI6220_RTC0_PCLK: u32 = 37;
pub const HI6220_RTC1_PCLK: u32 = 38;
pub const HI6220_AO_NR_CLKS: u32 = 39;

/* clk in Hi6220 systrl */
/* gate clock */
pub const HI6220_MMC0_CLK: u32 = 1;
pub const HI6220_MMC0_CIUCLK: u32 = 2;
pub const HI6220_MMC1_CLK: u32 = 3;
pub const HI6220_MMC1_CIUCLK: u32 = 4;
pub const HI6220_MMC2_CLK: u32 = 5;
pub const HI6220_MMC2_CIUCLK: u32 = 6;
pub const HI6220_USBOTG_HCLK: u32 = 7;
pub const HI6220_CLK_PICOPHY: u32 = 8;
pub const HI6220_HIFI: u32 = 9;
pub const HI6220_DACODEC_PCLK: u32 = 10;
pub const HI6220_EDMAC_ACLK: u32 = 11;
pub const HI6220_CS_ATB: u32 = 12;
pub const HI6220_I2C0_CLK: u32 = 13;
pub const HI6220_I2C1_CLK: u32 = 14;
pub const HI6220_I2C2_CLK: u32 = 15;
pub const HI6220_I2C3_CLK: u32 = 16;
pub const HI6220_UART1_PCLK: u32 = 17;
pub const HI6220_UART2_PCLK: u32 = 18;
pub const HI6220_UART3_PCLK: u32 = 19;
pub const HI6220_UART4_PCLK: u32 = 20;
pub const HI6220_SPI_CLK: u32 = 21;
pub const HI6220_TSENSOR_CLK: u32 = 22;
pub const HI6220_MMU_CLK: u32 = 23;
pub const HI6220_HIFI_SEL: u32 = 24;
pub const HI6220_MMC0_SYSPLL: u32 = 25;
pub const HI6220_MMC1_SYSPLL: u32 = 26;
pub const HI6220_MMC2_SYSPLL: u32 = 27;
pub const HI6220_MMC0_SEL: u32 = 28;
pub const HI6220_MMC1_SEL: u32 = 29;
pub const HI6220_BBPPLL_SEL: u32 = 30;
pub const HI6220_MEDIA_PLL_SRC: u32 = 31;
pub const HI6220_MMC2_SEL: u32 = 32;
pub const HI6220_CS_ATB_SYSPLL: u32 = 33;

/* mux clocks */
pub const HI6220_MMC0_SRC: u32 = 34;
pub const HI6220_MMC0_SMP_IN: u32 = 35;
pub const HI6220_MMC1_SRC: u32 = 36;
pub const HI6220_MMC1_SMP_IN: u32 = 37;
pub const HI6220_MMC2_SRC: u32 = 38;
pub const HI6220_MMC2_SMP_IN: u32 = 39;
pub const HI6220_HIFI_SRC: u32 = 40;
pub const HI6220_UART1_SRC: u32 = 41;
pub const HI6220_UART2_SRC: u32 = 42;
pub const HI6220_UART3_SRC: u32 = 43;
pub const HI6220_UART4_SRC: u32 = 44;
pub const HI6220_MMC0_MUX0: u32 = 45;
pub const HI6220_MMC1_MUX0: u32 = 46;
pub const HI6220_MMC2_MUX0: u32 = 47;
pub const HI6220_MMC0_MUX1: u32 = 48;
pub const HI6220_MMC1_MUX1: u32 = 49;
pub const HI6220_MMC2_MUX1: u32 = 50;

/* divider clocks */
pub const HI6220_CLK_BUS: u32 = 51;
pub const HI6220_MMC0_DIV: u32 = 52;
pub const HI6220_MMC1_DIV: u32 = 53;
pub const HI6220_MMC2_DIV: u32 = 54;
pub const HI6220_HIFI_DIV: u32 = 55;
pub const HI6220_BBPPLL0_DIV: u32 = 56;
pub const HI6220_CS_DAPB: u32 = 57;
pub const HI6220_CS_ATB_DIV: u32 = 58;

/* gate clock */
pub const HI6220_DAPB_CLK: u32 = 59;
pub const HI6220_SYS_NR_CLKS: u32 = 60;

/* clk in Hi6220 media controller */
/* gate clocks */
pub const HI6220_DSI_PCLK: u32 = 1;
pub const HI6220_G3D_PCLK: u32 = 2;
pub const HI6220_ACLK_CODEC_VPU: u32 = 3;
pub const HI6220_ISP_SCLK: u32 = 4;
pub const HI6220_ADE_CORE: u32 = 5;
pub const HI6220_MED_MMU: u32 = 6;
pub const HI6220_CFG_CSI4PHY: u32 = 7;
pub const HI6220_CFG_CSI2PHY: u32 = 8;
pub const HI6220_ISP_SCLK_GATE: u32 = 9;
pub const HI6220_ISP_SCLK_GATE1: u32 = 10;
pub const HI6220_ADE_CORE_GATE: u32 = 11;
pub const HI6220_CODEC_VPU_GATE: u32 = 12;
pub const HI6220_MED_SYSPLL: u32 = 13;

/* mux clocks */
pub const HI6220_1440_1200: u32 = 14;
pub const HI6220_1000_1200: u32 = 15;
pub const HI6220_1000_1440: u32 = 16;

/* divider clocks */
pub const HI6220_CODEC_JPEG: u32 = 17;
pub const HI6220_ISP_SCLK_SRC: u32 = 18;
pub const HI6220_ISP_SCLK1: u32 = 19;
pub const HI6220_ADE_CORE_SRC: u32 = 20;
pub const HI6220_ADE_PIX_SRC: u32 = 21;
pub const HI6220_G3D_CLK: u32 = 22;
pub const HI6220_CODEC_VPU_SRC: u32 = 23;
pub const HI6220_MEDIA_NR_CLKS: u32 = 24;

/* clk in Hi6220 power controller */
/* gate clocks */
pub const HI6220_PLL_GPU_GATE: u32 = 1;
pub const HI6220_PLL1_DDR_GATE: u32 = 2;
pub const HI6220_PLL_DDR_GATE: u32 = 3;
pub const HI6220_PLL_MEDIA_GATE: u32 = 4;
pub const HI6220_PLL0_BBP_GATE: u32 = 5;

/* divider clocks */
pub const HI6220_DDRC_SRC: u32 = 6;
pub const HI6220_DDRC_AXI1: u32 = 7;
pub const HI6220_POWER_NR_CLKS: u32 = 8;

/* clk in Hi6220 acpu sctrl */
pub const HI6220_ACPU_SFT_AT_S: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
