// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device, and
// MediaTek clock headers are intentionally left as external Rust items.

static VDEC0_CG_REGS: crate::clk_mtk::MtkGateRegs = crate::clk_mtk::MtkGateRegs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static VDEC1_CG_REGS: crate::clk_mtk::MtkGateRegs = crate::clk_mtk::MtkGateRegs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

macro_rules! gate_vdec0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        crate::clk_mtk::gate_mtk(
            $id,
            $name,
            $parent,
            &VDEC0_CG_REGS,
            $shift,
            &crate::clk_gate::MTK_CLK_GATE_OPS_SETCLR_INV,
        )
    };
}

macro_rules! gate_vdec1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        crate::clk_mtk::gate_mtk(
            $id,
            $name,
            $parent,
            &VDEC1_CG_REGS,
            $shift,
            &crate::clk_gate::MTK_CLK_GATE_OPS_SETCLR_INV,
        )
    };
}

static VDEC_CLKS: [crate::clk_mtk::MtkGate; 3] = [
    // VDEC0
    gate_vdec0!(crate::dt_bindings::clock::CLK_VDEC_CKEN, "vdec_cken", "vdec_sel", 0),
    // VDEC1
    gate_vdec1!(
        crate::dt_bindings::clock::CLK_VDEC_LARB1_CKEN,
        "vdec_larb1_cken",
        "vdec_sel",
        0,
    ),
    gate_vdec1!(
        crate::dt_bindings::clock::CLK_VDEC_IMGRZ_CKEN,
        "vdec_imgrz_cken",
        "vdec_sel",
        1,
    ),
];

static VDEC_DESC: crate::clk_mtk::MtkClkDesc = crate::clk_mtk::MtkClkDesc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: VDEC_CLKS.len(),
};

static OF_MATCH_CLK_MT2712_VDEC: [crate::platform_device::OfDeviceId; 2] = [
    crate::platform_device::OfDeviceId {
        compatible: "mediatek,mt2712-vdecsys",
        data: &VDEC_DESC as *const _ as *const core::ffi::c_void,
    },
    crate::platform_device::OfDeviceId {
        // sentinel
        compatible: "",
        data: core::ptr::null(),
    },
];

static mut CLK_MT2712_VDEC_DRV: crate::platform_device::PlatformDriver =
    crate::platform_device::PlatformDriver {
        probe: Some(crate::clk_mtk::mtk_clk_simple_probe),
        remove: Some(crate::clk_mtk::mtk_clk_simple_remove),
        driver: crate::platform_device::DeviceDriver {
            name: "clk-mt2712-vdec",
            of_match_table: OF_MATCH_CLK_MT2712_VDEC.as_ptr(),
        },
    };

crate::module_platform_driver!(CLK_MT2712_VDEC_DRV);

crate::module_description!("MediaTek MT2712 Video Decoders clocks driver");
crate::module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
