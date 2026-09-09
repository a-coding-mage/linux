// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 *                    Weiyi Lu <weiyi.lu@mediatek.com>
 * Copyright (c) 2023 Collabora Ltd.
 *                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel clock-driver environment.

const MT2712_PLL_FMAX: u64 = 3000u64 * MHZ;
const CON0_MT2712_RST_BAR: u32 = BIT(24);

const fn pll_b(
    id: u32, name: &'static str, reg: u32, pwr_reg: u32, en_mask: u32,
    flags: u32, pcwbits: u32, pd_reg: u32, pd_shift: u32, tuner_reg: u32,
    tuner_en_reg: u32, tuner_en_bit: u32, pcw_reg: u32, pcw_shift: u32,
    div_table: *const mtk_pll_div_table,
) -> mtk_pll_data {
    mtk_pll_data { id, name, reg, pwr_reg, en_mask, flags,
        rst_bar_mask: CON0_MT2712_RST_BAR, fmax: MT2712_PLL_FMAX,
        pcwbits, pd_reg, pd_shift, tuner_reg, tuner_en_reg, tuner_en_bit,
        pcw_reg, pcw_shift, div_table }
}

const fn pll(
    id: u32, name: &'static str, reg: u32, pwr_reg: u32, en_mask: u32,
    flags: u32, pcwbits: u32, pd_reg: u32, pd_shift: u32, tuner_reg: u32,
    tuner_en_reg: u32, tuner_en_bit: u32, pcw_reg: u32, pcw_shift: u32,
) -> mtk_pll_data {
    pll_b(id, name, reg, pwr_reg, en_mask, flags, pcwbits, pd_reg, pd_shift,
          tuner_reg, tuner_en_reg, tuner_en_bit, pcw_reg, pcw_shift,
          core::ptr::null())
}

static ARMCA35PLL_DIV_TABLE: [mtk_pll_div_table; 6] = [
    mtk_pll_div_table { div: 0, freq: MT2712_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1202500000 },
    mtk_pll_div_table { div: 2, freq: 500500000 },
    mtk_pll_div_table { div: 3, freq: 315250000 },
    mtk_pll_div_table { div: 4, freq: 157625000 },
    mtk_pll_div_table { div: 0, freq: 0 },
];
static ARMCA72PLL_DIV_TABLE: [mtk_pll_div_table; 6] = [
    mtk_pll_div_table { div: 0, freq: MT2712_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 994500000 },
    mtk_pll_div_table { div: 2, freq: 520000000 },
    mtk_pll_div_table { div: 3, freq: 315250000 },
    mtk_pll_div_table { div: 4, freq: 157625000 },
    mtk_pll_div_table { div: 0, freq: 0 },
];
static MMPLL_DIV_TABLE: [mtk_pll_div_table; 6] = [
    mtk_pll_div_table { div: 0, freq: MT2712_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1001000000 },
    mtk_pll_div_table { div: 2, freq: 601250000 },
    mtk_pll_div_table { div: 3, freq: 250250000 },
    mtk_pll_div_table { div: 4, freq: 125125000 },
    mtk_pll_div_table { div: 0, freq: 0 },
];

static PLLS: [mtk_pll_data; 15] = [
    pll(CLK_APMIXED_MAINPLL, "mainpll", 0x0230, 0x023c, 0xf0000100, HAVE_RST_BAR, 31, 0x0230, 4, 0, 0, 0, 0x0234, 0),
    pll(CLK_APMIXED_UNIVPLL, "univpll", 0x0240, 0x024c, 0xfe000100, HAVE_RST_BAR, 31, 0x0240, 4, 0, 0, 0, 0x0244, 0),
    pll(CLK_APMIXED_VCODECPLL, "vcodecpll", 0x0320, 0x032c, 0xc0000100, 0, 31, 0x0320, 4, 0, 0, 0, 0x0324, 0),
    pll(CLK_APMIXED_VENCPLL, "vencpll", 0x0280, 0x028c, 0x00000100, 0, 31, 0x0280, 4, 0, 0, 0, 0x0284, 0),
    pll(CLK_APMIXED_APLL1, "apll1", 0x0330, 0x0340, 0x100, 0, 31, 0x0330, 4, 0x0338, 0x0014, 0, 0x0334, 0),
    pll(CLK_APMIXED_APLL2, "apll2", 0x0350, 0x0360, 0x100, 0, 31, 0x0350, 4, 0x0358, 0x0014, 1, 0x0354, 0),
    pll(CLK_APMIXED_LVDSPLL, "lvdspll", 0x0370, 0x037c, 0x100, 0, 31, 0x0370, 4, 0, 0, 0, 0x0374, 0),
    pll(CLK_APMIXED_LVDSPLL2, "lvdspll2", 0x0390, 0x039c, 0x100, 0, 31, 0x0390, 4, 0, 0, 0, 0x0394, 0),
    pll(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0270, 0x027c, 0x100, 0, 31, 0x0270, 4, 0, 0, 0, 0x0274, 0),
    pll(CLK_APMIXED_MSDCPLL2, "msdcpll2", 0x0410, 0x041c, 0x100, 0, 31, 0x0410, 4, 0, 0, 0, 0x0414, 0),
    pll(CLK_APMIXED_TVDPLL, "tvdpll", 0x0290, 0x029c, 0xc0000100, 0, 31, 0x0290, 4, 0, 0, 0, 0x0294, 0),
    pll_b(CLK_APMIXED_MMPLL, "mmpll", 0x0250, 0x0260, 0x100, 0, 31, 0x0250, 4, 0, 0, 0, 0x0254, 0, MMPLL_DIV_TABLE.as_ptr()),
    pll_b(CLK_APMIXED_ARMCA35PLL, "armca35pll", 0x0100, 0x0110, 0xf0000100, HAVE_RST_BAR, 31, 0x0100, 4, 0, 0, 0, 0x0104, 0, ARMCA35PLL_DIV_TABLE.as_ptr()),
    pll_b(CLK_APMIXED_ARMCA72PLL, "armca72pll", 0x0210, 0x0220, 0x100, 0, 31, 0x0210, 4, 0, 0, 0, 0x0214, 0, ARMCA72PLL_DIV_TABLE.as_ptr()),
    pll(CLK_APMIXED_ETHERPLL, "etherpll", 0x0300, 0x030c, 0xc0000100, 0, 31, 0x0300, 4, 0, 0, 0, 0x0304, 0),
];

unsafe fn clk_mt2712_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let mut r: i32;
    let node = (*pdev).dev.of_node;
    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }
    r = mtk_clk_register_plls(&mut (*pdev).dev, PLLS.as_ptr(), PLLS.len(), clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        dev_err(&mut (*pdev).dev, "Cannot register clock provider: %d\n", r);
        mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
        mtk_free_clk_data(clk_data);
    }
    r
}

unsafe fn clk_mt2712_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static OF_MATCH_CLK_MT2712_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt2712-apmixedsys" },
    of_device_id { compatible: core::option::Option::None },
];

static mut CLK_MT2712_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt2712_apmixed_probe),
    remove: Some(clk_mt2712_apmixed_remove),
    driver: driver {
        name: "clk-mt2712-apmixed",
        of_match_table: OF_MATCH_CLK_MT2712_APMIXED.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2712_apmixed)
// module_platform_driver(clk_mt2712_apmixed_drv)
// MODULE_DESCRIPTION("MediaTek MT2712 apmixedsys clocks driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
