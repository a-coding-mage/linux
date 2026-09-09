// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated from the Linux kernel clock-provider, platform-device, MediaTek
// clock, clock-gate, and mt8192 clock binding headers.

use core::ffi::c_void;

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_clk_gate_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const u8,
    pub parent_name: *const u8,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const mtk_clk_gate_ops,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: platform_driver_driver,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: mtk_clk_gate_ops;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
    pub fn __platform_driver_register(driver: *mut platform_driver) -> i32;
    pub fn platform_driver_unregister(driver: *mut platform_driver);
}

// Clock IDs supplied by <dt-bindings/clock/mt8192-clk.h>.
extern "C" {
    pub static CLK_IPE_LARB19: u32;
    pub static CLK_IPE_LARB20: u32;
    pub static CLK_IPE_SMI_SUBCOM: u32;
    pub static CLK_IPE_FD: u32;
    pub static CLK_IPE_FE: u32;
    pub static CLK_IPE_RSC: u32;
    pub static CLK_IPE_DPE: u32;
    pub static CLK_IPE_GALS: u32;
}

static ipe_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_ipe {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: concat!($name, "\0").as_ptr(),
            parent_name: concat!($parent, "\0").as_ptr(),
            regs: &ipe_cg_regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr },
        }
    };
}

static ipe_clks: [mtk_gate; 8] = [
    gate_ipe!(unsafe { CLK_IPE_LARB19 }, "ipe_larb19", "ipe_sel", 0),
    gate_ipe!(unsafe { CLK_IPE_LARB20 }, "ipe_larb20", "ipe_sel", 1),
    gate_ipe!(unsafe { CLK_IPE_SMI_SUBCOM }, "ipe_smi_subcom", "ipe_sel", 2),
    gate_ipe!(unsafe { CLK_IPE_FD }, "ipe_fd", "ipe_sel", 3),
    gate_ipe!(unsafe { CLK_IPE_FE }, "ipe_fe", "ipe_sel", 4),
    gate_ipe!(unsafe { CLK_IPE_RSC }, "ipe_rsc", "ipe_sel", 5),
    gate_ipe!(unsafe { CLK_IPE_DPE }, "ipe_dpe", "ipe_sel", 6),
    gate_ipe!(unsafe { CLK_IPE_GALS }, "ipe_gals", "ipe_sel", 8),
];

static ipe_desc: mtk_clk_desc = mtk_clk_desc {
    clks: ipe_clks.as_ptr(),
    num_clks: ipe_clks.len(),
};

static of_match_clk_mt8192_ipe: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8192-ipesys\0".as_ptr(),
        data: &ipe_desc as *const mtk_clk_desc as *const c_void,
    },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut clk_mt8192_ipe_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_driver {
        name: b"clk-mt8192-ipe\0".as_ptr(),
        of_match_table: of_match_clk_mt8192_ipe.as_ptr(),
    },
};

#[used]
static MODULE_DEVICE_TABLE_OF: *const of_device_id = of_match_clk_mt8192_ipe.as_ptr();

#[used]
static MODULE_DESCRIPTION: &[u8] = b"MediaTek MT8192 Image Processing Engine clocks driver\0";
#[used]
static MODULE_LICENSE: &[u8] = b"GPL\0";

#[no_mangle]
pub unsafe extern "C" fn module_init() -> i32 {
    __platform_driver_register(&mut clk_mt8192_ipe_drv)
}

#[no_mangle]
pub unsafe extern "C" fn module_exit() {
    platform_driver_unregister(&mut clk_mt8192_ipe_drv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
