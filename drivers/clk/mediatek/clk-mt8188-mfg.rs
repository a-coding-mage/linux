// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock implementation:
// dt-bindings/clock/mediatek,mt8188-clk.h
// linux/clk-provider.h, linux/platform_device.h
// clk-gate.h, clk-mtk.h

static MFGCFG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

const fn gate_mfg(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    GATE_MTK_FLAGS(
        id,
        name,
        parent,
        &MFGCFG_CG_REGS,
        shift,
        &mtk_clk_gate_ops_setclr,
        CLK_SET_RATE_PARENT,
    )
}

static MFGCFG_CLKS: [mtk_gate; 1] = [
    gate_mfg(CLK_MFGCFG_BG3D, "mfgcfg_bg3d", "mfg_ck_fast_ref", 0),
];

static MFGCFG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MFGCFG_CLKS.as_ptr(),
    num_clks: MFGCFG_CLKS.len(),
};

static OF_MATCH_CLK_MT8188_MFGCFG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8188-mfgcfg",
        data: &MFGCFG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8188_MFGCFG);

static mut CLK_MT8188_MFGCFG_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8188-mfgcfg",
        of_match_table: OF_MATCH_CLK_MT8188_MFGCFG.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8188_MFGCFG_DRV);

MODULE_DESCRIPTION!("MediaTek MT8186 GPU mfg clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
