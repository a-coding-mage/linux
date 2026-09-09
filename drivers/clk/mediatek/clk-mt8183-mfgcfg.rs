// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Translated dependencies:
// linux/clk-provider.h, linux/platform_device.h, linux/pm_runtime.h
// clk-mtk.h, clk-gate.h, dt-bindings/clock/mt8183-clk.h

static MFG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_MFG(_id, _name, _parent, _shift)
//     GATE_MTK_FLAGS(_id, _name, _parent, &mfg_cg_regs, _shift,
//                    &mtk_clk_gate_ops_setclr, CLK_SET_RATE_PARENT)

static MFG_CLKS: [mtk_gate; 1] = [
    mtk_gate {
        id: CLK_MFG_BG3D,
        name: "mfg_bg3d",
        parent_name: "mfg_sel",
        regs: &MFG_CG_REGS,
        shift: 0,
        ops: &mtk_clk_gate_ops_setclr,
        flags: CLK_SET_RATE_PARENT,
    },
];

static MFG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &MFG_CLKS,
    num_clks: MFG_CLKS.len(),
    need_runtime_pm: true,
};

static OF_MATCH_CLK_MT8183_MFG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8183-mfgcfg",
        data: &MFG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_mfg);

static mut CLK_MT8183_MFG_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8183-mfg",
        of_match_table: &OF_MATCH_CLK_MT8183_MFG,
    },
};

// module_platform_driver(clk_mt8183_mfg_drv);

// MODULE_DESCRIPTION("MediaTek MT8183 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
