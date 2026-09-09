// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and MT8183 clock-binding interfaces are referenced
// below and are not defined in this translation unit.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// C macro equivalent of GATE_IMG(_id, _name, _parent, _shift).
macro_rules! gate_img {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &IMG_CG_REGS, $shift,
            &mtk_clk_gate_ops_setclr)
    };
}

static IMG_CLKS: [mtk_gate; 10] = [
    gate_img!(CLK_IMG_LARB5, "img_larb5", "img_sel", 0),
    gate_img!(CLK_IMG_LARB2, "img_larb2", "img_sel", 1),
    gate_img!(CLK_IMG_DIP, "img_dip", "img_sel", 2),
    gate_img!(CLK_IMG_FDVT, "img_fdvt", "img_sel", 3),
    gate_img!(CLK_IMG_DPE, "img_dpe", "img_sel", 4),
    gate_img!(CLK_IMG_RSC, "img_rsc", "img_sel", 5),
    gate_img!(CLK_IMG_MFB, "img_mfb", "img_sel", 6),
    gate_img!(CLK_IMG_WPE_A, "img_wpe_a", "img_sel", 7),
    gate_img!(CLK_IMG_WPE_B, "img_wpe_b", "img_sel", 8),
    gate_img!(CLK_IMG_OWE, "img_owe", "img_sel", 9),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: IMG_CLKS.as_ptr(),
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT8183_IMG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8183-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        // sentinel
        compatible: "",
        data: core::ptr::null(),
    },
];

module_device_table!(of, OF_MATCH_CLK_MT8183_IMG);

static mut CLK_MT8183_IMG_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8183-img",
        of_match_table: OF_MATCH_CLK_MT8183_IMG.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8183_IMG_DRV);

module_description!("MediaTek MT8183 imgsys clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
