/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// Dependency: <dt-bindings/clock/sophgo,cv1800.h>

pub const CV1800_CLK_MAX: u32 = CLK_XTAL_AP + 1;
pub const CV1810_CLK_MAX: u32 = CLK_DISP_SRC_VIP + 1;

pub const REG_PLL_G2_CTRL: u32 = 0x800;
pub const REG_PLL_G2_STATUS: u32 = 0x804;
pub const REG_MIPIMPLL_CSR: u32 = 0x808;
pub const REG_A0PLL_CSR: u32 = 0x80C;
pub const REG_DISPPLL_CSR: u32 = 0x810;
pub const REG_CAM0PLL_CSR: u32 = 0x814;
pub const REG_CAM1PLL_CSR: u32 = 0x818;
pub const REG_PLL_G2_SSC_SYN_CTRL: u32 = 0x840;
pub const REG_A0PLL_SSC_SYN_CTRL: u32 = 0x850;
pub const REG_A0PLL_SSC_SYN_SET: u32 = 0x854;
pub const REG_A0PLL_SSC_SYN_SPAN: u32 = 0x858;
pub const REG_A0PLL_SSC_SYN_STEP: u32 = 0x85C;
pub const REG_DISPPLL_SSC_SYN_CTRL: u32 = 0x860;
pub const REG_DISPPLL_SSC_SYN_SET: u32 = 0x864;
pub const REG_DISPPLL_SSC_SYN_SPAN: u32 = 0x868;
pub const REG_DISPPLL_SSC_SYN_STEP: u32 = 0x86C;
pub const REG_CAM0PLL_SSC_SYN_CTRL: u32 = 0x870;
pub const REG_CAM0PLL_SSC_SYN_SET: u32 = 0x874;
pub const REG_CAM0PLL_SSC_SYN_SPAN: u32 = 0x878;
pub const REG_CAM0PLL_SSC_SYN_STEP: u32 = 0x87C;
pub const REG_CAM1PLL_SSC_SYN_CTRL: u32 = 0x880;
pub const REG_CAM1PLL_SSC_SYN_SET: u32 = 0x884;
pub const REG_CAM1PLL_SSC_SYN_SPAN: u32 = 0x888;
pub const REG_CAM1PLL_SSC_SYN_STEP: u32 = 0x88C;
pub const REG_APLL_FRAC_DIV_CTRL: u32 = 0x890;
pub const REG_APLL_FRAC_DIV_M: u32 = 0x894;
pub const REG_APLL_FRAC_DIV_N: u32 = 0x898;
pub const REG_MIPIMPLL_CLK_CSR: u32 = 0x8A0;
pub const REG_A0PLL_CLK_CSR: u32 = 0x8A4;
pub const REG_DISPPLL_CLK_CSR: u32 = 0x8A8;
pub const REG_CAM0PLL_CLK_CSR: u32 = 0x8AC;
pub const REG_CAM1PLL_CLK_CSR: u32 = 0x8B0;
pub const REG_CLK_CAM0_SRC_DIV: u32 = 0x8C0;
pub const REG_CLK_CAM1_SRC_DIV: u32 = 0x8C4;

/* top_pll_g6 */
pub const REG_PLL_G6_CTRL: u32 = 0x900;
pub const REG_PLL_G6_STATUS: u32 = 0x904;
pub const REG_MPLL_CSR: u32 = 0x908;
pub const REG_TPLL_CSR: u32 = 0x90C;
pub const REG_FPLL_CSR: u32 = 0x910;
pub const REG_PLL_G6_SSC_SYN_CTRL: u32 = 0x940;
pub const REG_DPLL_SSC_SYN_CTRL: u32 = 0x950;
pub const REG_DPLL_SSC_SYN_SET: u32 = 0x954;
pub const REG_DPLL_SSC_SYN_SPAN: u32 = 0x958;
pub const REG_DPLL_SSC_SYN_STEP: u32 = 0x95C;
pub const REG_MPLL_SSC_SYN_CTRL: u32 = 0x960;
pub const REG_MPLL_SSC_SYN_SET: u32 = 0x964;
pub const REG_MPLL_SSC_SYN_SPAN: u32 = 0x968;
pub const REG_MPLL_SSC_SYN_STEP: u32 = 0x96C;
pub const REG_TPLL_SSC_SYN_CTRL: u32 = 0x970;
pub const REG_TPLL_SSC_SYN_SET: u32 = 0x974;
pub const REG_TPLL_SSC_SYN_SPAN: u32 = 0x978;
pub const REG_TPLL_SSC_SYN_STEP: u32 = 0x97C;

/* clkgen */
pub const REG_CLK_EN_0: u32 = 0x000;
pub const REG_CLK_EN_1: u32 = 0x004;
pub const REG_CLK_EN_2: u32 = 0x008;
pub const REG_CLK_EN_3: u32 = 0x00C;
pub const REG_CLK_EN_4: u32 = 0x010;
pub const REG_CLK_SEL_0: u32 = 0x020;
pub const REG_CLK_BYP_0: u32 = 0x030;
pub const REG_CLK_BYP_1: u32 = 0x034;

pub const REG_DIV_CLK_A53_0: u32 = 0x040;
pub const REG_DIV_CLK_A53_1: u32 = 0x044;
pub const REG_DIV_CLK_CPU_AXI0: u32 = 0x048;
pub const REG_DIV_CLK_CPU_GIC: u32 = 0x050;
pub const REG_DIV_CLK_TPU: u32 = 0x054;
pub const REG_DIV_CLK_EMMC: u32 = 0x064;
pub const REG_DIV_CLK_EMMC_100K: u32 = 0x06C;
pub const REG_DIV_CLK_SD0: u32 = 0x070;
pub const REG_DIV_CLK_SD0_100K: u32 = 0x078;
pub const REG_DIV_CLK_SD1: u32 = 0x07C;
pub const REG_DIV_CLK_SD1_100K: u32 = 0x084;
pub const REG_DIV_CLK_SPI_NAND: u32 = 0x088;
pub const REG_DIV_CLK_ETH0_500M: u32 = 0x08C;
pub const REG_DIV_CLK_ETH1_500M: u32 = 0x090;
pub const REG_DIV_CLK_GPIO_DB: u32 = 0x094;
pub const REG_DIV_CLK_SDMA_AUD0: u32 = 0x098;
pub const REG_DIV_CLK_SDMA_AUD1: u32 = 0x09C;
pub const REG_DIV_CLK_SDMA_AUD2: u32 = 0x0A0;
pub const REG_DIV_CLK_SDMA_AUD3: u32 = 0x0A4;
pub const REG_DIV_CLK_CAM0_200: u32 = 0x0A8;
pub const REG_DIV_CLK_AXI4: u32 = 0x0B8;
pub const REG_DIV_CLK_AXI6: u32 = 0x0BC;
pub const REG_DIV_CLK_DSI_ESC: u32 = 0x0C4;
pub const REG_DIV_CLK_AXI_VIP: u32 = 0x0C8;
pub const REG_DIV_CLK_SRC_VIP_SYS_0: u32 = 0x0D0;
pub const REG_DIV_CLK_SRC_VIP_SYS_1: u32 = 0x0D8;
pub const REG_DIV_CLK_DISP_SRC_VIP: u32 = 0x0E0;
pub const REG_DIV_CLK_AXI_VIDEO_CODEC: u32 = 0x0E4;
pub const REG_DIV_CLK_VC_SRC0: u32 = 0x0EC;
pub const REG_DIV_CLK_1M: u32 = 0x0FC;
pub const REG_DIV_CLK_SPI: u32 = 0x100;
pub const REG_DIV_CLK_I2C: u32 = 0x104;
pub const REG_DIV_CLK_SRC_VIP_SYS_2: u32 = 0x110;
pub const REG_DIV_CLK_AUDSRC: u32 = 0x118;
pub const REG_DIV_CLK_PWM_SRC_0: u32 = 0x120;
pub const REG_DIV_CLK_AP_DEBUG: u32 = 0x128;
pub const REG_DIV_CLK_RTCSYS_SRC_0: u32 = 0x12C;
pub const REG_DIV_CLK_C906_0_0: u32 = 0x130;
pub const REG_DIV_CLK_C906_0_1: u32 = 0x134;
pub const REG_DIV_CLK_C906_1_0: u32 = 0x138;
pub const REG_DIV_CLK_C906_1_1: u32 = 0x13C;
pub const REG_DIV_CLK_SRC_VIP_SYS_3: u32 = 0x140;
pub const REG_DIV_CLK_SRC_VIP_SYS_4: u32 = 0x144;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
