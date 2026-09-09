// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Translated from the Linux kernel C implementation.  The referenced clock,
// device, and module items are supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub regs: *const MtkGateRegs,
    pub shift: u8,
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr_inv: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

extern "C" {
    pub static CLK_VENC_GCON_LARB: u32;
    pub static CLK_VENC_GCON_VENC: u32;
    pub static CLK_VENC_GCON_JPGENC: u32;
    pub static CLK_VENC_GCON_GALS: u32;
}

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

macro_rules! gate_venc_i {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: &VENC_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_setclr_inv as *const _ as *const core::ffi::c_void,
        }
    };
}

static VENC_CLKS: [MtkGate; 4] = [
    gate_venc_i!(CLK_VENC_GCON_LARB, "venc_larb", "venc_sel", 0),
    gate_venc_i!(CLK_VENC_GCON_VENC, "venc_venc", "venc_sel", 4),
    gate_venc_i!(CLK_VENC_GCON_JPGENC, "venc_jpgenc", "venc_sel", 8),
    gate_venc_i!(CLK_VENC_GCON_GALS, "venc_gals", "venc_sel", 28),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static OF_MATCH_CLK_MT6779_VENC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6779-vencsys\0".as_ptr() as *const core::ffi::c_char,
        data: &VENC_DESC as *const _ as *const core::ffi::c_void,
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    }, // sentinel
];

static mut CLK_MT6779_VENC_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt6779-venc\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT6779_VENC.as_ptr(),
    },
};

// module_platform_driver(clk_mt6779_venc_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt6779_venc);
// MODULE_DESCRIPTION("MediaTek MT6779 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
