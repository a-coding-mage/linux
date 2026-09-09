// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * clk-h32mx.c
 *
 *  Copyright (C) 2014 Atmel
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const H32MX_MAX_FREQ: ::core::ffi::c_ulong = 90_000_000;

#[repr(C)]
pub struct clk_sama5d4_h32mx {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[inline]
unsafe fn to_clk_sama5d4_h32mx(hw: *mut clk_hw) -> *mut clk_sama5d4_h32mx {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_sama5d4_h32mx, hw))
        as *mut clk_sama5d4_h32mx
}

unsafe extern "C" fn clk_sama5d4_h32mx_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let h32mxclk = &mut *to_clk_sama5d4_h32mx(hw);
    let mut mckr: u32 = 0;

    regmap_read(h32mxclk.regmap, AT91_PMC_MCKR, &mut mckr);
    if mckr & AT91_PMC_H32MXDIV != 0 {
        return parent_rate / 2;
    }

    if parent_rate > H32MX_MAX_FREQ {
        pr_warn(b"H32MX clock is too fast\n\0".as_ptr() as *const _);
    }
    parent_rate
}

unsafe extern "C" fn clk_sama5d4_h32mx_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let req = &mut *req;
    let div: ::core::ffi::c_ulong;

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

unsafe extern "C" fn clk_sama5d4_h32mx_set_rate(
    hw: *mut clk_hw,
    rate: ::core::ffi::c_ulong,
    parent_rate: ::core::ffi::c_ulong,
) -> i32 {
    let h32mxclk = &mut *to_clk_sama5d4_h32mx(hw);
    let mut mckr: u32 = 0;

    if parent_rate != rate && (parent_rate / 2) != rate {
        return -22; // -EINVAL
    }
    if (parent_rate / 2) == rate {
        mckr = AT91_PMC_H32MXDIV;
    }
    regmap_update_bits(h32mxclk.regmap, AT91_PMC_MCKR, AT91_PMC_H32MXDIV, mckr);
    0
}

#[no_mangle]
pub static h32mx_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_sama5d4_h32mx_recalc_rate),
    determine_rate: Some(clk_sama5d4_h32mx_determine_rate),
    set_rate: Some(clk_sama5d4_h32mx_set_rate),
};

pub unsafe extern "C" fn at91_clk_register_h32mx(
    regmap: *mut regmap,
    name: *const ::core::ffi::c_char,
    parent_name: *const ::core::ffi::c_char,
) -> *mut clk_hw {
    let h32mxclk = kzalloc_obj::<clk_sama5d4_h32mx>();
    if h32mxclk.is_null() {
        return err_ptr(-12); // -ENOMEM
    }

    let init = clk_init_data {
        name,
        ops: &h32mx_ops,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
        flags: CLK_SET_RATE_GATE,
    };

    (*h32mxclk).hw.init = &init;
    (*h32mxclk).regmap = regmap;

    let ret = clk_hw_register(core::ptr::null_mut(), &mut (*h32mxclk).hw);
    if ret != 0 {
        kfree(h32mxclk as *mut _);
        return err_ptr(ret);
    }
    &mut (*h32mxclk).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
