// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Translated from the Linux kernel C implementation.  The declarations below
// refer to types and symbols supplied by the surrounding clock framework.

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
    pub shift: u32,
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
pub struct PlatformDriverInfo {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: PlatformDriverInfo,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr_inv: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

const CLK_VENC_SET0_LARB: u32 = 0;
const CLK_VENC_SET1_VENC: u32 = 1;
const CLK_VENC_SET2_JPGENC: u32 = 2;
const CLK_VENC_SET3_VDEC: u32 = 3;

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_venc {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: &VENC_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
        }
    };
}

static VENC_CLKS: [MtkGate; 4] = [
    gate_venc!(CLK_VENC_SET0_LARB, "venc_set0_larb", "mm_ck", 0),
    gate_venc!(CLK_VENC_SET1_VENC, "venc_set1_venc", "mm_ck", 4),
    gate_venc!(CLK_VENC_SET2_JPGENC, "jpgenc", "mm_ck", 8),
    gate_venc!(CLK_VENC_SET3_VDEC, "venc_set3_vdec", "mm_ck", 12),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_VCODEC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6765-vcodecsys\0".as_ptr() as *const core::ffi::c_char,
        data: &VENC_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6765_VCODEC_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: PlatformDriverInfo {
        name: b"clk-mt6765-vcodec\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT6765_VCODEC.as_ptr(),
    },
};

// Equivalent to module_platform_driver(clk_mt6765_vcodec_drv).

// MODULE_DESCRIPTION("MediaTek MT6765 Video Codec clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
