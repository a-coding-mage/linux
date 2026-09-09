// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * drivers/clk/at91/clk-slow.c
 *
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk_sam9260_slow {
    pub hw: crate::clk_hw,
    pub regmap: *mut crate::regmap,
}

unsafe fn to_clk_sam9260_slow(hw: *mut crate::clk_hw) -> *mut clk_sam9260_slow {
    hw as *mut clk_sam9260_slow
}

unsafe fn clk_sam9260_slow_get_parent(hw: *mut crate::clk_hw) -> u8 {
    let slowck = &mut *to_clk_sam9260_slow(hw);
    let mut status: core::ffi::c_uint = 0;

    crate::regmap_read(slowck.regmap, crate::AT91_PMC_SR, &mut status);

    if status & crate::AT91_PMC_OSCSEL != 0 { 1 } else { 0 }
}

pub static sam9260_slow_ops: crate::clk_ops = crate::clk_ops {
    get_parent: Some(clk_sam9260_slow_get_parent),
};

pub unsafe extern "C" fn at91_clk_register_sam9260_slow(
    regmap: *mut crate::regmap,
    name: *const core::ffi::c_char,
    parent_names: *const *const core::ffi::c_char,
    num_parents: core::ffi::c_int,
) -> *mut crate::clk_hw {
    if name.is_null() {
        return crate::ERR_PTR(-crate::EINVAL);
    }

    if parent_names.is_null() || num_parents == 0 {
        return crate::ERR_PTR(-crate::EINVAL);
    }

    let slowck = crate::kzalloc_obj::<clk_sam9260_slow>();
    if slowck.is_null() {
        return crate::ERR_PTR(-crate::ENOMEM);
    }

    let mut init: crate::clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &sam9260_slow_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = 0;

    (*slowck).hw.init = &init;
    (*slowck).regmap = regmap;

    let hw = &mut (*slowck).hw as *mut crate::clk_hw;
    let ret = crate::clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        crate::kfree(slowck);
        return crate::ERR_PTR(ret);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
