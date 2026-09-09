// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// Dependencies supplied by the kernel clock framework and MediaTek clock code:
// dt-bindings/clock/mediatek,mt8365-clk.h, linux/clk-provider.h,
// linux/platform_device.h, clk-gate.h, and clk-mtk.h.

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

static CAM_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent to the C GATE_CAM macro; GATE_MTK is provided by the
// MediaTek clock dependency.
macro_rules! gate_cam {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &CAM_CG_REGS, $shift,
                  &mtk_clk_gate_ops_setclr)
    };
}

static CAM_CLKS: [MtkGate; 8] = [
    gate_cam!(CLK_CAM_LARB2, "cam_larb2", "mm_sel", 0),
    gate_cam!(CLK_CAM, "cam", "mm_sel", 6),
    gate_cam!(CLK_CAMTG, "camtg", "mm_sel", 7),
    gate_cam!(CLK_CAM_SENIF, "cam_senif", "mm_sel", 8),
    gate_cam!(CLK_CAMSV0, "camsv0", "mm_sel", 9),
    gate_cam!(CLK_CAMSV1, "camsv1", "mm_sel", 10),
    gate_cam!(CLK_CAM_FDVT, "cam_fdvt", "mm_sel", 11),
    gate_cam!(CLK_CAM_WPE, "cam_wpe", "mm_sel", 12),
];

static CAM_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_CLKS.as_ptr(),
    num_clks: CAM_CLKS.len(),
};

static OF_MATCH_CLK_MT8365_CAM: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8365-imgsys",
        data: &CAM_DESC,
    },
    OfDeviceId::sentinel(),
];

module_device_table!(of, OF_MATCH_CLK_MT8365_CAM);

static mut CLK_MT8365_CAM_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8365-cam",
        of_match_table: OF_MATCH_CLK_MT8365_CAM.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8365_CAM_DRV);

module_description!("MediaTek MT8365 Camera clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
