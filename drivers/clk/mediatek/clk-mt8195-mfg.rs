// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the kernel clock and platform implementations:
// clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

static mfg_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_MFG {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK_FLAGS!(
            $id,
            $name,
            $parent,
            &mfg_cg_regs,
            $shift,
            &mtk_clk_gate_ops_setclr,
            CLK_SET_RATE_PARENT
        )
    };
}

static mfg_clks: [mtk_gate; 1] = [
    GATE_MFG!(CLK_MFG_BG3D, "mfg_bg3d", "mfg_ck_fast_ref", 0),
];

static mfg_desc: mtk_clk_desc = mtk_clk_desc {
    clks: mfg_clks.as_ptr(),
    num_clks: array_size!(mfg_clks),
};

static of_match_clk_mt8195_mfg: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8195-mfgcfg",
        data: &mfg_desc,
    },
    of_device_id {
        // sentinel
    },
];

module_device_table!(of, of_match_clk_mt8195_mfg);

static mut clk_mt8195_mfg_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: device_driver {
        name: "clk-mt8195-mfg",
        of_match_table: of_match_clk_mt8195_mfg.as_ptr(),
    },
};

module_platform_driver!(clk_mt8195_mfg_drv);

module_description!("MediaTek MT8195 GPU mfg clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
