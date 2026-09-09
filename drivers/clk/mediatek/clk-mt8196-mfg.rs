// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// External Linux kernel and MediaTek clock dependencies are supplied by other files.

const MFGPLL_CON0: u32 = 0x008;
const MFGPLL_CON1: u32 = 0x00c;
const MFGPLL_CON2: u32 = 0x010;
const MFGPLL_CON3: u32 = 0x014;
const MFGPLL_SC0_CON0: u32 = 0x008;
const MFGPLL_SC0_CON1: u32 = 0x00c;
const MFGPLL_SC0_CON2: u32 = 0x010;
const MFGPLL_SC0_CON3: u32 = 0x014;
const MFGPLL_SC1_CON0: u32 = 0x008;
const MFGPLL_SC1_CON1: u32 = 0x00c;
const MFGPLL_SC1_CON2: u32 = 0x010;
const MFGPLL_SC1_CON3: u32 = 0x014;

const MT8196_PLL_FMAX: usize = 3800 * MHZ;
const MT8196_PLL_FMIN: usize = 1500 * MHZ;
const MT8196_INTEGER_BITS: u32 = 8;

const fn pll(
    id: u32, name: &'static str, reg: u32, en_reg: u32, en_mask: u32,
    pll_en_bit: u32, flags: u32, rst_bar_mask: u32, pd_reg: u32,
    pd_shift: u32, tuner_reg: u32, tuner_en_reg: u32, tuner_en_bit: u32,
    pcw_reg: u32, pcw_shift: u32, pcwbits: u32,
) -> mtk_pll_data {
    mtk_pll_data {
        id, name, reg, en_reg, en_mask, pll_en_bit, flags, rst_bar_mask,
        fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN, pd_reg, pd_shift,
        tuner_reg, tuner_en_reg, tuner_en_bit, pcw_reg, pcw_shift, pcwbits,
        pcwibits: MT8196_INTEGER_BITS, parent_name: "mfg_eb",
    }
}

static mfg_ao_plls: [mtk_pll_data; 1] = [pll(
    CLK_MFG_AO_MFGPLL, "mfgpll", MFGPLL_CON0, MFGPLL_CON0, 0, 0,
    PLL_PARENT_EN, BIT(0), MFGPLL_CON1, 24, 0, 0, 0, MFGPLL_CON1, 0, 22,
)];

static mfgsc0_ao_plls: [mtk_pll_data; 1] = [pll(
    CLK_MFGSC0_AO_MFGPLL_SC0, "mfgpll-sc0", MFGPLL_SC0_CON0,
    MFGPLL_SC0_CON0, 0, 0, PLL_PARENT_EN, BIT(0), MFGPLL_SC0_CON1, 24,
    0, 0, 0, MFGPLL_SC0_CON1, 0, 22,
)];

static mfgsc1_ao_plls: [mtk_pll_data; 1] = [pll(
    CLK_MFGSC1_AO_MFGPLL_SC1, "mfgpll-sc1", MFGPLL_SC1_CON0,
    MFGPLL_SC1_CON0, 0, 0, PLL_PARENT_EN, BIT(0), MFGPLL_SC1_CON1, 24,
    0, 0, 0, MFGPLL_SC1_CON1, 0, 22,
)];

static of_match_clk_mt8196_mfg: [of_device_id; 4] = [
    of_device_id { compatible: "mediatek,mt8196-mfgpll-pll-ctrl", data: &mfg_ao_plls },
    of_device_id { compatible: "mediatek,mt8196-mfgpll-sc0-pll-ctrl", data: &mfgsc0_ao_plls },
    of_device_id { compatible: "mediatek,mt8196-mfgpll-sc1-pll-ctrl", data: &mfgsc1_ao_plls },
    of_device_id { /* sentinel */ ..Default::default() },
];

unsafe fn clk_mt8196_mfg_probe(pdev: *mut platform_device) -> i32 {
    let plls: *const mtk_pll_data;
    let clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let num_plls: i32 = 1;
    let mut r: i32;

    plls = of_device_get_match_data(&(*pdev).dev);
    if plls.is_null() { return -EINVAL; }

    clk_data = mtk_alloc_clk_data(num_plls);
    if clk_data.is_null() { return -ENOMEM; }

    r = mtk_clk_register_plls(&mut (*pdev).dev, plls, num_plls, clk_data);
    if r != 0 { goto_free_clk_data; }

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { goto_unregister_plls; }

    platform_set_drvdata(pdev, clk_data);
    return r;

    goto_unregister_plls: {
        mtk_clk_unregister_plls(plls, num_plls, clk_data);
    }
    goto_free_clk_data: {
        mtk_free_clk_data(clk_data);
    }
    r
}

unsafe fn clk_mt8196_mfg_remove(pdev: *mut platform_device) {
    let plls = of_device_get_match_data(&(*pdev).dev);
    let clk_data = platform_get_drvdata(pdev);
    let node = (*pdev).dev.of_node;

    of_clk_del_provider(node);
    mtk_clk_unregister_plls(plls, 1, clk_data);
    mtk_free_clk_data(clk_data);
}

static mut clk_mt8196_mfg_drv: platform_driver = platform_driver {
    probe: Some(clk_mt8196_mfg_probe),
    remove: Some(clk_mt8196_mfg_remove),
    driver: driver {
        name: "clk-mt8196-mfg",
        of_match_table: &of_match_clk_mt8196_mfg,
    },
};

module_platform_driver!(clk_mt8196_mfg_drv);

// MODULE_DESCRIPTION("MediaTek MT8196 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
