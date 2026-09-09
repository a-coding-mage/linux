/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

use core::ffi::c_char;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw_onecell_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: clk_ops;
    pub static mtk_clk_gate_ops_setclr_inv: clk_ops;
    pub static mtk_clk_gate_ops_no_setclr: clk_ops;
    pub static mtk_clk_gate_ops_no_setclr_inv: clk_ops;
    pub static mtk_clk_gate_hwv_ops_setclr: clk_ops;
    pub static mtk_clk_gate_hwv_ops_setclr_inv: clk_ops;
}

#[repr(C)]
pub struct mtk_gate_regs {
    pub sta_ofs: u32,
    pub clr_ofs: u32,
    pub set_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: i32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const mtk_gate_regs,
    pub hwv_regs: *const mtk_gate_regs,
    pub shift: i32,
    pub ops: *const clk_ops,
    pub flags: usize,
}

macro_rules! GATE_MTK_FLAGS {
    ($id:expr, $name:expr, $parent:expr, $regs:expr, $shift:expr, $ops:expr, $flags:expr) => {
        mtk_gate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: $regs,
            shift: $shift,
            ops: $ops,
            flags: $flags,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! GATE_MTK {
    ($id:expr, $name:expr, $parent:expr, $regs:expr, $shift:expr, $ops:expr) => {
        GATE_MTK_FLAGS!($id, $name, $parent, $regs, $shift, $ops, 0)
    };
}

extern "C" {
    pub fn mtk_clk_register_gates(
        dev: *mut device,
        node: *mut device_node,
        clks: *const mtk_gate,
        num: i32,
        clk_data: *mut clk_hw_onecell_data,
    ) -> i32;

    pub fn mtk_clk_unregister_gates(
        clks: *const mtk_gate,
        num: i32,
        clk_data: *mut clk_hw_onecell_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
