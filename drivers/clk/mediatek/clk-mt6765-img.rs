// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Linux clock-provider, platform-device, MediaTek clock, clock-gate, and
// device-tree declarations are supplied by external dependencies.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_IMG {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &IMG_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static IMG_CLKS: [mtk_gate; 5] = [
    GATE_IMG!(CLK_IMG_LARB2, "img_larb2", "mm_ck", 0),
    GATE_IMG!(CLK_IMG_DIP, "img_dip", "mm_ck", 2),
    GATE_IMG!(CLK_IMG_FDVT, "img_fdvt", "mm_ck", 3),
    GATE_IMG!(CLK_IMG_DPE, "img_dpe", "mm_ck", 4),
    GATE_IMG!(CLK_IMG_RSC, "img_rsc", "mm_ck", 5),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG_CLKS,
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_IMG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6765-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT6765_IMG);

static mut CLK_MT6765_IMG_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt6765-img",
        of_match_table: &OF_MATCH_CLK_MT6765_IMG,
    },
};

module_platform_driver!(CLK_MT6765_IMG_DRV);

MODULE_DESCRIPTION!("MediaTek MT6765 imgsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
