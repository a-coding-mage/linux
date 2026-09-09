// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated from the Linux kernel clock driver.  The declarations referenced
// by the original includes are supplied by the surrounding clock framework.

use core::ffi::c_void;

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: u32,
    pub name: *const u8,
    pub parent_name: *const u8,
    pub regs: *const MtkGateRegs,
    pub shift: u32,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr_inv: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
}

// GATE_VENC(_id, _name, _parent, _shift)
const fn gate_venc(
    id: u32,
    name: &'static [u8],
    parent: &'static [u8],
    regs: &'static MtkGateRegs,
    shift: u32,
    ops: *const c_void,
) -> MtkGate {
    MtkGate {
        id,
        name: name.as_ptr(),
        parent_name: parent.as_ptr(),
        regs,
        shift,
        ops,
    }
}

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Clock IDs are supplied by dt-bindings/clock/mt8186-clk.h.
extern "C" {
    static CLK_VENC_CKE0_LARB: u32;
    static CLK_VENC_CKE1_VENC: u32;
    static CLK_VENC_CKE2_JPGENC: u32;
    static CLK_VENC_CKE5_GALS: u32;
}

static VENC_CLKS: [MtkGate; 4] = [
    gate_venc(unsafe { CLK_VENC_CKE0_LARB }, b"venc_cke0_larb\0", b"top_venc\0", &VENC_CG_REGS, 0, unsafe { &mtk_clk_gate_ops_setclr_inv }),
    gate_venc(unsafe { CLK_VENC_CKE1_VENC }, b"venc_cke1_venc\0", b"top_venc\0", &VENC_CG_REGS, 4, unsafe { &mtk_clk_gate_ops_setclr_inv }),
    gate_venc(unsafe { CLK_VENC_CKE2_JPGENC }, b"venc_cke2_jpgenc\0", b"top_venc\0", &VENC_CG_REGS, 8, unsafe { &mtk_clk_gate_ops_setclr_inv }),
    gate_venc(unsafe { CLK_VENC_CKE5_GALS }, b"venc_cke5_gals\0", b"top_venc\0", &VENC_CG_REGS, 28, unsafe { &mtk_clk_gate_ops_setclr_inv }),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_VENC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8186-vencsys\0".as_ptr(),
        data: &VENC_DESC as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT8186_VENC_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt8186-venc\0".as_ptr(),
        of_match_table: OF_MATCH_CLK_MT8186_VENC.as_ptr(),
    },
};

// module_platform_driver(clk_mt8186_venc_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8186_venc);
// MODULE_DESCRIPTION("MediaTek MT8186 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
