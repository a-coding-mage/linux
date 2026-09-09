// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Linux clock-provider, platform-device, device-tree, clk-gate, and clk-mtk
// declarations are supplied by the surrounding translation environment.

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
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

unsafe extern "C" {
    static mtk_clk_gate_ops_setclr_inv: c_void;
    fn mtk_clk_simple_probe(device: *mut c_void) -> i32;
    fn mtk_clk_simple_remove(device: *mut c_void) -> i32;
}

// Clock IDs are provided by dt-bindings/clock/mediatek,mt8188-clk.h.
unsafe extern "C" {
    static CLK_VENC1_LARB: u32;
    static CLK_VENC1_VENC: u32;
    static CLK_VENC1_JPGENC: u32;
    static CLK_VENC1_JPGDEC: u32;
    static CLK_VENC1_JPGDEC_C1: u32;
    static CLK_VENC1_GALS: u32;
    static CLK_VENC1_GALS_SRAM: u32;
}

static venc1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_VENC1(_id, _name, _parent, _shift) expands to GATE_MTK with the
// venc1 register block and set/clear/inverted gate operations.
static venc1_clks: [mtk_gate; 7] = [
    mtk_gate { id: unsafe { CLK_VENC1_LARB }, name: b"venc1_larb\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 0, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_VENC }, name: b"venc1_venc\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 4, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_JPGENC }, name: b"venc1_jpgenc\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 8, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_JPGDEC }, name: b"venc1_jpgdec\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 12, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_JPGDEC_C1 }, name: b"venc1_jpgdec_c1\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 16, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_GALS }, name: b"venc1_gals\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 28, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
    mtk_gate { id: unsafe { CLK_VENC1_GALS_SRAM }, name: b"venc1_gals_sram\0".as_ptr(), parent_name: b"top_venc\0".as_ptr(), regs: &venc1_cg_regs, shift: 31, ops: unsafe { &mtk_clk_gate_ops_setclr_inv } },
];

static venc1_desc: mtk_clk_desc = mtk_clk_desc {
    clks: venc1_clks.as_ptr(),
    num_clks: venc1_clks.len(),
};

static of_match_clk_mt8188_venc1: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mt8188-vencsys\0".as_ptr(), data: &venc1_desc as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut clk_mt8188_venc1_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt8188-venc1\0".as_ptr(),
        of_match_table: of_match_clk_mt8188_venc1.as_ptr(),
    },
};

// module_platform_driver(clk_mt8188_venc1_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_venc1);
// MODULE_DESCRIPTION("MediaTek MT8188 Video Encoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
