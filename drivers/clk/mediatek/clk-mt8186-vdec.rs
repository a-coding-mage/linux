// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies:
// linux/clk-provider.h, linux/module.h, linux/platform_device.h,
// clk-mtk.h, clk-gate.h, and dt-bindings/clock/mt8186-clk.h

static vdec0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static vdec1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x190,
    clr_ofs: 0x190,
    sta_ofs: 0x190,
};

static vdec2_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x200,
    clr_ofs: 0x204,
    sta_ofs: 0x200,
};

static vdec3_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

macro_rules! GATE_VDEC0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec0_cg_regs, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec1_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! GATE_VDEC2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec2_cg_regs, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC3 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec3_cg_regs, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

static vdec_clks: [mtk_gate; 8] = [
    // VDEC0
    GATE_VDEC0!(CLK_VDEC_CKEN, "vdec_cken", "top_vdec", 0),
    GATE_VDEC0!(CLK_VDEC_ACTIVE, "vdec_active", "top_vdec", 4),
    GATE_VDEC0!(CLK_VDEC_CKEN_ENG, "vdec_cken_eng", "top_vdec", 8),
    // VDEC1
    GATE_VDEC1!(CLK_VDEC_MINI_MDP_CKEN_CFG_RG, "vdec_mini_mdp_cken_cfg_rg", "top_vdec", 0),
    // VDEC2
    GATE_VDEC2!(CLK_VDEC_LAT_CKEN, "vdec_lat_cken", "top_vdec", 0),
    GATE_VDEC2!(CLK_VDEC_LAT_ACTIVE, "vdec_lat_active", "top_vdec", 4),
    GATE_VDEC2!(CLK_VDEC_LAT_CKEN_ENG, "vdec_lat_cken_eng", "top_vdec", 8),
    // VDEC3
    GATE_VDEC3!(CLK_VDEC_LARB1_CKEN, "vdec_larb1_cken", "top_vdec", 0),
];

static vdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &vdec_clks,
    num_clks: vdec_clks.len(),
};

static of_match_clk_mt8186_vdec: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8186-vdecsys",
        data: &vdec_desc,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8186_vdec);

static mut clk_mt8186_vdec_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8186-vdec",
        of_match_table: &of_match_clk_mt8186_vdec,
    },
};

module_platform_driver!(clk_mt8186_vdec_drv);

MODULE_DESCRIPTION!("MediaTek MT8186 Video Decoders clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
