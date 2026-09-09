// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the Linux clock framework and clk.h are intentionally
// left as external Rust declarations.

unsafe fn clk_sync_source_recalc_rate(
    hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    let sync: *mut tegra_clk_sync_source = to_clk_sync_source(hw);

    (*sync).rate
}

unsafe fn clk_sync_source_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let sync: *mut tegra_clk_sync_source = to_clk_sync_source(hw);

    if (*req).rate > (*sync).max_rate {
        -EINVAL
    } else {
        0
    }
}

unsafe fn clk_sync_source_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    let sync: *mut tegra_clk_sync_source = to_clk_sync_source(hw);

    (*sync).rate = rate;
    0
}

#[no_mangle]
pub static tegra_clk_sync_source_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_sync_source_determine_rate),
    set_rate: Some(clk_sync_source_set_rate),
    recalc_rate: Some(clk_sync_source_recalc_rate),
};

#[no_mangle]
pub unsafe fn tegra_clk_register_sync_source(
    name: *const c_char,
    max_rate: c_ulong,
) -> *mut clk {
    let sync: *mut tegra_clk_sync_source = kzalloc_obj();
    let mut init: clk_init_data;
    let clk: *mut clk;

    if sync.is_null() {
        pr_err!("%s: could not allocate sync source clk\n", __func__);
        return ERR_PTR(-ENOMEM);
    }

    (*sync).max_rate = max_rate;

    init.ops = &tegra_clk_sync_source_ops;
    init.name = name;
    init.flags = 0;
    init.parent_names = core::ptr::null();
    init.num_parents = 0;

    /* Data in .init is copied by clk_register(), so stack variable OK */
    (*sync).hw.init = &init;

    clk = clk_register(core::ptr::null_mut(), &mut (*sync).hw);
    if IS_ERR(clk) {
        kfree(sync);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
