// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Mateusz Krawczuk <m.krawczuk@partner.samsung.com>
 *
 * Based on clock drivers for S3C64xx and Exynos4 SoCs.
 *
 * Common Clock Framework support for all S5PC110/S5PV210 SoCs.
 */

// #include <linux/clk-provider.h>
// #include <linux/of_address.h>

// #include "clk.h"
// #include "clk-pll.h"

// #include <dt-bindings/clock/s5pv210.h>

/* S5PC110/S5PV210 clock controller register offsets */
const APLL_LOCK: u32 = 0x0000;
const MPLL_LOCK: u32 = 0x0008;
const EPLL_LOCK: u32 = 0x0010;
const VPLL_LOCK: u32 = 0x0020;
const APLL_CON0: u32 = 0x0100;
const APLL_CON1: u32 = 0x0104;
const MPLL_CON: u32 = 0x0108;
const EPLL_CON0: u32 = 0x0110;
const EPLL_CON1: u32 = 0x0114;
const VPLL_CON: u32 = 0x0120;
const CLK_SRC0: u32 = 0x0200;
const CLK_SRC1: u32 = 0x0204;
const CLK_SRC2: u32 = 0x0208;
const CLK_SRC3: u32 = 0x020c;
const CLK_SRC4: u32 = 0x0210;
const CLK_SRC5: u32 = 0x0214;
const CLK_SRC6: u32 = 0x0218;
const CLK_SRC_MASK0: u32 = 0x0280;
const CLK_SRC_MASK1: u32 = 0x0284;
const CLK_DIV0: u32 = 0x0300;
const CLK_DIV1: u32 = 0x0304;
const CLK_DIV2: u32 = 0x0308;
const CLK_DIV3: u32 = 0x030c;
const CLK_DIV4: u32 = 0x0310;
const CLK_DIV5: u32 = 0x0314;
const CLK_DIV6: u32 = 0x0318;
const CLK_DIV7: u32 = 0x031c;
const CLK_GATE_MAIN0: u32 = 0x0400;
const CLK_GATE_MAIN1: u32 = 0x0404;
const CLK_GATE_MAIN2: u32 = 0x0408;
const CLK_GATE_PERI0: u32 = 0x0420;
const CLK_GATE_PERI1: u32 = 0x0424;
const CLK_GATE_SCLK0: u32 = 0x0440;
const CLK_GATE_SCLK1: u32 = 0x0444;
const CLK_GATE_IP0: u32 = 0x0460;
const CLK_GATE_IP1: u32 = 0x0464;
const CLK_GATE_IP2: u32 = 0x0468;
const CLK_GATE_IP3: u32 = 0x046c;
const CLK_GATE_IP4: u32 = 0x0470;
const CLK_GATE_BLOCK: u32 = 0x0480;
const CLK_GATE_IP5: u32 = 0x0484;
const CLK_OUT: u32 = 0x0500;
const MISC: u32 = 0xe000;
const OM_STAT: u32 = 0xe100;

/* IDs of PLLs available on S5PV210/S5P6442 SoCs */
const apll: usize = 0;
const mpll: usize = 1;
const epll: usize = 2;
const vpll: usize = 3;

/* IDs of external clocks (used for legacy boards) */
const xxti: usize = 0;
const xusbxti: usize = 1;

static void __iomem *reg_base;

/* List of registers that need to be preserved across suspend/resume. */
static mut s5pv210_clk_regs[]  = {
	CLK_SRC0,
	CLK_SRC1,
	CLK_SRC2,
	CLK_SRC3,
	CLK_SRC4,
	CLK_SRC5,
	CLK_SRC6,
	CLK_SRC_MASK0,
	CLK_SRC_MASK1,
	CLK_DIV0,
	CLK_DIV1,
	CLK_DIV2,
	CLK_DIV3,
	CLK_DIV4,
	CLK_DIV5,
	CLK_DIV6,
	CLK_DIV7,
	CLK_GATE_MAIN0,
	CLK_GATE_MAIN1,
	CLK_GATE_MAIN2,
	CLK_GATE_PERI0,
	CLK_GATE_PERI1,
	CLK_GATE_SCLK0,
	CLK_GATE_SCLK1,
	CLK_GATE_IP0,
	CLK_GATE_IP1,
	CLK_GATE_IP2,
	CLK_GATE_IP3,
	CLK_GATE_IP4,
	CLK_GATE_IP5,
	CLK_GATE_BLOCK,
	APLL_LOCK,
	MPLL_LOCK,
	EPLL_LOCK,
	VPLL_LOCK,
	APLL_CON0,
	APLL_CON1,
	MPLL_CON,
	EPLL_CON0,
	EPLL_CON1,
	VPLL_CON,
	CLK_OUT,
};

/* Mux parent lists. */
&[&str] fin_pll_p[]  = {
	"xxti",
	"xusbxti"
};

&[&str] mout_apll_p[]  = {
	"fin_pll",
	"fout_apll"
};

&[&str] mout_mpll_p[]  = {
	"fin_pll",
	"fout_mpll"
};

&[&str] mout_epll_p[]  = {
	"fin_pll",
	"fout_epll"
};

&[&str] mout_vpllsrc_p[]  = {
	"fin_pll",
	"sclk_hdmi27m"
};

&[&str] mout_vpll_p[]  = {
	"mout_vpllsrc",
	"fout_vpll"
};

&[&str] mout_group1_p[]  = {
	"dout_a2m",
	"mout_mpll",
	"mout_epll",
	"mout_vpll"
};

&[&str] mout_group2_p[]  = {
	"xxti",
	"xusbxti",
	"sclk_hdmi27m",
	"sclk_usbphy0",
	"sclk_usbphy1",
	"sclk_hdmiphy",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_audio0_p[]  = {
	"xxti",
	"pcmcdclk0",
	"sclk_hdmi27m",
	"sclk_usbphy0",
	"sclk_usbphy1",
	"sclk_hdmiphy",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_audio1_p[]  = {
	"i2scdclk1",
	"pcmcdclk1",
	"sclk_hdmi27m",
	"sclk_usbphy0",
	"sclk_usbphy1",
	"sclk_hdmiphy",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_audio2_p[]  = {
	"i2scdclk2",
	"pcmcdclk2",
	"sclk_hdmi27m",
	"sclk_usbphy0",
	"sclk_usbphy1",
	"sclk_hdmiphy",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_spdif_p[]  = {
	"dout_audio0",
	"dout_audio1",
	"dout_audio3",
};

&[&str] mout_group3_p[]  = {
	"mout_apll",
	"mout_mpll"
};

&[&str] mout_group4_p[]  = {
	"mout_mpll",
	"dout_a2m"
};

&[&str] mout_flash_p[]  = {
	"dout_hclkd",
	"dout_hclkp"
};

&[&str] mout_dac_p[]  = {
	"mout_vpll",
	"sclk_hdmiphy"
};

&[&str] mout_hdmi_p[]  = {
	"sclk_hdmiphy",
	"dout_tblk"
};

&[&str] mout_mixer_p[]  = {
	"mout_dac",
	"mout_hdmi"
};

&[&str] mout_vpll_6442_p[]  = {
	"fin_pll",
	"fout_vpll"
};

&[&str] mout_mixer_6442_p[]  = {
	"mout_vpll",
	"dout_mixer"
};

&[&str] mout_d0sync_6442_p[]  = {
	"mout_dsys",
	"div_apll"
};

&[&str] mout_d1sync_6442_p[]  = {
	"mout_psys",
	"div_apll"
};

&[&str] mout_group2_6442_p[]  = {
	"fin_pll",
	"none",
	"none",
	"sclk_usbphy0",
	"none",
	"none",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_audio0_6442_p[]  = {
	"fin_pll",
	"pcmcdclk0",
	"none",
	"sclk_usbphy0",
	"none",
	"none",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
};

&[&str] mout_audio1_6442_p[]  = {
	"i2scdclk1",
	"pcmcdclk1",
	"none",
	"sclk_usbphy0",
	"none",
	"none",
	"mout_mpll",
	"mout_epll",
	"mout_vpll",
	"fin_pll",
};

&[&str] mout_clksel_p[]  = {
	"fout_apll_clkout",
	"fout_mpll_clkout",
	"fout_epll",
	"fout_vpll",
	"sclk_usbphy0",
	"sclk_usbphy1",
	"sclk_hdmiphy",
	"rtc",
	"rtc_tick",
	"dout_hclkm",
	"dout_pclkm",
	"dout_hclkd",
	"dout_pclkd",
	"dout_hclkp",
	"dout_pclkp",
	"dout_apll_clkout",
	"dout_hpm",
	"xxti",
	"xusbxti",
	"div_dclk"
};

&[&str] mout_clksel_6442_p[]  = {
	"fout_apll_clkout",
	"fout_mpll_clkout",
	"fout_epll",
	"fout_vpll",
	"sclk_usbphy0",
	"none",
	"none",
	"rtc",
	"rtc_tick",
	"none",
	"none",
	"dout_hclkd",
	"dout_pclkd",
	"dout_hclkp",
	"dout_pclkp",
	"dout_apll_clkout",
	"none",
	"fin_pll",
	"none",
	"div_dclk"
};

&[&str] mout_clkout_p[]  = {
	"dout_clkout",
	"none",
	"xxti",
	"xusbxti"
};

/* Common fixed factor clocks. */
const samsung_fixed_factor_clock ffactor_clks[]  = {
	FFACTOR(FOUT_APLL_CLKOUT, "fout_apll_clkout", "fout_apll", 1, 4, 0),
	FFACTOR(FOUT_MPLL_CLKOUT, "fout_mpll_clkout", "fout_mpll", 1, 2, 0),
	FFACTOR(DOUT_APLL_CLKOUT, "dout_apll_clkout", "dout_apll", 1, 4, 0),
};

/* PLL input mux (fin_pll), which needs to be registered before PLLs. */
const samsung_mux_clock early_mux_clks[]  = {
	MUX_F(FIN_PLL, "fin_pll", fin_pll_p, OM_STAT, 0, 1,
					CLK_MUX_READ_ONLY, 0),
};

/* Common clock muxes. */
const samsung_mux_clock mux_clks[]  = {
	MUX(MOUT_FLASH, "mout_flash", mout_flash_p, CLK_SRC0, 28, 1),
	MUX(MOUT_PSYS, "mout_psys", mout_group4_p, CLK_SRC0, 24, 1),
	MUX(MOUT_DSYS, "mout_dsys", mout_group4_p, CLK_SRC0, 20, 1),
	MUX(MOUT_MSYS, "mout_msys", mout_group3_p, CLK_SRC0, 16, 1),
	MUX(MOUT_EPLL, "mout_epll", mout_epll_p, CLK_SRC0, 8, 1),
	MUX(MOUT_MPLL, "mout_mpll", mout_mpll_p, CLK_SRC0, 4, 1),
	MUX(MOUT_APLL, "mout_apll", mout_apll_p, CLK_SRC0, 0, 1),

	MUX(MOUT_CLKOUT, "mout_clkout", mout_clkout_p, MISC, 8, 2),
};

/* S5PV210-specific clock muxes. */
const samsung_mux_clock s5pv210_mux_clks[]  = {
	MUX(MOUT_VPLL, "mout_vpll", mout_vpll_p, CLK_SRC0, 12, 1),

	MUX(MOUT_VPLLSRC, "mout_vpllsrc", mout_vpllsrc_p, CLK_SRC1, 28, 1),
	MUX(MOUT_CSIS, "mout_csis", mout_group2_p, CLK_SRC1, 24, 4),
	MUX(MOUT_FIMD, "mout_fimd", mout_group2_p, CLK_SRC1, 20, 4),
	MUX(MOUT_CAM1, "mout_cam1", mout_group2_p, CLK_SRC1, 16, 4),
	MUX(MOUT_CAM0, "mout_cam0", mout_group2_p, CLK_SRC1, 12, 4),
	MUX(MOUT_DAC, "mout_dac", mout_dac_p, CLK_SRC1, 8, 1),
	MUX(MOUT_MIXER, "mout_mixer", mout_mixer_p, CLK_SRC1, 4, 1),
	MUX(MOUT_HDMI, "mout_hdmi", mout_hdmi_p, CLK_SRC1, 0, 1),

	MUX(MOUT_G2D, "mout_g2d", mout_group1_p, CLK_SRC2, 8, 2),
	MUX(MOUT_MFC, "mout_mfc", mout_group1_p, CLK_SRC2, 4, 2),
	MUX(MOUT_G3D, "mout_g3d", mout_group1_p, CLK_SRC2, 0, 2),

	MUX(MOUT_FIMC2, "mout_fimc2", mout_group2_p, CLK_SRC3, 20, 4),
	MUX(MOUT_FIMC1, "mout_fimc1", mout_group2_p, CLK_SRC3, 16, 4),
	MUX(MOUT_FIMC0, "mout_fimc0", mout_group2_p, CLK_SRC3, 12, 4),

	MUX(MOUT_UART3, "mout_uart3", mout_group2_p, CLK_SRC4, 28, 4),
	MUX(MOUT_UART2, "mout_uart2", mout_group2_p, CLK_SRC4, 24, 4),
	MUX(MOUT_UART1, "mout_uart1", mout_group2_p, CLK_SRC4, 20, 4),
	MUX(MOUT_UART0, "mout_uart0", mout_group2_p, CLK_SRC4, 16, 4),
	MUX(MOUT_MMC3, "mout_mmc3", mout_group2_p, CLK_SRC4, 12, 4),
	MUX(MOUT_MMC2, "mout_mmc2", mout_group2_p, CLK_SRC4, 8, 4),
	MUX(MOUT_MMC1, "mout_mmc1", mout_group2_p, CLK_SRC4, 4, 4),
	MUX(MOUT_MMC0, "mout_mmc0", mout_group2_p, CLK_SRC4, 0, 4),

	MUX(MOUT_PWM, "mout_pwm", mout_group2_p, CLK_SRC5, 12, 4),
	MUX(MOUT_SPI1, "mout_spi1", mout_group2_p, CLK_SRC5, 4, 4),
	MUX(MOUT_SPI0, "mout_spi0", mout_group2_p, CLK_SRC5, 0, 4),

	MUX(MOUT_DMC0, "mout_dmc0", mout_group1_p, CLK_SRC6, 24, 2),
	MUX(MOUT_PWI, "mout_pwi", mout_group2_p, CLK_SRC6, 20, 4),
	MUX(MOUT_HPM, "mout_hpm", mout_group3_p, CLK_SRC6, 16, 1),
	MUX(MOUT_SPDIF, "mout_spdif", mout_spdif_p, CLK_SRC6, 12, 2),
	MUX(MOUT_AUDIO2, "mout_audio2", mout_audio2_p, CLK_SRC6, 8, 4),
	MUX(MOUT_AUDIO1, "mout_audio1", mout_audio1_p, CLK_SRC6, 4, 4),
	MUX(MOUT_AUDIO0, "mout_audio0", mout_audio0_p, CLK_SRC6, 0, 4),

	MUX(MOUT_CLKSEL, "mout_clksel", mout_clksel_p, CLK_OUT, 12, 5),
};

/* S5P6442-specific clock muxes. */
const samsung_mux_clock s5p6442_mux_clks[]  = {
	MUX(MOUT_VPLL, "mout_vpll", mout_vpll_6442_p, CLK_SRC0, 12, 1),

	MUX(MOUT_FIMD, "mout_fimd", mout_group2_6442_p, CLK_SRC1, 20, 4),
	MUX(MOUT_CAM1, "mout_cam1", mout_group2_6442_p, CLK_SRC1, 16, 4),
	MUX(MOUT_CAM0, "mout_cam0", mout_group2_6442_p, CLK_SRC1, 12, 4),
	MUX(MOUT_MIXER, "mout_mixer", mout_mixer_6442_p, CLK_SRC1, 4, 1),

	MUX(MOUT_D0SYNC, "mout_d0sync", mout_d0sync_6442_p, CLK_SRC2, 28, 1),
	MUX(MOUT_D1SYNC, "mout_d1sync", mout_d1sync_6442_p, CLK_SRC2, 24, 1),

	MUX(MOUT_FIMC2, "mout_fimc2", mout_group2_6442_p, CLK_SRC3, 20, 4),
	MUX(MOUT_FIMC1, "mout_fimc1", mout_group2_6442_p, CLK_SRC3, 16, 4),
	MUX(MOUT_FIMC0, "mout_fimc0", mout_group2_6442_p, CLK_SRC3, 12, 4),

	MUX(MOUT_UART2, "mout_uart2", mout_group2_6442_p, CLK_SRC4, 24, 4),
	MUX(MOUT_UART1, "mout_uart1", mout_group2_6442_p, CLK_SRC4, 20, 4),
	MUX(MOUT_UART0, "mout_uart0", mout_group2_6442_p, CLK_SRC4, 16, 4),
	MUX(MOUT_MMC2, "mout_mmc2", mout_group2_6442_p, CLK_SRC4, 8, 4),
	MUX(MOUT_MMC1, "mout_mmc1", mout_group2_6442_p, CLK_SRC4, 4, 4),
	MUX(MOUT_MMC0, "mout_mmc0", mout_group2_6442_p, CLK_SRC4, 0, 4),

	MUX(MOUT_PWM, "mout_pwm", mout_group2_6442_p, CLK_SRC5, 12, 4),
	MUX(MOUT_SPI0, "mout_spi0", mout_group2_6442_p, CLK_SRC5, 0, 4),

	MUX(MOUT_AUDIO1, "mout_audio1", mout_audio1_6442_p, CLK_SRC6, 4, 4),
	MUX(MOUT_AUDIO0, "mout_audio0", mout_audio0_6442_p, CLK_SRC6, 0, 4),

	MUX(MOUT_CLKSEL, "mout_clksel", mout_clksel_6442_p, CLK_OUT, 12, 5),
};

/* S5PV210-specific fixed rate clocks generated inside the SoC. */
const samsung_fixed_rate_clock s5pv210_frate_clks[]  = {
	FRATE(SCLK_HDMI27M, "sclk_hdmi27m", core::ptr::null_mut(), 0, 27000000),
	FRATE(SCLK_HDMIPHY, "sclk_hdmiphy", core::ptr::null_mut(), 0, 27000000),
	FRATE(SCLK_USBPHY0, "sclk_usbphy0", core::ptr::null_mut(), 0, 48000000),
	FRATE(SCLK_USBPHY1, "sclk_usbphy1", core::ptr::null_mut(), 0, 48000000),
};

/* S5P6442-specific fixed rate clocks generated inside the SoC. */
const samsung_fixed_rate_clock s5p6442_frate_clks[]  = {
	FRATE(SCLK_USBPHY0, "sclk_usbphy0", core::ptr::null_mut(), 0, 30000000),
};

/* Common clock dividers. */
const samsung_div_clock div_clks[]  = {
	DIV(DOUT_PCLKP, "dout_pclkp", "dout_hclkp", CLK_DIV0, 28, 3),
	DIV(DOUT_PCLKD, "dout_pclkd", "dout_hclkd", CLK_DIV0, 20, 3),
	DIV(DOUT_A2M, "dout_a2m", "mout_apll", CLK_DIV0, 4, 3),
	DIV(DOUT_APLL, "dout_apll", "mout_msys", CLK_DIV0, 0, 3),

	DIV(DOUT_FIMD, "dout_fimd", "mout_fimd", CLK_DIV1, 20, 4),
	DIV(DOUT_CAM1, "dout_cam1", "mout_cam1", CLK_DIV1, 16, 4),
	DIV(DOUT_CAM0, "dout_cam0", "mout_cam0", CLK_DIV1, 12, 4),

	DIV(DOUT_FIMC2, "dout_fimc2", "mout_fimc2", CLK_DIV3, 20, 4),
	DIV(DOUT_FIMC1, "dout_fimc1", "mout_fimc1", CLK_DIV3, 16, 4),
	DIV(DOUT_FIMC0, "dout_fimc0", "mout_fimc0", CLK_DIV3, 12, 4),

	DIV(DOUT_UART2, "dout_uart2", "mout_uart2", CLK_DIV4, 24, 4),
	DIV(DOUT_UART1, "dout_uart1", "mout_uart1", CLK_DIV4, 20, 4),
	DIV(DOUT_UART0, "dout_uart0", "mout_uart0", CLK_DIV4, 16, 4),
	DIV(DOUT_MMC2, "dout_mmc2", "mout_mmc2", CLK_DIV4, 8, 4),
	DIV(DOUT_MMC1, "dout_mmc1", "mout_mmc1", CLK_DIV4, 4, 4),
	DIV(DOUT_MMC0, "dout_mmc0", "mout_mmc0", CLK_DIV4, 0, 4),

	DIV(DOUT_PWM, "dout_pwm", "mout_pwm", CLK_DIV5, 12, 4),
	DIV(DOUT_SPI0, "dout_spi0", "mout_spi0", CLK_DIV5, 0, 4),

	DIV(DOUT_FLASH, "dout_flash", "mout_flash", CLK_DIV6, 12, 3),
	DIV(DOUT_AUDIO1, "dout_audio1", "mout_audio1", CLK_DIV6, 4, 4),
	DIV(DOUT_AUDIO0, "dout_audio0", "mout_audio0", CLK_DIV6, 0, 4),

	DIV(DOUT_CLKOUT, "dout_clkout", "mout_clksel", CLK_OUT, 20, 4),
};

/* S5PV210-specific clock dividers. */
const samsung_div_clock s5pv210_div_clks[]  = {
	DIV(DOUT_HCLKP, "dout_hclkp", "mout_psys", CLK_DIV0, 24, 4),
	DIV(DOUT_HCLKD, "dout_hclkd", "mout_dsys", CLK_DIV0, 16, 4),
	DIV(DOUT_PCLKM, "dout_pclkm", "dout_hclkm", CLK_DIV0, 12, 3),
	DIV(DOUT_HCLKM, "dout_hclkm", "dout_apll", CLK_DIV0, 8, 3),

	DIV(DOUT_CSIS, "dout_csis", "mout_csis", CLK_DIV1, 28, 4),
	DIV(DOUT_TBLK, "dout_tblk", "mout_vpll", CLK_DIV1, 0, 4),

	DIV(DOUT_G2D, "dout_g2d", "mout_g2d", CLK_DIV2, 8, 4),
	DIV(DOUT_MFC, "dout_mfc", "mout_mfc", CLK_DIV2, 4, 4),
	DIV(DOUT_G3D, "dout_g3d", "mout_g3d", CLK_DIV2, 0, 4),

	DIV(DOUT_UART3, "dout_uart3", "mout_uart3", CLK_DIV4, 28, 4),
	DIV(DOUT_MMC3, "dout_mmc3", "mout_mmc3", CLK_DIV4, 12, 4),

	DIV(DOUT_SPI1, "dout_spi1", "mout_spi1", CLK_DIV5, 4, 4),

	DIV(DOUT_DMC0, "dout_dmc0", "mout_dmc0", CLK_DIV6, 28, 4),
	DIV(DOUT_PWI, "dout_pwi", "mout_pwi", CLK_DIV6, 24, 4),
	DIV(DOUT_HPM, "dout_hpm", "dout_copy", CLK_DIV6, 20, 3),
	DIV(DOUT_COPY, "dout_copy", "mout_hpm", CLK_DIV6, 16, 3),
	DIV(DOUT_AUDIO2, "dout_audio2", "mout_audio2", CLK_DIV6, 8, 4),

	DIV(DOUT_DPM, "dout_dpm", "dout_pclkp", CLK_DIV7, 8, 7),
	DIV(DOUT_DVSEM, "dout_dvsem", "dout_pclkp", CLK_DIV7, 0, 7),
};

/* S5P6442-specific clock dividers. */
const samsung_div_clock s5p6442_div_clks[]  = {
	DIV(DOUT_HCLKP, "dout_hclkp", "mout_d1sync", CLK_DIV0, 24, 4),
	DIV(DOUT_HCLKD, "dout_hclkd", "mout_d0sync", CLK_DIV0, 16, 4),

	DIV(DOUT_MIXER, "dout_mixer", "mout_vpll", CLK_DIV1, 0, 4),
};

/* Common clock gates. */
const samsung_gate_clock gate_clks[]  = {
	GATE(CLK_ROTATOR, "rotator", "dout_hclkd", CLK_GATE_IP0, 29, 0, 0),
	GATE(CLK_FIMC2, "fimc2", "dout_hclkd", CLK_GATE_IP0, 26, 0, 0),
	GATE(CLK_FIMC1, "fimc1", "dout_hclkd", CLK_GATE_IP0, 25, 0, 0),
	GATE(CLK_FIMC0, "fimc0", "dout_hclkd", CLK_GATE_IP0, 24, 0, 0),
	GATE(CLK_PDMA0, "pdma0", "dout_hclkp", CLK_GATE_IP0, 3, 0, 0),
	GATE(CLK_MDMA, "mdma", "dout_hclkd", CLK_GATE_IP0, 2, 0, 0),

	GATE(CLK_SROMC, "sromc", "dout_hclkp", CLK_GATE_IP1, 26, 0, 0),
	GATE(CLK_NANDXL, "nandxl", "dout_hclkp", CLK_GATE_IP1, 24, 0, 0),
	GATE(CLK_USB_OTG, "usb_otg", "dout_hclkp", CLK_GATE_IP1, 16, 0, 0),
	GATE(CLK_TVENC, "tvenc", "dout_hclkd", CLK_GATE_IP1, 10, 0, 0),
	GATE(CLK_MIXER, "mixer", "dout_hclkd", CLK_GATE_IP1, 9, 0, 0),
	GATE(CLK_VP, "vp", "dout_hclkd", CLK_GATE_IP1, 8, 0, 0),
	GATE(CLK_FIMD, "fimd", "dout_hclkd", CLK_GATE_IP1, 0, 0, 0),

	GATE(CLK_HSMMC2, "hsmmc2", "dout_hclkp", CLK_GATE_IP2, 18, 0, 0),
	GATE(CLK_HSMMC1, "hsmmc1", "dout_hclkp", CLK_GATE_IP2, 17, 0, 0),
	GATE(CLK_HSMMC0, "hsmmc0", "dout_hclkp", CLK_GATE_IP2, 16, 0, 0),
	GATE(CLK_MODEMIF, "modemif", "dout_hclkp", CLK_GATE_IP2, 9, 0, 0),
	GATE(CLK_SECSS, "secss", "dout_hclkp", CLK_GATE_IP2, 0, 0, 0),

	GATE(CLK_PCM1, "pcm1", "dout_pclkp", CLK_GATE_IP3, 29, 0, 0),
	GATE(CLK_PCM0, "pcm0", "dout_pclkp", CLK_GATE_IP3, 28, 0, 0),
	GATE(CLK_TSADC, "tsadc", "dout_pclkp", CLK_GATE_IP3, 24, 0, 0),
	GATE(CLK_PWM, "pwm", "dout_pclkp", CLK_GATE_IP3, 23, 0, 0),
	GATE(CLK_WDT, "watchdog", "dout_pclkp", CLK_GATE_IP3, 22, 0, 0),
	GATE(CLK_KEYIF, "keyif", "dout_pclkp", CLK_GATE_IP3, 21, 0, 0),
	GATE(CLK_UART2, "uart2", "dout_pclkp", CLK_GATE_IP3, 19, 0, 0),
	GATE(CLK_UART1, "uart1", "dout_pclkp", CLK_GATE_IP3, 18, 0, 0),
	GATE(CLK_UART0, "uart0", "dout_pclkp", CLK_GATE_IP3, 17, 0, 0),
	GATE(CLK_SYSTIMER, "systimer", "dout_pclkp", CLK_GATE_IP3, 16, 0, 0),
	GATE(CLK_RTC, "rtc", "dout_pclkp", CLK_GATE_IP3, 15, 0, 0),
	GATE(CLK_SPI0, "spi0", "dout_pclkp", CLK_GATE_IP3, 12, 0, 0),
	GATE(CLK_I2C2, "i2c2", "dout_pclkp", CLK_GATE_IP3, 9, 0, 0),
	GATE(CLK_I2C0, "i2c0", "dout_pclkp", CLK_GATE_IP3, 7, 0, 0),
	GATE(CLK_I2S1, "i2s1", "dout_pclkp", CLK_GATE_IP3, 5, 0, 0),
	GATE(CLK_I2S0, "i2s0", "dout_pclkp", CLK_GATE_IP3, 4, 0, 0),

	GATE(CLK_SECKEY, "seckey", "dout_pclkp", CLK_GATE_IP4, 3, 0, 0),
	GATE(CLK_CHIPID, "chipid", "dout_pclkp", CLK_GATE_IP4, 0, 0, 0),

	GATE(SCLK_AUDIO1, "sclk_audio1", "dout_audio1", CLK_SRC_MASK0, 25,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_AUDIO0, "sclk_audio0", "dout_audio0", CLK_SRC_MASK0, 24,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_PWM, "sclk_pwm", "dout_pwm", CLK_SRC_MASK0, 19,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_SPI0, "sclk_spi0", "dout_spi0", CLK_SRC_MASK0, 16,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_UART2, "sclk_uart2", "dout_uart2", CLK_SRC_MASK0, 14,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_UART1, "sclk_uart1", "dout_uart1", CLK_SRC_MASK0, 13,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_UART0, "sclk_uart0", "dout_uart0", CLK_SRC_MASK0, 12,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_MMC2, "sclk_mmc2", "dout_mmc2", CLK_SRC_MASK0, 10,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_MMC1, "sclk_mmc1", "dout_mmc1", CLK_SRC_MASK0, 9,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_MMC0, "sclk_mmc0", "dout_mmc0", CLK_SRC_MASK0, 8,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_FIMD, "sclk_fimd", "dout_fimd", CLK_SRC_MASK0, 5,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_CAM1, "sclk_cam1", "dout_cam1", CLK_SRC_MASK0, 4,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_CAM0, "sclk_cam0", "dout_cam0", CLK_SRC_MASK0, 3,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_MIXER, "sclk_mixer", "mout_mixer", CLK_SRC_MASK0, 1,
			CLK_SET_RATE_PARENT, 0),

	GATE(SCLK_FIMC2, "sclk_fimc2", "dout_fimc2", CLK_SRC_MASK1, 4,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_FIMC1, "sclk_fimc1", "dout_fimc1", CLK_SRC_MASK1, 3,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_FIMC0, "sclk_fimc0", "dout_fimc0", CLK_SRC_MASK1, 2,
			CLK_SET_RATE_PARENT, 0),
};

/* S5PV210-specific clock gates. */
const samsung_gate_clock s5pv210_gate_clks[]  = {
	GATE(CLK_CSIS, "clk_csis", "dout_hclkd", CLK_GATE_IP0, 31, 0, 0),
	GATE(CLK_MFC, "mfc", "dout_hclkm", CLK_GATE_IP0, 16, 0, 0),
	GATE(CLK_G2D, "g2d", "dout_hclkd", CLK_GATE_IP0, 12, 0, 0),
	GATE(CLK_G3D, "g3d", "dout_hclkm", CLK_GATE_IP0, 8, 0, 0),
	GATE(CLK_IMEM, "imem", "dout_hclkm", CLK_GATE_IP0, 5, 0, 0),
	GATE(CLK_PDMA1, "pdma1", "dout_hclkp", CLK_GATE_IP0, 4, 0, 0),

	GATE(CLK_NFCON, "nfcon", "dout_hclkp", CLK_GATE_IP1, 28, 0, 0),
	GATE(CLK_CFCON, "cfcon", "dout_hclkp", CLK_GATE_IP1, 25, 0, 0),
	GATE(CLK_USB_HOST, "usb_host", "dout_hclkp", CLK_GATE_IP1, 17, 0, 0),
	GATE(CLK_HDMI, "hdmi", "dout_hclkd", CLK_GATE_IP1, 11, 0, 0),
	GATE(CLK_DSIM, "dsim", "dout_pclkd", CLK_GATE_IP1, 2, 0, 0),

	GATE(CLK_TZIC3, "tzic3", "dout_hclkm", CLK_GATE_IP2, 31, 0, 0),
	GATE(CLK_TZIC2, "tzic2", "dout_hclkm", CLK_GATE_IP2, 30, 0, 0),
	GATE(CLK_TZIC1, "tzic1", "dout_hclkm", CLK_GATE_IP2, 29, 0, 0),
	GATE(CLK_TZIC0, "tzic0", "dout_hclkm", CLK_GATE_IP2, 28, 0, 0),
	GATE(CLK_TSI, "tsi", "dout_hclkd", CLK_GATE_IP2, 20, 0, 0),
	GATE(CLK_HSMMC3, "hsmmc3", "dout_hclkp", CLK_GATE_IP2, 19, 0, 0),
	GATE(CLK_JTAG, "jtag", "dout_hclkp", CLK_GATE_IP2, 11, 0, 0),
	GATE(CLK_CORESIGHT, "coresight", "dout_pclkp", CLK_GATE_IP2, 8, 0, 0),
	GATE(CLK_SDM, "sdm", "dout_pclkm", CLK_GATE_IP2, 1, 0, 0),

	GATE(CLK_PCM2, "pcm2", "dout_pclkp", CLK_GATE_IP3, 30, 0, 0),
	GATE(CLK_UART3, "uart3", "dout_pclkp", CLK_GATE_IP3, 20, 0, 0),
	GATE(CLK_SPI1, "spi1", "dout_pclkp", CLK_GATE_IP3, 13, 0, 0),
	GATE(CLK_I2C_HDMI_PHY, "i2c_hdmi_phy", "dout_pclkd",
			CLK_GATE_IP3, 11, 0, 0),
	GATE(CLK_I2C1, "i2c1", "dout_pclkd", CLK_GATE_IP3, 10, 0, 0),
	GATE(CLK_I2S2, "i2s2", "dout_pclkp", CLK_GATE_IP3, 6, 0, 0),
	GATE(CLK_AC97, "ac97", "dout_pclkp", CLK_GATE_IP3, 1, 0, 0),
	GATE(CLK_SPDIF, "spdif", "dout_pclkp", CLK_GATE_IP3, 0, 0, 0),

	GATE(CLK_TZPC3, "tzpc.3", "dout_pclkd", CLK_GATE_IP4, 8, 0, 0),
	GATE(CLK_TZPC2, "tzpc.2", "dout_pclkd", CLK_GATE_IP4, 7, 0, 0),
	GATE(CLK_TZPC1, "tzpc.1", "dout_pclkp", CLK_GATE_IP4, 6, 0, 0),
	GATE(CLK_TZPC0, "tzpc.0", "dout_pclkm", CLK_GATE_IP4, 5, 0, 0),
	GATE(CLK_IEM_APC, "iem_apc", "dout_pclkp", CLK_GATE_IP4, 2, 0, 0),
	GATE(CLK_IEM_IEC, "iem_iec", "dout_pclkp", CLK_GATE_IP4, 1, 0, 0),

	GATE(CLK_JPEG, "jpeg", "dout_hclkd", CLK_GATE_IP5, 29, 0, 0),

	GATE(SCLK_SPDIF, "sclk_spdif", "mout_spdif", CLK_SRC_MASK0, 27,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_AUDIO2, "sclk_audio2", "dout_audio2", CLK_SRC_MASK0, 26,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_SPI1, "sclk_spi1", "dout_spi1", CLK_SRC_MASK0, 17,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_UART3, "sclk_uart3", "dout_uart3", CLK_SRC_MASK0, 15,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_MMC3, "sclk_mmc3", "dout_mmc3", CLK_SRC_MASK0, 11,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_CSIS, "sclk_csis", "dout_csis", CLK_SRC_MASK0, 6,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_DAC, "sclk_dac", "mout_dac", CLK_SRC_MASK0, 2,
			CLK_SET_RATE_PARENT, 0),
	GATE(SCLK_HDMI, "sclk_hdmi", "mout_hdmi", CLK_SRC_MASK0, 0,
			CLK_SET_RATE_PARENT, 0),
};

/* S5P6442-specific clock gates. */
const samsung_gate_clock s5p6442_gate_clks[]  = {
	GATE(CLK_JPEG, "jpeg", "dout_hclkd", CLK_GATE_IP0, 28, 0, 0),
	GATE(CLK_MFC, "mfc", "dout_hclkd", CLK_GATE_IP0, 16, 0, 0),
	GATE(CLK_G2D, "g2d", "dout_hclkd", CLK_GATE_IP0, 12, 0, 0),
	GATE(CLK_G3D, "g3d", "dout_hclkd", CLK_GATE_IP0, 8, 0, 0),
	GATE(CLK_IMEM, "imem", "dout_hclkd", CLK_GATE_IP0, 5, 0, 0),

	GATE(CLK_ETB, "etb", "dout_hclkd", CLK_GATE_IP1, 31, 0, 0),
	GATE(CLK_ETM, "etm", "dout_hclkd", CLK_GATE_IP1, 30, 0, 0),

	GATE(CLK_I2C1, "i2c1", "dout_pclkp", CLK_GATE_IP3, 8, 0, 0),

	GATE(SCLK_DAC, "sclk_dac", "mout_vpll", CLK_SRC_MASK0, 2,
			CLK_SET_RATE_PARENT, 0),
};

/*
 * Clock aliases for legacy clkdev look-up.
 * NOTE: Needed only to support legacy board files.
 */
const samsung_clock_alias s5pv210_aliases[]  = {
	ALIAS(DOUT_APLL, core::ptr::null_mut(), "armclk"),
	ALIAS(DOUT_HCLKM, core::ptr::null_mut(), "hclk_msys"),
	ALIAS(MOUT_DMC0, core::ptr::null_mut(), "sclk_dmc0"),
};

/* S5PV210-specific PLLs. */
const samsung_pll_clock s5pv210_pll_clks[]  = {
	apll = PLL(pll_4508, FOUT_APLL, "fout_apll", "fin_pll",
						APLL_LOCK, APLL_CON0, core::ptr::null_mut()),
	mpll = PLL(pll_4502, FOUT_MPLL, "fout_mpll", "fin_pll",
						MPLL_LOCK, MPLL_CON, core::ptr::null_mut()),
	epll = PLL(pll_4600, FOUT_EPLL, "fout_epll", "fin_pll",
						EPLL_LOCK, EPLL_CON0, core::ptr::null_mut()),
	vpll = PLL(pll_4502, FOUT_VPLL, "fout_vpll", "mout_vpllsrc",
						VPLL_LOCK, VPLL_CON, core::ptr::null_mut()),
};

/* S5P6442-specific PLLs. */
const samsung_pll_clock s5p6442_pll_clks[]  = {
	apll = PLL(pll_4502, FOUT_APLL, "fout_apll", "fin_pll",
						APLL_LOCK, APLL_CON0, core::ptr::null_mut()),
	mpll = PLL(pll_4502, FOUT_MPLL, "fout_mpll", "fin_pll",
						MPLL_LOCK, MPLL_CON, core::ptr::null_mut()),
	epll = PLL(pll_4500, FOUT_EPLL, "fout_epll", "fin_pll",
						EPLL_LOCK, EPLL_CON0, core::ptr::null_mut()),
	vpll = PLL(pll_4500, FOUT_VPLL, "fout_vpll", "fin_pll",
						VPLL_LOCK, VPLL_CON, core::ptr::null_mut()),
};

unsafe fn __s5pv210_clk_init(device_node *np,
				      u64 xxti_f,
				      u64 xusbxti_f,
				      bool is_s5p6442)
{
	samsung_clk_provider *ctx;
	clk_hw **hws;

	ctx = samsung_clk_init(core::ptr::null_mut(), reg_base, NR_CLKS);
	hws = ctx->clk_data.hws;

	samsung_clk_register_mux(ctx, early_mux_clks,
					(early_mux_clks.len()));

	if (is_s5p6442) {
		samsung_clk_register_fixed_rate(ctx, s5p6442_frate_clks,
			(s5p6442_frate_clks.len()));
		samsung_clk_register_pll(ctx, s5p6442_pll_clks,
			(s5p6442_pll_clks.len()));
		samsung_clk_register_mux(ctx, s5p6442_mux_clks,
				(s5p6442_mux_clks.len()));
		samsung_clk_register_div(ctx, s5p6442_div_clks,
				(s5p6442_div_clks.len()));
		samsung_clk_register_gate(ctx, s5p6442_gate_clks,
				(s5p6442_gate_clks.len()));
	} else {
		samsung_clk_register_fixed_rate(ctx, s5pv210_frate_clks,
			(s5pv210_frate_clks.len()));
		samsung_clk_register_pll(ctx, s5pv210_pll_clks,
			(s5pv210_pll_clks.len()));
		samsung_clk_register_mux(ctx, s5pv210_mux_clks,
				(s5pv210_mux_clks.len()));
		samsung_clk_register_div(ctx, s5pv210_div_clks,
				(s5pv210_div_clks.len()));
		samsung_clk_register_gate(ctx, s5pv210_gate_clks,
				(s5pv210_gate_clks.len()));
	}

	samsung_clk_register_mux(ctx, mux_clks, (mux_clks.len()));
	samsung_clk_register_div(ctx, div_clks, (div_clks.len()));
	samsung_clk_register_gate(ctx, gate_clks, (gate_clks.len()));

	samsung_clk_register_fixed_factor(ctx, ffactor_clks,
						(ffactor_clks.len()));

	samsung_clk_register_alias(ctx, s5pv210_aliases,
						(s5pv210_aliases.len()));

	samsung_clk_sleep_init(reg_base, core::ptr::null_mut(), s5pv210_clk_regs,
			       (s5pv210_clk_regs.len()));

	samsung_clk_of_add_provider(np, ctx);

	pr_info("%s clocks: mout_apll = %ld, mout_mpll = %ld\n"
		"\tmout_epll = %ld, mout_vpll = %ld\n",
		is_s5p6442 ? "S5P6442" : "S5PV210",
		clk_hw_get_rate(hws[MOUT_APLL]),
		clk_hw_get_rate(hws[MOUT_MPLL]),
		clk_hw_get_rate(hws[MOUT_EPLL]),
		clk_hw_get_rate(hws[MOUT_VPLL]));
}

unsafe fn s5pv210_clk_dt_init(device_node *np)
{
	reg_base = of_iomap(np, 0);
	if reg_base.is_null()
		panic("%s: failed to map registers\n", __func__);

	__s5pv210_clk_init(np, 0, 0, false);
}
// CLK_OF_DECLARE(s5pv210_clk, "samsung,s5pv210-clock", s5pv210_clk_dt_init);

unsafe fn s5p6442_clk_dt_init(device_node *np)
{
	reg_base = of_iomap(np, 0);
	if reg_base.is_null()
		panic("%s: failed to map registers\n", __func__);

	__s5pv210_clk_init(np, 0, 0, true);
}
// CLK_OF_DECLARE(s5p6442_clk, "samsung,s5p6442-clock", s5p6442_clk_dt_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
