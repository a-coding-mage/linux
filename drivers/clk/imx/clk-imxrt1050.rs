// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright (C) 2021
 * Author(s):
 * Jesse Taube <Mr.Bossman075@gmail.com>
 * Giulio Benetti <giulio.benetti@benettiengineering.com>
 */
// Dependencies supplied by the surrounding kernel clock implementation.

static PLL_REF_SELS: [&str; 2] = ["osc", "dummy"];
static PER_SELS: [&str; 2] = ["ipg_pdof", "osc"];
static PLL1_BYPASS_SELS: [&str; 2] = ["pll1_arm", "pll1_arm_ref_sel"];
static PLL2_BYPASS_SELS: [&str; 2] = ["pll2_sys", "pll2_sys_ref_sel"];
static PLL3_BYPASS_SELS: [&str; 2] = ["pll3_usb_otg", "pll3_usb_otg_ref_sel"];
static PLL5_BYPASS_SELS: [&str; 2] = ["pll5_video", "pll5_video_ref_sel"];
static PRE_PERIPH_SELS: [&str; 4] = ["pll2_sys", "pll2_pfd2_396m", "pll2_pfd0_352m", "arm_podf"];
static PERIPH_SELS: [&str; 2] = ["pre_periph_sel", "todo"];
static USDHC_SELS: [&str; 2] = ["pll2_pfd2_396m", "pll2_pfd0_352m"];
static LPUART_SELS: [&str; 2] = ["pll3_80m", "osc"];
static LCDIF_SELS: [&str; 6] = ["pll2_sys", "pll3_pfd3_454_74m", "pll5_video", "pll2_pfd0_352m", "pll2_pfd1_594m", "pll3_pfd1_664_62m"];
static SEMC_ALT_SELS: [&str; 2] = ["pll2_pfd2_396m", "pll3_pfd1_664_62m"];
static SEMC_SELS: [&str; 2] = ["periph_sel", "semc_alt_sel"];

static mut HWS: *mut clk_hw = core::ptr::null_mut();
static mut CLK_HW_DATA: *mut clk_hw_onecell_data = core::ptr::null_mut();

unsafe fn imxrt1050_clocks_probe(pdev: *mut platform_device) -> i32 {
    let mut ccm_base: *mut core::ffi::c_void;
    let mut pll_base: *mut core::ffi::c_void;
    let dev: *mut device = &mut (*pdev).dev;
    let np: *mut device_node = (*dev).of_node;
    let anp: *mut device_node;
    let ret: i32;

    CLK_HW_DATA = devm_kzalloc(dev, struct_size(CLK_HW_DATA, hws, IMXRT1050_CLK_END), GFP_KERNEL);
    if WARN_ON(CLK_HW_DATA.is_null()) {
        return -ENOMEM;
    }

    (*CLK_HW_DATA).num = IMXRT1050_CLK_END;
    HWS = (*CLK_HW_DATA).hws;

    *HWS.add(IMXRT1050_CLK_OSC as usize) = imx_get_clk_hw_by_name(np, "osc");

    anp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imxrt-anatop");
    pll_base = devm_of_iomap(dev, anp, 0, core::ptr::null_mut());
    of_node_put(anp);
    if WARN_ON(IS_ERR(pll_base)) {
        ret = PTR_ERR(pll_base);
        goto_unregister_hws!(HWS, IMXRT1050_CLK_END);
    }

    /* Anatop clocks */
    *HWS.add(IMXRT1050_CLK_DUMMY as usize) = imx_clk_hw_fixed("dummy", 0u64);
    *HWS.add(IMXRT1050_CLK_PLL1_REF_SEL as usize) = imx_clk_hw_mux("pll1_arm_ref_sel", pll_base.add(0x0), 14, 2, PLL_REF_SELS.as_ptr(), PLL_REF_SELS.len());
    *HWS.add(IMXRT1050_CLK_PLL2_REF_SEL as usize) = imx_clk_hw_mux("pll2_sys_ref_sel", pll_base.add(0x30), 14, 2, PLL_REF_SELS.as_ptr(), PLL_REF_SELS.len());
    *HWS.add(IMXRT1050_CLK_PLL3_REF_SEL as usize) = imx_clk_hw_mux("pll3_usb_otg_ref_sel", pll_base.add(0x10), 14, 2, PLL_REF_SELS.as_ptr(), PLL_REF_SELS.len());
    *HWS.add(IMXRT1050_CLK_PLL5_REF_SEL as usize) = imx_clk_hw_mux("pll5_video_ref_sel", pll_base.add(0xa0), 14, 2, PLL_REF_SELS.as_ptr(), PLL_REF_SELS.len());
    *HWS.add(IMXRT1050_CLK_PLL1_ARM as usize) = imx_clk_hw_pllv3(IMX_PLLV3_SYS, "pll1_arm", "pll1_arm_ref_sel", pll_base.add(0x0), 0x7f);
    *HWS.add(IMXRT1050_CLK_PLL2_SYS as usize) = imx_clk_hw_pllv3(IMX_PLLV3_GENERIC, "pll2_sys", "pll2_sys_ref_sel", pll_base.add(0x30), 0x1);
    *HWS.add(IMXRT1050_CLK_PLL3_USB_OTG as usize) = imx_clk_hw_pllv3(IMX_PLLV3_USB, "pll3_usb_otg", "pll3_usb_otg_ref_sel", pll_base.add(0x10), 0x1);
    *HWS.add(IMXRT1050_CLK_PLL5_VIDEO as usize) = imx_clk_hw_pllv3(IMX_PLLV3_AV, "pll5_video", "pll5_video_ref_sel", pll_base.add(0xa0), 0x7f);

    /* PLL bypass out */
    *HWS.add(IMXRT1050_CLK_PLL1_BYPASS as usize) = imx_clk_hw_mux_flags("pll1_bypass", pll_base.add(0x0), 16, 1, PLL1_BYPASS_SELS.as_ptr(), PLL1_BYPASS_SELS.len(), CLK_SET_RATE_PARENT);
    *HWS.add(IMXRT1050_CLK_PLL2_BYPASS as usize) = imx_clk_hw_mux_flags("pll2_bypass", pll_base.add(0x30), 16, 1, PLL2_BYPASS_SELS.as_ptr(), PLL2_BYPASS_SELS.len(), CLK_SET_RATE_PARENT);
    *HWS.add(IMXRT1050_CLK_PLL3_BYPASS as usize) = imx_clk_hw_mux_flags("pll3_bypass", pll_base.add(0x10), 16, 1, PLL3_BYPASS_SELS.as_ptr(), PLL3_BYPASS_SELS.len(), CLK_SET_RATE_PARENT);
    *HWS.add(IMXRT1050_CLK_PLL5_BYPASS as usize) = imx_clk_hw_mux_flags("pll5_bypass", pll_base.add(0xa0), 16, 1, PLL5_BYPASS_SELS.as_ptr(), PLL5_BYPASS_SELS.len(), CLK_SET_RATE_PARENT);
    *HWS.add(IMXRT1050_CLK_VIDEO_POST_DIV_SEL as usize) = imx_clk_hw_divider("video_post_div_sel", "pll5_video", pll_base.add(0xa0), 19, 2);
    *HWS.add(IMXRT1050_CLK_VIDEO_DIV as usize) = imx_clk_hw_divider("video_div", "video_post_div_sel", pll_base.add(0x170), 30, 2);
    *HWS.add(IMXRT1050_CLK_PLL3_80M as usize) = imx_clk_hw_fixed_factor("pll3_80m", "pll3_usb_otg", 1, 6);
    *HWS.add(IMXRT1050_CLK_PLL2_PFD0_352M as usize) = imx_clk_hw_pfd("pll2_pfd0_352m", "pll2_sys", pll_base.add(0x100), 0);
    *HWS.add(IMXRT1050_CLK_PLL2_PFD1_594M as usize) = imx_clk_hw_pfd("pll2_pfd1_594m", "pll2_sys", pll_base.add(0x100), 1);
    *HWS.add(IMXRT1050_CLK_PLL2_PFD2_396M as usize) = imx_clk_hw_pfd("pll2_pfd2_396m", "pll2_sys", pll_base.add(0x100), 2);
    *HWS.add(IMXRT1050_CLK_PLL3_PFD1_664_62M as usize) = imx_clk_hw_pfd("pll3_pfd1_664_62m", "pll3_usb_otg", pll_base.add(0xf0), 1);
    *HWS.add(IMXRT1050_CLK_PLL3_PFD3_454_74M as usize) = imx_clk_hw_pfd("pll3_pfd3_454_74m", "pll3_usb_otg", pll_base.add(0xf0), 3);

    /* CCM clocks */
    ccm_base = devm_platform_ioremap_resource(pdev, 0);
    if WARN_ON(IS_ERR(ccm_base)) {
        ret = PTR_ERR(ccm_base);
        goto_unregister_hws!(HWS, IMXRT1050_CLK_END);
    }
    *HWS.add(IMXRT1050_CLK_ARM_PODF as usize) = imx_clk_hw_divider("arm_podf", "pll1_arm", ccm_base.add(0x10), 0, 3);
    *HWS.add(IMXRT1050_CLK_PRE_PERIPH_SEL as usize) = imx_clk_hw_mux("pre_periph_sel", ccm_base.add(0x18), 18, 2, PRE_PERIPH_SELS.as_ptr(), PRE_PERIPH_SELS.len());
    *HWS.add(IMXRT1050_CLK_PERIPH_SEL as usize) = imx_clk_hw_mux("periph_sel", ccm_base.add(0x14), 25, 1, PERIPH_SELS.as_ptr(), PERIPH_SELS.len());
    *HWS.add(IMXRT1050_CLK_USDHC1_SEL as usize) = imx_clk_hw_mux("usdhc1_sel", ccm_base.add(0x1c), 16, 1, USDHC_SELS.as_ptr(), USDHC_SELS.len());
    *HWS.add(IMXRT1050_CLK_USDHC2_SEL as usize) = imx_clk_hw_mux("usdhc2_sel", ccm_base.add(0x1c), 17, 1, USDHC_SELS.as_ptr(), USDHC_SELS.len());
    *HWS.add(IMXRT1050_CLK_LPUART_SEL as usize) = imx_clk_hw_mux("lpuart_sel", ccm_base.add(0x24), 6, 1, LPUART_SELS.as_ptr(), LPUART_SELS.len());
    *HWS.add(IMXRT1050_CLK_LCDIF_SEL as usize) = imx_clk_hw_mux("lcdif_sel", ccm_base.add(0x38), 15, 3, LCDIF_SELS.as_ptr(), LCDIF_SELS.len());
    *HWS.add(IMXRT1050_CLK_PER_CLK_SEL as usize) = imx_clk_hw_mux("per_sel", ccm_base.add(0x1c), 6, 1, PER_SELS.as_ptr(), PER_SELS.len());
    *HWS.add(IMXRT1050_CLK_SEMC_ALT_SEL as usize) = imx_clk_hw_mux("semc_alt_sel", ccm_base.add(0x14), 7, 1, SEMC_ALT_SELS.as_ptr(), SEMC_ALT_SELS.len());
    *HWS.add(IMXRT1050_CLK_SEMC_SEL as usize) = imx_clk_hw_mux_flags("semc_sel", ccm_base.add(0x14), 6, 1, SEMC_SELS.as_ptr(), SEMC_SELS.len(), CLK_IS_CRITICAL);
    *HWS.add(IMXRT1050_CLK_AHB_PODF as usize) = imx_clk_hw_divider("ahb", "periph_sel", ccm_base.add(0x14), 10, 3);
    *HWS.add(IMXRT1050_CLK_IPG_PDOF as usize) = imx_clk_hw_divider("ipg", "ahb", ccm_base.add(0x14), 8, 2);
    *HWS.add(IMXRT1050_CLK_PER_PDOF as usize) = imx_clk_hw_divider("per", "per_sel", ccm_base.add(0x1c), 0, 5);
    *HWS.add(IMXRT1050_CLK_USDHC1_PODF as usize) = imx_clk_hw_divider("usdhc1_podf", "usdhc1_sel", ccm_base.add(0x24), 11, 3);
    *HWS.add(IMXRT1050_CLK_USDHC2_PODF as usize) = imx_clk_hw_divider("usdhc2_podf", "usdhc2_sel", ccm_base.add(0x24), 16, 3);
    *HWS.add(IMXRT1050_CLK_LPUART_PODF as usize) = imx_clk_hw_divider("lpuart_podf", "lpuart_sel", ccm_base.add(0x24), 0, 6);
    *HWS.add(IMXRT1050_CLK_LCDIF_PRED as usize) = imx_clk_hw_divider("lcdif_pred", "lcdif_sel", ccm_base.add(0x38), 12, 3);
    *HWS.add(IMXRT1050_CLK_LCDIF_PODF as usize) = imx_clk_hw_divider("lcdif_podf", "lcdif_pred", ccm_base.add(0x18), 23, 3);
    *HWS.add(IMXRT1050_CLK_USDHC1 as usize) = imx_clk_hw_gate2("usdhc1", "usdhc1_podf", ccm_base.add(0x80), 2);
    *HWS.add(IMXRT1050_CLK_USDHC2 as usize) = imx_clk_hw_gate2("usdhc2", "usdhc2_podf", ccm_base.add(0x80), 4);
    *HWS.add(IMXRT1050_CLK_LPUART1 as usize) = imx_clk_hw_gate2("lpuart1", "lpuart_podf", ccm_base.add(0x7c), 24);
    *HWS.add(IMXRT1050_CLK_LCDIF_APB as usize) = imx_clk_hw_gate2("lcdif", "lcdif_podf", ccm_base.add(0x70), 28);
    *HWS.add(IMXRT1050_CLK_LCDIF_PIX as usize) = imx_clk_hw_gate2("lcdif_pix", "lcdif", ccm_base.add(0x74), 10);
    *HWS.add(IMXRT1050_CLK_DMA as usize) = imx_clk_hw_gate("dma", "ipg", ccm_base.add(0x7c), 6);
    *HWS.add(IMXRT1050_CLK_DMA_MUX as usize) = imx_clk_hw_gate("dmamux0", "ipg", ccm_base.add(0x7c), 7);
    imx_check_clk_hws(HWS, IMXRT1050_CLK_END);
    ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, CLK_HW_DATA);
    if ret < 0 {
        dev_err(dev, "Failed to register clks for i.MXRT1050.\n");
        goto_unregister_hws!(HWS, IMXRT1050_CLK_END);
    }
    return 0;
}

// The following registration metadata mirrors the C driver/module declarations.
static IMXRT1050_CLK_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "fsl,imxrt1050-ccm" },
    of_device_id { ..Default::default() },
];
static mut IMXRT1050_CLK_DRIVER: platform_driver = platform_driver {
    probe: Some(imxrt1050_clocks_probe),
    driver: device_driver { name: "imxrt1050-ccm", of_match_table: IMXRT1050_CLK_OF_MATCH.as_ptr() },
};
// module_platform_driver(imxrt1050_clk_driver);
// MODULE_DESCRIPTION("NXP i.MX RT1050 clock driver");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_AUTHOR("Jesse Taube <Mr.Bossman075@gmail.com>");
// MODULE_AUTHOR("Giulio Benetti <giulio.benetti@benettiengineering.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
