// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 *               Weiyi Lu <weiyi.lu@mediatek.com>
 * Copyright (c) 2023 Collabora, Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the kernel clock, device-tree, platform, gate, and PLL APIs.

static APmixed_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x20,
    clr_ofs: 0x20,
    sta_ofs: 0x20,
};

const MT8183_PLL_FMAX: usize = 3800usize * MHZ;
const MT8183_PLL_FMIN: usize = 1500usize * MHZ;

static APMIXED_CLKS: &[mtk_gate] = &[
    GATE_MTK_FLAGS!(CLK_APMIXED_SSUSB_26M, "apmixed_ssusb26m", "f_f26m_ck", &APmixed_CG_REGS, 4, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_APPLL_26M, "apmixed_appll26m", "f_f26m_ck", &APmixed_CG_REGS, 5, &mtk_clk_gate_ops_no_setclr_inv, CLK_IS_CRITICAL),
    GATE_MTK_FLAGS!(CLK_APMIXED_MIPIC0_26M, "apmixed_mipic026m", "f_f26m_ck", &APmixed_CG_REGS, 6, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MDPLLGP_26M, "apmixed_mdpll26m", "f_f26m_ck", &APmixed_CG_REGS, 7, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MMSYS_26M, "apmixed_mmsys26m", "f_f26m_ck", &APmixed_CG_REGS, 8, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_UFS_26M, "apmixed_ufs26m", "f_f26m_ck", &APmixed_CG_REGS, 9, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MIPIC1_26M, "apmixed_mipic126m", "f_f26m_ck", &APmixed_CG_REGS, 11, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MEMPLL_26M, "apmixed_mempll26m", "f_f26m_ck", &APmixed_CG_REGS, 13, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_CLKSQ_LVPLL_26M, "apmixed_lvpll26m", "f_f26m_ck", &APmixed_CG_REGS, 14, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MIPID0_26M, "apmixed_mipid026m", "f_f26m_ck", &APmixed_CG_REGS, 16, &mtk_clk_gate_ops_no_setclr_inv, 0),
    GATE_MTK_FLAGS!(CLK_APMIXED_MIPID1_26M, "apmixed_mipid126m", "f_f26m_ck", &APmixed_CG_REGS, 17, &mtk_clk_gate_ops_no_setclr_inv, 0),
];

static ARMPLL_DIV_TABLE: &[mtk_pll_div_table] = &[
    mtk_pll_div_table { div: 0, freq: MT8183_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1500 * MHZ },
    mtk_pll_div_table { div: 2, freq: 750 * MHZ },
    mtk_pll_div_table { div: 3, freq: 375 * MHZ },
    mtk_pll_div_table { div: 4, freq: 187500000 },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static MFGPLL_DIV_TABLE: &[mtk_pll_div_table] = &[
    mtk_pll_div_table { div: 0, freq: MT8183_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1600 * MHZ },
    mtk_pll_div_table { div: 2, freq: 800 * MHZ },
    mtk_pll_div_table { div: 3, freq: 400 * MHZ },
    mtk_pll_div_table { div: 4, freq: 200 * MHZ },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static PLLS: &[mtk_pll_data] = &[
    pll_b!(CLK_APMIXED_ARMPLL_LL, "armpll_ll", 0x0200, 0x020C, 0, HAVE_RST_BAR | PLL_AO, BIT!(24), 22, 8, 0x0204, 24, 0x0, 0x0, 0, 0x0204, 0, 0, ARMPLL_DIV_TABLE),
    pll_b!(CLK_APMIXED_ARMPLL_L, "armpll_l", 0x0210, 0x021C, 0, HAVE_RST_BAR | PLL_AO, BIT!(24), 22, 8, 0x0214, 24, 0x0, 0x0, 0, 0x0214, 0, 0, ARMPLL_DIV_TABLE),
    pll!(CLK_APMIXED_CCIPLL, "ccipll", 0x0290, 0x029C, 0, HAVE_RST_BAR | PLL_AO, BIT!(24), 22, 8, 0x0294, 24, 0x0, 0x0, 0, 0x0294, 0, 0),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x0220, 0x022C, 0, HAVE_RST_BAR, BIT!(24), 22, 8, 0x0224, 24, 0x0, 0x0, 0, 0x0224, 0, 0),
    pll!(CLK_APMIXED_UNIV2PLL, "univ2pll", 0x0230, 0x023C, 0, HAVE_RST_BAR, BIT!(24), 22, 8, 0x0234, 24, 0x0, 0x0, 0, 0x0234, 0, 0),
    pll_b!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0240, 0x024C, 0, 0, 0, 22, 8, 0x0244, 24, 0x0, 0x0, 0, 0x0244, 0, 0, MFGPLL_DIV_TABLE),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0250, 0x025C, 0, 0, 0, 22, 8, 0x0254, 24, 0x0, 0x0, 0, 0x0254, 0, 0),
    pll!(CLK_APMIXED_TVDPLL, "tvdpll", 0x0260, 0x026C, 0, 0, 0, 22, 8, 0x0264, 24, 0x0, 0x0, 0, 0x0264, 0, 0),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0270, 0x027C, 0, HAVE_RST_BAR, BIT!(23), 22, 8, 0x0274, 24, 0x0, 0x0, 0, 0x0274, 0, 0),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x02A0, 0x02B0, 0, 0, 0, 32, 8, 0x02A0, 1, 0x02A8, 0x0014, 0, 0x02A4, 0, 0x02A0),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x02b4, 0x02c4, 0, 0, 0, 32, 8, 0x02B4, 1, 0x02BC, 0x0014, 1, 0x02B8, 0, 0x02B4),
];

unsafe fn clk_mt8183_apmixed_probe(pdev: *mut platform_device) -> c_int {
    let mut base: *mut c_void;
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    clk_data = mtk_devm_alloc_clk_data(dev, CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }
    ret = mtk_clk_register_plls(dev, PLLS.as_ptr(), PLLS.len(), clk_data);
    if ret != 0 { return ret; }
    ret = mtk_clk_register_gates(dev, node, APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    if ret != 0 { goto unregister_plls; }
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 { goto unregister_gates; }
    return 0;
unregister_gates:
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
unregister_plls:
    mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
    ret
}

static OF_MATCH_CLK_MT8183_APMIXED: &[of_device_id] = &[
    of_device_id { compatible: "mediatek,mt8183-apmixedsys" },
    of_device_id { compatible: "" }, // sentinel
];

static mut CLK_MT8183_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8183_apmixed_probe),
    driver: device_driver { name: "clk-mt8183-apmixed", of_match_table: OF_MATCH_CLK_MT8183_APMIXED.as_ptr() },
};

// builtin_platform_driver(CLK_MT8183_APMIXED_DRV)
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_apmixed)
// MODULE_DESCRIPTION("MediaTek MT8183 apmixedsys clocks driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
