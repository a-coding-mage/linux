// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Dependencies supplied by the kernel and the MediaTek clock framework:
// linux/module.h, linux/clk-provider.h, linux/platform_device.h,
// dt-bindings/clock/mt6779-clk.h, clk-mtk.h, and clk-gate.h.

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
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct PlatformDriverInfo {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: PlatformDriverInfo,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
    pub fn module_platform_driver(driver: *mut PlatformDriver);
}

// Clock identifiers are supplied by dt-bindings/clock/mt6779-clk.h.
extern "C" {
    pub static CLK_CAM_LARB10: u32;
    pub static CLK_CAM_DFP_VAD: u32;
    pub static CLK_CAM_LARB11: u32;
    pub static CLK_CAM_LARB9: u32;
    pub static CLK_CAM_CAM: u32;
    pub static CLK_CAM_CAMTG: u32;
    pub static CLK_CAM_SENINF: u32;
    pub static CLK_CAM_CAMSV0: u32;
    pub static CLK_CAM_CAMSV1: u32;
    pub static CLK_CAM_CAMSV2: u32;
    pub static CLK_CAM_CAMSV3: u32;
    pub static CLK_CAM_CCU: u32;
    pub static CLK_CAM_FAKE_ENG: u32;
}

static CAM_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

macro_rules! gate_cam {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr(),
            parent_name: concat!($parent, "\0").as_ptr(),
            regs: &CAM_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr as *const c_void },
        }
    };
}

static CAM_CLKS: [MtkGate; 13] = [
    gate_cam!(unsafe { CLK_CAM_LARB10 }, "camsys_larb10", "cam_sel", 0),
    gate_cam!(unsafe { CLK_CAM_DFP_VAD }, "camsys_dfp_vad", "cam_sel", 1),
    gate_cam!(unsafe { CLK_CAM_LARB11 }, "camsys_larb11", "cam_sel", 2),
    gate_cam!(unsafe { CLK_CAM_LARB9 }, "camsys_larb9", "cam_sel", 3),
    gate_cam!(unsafe { CLK_CAM_CAM }, "camsys_cam", "cam_sel", 6),
    gate_cam!(unsafe { CLK_CAM_CAMTG }, "camsys_camtg", "cam_sel", 7),
    gate_cam!(unsafe { CLK_CAM_SENINF }, "camsys_seninf", "cam_sel", 8),
    gate_cam!(unsafe { CLK_CAM_CAMSV0 }, "camsys_camsv0", "cam_sel", 9),
    gate_cam!(unsafe { CLK_CAM_CAMSV1 }, "camsys_camsv1", "cam_sel", 10),
    gate_cam!(unsafe { CLK_CAM_CAMSV2 }, "camsys_camsv2", "cam_sel", 11),
    gate_cam!(unsafe { CLK_CAM_CAMSV3 }, "camsys_camsv3", "cam_sel", 12),
    gate_cam!(unsafe { CLK_CAM_CCU }, "camsys_ccu", "cam_sel", 13),
    gate_cam!(unsafe { CLK_CAM_FAKE_ENG }, "camsys_fake_eng", "cam_sel", 14),
];

static CAM_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_CLKS.as_ptr(),
    num_clks: CAM_CLKS.len(),
};

static OF_MATCH_CLK_MT6779_CAM: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6779-camsys\0".as_ptr(),
        data: &CAM_DESC as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6779_CAM_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: PlatformDriverInfo {
        name: b"clk-mt6779-cam\0".as_ptr(),
        of_match_table: OF_MATCH_CLK_MT6779_CAM.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6779_cam);
unsafe {
    module_platform_driver(&mut CLK_MT6779_CAM_DRV);
}

// MODULE_DESCRIPTION("MediaTek MT6779 Camera clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
