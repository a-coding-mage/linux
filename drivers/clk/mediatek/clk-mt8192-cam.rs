// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device, MediaTek
// clock, clock-gate, and MT8192 clock binding interfaces.

static CAM_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// C macro translation:
// GATE_CAM(_id, _name, _parent, _shift) expands to GATE_MTK(
//     _id, _name, _parent, &cam_cg_regs, _shift, &mtk_clk_gate_ops_setclr).

static CAM_CLKS: [mtk_gate; 16] = [
    GATE_MTK(CLK_CAM_LARB13, "cam_larb13", "cam_sel", &CAM_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_DFP_VAD, "cam_dfp_vad", "cam_sel", &CAM_CG_REGS, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_LARB14, "cam_larb14", "cam_sel", &CAM_CG_REGS, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAM, "cam_cam", "cam_sel", &CAM_CG_REGS, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAMTG, "cam_camtg", "cam_sel", &CAM_CG_REGS, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_SENINF, "cam_seninf", "cam_sel", &CAM_CG_REGS, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAMSV0, "cam_camsv0", "cam_sel", &CAM_CG_REGS, 9, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAMSV1, "cam_camsv1", "cam_sel", &CAM_CG_REGS, 10, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAMSV2, "cam_camsv2", "cam_sel", &CAM_CG_REGS, 11, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAMSV3, "cam_camsv3", "cam_sel", &CAM_CG_REGS, 12, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CCU0, "cam_ccu0", "cam_sel", &CAM_CG_REGS, 13, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CCU1, "cam_ccu1", "cam_sel", &CAM_CG_REGS, 14, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_MRAW0, "cam_mraw0", "cam_sel", &CAM_CG_REGS, 15, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_FAKE_ENG, "cam_fake_eng", "cam_sel", &CAM_CG_REGS, 17, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CCU_GALS, "cam_ccu_gals", "cam_sel", &CAM_CG_REGS, 18, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_CAM2MM_GALS, "cam2mm_gals", "cam_sel", &CAM_CG_REGS, 19, &mtk_clk_gate_ops_setclr),
];

static CAM_RAWA_CLKS: [mtk_gate; 3] = [
    GATE_MTK(CLK_CAM_RAWA_LARBX, "cam_rawa_larbx", "cam_sel", &CAM_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWA_CAM, "cam_rawa_cam", "cam_sel", &CAM_CG_REGS, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWA_CAMTG, "cam_rawa_camtg", "cam_sel", &CAM_CG_REGS, 2, &mtk_clk_gate_ops_setclr),
];

static CAM_RAWB_CLKS: [mtk_gate; 3] = [
    GATE_MTK(CLK_CAM_RAWB_LARBX, "cam_rawb_larbx", "cam_sel", &CAM_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWB_CAM, "cam_rawb_cam", "cam_sel", &CAM_CG_REGS, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWB_CAMTG, "cam_rawb_camtg", "cam_sel", &CAM_CG_REGS, 2, &mtk_clk_gate_ops_setclr),
];

static CAM_RAWC_CLKS: [mtk_gate; 3] = [
    GATE_MTK(CLK_CAM_RAWC_LARBX, "cam_rawc_larbx", "cam_sel", &CAM_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWC_CAM, "cam_rawc_cam", "cam_sel", &CAM_CG_REGS, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_CAM_RAWC_CAMTG, "cam_rawc_camtg", "cam_sel", &CAM_CG_REGS, 2, &mtk_clk_gate_ops_setclr),
];

static CAM_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_CLKS, num_clks: ARRAY_SIZE(CAM_CLKS) };
static CAM_RAWA_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_RAWA_CLKS, num_clks: ARRAY_SIZE(CAM_RAWA_CLKS) };
static CAM_RAWB_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_RAWB_CLKS, num_clks: ARRAY_SIZE(CAM_RAWB_CLKS) };
static CAM_RAWC_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_RAWC_CLKS, num_clks: ARRAY_SIZE(CAM_RAWC_CLKS) };

static OF_MATCH_CLK_MT8192_CAM: [of_device_id; 5] = [
    of_device_id { compatible: "mediatek,mt8192-camsys", data: &CAM_DESC },
    of_device_id { compatible: "mediatek,mt8192-camsys_rawa", data: &CAM_RAWA_DESC },
    of_device_id { compatible: "mediatek,mt8192-camsys_rawb", data: &CAM_RAWB_DESC },
    of_device_id { compatible: "mediatek,mt8192-camsys_rawc", data: &CAM_RAWC_DESC },
    of_device_id::sentinel(),
];

static mut CLK_MT8192_CAM_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8192-cam",
        of_match_table: &OF_MATCH_CLK_MT8192_CAM,
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_cam);
// module_platform_driver(clk_mt8192_cam_drv);
// MODULE_DESCRIPTION("MediaTek MT8192 Camera clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
