// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Dávid Virág <virag.david003@gmail.com>
 * Author: Dávid Virág <virag.david003@gmail.com>
 *
 * Common Clock Framework support for Exynos7885 SoC.
 */

// External dependency: <linux/clk-provider.h>
// External dependency: <linux/of.h>
// External dependency: <linux/platform_device.h>

// External dependency: <dt-bindings/clock/exynos7885.h>

// External dependency: clk.h
// External dependency: clk-exynos-arm64.h

/* NOTE: Must be equal to the last clock ID increased by one */
const CLKS_NR_TOP: u32 = (CLK_MOUT_SHARED1_PLL + 1);
const CLKS_NR_CORE: u32 = (CLK_GOUT_TREX_P_CORE_PCLK_P_CORE + 1);
const CLKS_NR_PERI: u32 = (CLK_GOUT_WDT1_PCLK + 1);
const CLKS_NR_FSYS: u32 = (CLK_FSYS_USB30DRD_REF_CLK + 1);

/* ---- CMU_TOP ------------------------------------------------------------- */

/* Register Offset definitions for CMU_TOP (0x12060000) */
const PLL_LOCKTIME_PLL_SHARED0: u32 = 0x0000;
const PLL_LOCKTIME_PLL_SHARED1: u32 = 0x0004;
const PLL_CON0_PLL_SHARED0: u32 = 0x0100;
const PLL_CON0_PLL_SHARED1: u32 = 0x0120;
const CLK_CON_MUX_MUX_CLKCMU_CORE_BUS: u32 = 0x1014;
const CLK_CON_MUX_MUX_CLKCMU_CORE_CCI: u32 = 0x1018;
const CLK_CON_MUX_MUX_CLKCMU_CORE_G3D: u32 = 0x101c;
const CLK_CON_MUX_MUX_CLKCMU_FSYS_BUS: u32 = 0x1028;
const CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_CARD: u32 = 0x102c;
const CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_EMBD: u32 = 0x1030;
const CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_SDIO: u32 = 0x1034;
const CLK_CON_MUX_MUX_CLKCMU_FSYS_USB30DRD: u32 = 0x1038;
const CLK_CON_MUX_MUX_CLKCMU_PERI_BUS: u32 = 0x1058;
const CLK_CON_MUX_MUX_CLKCMU_PERI_SPI0: u32 = 0x105c;
const CLK_CON_MUX_MUX_CLKCMU_PERI_SPI1: u32 = 0x1060;
const CLK_CON_MUX_MUX_CLKCMU_PERI_UART0: u32 = 0x1064;
const CLK_CON_MUX_MUX_CLKCMU_PERI_UART1: u32 = 0x1068;
const CLK_CON_MUX_MUX_CLKCMU_PERI_UART2: u32 = 0x106c;
const CLK_CON_MUX_MUX_CLKCMU_PERI_USI0: u32 = 0x1070;
const CLK_CON_MUX_MUX_CLKCMU_PERI_USI1: u32 = 0x1074;
const CLK_CON_MUX_MUX_CLKCMU_PERI_USI2: u32 = 0x1078;
const CLK_CON_DIV_CLKCMU_CORE_BUS: u32 = 0x181c;
const CLK_CON_DIV_CLKCMU_CORE_CCI: u32 = 0x1820;
const CLK_CON_DIV_CLKCMU_CORE_G3D: u32 = 0x1824;
const CLK_CON_DIV_CLKCMU_FSYS_BUS: u32 = 0x1844;
const CLK_CON_DIV_CLKCMU_FSYS_MMC_CARD: u32 = 0x1848;
const CLK_CON_DIV_CLKCMU_FSYS_MMC_EMBD: u32 = 0x184c;
const CLK_CON_DIV_CLKCMU_FSYS_MMC_SDIO: u32 = 0x1850;
const CLK_CON_DIV_CLKCMU_FSYS_USB30DRD: u32 = 0x1854;
const CLK_CON_DIV_CLKCMU_PERI_BUS: u32 = 0x1874;
const CLK_CON_DIV_CLKCMU_PERI_SPI0: u32 = 0x1878;
const CLK_CON_DIV_CLKCMU_PERI_SPI1: u32 = 0x187c;
const CLK_CON_DIV_CLKCMU_PERI_UART0: u32 = 0x1880;
const CLK_CON_DIV_CLKCMU_PERI_UART1: u32 = 0x1884;
const CLK_CON_DIV_CLKCMU_PERI_UART2: u32 = 0x1888;
const CLK_CON_DIV_CLKCMU_PERI_USI0: u32 = 0x188c;
const CLK_CON_DIV_CLKCMU_PERI_USI1: u32 = 0x1890;
const CLK_CON_DIV_CLKCMU_PERI_USI2: u32 = 0x1894;
const CLK_CON_DIV_PLL_SHARED0_DIV2: u32 = 0x189c;
const CLK_CON_DIV_PLL_SHARED0_DIV3: u32 = 0x18a0;
const CLK_CON_DIV_PLL_SHARED0_DIV4: u32 = 0x18a4;
const CLK_CON_DIV_PLL_SHARED0_DIV5: u32 = 0x18a8;
const CLK_CON_DIV_PLL_SHARED1_DIV2: u32 = 0x18ac;
const CLK_CON_DIV_PLL_SHARED1_DIV3: u32 = 0x18b0;
const CLK_CON_DIV_PLL_SHARED1_DIV4: u32 = 0x18b4;
const CLK_CON_GAT_GATE_CLKCMUC_PERI_UART1: u32 = 0x2004;
const CLK_CON_GAT_GATE_CLKCMU_CORE_BUS: u32 = 0x201c;
const CLK_CON_GAT_GATE_CLKCMU_CORE_CCI: u32 = 0x2020;
const CLK_CON_GAT_GATE_CLKCMU_CORE_G3D: u32 = 0x2024;
const CLK_CON_GAT_GATE_CLKCMU_FSYS_BUS: u32 = 0x2044;
const CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_CARD: u32 = 0x2048;
const CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_EMBD: u32 = 0x204c;
const CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_SDIO: u32 = 0x2050;
const CLK_CON_GAT_GATE_CLKCMU_FSYS_USB30DRD: u32 = 0x2054;
const CLK_CON_GAT_GATE_CLKCMU_PERI_BUS: u32 = 0x207c;
const CLK_CON_GAT_GATE_CLKCMU_PERI_SPI0: u32 = 0x2080;
const CLK_CON_GAT_GATE_CLKCMU_PERI_SPI1: u32 = 0x2084;
const CLK_CON_GAT_GATE_CLKCMU_PERI_UART0: u32 = 0x2088;
const CLK_CON_GAT_GATE_CLKCMU_PERI_UART2: u32 = 0x208c;
const CLK_CON_GAT_GATE_CLKCMU_PERI_USI0: u32 = 0x2090;
const CLK_CON_GAT_GATE_CLKCMU_PERI_USI1: u32 = 0x2094;
const CLK_CON_GAT_GATE_CLKCMU_PERI_USI2: u32 = 0x2098;

static top_clk_regs: &[u32] = &[
	PLL_LOCKTIME_PLL_SHARED0,
	PLL_LOCKTIME_PLL_SHARED1,
	PLL_CON0_PLL_SHARED0,
	PLL_CON0_PLL_SHARED1,
	CLK_CON_MUX_MUX_CLKCMU_CORE_BUS,
	CLK_CON_MUX_MUX_CLKCMU_CORE_CCI,
	CLK_CON_MUX_MUX_CLKCMU_CORE_G3D,
	CLK_CON_MUX_MUX_CLKCMU_FSYS_BUS,
	CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_CARD,
	CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_EMBD,
	CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_SDIO,
	CLK_CON_MUX_MUX_CLKCMU_FSYS_USB30DRD,
	CLK_CON_MUX_MUX_CLKCMU_PERI_BUS,
	CLK_CON_MUX_MUX_CLKCMU_PERI_SPI0,
	CLK_CON_MUX_MUX_CLKCMU_PERI_SPI1,
	CLK_CON_MUX_MUX_CLKCMU_PERI_UART0,
	CLK_CON_MUX_MUX_CLKCMU_PERI_UART1,
	CLK_CON_MUX_MUX_CLKCMU_PERI_UART2,
	CLK_CON_MUX_MUX_CLKCMU_PERI_USI0,
	CLK_CON_MUX_MUX_CLKCMU_PERI_USI1,
	CLK_CON_MUX_MUX_CLKCMU_PERI_USI2,
	CLK_CON_DIV_CLKCMU_CORE_BUS,
	CLK_CON_DIV_CLKCMU_CORE_CCI,
	CLK_CON_DIV_CLKCMU_CORE_G3D,
	CLK_CON_DIV_CLKCMU_FSYS_BUS,
	CLK_CON_DIV_CLKCMU_FSYS_MMC_CARD,
	CLK_CON_DIV_CLKCMU_FSYS_MMC_EMBD,
	CLK_CON_DIV_CLKCMU_FSYS_MMC_SDIO,
	CLK_CON_DIV_CLKCMU_FSYS_USB30DRD,
	CLK_CON_DIV_CLKCMU_PERI_BUS,
	CLK_CON_DIV_CLKCMU_PERI_SPI0,
	CLK_CON_DIV_CLKCMU_PERI_SPI1,
	CLK_CON_DIV_CLKCMU_PERI_UART0,
	CLK_CON_DIV_CLKCMU_PERI_UART1,
	CLK_CON_DIV_CLKCMU_PERI_UART2,
	CLK_CON_DIV_CLKCMU_PERI_USI0,
	CLK_CON_DIV_CLKCMU_PERI_USI1,
	CLK_CON_DIV_CLKCMU_PERI_USI2,
	CLK_CON_DIV_PLL_SHARED0_DIV2,
	CLK_CON_DIV_PLL_SHARED0_DIV3,
	CLK_CON_DIV_PLL_SHARED0_DIV4,
	CLK_CON_DIV_PLL_SHARED0_DIV5,
	CLK_CON_DIV_PLL_SHARED1_DIV2,
	CLK_CON_DIV_PLL_SHARED1_DIV3,
	CLK_CON_DIV_PLL_SHARED1_DIV4,
	CLK_CON_GAT_GATE_CLKCMUC_PERI_UART1,
	CLK_CON_GAT_GATE_CLKCMU_CORE_BUS,
	CLK_CON_GAT_GATE_CLKCMU_CORE_CCI,
	CLK_CON_GAT_GATE_CLKCMU_CORE_G3D,
	CLK_CON_GAT_GATE_CLKCMU_FSYS_BUS,
	CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_CARD,
	CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_EMBD,
	CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_SDIO,
	CLK_CON_GAT_GATE_CLKCMU_FSYS_USB30DRD,
	CLK_CON_GAT_GATE_CLKCMU_PERI_BUS,
	CLK_CON_GAT_GATE_CLKCMU_PERI_SPI0,
	CLK_CON_GAT_GATE_CLKCMU_PERI_SPI1,
	CLK_CON_GAT_GATE_CLKCMU_PERI_UART0,
	CLK_CON_GAT_GATE_CLKCMU_PERI_UART2,
	CLK_CON_GAT_GATE_CLKCMU_PERI_USI0,
	CLK_CON_GAT_GATE_CLKCMU_PERI_USI1,
	CLK_CON_GAT_GATE_CLKCMU_PERI_USI2,
];

static top_pll_clks: &[samsung_pll_clock] = &[
	PLL(pll_1417x, CLK_FOUT_SHARED0_PLL, "fout_shared0_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED0, PLL_CON0_PLL_SHARED0,
	    NULL),
	PLL(pll_1417x, CLK_FOUT_SHARED1_PLL, "fout_shared1_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED1, PLL_CON0_PLL_SHARED1,
	    NULL),
};

/* List of parent clocks for Muxes in CMU_TOP */
static mout_shared0_pll_p: &[&str] = &[ "oscclk", "fout_shared0_pll" ];
static mout_shared1_pll_p: &[&str] = &[ "oscclk", "fout_shared1_pll" ];

/* List of parent clocks for Muxes in CMU_TOP: for CMU_CORE */
static mout_core_bus_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				    "dout_shared0_div3", "dout_shared0_div3" ];
static mout_core_cci_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				    "dout_shared0_div3", "dout_shared0_div3" ];
static mout_core_g3d_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				    "dout_shared0_div3", "dout_shared0_div3" ];

/* List of parent clocks for Muxes in CMU_TOP: for CMU_PERI */
static mout_peri_bus_p: &[&str] = &[ "dout_shared0_div4", "dout_shared1_div4" ];
static mout_peri_spi0_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_spi1_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_uart0_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_uart1_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_uart2_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_usi0_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_usi1_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];
static mout_peri_usi2_p: &[&str] = &[ "oscclk", "dout_shared0_div4" ];

/* List of parent clocks for Muxes in CMU_TOP: for CMU_FSYS */
static mout_fsys_bus_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2" ];
static mout_fsys_mmc_card_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2" ];
static mout_fsys_mmc_embd_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2" ];
static mout_fsys_mmc_sdio_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2" ];
static mout_fsys_usb30drd_p: &[&str] = &[ "dout_shared0_div4", "dout_shared1_div4" ];

static top_mux_clks: &[samsung_mux_clock] = &[
	/* TOP */
	MUX(CLK_MOUT_SHARED0_PLL, "mout_shared0_pll", mout_shared0_pll_p,
	    PLL_CON0_PLL_SHARED0, 4, 1),
	MUX(CLK_MOUT_SHARED1_PLL, "mout_shared1_pll", mout_shared1_pll_p,
	    PLL_CON0_PLL_SHARED1, 4, 1),

	/* CORE */
	MUX(CLK_MOUT_CORE_BUS, "mout_core_bus", mout_core_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_CORE_BUS, 0, 2),
	MUX(CLK_MOUT_CORE_CCI, "mout_core_cci", mout_core_cci_p,
	    CLK_CON_MUX_MUX_CLKCMU_CORE_CCI, 0, 2),
	MUX(CLK_MOUT_CORE_G3D, "mout_core_g3d", mout_core_g3d_p,
	    CLK_CON_MUX_MUX_CLKCMU_CORE_G3D, 0, 2),

	/* PERI */
	MUX(CLK_MOUT_PERI_BUS, "mout_peri_bus", mout_peri_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_BUS, 0, 1),
	MUX(CLK_MOUT_PERI_SPI0, "mout_peri_spi0", mout_peri_spi0_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_SPI0, 0, 1),
	MUX(CLK_MOUT_PERI_SPI1, "mout_peri_spi1", mout_peri_spi1_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_SPI1, 0, 1),
	MUX(CLK_MOUT_PERI_UART0, "mout_peri_uart0", mout_peri_uart0_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_UART0, 0, 1),
	MUX(CLK_MOUT_PERI_UART1, "mout_peri_uart1", mout_peri_uart1_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_UART1, 0, 1),
	MUX(CLK_MOUT_PERI_UART2, "mout_peri_uart2", mout_peri_uart2_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_UART2, 0, 1),
	MUX(CLK_MOUT_PERI_USI0, "mout_peri_usi0", mout_peri_usi0_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_USI0, 0, 1),
	MUX(CLK_MOUT_PERI_USI1, "mout_peri_usi1", mout_peri_usi1_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_USI1, 0, 1),
	MUX(CLK_MOUT_PERI_USI2, "mout_peri_usi2", mout_peri_usi2_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERI_USI2, 0, 1),

	/* FSYS */
	MUX(CLK_MOUT_FSYS_BUS, "mout_fsys_bus", mout_fsys_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS_BUS, 0, 1),
	MUX(CLK_MOUT_FSYS_MMC_CARD, "mout_fsys_mmc_card", mout_fsys_mmc_card_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_CARD, 0, 1),
	MUX(CLK_MOUT_FSYS_MMC_EMBD, "mout_fsys_mmc_embd", mout_fsys_mmc_embd_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_EMBD, 0, 1),
	MUX(CLK_MOUT_FSYS_MMC_SDIO, "mout_fsys_mmc_sdio", mout_fsys_mmc_sdio_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS_MMC_SDIO, 0, 1),
	MUX(CLK_MOUT_FSYS_USB30DRD, "mout_fsys_usb30drd", mout_fsys_usb30drd_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS_USB30DRD, 0, 1),
];

static top_div_clks: &[samsung_div_clock] = &[
	/* TOP */
	DIV(CLK_DOUT_SHARED0_DIV2, "dout_shared0_div2", "mout_shared0_pll",
	    CLK_CON_DIV_PLL_SHARED0_DIV2, 0, 1),
	DIV(CLK_DOUT_SHARED0_DIV3, "dout_shared0_div3", "mout_shared0_pll",
	    CLK_CON_DIV_PLL_SHARED0_DIV3, 0, 2),
	DIV(CLK_DOUT_SHARED0_DIV4, "dout_shared0_div4", "dout_shared0_div2",
	    CLK_CON_DIV_PLL_SHARED0_DIV4, 0, 1),
	DIV(CLK_DOUT_SHARED0_DIV5, "dout_shared0_div5", "mout_shared0_pll",
	    CLK_CON_DIV_PLL_SHARED0_DIV5, 0, 3),
	DIV(CLK_DOUT_SHARED1_DIV2, "dout_shared1_div2", "mout_shared1_pll",
	    CLK_CON_DIV_PLL_SHARED1_DIV2, 0, 1),
	DIV(CLK_DOUT_SHARED1_DIV3, "dout_shared1_div3", "mout_shared1_pll",
	    CLK_CON_DIV_PLL_SHARED1_DIV3, 0, 2),
	DIV(CLK_DOUT_SHARED1_DIV4, "dout_shared1_div4", "dout_shared1_div2",
	    CLK_CON_DIV_PLL_SHARED1_DIV4, 0, 1),

	/* CORE */
	DIV(CLK_DOUT_CORE_BUS, "dout_core_bus", "gout_core_bus",
	    CLK_CON_DIV_CLKCMU_CORE_BUS, 0, 3),
	DIV(CLK_DOUT_CORE_CCI, "dout_core_cci", "gout_core_cci",
	    CLK_CON_DIV_CLKCMU_CORE_CCI, 0, 3),
	DIV(CLK_DOUT_CORE_G3D, "dout_core_g3d", "gout_core_g3d",
	    CLK_CON_DIV_CLKCMU_CORE_G3D, 0, 3),

	/* PERI */
	DIV(CLK_DOUT_PERI_BUS, "dout_peri_bus", "gout_peri_bus",
	    CLK_CON_DIV_CLKCMU_PERI_BUS, 0, 4),
	DIV(CLK_DOUT_PERI_SPI0, "dout_peri_spi0", "gout_peri_spi0",
	    CLK_CON_DIV_CLKCMU_PERI_SPI0, 0, 6),
	DIV(CLK_DOUT_PERI_SPI1, "dout_peri_spi1", "gout_peri_spi1",
	    CLK_CON_DIV_CLKCMU_PERI_SPI1, 0, 6),
	DIV(CLK_DOUT_PERI_UART0, "dout_peri_uart0", "gout_peri_uart0",
	    CLK_CON_DIV_CLKCMU_PERI_UART0, 0, 4),
	DIV(CLK_DOUT_PERI_UART1, "dout_peri_uart1", "gout_peri_uart1",
	    CLK_CON_DIV_CLKCMU_PERI_UART1, 0, 4),
	DIV(CLK_DOUT_PERI_UART2, "dout_peri_uart2", "gout_peri_uart2",
	    CLK_CON_DIV_CLKCMU_PERI_UART2, 0, 4),
	DIV(CLK_DOUT_PERI_USI0, "dout_peri_usi0", "gout_peri_usi0",
	    CLK_CON_DIV_CLKCMU_PERI_USI0, 0, 4),
	DIV(CLK_DOUT_PERI_USI1, "dout_peri_usi1", "gout_peri_usi1",
	    CLK_CON_DIV_CLKCMU_PERI_USI1, 0, 4),
	DIV(CLK_DOUT_PERI_USI2, "dout_peri_usi2", "gout_peri_usi2",
	    CLK_CON_DIV_CLKCMU_PERI_USI2, 0, 4),

	/* FSYS */
	DIV(CLK_DOUT_FSYS_BUS, "dout_fsys_bus", "gout_fsys_bus",
	    CLK_CON_DIV_CLKCMU_FSYS_BUS, 0, 4),
	DIV(CLK_DOUT_FSYS_MMC_CARD, "dout_fsys_mmc_card", "gout_fsys_mmc_card",
	    CLK_CON_DIV_CLKCMU_FSYS_MMC_CARD, 0, 9),
	DIV(CLK_DOUT_FSYS_MMC_EMBD, "dout_fsys_mmc_embd", "gout_fsys_mmc_embd",
	    CLK_CON_DIV_CLKCMU_FSYS_MMC_EMBD, 0, 9),
	DIV(CLK_DOUT_FSYS_MMC_SDIO, "dout_fsys_mmc_sdio", "gout_fsys_mmc_sdio",
	    CLK_CON_DIV_CLKCMU_FSYS_MMC_SDIO, 0, 9),
	DIV(CLK_DOUT_FSYS_USB30DRD, "dout_fsys_usb30drd", "gout_fsys_usb30drd",
	    CLK_CON_DIV_CLKCMU_FSYS_USB30DRD, 0, 4),
];

static top_gate_clks: &[samsung_gate_clock] = &[
	/* CORE */
	GATE(CLK_GOUT_CORE_BUS, "gout_core_bus", "mout_core_bus",
	     CLK_CON_GAT_GATE_CLKCMU_CORE_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CORE_CCI, "gout_core_cci", "mout_core_cci",
	     CLK_CON_GAT_GATE_CLKCMU_CORE_CCI, 21, 0, 0),
	GATE(CLK_GOUT_CORE_G3D, "gout_core_g3d", "mout_core_g3d",
	     CLK_CON_GAT_GATE_CLKCMU_CORE_G3D, 21, 0, 0),

	/* PERI */
	GATE(CLK_GOUT_PERI_BUS, "gout_peri_bus", "mout_peri_bus",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_BUS, 21, 0, 0),
	GATE(CLK_GOUT_PERI_SPI0, "gout_peri_spi0", "mout_peri_spi0",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_SPI0, 21, 0, 0),
	GATE(CLK_GOUT_PERI_SPI1, "gout_peri_spi1", "mout_peri_spi1",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_SPI1, 21, 0, 0),
	GATE(CLK_GOUT_PERI_UART0, "gout_peri_uart0", "mout_peri_uart0",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_UART0, 21, 0, 0),
	GATE(CLK_GOUT_PERI_UART1, "gout_peri_uart1", "mout_peri_uart1",
	     CLK_CON_GAT_GATE_CLKCMUC_PERI_UART1, 21, 0, 0),
	GATE(CLK_GOUT_PERI_UART2, "gout_peri_uart2", "mout_peri_uart2",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_UART2, 21, 0, 0),
	GATE(CLK_GOUT_PERI_USI0, "gout_peri_usi0", "mout_peri_usi0",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_USI0, 21, 0, 0),
	GATE(CLK_GOUT_PERI_USI1, "gout_peri_usi1", "mout_peri_usi1",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_USI1, 21, 0, 0),
	GATE(CLK_GOUT_PERI_USI2, "gout_peri_usi2", "mout_peri_usi2",
	     CLK_CON_GAT_GATE_CLKCMU_PERI_USI2, 21, 0, 0),

	/* FSYS */
	GATE(CLK_GOUT_FSYS_BUS, "gout_fsys_bus", "mout_fsys_bus",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS_BUS, 21, 0, 0),
	GATE(CLK_GOUT_FSYS_MMC_CARD, "gout_fsys_mmc_card", "mout_fsys_mmc_card",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_CARD, 21, 0, 0),
	GATE(CLK_GOUT_FSYS_MMC_EMBD, "gout_fsys_mmc_embd", "mout_fsys_mmc_embd",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_EMBD, 21, 0, 0),
	GATE(CLK_GOUT_FSYS_MMC_SDIO, "gout_fsys_mmc_sdio", "mout_fsys_mmc_sdio",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS_MMC_SDIO, 21, 0, 0),
	GATE(CLK_GOUT_FSYS_USB30DRD, "gout_fsys_usb30drd", "mout_fsys_usb30drd",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS_USB30DRD, 21, 0, 0),
];

static top_cmu_info: samsung_cmu_info = samsung_cmu_info {
	pll_clks: top_pll_clks,
	nr_pll_clks: ARRAY_SIZE(top_pll_clks),
	mux_clks: top_mux_clks,
	nr_mux_clks: ARRAY_SIZE(top_mux_clks),
	div_clks: top_div_clks,
	nr_div_clks: ARRAY_SIZE(top_div_clks),
	gate_clks: top_gate_clks,
	nr_gate_clks: ARRAY_SIZE(top_gate_clks),
	nr_clk_ids: CLKS_NR_TOP,
	clk_regs: top_clk_regs,
	nr_clk_regs: ARRAY_SIZE(top_clk_regs),
];

unsafe fn exynos7885_cmu_top_init(np: *mut device_node)
{
	exynos_arm64_register_cmu(NULL, np, &top_cmu_info);
}

/* Register CMU_TOP early, as it's a dependency for other early domains */
clk_of_declare!(exynos7885_cmu_top, "samsung,exynos7885-cmu-top",
	       exynos7885_cmu_top_init);

/* ---- CMU_PERI ------------------------------------------------------------ */

/* Register Offset definitions for CMU_PERI (0x10010000) */
const PLL_CON0_MUX_CLKCMU_PERI_BUS_USER: u32 = 0x0100;
const PLL_CON0_MUX_CLKCMU_PERI_SPI0_USER: u32 = 0x0120;
const PLL_CON0_MUX_CLKCMU_PERI_SPI1_USER: u32 = 0x0140;
const PLL_CON0_MUX_CLKCMU_PERI_UART0_USER: u32 = 0x0160;
const PLL_CON0_MUX_CLKCMU_PERI_UART1_USER: u32 = 0x0180;
const PLL_CON0_MUX_CLKCMU_PERI_UART2_USER: u32 = 0x01a0;
const PLL_CON0_MUX_CLKCMU_PERI_USI0_USER: u32 = 0x01c0;
const PLL_CON0_MUX_CLKCMU_PERI_USI1_USER: u32 = 0x01e0;
const PLL_CON0_MUX_CLKCMU_PERI_USI2_USER: u32 = 0x0200;
const CLK_CON_GAT_GOUT_PERI_GPIO_TOP_PCLK: u32 = 0x2024;
const CLK_CON_GAT_GOUT_PERI_HSI2C_0_PCLK: u32 = 0x2028;
const CLK_CON_GAT_GOUT_PERI_HSI2C_1_PCLK: u32 = 0x202c;
const CLK_CON_GAT_GOUT_PERI_HSI2C_2_PCLK: u32 = 0x2030;
const CLK_CON_GAT_GOUT_PERI_HSI2C_3_PCLK: u32 = 0x2034;
const CLK_CON_GAT_GOUT_PERI_I2C_0_PCLK: u32 = 0x2038;
const CLK_CON_GAT_GOUT_PERI_I2C_1_PCLK: u32 = 0x203c;
const CLK_CON_GAT_GOUT_PERI_I2C_2_PCLK: u32 = 0x2040;
const CLK_CON_GAT_GOUT_PERI_I2C_3_PCLK: u32 = 0x2044;
const CLK_CON_GAT_GOUT_PERI_I2C_4_PCLK: u32 = 0x2048;
const CLK_CON_GAT_GOUT_PERI_I2C_5_PCLK: u32 = 0x204c;
const CLK_CON_GAT_GOUT_PERI_I2C_6_PCLK: u32 = 0x2050;
const CLK_CON_GAT_GOUT_PERI_I2C_7_PCLK: u32 = 0x2054;
const CLK_CON_GAT_GOUT_PERI_PWM_MOTOR_PCLK: u32 = 0x2058;
const CLK_CON_GAT_GOUT_PERI_SPI_0_PCLK: u32 = 0x205c;
const CLK_CON_GAT_GOUT_PERI_SPI_0_EXT_CLK: u32 = 0x2060;
const CLK_CON_GAT_GOUT_PERI_SPI_1_PCLK: u32 = 0x2064;
const CLK_CON_GAT_GOUT_PERI_SPI_1_EXT_CLK: u32 = 0x2068;
const CLK_CON_GAT_GOUT_PERI_UART_0_EXT_UCLK: u32 = 0x206c;
const CLK_CON_GAT_GOUT_PERI_UART_0_PCLK: u32 = 0x2070;
const CLK_CON_GAT_GOUT_PERI_UART_1_EXT_UCLK: u32 = 0x2074;
const CLK_CON_GAT_GOUT_PERI_UART_1_PCLK: u32 = 0x2078;
const CLK_CON_GAT_GOUT_PERI_UART_2_EXT_UCLK: u32 = 0x207c;
const CLK_CON_GAT_GOUT_PERI_UART_2_PCLK: u32 = 0x2080;
const CLK_CON_GAT_GOUT_PERI_USI0_PCLK: u32 = 0x2084;
const CLK_CON_GAT_GOUT_PERI_USI0_SCLK: u32 = 0x2088;
const CLK_CON_GAT_GOUT_PERI_USI1_PCLK: u32 = 0x208c;
const CLK_CON_GAT_GOUT_PERI_USI1_SCLK: u32 = 0x2090;
const CLK_CON_GAT_GOUT_PERI_USI2_PCLK: u32 = 0x2094;
const CLK_CON_GAT_GOUT_PERI_USI2_SCLK: u32 = 0x2098;
const CLK_CON_GAT_GOUT_PERI_MCT_PCLK: u32 = 0x20a0;
const CLK_CON_GAT_GOUT_PERI_SYSREG_PERI_PCLK: u32 = 0x20b0;
const CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER0_PCLK: u32 = 0x20b4;
const CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER1_PCLK: u32 = 0x20b8;

static peri_clk_regs: &[u32] = &[
	PLL_CON0_MUX_CLKCMU_PERI_BUS_USER,
	PLL_CON0_MUX_CLKCMU_PERI_SPI0_USER,
	PLL_CON0_MUX_CLKCMU_PERI_SPI1_USER,
	PLL_CON0_MUX_CLKCMU_PERI_UART0_USER,
	PLL_CON0_MUX_CLKCMU_PERI_UART1_USER,
	PLL_CON0_MUX_CLKCMU_PERI_UART2_USER,
	PLL_CON0_MUX_CLKCMU_PERI_USI0_USER,
	PLL_CON0_MUX_CLKCMU_PERI_USI1_USER,
	PLL_CON0_MUX_CLKCMU_PERI_USI2_USER,
	CLK_CON_GAT_GOUT_PERI_GPIO_TOP_PCLK,
	CLK_CON_GAT_GOUT_PERI_HSI2C_0_PCLK,
	CLK_CON_GAT_GOUT_PERI_HSI2C_1_PCLK,
	CLK_CON_GAT_GOUT_PERI_HSI2C_2_PCLK,
	CLK_CON_GAT_GOUT_PERI_HSI2C_3_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_0_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_1_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_2_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_3_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_4_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_5_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_6_PCLK,
	CLK_CON_GAT_GOUT_PERI_I2C_7_PCLK,
	CLK_CON_GAT_GOUT_PERI_PWM_MOTOR_PCLK,
	CLK_CON_GAT_GOUT_PERI_SPI_0_PCLK,
	CLK_CON_GAT_GOUT_PERI_SPI_0_EXT_CLK,
	CLK_CON_GAT_GOUT_PERI_SPI_1_PCLK,
	CLK_CON_GAT_GOUT_PERI_SPI_1_EXT_CLK,
	CLK_CON_GAT_GOUT_PERI_UART_0_EXT_UCLK,
	CLK_CON_GAT_GOUT_PERI_UART_0_PCLK,
	CLK_CON_GAT_GOUT_PERI_UART_1_EXT_UCLK,
	CLK_CON_GAT_GOUT_PERI_UART_1_PCLK,
	CLK_CON_GAT_GOUT_PERI_UART_2_EXT_UCLK,
	CLK_CON_GAT_GOUT_PERI_UART_2_PCLK,
	CLK_CON_GAT_GOUT_PERI_USI0_PCLK,
	CLK_CON_GAT_GOUT_PERI_USI0_SCLK,
	CLK_CON_GAT_GOUT_PERI_USI1_PCLK,
	CLK_CON_GAT_GOUT_PERI_USI1_SCLK,
	CLK_CON_GAT_GOUT_PERI_USI2_PCLK,
	CLK_CON_GAT_GOUT_PERI_USI2_SCLK,
	CLK_CON_GAT_GOUT_PERI_MCT_PCLK,
	CLK_CON_GAT_GOUT_PERI_SYSREG_PERI_PCLK,
	CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER0_PCLK,
	CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER1_PCLK,
};

/* List of parent clocks for Muxes in CMU_PERI */
static mout_peri_bus_user_p: &[&str] = &[ "oscclk", "dout_peri_bus" ];
static mout_peri_spi0_user_p: &[&str] = &[ "oscclk", "dout_peri_spi0" ];
static mout_peri_spi1_user_p: &[&str] = &[ "oscclk", "dout_peri_spi1" ];
static mout_peri_uart0_user_p: &[&str] = &[ "oscclk", "dout_peri_uart0" ];
static mout_peri_uart1_user_p: &[&str] = &[ "oscclk", "dout_peri_uart1" ];
static mout_peri_uart2_user_p: &[&str] = &[ "oscclk", "dout_peri_uart2" ];
static mout_peri_usi0_user_p: &[&str] = &[ "oscclk", "dout_peri_usi0" ];
static mout_peri_usi1_user_p: &[&str] = &[ "oscclk", "dout_peri_usi1" ];
static mout_peri_usi2_user_p: &[&str] = &[ "oscclk", "dout_peri_usi2" ];

static peri_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERI_BUS_USER, "mout_peri_bus_user", mout_peri_bus_user_p,
	    PLL_CON0_MUX_CLKCMU_PERI_BUS_USER, 4, 1),
	MUX(CLK_MOUT_PERI_SPI0_USER, "mout_peri_spi0_user", mout_peri_spi0_user_p,
	    PLL_CON0_MUX_CLKCMU_PERI_SPI0_USER, 4, 1),
	MUX(CLK_MOUT_PERI_SPI1_USER, "mout_peri_spi1_user", mout_peri_spi1_user_p,
	    PLL_CON0_MUX_CLKCMU_PERI_SPI1_USER, 4, 1),
	MUX(CLK_MOUT_PERI_UART0_USER, "mout_peri_uart0_user",
	    mout_peri_uart0_user_p, PLL_CON0_MUX_CLKCMU_PERI_UART0_USER, 4, 1),
	MUX(CLK_MOUT_PERI_UART1_USER, "mout_peri_uart1_user",
	    mout_peri_uart1_user_p, PLL_CON0_MUX_CLKCMU_PERI_UART1_USER, 4, 1),
	MUX(CLK_MOUT_PERI_UART2_USER, "mout_peri_uart2_user",
	    mout_peri_uart2_user_p, PLL_CON0_MUX_CLKCMU_PERI_UART2_USER, 4, 1),
	MUX(CLK_MOUT_PERI_USI0_USER, "mout_peri_usi0_user",
	    mout_peri_usi0_user_p, PLL_CON0_MUX_CLKCMU_PERI_USI0_USER, 4, 1),
	MUX(CLK_MOUT_PERI_USI1_USER, "mout_peri_usi1_user",
	    mout_peri_usi1_user_p, PLL_CON0_MUX_CLKCMU_PERI_USI1_USER, 4, 1),
	MUX(CLK_MOUT_PERI_USI2_USER, "mout_peri_usi2_user",
	    mout_peri_usi2_user_p, PLL_CON0_MUX_CLKCMU_PERI_USI2_USER, 4, 1),
];

static peri_gate_clks: &[samsung_gate_clock] = &[
	/* TODO: Should be enabled in GPIO driver (or made CLK_IS_CRITICAL) */
	GATE(CLK_GOUT_GPIO_TOP_PCLK, "gout_gpio_top_pclk",
	     "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_GPIO_TOP_PCLK, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_HSI2C0_PCLK, "gout_hsi2c0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_HSI2C_0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_HSI2C1_PCLK, "gout_hsi2c1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_HSI2C_1_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_HSI2C2_PCLK, "gout_hsi2c2_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_HSI2C_2_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_HSI2C3_PCLK, "gout_hsi2c3_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_HSI2C_3_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C0_PCLK, "gout_i2c0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C1_PCLK, "gout_i2c1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_1_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C2_PCLK, "gout_i2c2_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_2_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C3_PCLK, "gout_i2c3_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_3_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C4_PCLK, "gout_i2c4_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_4_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C5_PCLK, "gout_i2c5_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_5_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C6_PCLK, "gout_i2c6_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_6_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_I2C7_PCLK, "gout_i2c7_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_I2C_7_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PWM_MOTOR_PCLK, "gout_pwm_motor_pclk",
	     "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_PWM_MOTOR_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_SPI0_PCLK, "gout_spi0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_SPI_0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_SPI0_EXT_CLK, "gout_spi0_ipclk", "mout_peri_spi0_user",
	     CLK_CON_GAT_GOUT_PERI_SPI_0_EXT_CLK, 21, 0, 0),
	GATE(CLK_GOUT_SPI1_PCLK, "gout_spi1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_SPI_1_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_SPI1_EXT_CLK, "gout_spi1_ipclk", "mout_peri_spi1_user",
	     CLK_CON_GAT_GOUT_PERI_SPI_1_EXT_CLK, 21, 0, 0),
	GATE(CLK_GOUT_UART0_EXT_UCLK, "gout_uart0_ext_uclk", "mout_peri_uart0_user",
	     CLK_CON_GAT_GOUT_PERI_UART_0_EXT_UCLK, 21, 0, 0),
	GATE(CLK_GOUT_UART0_PCLK, "gout_uart0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_UART_0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_UART1_EXT_UCLK, "gout_uart1_ext_uclk", "mout_peri_uart1_user",
	     CLK_CON_GAT_GOUT_PERI_UART_1_EXT_UCLK, 21, 0, 0),
	GATE(CLK_GOUT_UART1_PCLK, "gout_uart1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_UART_1_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_UART2_EXT_UCLK, "gout_uart2_ext_uclk", "mout_peri_uart2_user",
	     CLK_CON_GAT_GOUT_PERI_UART_2_EXT_UCLK, 21, 0, 0),
	GATE(CLK_GOUT_UART2_PCLK, "gout_uart2_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_UART_2_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI0_PCLK, "gout_usi0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_USI0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI0_SCLK, "gout_usi0_sclk", "mout_peri_usi0_user",
	     CLK_CON_GAT_GOUT_PERI_USI0_SCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI1_PCLK, "gout_usi1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_USI1_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI1_SCLK, "gout_usi1_sclk", "mout_peri_usi1_user",
	     CLK_CON_GAT_GOUT_PERI_USI1_SCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI2_PCLK, "gout_usi2_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_USI2_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_USI2_SCLK, "gout_usi2_sclk", "mout_peri_usi2_user",
	     CLK_CON_GAT_GOUT_PERI_USI2_SCLK, 21, 0, 0),
	GATE(CLK_GOUT_MCT_PCLK, "gout_mct_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_MCT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_SYSREG_PERI_PCLK, "gout_sysreg_peri_pclk",
	     "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_SYSREG_PERI_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_WDT0_PCLK, "gout_wdt0_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER0_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_WDT1_PCLK, "gout_wdt1_pclk", "mout_peri_bus_user",
	     CLK_CON_GAT_GOUT_PERI_WDT_CLUSTER1_PCLK, 21, 0, 0),
];

static peri_cmu_info: samsung_cmu_info = samsung_cmu_info {
	mux_clks: peri_mux_clks,
	nr_mux_clks: ARRAY_SIZE(peri_mux_clks),
	gate_clks: peri_gate_clks,
	nr_gate_clks: ARRAY_SIZE(peri_gate_clks),
	nr_clk_ids: CLKS_NR_PERI,
	clk_regs: peri_clk_regs,
	nr_clk_regs: ARRAY_SIZE(peri_clk_regs),
	clk_name: "dout_peri_bus",
];

unsafe fn exynos7885_cmu_peri_init(np: *mut device_node)
{
	exynos_arm64_register_cmu(NULL, np, &peri_cmu_info);
}

/* Register CMU_PERI early, as it's needed for MCT timer */
clk_of_declare!(exynos7885_cmu_peri, "samsung,exynos7885-cmu-peri",
	       exynos7885_cmu_peri_init);

/* ---- CMU_CORE ------------------------------------------------------------ */

/* Register Offset definitions for CMU_CORE (0x12000000) */
const PLL_CON0_MUX_CLKCMU_CORE_BUS_USER: u32 = 0x0100;
const PLL_CON0_MUX_CLKCMU_CORE_CCI_USER: u32 = 0x0120;
const PLL_CON0_MUX_CLKCMU_CORE_G3D_USER: u32 = 0x0140;
const CLK_CON_MUX_MUX_CLK_CORE_GIC: u32 = 0x1000;
const CLK_CON_DIV_DIV_CLK_CORE_BUSP: u32 = 0x1800;
const CLK_CON_GAT_GOUT_CORE_CCI_550_ACLK: u32 = 0x2054;
const CLK_CON_GAT_GOUT_CORE_GIC400_CLK: u32 = 0x2058;
const CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_ACLK: u32 = 0x215c;
const CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_GCLK: u32 = 0x2160;
const CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_PCLK: u32 = 0x2164;
const CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_ACLK_P_CORE: u32 = 0x2168;
const CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_CCLK_P_CORE: u32 = 0x216c;
const CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK: u32 = 0x2170;
const CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK_P_CORE: u32 = 0x2174;

static core_clk_regs: &[u32] = &[
	PLL_CON0_MUX_CLKCMU_CORE_BUS_USER,
	PLL_CON0_MUX_CLKCMU_CORE_CCI_USER,
	PLL_CON0_MUX_CLKCMU_CORE_G3D_USER,
	CLK_CON_MUX_MUX_CLK_CORE_GIC,
	CLK_CON_DIV_DIV_CLK_CORE_BUSP,
	CLK_CON_GAT_GOUT_CORE_CCI_550_ACLK,
	CLK_CON_GAT_GOUT_CORE_GIC400_CLK,
	CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_ACLK,
	CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_GCLK,
	CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_PCLK,
	CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_ACLK_P_CORE,
	CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_CCLK_P_CORE,
	CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK,
	CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK_P_CORE,
};

/* List of parent clocks for Muxes in CMU_CORE */
static mout_core_bus_user_p: &[&str] = &[ "oscclk", "dout_core_bus" ];
static mout_core_cci_user_p: &[&str] = &[ "oscclk", "dout_core_cci" ];
static mout_core_g3d_user_p: &[&str] = &[ "oscclk", "dout_core_g3d" ];
static mout_core_gic_p: &[&str] = &[ "dout_core_busp", "oscclk" ];

static core_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_CORE_BUS_USER, "mout_core_bus_user", mout_core_bus_user_p,
	    PLL_CON0_MUX_CLKCMU_CORE_BUS_USER, 4, 1),
	MUX(CLK_MOUT_CORE_CCI_USER, "mout_core_cci_user", mout_core_cci_user_p,
	    PLL_CON0_MUX_CLKCMU_CORE_CCI_USER, 4, 1),
	MUX(CLK_MOUT_CORE_G3D_USER, "mout_core_g3d_user", mout_core_g3d_user_p,
	    PLL_CON0_MUX_CLKCMU_CORE_G3D_USER, 4, 1),
	MUX(CLK_MOUT_CORE_GIC, "mout_core_gic", mout_core_gic_p,
	    CLK_CON_MUX_MUX_CLK_CORE_GIC, 0, 1),
];

static core_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_CORE_BUSP, "dout_core_busp", "mout_core_bus_user",
	    CLK_CON_DIV_DIV_CLK_CORE_BUSP, 0, 2),
];

static core_gate_clks: &[samsung_gate_clock] = &[
	/* CCI (interconnect) clock must be always running */
	GATE(CLK_GOUT_CCI_ACLK, "gout_cci_aclk", "mout_core_cci_user",
	     CLK_CON_GAT_GOUT_CORE_CCI_550_ACLK, 21, CLK_IS_CRITICAL, 0),
	/* GIC (interrupt controller) clock must be always running */
	GATE(CLK_GOUT_GIC400_CLK, "gout_gic400_clk", "mout_core_gic",
	     CLK_CON_GAT_GOUT_CORE_GIC400_CLK, 21, CLK_IS_CRITICAL, 0),
	/*
	 * TREX D and P Core (seems to be related to "bus traffic shaper")
	 * clocks must always be running
	 */
	GATE(CLK_GOUT_TREX_D_CORE_ACLK, "gout_trex_d_core_aclk", "mout_core_bus_user",
	     CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_ACLK, 21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_D_CORE_GCLK, "gout_trex_d_core_gclk", "mout_core_g3d_user",
	     CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_GCLK, 21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_D_CORE_PCLK, "gout_trex_d_core_pclk", "dout_core_busp",
	     CLK_CON_GAT_GOUT_CORE_TREX_D_CORE_PCLK, 21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_P_CORE_ACLK_P_CORE, "gout_trex_p_core_aclk_p_core",
	     "mout_core_bus_user", CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_ACLK_P_CORE, 21,
	     CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_P_CORE_CCLK_P_CORE, "gout_trex_p_core_cclk_p_core",
	     "mout_core_cci_user", CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_CCLK_P_CORE, 21,
	     CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_P_CORE_PCLK, "gout_trex_p_core_pclk", "dout_core_busp",
	     CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK, 21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_TREX_P_CORE_PCLK_P_CORE, "gout_trex_p_core_pclk_p_core",
	     "dout_core_busp", CLK_CON_GAT_GOUT_CORE_TREX_P_CORE_PCLK_P_CORE, 21,
	     CLK_IS_CRITICAL, 0),
];

static core_cmu_info: samsung_cmu_info = samsung_cmu_info {
	mux_clks: core_mux_clks,
	nr_mux_clks: ARRAY_SIZE(core_mux_clks),
	div_clks: core_div_clks,
	nr_div_clks: ARRAY_SIZE(core_div_clks),
	gate_clks: core_gate_clks,
	nr_gate_clks: ARRAY_SIZE(core_gate_clks),
	nr_clk_ids: CLKS_NR_CORE,
	clk_regs: core_clk_regs,
	nr_clk_regs: ARRAY_SIZE(core_clk_regs),
	clk_name: "dout_core_bus",
];

/* ---- CMU_FSYS ------------------------------------------------------------ */

/* Register Offset definitions for CMU_FSYS (0x13400000) */
const PLL_LOCKTIME_PLL_USB: u32 = 0x0000;
const PLL_CON0_MUX_CLKCMU_FSYS_BUS_USER: u32 = 0x0100;
const PLL_CON0_MUX_CLKCMU_FSYS_MMC_CARD_USER: u32 = 0x0120;
const PLL_CON0_MUX_CLKCMU_FSYS_MMC_EMBD_USER: u32 = 0x0140;
const PLL_CON0_MUX_CLKCMU_FSYS_MMC_SDIO_USER: u32 = 0x0160;
const PLL_CON0_MUX_CLKCMU_FSYS_USB30DRD_USER: u32 = 0x0180;
const PLL_CON0_PLL_USB: u32 = 0x01a0;
const CLK_CON_GAT_CLK_FSYS_USB20PHY_CLKCORE: u32 = 0x200c;
const CLK_CON_GAT_GOUT_FSYS_MMC_CARD_I_ACLK: u32 = 0x2030;
const CLK_CON_GAT_GOUT_FSYS_MMC_CARD_SDCLKIN: u32 = 0x2034;
const CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_I_ACLK: u32 = 0x2038;
const CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_SDCLKIN: u32 = 0x203c;
const CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_I_ACLK: u32 = 0x2040;
const CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_SDCLKIN: u32 = 0x2044;
const CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_20PHYCTRL: u32 = 0x2068;
const CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_0: u32 = 0x206c;
const CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_1: u32 = 0x2070;
const CLK_CON_GAT_GOUT_FSYS_USB30DRD_BUS_CLK_EARLY: u32 = 0x2074;
const CLK_CON_GAT_GOUT_FSYS_USB30DRD_REF_CLK: u32 = 0x2078;

static fsys_clk_regs: &[u32] = &[
	PLL_LOCKTIME_PLL_USB,
	PLL_CON0_MUX_CLKCMU_FSYS_BUS_USER,
	PLL_CON0_MUX_CLKCMU_FSYS_MMC_CARD_USER,
	PLL_CON0_MUX_CLKCMU_FSYS_MMC_EMBD_USER,
	PLL_CON0_MUX_CLKCMU_FSYS_MMC_SDIO_USER,
	PLL_CON0_MUX_CLKCMU_FSYS_USB30DRD_USER,
	PLL_CON0_PLL_USB,
	CLK_CON_GAT_CLK_FSYS_USB20PHY_CLKCORE,
	CLK_CON_GAT_GOUT_FSYS_MMC_CARD_I_ACLK,
	CLK_CON_GAT_GOUT_FSYS_MMC_CARD_SDCLKIN,
	CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_I_ACLK,
	CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_SDCLKIN,
	CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_I_ACLK,
	CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_SDCLKIN,
	CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_20PHYCTRL,
	CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_0,
	CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_1,
	CLK_CON_GAT_GOUT_FSYS_USB30DRD_BUS_CLK_EARLY,
	CLK_CON_GAT_GOUT_FSYS_USB30DRD_REF_CLK,
];

static pll_usb_rate_table: &[samsung_pll_rate_table] = &[
	PLL_35XX_RATE(26 * MHZ, 50000000U, 400, 13, 4),
];

static fsys_pll_clks: &[samsung_pll_clock] = &[
	PLL(pll_1418x, CLK_FOUT_USB_PLL, "fout_usb_pll", "oscclk",
	    PLL_LOCKTIME_PLL_USB, PLL_CON0_PLL_USB,
	    pll_usb_rate_table),
};

/* List of parent clocks for Muxes in CMU_FSYS */
static mout_fsys_bus_user_p: &[&str] = &[ "oscclk", "dout_fsys_bus" ];
static mout_fsys_mmc_card_user_p: &[&str] = &[ "oscclk", "dout_fsys_mmc_card" ];
static mout_fsys_mmc_embd_user_p: &[&str] = &[ "oscclk", "dout_fsys_mmc_embd" ];
static mout_fsys_mmc_sdio_user_p: &[&str] = &[ "oscclk", "dout_fsys_mmc_sdio" ];
static mout_fsys_usb30drd_user_p: &[&str] = &[ "oscclk", "dout_fsys_usb30drd" ];
static mout_usb_pll_p: &[&str] = &[ "oscclk", "fout_usb_pll" ];

static fsys_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_FSYS_BUS_USER, "mout_fsys_bus_user", mout_fsys_bus_user_p,
	    PLL_CON0_MUX_CLKCMU_FSYS_BUS_USER, 4, 1),
	MUX_F(CLK_MOUT_FSYS_MMC_CARD_USER, "mout_fsys_mmc_card_user",
	      mout_fsys_mmc_card_user_p, PLL_CON0_MUX_CLKCMU_FSYS_MMC_CARD_USER,
	      4, 1, CLK_SET_RATE_PARENT, 0),
	MUX_F(CLK_MOUT_FSYS_MMC_EMBD_USER, "mout_fsys_mmc_embd_user",
	      mout_fsys_mmc_embd_user_p, PLL_CON0_MUX_CLKCMU_FSYS_MMC_EMBD_USER,
	      4, 1, CLK_SET_RATE_PARENT, 0),
	MUX_F(CLK_MOUT_FSYS_MMC_SDIO_USER, "mout_fsys_mmc_sdio_user",
	      mout_fsys_mmc_sdio_user_p, PLL_CON0_MUX_CLKCMU_FSYS_MMC_SDIO_USER,
	      4, 1, CLK_SET_RATE_PARENT, 0),
	MUX(CLK_MOUT_FSYS_USB30DRD_USER, "mout_fsys_usb30drd_user",
	      mout_fsys_usb30drd_user_p, PLL_CON0_MUX_CLKCMU_FSYS_USB30DRD_USER,
	      4, 1),
	nMUX_F(CLK_MOUT_USB_PLL, "mout_usb_pll", mout_usb_pll_p,
	    PLL_CON0_PLL_USB, 4, 1, CLK_SET_RATE_PARENT, 0),
];

static fsys_gate_clks: &[samsung_gate_clock] = &[
	GATE(CLK_FSYS_USB20PHY_CLKCORE, "clk_fsys_usb20phy_clkcore", "mout_usb_pll",
	     CLK_CON_GAT_CLK_FSYS_USB20PHY_CLKCORE, 21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_GOUT_MMC_CARD_ACLK, "gout_mmc_card_aclk", "mout_fsys_bus_user",
	     CLK_CON_GAT_GOUT_FSYS_MMC_CARD_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_MMC_CARD_SDCLKIN, "gout_mmc_card_sdclkin",
	     "mout_fsys_mmc_card_user", CLK_CON_GAT_GOUT_FSYS_MMC_CARD_SDCLKIN,
	     21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_GOUT_MMC_EMBD_ACLK, "gout_mmc_embd_aclk", "mout_fsys_bus_user",
	     CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_MMC_EMBD_SDCLKIN, "gout_mmc_embd_sdclkin",
	     "mout_fsys_mmc_embd_user", CLK_CON_GAT_GOUT_FSYS_MMC_EMBD_SDCLKIN,
	     21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_GOUT_MMC_SDIO_ACLK, "gout_mmc_sdio_aclk", "mout_fsys_bus_user",
	     CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_MMC_SDIO_SDCLKIN, "gout_mmc_sdio_sdclkin",
	     "mout_fsys_mmc_sdio_user", CLK_CON_GAT_GOUT_FSYS_MMC_SDIO_SDCLKIN,
	     21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_FSYS_USB30DRD_ACLK_20PHYCTRL, "clk_fsys_usb30drd_aclk_20phyctrl",
	     "mout_fsys_bus_user", CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_20PHYCTRL, 21, 0, 0),
	GATE(CLK_FSYS_USB30DRD_ACLK_30PHYCTRL_0, "clk_fsys_usb30drd_aclk_30phyctrl_0",
	     "mout_fsys_bus_user", CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_0, 21, 0, 0),
	GATE(CLK_FSYS_USB30DRD_ACLK_30PHYCTRL_1, "clk_fsys_usb30drd_aclk_30phyctrl_1",
	     "mout_fsys_bus_user", CLK_CON_GAT_GOUT_FSYS_USB30DRD_ACLK_30PHYCTRL_1, 21, 0, 0),
	GATE(CLK_FSYS_USB30DRD_BUS_CLK_EARLY, "clk_fsys_usb30drd_bus_clk_early",
	     "mout_fsys_bus_user", CLK_CON_GAT_GOUT_FSYS_USB30DRD_BUS_CLK_EARLY, 21, 0, 0),
	GATE(CLK_FSYS_USB30DRD_REF_CLK, "clk_fsys_usb30drd_ref_clk", "mout_fsys_usb30drd_user",
	     CLK_CON_GAT_GOUT_FSYS_USB30DRD_REF_CLK, 21, 0, 0),
];

static fsys_cmu_info: samsung_cmu_info = samsung_cmu_info {
	pll_clks: fsys_pll_clks,
	nr_pll_clks: ARRAY_SIZE(fsys_pll_clks),
	mux_clks: fsys_mux_clks,
	nr_mux_clks: ARRAY_SIZE(fsys_mux_clks),
	gate_clks: fsys_gate_clks,
	nr_gate_clks: ARRAY_SIZE(fsys_gate_clks),
	nr_clk_ids: CLKS_NR_FSYS,
	clk_regs: fsys_clk_regs,
	nr_clk_regs: ARRAY_SIZE(fsys_clk_regs),
	clk_name: "dout_fsys_bus",
];

/* ---- platform_driver ----------------------------------------------------- */

unsafe fn exynos7885_cmu_probe(pdev: *mut platform_device) -> i32
{
	let mut info: *const samsung_cmu_info;
	let dev: *mut device = unsafe { &mut (*pdev).dev };

	info = of_device_get_match_data(dev);
	exynos_arm64_register_cmu(dev, dev->of_node, info);

	return 0;
}

static exynos7885_cmu_of_match: &[of_device_id] = &[
	{
		compatible: "samsung,exynos7885-cmu-core",
		data: &core_cmu_info,
	}, {
		compatible: "samsung,exynos7885-cmu-fsys",
		data: &fsys_cmu_info,
	}, {
	},
];

static mut exynos7885_cmu_driver: platform_driver = platform_driver {
	driver: {
		name: "exynos7885-cmu",
		of_match_table: exynos7885_cmu_of_match,
		suppress_bind_attrs: true,
	},
	probe: exynos7885_cmu_probe,
];

unsafe fn exynos7885_cmu_init() -> i32
{
	return platform_driver_register(&exynos7885_cmu_driver);
}
core_initcall!(exynos7885_cmu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
