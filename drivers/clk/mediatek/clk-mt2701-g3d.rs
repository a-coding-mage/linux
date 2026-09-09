// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Sean Wang <sean.wang@mediatek.com>
 *
 */

// Translated from the Linux kernel implementation.  The referenced clock,
// platform, and device-tree definitions are supplied by external dependencies.

macro_rules! GATE_G3D {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &g3d_cg_regs, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static G3D_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    sta_ofs: 0x0,
    set_ofs: 0x4,
    clr_ofs: 0x8,
};

static G3D_CLKS: [mtk_gate; 2] = [
    GATE_DUMMY!(CLK_DUMMY, "g3d_dummy"),
    GATE_G3D!(CLK_G3DSYS_CORE, "g3d_core", "mfg_sel", 0),
];

static mut RST_OFS: [u16; 1] = [0xc];

static CLK_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: RST_OFS.as_ptr(),
    rst_bank_nr: RST_OFS.len(),
};

static G3D_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: G3D_CLKS.as_ptr(),
    num_clks: G3D_CLKS.len(),
    rst_desc: &CLK_RST_DESC,
};

static OF_MATCH_CLK_MT2701_G3D: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2701-g3dsys",
        data: &G3D_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT2701_G3D);

static mut CLK_MT2701_G3D_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt2701-g3d",
        of_match_table: OF_MATCH_CLK_MT2701_G3D.as_ptr(),
    },
};

module_platform_driver!(CLK_MT2701_G3D_DRV);

MODULE_DESCRIPTION!("MediaTek MT2701 GPU g3d clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
