// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and MT8183 clock-binding interfaces.

static CAM_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_cam {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &CAM_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static CAM_CLKS: [MtkGate; 10] = [
    gate_cam!(CLK_CAM_LARB6, "cam_larb6", "cam_sel", 0),
    gate_cam!(CLK_CAM_DFP_VAD, "cam_dfp_vad", "cam_sel", 1),
    gate_cam!(CLK_CAM_LARB3, "cam_larb3", "cam_sel", 2),
    gate_cam!(CLK_CAM_CAM, "cam_cam", "cam_sel", 6),
    gate_cam!(CLK_CAM_CAMTG, "cam_camtg", "cam_sel", 7),
    gate_cam!(CLK_CAM_SENINF, "cam_seninf", "cam_sel", 8),
    gate_cam!(CLK_CAM_CAMSV0, "cam_camsv0", "cam_sel", 9),
    gate_cam!(CLK_CAM_CAMSV1, "cam_camsv1", "cam_sel", 10),
    gate_cam!(CLK_CAM_CAMSV2, "cam_camsv2", "cam_sel", 11),
    gate_cam!(CLK_CAM_CCU, "cam_ccu", "cam_sel", 12),
];

static CAM_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_CLKS.as_ptr(),
    num_clks: CAM_CLKS.len(),
};

static OF_MATCH_CLK_MT8183_CAM: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8183-camsys",
        data: &CAM_DESC,
    },
    OfDeviceId {
        // sentinel
        ..OfDeviceId::empty()
    },
];

static mut CLK_MT8183_CAM_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8183-cam",
        of_match_table: OF_MATCH_CLK_MT8183_CAM.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8183_CAM_DRV);

module_description!("MediaTek MT8183 Camera clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
