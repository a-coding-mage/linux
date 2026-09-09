// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014 Freescale Semiconductor, Inc.
 */

// Dependency declarations corresponding to the Linux clock, error, I/O,
// allocator, and i.MX clock headers used by the original implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_gate {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub bit_idx: u8,
    pub lock: *mut c_void,
}

#[repr(C)]
pub struct clk_gate_exclusive {
    pub gate: clk_gate,
    pub exclusive_mask: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const i8,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const i8,
    pub num_parents: u8,
}

const EBUSY: i32 = 16;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const CLK_SET_RATE_PARENT: u32 = 1 << 2;

extern "C" {
    static mut imx_ccm_lock: *mut c_void;
    static clk_gate_ops: clk_ops;

    fn readl(addr: *mut c_void) -> u32;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn ERR_PTR<T>(error: isize) -> *mut T;
}

unsafe extern "C" fn clk_gate_exclusive_enable(hw: *mut clk_hw) -> i32 {
    let gate = hw as *mut clk_gate;
    // `gate` is the first field of clk_gate_exclusive, matching container_of.
    let exgate = gate as *mut clk_gate_exclusive;
    let val = readl((*gate).reg);

    if val & (*exgate).exclusive_mask != 0 {
        return -EBUSY;
    }

    ((*clk_gate_ops.enable).unwrap())(hw)
}

unsafe extern "C" fn clk_gate_exclusive_disable(hw: *mut clk_hw) {
    ((*clk_gate_ops.disable).unwrap())(hw);
}

unsafe extern "C" fn clk_gate_exclusive_is_enabled(hw: *mut clk_hw) -> i32 {
    ((*clk_gate_ops.is_enabled).unwrap())(hw)
}

#[repr(C)]
static CLK_GATE_EXCLUSIVE_OPS: clk_ops = clk_ops {
    enable: Some(clk_gate_exclusive_enable),
    disable: Some(clk_gate_exclusive_disable),
    is_enabled: Some(clk_gate_exclusive_is_enabled),
};

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_gate_exclusive(
    name: *const i8,
    parent: *const i8,
    reg: *mut c_void,
    shift: u8,
    exclusive_mask: u32,
) -> *mut clk_hw {
    let exgate: *mut clk_gate_exclusive;
    let gate: *mut clk_gate;
    let hw: *mut clk_hw;
    let mut init: clk_init_data;
    let ret: i32;

    if exclusive_mask == 0 {
        return ERR_PTR::<clk_hw>(-(EINVAL as isize));
    }

    exgate = kzalloc_obj::<clk_gate_exclusive>();
    if exgate.is_null() {
        return ERR_PTR::<clk_hw>(-(ENOMEM as isize));
    }
    gate = &mut (*exgate).gate;

    init.name = name;
    init.ops = &CLK_GATE_EXCLUSIVE_OPS;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = if !parent.is_null() { &parent } else { core::ptr::null() };
    init.num_parents = if !parent.is_null() { 1 } else { 0 };

    (*gate).reg = reg;
    (*gate).bit_idx = shift;
    (*gate).lock = imx_ccm_lock;
    (*gate).hw.init = &init;
    (*exgate).exclusive_mask = exclusive_mask;

    hw = &mut (*gate).hw;

    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(gate as *mut c_void);
        return ERR_PTR::<clk_hw>(ret as isize);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
