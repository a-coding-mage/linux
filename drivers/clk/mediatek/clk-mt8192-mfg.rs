// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device, and
// MediaTek clock headers are referenced here but are defined externally.

use core::mem::size_of;

// Equivalent of the C GATE_MFG macro. The referenced constructor and symbols
// are supplied by the MediaTek clock implementation.
macro_rules! gate_mfg {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk_flags(
            $id,
            $name,
            $parent,
            &mfg_cg_regs,
            $shift,
            &mtk_clk_gate_ops_setclr,
            CLK_SET_RATE_PARENT,
        )
    };
}

static mfg_cg_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static mfg_clks: [MtkGate; 1] = [gate_mfg!(
    CLK_MFG_BG3D,
    "mfg_bg3d",
    "mfg_pll_sel",
    0,
)];

static mfg_desc: MtkClkDesc = MtkClkDesc {
    clks: mfg_clks.as_ptr(),
    num_clks: size_of::<[MtkGate; 1]>() / size_of::<MtkGate>(),
};

static of_match_clk_mt8192_mfg: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8192-mfgcfg",
        data: &mfg_desc,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// Equivalent of MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_mfg).

static mut clk_mt8192_mfg_drv: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8192-mfg",
        of_match_table: of_match_clk_mt8192_mfg.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt8192_mfg_drv).

// Equivalent of MODULE_DESCRIPTION("MediaTek MT8192 GPU mfg clocks driver").
// Equivalent of MODULE_LICENSE("GPL").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
