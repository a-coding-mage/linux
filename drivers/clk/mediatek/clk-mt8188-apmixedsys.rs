// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock framework.

static AP_MIXED_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0x8,
    sta_ofs: 0x8,
};

macro_rules! gate_apmixed {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &AP_MIXED_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static APMIXED_CLKS: [mtk_gate; 1] = [
    gate_apmixed!(CLK_APMIXED_PLL_SSUSB26M_EN, "pll_ssusb26m_en", "clk26m", 1),
];

const MT8188_PLL_FMAX: u32 = 3800u32 * MHZ;
const MT8188_PLL_FMIN: u32 = 1500u32 * MHZ;
const MT8188_INTEGER_BITS: u32 = 8;

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $rst_bar_mask:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $pcw_chg_reg:expr,
     $en_reg:expr, $pll_en_bit:expr) => {
        mtk_pll_data {
            id: $id,
            name: $name,
            reg: $reg,
            pwr_reg: $pwr_reg,
            en_mask: $en_mask,
            flags: $flags,
            rst_bar_mask: $rst_bar_mask,
            fmax: MT8188_PLL_FMAX,
            fmin: MT8188_PLL_FMIN,
            pcwbits: $pcwbits,
            pcwibits: MT8188_INTEGER_BITS,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            tuner_reg: $tuner_reg,
            tuner_en_reg: $tuner_en_reg,
            tuner_en_bit: $tuner_en_bit,
            pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift,
            pcw_chg_reg: $pcw_chg_reg,
            en_reg: $en_reg,
            pll_en_bit: $pll_en_bit,
        }
    };
}

static PLLS: [mtk_pll_data; 15] = [
    pll!(CLK_APMIXED_ETHPLL, "ethpll", 0x044C, 0x0458, 0, 0, 0, 22, 0x0450, 24, 0, 0, 0, 0x0450, 0, 0, 0, 9),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0514, 0x0520, 0, 0, 0, 22, 0x0518, 24, 0, 0, 0, 0x0518, 0, 0, 0, 9),
    pll!(CLK_APMIXED_TVDPLL1, "tvdpll1", 0x0524, 0x0530, 0, 0, 0, 22, 0x0528, 24, 0, 0, 0, 0x0528, 0, 0, 0, 9),
    pll!(CLK_APMIXED_TVDPLL2, "tvdpll2", 0x0534, 0x0540, 0, 0, 0, 22, 0x0538, 24, 0, 0, 0, 0x0538, 0, 0, 0, 9),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0544, 0x0550, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0548, 24, 0, 0, 0, 0x0548, 0, 0, 0, 9),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x045C, 0x0468, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0460, 24, 0, 0, 0, 0x0460, 0, 0, 0, 9),
    pll!(CLK_APMIXED_IMGPLL, "imgpll", 0x0554, 0x0560, 0, 0, 0, 22, 0x0558, 24, 0, 0, 0, 0x0558, 0, 0, 0, 9),
    pll!(CLK_APMIXED_UNIVPLL, "univpll", 0x0504, 0x0510, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0508, 24, 0, 0, 0, 0x0508, 0, 0, 0, 9),
    pll!(CLK_APMIXED_ADSPPLL, "adsppll", 0x042C, 0x0438, 0, 0, 0, 22, 0x0430, 24, 0, 0, 0, 0x0430, 0, 0, 0, 9),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x0304, 0x0314, 0, 0, 0, 32, 0x0308, 24, 0x0034, 0x0000, 12, 0x030C, 0, 0, 0, 9),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x0318, 0x0328, 0, 0, 0, 32, 0x031C, 24, 0x0038, 0x0000, 13, 0x0320, 0, 0, 0, 9),
    pll!(CLK_APMIXED_APLL3, "apll3", 0x032C, 0x033C, 0, 0, 0, 32, 0x0330, 24, 0x003C, 0x0000, 14, 0x0334, 0, 0, 0, 9),
    pll!(CLK_APMIXED_APLL4, "apll4", 0x0404, 0x0414, 0, 0, 0, 32, 0x0408, 24, 0x0040, 0x0000, 15, 0x040C, 0, 0, 0, 9),
    pll!(CLK_APMIXED_APLL5, "apll5", 0x0418, 0x0428, 0, 0, 0, 32, 0x041C, 24, 0x0044, 0x0000, 16, 0x0420, 0, 0, 0, 9),
    pll!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0340, 0x034C, 0, 0, 0, 22, 0x0344, 24, 0, 0, 0, 0x0344, 0, 0, 0, 9),
];

static OF_MATCH_CLK_MT8188_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8188-apmixedsys" },
    of_device_id { /* sentinel */ },
];

unsafe fn clk_mt8188_apmixed_probe(pdev: *mut platform_device) -> c_int {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let mut r: c_int;

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }

    r = mtk_clk_register_plls(&mut (*pdev).dev, PLLS.as_ptr(), PLLS.len(), clk_data);
    if r != 0 { goto!(free_apmixed_data); }
    r = mtk_clk_register_gates(&mut (*pdev).dev, node, APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    if r != 0 { goto!(unregister_plls); }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { goto!(unregister_gates); }
    platform_set_drvdata(pdev, clk_data);
    return 0;

    unregister_gates: mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    unregister_plls: mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
    free_apmixed_data: mtk_free_clk_data(clk_data);
    r
}

unsafe fn clk_mt8188_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8188_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8188_apmixed_probe),
    remove: Some(clk_mt8188_apmixed_remove),
    driver: device_driver {
        name: "clk-mt8188-apmixed",
        of_match_table: OF_MATCH_CLK_MT8188_APMIXED.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8188_APMIXED_DRV);

// MODULE_DESCRIPTION("MediaTek MT8188 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
