// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Copyright (c) 2023 Collabora Ltd.
 */

// Dependency declarations supplied by the surrounding kernel translation.

const MT8365_PLL_FMAX: u64 = 3800u64 * MHZ;
const MT8365_PLL_FMIN: u64 = 1500u64 * MHZ;
const CON0_MT8365_RST_BAR: u32 = BIT(23);

macro_rules! pll_b {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $div_table:expr,
     $rst_bar_mask:expr, $pcw_chg_reg:expr) => {
        mtk_pll_data {
            id: $id,
            name: $name,
            reg: $reg,
            pwr_reg: $pwr_reg,
            en_mask: $en_mask,
            flags: $flags,
            rst_bar_mask: $rst_bar_mask,
            fmax: MT8365_PLL_FMAX,
            fmin: MT8365_PLL_FMIN,
            pcwbits: $pcwbits,
            pcwibits: 8,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            tuner_reg: $tuner_reg,
            tuner_en_reg: $tuner_en_reg,
            tuner_en_bit: $tuner_en_bit,
            pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift,
            pcw_chg_reg: $pcw_chg_reg,
            div_table: $div_table,
        }
    };
}

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $rst_bar_mask:expr,
     $pcw_chg_reg:expr) => {
        pll_b!($id, $name, $reg, $pwr_reg, $en_mask, $flags, $pcwbits,
               $pd_reg, $pd_shift, $tuner_reg, $tuner_en_reg,
               $tuner_en_bit, $pcw_reg, $pcw_shift, core::ptr::null(),
               $rst_bar_mask, $pcw_chg_reg)
    };
}

static ARMPLL_DIV_TABLE: &[mtk_pll_div_table] = &[
    mtk_pll_div_table { div: 0, freq: MT8365_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1500 * MHZ },
    mtk_pll_div_table { div: 2, freq: 750 * MHZ },
    mtk_pll_div_table { div: 3, freq: 375 * MHZ },
    mtk_pll_div_table { div: 4, freq: 182500000 },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static MFGPLL_DIV_TABLE: &[mtk_pll_div_table] = &[
    mtk_pll_div_table { div: 0, freq: MT8365_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1600 * MHZ },
    mtk_pll_div_table { div: 2, freq: 800 * MHZ },
    mtk_pll_div_table { div: 3, freq: 400 * MHZ },
    mtk_pll_div_table { div: 4, freq: 200 * MHZ },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static DSPPLL_DIV_TABLE: &[mtk_pll_div_table] = &[
    mtk_pll_div_table { div: 0, freq: MT8365_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1600 * MHZ },
    mtk_pll_div_table { div: 2, freq: 600 * MHZ },
    mtk_pll_div_table { div: 3, freq: 400 * MHZ },
    mtk_pll_div_table { div: 4, freq: 200 * MHZ },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static PLLS: &[mtk_pll_data] = &[
    pll_b!(CLK_APMIXED_ARMPLL, "armpll", 0x030C, 0x0318, 0x00000001, PLL_AO, 22, 0x0310, 24, 0, 0, 0, 0x0310, 0, ARMPLL_DIV_TABLE, 0, 0),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x0228, 0x0234, 0xFF000001, HAVE_RST_BAR, 22, 0x022C, 24, 0, 0, 0, 0x022C, 0, CON0_MT8365_RST_BAR, 0),
    pll!(CLK_APMIXED_UNIVPLL, "univpll2", 0x0208, 0x0214, 0xFF000001, HAVE_RST_BAR, 22, 0x020C, 24, 0, 0, 0, 0x020C, 0, CON0_MT8365_RST_BAR, 0),
    pll_b!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0218, 0x0224, 1, 0, 22, 0x021C, 24, 0, 0, 0, 0x021C, 0, MFGPLL_DIV_TABLE, 0, 0),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0350, 0x035C, 1, 0, 22, 0x0354, 24, 0, 0, 0, 0x0354, 0, 0, 0),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0330, 0x033C, 1, 0, 22, 0x0334, 24, 0, 0, 0, 0x0334, 0, 0, 0),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x031C, 0x032C, 1, 0, 32, 0x0320, 24, 0x0040, 0x000C, 0, 0x0324, 0, 0, 0x0320),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x0360, 0x0370, 1, 0, 32, 0x0364, 24, 0x004C, 0x000C, 5, 0x0368, 0, 0, 0x0364),
    pll!(CLK_APMIXED_LVDSPLL, "lvdspll", 0x0374, 0x0380, 1, 0, 22, 0x0378, 24, 0, 0, 0, 0x0378, 0, 0, 0),
    pll_b!(CLK_APMIXED_DSPPLL, "dsppll", 0x0390, 0x039C, 1, 0, 22, 0x0394, 24, 0, 0, 0, 0x0394, 0, DSPPLL_DIV_TABLE, 0, 0),
    pll!(CLK_APMIXED_APUPLL, "apupll", 0x03A0, 0x03AC, 1, 0, 22, 0x03A4, 24, 0, 0, 0, 0x03A4, 0, 0, 0),
];

unsafe fn clk_mt8365_apmixed_probe(pdev: *mut platform_device) -> c_int {
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let clk_data = mtk_devm_alloc_clk_data((*pdev).dev, CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }
    let hw = devm_clk_hw_register_gate((*pdev).dev, "univ_en", "univpll2", 0, base.add(0x204), 0, 0, core::ptr::null_mut());
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[CLK_APMIXED_UNIV_EN] = hw;
    let hw = devm_clk_hw_register_gate((*pdev).dev, "usb20_en", "univ_en", 0, base.add(0x204), 1, 0, core::ptr::null_mut());
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[CLK_APMIXED_USB20_EN] = hw;
    let mut ret = mtk_clk_register_plls((*pdev).dev, PLLS.as_ptr(), PLLS.len(), clk_data);
    if ret != 0 { return ret; }
    ret = of_clk_add_hw_provider((*pdev).dev.of_node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 { mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data); }
    ret
}

static OF_MATCH_CLK_MT8365_APMIXED: &[of_device_id] = &[
    of_device_id { compatible: "mediatek,mt8365-apmixedsys" },
    of_device_id { compatible: core::ptr::null() }, // sentinel
];

static mut CLK_MT8365_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8365_apmixed_probe),
    driver: device_driver { name: "clk-mt8365-apmixed", of_match_table: OF_MATCH_CLK_MT8365_APMIXED },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8365_apmixed)
// builtin_platform_driver(clk_mt8365_apmixed_drv)
// MODULE_DESCRIPTION("MediaTek MT8365 apmixedsys clocks driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
