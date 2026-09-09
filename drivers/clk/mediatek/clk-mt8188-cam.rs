// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the corresponding Linux clock/platform headers.

static CAM_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_cam {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &CAM_CG_REGS, $shift, &MTK_CLK_GATE_OPS_SETCLR)
    };
}

const CAM_SYS_SMI_LARB_RST_OFF: u16 = 0xA0;

static CAM_MAIN_CLKS: [MtkGate; 24] = [
    gate_cam!(CLK_CAM_MAIN_LARB13, "cam_main_larb13", "top_cam", 0),
    gate_cam!(CLK_CAM_MAIN_LARB14, "cam_main_larb14", "top_cam", 1),
    gate_cam!(CLK_CAM_MAIN_CAM, "cam_main_cam", "top_cam", 2),
    gate_cam!(CLK_CAM_MAIN_CAM_SUBA, "cam_main_cam_suba", "top_cam", 3),
    gate_cam!(CLK_CAM_MAIN_CAM_SUBB, "cam_main_cam_subb", "top_cam", 4),
    gate_cam!(CLK_CAM_MAIN_CAMTG, "cam_main_camtg", "top_cam", 7),
    gate_cam!(CLK_CAM_MAIN_SENINF, "cam_main_seninf", "top_cam", 8),
    gate_cam!(CLK_CAM_MAIN_GCAMSVA, "cam_main_gcamsva", "top_cam", 9),
    gate_cam!(CLK_CAM_MAIN_GCAMSVB, "cam_main_gcamsvb", "top_cam", 10),
    gate_cam!(CLK_CAM_MAIN_GCAMSVC, "cam_main_gcamsvc", "top_cam", 11),
    gate_cam!(CLK_CAM_MAIN_GCAMSV D, "cam_main_gcamsvd", "top_cam", 12),
    gate_cam!(CLK_CAM_MAIN_GCAMSV E, "cam_main_gcamsve", "top_cam", 13),
    gate_cam!(CLK_CAM_MAIN_GCAMSV F, "cam_main_gcamsvf", "top_cam", 14),
    gate_cam!(CLK_CAM_MAIN_GCAMSV G, "cam_main_gcamsvg", "top_cam", 15),
    gate_cam!(CLK_CAM_MAIN_GCAMSV H, "cam_main_gcamsvh", "top_cam", 16),
    gate_cam!(CLK_CAM_MAIN_GCAMSV I, "cam_main_gcamsvi", "top_cam", 17),
    gate_cam!(CLK_CAM_MAIN_GCAMSV J, "cam_main_gcamsvj", "top_cam", 18),
    gate_cam!(CLK_CAM_MAIN_CAMSV_TOP, "cam_main_camsv", "top_cam", 19),
    gate_cam!(CLK_CAM_MAIN_CAMSV_CQ_A, "cam_main_camsv_cq_a", "top_cam", 20),
    gate_cam!(CLK_CAM_MAIN_CAMSV_CQ_B, "cam_main_camsv_cq_b", "top_cam", 21),
    gate_cam!(CLK_CAM_MAIN_CAMSV_CQ_C, "cam_main_camsv_cq_c", "top_cam", 22),
    gate_cam!(CLK_CAM_MAIN_FAKE_ENG, "cam_main_fake_eng", "top_cam", 28),
    gate_cam!(CLK_CAM_MAIN_CAM2MM0_GALS, "cam_main_cam2mm0_gals", "top_cam", 29),
    gate_cam!(CLK_CAM_MAIN_CAM2MM1_GALS, "cam_main_cam2mm1_gals", "top_cam", 30),
    gate_cam!(CLK_CAM_MAIN_CAM2SYS_GALS, "cam_main_cam2sys_gals", "top_cam", 31),
];

static CAM_RAWA_CLKS: [MtkGate; 3] = [
    gate_cam!(CLK_CAM_RAWA_LARBX, "cam_rawa_larbx", "top_cam", 0),
    gate_cam!(CLK_CAM_RAWA_CAM, "cam_rawa_cam", "top_cam", 1),
    gate_cam!(CLK_CAM_RAWA_CAMTG, "cam_rawa_camtg", "top_cam", 2),
];

static CAM_RAWB_CLKS: [MtkGate; 3] = [
    gate_cam!(CLK_CAM_RAWB_LARBX, "cam_rawb_larbx", "top_cam", 0),
    gate_cam!(CLK_CAM_RAWB_CAM, "cam_rawb_cam", "top_cam", 1),
    gate_cam!(CLK_CAM_RAWB_CAMTG, "cam_rawb_camtg", "top_cam", 2),
];

static CAM_YUVA_CLKS: [MtkGate; 3] = [
    gate_cam!(CLK_CAM_YUVA_LARBX, "cam_yuva_larbx", "top_cam", 0),
    gate_cam!(CLK_CAM_YUVA_CAM, "cam_yuva_cam", "top_cam", 1),
    gate_cam!(CLK_CAM_YUVA_CAMTG, "cam_yuva_camtg", "top_cam", 2),
];

static CAM_YUVB_CLKS: [MtkGate; 3] = [
    gate_cam!(CLK_CAM_YUVB_LARBX, "cam_yuvb_larbx", "top_cam", 0),
    gate_cam!(CLK_CAM_YUVB_CAM, "cam_yuvb_cam", "top_cam", 1),
    gate_cam!(CLK_CAM_YUVB_CAMTG, "cam_yuvb_camtg", "top_cam", 2),
];

/* Reset for SMI larb 16a/16b/17a/17b */
static mut CAM_SYS_RST_OFS: [u16; 1] = [CAM_SYS_SMI_LARB_RST_OFF];

static CAM_SYS_RST_DESC: MtkClkRstDesc = MtkClkRstDesc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: CAM_SYS_RST_OFS.as_ptr(),
    rst_bank_nr: CAM_SYS_RST_OFS.len(),
};

static CAM_MAIN_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_MAIN_CLKS.as_ptr(),
    num_clks: CAM_MAIN_CLKS.len(),
    rst_desc: core::ptr::null(),
};

static CAM_RAWA_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_RAWA_CLKS.as_ptr(),
    num_clks: CAM_RAWA_CLKS.len(),
    rst_desc: &CAM_SYS_RST_DESC,
};

static CAM_RAWB_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_RAWB_CLKS.as_ptr(),
    num_clks: CAM_RAWB_CLKS.len(),
    rst_desc: &CAM_SYS_RST_DESC,
};

static CAM_YUVA_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_YUVA_CLKS.as_ptr(),
    num_clks: CAM_YUVA_CLKS.len(),
    rst_desc: &CAM_SYS_RST_DESC,
};

static CAM_YUVB_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_YUVB_CLKS.as_ptr(),
    num_clks: CAM_YUVB_CLKS.len(),
    rst_desc: &CAM_SYS_RST_DESC,
};

static OF_MATCH_CLK_MT8188_CAM: [OfDeviceId; 6] = [
    OfDeviceId { compatible: "mediatek,mt8188-camsys", data: &CAM_MAIN_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-camsys-rawa", data: &CAM_RAWA_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-camsys-rawb", data: &CAM_RAWB_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-camsys-yuva", data: &CAM_YUVA_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-camsys-yuvb", data: &CAM_YUVB_DESC },
    OfDeviceId::sentinel(),
];

static mut CLK_MT8188_CAM_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: Driver {
        name: "clk-mt8188-cam",
        of_match_table: OF_MATCH_CLK_MT8188_CAM.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8188_CAM_DRV);

module_description!("MediaTek MT8188 Camera clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
