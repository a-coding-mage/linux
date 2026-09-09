// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel translation.

static venc_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_VENC {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &venc_cg_regs, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

static venc_clks: [mtk_gate; 4] = [
    GATE_VENC!(CLK_VENC_SET0_LARB, "venc_set0_larb", "venc_sel", 0),
    GATE_VENC!(CLK_VENC_SET1_VENC, "venc_set1_venc", "venc_sel", 4),
    GATE_VENC!(CLK_VENC_SET2_JPGENC, "venc_set2_jpgenc", "venc_sel", 8),
    GATE_VENC!(CLK_VENC_SET5_GALS, "venc_set5_gals", "venc_sel", 28),
];

static venc_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &venc_clks,
    num_clks: venc_clks.len(),
};

static of_match_clk_mt8192_venc: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8192-vencsys",
        data: &venc_desc,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8192_venc);

static mut clk_mt8192_venc_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: device_driver {
        name: "clk-mt8192-venc",
        of_match_table: &of_match_clk_mt8192_venc,
    },
};

module_platform_driver!(clk_mt8192_venc_drv);

MODULE_DESCRIPTION!("MediaTek MT8192 Video Encoders clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
