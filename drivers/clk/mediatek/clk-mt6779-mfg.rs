// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Dependencies supplied by the Linux clock, platform, and MediaTek clock
// headers are intentionally left as external Rust items.

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
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

// Build-time symbol supplied by dt-bindings/clock/mt6779-clk.h.
extern "C" {
    pub static CLK_MFGCFG_BG3D: u32;
}

static MFG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_mfg {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: &MFG_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr as *const _ as *const core::ffi::c_void },
        }
    };
}

static MFG_CLKS: [MtkGate; 1] = [gate_mfg!(
    unsafe { CLK_MFGCFG_BG3D },
    "mfg_bg3d",
    "mfg_sel",
    0
)];

static MFG_DESC: MtkClkDesc = MtkClkDesc {
    clks: MFG_CLKS.as_ptr(),
    num_clks: MFG_CLKS.len(),
};

static OF_MATCH_CLK_MT6779_MFG: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6779-mfgcfg\0".as_ptr() as *const core::ffi::c_char,
        data: &MFG_DESC as *const _ as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6779_MFG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt6779-mfg\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT6779_MFG.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt6779_mfg_drv).
#[no_mangle]
pub unsafe extern "C" fn init_module() -> i32 {
    let _ = &mut CLK_MT6779_MFG_DRV;
    0
}

// MODULE_DESCRIPTION("MediaTek MT6779 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
