// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel clock and platform code are
// intentionally referenced here rather than reimplemented.

static VDEC0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static VDEC1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x200,
    clr_ofs: 0x204,
    sta_ofs: 0x200,
};

static VDEC2_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

macro_rules! gate_vdec0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! gate_vdec1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! gate_vdec2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC2_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

static VDEC_CLKS: [mtk_gate; 5] = [
    // VDEC0
    gate_vdec0!(CLK_VDEC_VDEC, "vdec_vdec", "vdec_sel", 0),
    gate_vdec0!(CLK_VDEC_ACTIVE, "vdec_active", "vdec_sel", 4),
    // VDEC1
    gate_vdec1!(CLK_VDEC_LAT, "vdec_lat", "vdec_sel", 0),
    gate_vdec1!(CLK_VDEC_LAT_ACTIVE, "vdec_lat_active", "vdec_sel", 4),
    // VDEC2
    gate_vdec2!(CLK_VDEC_LARB1, "vdec_larb1", "vdec_sel", 0),
];

static VDEC_SOC_CLKS: [mtk_gate; 5] = [
    // VDEC_SOC0
    gate_vdec0!(CLK_VDEC_SOC_VDEC, "vdec_soc_vdec", "vdec_sel", 0),
    gate_vdec0!(CLK_VDEC_SOC_VDEC_ACTIVE, "vdec_soc_vdec_active", "vdec_sel", 4),
    // VDEC_SOC1
    gate_vdec1!(CLK_VDEC_SOC_LAT, "vdec_soc_lat", "vdec_sel", 0),
    gate_vdec1!(CLK_VDEC_SOC_LAT_ACTIVE, "vdec_soc_lat_active", "vdec_sel", 4),
    // VDEC_SOC2
    gate_vdec2!(CLK_VDEC_SOC_LARB1, "vdec_soc_larb1", "vdec_sel", 0),
];

static VDEC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &VDEC_CLKS,
    num_clks: VDEC_CLKS.len(),
};

static VDEC_SOC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &VDEC_SOC_CLKS,
    num_clks: VDEC_SOC_CLKS.len(),
};

static OF_MATCH_CLK_MT8192_VDEC: [of_device_id; 3] = [
    of_device_id {
        compatible: "mediatek,mt8192-vdecsys",
        data: &VDEC_DESC,
    },
    of_device_id {
        compatible: "mediatek,mt8192-vdecsys_soc",
        data: &VDEC_SOC_DESC,
    },
    of_device_id {
        // sentinel
    },
];

static mut CLK_MT8192_VDEC_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8192-vdec",
        of_match_table: &OF_MATCH_CLK_MT8192_VDEC,
    },
};

module_platform_driver!(CLK_MT8192_VDEC_DRV);

module_description!("MediaTek MT8192 Video Decoders clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
