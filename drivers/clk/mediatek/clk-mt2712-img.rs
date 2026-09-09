// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Dependency declarations corresponding to:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt2712-clk.h.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

macro_rules! gate_img {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &IMG_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

static IMG_CLKS: [mtk_gate; 6] = [
    gate_img!(CLK_IMG_SMI_LARB2, "img_smi_larb2", "mm_sel", 0),
    gate_img!(CLK_IMG_SENINF_SCAM_EN, "img_scam_en", "csi0", 3),
    gate_img!(CLK_IMG_SENINF_CAM_EN, "img_cam_en", "mm_sel", 8),
    gate_img!(CLK_IMG_CAM_SV_EN, "img_cam_sv_en", "mm_sel", 9),
    gate_img!(CLK_IMG_CAM_SV1_EN, "img_cam_sv1_en", "mm_sel", 10),
    gate_img!(CLK_IMG_CAM_SV2_EN, "img_cam_sv2_en", "mm_sel", 11),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG_CLKS,
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT2712_IMG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2712-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT2712_IMG);

static mut CLK_MT2712_IMG_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt2712-img",
        of_match_table: &OF_MATCH_CLK_MT2712_IMG,
    },
};

module_platform_driver!(CLK_MT2712_IMG_DRV);

MODULE_DESCRIPTION!("MediaTek MT2712 imgsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
