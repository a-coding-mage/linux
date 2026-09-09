// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2025 Collabora Ltd.
 * Author: Nicolas Frattaroli <nicolas.frattaroli@collabora.com>
 *
 * Certain clocks on Rockchip are "gated" behind an additional register bit
 * write in a GRF register, such as the SAI MCLKs on RK3576. This code
 * implements a clock driver for these types of gates, based on regmaps.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct rockchip_gate_grf {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: u32,
    pub shift: u32,
    pub flags: u8,
}

// The following types and constants are supplied by the surrounding kernel bindings.
#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
}
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub flags: c_ulong,
    pub num_parents: u8,
    pub parent_names: *const *const c_char,
    pub ops: *const clk_ops,
}

const CLK_GATE_SET_TO_DISABLE: u8 = 1 << 0;
const CLK_GATE_HIWORD_MASK: u8 = 1 << 1;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;

extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_test_bits(map: *mut regmap, reg: u32, bits: u32) -> c_int;
    fn clk_register(hw: *mut c_void, init: *mut clk_hw) -> *mut clk;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize) -> *mut c_void;
    fn pr_err(fmt: *const c_char, ...);
    fn is_err<T>(ptr: *const T) -> bool;
    fn err_ptr<T>(err: c_int) -> *mut T;
}

#[inline]
unsafe fn to_gate_grf(hw: *mut clk_hw) -> *mut rockchip_gate_grf {
    // Equivalent to container_of(_hw, struct rockchip_gate_grf, hw).
    hw.cast::<rockchip_gate_grf>()
}

unsafe extern "C" fn rockchip_gate_grf_enable(hw: *mut clk_hw) -> c_int {
    let gate = &mut *to_gate_grf(hw);
    let val = if gate.flags & CLK_GATE_SET_TO_DISABLE == 0 {
        1u32 << gate.shift
    } else {
        0
    };
    let hiword = if gate.flags & CLK_GATE_HIWORD_MASK != 0 { 1u32 } else { 0 }
        << (gate.shift + 16);

    regmap_update_bits(
        gate.regmap,
        gate.reg,
        hiword | (1u32 << gate.shift),
        hiword | val,
    )
}

unsafe extern "C" fn rockchip_gate_grf_disable(hw: *mut clk_hw) {
    let gate = &mut *to_gate_grf(hw);
    let val = if gate.flags & CLK_GATE_SET_TO_DISABLE == 0 {
        0
    } else {
        1u32 << gate.shift
    };
    let hiword = if gate.flags & CLK_GATE_HIWORD_MASK != 0 { 1u32 } else { 0 }
        << (gate.shift + 16);

    regmap_update_bits(
        gate.regmap,
        gate.reg,
        hiword | (1u32 << gate.shift),
        hiword | val,
    );
}

unsafe extern "C" fn rockchip_gate_grf_is_enabled(hw: *mut clk_hw) -> c_int {
    let gate = &mut *to_gate_grf(hw);
    let invert = gate.flags & CLK_GATE_SET_TO_DISABLE != 0;
    let mut ret = regmap_test_bits(gate.regmap, gate.reg, 1u32 << gate.shift);
    if ret < 0 {
        ret = 0;
    }
    if invert { 1 - ret } else { ret }
}

static ROCKCHIP_GATE_GRF_OPS: clk_ops = clk_ops {
    enable: Some(rockchip_gate_grf_enable),
    disable: Some(rockchip_gate_grf_disable),
    is_enabled: Some(rockchip_gate_grf_is_enabled),
};

pub unsafe extern "C" fn rockchip_clk_register_gate_grf(
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    regmap: *mut regmap,
    reg: u32,
    shift: u32,
    gate_flags: u8,
) -> *mut clk {
    if is_err(regmap) {
        pr_err(b"%s: regmap not available\0".as_ptr().cast());
        return err_ptr(-EOPNOTSUPP);
    }

    let gate = kzalloc(core::mem::size_of::<rockchip_gate_grf>())
        as *mut rockchip_gate_grf;
    if gate.is_null() {
        return err_ptr(-ENOMEM);
    }

    let mut init = clk_init_data {
        name,
        flags,
        num_parents: if parent_name.is_null() { 0 } else { 1 },
        parent_names: if parent_name.is_null() { core::ptr::null() } else { &parent_name },
        ops: &ROCKCHIP_GATE_GRF_OPS,
    };

    (*gate).hw.init = &init;
    (*gate).regmap = regmap;
    (*gate).reg = reg;
    (*gate).shift = shift;
    (*gate).flags = gate_flags;

    let clk = clk_register(core::ptr::null_mut(), &mut (*gate).hw);
    if is_err(clk) {
        kfree(gate.cast());
    }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
