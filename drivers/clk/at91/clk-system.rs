// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this file.

const SYSTEM_MAX_ID: u8 = 31;
const SYSTEM_MAX_NAME_SZ: usize = 32;

#[repr(C)]
struct clk_system {
    hw: clk_hw,
    regmap: *mut regmap,
    pms: at91_clk_pms,
    id: u8,
}

#[inline]
unsafe fn is_pck(id: i32) -> bool {
    (id >= 8) && (id <= 15)
}

#[inline]
unsafe fn clk_system_ready(regmap: *mut regmap, id: i32) -> bool {
    let mut status: u32 = 0;

    regmap_read(regmap, AT91_PMC_SR, &mut status);

    (status & (1u32 << id)) != 0
}

unsafe extern "C" fn clk_system_prepare(hw: *mut clk_hw) -> i32 {
    let sys = (hw as *mut clk_system);

    regmap_write((*sys).regmap, AT91_PMC_SCER, 1u32 << (*sys).id);

    if !is_pck((*sys).id as i32) {
        return 0;
    }

    while !clk_system_ready((*sys).regmap, (*sys).id as i32) {
        cpu_relax();
    }

    0
}

unsafe extern "C" fn clk_system_unprepare(hw: *mut clk_hw) {
    let sys = hw as *mut clk_system;

    regmap_write((*sys).regmap, AT91_PMC_SCDR, 1u32 << (*sys).id);
}

unsafe extern "C" fn clk_system_is_prepared(hw: *mut clk_hw) -> i32 {
    let sys = hw as *mut clk_system;
    let mut status: u32 = 0;

    regmap_read((*sys).regmap, AT91_PMC_SCSR, &mut status);

    if (status & (1u32 << (*sys).id)) == 0 {
        return 0;
    }

    if !is_pck((*sys).id as i32) {
        return 1;
    }

    regmap_read((*sys).regmap, AT91_PMC_SR, &mut status);

    if (status & (1u32 << (*sys).id)) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn clk_system_save_context(hw: *mut clk_hw) -> i32 {
    let sys = hw as *mut clk_system;

    (*sys).pms.status = clk_system_is_prepared(hw);

    0
}

unsafe extern "C" fn clk_system_restore_context(hw: *mut clk_hw) {
    let sys = hw as *mut clk_system;

    if (*sys).pms.status != 0 {
        clk_system_prepare(&mut (*sys).hw);
    }
}

static system_ops: clk_ops = clk_ops {
    prepare: Some(clk_system_prepare),
    unprepare: Some(clk_system_unprepare),
    is_prepared: Some(clk_system_is_prepared),
    save_context: Some(clk_system_save_context),
    restore_context: Some(clk_system_restore_context),
};

pub unsafe extern "C" fn at91_clk_register_system(
    regmap: *mut regmap,
    name: *const i8,
    parent_name: *const i8,
    parent_hw: *mut clk_hw,
    id: u8,
    flags: c_ulong,
) -> *mut clk_hw {
    let mut sys: *mut clk_system;
    let hw: *mut clk_hw;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut ret: i32;

    if (parent_name.is_null() && parent_hw.is_null()) || id > SYSTEM_MAX_ID {
        return ERR_PTR(-EINVAL);
    }

    sys = kzalloc_obj::<clk_system>();
    if sys.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*sys).hw.init = &mut init;
    init.name = name;
    init.ops = &system_ops;
    if !parent_hw.is_null() {
        init.parent_hws = &parent_hw as *const *mut clk_hw as *const *const clk_hw;
    } else {
        init.parent_names = &parent_name;
    }
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_PARENT | flags;

    (*sys).id = id;
    (*sys).regmap = regmap;

    hw = &mut (*sys).hw;
    ret = clk_hw_register(core::ptr::null_mut(), &mut (*sys).hw);
    if ret != 0 {
        kfree(sys as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
