// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Dependencies supplied by the Linux clock, platform, device-tree, and
// MediaTek clock headers are intentionally left as external Rust items.

use core::ffi::c_void;

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const u8,
    pub parent_name: *const u8,
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
    static mtk_clk_gate_ops_setclr_inv: c_void;
    fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
}

// Device-tree clock identifiers supplied by dt-bindings/clock/mt6779-clk.h.
extern "C" {
    static CLK_VDEC_VDEC: u32;
    static CLK_VDEC_LARB1: u32;
}

static vdec0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0000,
    clr_ofs: 0x0004,
    sta_ofs: 0x0000,
};

static vdec1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0008,
    clr_ofs: 0x000c,
    sta_ofs: 0x0008,
};

macro_rules! gate_vdec_i {
    ($id:expr, $name:expr, $parent:expr, $regs:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: concat!($name, "\0").as_ptr(),
            parent_name: concat!($parent, "\0").as_ptr(),
            regs: &$regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr_inv as *const c_void },
        }
    };
}

static vdec_clks: [mtk_gate; 2] = [
    // VDEC0
    gate_vdec_i!(unsafe { CLK_VDEC_VDEC }, "vdec_cken", "vdec_sel", vdec0_cg_regs, 0),
    // VDEC1
    gate_vdec_i!(unsafe { CLK_VDEC_LARB1 }, "vdec_larb1_cken", "vdec_sel", vdec1_cg_regs, 0),
];

static vdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: vdec_clks.as_ptr(),
    num_clks: vdec_clks.len(),
};

static of_match_clk_mt6779_vdec: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt6779-vdecsys\0".as_ptr(),
        data: &vdec_desc as *const mtk_clk_desc as *const c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt6779_vdec_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_driver {
        name: b"clk-mt6779-vdec\0".as_ptr(),
        of_match_table: of_match_clk_mt6779_vdec.as_ptr(),
    },
};

// Equivalent to module_platform_driver(clk_mt6779_vdec_drv).
// MODULE_DEVICE_TABLE(of, of_match_clk_mt6779_vdec);
// MODULE_DESCRIPTION("MediaTek MT6779 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
