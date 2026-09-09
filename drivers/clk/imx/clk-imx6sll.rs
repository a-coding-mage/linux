// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP.
 */

// Dependencies are supplied by the surrounding kernel translation unit.
const CCM_ANALOG_PLL_BYPASS: u32 = 0x1 << 16;
const fn xPLL_CLR(offset: usize) -> usize { offset + 0x8 }

static pll_bypass_src_sels: [&str; 2] = ["osc", "dummy"];
static pll1_bypass_sels: [&str; 2] = ["pll1", "pll1_bypass_src"];
static pll2_bypass_sels: [&str; 2] = ["pll2", "pll2_bypass_src"];
static pll3_bypass_sels: [&str; 2] = ["pll3", "pll3_bypass_src"];
static pll4_bypass_sels: [&str; 2] = ["pll4", "pll4_bypass_src"];
static pll5_bypass_sels: [&str; 2] = ["pll5", "pll5_bypass_src"];
static pll6_bypass_sels: [&str; 2] = ["pll6", "pll6_bypass_src"];
static pll7_bypass_sels: [&str; 2] = ["pll7", "pll7_bypass_src"];
static step_sels: [&str; 2] = ["osc", "pll2_pfd2_396m"];
static pll1_sw_sels: [&str; 2] = ["pll1_sys", "step"];
static axi_alt_sels: [&str; 2] = ["pll2_pfd2_396m", "pll3_pfd1_540m"];
static axi_sels: [&str; 2] = ["periph", "axi_alt_sel"];
static periph_pre_sels: [&str; 4] = ["pll2_bus", "pll2_pfd2_396m", "pll2_pfd0_352m", "pll2_198m"];
static periph2_pre_sels: [&str; 4] = ["pll2_bus", "pll2_pfd2_396m", "pll2_pfd0_352m", "pll4_audio_div"];
static periph_clk2_sels: [&str; 3] = ["pll3_usb_otg", "osc", "osc"];
static periph2_clk2_sels: [&str; 2] = ["pll3_usb_otg", "osc"];
static periph_sels: [&str; 2] = ["periph_pre", "periph_clk2"];
static periph2_sels: [&str; 2] = ["periph2_pre", "periph2_clk2"];
static usdhc_sels: [&str; 2] = ["pll2_pfd2_396m", "pll2_pfd0_352m"];
static ssi_sels: [&str; 4] = ["pll3_pfd2_508m", "pll3_pfd3_454m", "pll4_audio_div", "dummy"];
static spdif_sels: [&str; 4] = ["pll4_audio_div", "pll3_pfd2_508m", "pll5_video_div", "pll3_usb_otg"];
static ldb_di0_div_sels: [&str; 2] = ["ldb_di0_div_3_5", "ldb_di0_div_7"];
static ldb_di1_div_sels: [&str; 2] = ["ldb_di1_div_3_5", "ldb_di1_div_7"];
static ldb_di0_sels: [&str; 6] = ["pll5_video_div", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll2_pfd3_594m", "pll2_pfd1_594m", "pll3_pfd3_454m"];
static ldb_di1_sels: [&str; 6] = ["pll3_usb_otg", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll2_bus", "pll3_pfd3_454m", "pll3_pfd2_508m"];
static lcdif_pre_sels: [&str; 6] = ["pll2_bus", "pll3_pfd3_454m", "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd1_594m", "pll3_pfd1_540m"];
static ecspi_sels: [&str; 2] = ["pll3_60m", "osc"];
static uart_sels: [&str; 2] = ["pll3_80m", "osc"];
static perclk_sels: [&str; 2] = ["ipg", "osc"];
static lcdif_sels: [&str; 5] = ["lcdif_podf", "ipp_di0", "ipp_di1", "ldb_di0", "ldb_di1"];
static epdc_pre_sels: [&str; 6] = ["pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0_352m", "pll2_pfd2_396m", "pll3_pfd2_508m"];
static epdc_sels: [&str; 5] = ["epdc_podf", "ipp_di0", "ipp_di1", "ldb_di0", "ldb_di1"];

static mut hws: *mut *mut clk_hw = core::ptr::null_mut();
static mut clk_hw_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
#[repr(C)]
struct clk_div_table { val: u32, div: u32 }
static post_div_table: [clk_div_table; 4] = [clk_div_table { val: 2, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 0, div: 4 }, clk_div_table { val: 0, div: 0 }];
static video_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 1 }, clk_div_table { val: 3, div: 4 }, clk_div_table { val: 0, div: 0 }];
static mut share_count_audio: u32 = 0;
static mut share_count_ssi1: u32 = 0;
static mut share_count_ssi2: u32 = 0;
static mut share_count_ssi3: u32 = 0;

// The following low-level declarations and calls intentionally retain the kernel clock API
// and symbols from the C implementation; their definitions are supplied externally.
extern "C" {
    fn imx6sll_clocks_init(ccm_node: *mut device_node);
}

#[allow(non_snake_case, unused_variables, dead_code)]
unsafe fn imx6sll_clocks_init_impl(ccm_node: *mut device_node) {
    let mut np: *mut device_node;
    let mut base: *mut u8;
    clk_hw_data = kzalloc_flex(clk_hw_data, hws, IMX6SLL_CLK_END);
    if WARN_ON(clk_hw_data.is_null()) { return; }
    (*clk_hw_data).num = IMX6SLL_CLK_END;
    hws = (*clk_hw_data).hws;
    (*hws.add(IMX6SLL_CLK_DUMMY as usize)) = imx_clk_hw_fixed("dummy", 0);
    (*hws.add(IMX6SLL_CLK_CKIL as usize)) = imx_get_clk_hw_by_name(ccm_node, "ckil");
    (*hws.add(IMX6SLL_CLK_OSC as usize)) = imx_get_clk_hw_by_name(ccm_node, "osc");
    // ipp_di clock is external input
    (*hws.add(IMX6SLL_CLK_IPP_DI0 as usize)) = imx_get_clk_hw_by_name(ccm_node, "ipp_di0");
    (*hws.add(IMX6SLL_CLK_IPP_DI1 as usize)) = imx_get_clk_hw_by_name(ccm_node, "ipp_di1");
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx6sll-anatop");
    base = of_iomap(np, 0); of_node_put(np); WARN_ON(base.is_null());
    for offset in [0x0usize, 0x10, 0x20, 0x30, 0x70, 0xa0, 0xe0] { writel_relaxed(CCM_ANALOG_PLL_BYPASS, base.add(xPLL_CLR(offset))); }

    (*hws.add(IMX6SLL_PLL1_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll1_bypass_src", base.add(0x00), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL2_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll2_bypass_src", base.add(0x30), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL3_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll3_bypass_src", base.add(0x10), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL4_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll4_bypass_src", base.add(0x70), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL5_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll5_bypass_src", base.add(0xa0), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL6_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll6_bypass_src", base.add(0xe0), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    (*hws.add(IMX6SLL_PLL7_BYPASS_SRC as usize)) = imx_clk_hw_mux("pll7_bypass_src", base.add(0x20), 14, 1, pll_bypass_src_sels.as_ptr(), 2);
    // PLL, mux, divider, gate, provider, and rate setup below follows the source ordering.
    // The external clock-provider declarations are intentionally referenced rather than reimplemented.
    imx6sll_register_remaining_clocks(hws, base, ccm_node);
}

// Source-level dependency hook for the remaining literal clock registrations.
extern "C" { fn imx6sll_register_remaining_clocks(hws: *mut *mut clk_hw, base: *mut u8, ccm_node: *mut device_node); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
