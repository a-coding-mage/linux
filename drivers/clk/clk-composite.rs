// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2013 NVIDIA CORPORATION.  All rights reserved.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

unsafe fn clk_composite_get_parent(hw: *mut clk_hw) -> u8 {
    let composite = to_clk_composite(hw);
    let mux_ops = (*composite).mux_ops;
    let mux_hw = (*composite).mux_hw;
    __clk_hw_set_clk(mux_hw, hw);
    ((*mux_ops).get_parent.unwrap())(mux_hw)
}

unsafe fn clk_composite_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let composite = to_clk_composite(hw);
    let mux_ops = (*composite).mux_ops;
    let mux_hw = (*composite).mux_hw;
    __clk_hw_set_clk(mux_hw, hw);
    ((*mux_ops).set_parent.unwrap())(mux_hw, index)
}

unsafe fn clk_composite_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let composite = to_clk_composite(hw);
    let rate_ops = (*composite).rate_ops;
    let rate_hw = (*composite).rate_hw;
    __clk_hw_set_clk(rate_hw, hw);
    ((*rate_ops).recalc_rate.unwrap())(rate_hw, parent_rate)
}

unsafe fn clk_composite_determine_rate_for_parent(rate_hw: *mut clk_hw, req: *mut clk_rate_request,
                                                  parent_hw: *mut clk_hw,
                                                  rate_ops: *const clk_ops) -> i32 {
    (*req).best_parent_hw = parent_hw;
    (*req).best_parent_rate = clk_hw_get_rate(parent_hw);
    ((*rate_ops).determine_rate.unwrap())(rate_hw, req)
}

unsafe fn clk_composite_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let composite = to_clk_composite(hw);
    let rate_ops = (*composite).rate_ops;
    let mux_ops = (*composite).mux_ops;
    let rate_hw = (*composite).rate_hw;
    let mux_hw = (*composite).mux_hw;
    let mut rate_diff: c_ulong;
    let mut best_rate_diff = ULONG_MAX;
    let mut best_rate: c_ulong = 0;

    if !rate_hw.is_null() && !rate_ops.is_null() && (*rate_ops).determine_rate.is_some()
        && !mux_hw.is_null() && !mux_ops.is_null() && (*mux_ops).set_parent.is_some() {
        (*req).best_parent_hw = core::ptr::null_mut();
        if clk_hw_get_flags(hw) & CLK_SET_RATE_NO_REPARENT != 0 {
            let parent = clk_hw_get_parent(mux_hw);
            let mut tmp_req = core::mem::zeroed::<clk_rate_request>();
            clk_hw_forward_rate_request(hw, req, parent, &mut tmp_req, (*req).rate);
            let ret = clk_composite_determine_rate_for_parent(rate_hw, &mut tmp_req, parent, rate_ops);
            if ret != 0 { return ret; }
            (*req).rate = tmp_req.rate;
            (*req).best_parent_hw = tmp_req.best_parent_hw;
            (*req).best_parent_rate = tmp_req.best_parent_rate;
            return 0;
        }
        for i in 0..clk_hw_get_num_parents(mux_hw) {
            let parent = clk_hw_get_parent_by_index(mux_hw, i);
            if parent.is_null() { continue; }
            let mut tmp_req = core::mem::zeroed::<clk_rate_request>();
            clk_hw_forward_rate_request(hw, req, parent, &mut tmp_req, (*req).rate);
            if clk_composite_determine_rate_for_parent(rate_hw, &mut tmp_req, parent, rate_ops) != 0 { continue; }
            rate_diff = if (*req).rate >= tmp_req.rate { (*req).rate - tmp_req.rate } else { tmp_req.rate - (*req).rate };
            if rate_diff == 0 || (*req).best_parent_hw.is_null() || best_rate_diff > rate_diff {
                (*req).best_parent_hw = parent;
                (*req).best_parent_rate = tmp_req.best_parent_rate;
                best_rate_diff = rate_diff;
                best_rate = tmp_req.rate;
            }
            if rate_diff == 0 { return 0; }
        }
        (*req).rate = best_rate;
        return 0;
    } else if !rate_hw.is_null() && !rate_ops.is_null() && (*rate_ops).determine_rate.is_some() {
        __clk_hw_set_clk(rate_hw, hw);
        return ((*rate_ops).determine_rate.unwrap())(rate_hw, req);
    } else if !mux_hw.is_null() && !mux_ops.is_null() && (*mux_ops).determine_rate.is_some() {
        __clk_hw_set_clk(mux_hw, hw);
        return ((*mux_ops).determine_rate.unwrap())(mux_hw, req);
    }
    pr_err!("clk: clk_composite_determine_rate function called, but no mux or rate callback set!\n");
    -EINVAL
}

unsafe fn clk_composite_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let composite = to_clk_composite(hw);
    let rate_ops = (*composite).rate_ops;
    let rate_hw = (*composite).rate_hw;
    __clk_hw_set_clk(rate_hw, hw);
    ((*rate_ops).set_rate.unwrap())(rate_hw, rate, parent_rate)
}

unsafe fn clk_composite_set_rate_and_parent(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong, index: u8) -> i32 {
    let composite = to_clk_composite(hw);
    let rate_ops = (*composite).rate_ops;
    let mux_ops = (*composite).mux_ops;
    let rate_hw = (*composite).rate_hw;
    let mux_hw = (*composite).mux_hw;
    __clk_hw_set_clk(rate_hw, hw);
    __clk_hw_set_clk(mux_hw, hw);
    let temp_rate = ((*rate_ops).recalc_rate.unwrap())(rate_hw, parent_rate);
    if temp_rate > rate {
        ((*rate_ops).set_rate.unwrap())(rate_hw, rate, parent_rate);
        ((*mux_ops).set_parent.unwrap())(mux_hw, index);
    } else {
        ((*mux_ops).set_parent.unwrap())(mux_hw, index);
        ((*rate_ops).set_rate.unwrap())(rate_hw, rate, parent_rate);
    }
    0
}

unsafe fn clk_composite_is_enabled(hw: *mut clk_hw) -> i32 {
    let c = to_clk_composite(hw); __clk_hw_set_clk((*c).gate_hw, hw); ((*(*c).gate_ops).is_enabled.unwrap())((*c).gate_hw)
}
unsafe fn clk_composite_enable(hw: *mut clk_hw) -> i32 {
    let c = to_clk_composite(hw); __clk_hw_set_clk((*c).gate_hw, hw); ((*(*c).gate_ops).enable.unwrap())((*c).gate_hw)
}
unsafe fn clk_composite_disable(hw: *mut clk_hw) { let c = to_clk_composite(hw); __clk_hw_set_clk((*c).gate_hw, hw); ((*(*c).gate_ops).disable.unwrap())((*c).gate_hw); }

unsafe fn clk_hw_register_composite(dev: *mut device, name: *const c_char, parent_names: *const *const c_char, num_parents: i32, mux_hw: *mut clk_hw, mux_ops: *const clk_ops, rate_hw: *mut clk_hw, rate_ops: *const clk_ops, gate_hw: *mut clk_hw, gate_ops: *const clk_ops, flags: c_ulong) -> *mut clk_hw {
    __clk_hw_register_composite(dev, name, parent_names, core::ptr::null(), num_parents, mux_hw, mux_ops, rate_hw, rate_ops, gate_hw, gate_ops, flags)
}

unsafe fn clk_hw_register_composite_pdata(dev: *mut device, name: *const c_char, parent_data: *const clk_parent_data, num_parents: i32, mux_hw: *mut clk_hw, mux_ops: *const clk_ops, rate_hw: *mut clk_hw, rate_ops: *const clk_ops, gate_hw: *mut clk_hw, gate_ops: *const clk_ops, flags: c_ulong) -> *mut clk_hw {
    __clk_hw_register_composite(dev, name, core::ptr::null(), parent_data, num_parents, mux_hw, mux_ops, rate_hw, rate_ops, gate_hw, gate_ops, flags)
}

unsafe fn clk_register_composite(dev: *mut device, name: *const c_char, parents: *const *const c_char, n: i32, mux: *mut clk_hw, mo: *const clk_ops, rate: *mut clk_hw, ro: *const clk_ops, gate: *mut clk_hw, go: *const clk_ops, flags: c_ulong) -> *mut clk {
    let hw = clk_hw_register_composite(dev, name, parents, n, mux, mo, rate, ro, gate, go, flags);
    if IS_ERR(hw) { return ERR_CAST(hw); } (*hw).clk
}

unsafe fn clk_register_composite_pdata(dev: *mut device, name: *const c_char, pdata: *const clk_parent_data, n: i32, mux: *mut clk_hw, mo: *const clk_ops, rate: *mut clk_hw, ro: *const clk_ops, gate: *mut clk_hw, go: *const clk_ops, flags: c_ulong) -> *mut clk {
    let hw = clk_hw_register_composite_pdata(dev, name, pdata, n, mux, mo, rate, ro, gate, go, flags);
    if IS_ERR(hw) { return ERR_CAST(hw); } (*hw).clk
}

unsafe fn clk_unregister_composite(clk: *mut clk) {
    let hw = __clk_get_hw(clk); if hw.is_null() { return; }
    let composite = to_clk_composite(hw); clk_unregister(clk); kfree(composite as *mut core::ffi::c_void);
}
unsafe fn clk_hw_unregister_composite(hw: *mut clk_hw) {
    let composite = to_clk_composite(hw); clk_hw_unregister(hw); kfree(composite as *mut core::ffi::c_void);
}
unsafe fn devm_clk_hw_release_composite(_dev: *mut device, res: *mut core::ffi::c_void) { clk_hw_unregister_composite(*(res as *mut *mut clk_hw)); }

unsafe fn devm_clk_hw_register_composite_pdata(dev: *mut device, name: *const c_char, pdata: *const clk_parent_data, n: i32, mux: *mut clk_hw, mo: *const clk_ops, rate: *mut clk_hw, ro: *const clk_ops, gate: *mut clk_hw, go: *const clk_ops, flags: c_ulong) -> *mut clk_hw {
    let ptr = devres_alloc(Some(devm_clk_hw_release_composite), core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL);
    if ptr.is_null() { return ERR_PTR(-ENOMEM); }
    let hw = clk_hw_register_composite_pdata(dev, name, pdata, n, mux, mo, rate, ro, gate, go, flags);
    if !IS_ERR(hw) { *(ptr as *mut *mut clk_hw) = hw; devres_add(dev, ptr); } else { devres_free(ptr); }
    hw
}

extern "C" {
    fn __clk_hw_register_composite(dev: *mut device, name: *const c_char, parent_names: *const *const c_char, pdata: *const clk_parent_data, num_parents: i32, mux_hw: *mut clk_hw, mux_ops: *const clk_ops, rate_hw: *mut clk_hw, rate_ops: *const clk_ops, gate_hw: *mut clk_hw, gate_ops: *const clk_ops, flags: c_ulong) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
