// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// Translated dependencies:
// <dt-bindings/clock/mediatek,mt8365-clk.h>
// <linux/clk-provider.h>
// <linux/platform_device.h>
// "clk-gate.h"
// "clk-mtk.h"

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
pub struct PlatformDriverDriver {
    pub name: *const c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: PlatformDriverDriver,
}

extern "C" {
    static mtk_clk_gate_ops_setclr_inv: c_void;
    static mtk_clk_simple_probe: unsafe extern "C" fn(*mut c_void) -> i32;
    static mtk_clk_simple_remove: unsafe extern "C" fn(*mut c_void) -> i32;

    // Clock IDs supplied by dt-bindings/clock/mediatek,mt8365-clk.h.
    static CLK_VENC: u32;
    static CLK_VENC_JPGENC: u32;
}

static venc_cg_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static venc_clks: [MtkGate; 2] = [
    // VENC
    MtkGate {
        id: unsafe { CLK_VENC },
        name: b"venc_fvenc_ck\0".as_ptr() as *const c_char,
        parent_name: b"mm_sel\0".as_ptr() as *const c_char,
        regs: &venc_cg_regs,
        shift: 4,
        ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
    },
    MtkGate {
        id: unsafe { CLK_VENC_JPGENC },
        name: b"venc_jpgenc_ck\0".as_ptr() as *const c_char,
        parent_name: b"mm_sel\0".as_ptr() as *const c_char,
        regs: &venc_cg_regs,
        shift: 8,
        ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
    },
];

static venc_desc: MtkClkDesc = MtkClkDesc {
    clks: venc_clks.as_ptr(),
    num_clks: venc_clks.len(),
};

static of_match_clk_mt8365_venc: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8365-vencsys\0".as_ptr() as *const c_char,
        data: &venc_desc as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8365_venc);

static mut clk_mt8365_venc_drv: PlatformDriver = PlatformDriver {
    probe: Some(unsafe { core::mem::transmute(mtk_clk_simple_probe) }),
    remove: Some(unsafe { core::mem::transmute(mtk_clk_simple_remove) }),
    driver: PlatformDriverDriver {
        name: b"clk-mt8365-venc\0".as_ptr() as *const c_char,
        of_match_table: of_match_clk_mt8365_venc.as_ptr(),
    },
};

// module_platform_driver(clk_mt8365_venc_drv);

// MODULE_DESCRIPTION("MediaTek MT8365 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
