// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

// Equivalent of:
// GATE_IMG(_id, _name, _parent, _shift) \
//     GATE_MTK(_id, _name, _parent, &img_cg_regs, _shift, &mtk_clk_gate_ops_setclr)
macro_rules! GATE_IMG {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &IMG_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static IMG_CLKS: [mtk_gate; 8] = [
    GATE_DUMMY!(CLK_DUMMY, "img_dummy"),
    GATE_IMG!(CLK_IMG_LARB2_SMI, "img_larb2_smi", "mm_sel", 0),
    GATE_IMG!(CLK_IMG_CAM_SMI, "img_cam_smi", "mm_sel", 5),
    GATE_IMG!(CLK_IMG_CAM_CAM, "img_cam_cam", "mm_sel", 6),
    GATE_IMG!(CLK_IMG_SEN_TG, "img_sen_tg", "camtg_sel", 7),
    GATE_IMG!(CLK_IMG_SEN_CAM, "img_sen_cam", "mm_sel", 8),
    GATE_IMG!(CLK_IMG_CAM_SV, "img_cam_sv", "mm_sel", 9),
    GATE_IMG!(CLK_IMG_FD, "img_fd", "mm_sel", 11),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG_CLKS,
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT8173_IMGSYS: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8173-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8173_IMGSYS);

static mut CLK_MT8173_VDECSYS_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8173-imgsys",
        of_match_table: OF_MATCH_CLK_MT8173_IMGSYS,
    },
};

module_platform_driver!(CLK_MT8173_VDECSYS_DRV);

MODULE_DESCRIPTION!("MediaTek MT8173 vdecsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
