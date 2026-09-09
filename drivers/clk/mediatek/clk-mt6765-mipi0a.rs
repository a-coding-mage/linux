// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock-provider code.
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
    pub static mtk_clk_gate_ops_no_setclr_inv: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
    pub fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
}

// The clock binding constant is supplied by dt-bindings/clock/mt6765-clk.h.
extern "C" {
    pub static CLK_MIPI0A_CSR_CSI_EN_0A: u32;
}

static MIPI0A_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x80,
    clr_ofs: 0x80,
    sta_ofs: 0x80,
};

macro_rules! gate_mipi0a {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr(),
            parent_name: concat!($parent, "\0").as_ptr(),
            regs: &MIPI0A_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv as *const c_void },
        }
    };
}

static MIPI0A_CLKS: [MtkGate; 1] = [gate_mipi0a!(
    unsafe { CLK_MIPI0A_CSR_CSI_EN_0A },
    "mipi0a_csr_0a",
    "f_fseninf_ck",
    1
)];

static MIPI0A_DESC: MtkClkDesc = MtkClkDesc {
    clks: MIPI0A_CLKS.as_ptr(),
    num_clks: MIPI0A_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_MIPI0A: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt6765-mipi0a\0".as_ptr(),
        data: &MIPI0A_DESC as *const MtkClkDesc as *const c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6765_MIPI0A_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt6765-mipi0a\0".as_ptr(),
        of_match_table: OF_MATCH_CLK_MT6765_MIPI0A.as_ptr(),
    },
};

#[used]
static REGISTER_CLK_MT6765_MIPI0A: unsafe extern "C" fn(*mut PlatformDriver) -> i32 =
    platform_driver_register;

// MODULE_DESCRIPTION("MediaTek MT6765 Camera clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
