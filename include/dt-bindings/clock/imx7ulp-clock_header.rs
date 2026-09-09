/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 */

/* SCG1 */

pub const IMX7ULP_CLK_DUMMY: u32 = 0;
pub const IMX7ULP_CLK_ROSC: u32 = 1;
pub const IMX7ULP_CLK_SOSC: u32 = 2;
pub const IMX7ULP_CLK_FIRC: u32 = 3;
pub const IMX7ULP_CLK_SPLL_PRE_SEL: u32 = 4;
pub const IMX7ULP_CLK_SPLL_PRE_DIV: u32 = 5;
pub const IMX7ULP_CLK_SPLL: u32 = 6;
pub const IMX7ULP_CLK_SPLL_POST_DIV1: u32 = 7;
pub const IMX7ULP_CLK_SPLL_POST_DIV2: u32 = 8;
pub const IMX7ULP_CLK_SPLL_PFD0: u32 = 9;
pub const IMX7ULP_CLK_SPLL_PFD1: u32 = 10;
pub const IMX7ULP_CLK_SPLL_PFD2: u32 = 11;
pub const IMX7ULP_CLK_SPLL_PFD3: u32 = 12;
pub const IMX7ULP_CLK_SPLL_PFD_SEL: u32 = 13;
pub const IMX7ULP_CLK_SPLL_SEL: u32 = 14;
pub const IMX7ULP_CLK_APLL_PRE_SEL: u32 = 15;
pub const IMX7ULP_CLK_APLL_PRE_DIV: u32 = 16;
pub const IMX7ULP_CLK_APLL: u32 = 17;
pub const IMX7ULP_CLK_APLL_POST_DIV1: u32 = 18;
pub const IMX7ULP_CLK_APLL_POST_DIV2: u32 = 19;
pub const IMX7ULP_CLK_APLL_PFD0: u32 = 20;
pub const IMX7ULP_CLK_APLL_PFD1: u32 = 21;
pub const IMX7ULP_CLK_APLL_PFD2: u32 = 22;
pub const IMX7ULP_CLK_APLL_PFD3: u32 = 23;
pub const IMX7ULP_CLK_APLL_PFD_SEL: u32 = 24;
pub const IMX7ULP_CLK_APLL_SEL: u32 = 25;
pub const IMX7ULP_CLK_UPLL: u32 = 26;
pub const IMX7ULP_CLK_SYS_SEL: u32 = 27;
pub const IMX7ULP_CLK_CORE_DIV: u32 = 28;
pub const IMX7ULP_CLK_BUS_DIV: u32 = 29;
pub const IMX7ULP_CLK_PLAT_DIV: u32 = 30;
pub const IMX7ULP_CLK_DDR_SEL: u32 = 31;
pub const IMX7ULP_CLK_DDR_DIV: u32 = 32;
pub const IMX7ULP_CLK_NIC_SEL: u32 = 33;
pub const IMX7ULP_CLK_NIC0_DIV: u32 = 34;
pub const IMX7ULP_CLK_GPU_DIV: u32 = 35;
pub const IMX7ULP_CLK_NIC1_DIV: u32 = 36;
pub const IMX7ULP_CLK_NIC1_BUS_DIV: u32 = 37;
pub const IMX7ULP_CLK_NIC1_EXT_DIV: u32 = 38;
/* IMX7ULP_CLK_MIPI_PLL is unsupported and shouldn't be used in DT */
pub const IMX7ULP_CLK_MIPI_PLL: u32 = 39;
pub const IMX7ULP_CLK_SIRC: u32 = 40;
pub const IMX7ULP_CLK_SOSC_BUS_CLK: u32 = 41;
pub const IMX7ULP_CLK_FIRC_BUS_CLK: u32 = 42;
pub const IMX7ULP_CLK_SPLL_BUS_CLK: u32 = 43;
pub const IMX7ULP_CLK_HSRUN_SYS_SEL: u32 = 44;
pub const IMX7ULP_CLK_HSRUN_CORE_DIV: u32 = 45;

pub const IMX7ULP_CLK_CORE: u32 = 46;
pub const IMX7ULP_CLK_HSRUN_CORE: u32 = 47;

pub const IMX7ULP_CLK_SCG1_END: u32 = 48;

/* PCC2 */
pub const IMX7ULP_CLK_DMA1: u32 = 0;
pub const IMX7ULP_CLK_RGPIO2P1: u32 = 1;
pub const IMX7ULP_CLK_FLEXBUS: u32 = 2;
pub const IMX7ULP_CLK_SEMA42_1: u32 = 3;
pub const IMX7ULP_CLK_DMA_MUX1: u32 = 4;
pub const IMX7ULP_CLK_CAAM: u32 = 6;
pub const IMX7ULP_CLK_LPTPM4: u32 = 7;
pub const IMX7ULP_CLK_LPTPM5: u32 = 8;
pub const IMX7ULP_CLK_LPIT1: u32 = 9;
pub const IMX7ULP_CLK_LPSPI2: u32 = 10;
pub const IMX7ULP_CLK_LPSPI3: u32 = 11;
pub const IMX7ULP_CLK_LPI2C4: u32 = 12;
pub const IMX7ULP_CLK_LPI2C5: u32 = 13;
pub const IMX7ULP_CLK_LPUART4: u32 = 14;
pub const IMX7ULP_CLK_LPUART5: u32 = 15;
pub const IMX7ULP_CLK_FLEXIO1: u32 = 16;
pub const IMX7ULP_CLK_USB0: u32 = 17;
pub const IMX7ULP_CLK_USB1: u32 = 18;
pub const IMX7ULP_CLK_USB_PHY: u32 = 19;
pub const IMX7ULP_CLK_USB_PL301: u32 = 20;
pub const IMX7ULP_CLK_USDHC0: u32 = 21;
pub const IMX7ULP_CLK_USDHC1: u32 = 22;
pub const IMX7ULP_CLK_WDG1: u32 = 23;
pub const IMX7ULP_CLK_WDG2: u32 = 24;

pub const IMX7ULP_CLK_PCC2_END: u32 = 25;

/* PCC3 */
pub const IMX7ULP_CLK_LPTPM6: u32 = 0;
pub const IMX7ULP_CLK_LPTPM7: u32 = 1;
pub const IMX7ULP_CLK_LPI2C6: u32 = 2;
pub const IMX7ULP_CLK_LPI2C7: u32 = 3;
pub const IMX7ULP_CLK_LPUART6: u32 = 4;
pub const IMX7ULP_CLK_LPUART7: u32 = 5;
pub const IMX7ULP_CLK_VIU: u32 = 6;
pub const IMX7ULP_CLK_DSI: u32 = 7;
pub const IMX7ULP_CLK_LCDIF: u32 = 8;
pub const IMX7ULP_CLK_MMDC: u32 = 9;
pub const IMX7ULP_CLK_PCTLC: u32 = 10;
pub const IMX7ULP_CLK_PCTLD: u32 = 11;
pub const IMX7ULP_CLK_PCTLE: u32 = 12;
pub const IMX7ULP_CLK_PCTLF: u32 = 13;
pub const IMX7ULP_CLK_GPU3D: u32 = 14;
pub const IMX7ULP_CLK_GPU2D: u32 = 15;

pub const IMX7ULP_CLK_PCC3_END: u32 = 16;

/* SMC1 */
pub const IMX7ULP_CLK_ARM: u32 = 0;

pub const IMX7ULP_CLK_SMC1_END: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
