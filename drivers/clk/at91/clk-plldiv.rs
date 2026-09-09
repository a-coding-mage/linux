// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Types, constants, and functions referenced below are supplied by the
// corresponding kernel headers and other translation units.

#[repr(C)]
pub struct clk_plldiv {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

unsafe fn to_clk_plldiv(hw: *mut clk_hw) -> *mut clk_plldiv {
    hw as *mut clk_plldiv
}

unsafe fn clk_plldiv_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let plldiv = &mut *to_clk_plldiv(hw);
    let mut mckr: c_uint = 0;

    regmap_read(plldiv.regmap, AT91_PMC_MCKR, &mut mckr);

    if mckr & AT91_PMC_PLLADIV2 != 0 {
        return parent_rate / 2;
    }

    parent_rate
}

unsafe fn clk_plldiv_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let req = &mut *req;
    let div: c_ulong;

    if req.rate > req.best_parent_rate {
        req.rate = req.best_parent_rate;
        return 0;
    }

    div = req.best_parent_rate / 2;
    if req.rate < div {
        req.rate = div;
        return 0;
    }

    if req.rate - div < req.best_parent_rate - req.rate {
        req.rate = div;
        return 0;
    }

    req.rate = req.best_parent_rate;
    0
}

unsafe fn clk_plldiv_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let plldiv = &mut *to_clk_plldiv(hw);

    if parent_rate != rate && parent_rate / 2 != rate {
        return -EINVAL;
    }

    regmap_update_bits(
        plldiv.regmap,
        AT91_PMC_MCKR,
        AT91_PMC_PLLADIV2,
        if parent_rate != rate { AT91_PMC_PLLADIV2 } else { 0 },
    );

    0
}

static plldiv_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_plldiv_recalc_rate),
    determine_rate: Some(clk_plldiv_determine_rate),
    set_rate: Some(clk_plldiv_set_rate),
};

pub unsafe fn at91_clk_register_plldiv(
    regmap: *mut regmap,
    name: *const c_char,
    parent_name: *const c_char,
) -> *mut clk_hw {
    let plldiv = kzalloc(core::mem::size_of::<clk_plldiv>()) as *mut clk_plldiv;
    let hw: *mut clk_hw;
    let mut init: clk_init_data;
    let ret: c_int;

    if plldiv.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &plldiv_ops;
    init.parent_names = if !parent_name.is_null() {
        &parent_name
    } else {
        core::ptr::null()
    };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };
    init.flags = CLK_SET_RATE_GATE;

    (*plldiv).hw.init = &init;
    (*plldiv).regmap = regmap;

    hw = &mut (*plldiv).hw;
    ret = clk_hw_register(core::ptr::null_mut(), &mut (*plldiv).hw);
    if ret != 0 {
        kfree(plldiv as *mut c_void);
        return ERR_PTR(ret);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
