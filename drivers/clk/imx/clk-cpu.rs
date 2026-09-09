// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Lucas Stach <l.stach@pengutronix.de>, Pengutronix
 */

// Dependencies supplied by the Linux clock framework and clk.h.

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
}

pub type c_int = i32;
pub type c_ulong = usize;

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

pub type c_char = i8;

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate:
        Option<unsafe extern "C" fn(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong>,
    pub determine_rate:
        Option<unsafe extern "C" fn(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<
        unsafe extern "C" fn(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int,
    >,
}

#[repr(C)]
pub struct clk_cpu {
    pub hw: clk_hw,
    pub div: *mut clk,
    pub mux: *mut clk,
    pub pll: *mut clk,
    pub step: *mut clk,
}

unsafe fn to_clk_cpu(hw: *mut clk_hw) -> *mut clk_cpu {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_cpu, hw)) as *mut clk_cpu
}

unsafe extern "C" fn clk_cpu_recalc_rate(
    hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    let cpu = to_clk_cpu(hw);

    clk_get_rate((*cpu).div)
}

unsafe extern "C" fn clk_cpu_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let cpu = to_clk_cpu(hw);

    (*req).rate = clk_round_rate((*cpu).pll, (*req).rate);

    0
}

unsafe extern "C" fn clk_cpu_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    let cpu = to_clk_cpu(hw);
    let mut ret: c_int;

    // switch to PLL bypass clock
    ret = clk_set_parent((*cpu).mux, (*cpu).step);
    if ret != 0 {
        return ret;
    }

    // reprogram PLL
    ret = clk_set_rate((*cpu).pll, rate);
    if ret != 0 {
        clk_set_parent((*cpu).mux, (*cpu).pll);
        return ret;
    }
    // switch back to PLL clock
    clk_set_parent((*cpu).mux, (*cpu).pll);

    // Ensure the divider is what we expect
    clk_set_rate((*cpu).div, rate);

    0
}

static CLK_CPU_OPS: clk_ops = clk_ops {
    recalc_rate: Some(clk_cpu_recalc_rate),
    determine_rate: Some(clk_cpu_determine_rate),
    set_rate: Some(clk_cpu_set_rate),
};

pub unsafe extern "C" fn imx_clk_hw_cpu(
    name: *const c_char,
    parent_name: *const c_char,
    div: *mut clk,
    mux: *mut clk,
    pll: *mut clk,
    step: *mut clk,
) -> *mut clk_hw {
    let cpu = kzalloc_obj_clk_cpu();
    if cpu.is_null() {
        return err_ptr(-12);
    }

    (*cpu).div = div;
    (*cpu).mux = mux;
    (*cpu).pll = pll;
    (*cpu).step = step;

    let mut init = clk_init_data {
        name,
        ops: &CLK_CPU_OPS,
        flags: CLK_IS_CRITICAL,
        parent_names: &parent_name,
        num_parents: 1,
    };

    (*cpu).hw.init = &mut init;
    let hw = &mut (*cpu).hw as *mut clk_hw;

    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(cpu);
        return err_ptr(ret);
    }

    hw
}

extern "C" {
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_ulong;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> c_int;
    fn kfree(ptr: *mut clk_cpu);
    fn kzalloc_obj_clk_cpu() -> *mut clk_cpu;
    fn err_ptr(err: c_int) -> *mut clk_hw;
}

const CLK_IS_CRITICAL: u32 = 1 << 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
