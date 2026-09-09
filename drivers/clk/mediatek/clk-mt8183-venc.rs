// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and MT8183 clock-binding interfaces are referenced
// below and are intentionally not reimplemented here.

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_VENC_I {
    ($id:ident, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VENC_CG_REGS, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

static VENC_CLKS: [MtkGate; 3] = [
    GATE_VENC_I!(CLK_VENC_LARB, "venc_larb", "mm_sel", 0),
    GATE_VENC_I!(CLK_VENC_VENC, "venc_venc", "mm_sel", 4),
    GATE_VENC_I!(CLK_VENC_JPGENC, "venc_jpgenc", "mm_sel", 8),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: &VENC_CLKS,
    num_clks: ARRAY_SIZE!(VENC_CLKS),
};

static OF_MATCH_CLK_MT8183_VENC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8183-vencsys",
        data: &VENC_DESC,
    },
    OfDeviceId {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8183_VENC);

static mut CLK_MT8183_VENC_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: PlatformDriverData {
        name: "clk-mt8183-venc",
        of_match_table: &OF_MATCH_CLK_MT8183_VENC,
    },
};

module_platform_driver!(CLK_MT8183_VENC_DRV);

MODULE_DESCRIPTION!("MediaTek MT8183 Video Encoders clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
