// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Kevin Chen <kevin-cw.chen@mediatek.com>
 */

// Translated from the Linux clock-provider and platform-device interfaces.
// The declarations below are supplied by the surrounding clock framework.

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

macro_rules! gate_venc {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VENC_CG_REGS, $shift, &MTK_CLK_GATE_OPS_SETCLR_INV)
    };
}

static VENC_CLKS: [MtkGate; 4] = [
    gate_venc!(CLK_VENC_0, "venc_0", "mm_sel", 0),
    gate_venc!(CLK_VENC_1, "venc_1", "venc_sel", 4),
    gate_venc!(CLK_VENC_2, "venc_2", "venc_sel", 8),
    gate_venc!(CLK_VENC_3, "venc_3", "venc_sel", 12),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static OF_MATCH_CLK_MT6797_VENC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt6797-vencsys",
        data: &VENC_DESC,
    },
    OfDeviceId {
        // sentinel
        ..OfDeviceId::default()
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6797_venc);

static mut CLK_MT6797_VENC_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt6797-venc",
        of_match_table: OF_MATCH_CLK_MT6797_VENC.as_ptr(),
    },
};

// module_platform_driver(clk_mt6797_venc_drv);

// MODULE_DESCRIPTION("MediaTek MT6797 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
