// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the Linux clock and platform headers are intentionally
// left as external Rust items/macros.

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

macro_rules! GATE_VDEC0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC2_CG_REGS, $shift, &mtk_clk_gate_ops_setclr_inv)
    };
}

static VDEC1_CLKS: [mtk_gate; 7] = [
    // VDEC1_0
    GATE_VDEC0!(CLK_VDEC1_SOC_VDEC, "vdec1_soc_vdec", "top_vdec", 0),
    GATE_VDEC0!(CLK_VDEC1_SOC_VDEC_ACTIVE, "vdec1_soc_vdec_active", "top_vdec", 4),
    GATE_VDEC0!(CLK_VDEC1_SOC_VDEC_ENG, "vdec1_soc_vdec_eng", "top_vdec", 8),
    // VDEC1_1
    GATE_VDEC1!(CLK_VDEC1_SOC_LAT, "vdec1_soc_lat", "top_vdec", 0),
    GATE_VDEC1!(CLK_VDEC1_SOC_LAT_ACTIVE, "vdec1_soc_lat_active", "top_vdec", 4),
    GATE_VDEC1!(CLK_VDEC1_SOC_LAT_ENG, "vdec1_soc_lat_eng", "top_vdec", 8),
    // VDEC1_2
    GATE_VDEC2!(CLK_VDEC1_SOC_LARB1, "vdec1_soc_larb1", "top_vdec", 0),
];

static VDEC2_CLKS: [mtk_gate; 5] = [
    // VDEC2_0
    GATE_VDEC0!(CLK_VDEC2_VDEC, "vdec2_vdec", "top_vdec", 0),
    GATE_VDEC0!(CLK_VDEC2_VDEC_ACTIVE, "vdec2_vdec_active", "top_vdec", 4),
    GATE_VDEC0!(CLK_VDEC2_VDEC_ENG, "vdec2_vdec_eng", "top_vdec", 8),
    // VDEC2_1
    GATE_VDEC1!(CLK_VDEC2_LAT, "vdec2_lat", "top_vdec", 0),
    // VDEC2_2
    GATE_VDEC2!(CLK_VDEC2_LARB1, "vdec2_larb1", "top_vdec", 0),
];

static VDEC1_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC1_CLKS.as_ptr(),
    num_clks: VDEC1_CLKS.len(),
};

static VDEC2_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC2_CLKS.as_ptr(),
    num_clks: VDEC2_CLKS.len(),
};

static OF_MATCH_CLK_MT8188_VDEC: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt8188-vdecsys-soc", data: &VDEC1_DESC },
    of_device_id { compatible: "mediatek,mt8188-vdecsys", data: &VDEC2_DESC },
    of_device_id { /* sentinel */ },
];

static mut CLK_MT8188_VDEC_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8188-vdec",
        of_match_table: OF_MATCH_CLK_MT8188_VDEC.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8188_VDEC_DRV);

module_description!("MediaTek MT8188 Video Decoders clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
