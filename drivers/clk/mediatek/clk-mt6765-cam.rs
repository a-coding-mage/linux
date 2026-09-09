// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Linux clock-provider, platform-device, MediaTek clock, gate, and device-tree
// bindings are supplied by the surrounding translation environment.

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: MtkGateOps;
}

#[repr(C)]
pub struct MtkGateOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MtkGate {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const MtkClkDesc,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn()>,
    pub remove: Option<unsafe extern "C" fn()>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    pub fn mtk_clk_simple_probe();
    pub fn mtk_clk_simple_remove();
    pub fn module_platform_driver(driver: *mut PlatformDriver);
}

static CAM_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Corresponds to the C GATE_CAM macro and the dependency-provided GATE_MTK
// constructor. The clock IDs are supplied by the device-tree bindings.
macro_rules! GATE_CAM {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &CAM_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static CAM_CLKS: [MtkGate; 9] = [
    GATE_CAM!(CLK_CAM_LARB3, b"cam_larb3\0", b"mm_ck\0", 0),
    GATE_CAM!(CLK_CAM_DFP_VAD, b"cam_dfp_vad\0", b"mm_ck\0", 1),
    GATE_CAM!(CLK_CAM, b"cam\0", b"mm_ck\0", 6),
    GATE_CAM!(CLK_CAMTG, b"camtg\0", b"mm_ck\0", 7),
    GATE_CAM!(CLK_CAM_SENINF, b"cam_seninf\0", b"mm_ck\0", 8),
    GATE_CAM!(CLK_CAMSV0, b"camsv0\0", b"mm_ck\0", 9),
    GATE_CAM!(CLK_CAMSV1, b"camsv1\0", b"mm_ck\0", 10),
    GATE_CAM!(CLK_CAMSV2, b"camsv2\0", b"mm_ck\0", 11),
    GATE_CAM!(CLK_CAM_CCU, b"cam_ccu\0", b"mm_ck\0", 12),
];

static CAM_DESC: MtkClkDesc = MtkClkDesc {
    clks: CAM_CLKS.as_ptr(),
    num_clks: CAM_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_CAM: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6765-camsys\0".as_ptr(),
        data: &CAM_DESC,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6765_cam);

static mut CLK_MT6765_CAM_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt6765-cam\0".as_ptr(),
        of_match_table: OF_MATCH_CLK_MT6765_CAM.as_ptr(),
    },
};

// module_platform_driver(clk_mt6765_cam_drv);
unsafe fn register_driver() {
    module_platform_driver(&raw mut CLK_MT6765_CAM_DRV);
}

// MODULE_DESCRIPTION("MediaTek MT6765 Camera clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
