// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Translated from the Linux kernel clock-provider/platform-driver implementation.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    pub static mtk_clk_gate_ops_no_setclr_inv: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
    pub static CLK_IPU_ADL_CABGEN: u32;
}

static ipu_adl_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x204,
    clr_ofs: 0x204,
    sta_ofs: 0x204,
};

static ipu_adl_clks: [mtk_gate; 1] = [mtk_gate {
    id: unsafe { CLK_IPU_ADL_CABGEN },
    name: b"ipu_adl_cabgen\0".as_ptr() as *const c_char,
    parent_name: b"dsp_sel\0".as_ptr() as *const c_char,
    regs: &ipu_adl_cg_regs,
    shift: 24,
    ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv },
}];

static ipu_adl_desc: mtk_clk_desc = mtk_clk_desc {
    clks: ipu_adl_clks.as_ptr(),
    num_clks: ipu_adl_clks.len(),
};

static of_match_clk_mt8183_ipu_adl: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8183-ipu_adl\0".as_ptr() as *const c_char,
        data: &ipu_adl_desc as *const mtk_clk_desc as *const c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt8183_ipu_adl_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt8183-ipu_adl\0".as_ptr() as *const c_char,
        of_match_table: of_match_clk_mt8183_ipu_adl.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_adl);
// module_platform_driver(clk_mt8183_ipu_adl_drv);
// MODULE_DESCRIPTION("MediaTek MT8183 Image Processing Unit ADL driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
