// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies:
// linux/clk-provider.h, linux/platform_device.h, dt-bindings/clock/mt8186-clk.h,
// clk-gate.h, and clk-mtk.h are supplied by the surrounding tree.

static IPE_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_IPE {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &IPE_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static IPE_CLKS: [mtk_gate; 8] = [
    GATE_IPE!(CLK_IPE_LARB19, "ipe_larb19", "top_ipe", 0),
    GATE_IPE!(CLK_IPE_LARB20, "ipe_larb20", "top_ipe", 1),
    GATE_IPE!(CLK_IPE_SMI_SUBCOM, "ipe_smi_subcom", "top_ipe", 2),
    GATE_IPE!(CLK_IPE_FD, "ipe_fd", "top_ipe", 3),
    GATE_IPE!(CLK_IPE_FE, "ipe_fe", "top_ipe", 4),
    GATE_IPE!(CLK_IPE_RSC, "ipe_rsc", "top_ipe", 5),
    GATE_IPE!(CLK_IPE_DPE, "ipe_dpe", "top_ipe", 6),
    GATE_IPE!(CLK_IPE_GALS_IPE, "ipe_gals_ipe", "top_img1", 8),
];

static IPE_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IPE_CLKS,
    num_clks: IPE_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_IPE: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8186-ipesys",
        data: &IPE_DESC,
    },
    of_device_id {
        // sentinel
    },
];

module_device_table!(of, OF_MATCH_CLK_MT8186_IPE);

static mut CLK_MT8186_IPE_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8186-ipe",
        of_match_table: &OF_MATCH_CLK_MT8186_IPE,
    },
};

module_platform_driver!(CLK_MT8186_IPE_DRV);

module_description!("MediaTek MT8186 Image Processing Engine clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
