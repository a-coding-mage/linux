// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// C dependencies: clk-mtk.h, clk-pll.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

const MT8195_PLL_FMAX: u64 = 3800u64 * MHZ;
const MT8195_PLL_FMIN: u64 = 1500u64 * MHZ;
const MT8195_INTEGER_BITS: u32 = 8;
const MT8195_PCW_BITS: u32 = 22;
const MT8195_POSDIV_SHIFT: u32 = 24;
const MT8195_PLL_EN_BIT: u32 = 0;
const MT8195_PCW_SHIFT: u32 = 0;

// The "en_reg" and "pcw_chg_reg" fields are standard offset registers
// compared with "reg", so zero implies them. There is no tuner control in
// the APU PLL, so the tuner fields are zero. There is no reset or post-divider
// enable in the APU PLL, so "rst_bar_mask" and "en_mask" are zero.

const fn pll(
    id: u32,
    name: &'static [u8],
    reg: u32,
    pwr_reg: u32,
    pd_reg: u32,
    pcw_reg: u32,
) -> mtk_pll_data {
    mtk_pll_data {
        id,
        name,
        reg,
        pwr_reg,
        en_mask: 0,
        flags: 0,
        rst_bar_mask: 0,
        fmax: MT8195_PLL_FMAX,
        fmin: MT8195_PLL_FMIN,
        pcwbits: MT8195_PCW_BITS,
        pcwibits: MT8195_INTEGER_BITS,
        pd_reg,
        pd_shift: MT8195_POSDIV_SHIFT,
        tuner_reg: 0,
        tuner_en_reg: 0,
        tuner_en_bit: 0,
        pcw_reg,
        pcw_shift: MT8195_PCW_SHIFT,
        pcw_chg_reg: 0,
        en_reg: 0,
        pll_en_bit: MT8195_PLL_EN_BIT,
    }
}

static APUSYS_PLLS: [mtk_pll_data; 4] = [
    pll(CLK_APUSYS_PLL_APUPLL, b"apusys_pll_apupll\0", 0x008, 0x014, 0x00c, 0x00c),
    pll(CLK_APUSYS_PLL_NPUPLL, b"apusys_pll_npupll\0", 0x018, 0x024, 0x01c, 0x01c),
    pll(CLK_APUSYS_PLL_APUPLL1, b"apusys_pll_apupll1\0", 0x028, 0x034, 0x02c, 0x02c),
    pll(CLK_APUSYS_PLL_APUPLL2, b"apusys_pll_apupll2\0", 0x038, 0x044, 0x03c, 0x03c),
];

unsafe fn clk_mt8195_apusys_pll_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node: *mut device_node = (*pdev).dev.of_node;
    let mut r: i32;

    clk_data = mtk_alloc_clk_data(CLK_APUSYS_PLL_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    r = mtk_clk_register_plls(
        &mut (*pdev).dev,
        APUSYS_PLLS.as_ptr(),
        APUSYS_PLLS.len(),
        clk_data,
    );
    if r != 0 {
        mtk_free_clk_data(clk_data);
        return r;
    }

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        mtk_clk_unregister_plls(APUSYS_PLLS.as_ptr(), APUSYS_PLLS.len(), clk_data);
        mtk_free_clk_data(clk_data);
        return r;
    }

    platform_set_drvdata(pdev, clk_data);
    r
}

unsafe fn clk_mt8195_apusys_pll_remove(pdev: *mut platform_device) {
    let clk_data: *mut clk_hw_onecell_data = platform_get_drvdata(pdev);
    let node: *mut device_node = (*pdev).dev.of_node;

    of_clk_del_provider(node);
    mtk_clk_unregister_plls(APUSYS_PLLS.as_ptr(), APUSYS_PLLS.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static OF_MATCH_CLK_MT8195_APUSYS_PLL: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mt8195-apusys_pll\0" },
    of_device_id { compatible: core::ptr::null() },
];

static mut CLK_MT8195_APUSYS_PLL_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8195_apusys_pll_probe),
    remove: Some(clk_mt8195_apusys_pll_remove),
    driver: driver {
        name: b"clk-mt8195-apusys_pll\0",
        of_match_table: OF_MATCH_CLK_MT8195_APUSYS_PLL.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_apusys_pll);
// module_platform_driver(clk_mt8195_apusys_pll_drv);
// MODULE_DESCRIPTION("MediaTek MT8195 AI Processing Unit PLL clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
