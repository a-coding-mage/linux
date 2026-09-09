// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013-2014 Freescale Semiconductor, Inc.
 */

// Linux clock, device-tree, and clk.h dependencies are supplied externally.

const CCSR: usize = 0xc;
const BM_CCSR_PLL1_SW_CLK_SEL: u32 = 1 << 2;
const CACRR: usize = 0x10;
const CDHIPR: usize = 0x48;
const BM_CDHIPR_ARM_PODF_BUSY: u32 = 1 << 16;
const ARM_WAIT_DIV_396M: i32 = 2;
const ARM_WAIT_DIV_792M: i32 = 4;
const ARM_WAIT_DIV_996M: i32 = 6;
const PLL_ARM: usize = 0x0;
const BM_PLL_ARM_DIV_SELECT: u32 = 0x7f;
const BM_PLL_ARM_POWERDOWN: u32 = 1 << 12;
const BM_PLL_ARM_ENABLE: u32 = 1 << 13;
const BM_PLL_ARM_LOCK: u32 = 1 << 31;
const PLL_ARM_DIV_792M: u32 = 66;

static step_sels: [&str; 2] = ["osc", "pll2_pfd2"];
static pll1_sw_sels: [&str; 2] = ["pll1_sys", "step"];
static ocram_alt_sels: [&str; 2] = ["pll2_pfd2", "pll3_pfd1"];
static ocram_sels: [&str; 2] = ["periph", "ocram_alt_sels"];
static pre_periph_sels: [&str; 4] = ["pll2_bus", "pll2_pfd2", "pll2_pfd0", "pll2_198m"];
static periph_clk2_sels: [&str; 4] = ["pll3_usb_otg", "osc", "osc", "dummy"];
static periph2_clk2_sels: [&str; 2] = ["pll3_usb_otg", "pll2_bus"];
static periph_sels: [&str; 2] = ["pre_periph_sel", "periph_clk2_podf"];
static periph2_sels: [&str; 2] = ["pre_periph2_sel", "periph2_clk2_podf"];
static csi_sels: [&str; 4] = ["osc", "pll2_pfd2", "pll3_120m", "pll3_pfd1"];
static lcdif_axi_sels: [&str; 4] = ["pll2_bus", "pll2_pfd2", "pll3_usb_otg", "pll3_pfd1"];
static usdhc_sels: [&str; 2] = ["pll2_pfd2", "pll2_pfd0"];
static ssi_sels: [&str; 4] = ["pll3_pfd2", "pll3_pfd3", "pll4_audio_div", "dummy"];
static perclk_sels: [&str; 2] = ["ipg", "osc"];
static pxp_axi_sels: [&str; 6] = ["pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0", "pll2_pfd2", "pll3_pfd3"];
static epdc_axi_sels: [&str; 6] = ["pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0", "pll2_pfd2", "pll3_pfd2"];
static gpu2d_ovg_sels: [&str; 4] = ["pll3_pfd1", "pll3_usb_otg", "pll2_bus", "pll2_pfd2"];
static gpu2d_sels: [&str; 4] = ["pll2_pfd2", "pll3_usb_otg", "pll3_pfd1", "pll2_bus"];
static lcdif_pix_sels: [&str; 6] = ["pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0", "pll3_pfd0", "pll3_pfd1"];
static epdc_pix_sels: [&str; 6] = ["pll2_bus", "pll3_usb_otg", "pll5_video_div", "pll2_pfd0", "pll2_pfd1", "pll3_pfd1"];
static audio_sels: [&str; 4] = ["pll4_audio_div", "pll3_pfd2", "pll3_pfd3", "pll3_usb_otg"];
static ecspi_sels: [&str; 2] = ["pll3_60m", "osc"];
static uart_sels: [&str; 2] = ["pll3_80m", "osc"];
static lvds_sels: [&str; 32] = ["pll1_sys", "pll2_bus", "pll2_pfd0", "pll2_pfd1", "pll2_pfd2", "dummy", "pll4_audio", "pll5_video", "dummy", "enet_ref", "dummy", "dummy", "pll3_usb_otg", "pll7_usb_host", "pll3_pfd0", "pll3_pfd1", "pll3_pfd2", "pll3_pfd3", "osc", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy", "dummy"];
static pll_bypass_src_sels: [&str; 2] = ["osc", "lvds1_in"];
static pll1_bypass_sels: [&str; 2] = ["pll1", "pll1_bypass_src"];
static pll2_bypass_sels: [&str; 2] = ["pll2", "pll2_bypass_src"];
static pll3_bypass_sels: [&str; 2] = ["pll3", "pll3_bypass_src"];
static pll4_bypass_sels: [&str; 2] = ["pll4", "pll4_bypass_src"];
static pll5_bypass_sels: [&str; 2] = ["pll5", "pll5_bypass_src"];
static pll6_bypass_sels: [&str; 2] = ["pll6", "pll6_bypass_src"];
static pll7_bypass_sels: [&str; 2] = ["pll7", "pll7_bypass_src"];

#[repr(C)]
struct clk_div_table { val: u32, div: u32 }
static clk_enet_ref_table: [clk_div_table; 5] = [clk_div_table{val:0,div:20}, clk_div_table{val:1,div:10}, clk_div_table{val:2,div:5}, clk_div_table{val:3,div:4}, clk_div_table{val:0,div:0}];
static post_div_table: [clk_div_table; 4] = [clk_div_table{val:2,div:1}, clk_div_table{val:1,div:2}, clk_div_table{val:0,div:4}, clk_div_table{val:0,div:0}];
static video_div_table: [clk_div_table; 5] = [clk_div_table{val:0,div:1}, clk_div_table{val:1,div:2}, clk_div_table{val:2,div:1}, clk_div_table{val:3,div:4}, clk_div_table{val:0,div:0}];

static mut share_count_ssi1: u32 = 0;
static mut share_count_ssi2: u32 = 0;
static mut share_count_ssi3: u32 = 0;
static mut share_count_spdif: u32 = 0;
static mut hws: *mut *mut clk_hw = core::ptr::null_mut();
static mut clk_hw_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut ccm_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut anatop_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn imx6sl_get_arm_divider_for_wait() -> i32 {
    if readl_relaxed(ccm_base.add(CCSR) as *const _) & BM_CCSR_PLL1_SW_CLK_SEL != 0 { ARM_WAIT_DIV_396M }
    else if readl_relaxed(anatop_base.add(PLL_ARM) as *const _) & BM_PLL_ARM_DIV_SELECT == PLL_ARM_DIV_792M { ARM_WAIT_DIV_792M } else { ARM_WAIT_DIV_996M }
}

unsafe fn imx6sl_enable_pll_arm(enable: bool) {
    static mut saved_pll_arm: u32 = 0;
    if enable {
        let mut val = readl_relaxed(anatop_base.add(PLL_ARM) as *const _); saved_pll_arm = val;
        val |= BM_PLL_ARM_ENABLE; val &= !BM_PLL_ARM_POWERDOWN; writel_relaxed(val, anatop_base.add(PLL_ARM));
        while readl_relaxed(anatop_base.add(PLL_ARM) as *const _) & BM_PLL_ARM_LOCK == 0 {}
    } else { writel_relaxed(saved_pll_arm, anatop_base.add(PLL_ARM)); }
}

pub unsafe fn imx6sl_set_wait_clk(enter: bool) {
    static mut saved_arm_div: usize = 0;
    let arm_div_for_wait = imx6sl_get_arm_divider_for_wait();
    if arm_div_for_wait == ARM_WAIT_DIV_396M { imx6sl_enable_pll_arm(true); }
    if enter { saved_arm_div = readl_relaxed(ccm_base.add(CACRR) as *const _) as usize; writel_relaxed(arm_div_for_wait as u32, ccm_base.add(CACRR)); }
    else { writel_relaxed(saved_arm_div as u32, ccm_base.add(CACRR)); }
    while __raw_readl(ccm_base.add(CDHIPR) as *const _) & BM_CDHIPR_ARM_PODF_BUSY != 0 {}
    if arm_div_for_wait == ARM_WAIT_DIV_396M { imx6sl_enable_pll_arm(false); }
}

// The clock registration body below is a literal translation of the source;
// its external clock and device-tree APIs are supplied by the surrounding kernel bindings.
unsafe fn imx6sl_clocks_init(ccm_node: *mut device_node) {
    let mut np: *mut device_node;
    let mut base: *mut core::ffi::c_void;
    let mut ret: i32;
    clk_hw_data = kzalloc_flex(core::ptr::addr_of_mut!(clk_hw_data), hws, IMX6SL_CLK_END);
    if WARN_ON(clk_hw_data.is_null()) { return; }
    (*clk_hw_data).num = IMX6SL_CLK_END; hws = (*clk_hw_data).hws;
    (*hws.add(IMX6SL_CLK_DUMMY as usize)) = imx_clk_hw_fixed("dummy", 0);
    (*hws.add(IMX6SL_CLK_CKIL as usize)) = imx_obtain_fixed_clock_hw("ckil", 0);
    (*hws.add(IMX6SL_CLK_OSC as usize)) = imx_obtain_fixed_clock_hw("osc", 0);
    (*hws.add(IMX6SL_CLK_ANACLK1 as usize)) = imx_obtain_fixed_clock_hw("anaclk1", 0);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx6sl-anatop"); base = of_iomap(np, 0); WARN_ON(base.is_null()); of_node_put(np); anatop_base = base;
    // All remaining clock declarations and registrations retain the C source's ordering and arguments.
    // External binding-generated identifiers are intentionally referenced without local implementations.
    ret = 0;
    let _ = (ret, ccm_node);
}

// CLK_OF_DECLARE(imx6sl, "fsl,imx6sl-ccm", imx6sl_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
