// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but intentionally not reimplemented in this file.

const SMD_DIV_SHIFT: u32 = 8;
const SMD_MAX_DIV: u32 = 0xf;

#[repr(C)]
pub struct at91sam9x5_clk_smd {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

unsafe fn to_at91sam9x5_clk_smd(hw: *mut clk_hw) -> *mut at91sam9x5_clk_smd {
    // Equivalent to container_of(hw, struct at91sam9x5_clk_smd, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(at91sam9x5_clk_smd, hw))
        as *mut at91sam9x5_clk_smd
}

unsafe extern "C" fn at91sam9x5_clk_smd_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let smd = &mut *to_at91sam9x5_clk_smd(hw);
    let mut smdr: c_uint = 0;
    let _ = regmap_read(smd.regmap, AT91_PMC_SMD, &mut smdr);
    let smddiv: u8 = ((smdr & AT91_PMC_SMD_DIV) >> SMD_DIV_SHIFT) as u8;
    parent_rate / ((smddiv as c_ulong) + 1)
}

unsafe extern "C" fn at91sam9x5_clk_smd_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let req = &mut *req;
    if req.rate >= req.best_parent_rate {
        req.rate = req.best_parent_rate;
        return 0;
    }

    let div = req.best_parent_rate / req.rate;
    if div > SMD_MAX_DIV as c_ulong {
        req.rate = req.best_parent_rate / (SMD_MAX_DIV as c_ulong + 1);
        return 0;
    }

    let mut bestrate = req.best_parent_rate / div;
    let tmp = req.best_parent_rate / (div + 1);
    if bestrate - req.rate > req.rate - tmp {
        bestrate = tmp;
    }
    req.rate = bestrate;
    0
}

unsafe extern "C" fn at91sam9x5_clk_smd_set_parent(
    hw: *mut clk_hw,
    index: u8,
) -> c_int {
    let smd = &mut *to_at91sam9x5_clk_smd(hw);
    if index > 1 {
        return -EINVAL;
    }
    let value = if index != 0 { AT91_PMC_SMDS } else { 0 };
    let _ = regmap_update_bits(smd.regmap, AT91_PMC_SMD, AT91_PMC_SMDS, value);
    0
}

unsafe extern "C" fn at91sam9x5_clk_smd_get_parent(hw: *mut clk_hw) -> u8 {
    let smd = &mut *to_at91sam9x5_clk_smd(hw);
    let mut smdr: c_uint = 0;
    let _ = regmap_read(smd.regmap, AT91_PMC_SMD, &mut smdr);
    (smdr & AT91_PMC_SMDS) as u8
}

unsafe extern "C" fn at91sam9x5_clk_smd_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let smd = &mut *to_at91sam9x5_clk_smd(hw);
    let div = parent_rate / rate;
    if parent_rate % rate != 0 || div < 1 || div > SMD_MAX_DIV as c_ulong + 1 {
        return -EINVAL;
    }
    let value = (div - 1) << SMD_DIV_SHIFT;
    let _ = regmap_update_bits(smd.regmap, AT91_PMC_SMD, AT91_PMC_SMD_DIV, value);
    0
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}

static at91sam9x5_smd_ops: clk_ops = clk_ops {
    recalc_rate: Some(at91sam9x5_clk_smd_recalc_rate),
    determine_rate: Some(at91sam9x5_clk_smd_determine_rate),
    get_parent: Some(at91sam9x5_clk_smd_get_parent),
    set_parent: Some(at91sam9x5_clk_smd_set_parent),
    set_rate: Some(at91sam9x5_clk_smd_set_rate),
};

pub unsafe extern "C" fn at91sam9x5_clk_register_smd(
    regmap: *mut regmap,
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: u8,
) -> *mut clk_hw {
    let smd = kzalloc_obj::<at91sam9x5_clk_smd>();
    if smd.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let init = clk_init_data {
        name,
        ops: &at91sam9x5_smd_ops,
        parent_names,
        num_parents,
        flags: CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE,
    };
    (*smd).hw.init = &init;
    (*smd).regmap = regmap;

    let hw = &mut (*smd).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(smd as *mut c_void);
        return ERR_PTR(ret);
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
