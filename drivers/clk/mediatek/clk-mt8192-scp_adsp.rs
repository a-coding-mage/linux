// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt8192-clk.h are supplied by other translation units.

use core::ffi::c_char;

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
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    pub static mtk_clk_gate_ops_no_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

// The C GATE_MTK initializer is supplied by clk-mtk.h/clk-gate.h.
macro_rules! GATE_SCP_ADSP {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const c_char,
            regs: &scp_adsp_cg_regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_no_setclr },
        }
    };
}

// CLK_SCP_ADSP_AUDIODSP is provided by dt-bindings/clock/mt8192-clk.h.
extern "C" {
    pub static CLK_SCP_ADSP_AUDIODSP: u32;
}

static scp_adsp_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x180,
    clr_ofs: 0x180,
    sta_ofs: 0x180,
};

static scp_adsp_clks: [mtk_gate; 1] = [GATE_SCP_ADSP!(
    unsafe { CLK_SCP_ADSP_AUDIODSP },
    "scp_adsp_audiodsp",
    "adsp_sel",
    0,
)];

static scp_adsp_desc: mtk_clk_desc = mtk_clk_desc {
    clks: scp_adsp_clks.as_ptr(),
    num_clks: scp_adsp_clks.len(),
};

static of_match_clk_mt8192_scp_adsp: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8192-scp_adsp\0".as_ptr() as *const c_char,
        data: &scp_adsp_desc as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt8192_scp_adsp_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt8192-scp_adsp\0".as_ptr() as *const c_char,
        of_match_table: of_match_clk_mt8192_scp_adsp.as_ptr(),
    },
};

// module_platform_driver(clk_mt8192_scp_adsp_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_scp_adsp);
// MODULE_DESCRIPTION("MediaTek MT8192 SCP AudioDSP clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
