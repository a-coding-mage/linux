// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

// Dependency declarations are supplied by the surrounding kernel translation.

static IMG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent of the C GATE_IMG(_id, _name, _parent, _shift) macro.
macro_rules! gate_img {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &IMG_CG_REGS, $shift, &MTK_CLK_GATE_OPS_SETCLR)
    };
}

static IMG_CLKS: [MtkGate; 6] = [
    gate_img!(CLK_IMG_LARB1_SMI, b"img_larb1_smi\0", b"smi_mm\0", 0),
    gate_img!(CLK_IMG_CAM_SMI, b"img_cam_smi\0", b"smi_mm\0", 5),
    gate_img!(CLK_IMG_CAM_CAM, b"img_cam_cam\0", b"smi_mm\0", 6),
    gate_img!(CLK_IMG_SEN_TG, b"img_sen_tg\0", b"cam_mm\0", 7),
    gate_img!(CLK_IMG_SEN_CAM, b"img_sen_cam\0", b"smi_mm\0", 8),
    gate_img!(CLK_IMG_VENC, b"img_venc\0", b"smi_mm\0", 9),
];

static IMG_DESC: MtkClkDesc = MtkClkDesc {
    clks: &IMG_CLKS,
    num_clks: array_size(&IMG_CLKS),
};

static OF_MATCH_CLK_MT8167_IMGSYS: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8167-imgsys\0",
        data: &IMG_DESC,
    },
    OfDeviceId::sentinel(),
];

static mut CLK_MT8167_IMGSYS_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: DeviceDriver {
        name: b"clk-mt8167-imgsys\0",
        of_match_table: &OF_MATCH_CLK_MT8167_IMGSYS,
    },
};

module_platform_driver!(CLK_MT8167_IMGSYS_DRV);

module_description!("MediaTek MT8167 imgsys clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
