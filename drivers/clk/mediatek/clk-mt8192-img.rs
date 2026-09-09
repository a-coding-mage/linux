// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt8192-clk.h.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

#[inline]
const fn gate_img(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    // Equivalent of GATE_MTK(_id, _name, _parent, &img_cg_regs, _shift,
    // &mtk_clk_gate_ops_setclr).
    GATE_MTK(id, name, parent, &IMG_CG_REGS, shift, &mtk_clk_gate_ops_setclr)
}

static IMG_CLKS: [mtk_gate; 4] = [
    gate_img(CLK_IMG_LARB9, "img_larb9", "img1_sel", 0),
    gate_img(CLK_IMG_LARB10, "img_larb10", "img1_sel", 1),
    gate_img(CLK_IMG_DIP, "img_dip", "img1_sel", 2),
    gate_img(CLK_IMG_GALS, "img_gals", "img1_sel", 12),
];

static IMG2_CLKS: [mtk_gate; 6] = [
    gate_img(CLK_IMG2_LARB11, "img2_larb11", "img1_sel", 0),
    gate_img(CLK_IMG2_LARB12, "img2_larb12", "img1_sel", 1),
    gate_img(CLK_IMG2_MFB, "img2_mfb", "img1_sel", 6),
    gate_img(CLK_IMG2_WPE, "img2_wpe", "img1_sel", 7),
    gate_img(CLK_IMG2_MSS, "img2_mss", "img1_sel", 8),
    gate_img(CLK_IMG2_GALS, "img2_gals", "img1_sel", 12),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG_CLKS,
    num_clks: IMG_CLKS.len(),
};

static IMG2_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG2_CLKS,
    num_clks: IMG2_CLKS.len(),
};

static OF_MATCH_CLK_MT8192_IMG: [of_device_id; 3] = [
    of_device_id {
        compatible: "mediatek,mt8192-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        compatible: "mediatek,mt8192-imgsys2",
        data: &IMG2_DESC,
    },
    of_device_id {
        // sentinel
    },
];

// Equivalent of MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_img).

static mut CLK_MT8192_IMG_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8192-img",
        of_match_table: &OF_MATCH_CLK_MT8192_IMG,
    },
};

// Equivalent of module_platform_driver(clk_mt8192_img_drv).
// MODULE_DESCRIPTION("MediaTek MT8192 imgsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
