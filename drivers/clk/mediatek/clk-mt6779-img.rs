// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Translated dependencies:
// linux/module.h, linux/clk-provider.h, linux/platform_device.h,
// dt-bindings/clock/mt6779-clk.h, clk-mtk.h, and clk-gate.h.

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
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(device: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(device: *mut core::ffi::c_void) -> i32;
}

const CLK_IMG_LARB5: u32 = 0;
const CLK_IMG_LARB6: u32 = 1;
const CLK_IMG_DIP: u32 = 2;
const CLK_IMG_MFB: u32 = 6;
const CLK_IMG_WPE_A: u32 = 7;

static IMG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

macro_rules! gate_mtk {
    ($id:expr, $name:expr, $parent:expr, $regs:expr, $shift:expr, $ops:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: $regs,
            shift: $shift,
            ops: $ops as *const _ as *const core::ffi::c_void,
        }
    };
}

macro_rules! gate_img {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &IMG_CG_REGS, $shift, unsafe {
            &mtk_clk_gate_ops_setclr
        })
    };
}

static IMG_CLKS: [MtkGate; 5] = [
    gate_img!(CLK_IMG_LARB5, "imgsys_larb5", "img_sel", 0),
    gate_img!(CLK_IMG_LARB6, "imgsys_larb6", "img_sel", 1),
    gate_img!(CLK_IMG_DIP, "imgsys_dip", "img_sel", 2),
    gate_img!(CLK_IMG_MFB, "imgsys_mfb", "img_sel", 6),
    gate_img!(CLK_IMG_WPE_A, "imgsys_wpe_a", "img_sel", 7),
];

static IMG_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMG_CLKS.as_ptr(),
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT6779_IMG: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6779-imgsys\0".as_ptr() as *const core::ffi::c_char,
        data: &IMG_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6779_IMG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt6779-img\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT6779_IMG.as_ptr(),
    },
};

// Equivalent to module_platform_driver(clk_mt6779_img_drv).
extern "C" {
    pub fn module_platform_driver(driver: *mut PlatformDriver);
}

#[used]
static REGISTER_CLK_MT6779_IMG: unsafe extern "C" fn(*mut PlatformDriver) =
    register_clk_mt6779_img;

unsafe extern "C" fn register_clk_mt6779_img() {
    module_platform_driver(&raw mut CLK_MT6779_IMG_DRV);
}

// MODULE_DESCRIPTION("MediaTek MT6779 imgsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
