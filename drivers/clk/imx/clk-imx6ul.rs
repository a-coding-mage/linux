// Direct translation of clk-imx6ul.c. External kernel symbols and types are supplied by other files.
#![allow(non_upper_case_globals, non_snake_case, dead_code, unused_variables, unused_mut)]

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Freescale Semiconductor, Inc.
 */

// #include <dt-bindings/clock/imx6ul-clock.h>
// #include <linux/clk.h>
// #include <linux/clkdev.h>
// #include <linux/clk-provider.h>
// #include <linux/err.h>
// #include <linux/init.h>
// #include <linux/io.h>
// #include <linux/mfd/syscon/imx6q-iomuxc-gpr.h>
// #include <linux/of.h>
// #include <linux/of_address.h>
// #include <linux/of_irq.h>
// #include <linux/types.h>

// #include "clk.h"

static pll_bypass_src_sels: &[&str) = &[ "osc", "dummy", ];
static pll1_bypass_sels: &[&str) = &[ "pll1", "pll1_bypass_src", ];
static pll2_bypass_sels: &[&str) = &[ "pll2", "pll2_bypass_src", ];
static pll3_bypass_sels: &[&str) = &[ "pll3", "pll3_bypass_src", ];
static pll4_bypass_sels: &[&str) = &[ "pll4", "pll4_bypass_src", ];
static pll5_bypass_sels: &[&str) = &[ "pll5", "pll5_bypass_src", ];
static pll6_bypass_sels: &[&str) = &[ "pll6", "pll6_bypass_src", ];
static pll7_bypass_sels: &[&str) = &[ "pll7", "pll7_bypass_src", ];
static ca7_secondary_sels: &[&str) = &[ "pll2_pfd2_396m", "pll2_bus", ];
static step_sels: &[&str) = &[ "osc", "ca7_secondary_sel", ];
static pll1_sw_sels: &[&str) = &[ "pll1_sys", "step", ];
static axi_alt_sels: &[&str) = &[ "pll2_pfd2_396m", "pll3_pfd1_540m", ];
static axi_sels: &[&str) = &["periph", "axi_alt_sel", ];
static periph_pre_sels: &[&str) = &[ "pll2_bus", "pll2_pfd2_396m", "pll2_pfd0_352m", "pll2_198m", ];
static periph2_pre_sels: &[&str) = &[ "pll2_bus", "pll2_pfd2_396m", "pll2_pfd0_352m", "pll4_audio_div", ];
static periph_clk2_sels: &[&str) = &[ "pll3_usb_otg", "osc", "pll2_bypass_src", ];
static periph2_clk2_sels: &[&str) = &[ "pll3_usb_otg", "osc", ];
static periph_sels: &[&str) = &[ "periph_pre", "periph_clk2", ];
static periph2_sels: &[&str) = &[ "periph2_pre", "periph2_clk2", ];
static usdhc_sels: &[&str) = &[ "pll2_pfd2_396m", "pll2_pfd0_352m", ];
static bch_sels: &[&str) = &[ "pll2_pfd2_396m", "pll2_pfd0_352m", ];
static gpmi_sels: &[&str) = &[ "pll2_pfd2_396m", "pll2_pfd0_352m", ];
static const char *eim_slow_sels[) =  { "axi", "pll3_usb_otg", "pll2_pfd2_396m", "pll3_pfd0_720m", };
static spdif_sels: &[&str) = &[ "pll4_audio_div", "pll3_pfd2_508m", "pll5_video_div", "pll3_usb_otg", ];
static sai_sels: &[&str) = &[ "pll3_pfd2_508m", "pll5_video_div", "pll4_audio_div", ];
static lcdif_pre_sels: &[&str) = &[ "pll2_bus", "pll3_pfd3_454m", "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd1_594m", "pll3_pfd1_540m", ];
static sim_pre_sels: &[&str) = &[ "pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll3_pfd2_508m", ];
static ldb_di0_sels: &[&str) = &[ "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll2_pfd3_594m", "pll2_pfd1_594m", "pll3_pfd3_454m", ];
static ldb_di0_div_sels: &[&str) = &[ "ldb_di0_div_3_5", "ldb_di0_div_7", ];
static ldb_di1_div_sels: &[&str) = &[ "ldb_di1_div_3_5", "ldb_di1_div_7", ];
static qspi1_sels: &[&str) = &[ "pll3_usb_otg", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll2_bus", "pll3_pfd3_454m", "pll3_pfd2_508m", ];
static enfc_sels: &[&str) = &[ "pll2_pfd0_352m", "pll2_bus", "pll3_usb_otg", "pll2_pfd2_396m", "pll3_pfd3_454m", "dummy", "dummy", "dummy", ];
static can_sels: &[&str) = &[ "pll3_60m", "osc", "pll3_80m", "dummy", ];
static ecspi_sels: &[&str) = &[ "pll3_60m", "osc", ];
static uart_sels: &[&str) = &[ "pll3_80m", "osc", ];
static perclk_sels: &[&str) = &[ "ipg", "osc", ];
static lcdif_sels: &[&str) = &[ "lcdif_podf", "ipp_di0", "ipp_di1", "ldb_di0", "ldb_di1", ];
static csi_sels: &[&str) = &[ "osc", "pll2_pfd2_396m", "pll3_120m", "pll3_pfd1_540m", ];
static sim_sels: &[&str) = &[ "sim_podf", "ipp_di0", "ipp_di1", "ldb_di0", "ldb_di1", ];
/* epdc_pre_sels, epdc_sels, esai_sels only exists on i.MX6ULL */
static epdc_pre_sels: &[&str) = &[ "pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll3_pfd2_508m", ];
static esai_sels: &[&str) = &[ "pll4_audio_div", "pll3_pfd2_508m", "pll5_video_div", "pll3_usb_otg", ];
static epdc_sels: &[&str) = &[ "epdc_podf", "ipp_di0", "ipp_di1", "ldb_di0", "ldb_di1", ];
static cko1_sels: &[&str) = &[ "dummy", "dummy", "dummy", "dummy", "dummy", "axi", "enfc", "dummy", "dummy",
				   "dummy", "lcdif_pix", "ahb", "ipg", "ipg_per", "ckil", "pll4_audio_div", ];
static cko2_sels: &[&str) = &[ "dummy", "dummy", "dummy", "usdhc1", "dummy", "dummy", "ecspi_root", "dummy",
				   "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "osc", "dummy",
				   "dummy", "usdhc2", "sai1", "sai2", "sai3", "dummy", "dummy", "can_root",
				   "dummy", "dummy", "dummy", "dummy", "uart_serial", "spdif", "dummy", "dummy", ];
static cko_sels: &[&str) = &[ "cko1", "cko2", ];

static mut hws: *mut *mut clk_hw;
static mut clk_hw_data: *mut clk_hw_onecell_data;

static clk_enet_ref_table: &[clk_div_table) = &[
	{ .val = 0, .div = 20, },
	{ .val = 1, .div = 10, },
	{ .val = 2, .div = 5, },
	{ .val = 3, .div = 4, },
	{ }
};

static post_div_table: &[clk_div_table) = &[
	{ .val = 2, .div = 1, },
	{ .val = 1, .div = 2, },
	{ .val = 0, .div = 4, },
	{ }
};

static video_div_table: &[clk_div_table) = &[
	{ .val = 0, .div = 1, },
	{ .val = 1, .div = 2, },
	{ .val = 2, .div = 1, },
	{ .val = 3, .div = 4, },
	{ }
};

static const char * enet1_ref_sels[) = { "enet1_ref_125m", "enet1_ref_pad", "dummy", "dummy"};
static const u32 enet1_ref_sels_table[) = { IMX6UL_GPR1_ENET1_TX_CLK_DIR,
					    IMX6UL_GPR1_ENET1_CLK_SEL, 0,
					    IMX6UL_GPR1_ENET1_TX_CLK_DIR | IMX6UL_GPR1_ENET1_CLK_SEL };
static enet1_ref_sels_table_mask: u32 = IMX6UL_GPR1_ENET1_TX_CLK_DIR |
					     IMX6UL_GPR1_ENET1_CLK_SEL;
static const char * enet2_ref_sels[) = { "enet2_ref_125m", "enet2_ref_pad", "dummy", "dummy"};
static const u32 enet2_ref_sels_table[) = { IMX6UL_GPR1_ENET2_TX_CLK_DIR,
					    IMX6UL_GPR1_ENET2_CLK_SEL, 0,
					    IMX6UL_GPR1_ENET2_TX_CLK_DIR | IMX6UL_GPR1_ENET2_CLK_SEL };
static enet2_ref_sels_table_mask: u32 = IMX6UL_GPR1_ENET2_TX_CLK_DIR |
					     IMX6UL_GPR1_ENET2_CLK_SEL;

static mut share_count_asrc: u32;
static mut share_count_audio: u32;
static mut share_count_sai1: u32;
static mut share_count_sai2: u32;
static mut share_count_sai3: u32;
static mut share_count_esai: u32;

unsafe fn clk_on_imx6ul() . i32
{
	return of_machine_is_compatible("fsl,imx6ul");
}

unsafe fn clk_on_imx6ull() . i32
{
	return of_machine_is_compatible("fsl,imx6ull");
}

unsafe fn imx6ul_clocks_init(ccm_node: *mut device_node)
{
	let mut np: *mut device_node;
	let mut base: *mut core::ffi::c_void;

	clk_hw_data = kzalloc_flex(*clk_hw_data, hws, IMX6UL_CLK_END);
	if (WARN_ON(!clk_hw_data) != 0) {
		return;

	clk_hw_data.num = IMX6UL_CLK_END;
	hws = clk_hw_data.hws;

	*hws.add(IMX6UL_CLK_DUMMY) = imx_clk_hw_fixed("dummy", 0);

	*hws.add(IMX6UL_CLK_CKIL) = imx_get_clk_hw_by_name(ccm_node, "ckil");
	*hws.add(IMX6UL_CLK_OSC) = imx_get_clk_hw_by_name(ccm_node, "osc");

	/* ipp_di clock is external input */
	*hws.add(IMX6UL_CLK_IPP_DI0) = imx_get_clk_hw_by_name(ccm_node, "ipp_di0");
	*hws.add(IMX6UL_CLK_IPP_DI1) = imx_get_clk_hw_by_name(ccm_node, "ipp_di1");

	np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx6ul-anatop");
	base = of_iomap(np, 0);
	of_node_put(np);
	WARN_ON(!base);

	*hws.add(IMX6UL_PLL1_BYPASS_SRC) = imx_clk_hw_mux("pll1_bypass_src", base.add(0x00 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL2_BYPASS_SRC) = imx_clk_hw_mux("pll2_bypass_src", base.add(0x30 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL3_BYPASS_SRC) = imx_clk_hw_mux("pll3_bypass_src", base.add(0x10 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL4_BYPASS_SRC) = imx_clk_hw_mux("pll4_bypass_src", base.add(0x70 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL5_BYPASS_SRC) = imx_clk_hw_mux("pll5_bypass_src", base.add(0xa0 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL6_BYPASS_SRC) = imx_clk_hw_mux("pll6_bypass_src", base.add(0xe0 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());
	*hws.add(IMX6UL_PLL7_BYPASS_SRC) = imx_clk_hw_mux("pll7_bypass_src", base.add(0x20 as usize), 14, 1, pll_bypass_src_sels, pll_bypass_src_sels.len());

	*hws.add(IMX6UL_CLK_PLL1) = imx_clk_hw_pllv3(IMX_PLLV3_SYS,	 "pll1", "osc", base.add(0x00 as usize), 0x7f);
	*hws.add(IMX6UL_CLK_PLL2) = imx_clk_hw_pllv3(IMX_PLLV3_GENERIC, "pll2", "osc", base.add(0x30 as usize), 0x1);
	*hws.add(IMX6UL_CLK_PLL3) = imx_clk_hw_pllv3(IMX_PLLV3_USB,	 "pll3", "osc", base.add(0x10 as usize), 0x3);
	*hws.add(IMX6UL_CLK_PLL4) = imx_clk_hw_pllv3(IMX_PLLV3_AV,	 "pll4", "osc", base.add(0x70 as usize), 0x7f);
	*hws.add(IMX6UL_CLK_PLL5) = imx_clk_hw_pllv3(IMX_PLLV3_AV,	 "pll5", "osc", base.add(0xa0 as usize), 0x7f);
	*hws.add(IMX6UL_CLK_PLL6) = imx_clk_hw_pllv3(IMX_PLLV3_ENET,	 "pll6", "osc", base.add(0xe0 as usize), 0x3);
	*hws.add(IMX6UL_CLK_PLL7) = imx_clk_hw_pllv3(IMX_PLLV3_USB,	 "pll7", "osc", base.add(0x20 as usize), 0x3);

	*hws.add(IMX6UL_PLL1_BYPASS) = imx_clk_hw_mux_flags("pll1_bypass", base.add(0x00 as usize), 16, 1, pll1_bypass_sels, pll1_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL2_BYPASS) = imx_clk_hw_mux_flags("pll2_bypass", base.add(0x30 as usize), 16, 1, pll2_bypass_sels, pll2_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL3_BYPASS) = imx_clk_hw_mux_flags("pll3_bypass", base.add(0x10 as usize), 16, 1, pll3_bypass_sels, pll3_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL4_BYPASS) = imx_clk_hw_mux_flags("pll4_bypass", base.add(0x70 as usize), 16, 1, pll4_bypass_sels, pll4_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL5_BYPASS) = imx_clk_hw_mux_flags("pll5_bypass", base.add(0xa0 as usize), 16, 1, pll5_bypass_sels, pll5_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL6_BYPASS) = imx_clk_hw_mux_flags("pll6_bypass", base.add(0xe0 as usize), 16, 1, pll6_bypass_sels, pll6_bypass_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_PLL7_BYPASS) = imx_clk_hw_mux_flags("pll7_bypass", base.add(0x20 as usize), 16, 1, pll7_bypass_sels, pll7_bypass_sels.len(), CLK_SET_RATE_PARENT);

	/* Do not bypass PLLs initially */
	clk_set_parent(*hws.add(IMX6UL_PLL1_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL1 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL2_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL2 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL3_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL3 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL4_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL4 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL5_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL5 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL6_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL6 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_PLL7_BYPASS as usize).clk, *hws.add(IMX6UL_CLK_PLL7 as usize).clk);

	*hws.add(IMX6UL_CLK_PLL1_SYS) = imx_clk_hw_fixed_factor("pll1_sys",	"pll1_bypass", 1, 1);
	*hws.add(IMX6UL_CLK_PLL2_BUS) = imx_clk_hw_gate("pll2_bus",	"pll2_bypass", base.add(0x30 as usize), 13);
	*hws.add(IMX6UL_CLK_PLL3_USB_OTG) = imx_clk_hw_gate("pll3_usb_otg",	"pll3_bypass", base.add(0x10 as usize), 13);
	*hws.add(IMX6UL_CLK_PLL4_AUDIO) = imx_clk_hw_gate("pll4_audio",	"pll4_bypass", base.add(0x70 as usize), 13);
	*hws.add(IMX6UL_CLK_PLL5_VIDEO) = imx_clk_hw_gate("pll5_video",	"pll5_bypass", base.add(0xa0 as usize), 13);
	*hws.add(IMX6UL_CLK_PLL6_ENET) = imx_clk_hw_fixed_factor("pll6_enet",	"pll6_bypass", 1, 1);
	*hws.add(IMX6UL_CLK_PLL7_USB_HOST) = imx_clk_hw_gate("pll7_usb_host",	"pll7_bypass", base.add(0x20 as usize), 13);

	/*
	 * Bit 20 is the reserved and read-only bit, we do this only for:
	 * - Do nothing for usbphy clk_enable/disable
	 * - Keep refcount when do usbphy clk_enable/disable, in that case,
	 * the clk framework many need to enable/disable usbphy's parent
	 */
	*hws.add(IMX6UL_CLK_USBPHY1) = imx_clk_hw_gate("usbphy1", "pll3_usb_otg",  base.add(0x10 as usize), 20);
	*hws.add(IMX6UL_CLK_USBPHY2) = imx_clk_hw_gate("usbphy2", "pll7_usb_host", base.add(0x20 as usize), 20);

	/*
	 * usbphy*_gate needs to be on after system boots up, and software
	 * never needs to control it anymore.
	 */
	*hws.add(IMX6UL_CLK_USBPHY1_GATE) = imx_clk_hw_gate("usbphy1_gate", "dummy", base.add(0x10 as usize), 6);
	*hws.add(IMX6UL_CLK_USBPHY2_GATE) = imx_clk_hw_gate("usbphy2_gate", "dummy", base.add(0x20 as usize), 6);

	/*					name		   parent_name	   reg		idx */
	*hws.add(IMX6UL_CLK_PLL2_PFD0) = imx_clk_hw_pfd("pll2_pfd0_352m", "pll2_bus",	   base.add(0x100 as usize), 0);
	*hws.add(IMX6UL_CLK_PLL2_PFD1) = imx_clk_hw_pfd("pll2_pfd1_594m", "pll2_bus",	   base.add(0x100 as usize), 1);
	*hws.add(IMX6UL_CLK_PLL2_PFD2) = imx_clk_hw_pfd("pll2_pfd2_396m", "pll2_bus",	   base.add(0x100 as usize), 2);
	*hws.add(IMX6UL_CLK_PLL2_PFD3) = imx_clk_hw_pfd("pll2_pfd3_594m", "pll2_bus",	   base.add(0x100 as usize), 3);
	*hws.add(IMX6UL_CLK_PLL3_PFD0) = imx_clk_hw_pfd("pll3_pfd0_720m", "pll3_usb_otg", base.add(0xf0 as usize),  0);
	*hws.add(IMX6UL_CLK_PLL3_PFD1) = imx_clk_hw_pfd("pll3_pfd1_540m", "pll3_usb_otg", base.add(0xf0 as usize),  1);
	*hws.add(IMX6UL_CLK_PLL3_PFD2) = imx_clk_hw_pfd("pll3_pfd2_508m", "pll3_usb_otg", base.add(0xf0 as usize),	 2);
	*hws.add(IMX6UL_CLK_PLL3_PFD3) = imx_clk_hw_pfd("pll3_pfd3_454m", "pll3_usb_otg", base.add(0xf0 as usize),	 3);

	*hws.add(IMX6UL_CLK_ENET_REF) = clk_hw_register_divider_table(core::ptr::null_mut(), "enet1_ref", "pll6_enet", 0,
			base.add(0xe0 as usize), 0, 2, 0, clk_enet_ref_table, &imx_ccm_lock);
	*hws.add(IMX6UL_CLK_ENET2_REF) = clk_hw_register_divider_table(core::ptr::null_mut(), "enet2_ref", "pll6_enet", 0,
			base.add(0xe0 as usize), 2, 2, 0, clk_enet_ref_table, &imx_ccm_lock);

	*hws.add(IMX6UL_CLK_ENET1_REF_125M) = imx_clk_hw_gate("enet1_ref_125m", "enet1_ref", base.add(0xe0 as usize), 13);
	*hws.add(IMX6UL_CLK_ENET2_REF_125M) = imx_clk_hw_gate("enet2_ref_125m", "enet2_ref", base.add(0xe0 as usize), 20);
	*hws.add(IMX6UL_CLK_ENET_PTP_REF) = imx_clk_hw_fixed_factor("enet_ptp_ref", "pll6_enet", 1, 20);
	*hws.add(IMX6UL_CLK_ENET_PTP) = imx_clk_hw_gate("enet_ptp", "enet_ptp_ref", base.add(0xe0 as usize), 21);

	*hws.add(IMX6UL_CLK_PLL4_POST_DIV) = clk_hw_register_divider_table(core::ptr::null_mut(), "pll4_post_div", "pll4_audio",
		 CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE, base.add(0x70 as usize), 19, 2, 0, post_div_table, &imx_ccm_lock);
	*hws.add(IMX6UL_CLK_PLL4_AUDIO_DIV) = clk_hw_register_divider(core::ptr::null_mut(), "pll4_audio_div", "pll4_post_div",
		 CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE, base.add(0x170 as usize), 15, 1, 0, &imx_ccm_lock);
	*hws.add(IMX6UL_CLK_PLL5_POST_DIV) = clk_hw_register_divider_table(core::ptr::null_mut(), "pll5_post_div", "pll5_video",
		 CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE, base.add(0xa0 as usize), 19, 2, 0, post_div_table, &imx_ccm_lock);
	*hws.add(IMX6UL_CLK_PLL5_VIDEO_DIV) = clk_hw_register_divider_table(core::ptr::null_mut(), "pll5_video_div", "pll5_post_div",
		 CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE, base.add(0x170 as usize), 30, 2, 0, video_div_table, &imx_ccm_lock);

	/*						   name		parent_name	 mult  div */
	*hws.add(IMX6UL_CLK_PLL2_198M) = imx_clk_hw_fixed_factor("pll2_198m", "pll2_pfd2_396m", 1,	2);
	*hws.add(IMX6UL_CLK_PLL3_80M) = imx_clk_hw_fixed_factor("pll3_80m",  "pll3_usb_otg",   1,	6);
	*hws.add(IMX6UL_CLK_PLL3_60M) = imx_clk_hw_fixed_factor("pll3_60m",  "pll3_usb_otg",   1,	8);
	*hws.add(IMX6UL_CLK_GPT_3M) = imx_clk_hw_fixed_factor("gpt_3m",	"osc",		 1,	8);

	np = ccm_node;
	base = of_iomap(np, 0);
	WARN_ON(!base);

	*hws.add(IMX6UL_CA7_SECONDARY_SEL) = imx_clk_hw_mux("ca7_secondary_sel", base.add(0xc as usize), 3, 1, ca7_secondary_sels, ca7_secondary_sels.len());
	*hws.add(IMX6UL_CLK_STEP) = imx_clk_hw_mux("step", base.add(0x0c as usize), 8, 1, step_sels, step_sels.len());
	*hws.add(IMX6UL_CLK_PLL1_SW) = imx_clk_hw_mux_flags("pll1_sw",   base.add(0x0c as usize), 2,  1, pll1_sw_sels, pll1_sw_sels.len(), 0);
	*hws.add(IMX6UL_CLK_AXI_ALT_SEL) = imx_clk_hw_mux("axi_alt_sel",		base.add(0x14 as usize), 7,  1, axi_alt_sels, axi_alt_sels.len());
	*hws.add(IMX6UL_CLK_AXI_SEL) = imx_clk_hw_mux_flags("axi_sel",	base.add(0x14 as usize), 6,  1, axi_sels, axi_sels.len(), 0);
	*hws.add(IMX6UL_CLK_PERIPH_PRE) = imx_clk_hw_mux("periph_pre",       base.add(0x18 as usize), 18, 2, periph_pre_sels, periph_pre_sels.len());
	*hws.add(IMX6UL_CLK_PERIPH2_PRE) = imx_clk_hw_mux("periph2_pre",      base.add(0x18 as usize), 21, 2, periph2_pre_sels, periph2_pre_sels.len());
	*hws.add(IMX6UL_CLK_PERIPH_CLK2_SEL) = imx_clk_hw_mux("periph_clk2_sel",  base.add(0x18 as usize), 12, 2, periph_clk2_sels, periph_clk2_sels.len());
	*hws.add(IMX6UL_CLK_PERIPH2_CLK2_SEL) = imx_clk_hw_mux("periph2_clk2_sel", base.add(0x18 as usize), 20, 1, periph2_clk2_sels, periph2_clk2_sels.len());
	*hws.add(IMX6UL_CLK_EIM_SLOW_SEL) = imx_clk_hw_mux("eim_slow_sel", base.add(0x1c as usize), 29, 2, eim_slow_sels, eim_slow_sels.len());
	*hws.add(IMX6UL_CLK_GPMI_SEL) = imx_clk_hw_mux("gpmi_sel",     base.add(0x1c as usize), 19, 1, gpmi_sels, gpmi_sels.len());
	*hws.add(IMX6UL_CLK_BCH_SEL) = imx_clk_hw_mux("bch_sel",	base.add(0x1c as usize), 18, 1, bch_sels, bch_sels.len());
	*hws.add(IMX6UL_CLK_USDHC2_SEL) = imx_clk_hw_mux("usdhc2_sel",   base.add(0x1c as usize), 17, 1, usdhc_sels, usdhc_sels.len());
	*hws.add(IMX6UL_CLK_USDHC1_SEL) = imx_clk_hw_mux("usdhc1_sel",   base.add(0x1c as usize), 16, 1, usdhc_sels, usdhc_sels.len());
	*hws.add(IMX6UL_CLK_SAI3_SEL) = imx_clk_hw_mux("sai3_sel",     base.add(0x1c as usize), 14, 2, sai_sels, sai_sels.len());
	*hws.add(IMX6UL_CLK_SAI2_SEL) = imx_clk_hw_mux("sai2_sel",     base.add(0x1c as usize), 12, 2, sai_sels, sai_sels.len());
	*hws.add(IMX6UL_CLK_SAI1_SEL) = imx_clk_hw_mux("sai1_sel",     base.add(0x1c as usize), 10, 2, sai_sels, sai_sels.len());
	*hws.add(IMX6UL_CLK_QSPI1_SEL) = imx_clk_hw_mux("qspi1_sel",    base.add(0x1c as usize), 7,  3, qspi1_sels, qspi1_sels.len());
	*hws.add(IMX6UL_CLK_PERCLK_SEL) = imx_clk_hw_mux("perclk_sel",	base.add(0x1c as usize), 6,  1, perclk_sels, perclk_sels.len());
	*hws.add(IMX6UL_CLK_CAN_SEL) = imx_clk_hw_mux("can_sel",	base.add(0x20 as usize), 8,  2, can_sels, can_sels.len());
	if (clk_on_imx6ull() != 0) {
		*hws.add(IMX6ULL_CLK_ESAI_SEL) = imx_clk_hw_mux("esai_sel",	base.add(0x20 as usize), 19, 2, esai_sels, esai_sels.len());
	*hws.add(IMX6UL_CLK_UART_SEL) = imx_clk_hw_mux("uart_sel",	base.add(0x24 as usize), 6,  1, uart_sels, uart_sels.len());
	*hws.add(IMX6UL_CLK_ENFC_SEL) = imx_clk_hw_mux("enfc_sel",	base.add(0x2c as usize), 15, 3, enfc_sels, enfc_sels.len());
	*hws.add(IMX6UL_CLK_LDB_DI0_SEL) = imx_clk_hw_mux("ldb_di0_sel",	base.add(0x2c as usize), 9,  3, ldb_di0_sels, ldb_di0_sels.len());
	*hws.add(IMX6UL_CLK_SPDIF_SEL) = imx_clk_hw_mux("spdif_sel",	base.add(0x30 as usize), 20, 2, spdif_sels, spdif_sels.len());
	if (clk_on_imx6ul() != 0) {
		*hws.add(IMX6UL_CLK_SIM_PRE_SEL) = imx_clk_hw_mux("sim_pre_sel",	base.add(0x34 as usize), 15, 3, sim_pre_sels, sim_pre_sels.len());
		*hws.add(IMX6UL_CLK_SIM_SEL) = imx_clk_hw_mux("sim_sel",	base.add(0x34 as usize), 9, 3, sim_sels, sim_sels.len());
	} else if (clk_on_imx6ull() != 0 != 0) {
		*hws.add(IMX6ULL_CLK_EPDC_PRE_SEL) = imx_clk_hw_mux("epdc_pre_sel",	base.add(0x34 as usize), 15, 3, epdc_pre_sels, epdc_pre_sels.len());
		*hws.add(IMX6ULL_CLK_EPDC_SEL) = imx_clk_hw_mux("epdc_sel",	base.add(0x34 as usize), 9, 3, epdc_sels, epdc_sels.len());
	}
	*hws.add(IMX6UL_CLK_ECSPI_SEL) = imx_clk_hw_mux("ecspi_sel",	base.add(0x38 as usize), 18, 1, ecspi_sels, ecspi_sels.len());
	*hws.add(IMX6UL_CLK_LCDIF_PRE_SEL) = imx_clk_hw_mux_flags("lcdif_pre_sel", base.add(0x38 as usize), 15, 3, lcdif_pre_sels, lcdif_pre_sels.len(), CLK_SET_RATE_PARENT);
	*hws.add(IMX6UL_CLK_LCDIF_SEL) = imx_clk_hw_mux("lcdif_sel",	base.add(0x38 as usize), 9, 3, lcdif_sels, lcdif_sels.len());
	*hws.add(IMX6UL_CLK_CSI_SEL) = imx_clk_hw_mux("csi_sel", base.add(0x3c as usize), 9, 2, csi_sels, csi_sels.len());

	*hws.add(IMX6UL_CLK_LDB_DI0_DIV_SEL) = imx_clk_hw_mux("ldb_di0", base.add(0x20 as usize), 10, 1, ldb_di0_div_sels, ldb_di0_div_sels.len());
	*hws.add(IMX6UL_CLK_LDB_DI1_DIV_SEL) = imx_clk_hw_mux("ldb_di1", base.add(0x20 as usize), 11, 1, ldb_di1_div_sels, ldb_di1_div_sels.len());

	*hws.add(IMX6UL_CLK_CKO1_SEL) = imx_clk_hw_mux("cko1_sel", base.add(0x60 as usize), 0,  4, cko1_sels, cko1_sels.len());
	*hws.add(IMX6UL_CLK_CKO2_SEL) = imx_clk_hw_mux("cko2_sel", base.add(0x60 as usize), 16, 5, cko2_sels, cko2_sels.len());
	*hws.add(IMX6UL_CLK_CKO) = imx_clk_hw_mux("cko", base.add(0x60 as usize), 8, 1, cko_sels, cko_sels.len());

	*hws.add(IMX6UL_CLK_LDB_DI0_DIV_3_5) = imx_clk_hw_fixed_factor("ldb_di0_div_3_5", "ldb_di0_sel", 2, 7);
	*hws.add(IMX6UL_CLK_LDB_DI0_DIV_7) = imx_clk_hw_fixed_factor("ldb_di0_div_7",   "ldb_di0_sel", 1, 7);
	*hws.add(IMX6UL_CLK_LDB_DI1_DIV_3_5) = imx_clk_hw_fixed_factor("ldb_di1_div_3_5", "qspi1_sel", 2, 7);
	*hws.add(IMX6UL_CLK_LDB_DI1_DIV_7) = imx_clk_hw_fixed_factor("ldb_di1_div_7",   "qspi1_sel", 1, 7);

	*hws.add(IMX6UL_CLK_PERIPH) = imx_clk_hw_busy_mux("periph",  base.add(0x14 as usize), 25, 1, base.add(0x48 as usize), 5, periph_sels, periph_sels.len());
	*hws.add(IMX6UL_CLK_PERIPH2) = imx_clk_hw_busy_mux("periph2", base.add(0x14 as usize), 26, 1, base.add(0x48 as usize), 3, periph2_sels, periph2_sels.len());

	*hws.add(IMX6UL_CLK_PERIPH_CLK2) = imx_clk_hw_divider("periph_clk2",   "periph_clk2_sel",	base.add(0x14 as usize), 27, 3);
	*hws.add(IMX6UL_CLK_PERIPH2_CLK2) = imx_clk_hw_divider("periph2_clk2",  "periph2_clk2_sel",	base.add(0x14 as usize), 0,  3);
	*hws.add(IMX6UL_CLK_IPG) = imx_clk_hw_divider("ipg",	   "ahb",		base.add(0x14 as usize), 8,	 2);
	*hws.add(IMX6UL_CLK_LCDIF_PODF) = imx_clk_hw_divider("lcdif_podf",	   "lcdif_pred",	base.add(0x18 as usize), 23, 3);
	*hws.add(IMX6UL_CLK_QSPI1_PDOF) = imx_clk_hw_divider("qspi1_podf",	   "qspi1_sel",		base.add(0x1c as usize), 26, 3);
	*hws.add(IMX6UL_CLK_EIM_SLOW_PODF) = imx_clk_hw_divider("eim_slow_podf", "eim_slow_sel",	base.add(0x1c as usize), 23, 3);
	*hws.add(IMX6UL_CLK_PERCLK) = imx_clk_hw_divider("perclk",	   "perclk_sel",	base.add(0x1c as usize), 0,  6);
	*hws.add(IMX6UL_CLK_CAN_PODF) = imx_clk_hw_divider("can_podf",	   "can_sel",		base.add(0x20 as usize), 2,  6);
	*hws.add(IMX6UL_CLK_GPMI_PODF) = imx_clk_hw_divider("gpmi_podf",	   "gpmi_sel",		base.add(0x24 as usize), 22, 3);
	*hws.add(IMX6UL_CLK_BCH_PODF) = imx_clk_hw_divider("bch_podf",	   "bch_sel",		base.add(0x24 as usize), 19, 3);
	*hws.add(IMX6UL_CLK_USDHC2_PODF) = imx_clk_hw_divider("usdhc2_podf",   "usdhc2_sel",	base.add(0x24 as usize), 16, 3);
	*hws.add(IMX6UL_CLK_USDHC1_PODF) = imx_clk_hw_divider("usdhc1_podf",   "usdhc1_sel",	base.add(0x24 as usize), 11, 3);
	*hws.add(IMX6UL_CLK_UART_PODF) = imx_clk_hw_divider("uart_podf",	   "uart_sel",		base.add(0x24 as usize), 0,  6);
	*hws.add(IMX6UL_CLK_SAI3_PRED) = imx_clk_hw_divider("sai3_pred",	   "sai3_sel",		base.add(0x28 as usize), 22, 3);
	*hws.add(IMX6UL_CLK_SAI3_PODF) = imx_clk_hw_divider("sai3_podf",	   "sai3_pred",		base.add(0x28 as usize), 16, 6);
	*hws.add(IMX6UL_CLK_SAI1_PRED) = imx_clk_hw_divider("sai1_pred",	   "sai1_sel",		base.add(0x28 as usize), 6,	 3);
	*hws.add(IMX6UL_CLK_SAI1_PODF) = imx_clk_hw_divider("sai1_podf",	   "sai1_pred",		base.add(0x28 as usize), 0,	 6);
	if (clk_on_imx6ull() != 0) {
		*hws.add(IMX6ULL_CLK_ESAI_PRED) = imx_clk_hw_divider("esai_pred",     "esai_sel",		base.add(0x28 as usize), 9,  3);
		*hws.add(IMX6ULL_CLK_ESAI_PODF) = imx_clk_hw_divider("esai_podf",     "esai_pred",		base.add(0x28 as usize), 25, 3);
	}
	*hws.add(IMX6UL_CLK_ENFC_PRED) = imx_clk_hw_divider("enfc_pred",	   "enfc_sel",		base.add(0x2c as usize), 18, 3);
	*hws.add(IMX6UL_CLK_ENFC_PODF) = imx_clk_hw_divider("enfc_podf",	   "enfc_pred",		base.add(0x2c as usize), 21, 6);
	*hws.add(IMX6UL_CLK_SAI2_PRED) = imx_clk_hw_divider("sai2_pred",	   "sai2_sel",		base.add(0x2c as usize), 6,	 3);
	*hws.add(IMX6UL_CLK_SAI2_PODF) = imx_clk_hw_divider("sai2_podf",	   "sai2_pred",		base.add(0x2c as usize), 0,  6);
	*hws.add(IMX6UL_CLK_SPDIF_PRED) = imx_clk_hw_divider("spdif_pred",	   "spdif_sel",		base.add(0x30 as usize), 25, 3);
	*hws.add(IMX6UL_CLK_SPDIF_PODF) = imx_clk_hw_divider("spdif_podf",	   "spdif_pred",	base.add(0x30 as usize), 22, 3);
	if (clk_on_imx6ul() != 0) {
		*hws.add(IMX6UL_CLK_SIM_PODF) = imx_clk_hw_divider("sim_podf",	   "sim_pre_sel",	base.add(0x34 as usize), 12, 3);
	else if (clk_on_imx6ull() != 0) {
		*hws.add(IMX6ULL_CLK_EPDC_PODF) = imx_clk_hw_divider("epdc_podf",	   "epdc_pre_sel",	base.add(0x34 as usize), 12, 3);
	*hws.add(IMX6UL_CLK_ECSPI_PODF) = imx_clk_hw_divider("ecspi_podf",	   "ecspi_sel",		base.add(0x38 as usize), 19, 6);
	*hws.add(IMX6UL_CLK_LCDIF_PRED) = imx_clk_hw_divider("lcdif_pred",	   "lcdif_pre_sel",	base.add(0x38 as usize), 12, 3);
	*hws.add(IMX6UL_CLK_CSI_PODF) = imx_clk_hw_divider("csi_podf",      "csi_sel",           base.add(0x3c as usize), 11, 3);

	*hws.add(IMX6UL_CLK_CKO1_PODF) = imx_clk_hw_divider("cko1_podf",     "cko1_sel",          base.add(0x60 as usize), 4,  3);
	*hws.add(IMX6UL_CLK_CKO2_PODF) = imx_clk_hw_divider("cko2_podf",     "cko2_sel",          base.add(0x60 as usize), 21, 3);

	*hws.add(IMX6UL_CLK_ARM) = imx_clk_hw_busy_divider("arm",	    "pll1_sw",	base +	0x10, 0,  3,  base.add(0x48 as usize), 16);
	*hws.add(IMX6UL_CLK_MMDC_PODF) = imx_clk_hw_busy_divider("mmdc_podf", "periph2",	base +  0x14, 3,  3,  base.add(0x48 as usize), 2);
	*hws.add(IMX6UL_CLK_AXI_PODF) = imx_clk_hw_busy_divider("axi_podf",  "axi_sel",	base +  0x14, 16, 3,  base.add(0x48 as usize), 0);
	*hws.add(IMX6UL_CLK_AHB) = imx_clk_hw_busy_divider("ahb",	    "periph",	base +  0x14, 10, 3,  base.add(0x48 as usize), 1);

	/* CCGR0 */
	*hws.add(IMX6UL_CLK_AIPSTZ1) = imx_clk_hw_gate2_flags("aips_tz1", "ahb", base.add(0x68 as usize), 0, CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_AIPSTZ2) = imx_clk_hw_gate2_flags("aips_tz2", "ahb", base.add(0x68 as usize), 2, CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_APBHDMA) = imx_clk_hw_gate2("apbh_dma",	"bch_podf",	base.add(0x68 as usize),	4);
	*hws.add(IMX6UL_CLK_ASRC_IPG) = imx_clk_hw_gate2_shared("asrc_ipg",	"ahb",	base.add(0x68 as usize),	6, &share_count_asrc);
	*hws.add(IMX6UL_CLK_ASRC_MEM) = imx_clk_hw_gate2_shared("asrc_mem",	"ahb",	base.add(0x68 as usize),	6, &share_count_asrc);
	if (clk_on_imx6ul() != 0) {
		*hws.add(IMX6UL_CLK_CAAM_MEM) = imx_clk_hw_gate2("caam_mem",	"ahb",		base.add(0x68 as usize),	8);
		*hws.add(IMX6UL_CLK_CAAM_ACLK) = imx_clk_hw_gate2("caam_aclk",	"ahb",		base.add(0x68 as usize),	10);
		*hws.add(IMX6UL_CLK_CAAM_IPG) = imx_clk_hw_gate2("caam_ipg",	"ipg",		base.add(0x68 as usize),	12);
	} else if (clk_on_imx6ull() != 0 != 0) {
		*hws.add(IMX6ULL_CLK_DCP_CLK) = imx_clk_hw_gate2("dcp",		"ahb",		base.add(0x68 as usize),	10);
		*hws.add(IMX6UL_CLK_ENET) = imx_clk_hw_gate2("enet",		"ipg",		base.add(0x68 as usize),	12);
		*hws.add(IMX6UL_CLK_ENET_AHB) = imx_clk_hw_gate2("enet_ahb",	"ahb",		base.add(0x68 as usize),	12);
	}
	*hws.add(IMX6UL_CLK_CAN1_IPG) = imx_clk_hw_gate2("can1_ipg",	"ipg",		base.add(0x68 as usize),	14);
	*hws.add(IMX6UL_CLK_CAN1_SERIAL) = imx_clk_hw_gate2("can1_serial",	"can_podf",	base.add(0x68 as usize),	16);
	*hws.add(IMX6UL_CLK_CAN2_IPG) = imx_clk_hw_gate2("can2_ipg",	"ipg",		base.add(0x68 as usize),	18);
	*hws.add(IMX6UL_CLK_CAN2_SERIAL) = imx_clk_hw_gate2("can2_serial",	"can_podf",	base.add(0x68 as usize),	20);
	*hws.add(IMX6UL_CLK_GPT2_BUS) = imx_clk_hw_gate2("gpt2_bus",	"perclk",	base.add(0x68 as usize),	24);
	*hws.add(IMX6UL_CLK_GPT2_SERIAL) = imx_clk_hw_gate2("gpt2_serial",	"perclk",	base.add(0x68 as usize),	26);
	*hws.add(IMX6UL_CLK_UART2_IPG) = imx_clk_hw_gate2("uart2_ipg",	"ipg",		base.add(0x68 as usize),	28);
	*hws.add(IMX6UL_CLK_UART2_SERIAL) = imx_clk_hw_gate2("uart2_serial",	"uart_podf",	base.add(0x68 as usize),	28);
	if (clk_on_imx6ull() != 0) {
		*hws.add(IMX6UL_CLK_AIPSTZ3) = imx_clk_hw_gate2("aips_tz3",	"ahb",		 base.add(0x80 as usize),	18);
	*hws.add(IMX6UL_CLK_GPIO2) = imx_clk_hw_gate2("gpio2",	"ipg",		base.add(0x68 as usize),	30);

	/* CCGR1 */
	*hws.add(IMX6UL_CLK_ECSPI1) = imx_clk_hw_gate2("ecspi1",	"ecspi_podf",	base.add(0x6c as usize),	0);
	*hws.add(IMX6UL_CLK_ECSPI2) = imx_clk_hw_gate2("ecspi2",	"ecspi_podf",	base.add(0x6c as usize),	2);
	*hws.add(IMX6UL_CLK_ECSPI3) = imx_clk_hw_gate2("ecspi3",	"ecspi_podf",	base.add(0x6c as usize),	4);
	*hws.add(IMX6UL_CLK_ECSPI4) = imx_clk_hw_gate2("ecspi4",	"ecspi_podf",	base.add(0x6c as usize),	6);
	*hws.add(IMX6UL_CLK_ADC2) = imx_clk_hw_gate2("adc2",		"ipg",		base.add(0x6c as usize),	8);
	*hws.add(IMX6UL_CLK_UART3_IPG) = imx_clk_hw_gate2("uart3_ipg",	"ipg",		base.add(0x6c as usize),	10);
	*hws.add(IMX6UL_CLK_UART3_SERIAL) = imx_clk_hw_gate2("uart3_serial",	"uart_podf",	base.add(0x6c as usize),	10);
	*hws.add(IMX6UL_CLK_EPIT1) = imx_clk_hw_gate2("epit1",	"perclk",	base.add(0x6c as usize),	12);
	*hws.add(IMX6UL_CLK_EPIT2) = imx_clk_hw_gate2("epit2",	"perclk",	base.add(0x6c as usize),	14);
	*hws.add(IMX6UL_CLK_ADC1) = imx_clk_hw_gate2("adc1",		"ipg",		base.add(0x6c as usize),	16);
	*hws.add(IMX6UL_CLK_GPT1_BUS) = imx_clk_hw_gate2("gpt1_bus",	"perclk",	base.add(0x6c as usize),	20);
	*hws.add(IMX6UL_CLK_GPT1_SERIAL) = imx_clk_hw_gate2("gpt1_serial",	"perclk",	base.add(0x6c as usize),	22);
	*hws.add(IMX6UL_CLK_UART4_IPG) = imx_clk_hw_gate2("uart4_ipg",	"ipg",		base.add(0x6c as usize),	24);
	*hws.add(IMX6UL_CLK_UART4_SERIAL) = imx_clk_hw_gate2("uart4_serial",	"uart_podf",	base.add(0x6c as usize),	24);
	*hws.add(IMX6UL_CLK_GPIO1) = imx_clk_hw_gate2("gpio1",	"ipg",		base.add(0x6c as usize),	26);
	*hws.add(IMX6UL_CLK_GPIO5) = imx_clk_hw_gate2("gpio5",	"ipg",		base.add(0x6c as usize),	30);

	/* CCGR2 */
	if (clk_on_imx6ull() != 0) {
		*hws.add(IMX6ULL_CLK_ESAI_EXTAL) = imx_clk_hw_gate2_shared("esai_extal",	"esai_podf",	base.add(0x70 as usize),	0, &share_count_esai);
		*hws.add(IMX6ULL_CLK_ESAI_IPG) = imx_clk_hw_gate2_shared("esai_ipg",	"ahb",		base.add(0x70 as usize),	0, &share_count_esai);
		*hws.add(IMX6ULL_CLK_ESAI_MEM) = imx_clk_hw_gate2_shared("esai_mem",	"ahb",		base.add(0x70 as usize),	0, &share_count_esai);
	}
	*hws.add(IMX6UL_CLK_I2C1) = imx_clk_hw_gate2("i2c1",		"perclk",	base.add(0x70 as usize),	6);
	*hws.add(IMX6UL_CLK_I2C2) = imx_clk_hw_gate2("i2c2",		"perclk",	base.add(0x70 as usize),	8);
	*hws.add(IMX6UL_CLK_I2C3) = imx_clk_hw_gate2("i2c3",		"perclk",	base.add(0x70 as usize),	10);
	*hws.add(IMX6UL_CLK_OCOTP) = imx_clk_hw_gate2("ocotp",	"ipg",		base.add(0x70 as usize),	12);
	*hws.add(IMX6UL_CLK_IOMUXC) = imx_clk_hw_gate2("iomuxc",	"lcdif_podf",	base.add(0x70 as usize),	14);
	*hws.add(IMX6UL_CLK_GPIO3) = imx_clk_hw_gate2("gpio3",	"ipg",		base.add(0x70 as usize),	26);
	*hws.add(IMX6UL_CLK_LCDIF_APB) = imx_clk_hw_gate2("lcdif_apb",	"axi",		base.add(0x70 as usize),	28);
	*hws.add(IMX6UL_CLK_PXP) = imx_clk_hw_gate2("pxp",		"axi",		base.add(0x70 as usize),	30);

	/* CCGR3 */
	/*
	 * Although the imx6ull reference manual lists CCGR2 as the csi clk
	 * gate register, tests have shown that it is actually the CCGR3
	 * register bit 0/1, same as for the imx6ul.
	 */
	*hws.add(IMX6UL_CLK_CSI) = imx_clk_hw_gate2("csi",	"csi_podf",	base.add(0x74 as usize),	0);
	*hws.add(IMX6UL_CLK_UART5_IPG) = imx_clk_hw_gate2("uart5_ipg",	"ipg",		base.add(0x74 as usize),	2);
	*hws.add(IMX6UL_CLK_UART5_SERIAL) = imx_clk_hw_gate2("uart5_serial",	"uart_podf",	base.add(0x74 as usize),	2);
	if (clk_on_imx6ul() != 0) {
		*hws.add(IMX6UL_CLK_ENET) = imx_clk_hw_gate2("enet",		"ipg",		base.add(0x74 as usize),	4);
		*hws.add(IMX6UL_CLK_ENET_AHB) = imx_clk_hw_gate2("enet_ahb",	"ahb",		base.add(0x74 as usize),	4);
	} else if (clk_on_imx6ull() != 0 != 0) {
		*hws.add(IMX6ULL_CLK_EPDC_ACLK) = imx_clk_hw_gate2("epdc_aclk",	"axi",		base.add(0x74 as usize),	4);
		*hws.add(IMX6ULL_CLK_EPDC_PIX) = imx_clk_hw_gate2("epdc_pix",	"epdc_podf",	base.add(0x74 as usize),	4);
	}
	*hws.add(IMX6UL_CLK_UART6_IPG) = imx_clk_hw_gate2("uart6_ipg",	"ipg",		base.add(0x74 as usize),	6);
	*hws.add(IMX6UL_CLK_UART6_SERIAL) = imx_clk_hw_gate2("uart6_serial",	"uart_podf",	base.add(0x74 as usize),	6);
	*hws.add(IMX6UL_CLK_LCDIF_PIX) = imx_clk_hw_gate2("lcdif_pix",	"lcdif_podf",	base.add(0x74 as usize),	10);
	*hws.add(IMX6UL_CLK_GPIO4) = imx_clk_hw_gate2("gpio4",	"ipg",		base.add(0x74 as usize),	12);
	*hws.add(IMX6UL_CLK_QSPI) = imx_clk_hw_gate2("qspi1",	"qspi1_podf",	base.add(0x74 as usize),	14);
	*hws.add(IMX6UL_CLK_WDOG1) = imx_clk_hw_gate2("wdog1",	"ipg",		base.add(0x74 as usize),	16);
	*hws.add(IMX6UL_CLK_MMDC_P0_FAST) = imx_clk_hw_gate_flags("mmdc_p0_fast", "mmdc_podf", base.add(0x74 as usize),	20, CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_MMDC_P0_IPG) = imx_clk_hw_gate2_flags("mmdc_p0_ipg",	"ipg",		base.add(0x74 as usize),	24, CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_MMDC_P1_IPG) = imx_clk_hw_gate2_flags("mmdc_p1_ipg",	"ipg",		base.add(0x74 as usize),	26, CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_AXI) = imx_clk_hw_gate_flags("axi",	"axi_podf",	base.add(0x74 as usize),	28, CLK_IS_CRITICAL);

	/* CCGR4 */
	*hws.add(IMX6UL_CLK_PER_BCH) = imx_clk_hw_gate2("per_bch",	"bch_podf",	base.add(0x78 as usize),	12);
	*hws.add(IMX6UL_CLK_PWM1) = imx_clk_hw_gate2("pwm1",		"perclk",	base.add(0x78 as usize),	16);
	*hws.add(IMX6UL_CLK_PWM2) = imx_clk_hw_gate2("pwm2",		"perclk",	base.add(0x78 as usize),	18);
	*hws.add(IMX6UL_CLK_PWM3) = imx_clk_hw_gate2("pwm3",		"perclk",	base.add(0x78 as usize),	20);
	*hws.add(IMX6UL_CLK_PWM4) = imx_clk_hw_gate2("pwm4",		"perclk",	base.add(0x78 as usize),	22);
	*hws.add(IMX6UL_CLK_GPMI_BCH_APB) = imx_clk_hw_gate2("gpmi_bch_apb",	"bch_podf",	base.add(0x78 as usize),	24);
	*hws.add(IMX6UL_CLK_GPMI_BCH) = imx_clk_hw_gate2("gpmi_bch",	"gpmi_podf",	base.add(0x78 as usize),	26);
	*hws.add(IMX6UL_CLK_GPMI_IO) = imx_clk_hw_gate2("gpmi_io",	"enfc_podf",	base.add(0x78 as usize),	28);
	*hws.add(IMX6UL_CLK_GPMI_APB) = imx_clk_hw_gate2("gpmi_apb",	"bch_podf",	base.add(0x78 as usize),	30);

	/* CCGR5 */
	*hws.add(IMX6UL_CLK_ROM) = imx_clk_hw_gate2_flags("rom",	"ahb",		base.add(0x7c as usize),	0,	CLK_IS_CRITICAL);
	*hws.add(IMX6UL_CLK_SDMA) = imx_clk_hw_gate2("sdma",		"ahb",		base.add(0x7c as usize),	6);
	*hws.add(IMX6UL_CLK_KPP) = imx_clk_hw_gate2("kpp",		"ipg",		base.add(0x7c as usize),	8);
	*hws.add(IMX6UL_CLK_WDOG2) = imx_clk_hw_gate2("wdog2",	"ipg",		base.add(0x7c as usize),	10);
	*hws.add(IMX6UL_CLK_SPBA) = imx_clk_hw_gate2("spba",		"ipg",		base.add(0x7c as usize),	12);
	*hws.add(IMX6UL_CLK_SPDIF) = imx_clk_hw_gate2_shared("spdif",		"spdif_podf",	base.add(0x7c as usize),	14, &share_count_audio);
	*hws.add(IMX6UL_CLK_SPDIF_GCLK) = imx_clk_hw_gate2_shared("spdif_gclk",	"ipg",		base.add(0x7c as usize),	14, &share_count_audio);
	*hws.add(IMX6UL_CLK_SAI3) = imx_clk_hw_gate2_shared("sai3",		"sai3_podf",	base.add(0x7c as usize),	22, &share_count_sai3);
	*hws.add(IMX6UL_CLK_SAI3_IPG) = imx_clk_hw_gate2_shared("sai3_ipg",	"ipg",		base.add(0x7c as usize),	22, &share_count_sai3);
	*hws.add(IMX6UL_CLK_UART1_IPG) = imx_clk_hw_gate2("uart1_ipg",	"ipg",		base.add(0x7c as usize),	24);
	*hws.add(IMX6UL_CLK_UART1_SERIAL) = imx_clk_hw_gate2("uart1_serial",	"uart_podf",	base.add(0x7c as usize),	24);
	*hws.add(IMX6UL_CLK_UART7_IPG) = imx_clk_hw_gate2("uart7_ipg",	"ipg",		base.add(0x7c as usize),	26);
	*hws.add(IMX6UL_CLK_UART7_SERIAL) = imx_clk_hw_gate2("uart7_serial",	"uart_podf",	base.add(0x7c as usize),	26);
	*hws.add(IMX6UL_CLK_SAI1) = imx_clk_hw_gate2_shared("sai1",		"sai1_podf",	base.add(0x7c as usize),	28, &share_count_sai1);
	*hws.add(IMX6UL_CLK_SAI1_IPG) = imx_clk_hw_gate2_shared("sai1_ipg",	"ipg",		base.add(0x7c as usize),	28, &share_count_sai1);
	*hws.add(IMX6UL_CLK_SAI2) = imx_clk_hw_gate2_shared("sai2",		"sai2_podf",	base.add(0x7c as usize),	30, &share_count_sai2);
	*hws.add(IMX6UL_CLK_SAI2_IPG) = imx_clk_hw_gate2_shared("sai2_ipg",	"ipg",		base.add(0x7c as usize),	30, &share_count_sai2);

	/* CCGR6 */
	*hws.add(IMX6UL_CLK_USBOH3) = imx_clk_hw_gate2("usboh3",	"ipg",		 base.add(0x80 as usize),	0);
	*hws.add(IMX6UL_CLK_USDHC1) = imx_clk_hw_gate2("usdhc1",	"usdhc1_podf",	 base.add(0x80 as usize),	2);
	*hws.add(IMX6UL_CLK_USDHC2) = imx_clk_hw_gate2("usdhc2",	"usdhc2_podf",	 base.add(0x80 as usize),	4);
	if (clk_on_imx6ul() != 0) {
		*hws.add(IMX6UL_CLK_SIM1) = imx_clk_hw_gate2("sim1",		"sim_sel",	 base.add(0x80 as usize),	6);
		*hws.add(IMX6UL_CLK_SIM2) = imx_clk_hw_gate2("sim2",		"sim_sel",	 base.add(0x80 as usize),	8);
	}
	*hws.add(IMX6UL_CLK_EIM) = imx_clk_hw_gate2("eim",		"eim_slow_podf", base.add(0x80 as usize),	10);
	*hws.add(IMX6UL_CLK_PWM8) = imx_clk_hw_gate2("pwm8",		"perclk",	 base.add(0x80 as usize),	16);
	*hws.add(IMX6UL_CLK_UART8_IPG) = imx_clk_hw_gate2("uart8_ipg",	"ipg",		 base.add(0x80 as usize),	14);
	*hws.add(IMX6UL_CLK_UART8_SERIAL) = imx_clk_hw_gate2("uart8_serial", "uart_podf",	 base.add(0x80 as usize),	14);
	*hws.add(IMX6UL_CLK_WDOG3) = imx_clk_hw_gate2("wdog3",	"ipg",		 base.add(0x80 as usize),	20);
	*hws.add(IMX6UL_CLK_I2C4) = imx_clk_hw_gate2("i2c4",		"perclk",	 base.add(0x80 as usize),	24);
	*hws.add(IMX6UL_CLK_PWM5) = imx_clk_hw_gate2("pwm5",		"perclk",	 base.add(0x80 as usize),	26);
	*hws.add(IMX6UL_CLK_PWM6) = imx_clk_hw_gate2("pwm6",		"perclk",	 base +	0x80,	28);
	*hws.add(IMX6UL_CLK_PWM7) = imx_clk_hw_gate2("pwm7",		"perclk",	 base.add(0x80 as usize),	30);

	/* CCOSR */
	*hws.add(IMX6UL_CLK_CKO1) = imx_clk_hw_gate("cko1",		"cko1_podf",	 base.add(0x60 as usize),	7);
	*hws.add(IMX6UL_CLK_CKO2) = imx_clk_hw_gate("cko2",		"cko2_podf",	 base.add(0x60 as usize),	24);

	/* mask handshake of mmdc */
	imx_mmdc_mask_handshake(base, 0);

	*hws.add(IMX6UL_CLK_ENET1_REF_PAD) = imx_obtain_fixed_of_clock(ccm_node, "enet1_ref_pad", 0);

	*hws.add(IMX6UL_CLK_ENET1_REF_SEL) = imx_clk_gpr_mux("enet1_ref_sel", "fsl,imx6ul-iomuxc-gpr",
				IOMUXC_GPR1, enet1_ref_sels, enet1_ref_sels.len(),
				enet1_ref_sels_table, enet1_ref_sels_table_mask);
	*hws.add(IMX6UL_CLK_ENET2_REF_PAD) = imx_obtain_fixed_of_clock(ccm_node, "enet2_ref_pad", 0);

	*hws.add(IMX6UL_CLK_ENET2_REF_SEL) = imx_clk_gpr_mux("enet2_ref_sel", "fsl,imx6ul-iomuxc-gpr",
				IOMUXC_GPR1, enet2_ref_sels, enet2_ref_sels.len(),
				enet2_ref_sels_table, enet2_ref_sels_table_mask);

	imx_check_clk_hws(hws, IMX6UL_CLK_END);

	of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_hw_data);

	/*
	 * Lower the AHB clock rate before changing the parent clock source,
	 * as AHB clock rate can NOT be higher than 133MHz, but its parent
	 * will be switched from 396MHz PFD to 528MHz PLL in order to increase
	 * AXI clock rate, so we need to lower AHB rate first to make sure at
	 * any time, AHB rate is <= 133MHz.
	 */
	clk_set_rate(*hws.add(IMX6UL_CLK_AHB as usize).clk, 99000000);

	/* Change periph_pre clock to pll2_bus to adjust AXI rate to 264MHz */
	clk_set_parent(*hws.add(IMX6UL_CLK_PERIPH_CLK2_SEL as usize).clk, *hws.add(IMX6UL_CLK_OSC as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_CLK_PERIPH as usize).clk, *hws.add(IMX6UL_CLK_PERIPH_CLK2 as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_CLK_PERIPH_PRE as usize).clk, *hws.add(IMX6UL_CLK_PLL2_BUS as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_CLK_PERIPH as usize).clk, *hws.add(IMX6UL_CLK_PERIPH_PRE as usize).clk);

	/* Make sure AHB rate is 132MHz  */
	clk_set_rate(*hws.add(IMX6UL_CLK_AHB as usize).clk, 132000000);

	/* set perclk to from OSC */
	clk_set_parent(*hws.add(IMX6UL_CLK_PERCLK_SEL as usize).clk, *hws.add(IMX6UL_CLK_OSC as usize).clk);

	clk_set_rate(*hws.add(IMX6UL_CLK_ENET_REF as usize).clk, 50000000);
	clk_set_rate(*hws.add(IMX6UL_CLK_ENET2_REF as usize).clk, 50000000);
	clk_set_rate(*hws.add(IMX6UL_CLK_CSI as usize).clk, 24000000);

	if (clk_on_imx6ull() != 0) {
		clk_prepare_enable(*hws.add(IMX6UL_CLK_AIPSTZ3 as usize).clk);

	if (IS_ENABLED(CONFIG_USB_MXS_PHY) != 0) {
		clk_prepare_enable(*hws.add(IMX6UL_CLK_USBPHY1_GATE as usize).clk);
		clk_prepare_enable(*hws.add(IMX6UL_CLK_USBPHY2_GATE as usize).clk);
	}

	clk_set_parent(*hws.add(IMX6UL_CLK_CAN_SEL as usize).clk, *hws.add(IMX6UL_CLK_PLL3_80M as usize).clk);
	if (clk_on_imx6ul() != 0) {
		clk_set_parent(*hws.add(IMX6UL_CLK_SIM_PRE_SEL as usize).clk, *hws.add(IMX6UL_CLK_PLL3_USB_OTG as usize).clk);
	else if (clk_on_imx6ull() != 0) {
		clk_set_parent(*hws.add(IMX6ULL_CLK_EPDC_PRE_SEL as usize).clk, *hws.add(IMX6UL_CLK_PLL3_PFD2 as usize).clk);

	clk_set_parent(*hws.add(IMX6UL_CLK_ENFC_SEL as usize).clk, *hws.add(IMX6UL_CLK_PLL2_PFD2 as usize).clk);

	clk_set_parent(*hws.add(IMX6UL_CLK_ENET1_REF_SEL as usize).clk, *hws.add(IMX6UL_CLK_ENET1_REF_125M as usize).clk);
	clk_set_parent(*hws.add(IMX6UL_CLK_ENET2_REF_SEL as usize).clk, *hws.add(IMX6UL_CLK_ENET2_REF_125M as usize).clk);

	imx_register_uart_clocks();
}

CLK_OF_DECLARE(imx6ul, "fsl,imx6ul-ccm", imx6ul_clocks_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
