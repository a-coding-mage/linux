// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 MediaTek Inc.
 * Author: Kevin Chen <kevin-cw.chen@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device,
// dt-bindings, and MediaTek clock headers are referenced below.

static IMG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

static IMG_CLKS: [mtk_gate; 4] = [
    GATE_MTK(CLK_IMG_FDVT, "img_fdvt", "mm_sel", &IMG_CG_REGS, 11, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_DPE, "img_dpe", "mm_sel", &IMG_CG_REGS, 10, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_DIP, "img_dip", "mm_sel", &IMG_CG_REGS, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_LARB6, "img_larb6", "mm_sel", &IMG_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
];

static IMG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &IMG_CLKS,
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT6797_IMG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6797-imgsys",
        data: &IMG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT6797_IMG);

static mut CLK_MT6797_IMG_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt6797-img",
        of_match_table: &OF_MATCH_CLK_MT6797_IMG,
    },
};

module_platform_driver!(CLK_MT6797_IMG_DRV);

MODULE_DESCRIPTION!("MediaTek MT6797 imgsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
