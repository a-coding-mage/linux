// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by clk-gate.h, clk-mtk.h, and the Linux clock/platform
// headers are intentionally referenced here rather than reimplemented.

extern "C" {
    static mtk_clk_gate_ops_setclr_inv: mtk_clk_gate_ops;
    fn mtk_clk_simple_probe() -> i32;
    fn mtk_clk_simple_remove() -> i32;
}

static vdec0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static vdec1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x200,
    clr_ofs: 0x204,
    sta_ofs: 0x200,
};

static vdec2_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

macro_rules! GATE_VDEC0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec0_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec1_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec2_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

static vdec_clks: [mtk_gate; 3] = [
    // VDEC0
    GATE_VDEC0!(CLK_VDEC_VDEC, "vdec_vdec", "top_vdec", 0),
    // VDEC1
    GATE_VDEC1!(CLK_VDEC_LAT, "vdec_lat", "top_vdec", 0),
    // VDEC2
    GATE_VDEC2!(CLK_VDEC_LARB1, "vdec_larb1", "top_vdec", 0),
];

static vdec_core1_clks: [mtk_gate; 3] = [
    // VDEC0
    GATE_VDEC0!(CLK_VDEC_CORE1_VDEC, "vdec_core1_vdec", "top_vdec", 0),
    // VDEC1
    GATE_VDEC1!(CLK_VDEC_CORE1_LAT, "vdec_core1_lat", "top_vdec", 0),
    // VDEC2
    GATE_VDEC2!(CLK_VDEC_CORE1_LARB1, "vdec_core1_larb1", "top_vdec", 0),
];

static vdec_soc_clks: [mtk_gate; 3] = [
    // VDEC0
    GATE_VDEC0!(CLK_VDEC_SOC_VDEC, "vdec_soc_vdec", "top_vdec", 0),
    // VDEC1
    GATE_VDEC1!(CLK_VDEC_SOC_LAT, "vdec_soc_lat", "top_vdec", 0),
    // VDEC2
    GATE_VDEC2!(CLK_VDEC_SOC_LARB1, "vdec_soc_larb1", "top_vdec", 0),
];

static vdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: vdec_clks.as_ptr(),
    num_clks: vdec_clks.len(),
};

static vdec_core1_desc: mtk_clk_desc = mtk_clk_desc {
    clks: vdec_core1_clks.as_ptr(),
    num_clks: vdec_core1_clks.len(),
};

static vdec_soc_desc: mtk_clk_desc = mtk_clk_desc {
    clks: vdec_soc_clks.as_ptr(),
    num_clks: vdec_soc_clks.len(),
};

static of_match_clk_mt8195_vdec: [of_device_id; 4] = [
    of_device_id {
        compatible: "mediatek,mt8195-vdecsys",
        data: &vdec_desc,
    },
    of_device_id {
        compatible: "mediatek,mt8195-vdecsys_core1",
        data: &vdec_core1_desc,
    },
    of_device_id {
        compatible: "mediatek,mt8195-vdecsys_soc",
        data: &vdec_soc_desc,
    },
    // sentinel
    of_device_id::default(),
];

static mut clk_mt8195_vdec_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8195-vdec",
        of_match_table: of_match_clk_mt8195_vdec.as_ptr(),
    },
};

// module_platform_driver(clk_mt8195_vdec_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_vdec);
// MODULE_DESCRIPTION("MediaTek MT8195 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
