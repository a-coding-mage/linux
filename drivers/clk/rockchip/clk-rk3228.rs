// Translated from clk-rk3228.c; external kernel symbols and initializer macros are dependencies.\n// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2015 Rockchip Electronics Co. Ltd.
 * Author: Xing Zheng <zhengxing@rock-chips.com>
 *         Jeffy Chen <jeffy.chen@rock-chips.com>
 */


const RK3228_GRF_SOC_STATUS0: u32 = 0x480;

#[repr(usize)]\nenum Rk3228Plls {\n    Apll, Dpll, Cpll, Gpll,\n}

static mut rk3228_pll_rates: [rockchip_pll_rate_table; 43] = {
	/* _mhz, _refdiv, _fbdiv, _postdiv1, _postdiv2, _dsmpd, _frac */
	rk3036_pll_rate!(1608000000, 1, 67, 1, 1, 1, 0),
	rk3036_pll_rate!(1584000000, 1, 66, 1, 1, 1, 0),
	rk3036_pll_rate!(1560000000, 1, 65, 1, 1, 1, 0),
	rk3036_pll_rate!(1536000000, 1, 64, 1, 1, 1, 0),
	rk3036_pll_rate!(1512000000, 1, 63, 1, 1, 1, 0),
	rk3036_pll_rate!(1488000000, 1, 62, 1, 1, 1, 0),
	rk3036_pll_rate!(1464000000, 1, 61, 1, 1, 1, 0),
	rk3036_pll_rate!(1440000000, 1, 60, 1, 1, 1, 0),
	rk3036_pll_rate!(1416000000, 1, 59, 1, 1, 1, 0),
	rk3036_pll_rate!(1392000000, 1, 58, 1, 1, 1, 0),
	rk3036_pll_rate!(1368000000, 1, 57, 1, 1, 1, 0),
	rk3036_pll_rate!(1344000000, 1, 56, 1, 1, 1, 0),
	rk3036_pll_rate!(1320000000, 1, 55, 1, 1, 1, 0),
	rk3036_pll_rate!(1296000000, 1, 54, 1, 1, 1, 0),
	rk3036_pll_rate!(1272000000, 1, 53, 1, 1, 1, 0),
	rk3036_pll_rate!(1248000000, 1, 52, 1, 1, 1, 0),
	rk3036_pll_rate!(1200000000, 1, 50, 1, 1, 1, 0),
	rk3036_pll_rate!(1188000000, 2, 99, 1, 1, 1, 0),
	rk3036_pll_rate!(1104000000, 1, 46, 1, 1, 1, 0),
	rk3036_pll_rate!(1100000000, 12, 550, 1, 1, 1, 0),
	rk3036_pll_rate!(1008000000, 1, 84, 2, 1, 1, 0),
	rk3036_pll_rate!(1000000000, 6, 500, 2, 1, 1, 0),
	rk3036_pll_rate!( 984000000, 1, 82, 2, 1, 1, 0),
	rk3036_pll_rate!( 960000000, 1, 80, 2, 1, 1, 0),
	rk3036_pll_rate!( 936000000, 1, 78, 2, 1, 1, 0),
	rk3036_pll_rate!( 912000000, 1, 76, 2, 1, 1, 0),
	rk3036_pll_rate!( 900000000, 4, 300, 2, 1, 1, 0),
	rk3036_pll_rate!( 888000000, 1, 74, 2, 1, 1, 0),
	rk3036_pll_rate!( 864000000, 1, 72, 2, 1, 1, 0),
	rk3036_pll_rate!( 840000000, 1, 70, 2, 1, 1, 0),
	rk3036_pll_rate!( 816000000, 1, 68, 2, 1, 1, 0),
	rk3036_pll_rate!( 800000000, 6, 400, 2, 1, 1, 0),
	rk3036_pll_rate!( 700000000, 6, 350, 2, 1, 1, 0),
	rk3036_pll_rate!( 696000000, 1, 58, 2, 1, 1, 0),
	rk3036_pll_rate!( 600000000, 1, 75, 3, 1, 1, 0),
	rk3036_pll_rate!( 594000000, 2, 99, 2, 1, 1, 0),
	rk3036_pll_rate!( 504000000, 1, 63, 3, 1, 1, 0),
	rk3036_pll_rate!( 500000000, 6, 250, 2, 1, 1, 0),
	rk3036_pll_rate!( 408000000, 1, 68, 2, 2, 1, 0),
	rk3036_pll_rate!( 312000000, 1, 52, 2, 2, 1, 0),
	rk3036_pll_rate!( 216000000, 1, 72, 4, 2, 1, 0),
	rk3036_pll_rate!(  96000000, 1, 64, 4, 4, 1, 0),
	{ /* sentinel */ },
};

const RK3228_DIV_CPU_MASK: u32 = 0x1f;
const RK3228_DIV_CPU_SHIFT: u32 = 8;

const RK3228_DIV_PERI_MASK: u32 = 0xf;
const RK3228_DIV_PERI_SHIFT: u32 = 0;
const RK3228_DIV_ACLK_MASK: u32 = 0x7;
const RK3228_DIV_ACLK_SHIFT: u32 = 4;
const RK3228_DIV_HCLK_MASK: u32 = 0x3;
const RK3228_DIV_HCLK_SHIFT: u32 = 8;
const RK3228_DIV_PCLK_MASK: u32 = 0x7;
const RK3228_DIV_PCLK_SHIFT: u32 = 12;

}

	}

static mut rk3228_cpuclk_rates: &[rockchip_cpuclk_rate_table] = {
	rk3228_cpuclk_rate!(1800000000, 1, 7),
	rk3228_cpuclk_rate!(1704000000, 1, 7),
	rk3228_cpuclk_rate!(1608000000, 1, 7),
	rk3228_cpuclk_rate!(1512000000, 1, 7),
	rk3228_cpuclk_rate!(1488000000, 1, 5),
	rk3228_cpuclk_rate!(1464000000, 1, 5),
	rk3228_cpuclk_rate!(1416000000, 1, 5),
	rk3228_cpuclk_rate!(1392000000, 1, 5),
	rk3228_cpuclk_rate!(1296000000, 1, 5),
	rk3228_cpuclk_rate!(1200000000, 1, 5),
	rk3228_cpuclk_rate!(1104000000, 1, 5),
	rk3228_cpuclk_rate!(1008000000, 1, 5),
	rk3228_cpuclk_rate!(912000000, 1, 5),
	rk3228_cpuclk_rate!(816000000, 1, 3),
	rk3228_cpuclk_rate!(696000000, 1, 3),
	rk3228_cpuclk_rate!(600000000, 1, 3),
	rk3228_cpuclk_rate!(408000000, 1, 1),
	rk3228_cpuclk_rate!(312000000, 1, 1),
	rk3228_cpuclk_rate!(216000000,  1, 1),
	rk3228_cpuclk_rate!(96000000, 1, 1),
};

static rk3228_cpuclk_data: rockchip_cpuclk_reg_data = {
	.core_reg[0] = RK2928_CLKSEL_CON(0),
	.div_core_shift[0] = 0,
	.div_core_mask[0] = 0x1f,
	.num_cores = 1,
	.mux_core_alt = 1,
	.mux_core_main = 0,
	.mux_core_shift = 6,
	.mux_core_mask = 0x1,
};

static mux_pll_p: &[&str] = &[ "clk_24m", "xin24m" ];

static mux_ddrphy_p: &[&str] = &[ "dpll_ddr", "gpll_ddr", "apll_ddr" ];
static mux_armclk_p: &[&str] = &[ "apll_core", "gpll_core", "dpll_core" ];
static mux_usb480m_phy_p: &[&str] = &[ "usb480m_phy0", "usb480m_phy1" ];
static mux_usb480m_p: &[&str] = &[ "usb480m_phy", "xin24m" ];
static mux_hdmiphy_p: &[&str] = &[ "hdmiphy_phy", "xin24m" ];
static mux_aclk_cpu_src_p: &[&str] = &[ "cpll_aclk_cpu", "gpll_aclk_cpu", "hdmiphy_aclk_cpu" ];

static mux_pll_src_4plls_p: &[&str] = &[ "cpll", "gpll", "hdmiphy", "usb480m" ];
static mux_pll_src_3plls_p: &[&str] = &[ "cpll", "gpll", "hdmiphy" ];
static mux_pll_src_2plls_p: &[&str] = &[ "cpll", "gpll" ];
static mux_sclk_hdmi_cec_p: &[&str] = &[ "cpll", "gpll", "xin24m" ];
static mux_aclk_peri_src_p: &[&str] = &[ "cpll_peri", "gpll_peri", "hdmiphy_peri" ];
static mux_mmc_src_p: &[&str] = &[ "cpll", "gpll", "xin24m", "usb480m" ];
static mux_pll_src_cpll_gpll_usb480m_p: &[&str] = &[ "cpll", "gpll", "usb480m" ];

static mux_sclk_rga_p: &[&str] = &[ "gpll", "cpll", "sclk_rga_src" ];

static mux_sclk_vop_src_p: &[&str] = &[ "gpll_vop", "cpll_vop" ];
static mux_dclk_vop_p: &[&str] = &[ "hdmiphy", "sclk_vop_pre" ];

static mux_i2s0_p: &[&str] = &[ "i2s0_src", "i2s0_frac", "ext_i2s", "xin12m" ];
static mux_i2s1_pre_p: &[&str] = &[ "i2s1_src", "i2s1_frac", "ext_i2s", "xin12m" ];
static mux_i2s_out_p: &[&str] = &[ "i2s1_pre", "xin12m" ];
static mux_i2s2_p: &[&str] = &[ "i2s2_src", "i2s2_frac", "xin12m" ];
static mux_sclk_spdif_p: &[&str] = &[ "sclk_spdif_src", "spdif_frac", "xin12m" ];

static mux_uart0_p: &[&str] = &[ "uart0_src", "uart0_frac", "xin24m" ];
static mux_uart1_p: &[&str] = &[ "uart1_src", "uart1_frac", "xin24m" ];
static mux_uart2_p: &[&str] = &[ "uart2_src", "uart2_frac", "xin24m" ];

static mux_sclk_mac_extclk_p: &[&str] = &[ "ext_gmac", "phy_50m_out" ];
static mux_sclk_gmac_pre_p: &[&str] = &[ "sclk_gmac_src", "sclk_mac_extclk" ];
static mux_sclk_macphy_p: &[&str] = &[ "sclk_gmac_src", "ext_gmac" ];

static mut rk3228_pll_clks: &[rockchip_pll_clock] = {
	[apll] = pll!(pll_rk3036, PLL_APLL, "apll", mux_pll_p, 0, RK2928_PLL_CON(0),
		     RK2928_MODE_CON, 0, 7, 0, rk3228_pll_rates),
	[dpll] = pll!(pll_rk3036, PLL_DPLL, "dpll", mux_pll_p, 0, RK2928_PLL_CON(3),
		     RK2928_MODE_CON, 4, 6, 0, NULL),
	[cpll] = pll!(pll_rk3036, PLL_CPLL, "cpll", mux_pll_p, 0, RK2928_PLL_CON(6),
		     RK2928_MODE_CON, 8, 8, 0, NULL),
	[gpll] = pll!(pll_rk3036, PLL_GPLL, "gpll", mux_pll_p, 0, RK2928_PLL_CON(9),
		     RK2928_MODE_CON, 12, 9, ROCKCHIP_PLL_SYNC_RATE, rk3228_pll_rates),
};

const MFLAGS: u32 = CLK_MUX_HIWORD_MASK;
const DFLAGS: u32 = CLK_DIVIDER_HIWORD_MASK;
const GFLAGS: u32 = (CLK_GATE_HIWORD_MASK | CLK_GATE_SET_TO_DISABLE);

static mut rk3228_i2s0_fracmux: rockchip_clk_branch =
	mux!(0, "i2s0_pre", mux_i2s0_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(9), 8, 2, MFLAGS);

static mut rk3228_i2s1_fracmux: rockchip_clk_branch =
	mux!(0, "i2s1_pre", mux_i2s1_pre_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(3), 8, 2, MFLAGS);

static mut rk3228_i2s2_fracmux: rockchip_clk_branch =
	mux!(0, "i2s2_pre", mux_i2s2_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(16), 8, 2, MFLAGS);

static mut rk3228_spdif_fracmux: rockchip_clk_branch =
	mux!(SCLK_SPDIF, "sclk_spdif", mux_sclk_spdif_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(6), 8, 2, MFLAGS);

static mut rk3228_uart0_fracmux: rockchip_clk_branch =
	mux!(SCLK_UART0, "sclk_uart0", mux_uart0_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(13), 8, 2, MFLAGS);

static mut rk3228_uart1_fracmux: rockchip_clk_branch =
	mux!(SCLK_UART1, "sclk_uart1", mux_uart1_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(14), 8, 2, MFLAGS);

static mut rk3228_uart2_fracmux: rockchip_clk_branch =
	mux!(SCLK_UART2, "sclk_uart2", mux_uart2_p, CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(15), 8, 2, MFLAGS);

static mut rk3228_clk_branches: &[rockchip_clk_branch] = {
	/*
	 * Clock-Architecture Diagram 1
	 */

	div!(0, "clk_24m", "xin24m", CLK_IGNORE_UNUSED,
			RK2928_CLKSEL_CON(4), 8, 5, DFLAGS),

	/* PD_DDR */
	gate!(0, "apll_ddr", "apll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 2, GFLAGS),
	gate!(0, "dpll_ddr", "dpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 2, GFLAGS),
	gate!(0, "gpll_ddr", "gpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 2, GFLAGS),
	composite!(0, "ddrphy4x", mux_ddrphy_p, CLK_IGNORE_UNUSED,
			RK2928_CLKSEL_CON(26), 8, 2, MFLAGS, 0, 3, DFLAGS | CLK_DIVIDER_POWER_OF_TWO,
			RK2928_CLKGATE_CON(7), 1, GFLAGS),
	gate!(0, "ddrc", "ddrphy_pre", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(8), 5, GFLAGS),
	FACTOR_gate!(0, "ddrphy", "ddrphy4x", CLK_IGNORE_UNUSED, 1, 4,
			RK2928_CLKGATE_CON(7), 0, GFLAGS),

	/* PD_CORE */
	gate!(0, "dpll_core", "dpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 6, GFLAGS),
	gate!(0, "apll_core", "apll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 6, GFLAGS),
	gate!(0, "gpll_core", "gpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 6, GFLAGS),
	COMPOSITE_NOmux!(0, "pclk_dbg", "armclk", CLK_IGNORE_UNUSED,
			RK2928_CLKSEL_CON(1), 0, 4, DFLAGS | CLK_DIVIDER_READ_ONLY,
			RK2928_CLKGATE_CON(4), 1, GFLAGS),
	COMPOSITE_NOmux!(0, "armcore", "armclk", CLK_IGNORE_UNUSED,
			RK2928_CLKSEL_CON(1), 4, 3, DFLAGS | CLK_DIVIDER_READ_ONLY,
			RK2928_CLKGATE_CON(4), 0, GFLAGS),

	/* PD_MISC */
	mux!(SCLK_HDMI_PHY, "hdmiphy", mux_hdmiphy_p, CLK_SET_RATE_PARENT,
			RK2928_MISC_CON, 13, 1, MFLAGS),
	mux!(0, "usb480m_phy", mux_usb480m_phy_p, CLK_SET_RATE_PARENT,
			RK2928_MISC_CON, 14, 1, MFLAGS),
	mux!(0, "usb480m", mux_usb480m_p, CLK_SET_RATE_PARENT,
			RK2928_MISC_CON, 15, 1, MFLAGS),

	/* PD_BUS */
	gate!(0, "hdmiphy_aclk_cpu", "hdmiphy", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 1, GFLAGS),
	gate!(0, "gpll_aclk_cpu", "gpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 1, GFLAGS),
	gate!(0, "cpll_aclk_cpu", "cpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(0), 1, GFLAGS),
	COMPOSITE_NOgate!(0, "aclk_cpu_src", mux_aclk_cpu_src_p, 0,
			RK2928_CLKSEL_CON(0), 13, 2, MFLAGS, 8, 5, DFLAGS),
	gate!(ACLK_CPU, "aclk_cpu", "aclk_cpu_src", 0,
			RK2928_CLKGATE_CON(6), 0, GFLAGS),
	COMPOSITE_NOmux!(HCLK_CPU, "hclk_cpu", "aclk_cpu_src", 0,
			RK2928_CLKSEL_CON(1), 8, 2, DFLAGS,
			RK2928_CLKGATE_CON(6), 1, GFLAGS),
	COMPOSITE_NOmux!(0, "pclk_bus_src", "aclk_cpu_src", 0,
			RK2928_CLKSEL_CON(1), 12, 3, DFLAGS,
			RK2928_CLKGATE_CON(6), 2, GFLAGS),
	gate!(PCLK_CPU, "pclk_cpu", "pclk_bus_src", 0,
			RK2928_CLKGATE_CON(6), 3, GFLAGS),
	gate!(0, "pclk_phy_pre", "pclk_bus_src", 0,
			RK2928_CLKGATE_CON(6), 4, GFLAGS),
	gate!(0, "pclk_ddr_pre", "pclk_bus_src", 0,
			RK2928_CLKGATE_CON(6), 13, GFLAGS),

	/* PD_VIDEO */
	composite!(ACLK_VPU_PRE, "aclk_vpu_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(32), 5, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 11, GFLAGS),
	FACTOR_gate!(HCLK_VPU_PRE, "hclk_vpu_pre", "aclk_vpu_pre", 0, 1, 4,
			RK2928_CLKGATE_CON(4), 4, GFLAGS),

	composite!(ACLK_RKVDEC_PRE, "aclk_rkvdec_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(28), 6, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 2, GFLAGS),
	FACTOR_gate!(HCLK_RKVDEC_PRE, "hclk_rkvdec_pre", "aclk_rkvdec_pre", 0, 1, 4,
			RK2928_CLKGATE_CON(4), 5, GFLAGS),

	composite!(SCLK_VDEC_CABAC, "sclk_vdec_cabac", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(28), 14, 2, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 3, GFLAGS),

	composite!(SCLK_VDEC_CORE, "sclk_vdec_core", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(34), 13, 2, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 4, GFLAGS),

	/* PD_VIO */
	composite!(ACLK_IEP_PRE, "aclk_iep_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(31), 5, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 0, GFLAGS),
	div!(HCLK_VIO_PRE, "hclk_vio_pre", "aclk_iep_pre", 0,
			RK2928_CLKSEL_CON(2), 0, 5, DFLAGS),

	composite!(ACLK_HDCP_PRE, "aclk_hdcp_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(31), 13, 2, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(1), 4, GFLAGS),

	mux!(0, "sclk_rga_src", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(33), 13, 2, MFLAGS),
	COMPOSITE_NOmux!(ACLK_RGA_PRE, "aclk_rga_pre", "sclk_rga_src", 0,
			RK2928_CLKSEL_CON(33), 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(1), 2, GFLAGS),
	composite!(SCLK_RGA, "sclk_rga", mux_sclk_rga_p, 0,
			RK2928_CLKSEL_CON(22), 5, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 6, GFLAGS),

	composite!(ACLK_VOP_PRE, "aclk_vop_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(33), 5, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(1), 1, GFLAGS),

	composite!(SCLK_HDCP, "sclk_hdcp", mux_pll_src_3plls_p, 0,
			RK2928_CLKSEL_CON(23), 14, 2, MFLAGS, 8, 6, DFLAGS,
			RK2928_CLKGATE_CON(3), 5, GFLAGS),

	gate!(SCLK_HDMI_HDCP, "sclk_hdmi_hdcp", "xin24m", 0,
			RK2928_CLKGATE_CON(3), 7, GFLAGS),

	composite!(SCLK_HDMI_CEC, "sclk_hdmi_cec", mux_sclk_hdmi_cec_p, 0,
			RK2928_CLKSEL_CON(21), 14, 2, MFLAGS, 0, 14, DFLAGS,
			RK2928_CLKGATE_CON(3), 8, GFLAGS),

	/* PD_PERI */
	gate!(0, "cpll_peri", "cpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(2), 0, GFLAGS),
	gate!(0, "gpll_peri", "gpll", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(2), 0, GFLAGS),
	gate!(0, "hdmiphy_peri", "hdmiphy", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(2), 0, GFLAGS),
	COMPOSITE_NOgate!(0, "aclk_peri_src", mux_aclk_peri_src_p, 0,
			RK2928_CLKSEL_CON(10), 10, 2, MFLAGS, 0, 5, DFLAGS),
	COMPOSITE_NOmux!(PCLK_PERI, "pclk_peri", "aclk_peri_src", 0,
			RK2928_CLKSEL_CON(10), 12, 3, DFLAGS,
			RK2928_CLKGATE_CON(5), 2, GFLAGS),
	COMPOSITE_NOmux!(HCLK_PERI, "hclk_peri", "aclk_peri_src", 0,
			RK2928_CLKSEL_CON(10), 8, 2, DFLAGS,
			RK2928_CLKGATE_CON(5), 1, GFLAGS),
	gate!(ACLK_PERI, "aclk_peri", "aclk_peri_src", 0,
			RK2928_CLKGATE_CON(5), 0, GFLAGS),

	gate!(SCLK_TIMER0, "sclk_timer0", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 5, GFLAGS),
	gate!(SCLK_TIMER1, "sclk_timer1", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 6, GFLAGS),
	gate!(SCLK_TIMER2, "sclk_timer2", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 7, GFLAGS),
	gate!(SCLK_TIMER3, "sclk_timer3", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 8, GFLAGS),
	gate!(SCLK_TIMER4, "sclk_timer4", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 9, GFLAGS),
	gate!(SCLK_TIMER5, "sclk_timer5", "xin24m", 0,
			RK2928_CLKGATE_CON(6), 10, GFLAGS),

	composite!(SCLK_CRYPTO, "sclk_crypto", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(24), 5, 1, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(2), 7, GFLAGS),

	composite!(SCLK_TSP, "sclk_tsp", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(22), 15, 1, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(2), 6, GFLAGS),

	gate!(SCLK_HSADC, "sclk_hsadc", "ext_hsadc", 0,
			RK2928_CLKGATE_CON(10), 12, GFLAGS),

	composite!(SCLK_WIFI, "sclk_wifi", mux_pll_src_cpll_gpll_usb480m_p, 0,
			RK2928_CLKSEL_CON(23), 5, 2, MFLAGS, 0, 6, DFLAGS,
			RK2928_CLKGATE_CON(2), 15, GFLAGS),

	composite!(SCLK_SDMMC, "sclk_sdmmc", mux_mmc_src_p, 0,
			RK2928_CLKSEL_CON(11), 8, 2, MFLAGS, 0, 8, DFLAGS,
			RK2928_CLKGATE_CON(2), 11, GFLAGS),

	COMPOSITE_NOdiv!(SCLK_SDIO_SRC, "sclk_sdio_src", mux_mmc_src_p, 0,
			RK2928_CLKSEL_CON(11), 10, 2, MFLAGS,
			RK2928_CLKGATE_CON(2), 13, GFLAGS),
	div!(SCLK_SDIO, "sclk_sdio", "sclk_sdio_src", 0,
			RK2928_CLKSEL_CON(12), 0, 8, DFLAGS),

	COMPOSITE_NOdiv!(0, "sclk_emmc_src", mux_mmc_src_p, 0,
			RK2928_CLKSEL_CON(11), 12, 2, MFLAGS,
			RK2928_CLKGATE_CON(2), 14, GFLAGS),
	div!(SCLK_EMMC, "sclk_emmc", "sclk_emmc_src", 0,
			RK2928_CLKSEL_CON(12), 8, 8, DFLAGS),

	/*
	 * Clock-Architecture Diagram 2
	 */

	gate!(0, "gpll_vop", "gpll", 0,
			RK2928_CLKGATE_CON(3), 1, GFLAGS),
	gate!(0, "cpll_vop", "cpll", 0,
			RK2928_CLKGATE_CON(3), 1, GFLAGS),
	mux!(0, "sclk_vop_src", mux_sclk_vop_src_p, 0,
			RK2928_CLKSEL_CON(27), 0, 1, MFLAGS),
	div!(DCLK_HDMI_PHY, "dclk_hdmiphy", "sclk_vop_src", 0,
			RK2928_CLKSEL_CON(29), 0, 3, DFLAGS),
	div!(0, "sclk_vop_pre", "sclk_vop_src", 0,
			RK2928_CLKSEL_CON(27), 8, 8, DFLAGS),
	mux!(DCLK_VOP, "dclk_vop", mux_dclk_vop_p, CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
			RK2928_CLKSEL_CON(27), 1, 1, MFLAGS),

	factor!(0, "xin12m", "xin24m", 0, 1, 2),

	composite!(0, "i2s0_src", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(9), 15, 1, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(0), 3, GFLAGS),
	COMPOSITE_FRACmux!(0, "i2s0_frac", "i2s0_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(8), 0,
			RK2928_CLKGATE_CON(0), 4, GFLAGS,
			&rk3228_i2s0_fracmux),
	gate!(SCLK_I2S0, "sclk_i2s0", "i2s0_pre", CLK_SET_RATE_PARENT,
			RK2928_CLKGATE_CON(0), 5, GFLAGS),

	composite!(0, "i2s1_src", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(3), 15, 1, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(0), 10, GFLAGS),
	COMPOSITE_FRACmux!(0, "i2s1_frac", "i2s1_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(7), 0,
			RK2928_CLKGATE_CON(0), 11, GFLAGS,
			&rk3228_i2s1_fracmux),
	gate!(SCLK_I2S1, "sclk_i2s1", "i2s1_pre", CLK_SET_RATE_PARENT,
			RK2928_CLKGATE_CON(0), 14, GFLAGS),
	COMPOSITE_NOdiv!(SCLK_I2S_OUT, "i2s_out", mux_i2s_out_p, 0,
			RK2928_CLKSEL_CON(3), 12, 1, MFLAGS,
			RK2928_CLKGATE_CON(0), 13, GFLAGS),

	composite!(0, "i2s2_src", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(16), 15, 1, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(0), 7, GFLAGS),
	COMPOSITE_FRACmux!(0, "i2s2_frac", "i2s2_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(30), 0,
			RK2928_CLKGATE_CON(0), 8, GFLAGS,
			&rk3228_i2s2_fracmux),
	gate!(SCLK_I2S2, "sclk_i2s2", "i2s2_pre", CLK_SET_RATE_PARENT,
			RK2928_CLKGATE_CON(0), 9, GFLAGS),

	composite!(0, "sclk_spdif_src", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(6), 15, 1, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(2), 10, GFLAGS),
	COMPOSITE_FRACmux!(0, "spdif_frac", "sclk_spdif_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(20), 0,
			RK2928_CLKGATE_CON(2), 12, GFLAGS,
			&rk3228_spdif_fracmux),

	gate!(0, "jtag", "ext_jtag", CLK_IGNORE_UNUSED,
			RK2928_CLKGATE_CON(1), 3, GFLAGS),

	gate!(SCLK_OTGPHY0, "sclk_otgphy0", "xin24m", 0,
			RK2928_CLKGATE_CON(1), 5, GFLAGS),
	gate!(SCLK_OTGPHY1, "sclk_otgphy1", "xin24m", 0,
			RK2928_CLKGATE_CON(1), 6, GFLAGS),

	COMPOSITE_NOmux!(SCLK_TSADC, "sclk_tsadc", "xin24m", 0,
			RK2928_CLKSEL_CON(24), 6, 10, DFLAGS,
			RK2928_CLKGATE_CON(2), 8, GFLAGS),

	composite!(0, "aclk_gpu_pre", mux_pll_src_4plls_p, 0,
			RK2928_CLKSEL_CON(34), 5, 2, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(3), 13, GFLAGS),

	composite!(SCLK_SPI0, "sclk_spi0", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(25), 8, 1, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(2), 9, GFLAGS),

	/* PD_UART */
	composite!(0, "uart0_src", mux_pll_src_cpll_gpll_usb480m_p, 0,
			RK2928_CLKSEL_CON(13), 12, 2, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(1), 8, GFLAGS),
	composite!(0, "uart1_src", mux_pll_src_cpll_gpll_usb480m_p, 0,
			RK2928_CLKSEL_CON(14), 12, 2, MFLAGS, 0, 7, DFLAGS,
			RK2928_CLKGATE_CON(1), 10, GFLAGS),
	composite!(0, "uart2_src", mux_pll_src_cpll_gpll_usb480m_p,
			0, RK2928_CLKSEL_CON(15), 12, 2,
			MFLAGS, 0, 7, DFLAGS, RK2928_CLKGATE_CON(1), 12, GFLAGS),
	COMPOSITE_FRACmux!(0, "uart0_frac", "uart0_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(17), 0,
			RK2928_CLKGATE_CON(1), 9, GFLAGS,
			&rk3228_uart0_fracmux),
	COMPOSITE_FRACmux!(0, "uart1_frac", "uart1_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(18), 0,
			RK2928_CLKGATE_CON(1), 11, GFLAGS,
			&rk3228_uart1_fracmux),
	COMPOSITE_FRACmux!(0, "uart2_frac", "uart2_src", CLK_SET_RATE_PARENT,
			RK2928_CLKSEL_CON(19), 0,
			RK2928_CLKGATE_CON(1), 13, GFLAGS,
			&rk3228_uart2_fracmux),

	composite!(SCLK_NANDC, "sclk_nandc", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(2), 14, 1, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(1), 0, GFLAGS),

	composite!(SCLK_MAC_SRC, "sclk_gmac_src", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(5), 7, 1, MFLAGS, 0, 5, DFLAGS,
			RK2928_CLKGATE_CON(1), 7, GFLAGS),
	mux!(SCLK_MAC_EXTCLK, "sclk_mac_extclk", mux_sclk_mac_extclk_p, 0,
			RK2928_CLKSEL_CON(29), 10, 1, MFLAGS),
	mux!(SCLK_MAC, "sclk_gmac_pre", mux_sclk_gmac_pre_p, 0,
			RK2928_CLKSEL_CON(5), 5, 1, MFLAGS),
	gate!(SCLK_MAC_REFOUT, "sclk_mac_refout", "sclk_gmac_pre", 0,
			RK2928_CLKGATE_CON(5), 4, GFLAGS),
	gate!(SCLK_MAC_REF, "sclk_mac_ref", "sclk_gmac_pre", 0,
			RK2928_CLKGATE_CON(5), 3, GFLAGS),
	gate!(SCLK_MAC_RX, "sclk_mac_rx", "sclk_gmac_pre", 0,
			RK2928_CLKGATE_CON(5), 5, GFLAGS),
	gate!(SCLK_MAC_TX, "sclk_mac_tx", "sclk_gmac_pre", 0,
			RK2928_CLKGATE_CON(5), 6, GFLAGS),
	composite!(SCLK_MAC_PHY, "sclk_macphy", mux_sclk_macphy_p, 0,
			RK2928_CLKSEL_CON(29), 12, 1, MFLAGS, 8, 2, DFLAGS,
			RK2928_CLKGATE_CON(5), 7, GFLAGS),
	composite!(SCLK_MAC_OUT, "sclk_gmac_out", mux_pll_src_2plls_p, 0,
			RK2928_CLKSEL_CON(5), 15, 1, MFLAGS, 8, 5, DFLAGS,
			RK2928_CLKGATE_CON(2), 2, GFLAGS),

	/*
	 * Clock-Architecture Diagram 3
	 */

	/* PD_VOP */
	gate!(ACLK_RGA, "aclk_rga", "aclk_rga_pre", 0, RK2928_CLKGATE_CON(13), 0, GFLAGS),
	gate!(0, "aclk_rga_noc", "aclk_rga_pre", 0, RK2928_CLKGATE_CON(13), 11, GFLAGS),
	gate!(ACLK_IEP, "aclk_iep", "aclk_iep_pre", 0, RK2928_CLKGATE_CON(13), 2, GFLAGS),
	gate!(0, "aclk_iep_noc", "aclk_iep_pre", 0, RK2928_CLKGATE_CON(13), 9, GFLAGS),

	gate!(ACLK_VOP, "aclk_vop", "aclk_vop_pre", 0, RK2928_CLKGATE_CON(13), 5, GFLAGS),
	gate!(0, "aclk_vop_noc", "aclk_vop_pre", 0, RK2928_CLKGATE_CON(13), 12, GFLAGS),

	gate!(ACLK_HDCP, "aclk_hdcp", "aclk_hdcp_pre", 0, RK2928_CLKGATE_CON(14), 10, GFLAGS),
	gate!(0, "aclk_hdcp_noc", "aclk_hdcp_pre", 0, RK2928_CLKGATE_CON(13), 10, GFLAGS),

	gate!(HCLK_RGA, "hclk_rga", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 1, GFLAGS),
	gate!(HCLK_IEP, "hclk_iep", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 3, GFLAGS),
	gate!(HCLK_VOP, "hclk_vop", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 6, GFLAGS),
	gate!(0, "hclk_vio_ahb_arbi", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 7, GFLAGS),
	gate!(0, "hclk_vio_noc", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 8, GFLAGS),
	gate!(0, "hclk_vop_noc", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(13), 13, GFLAGS),
	gate!(HCLK_VIO_H2P, "hclk_vio_h2p", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(14), 7, GFLAGS),
	gate!(HCLK_HDCP_MMU, "hclk_hdcp_mmu", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(14), 12, GFLAGS),
	gate!(PCLK_HDMI_CTRL, "pclk_hdmi_ctrl", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(14), 6, GFLAGS),
	gate!(PCLK_VIO_H2P, "pclk_vio_h2p", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(14), 8, GFLAGS),
	gate!(PCLK_HDCP, "pclk_hdcp", "hclk_vio_pre", 0, RK2928_CLKGATE_CON(14), 11, GFLAGS),

	/* PD_PERI */
	gate!(0, "aclk_peri_noc", "aclk_peri", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(12), 0, GFLAGS),
	gate!(ACLK_GMAC, "aclk_gmac", "aclk_peri", 0, RK2928_CLKGATE_CON(11), 4, GFLAGS),

	gate!(HCLK_SDMMC, "hclk_sdmmc", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 0, GFLAGS),
	gate!(HCLK_SDIO, "hclk_sdio", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 1, GFLAGS),
	gate!(HCLK_EMMC, "hclk_emmc", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 2, GFLAGS),
	gate!(HCLK_NANDC, "hclk_nandc", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 3, GFLAGS),
	gate!(HCLK_HOST0, "hclk_host0", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 6, GFLAGS),
	gate!(0, "hclk_host0_arb", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 7, GFLAGS),
	gate!(HCLK_HOST1, "hclk_host1", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 8, GFLAGS),
	gate!(0, "hclk_host1_arb", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 9, GFLAGS),
	gate!(HCLK_HOST2, "hclk_host2", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 10, GFLAGS),
	gate!(HCLK_OTG, "hclk_otg", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 12, GFLAGS),
	gate!(0, "hclk_otg_pmu", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 13, GFLAGS),
	gate!(0, "hclk_host2_arb", "hclk_peri", 0, RK2928_CLKGATE_CON(11), 14, GFLAGS),
	gate!(0, "hclk_peri_noc", "hclk_peri", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(12), 1, GFLAGS),

	gate!(PCLK_GMAC, "pclk_gmac", "pclk_peri", 0, RK2928_CLKGATE_CON(11), 5, GFLAGS),
	gate!(0, "pclk_peri_noc", "pclk_peri", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(12), 2, GFLAGS),

	/* PD_GPU */
	gate!(ACLK_GPU, "aclk_gpu", "aclk_gpu_pre", 0, RK2928_CLKGATE_CON(7), 14, GFLAGS),
	gate!(0, "aclk_gpu_noc", "aclk_gpu_pre", 0, RK2928_CLKGATE_CON(7), 15, GFLAGS),

	/* PD_BUS */
	gate!(0, "sclk_initmem_mbist", "aclk_cpu", 0, RK2928_CLKGATE_CON(8), 1, GFLAGS),
	gate!(0, "aclk_initmem", "aclk_cpu", 0, RK2928_CLKGATE_CON(8), 0, GFLAGS),
	gate!(ACLK_DMAC, "aclk_dmac_bus", "aclk_cpu", 0, RK2928_CLKGATE_CON(8), 2, GFLAGS),
	gate!(0, "aclk_bus_noc", "aclk_cpu", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(10), 1, GFLAGS),

	gate!(0, "hclk_rom", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 3, GFLAGS),
	gate!(HCLK_I2S0_8CH, "hclk_i2s0_8ch", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 7, GFLAGS),
	gate!(HCLK_I2S1_8CH, "hclk_i2s1_8ch", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 8, GFLAGS),
	gate!(HCLK_I2S2_2CH, "hclk_i2s2_2ch", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 9, GFLAGS),
	gate!(HCLK_SPDIF_8CH, "hclk_spdif_8ch", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 10, GFLAGS),
	gate!(HCLK_TSP, "hclk_tsp", "hclk_cpu", 0, RK2928_CLKGATE_CON(10), 11, GFLAGS),
	gate!(HCLK_M_CRYPTO, "hclk_crypto_mst", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 11, GFLAGS),
	gate!(HCLK_S_CRYPTO, "hclk_crypto_slv", "hclk_cpu", 0, RK2928_CLKGATE_CON(8), 12, GFLAGS),

	gate!(0, "pclk_ddrupctl", "pclk_ddr_pre", 0, RK2928_CLKGATE_CON(8), 4, GFLAGS),
	gate!(0, "pclk_ddrmon", "pclk_ddr_pre", 0, RK2928_CLKGATE_CON(8), 6, GFLAGS),
	gate!(0, "pclk_msch_noc", "pclk_ddr_pre", 0, RK2928_CLKGATE_CON(10), 2, GFLAGS),

	gate!(PCLK_EFUSE_1024, "pclk_efuse_1024", "pclk_cpu", 0, RK2928_CLKGATE_CON(8), 13, GFLAGS),
	gate!(PCLK_EFUSE_256, "pclk_efuse_256", "pclk_cpu", 0, RK2928_CLKGATE_CON(8), 14, GFLAGS),
	gate!(PCLK_I2C0, "pclk_i2c0", "pclk_cpu", 0, RK2928_CLKGATE_CON(8), 15, GFLAGS),
	gate!(PCLK_I2C1, "pclk_i2c1", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 0, GFLAGS),
	gate!(PCLK_I2C2, "pclk_i2c2", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 1, GFLAGS),
	gate!(PCLK_I2C3, "pclk_i2c3", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 2, GFLAGS),
	gate!(PCLK_TIMER, "pclk_timer0", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 4, GFLAGS),
	gate!(0, "pclk_stimer", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 5, GFLAGS),
	gate!(PCLK_SPI0, "pclk_spi0", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 6, GFLAGS),
	gate!(PCLK_PWM, "pclk_rk_pwm", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 7, GFLAGS),
	gate!(PCLK_GPIO0, "pclk_gpio0", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 8, GFLAGS),
	gate!(PCLK_GPIO1, "pclk_gpio1", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 9, GFLAGS),
	gate!(PCLK_GPIO2, "pclk_gpio2", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 10, GFLAGS),
	gate!(PCLK_GPIO3, "pclk_gpio3", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 11, GFLAGS),
	gate!(PCLK_UART0, "pclk_uart0", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 12, GFLAGS),
	gate!(PCLK_UART1, "pclk_uart1", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 13, GFLAGS),
	gate!(PCLK_UART2, "pclk_uart2", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 14, GFLAGS),
	gate!(PCLK_TSADC, "pclk_tsadc", "pclk_cpu", 0, RK2928_CLKGATE_CON(9), 15, GFLAGS),
	gate!(PCLK_GRF, "pclk_grf", "pclk_cpu", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(10), 0, GFLAGS),
	gate!(0, "pclk_cru", "pclk_cpu", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(10), 1, GFLAGS),
	gate!(0, "pclk_sgrf", "pclk_cpu", CLK_IGNORE_UNUSED, RK2928_CLKGATE_CON(10), 2, GFLAGS),
	gate!(0, "pclk_sim", "pclk_cpu", 0, RK2928_CLKGATE_CON(10), 3, GFLAGS),

	gate!(0, "pclk_ddrphy", "pclk_phy_pre", 0, RK2928_CLKGATE_CON(10), 3, GFLAGS),
	gate!(0, "pclk_acodecphy", "pclk_phy_pre", 0, RK2928_CLKGATE_CON(10), 5, GFLAGS),
	gate!(PCLK_HDMI_PHY, "pclk_hdmiphy", "pclk_phy_pre", 0, RK2928_CLKGATE_CON(10), 7, GFLAGS),
	gate!(0, "pclk_vdacphy", "pclk_phy_pre", 0, RK2928_CLKGATE_CON(10), 8, GFLAGS),
	gate!(0, "pclk_phy_noc", "pclk_phy_pre", 0, RK2928_CLKGATE_CON(10), 9, GFLAGS),

	gate!(ACLK_VPU, "aclk_vpu", "aclk_vpu_pre", 0, RK2928_CLKGATE_CON(15), 0, GFLAGS),
	gate!(0, "aclk_vpu_noc", "aclk_vpu_pre", 0, RK2928_CLKGATE_CON(15), 4, GFLAGS),
	gate!(ACLK_RKVDEC, "aclk_rkvdec", "aclk_rkvdec_pre", 0, RK2928_CLKGATE_CON(15), 2, GFLAGS),
	gate!(0, "aclk_rkvdec_noc", "aclk_rkvdec_pre", 0, RK2928_CLKGATE_CON(15), 6, GFLAGS),
	gate!(HCLK_VPU, "hclk_vpu", "hclk_vpu_pre", 0, RK2928_CLKGATE_CON(15), 1, GFLAGS),
	gate!(0, "hclk_vpu_noc", "hclk_vpu_pre", 0, RK2928_CLKGATE_CON(15), 5, GFLAGS),
	gate!(HCLK_RKVDEC, "hclk_rkvdec", "hclk_rkvdec_pre", 0, RK2928_CLKGATE_CON(15), 3, GFLAGS),
	gate!(0, "hclk_rkvdec_noc", "hclk_rkvdec_pre", 0, RK2928_CLKGATE_CON(15), 7, GFLAGS),

	/* PD_MMC */
	mmc!(SCLK_SDMMC_DRV,    "sdmmc_drv",    "sclk_sdmmc", RK3228_SDMMC_CON0, 1),
	mmc!(SCLK_SDMMC_SAMPLE, "sdmmc_sample", "sclk_sdmmc", RK3228_SDMMC_CON1, 0),

	mmc!(SCLK_SDIO_DRV,     "sdio_drv",     "sclk_sdio",  RK3228_SDIO_CON0,  1),
	mmc!(SCLK_SDIO_SAMPLE,  "sdio_sample",  "sclk_sdio",  RK3228_SDIO_CON1,  0),

	mmc!(SCLK_EMMC_DRV,     "emmc_drv",     "sclk_emmc",  RK3228_EMMC_CON0,  1),
	mmc!(SCLK_EMMC_SAMPLE,  "emmc_sample",  "sclk_emmc",  RK3228_EMMC_CON1,  0),
};

static rk3228_critical_clocks: &[&str] = {
	"aclk_cpu",
	"pclk_cpu",
	"hclk_cpu",
	"aclk_peri",
	"hclk_peri",
	"pclk_peri",
	"aclk_rga_noc",
	"aclk_iep_noc",
	"aclk_vop_noc",
	"aclk_hdcp_noc",
	"hclk_vio_ahb_arbi",
	"hclk_vio_noc",
	"hclk_vop_noc",
	"hclk_host0_arb",
	"hclk_host1_arb",
	"hclk_host2_arb",
	"hclk_otg_pmu",
	"aclk_gpu_noc",
	"sclk_initmem_mbist",
	"aclk_initmem",
	"hclk_rom",
	"pclk_ddrupctl",
	"pclk_ddrmon",
	"pclk_msch_noc",
	"pclk_stimer",
	"pclk_ddrphy",
	"pclk_acodecphy",
	"pclk_phy_noc",
	"aclk_vpu_noc",
	"aclk_rkvdec_noc",
	"hclk_vpu_noc",
	"hclk_rkvdec_noc",
};

static void  rk3228_clk_init(struct device_node *np)
{
	struct rockchip_clk_provider *ctx;
	unsigned long clk_nr_clks;
	void __iomem *reg_base;

	reg_base = of_iomap(np, 0);
	if (!reg_base) {
		pr_err("%s: could not map cru region\n", __func__);
		return;
	}

	clk_nr_clks = rockchip_clk_find_max_clk_id(rk3228_clk_branches,
						   ARRAY_SIZE(rk3228_clk_branches)) + 1;
	ctx = rockchip_clk_init(np, reg_base, clk_nr_clks);
	if (IS_ERR(ctx)) {
		pr_err("%s: rockchip clk init failed\n", __func__);
		iounmap(reg_base);
		return;
	}

	rockchip_clk_register_plls(ctx, rk3228_pll_clks,
				   ARRAY_SIZE(rk3228_pll_clks),
				   RK3228_GRF_SOC_STATUS0);
	rockchip_clk_register_branches(ctx, rk3228_clk_branches,
				  ARRAY_SIZE(rk3228_clk_branches));
	rockchip_clk_protect_critical(rk3228_critical_clocks,
				      ARRAY_SIZE(rk3228_critical_clocks));

	rockchip_clk_register_armclk(ctx, ARMCLK, "armclk",
			mux_armclk_p, ARRAY_SIZE(mux_armclk_p),
			&rk3228_cpuclk_data, rk3228_cpuclk_rates,
			ARRAY_SIZE(rk3228_cpuclk_rates));

	rockchip_register_softrst(np, 9, reg_base + RK2928_SOFTRST_CON(0),
				  ROCKCHIP_SOFTRST_HIWORD_MASK);

	rockchip_register_restart_notifier(ctx, RK3228_GLB_SRST_FST, NULL);

	rockchip_clk_of_add_provider(np, ctx);
}
CLK_OF_DECLARE(rk3228_cru, "rockchip,rk3228-cru", rk3228_clk_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
