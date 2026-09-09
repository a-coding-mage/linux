// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock, platform-device, and MTK clock
// headers are intentionally referenced here rather than reimplemented.

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
    pub flags: u32,
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
pub struct DeviceDriver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: DeviceDriver,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    static CLK_MFG_BG3D: u32;
    static CLK_SET_RATE_PARENT: u32;
    unsafe fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    unsafe fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
    unsafe fn __platform_driver_register(
        driver: *mut PlatformDriver,
        owner: *mut core::ffi::c_void,
    ) -> i32;
}

static MFG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static MFG_CLKS: [MtkGate; 1] = [MtkGate {
    id: unsafe { CLK_MFG_BG3D },
    name: b"mfg_bg3d\0".as_ptr() as *const core::ffi::c_char,
    parent_name: b"top_mfg\0".as_ptr() as *const core::ffi::c_char,
    regs: &MFG_CG_REGS,
    shift: 0,
    ops: unsafe { &mtk_clk_gate_ops_setclr },
    flags: unsafe { CLK_SET_RATE_PARENT },
}];

static MFG_DESC: MtkClkDesc = MtkClkDesc {
    clks: MFG_CLKS.as_ptr(),
    num_clks: MFG_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_MFG: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8186-mfgsys\0".as_ptr() as *const core::ffi::c_char,
        data: &MFG_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT8186_MFG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: DeviceDriver {
        name: b"clk-mt8186-mfg\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT8186_MFG.as_ptr(),
    },
};

// module_platform_driver(clk_mt8186_mfg_drv);
#[allow(dead_code)]
unsafe fn register_clk_mt8186_mfg_driver() -> i32 {
    __platform_driver_register(&raw mut CLK_MT8186_MFG_DRV, core::ptr::null_mut())
}

// MODULE_DESCRIPTION("MediaTek MT8186 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
