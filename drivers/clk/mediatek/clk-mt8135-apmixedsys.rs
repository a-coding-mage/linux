// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 *               James Liao <jamesjj.liao@mediatek.com>
 * Copyright (c) 2023 Collabora, Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const MT8135_PLL_FMAX: u32 = 2000 * MHZ;
const CON0_MT8135_RST_BAR: u32 = BIT(27);

const fn pll(
    id: u32,
    name: &'static str,
    reg: u32,
    pwr_reg: u32,
    en_mask: u32,
    flags: u32,
    pcwbits: u32,
    pd_reg: u32,
    pd_shift: u32,
    tuner_reg: u32,
    pcw_reg: u32,
    pcw_shift: u32,
) -> mtk_pll_data {
    mtk_pll_data {
        id,
        name,
        reg,
        pwr_reg,
        en_mask,
        flags,
        rst_bar_mask: CON0_MT8135_RST_BAR,
        fmax: MT8135_PLL_FMAX,
        pcwbits,
        pd_reg,
        pd_shift,
        tuner_reg,
        pcw_reg,
        pcw_shift,
    }
}

static PLls: [mtk_pll_data; 10] = [
    pll(CLK_APMIXED_ARMPLL1, "armpll1", 0x200, 0x218, 0x80000000, 0, 21, 0x204, 24, 0x0, 0x204, 0),
    pll(CLK_APMIXED_ARMPLL2, "armpll2", 0x2cc, 0x2e4, 0x80000000, 0, 21, 0x2d0, 24, 0x0, 0x2d0, 0),
    pll(CLK_APMIXED_MAINPLL, "mainpll", 0x21c, 0x234, 0xf0000000, HAVE_RST_BAR, 21, 0x21c, 6, 0x0, 0x220, 0),
    pll(CLK_APMIXED_UNIVPLL, "univpll", 0x238, 0x250, 0xf3000000, HAVE_RST_BAR, 7, 0x238, 6, 0x0, 0x238, 9),
    pll(CLK_APMIXED_MMPLL, "mmpll", 0x254, 0x26c, 0xf0000000, HAVE_RST_BAR, 21, 0x254, 6, 0x0, 0x258, 0),
    pll(CLK_APMIXED_MSDCPLL, "msdcpll", 0x278, 0x290, 0x80000000, 0, 21, 0x278, 6, 0x0, 0x27c, 0),
    pll(CLK_APMIXED_TVDPLL, "tvdpll", 0x294, 0x2ac, 0x80000000, 0, 31, 0x294, 6, 0x0, 0x298, 0),
    pll(CLK_APMIXED_LVDSPLL, "lvdspll", 0x2b0, 0x2c8, 0x80000000, 0, 21, 0x2b0, 6, 0x0, 0x2b4, 0),
    pll(CLK_APMIXED_AUDPLL, "audpll", 0x2e8, 0x300, 0x80000000, 0, 31, 0x2e8, 6, 0x2f8, 0x2ec, 0),
    pll(CLK_APMIXED_VDECPLL, "vdecpll", 0x304, 0x31c, 0x80000000, 0, 21, 0x2b0, 6, 0x0, 0x308, 0),
];

unsafe fn clk_mt8135_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let mut ret: i32;

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    ret = mtk_clk_register_plls(&mut (*pdev).dev, PLls.as_ptr(), PLls.len(), clk_data);
    if ret != 0 {
        mtk_free_clk_data(clk_data);
        return ret;
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        mtk_clk_unregister_plls(PLls.as_ptr(), PLls.len(), clk_data);
        mtk_free_clk_data(clk_data);
    }

    ret
}

unsafe fn clk_mt8135_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);

    of_clk_del_provider(node);
    mtk_clk_unregister_plls(PLls.as_ptr(), PLls.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static OF_MATCH_CLK_MT8135_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8135-apmixedsys" },
    of_device_id { /* sentinel */ ..of_device_id::default() },
];

static mut CLK_MT8135_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8135_apmixed_probe),
    remove: Some(clk_mt8135_apmixed_remove),
    driver: device_driver {
        name: "clk-mt8135-apmixed",
        of_match_table: OF_MATCH_CLK_MT8135_APMIXED.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8135_apmixed);
// module_platform_driver(clk_mt8135_apmixed_drv)
// MODULE_DESCRIPTION("MediaTek MT8135 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
