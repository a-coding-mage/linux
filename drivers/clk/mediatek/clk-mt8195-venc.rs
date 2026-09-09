// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the original C headers:
// "clk-gate.h", "clk-mtk.h", <dt-bindings/clock/mt8195-clk.h>,
// <linux/clk-provider.h>, and <linux/platform_device.h>.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const MtkGateRegs,
    pub shift: u8,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct PlatformDriverInner {
    pub name: *const c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: unsafe extern "C" fn(*mut c_void) -> i32,
    pub remove: unsafe extern "C" fn(*mut c_void) -> i32,
    pub driver: PlatformDriverInner,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr_inv: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
}

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_venc {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const c_char,
            regs: &VENC_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr_inv as *const c_void },
        }
    };
}

static VENC_CLKS: [MtkGate; 6] = [
    gate_venc!(CLK_VENC_LARB, "venc_larb", "top_venc", 0),
    gate_venc!(CLK_VENC_VENC, "venc_venc", "top_venc", 4),
    gate_venc!(CLK_VENC_JPGENC, "venc_jpgenc", "top_venc", 8),
    gate_venc!(CLK_VENC_JPGDEC, "venc_jpgdec", "top_venc", 12),
    gate_venc!(CLK_VENC_JPGDEC_C1, "venc_jpgdec_c1", "top_venc", 16),
    gate_venc!(CLK_VENC_GALS, "venc_gals", "top_venc", 28),
];

static VENC_CORE1_CLKS: [MtkGate; 6] = [
    gate_venc!(CLK_VENC_CORE1_LARB, "venc_core1_larb", "top_venc", 0),
    gate_venc!(CLK_VENC_CORE1_VENC, "venc_core1_venc", "top_venc", 4),
    gate_venc!(CLK_VENC_CORE1_JPGENC, "venc_core1_jpgenc", "top_venc", 8),
    gate_venc!(CLK_VENC_CORE1_JPGDEC, "venc_core1_jpgdec", "top_venc", 12),
    gate_venc!(CLK_VENC_CORE1_JPGDEC_C1, "venc_core1_jpgdec_c1", "top_venc", 16),
    gate_venc!(CLK_VENC_CORE1_GALS, "venc_core1_gals", "top_venc", 28),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static VENC_CORE1_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CORE1_CLKS.as_ptr(),
    num_clks: VENC_CORE1_CLKS.len(),
};

static OF_MATCH_CLK_MT8195_VENC: [OfDeviceId; 3] = [
    OfDeviceId {
        compatible: b"mediatek,mt8195-vencsys\0".as_ptr() as *const c_char,
        data: &VENC_DESC as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        compatible: b"mediatek,mt8195-vencsys_core1\0".as_ptr() as *const c_char,
        data: &VENC_CORE1_DESC as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT8195_VENC_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: PlatformDriverInner {
        name: b"clk-mt8195-venc\0".as_ptr() as *const c_char,
        of_match_table: OF_MATCH_CLK_MT8195_VENC.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt8195_venc_drv).
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_venc);
// MODULE_DESCRIPTION("MediaTek MT8195 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
