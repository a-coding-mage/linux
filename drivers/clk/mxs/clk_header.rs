/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// C header dependencies: linux/clk-provider.h and linux/spinlock.h.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub const SET: u32 = 0x4;
pub const CLR: u32 = 0x8;

extern "C" {
    pub static mut mxs_lock: spinlock_t;

    pub fn mxs_clk_wait(reg: *mut kernel::ffi::c_void, shift: u8) -> kernel::ffi::c_int;

    pub fn mxs_clk_pll(
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        base: *mut kernel::ffi::c_void,
        power: u8,
        rate: kernel::ffi::c_ulong,
    ) -> *mut clk;

    pub fn mxs_clk_ref(
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        reg: *mut kernel::ffi::c_void,
        idx: u8,
    ) -> *mut clk;

    pub fn mxs_clk_div(
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        reg: *mut kernel::ffi::c_void,
        shift: u8,
        width: u8,
        busy: u8,
    ) -> *mut clk;

    pub fn mxs_clk_frac(
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        reg: *mut kernel::ffi::c_void,
        shift: u8,
        width: u8,
        busy: u8,
    ) -> *mut clk;

    fn clk_register_fixed_rate(
        dev: *mut kernel::ffi::c_void,
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        flags: u32,
        rate: kernel::ffi::c_int,
    ) -> *mut clk;

    fn clk_register_gate(
        dev: *mut kernel::ffi::c_void,
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        flags: u32,
        reg: *mut kernel::ffi::c_void,
        shift: u8,
        flags2: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;

    fn clk_register_mux(
        dev: *mut kernel::ffi::c_void,
        name: *const kernel::ffi::c_char,
        parent_names: *const *const kernel::ffi::c_char,
        num_parents: kernel::ffi::c_int,
        flags: u32,
        reg: *mut kernel::ffi::c_void,
        shift: u8,
        width: u8,
        mux_flags: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;

    fn clk_register_fixed_factor(
        dev: *mut kernel::ffi::c_void,
        name: *const kernel::ffi::c_char,
        parent_name: *const kernel::ffi::c_char,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk;
}

// Supplied by linux/spinlock.h.
pub type spinlock_t = kernel::ffi::c_void;

// Supplied by linux/clk-provider.h.
pub const CLK_SET_RATE_PARENT: u32 = 1 << 0;
pub const CLK_SET_RATE_NO_REPARENT: u32 = 1 << 1;
pub const CLK_GATE_SET_TO_DISABLE: u8 = 1 << 0;

#[inline]
pub unsafe fn mxs_clk_fixed(name: *const kernel::ffi::c_char, rate: kernel::ffi::c_int) -> *mut clk {
    clk_register_fixed_rate(core::ptr::null_mut(), name, core::ptr::null(), 0, rate)
}

#[inline]
pub unsafe fn mxs_clk_gate(
    name: *const kernel::ffi::c_char,
    parent_name: *const kernel::ffi::c_char,
    reg: *mut kernel::ffi::c_void,
    shift: u8,
) -> *mut clk {
    clk_register_gate(
        core::ptr::null_mut(),
        name,
        parent_name,
        CLK_SET_RATE_PARENT,
        reg,
        shift,
        CLK_GATE_SET_TO_DISABLE,
        &raw mut mxs_lock,
    )
}

#[inline]
pub unsafe fn mxs_clk_mux(
    name: *const kernel::ffi::c_char,
    reg: *mut kernel::ffi::c_void,
    shift: u8,
    width: u8,
    parent_names: *const *const kernel::ffi::c_char,
    num_parents: kernel::ffi::c_int,
) -> *mut clk {
    clk_register_mux(
        core::ptr::null_mut(),
        name,
        parent_names,
        num_parents,
        CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
        reg,
        shift,
        width,
        0,
        &raw mut mxs_lock,
    )
}

#[inline]
pub unsafe fn mxs_clk_fixed_factor(
    name: *const kernel::ffi::c_char,
    parent_name: *const kernel::ffi::c_char,
    mult: u32,
    div: u32,
) -> *mut clk {
    clk_register_fixed_factor(
        core::ptr::null_mut(),
        name,
        parent_name,
        CLK_SET_RATE_PARENT,
        mult,
        div,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
