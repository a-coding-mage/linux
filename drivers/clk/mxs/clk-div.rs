// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding clock framework are referenced as
// external Rust items.

/**
 * struct clk_div - mxs integer divider clock
 * @divider: the parent class
 * @ops: pointer to clk_ops of parent class
 * @reg: register address
 * @busy: busy bit shift
 *
 * The mxs divider clock is a subclass of basic clk_divider with an
 * additional busy bit.
 */
#[repr(C)]
struct clk_div {
    divider: clk_divider,
    ops: *const clk_ops,
    reg: *mut core::ffi::c_void,
    busy: u8,
}

#[inline]
unsafe fn to_clk_div(hw: *mut clk_hw) -> *mut clk_div {
    let divider = to_clk_divider(hw);

    container_of!(divider, clk_div, divider)
}

unsafe extern "C" fn clk_div_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    let div = to_clk_div(hw);

    ((*(*div).ops).recalc_rate)(
        &mut (*div).divider.hw,
        parent_rate,
    )
}

unsafe extern "C" fn clk_div_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> core::ffi::c_int {
    let div = to_clk_div(hw);

    ((*(*div).ops).determine_rate)(&mut (*div).divider.hw, req)
}

unsafe extern "C" fn clk_div_set_rate(
    hw: *mut clk_hw,
    rate: core::ffi::c_ulong,
    parent_rate: core::ffi::c_ulong,
) -> core::ffi::c_int {
    let div = to_clk_div(hw);
    let mut ret: core::ffi::c_int;

    ret = ((*(*div).ops).set_rate)(&mut (*div).divider.hw, rate, parent_rate);
    if ret == 0 {
        ret = mxs_clk_wait((*div).reg, (*div).busy);
    }

    ret
}

static clk_div_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_div_recalc_rate),
    determine_rate: Some(clk_div_determine_rate),
    set_rate: Some(clk_div_set_rate),
};

unsafe fn mxs_clk_div(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
    shift: u8,
    width: u8,
    busy: u8,
) -> *mut clk {
    let div: *mut clk_div;
    let clk: *mut clk;
    let mut init: clk_init_data;

    div = kzalloc_obj::<clk_div>();
    if div.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &clk_div_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = if !parent_name.is_null() {
        &parent_name
    } else {
        core::ptr::null()
    };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*div).reg = reg;
    (*div).busy = busy;

    (*div).divider.reg = reg;
    (*div).divider.shift = shift;
    (*div).divider.width = width;
    (*div).divider.flags = CLK_DIVIDER_ONE_BASED;
    (*div).divider.lock = &mxs_lock;
    (*div).divider.hw.init = &init;
    (*div).ops = &clk_divider_ops;

    clk = clk_register(core::ptr::null_mut(), &mut (*div).divider.hw);
    if IS_ERR(clk) {
        kfree(div as *mut core::ffi::c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
