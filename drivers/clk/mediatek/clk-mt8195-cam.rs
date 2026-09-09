// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding clock-driver implementation:
// clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

static const cam_cg_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_CAM {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &cam_cg_regs, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static const cam_clks: [MtkGate; 22] = [
    GATE_CAM!(CLK_CAM_LARB13, "cam_larb13", "top_cam", 0),
    GATE_CAM!(CLK_CAM_LARB14, "cam_larb14", "top_cam", 1),
    GATE_CAM!(CLK_CAM_MAIN_CAM, "cam_main_cam", "top_cam", 3),
    GATE_CAM!(CLK_CAM_MAIN_CAMTG, "cam_main_camtg", "top_cam", 4),
    GATE_CAM!(CLK_CAM_SENINF, "cam_seninf", "top_cam", 5),
    GATE_CAM!(CLK_CAM_GCAMSVA, "cam_gcamsva", "top_cam", 6),
    GATE_CAM!(CLK_CAM_GCAMSVB, "cam_gcamsvb", "top_cam", 7),
    GATE_CAM!(CLK_CAM_GCAMSVC, "cam_gcamsvc", "top_cam", 8),
    GATE_CAM!(CLK_CAM_SCAMSA, "cam_scamsa", "top_cam", 9),
    GATE_CAM!(CLK_CAM_SCAMSB, "cam_scamsb", "top_cam", 10),
    GATE_CAM!(CLK_CAM_CAMSV_TOP, "cam_camsv_top", "top_cam", 11),
    GATE_CAM!(CLK_CAM_CAMSV_CQ, "cam_camsv_cq", "top_cam", 12),
    GATE_CAM!(CLK_CAM_ADL, "cam_adl", "top_cam", 16),
    GATE_CAM!(CLK_CAM_ASG, "cam_asg", "top_cam", 17),
    GATE_CAM!(CLK_CAM_PDA, "cam_pda", "top_cam", 18),
    GATE_CAM!(CLK_CAM_FAKE_ENG, "cam_fake_eng", "top_cam", 19),
    GATE_CAM!(CLK_CAM_MAIN_MRAW0, "cam_main_mraw0", "top_cam", 20),
    GATE_CAM!(CLK_CAM_MAIN_MRAW1, "cam_main_mraw1", "top_cam", 21),
    GATE_CAM!(CLK_CAM_MAIN_MRAW2, "cam_main_mraw2", "top_cam", 22),
    GATE_CAM!(CLK_CAM_MAIN_MRAW3, "cam_main_mraw3", "top_cam", 23),
    GATE_CAM!(CLK_CAM_CAM2MM0_GALS, "cam_cam2mm0_gals", "top_cam", 24),
    GATE_CAM!(CLK_CAM_CAM2MM1_GALS, "cam_cam2mm1_gals", "top_cam", 25),
    GATE_CAM!(CLK_CAM_CAM2SYS_GALS, "cam_cam2sys_gals", "top_cam", 26),
];

static const cam_mraw_clks: [MtkGate; 6] = [
    GATE_CAM!(CLK_CAM_MRAW_LARBX, "cam_mraw_larbx", "top_cam", 0),
    GATE_CAM!(CLK_CAM_MRAW_CAMTG, "cam_mraw_camtg", "top_cam", 2),
    GATE_CAM!(CLK_CAM_MRAW_MRAW0, "cam_mraw_mraw0", "top_cam", 3),
    GATE_CAM!(CLK_CAM_MRAW_MRAW1, "cam_mraw_mraw1", "top_cam", 4),
    GATE_CAM!(CLK_CAM_MRAW_MRAW2, "cam_mraw_mraw2", "top_cam", 5),
    GATE_CAM!(CLK_CAM_MRAW_MRAW3, "cam_mraw_mraw3", "top_cam", 6),
];

static const cam_rawa_clks: [MtkGate; 3] = [
    GATE_CAM!(CLK_CAM_RAWA_LARBX, "cam_rawa_larbx", "top_cam", 0),
    GATE_CAM!(CLK_CAM_RAWA_CAM, "cam_rawa_cam", "top_cam", 1),
    GATE_CAM!(CLK_CAM_RAWA_CAMTG, "cam_rawa_camtg", "top_cam", 2),
];

static const cam_rawb_clks: [MtkGate; 3] = [
    GATE_CAM!(CLK_CAM_RAWB_LARBX, "cam_rawb_larbx", "top_cam", 0),
    GATE_CAM!(CLK_CAM_RAWB_CAM, "cam_rawb_cam", "top_cam", 1),
    GATE_CAM!(CLK_CAM_RAWB_CAMTG, "cam_rawb_camtg", "top_cam", 2),
];

static const cam_yuva_clks: [MtkGate; 3] = [
    GATE_CAM!(CLK_CAM_YUVA_LARBX, "cam_yuva_larbx", "top_cam", 0),
    GATE_CAM!(CLK_CAM_YUVA_CAM, "cam_yuva_cam", "top_cam", 1),
    GATE_CAM!(CLK_CAM_YUVA_CAMTG, "cam_yuva_camtg", "top_cam", 2),
];

static const cam_yuvb_clks: [MtkGate; 3] = [
    GATE_CAM!(CLK_CAM_YUVB_LARBX, "cam_yuvb_larbx", "top_cam", 0),
    GATE_CAM!(CLK_CAM_YUVB_CAM, "cam_yuvb_cam", "top_cam", 1),
    GATE_CAM!(CLK_CAM_YUVB_CAMTG, "cam_yuvb_camtg", "top_cam", 2),
];

static const cam_desc: MtkClkDesc = MtkClkDesc { clks: &cam_clks, num_clks: cam_clks.len() };
static const cam_mraw_desc: MtkClkDesc = MtkClkDesc { clks: &cam_mraw_clks, num_clks: cam_mraw_clks.len() };
static const cam_rawa_desc: MtkClkDesc = MtkClkDesc { clks: &cam_rawa_clks, num_clks: cam_rawa_clks.len() };
static const cam_rawb_desc: MtkClkDesc = MtkClkDesc { clks: &cam_rawb_clks, num_clks: cam_rawb_clks.len() };
static const cam_yuva_desc: MtkClkDesc = MtkClkDesc { clks: &cam_yuva_clks, num_clks: cam_yuva_clks.len() };
static const cam_yuvb_desc: MtkClkDesc = MtkClkDesc { clks: &cam_yuvb_clks, num_clks: cam_yuvb_clks.len() };

static const of_match_clk_mt8195_cam: [OfDeviceId; 7] = [
    OfDeviceId { compatible: "mediatek,mt8195-camsys", data: Some(&cam_desc) },
    OfDeviceId { compatible: "mediatek,mt8195-camsys_mraw", data: Some(&cam_mraw_desc) },
    OfDeviceId { compatible: "mediatek,mt8195-camsys_rawa", data: Some(&cam_rawa_desc) },
    OfDeviceId { compatible: "mediatek,mt8195-camsys_rawb", data: Some(&cam_rawb_desc) },
    OfDeviceId { compatible: "mediatek,mt8195-camsys_yuva", data: Some(&cam_yuva_desc) },
    OfDeviceId { compatible: "mediatek,mt8195-camsys_yuvb", data: Some(&cam_yuvb_desc) },
    OfDeviceId::SENTINEL,
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8195_cam);

static mut clk_mt8195_cam_drv: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8195-cam",
        of_match_table: &of_match_clk_mt8195_cam,
    },
};

module_platform_driver!(clk_mt8195_cam_drv);

MODULE_DESCRIPTION!("MediaTek MT8195 Camera clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
