// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding clock framework and generated
// clock bindings are intentionally left as external Rust items/macros.

static CCU_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_CCU {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &CCU_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static CCU_CLKS: [mtk_gate; 4] = [
    GATE_CCU!(CLK_CCU_LARB18, "ccu_larb18", "top_ccu", 0),
    GATE_CCU!(CLK_CCU_AHB, "ccu_ahb", "top_ccu", 1),
    GATE_CCU!(CLK_CCU_CCU0, "ccu_ccu0", "top_ccu", 2),
    GATE_CCU!(CLK_CCU_CCU1, "ccu_ccu1", "top_ccu", 3),
];

static CCU_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: CCU_CLKS.as_ptr(),
    num_clks: CCU_CLKS.len(),
};

static OF_MATCH_CLK_MT8195_CCU: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8195-ccusys",
        data: &CCU_DESC,
    },
    of_device_id {
        // sentinel
        ..of_device_id::default()
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8195_CCU);

static mut CLK_MT8195_CCU_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8195-ccu",
        of_match_table: OF_MATCH_CLK_MT8195_CCU.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8195_CCU_DRV);

MODULE_DESCRIPTION!("MediaTek MT8195 Camera Control Unit clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
