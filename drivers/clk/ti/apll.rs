// SPDX-License-Identifier: GPL-2.0-only
/* OMAP APLL clock support */

// Linux headers and "clock.h" provide the referenced kernel types, helpers,
// logging functions, and registration interfaces in the containing project.

const APLL_FORCE_LOCK: u32 = 0x1;
const APLL_AUTO_IDLE: u32 = 0x2;
const MAX_APLL_WAIT_TRIES: i32 = 1_000_000;

const OMAP2_EN_APLL_LOCKED: u32 = 0x3;
const OMAP2_EN_APLL_STOPPED: u32 = 0x0;
const OMAP2_APLL_AUTOIDLE_LOW_POWER_STOP: u32 = 0x3;
const OMAP2_APLL_AUTOIDLE_DISABLE: u32 = 0x0;

unsafe fn dra7_apll_enable(hw: *mut clk_hw) -> i32 {
    let clk: *mut clk_hw_omap = to_clk_hw_omap(hw);
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let ad = (*clk).dpll_data;
    let clk_name: *const core::ffi::c_char;
    let mut state: u8 = 1;
    let mut v: u32;

    if ad.is_null() { return -EINVAL; }
    clk_name = clk_hw_get_name(&(*clk).hw);
    state = state.wrapping_shl(__ffs((*ad).idlest_mask));
    v = (*ti_clk_ll_ops).clk_readl(&(*ad).idlest_reg);
    if (v & (*ad).idlest_mask) == state as u32 { return r; }

    v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg);
    v &= !(*ad).enable_mask;
    v |= APLL_FORCE_LOCK << __ffs((*ad).enable_mask);
    (*ti_clk_ll_ops).clk_writel(v, &mut (*ad).control_reg);

    state = state.wrapping_shl(__ffs((*ad).idlest_mask));
    loop {
        v = (*ti_clk_ll_ops).clk_readl(&(*ad).idlest_reg);
        if (v & (*ad).idlest_mask) == state as u32 { break; }
        if i > MAX_APLL_WAIT_TRIES { break; }
        i += 1;
        udelay(1);
    }
    if i == MAX_APLL_WAIT_TRIES {
        pr_warn!("clock: {:?} failed transition to '{}'\n", clk_name, if state != 0 { "locked" } else { "bypassed" });
        r = -EBUSY;
    } else {
        pr_debug!("clock: {:?} transition to '{}' in {} loops\n", clk_name, if state != 0 { "locked" } else { "bypassed" }, i);
    }
    r
}

unsafe fn dra7_apll_disable(hw: *mut clk_hw) {
    let clk = to_clk_hw_omap(hw);
    let ad = (*clk).dpll_data;
    let mut state: u8 = 1;
    state = state.wrapping_shl(__ffs((*ad).idlest_mask));
    let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg);
    v &= !(*ad).enable_mask;
    v |= APLL_AUTO_IDLE << __ffs((*ad).enable_mask);
    (*ti_clk_ll_ops).clk_writel(v, &mut (*ad).control_reg);
}

unsafe fn dra7_apll_is_enabled(hw: *mut clk_hw) -> i32 {
    let ad = (*to_clk_hw_omap(hw)).dpll_data;
    let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg);
    v &= (*ad).enable_mask;
    v >>= __ffs((*ad).enable_mask);
    if v == APLL_AUTO_IDLE { 0 } else { 1 }
}

unsafe fn dra7_init_apll_parent(_hw: *mut clk_hw) -> u8 { 0 }

static apll_ck_ops: clk_ops = clk_ops {
    enable: Some(dra7_apll_enable),
    disable: Some(dra7_apll_disable),
    is_enabled: Some(dra7_apll_is_enabled),
    get_parent: Some(dra7_init_apll_parent),
};

unsafe fn omap_clk_register_apll(user: *mut core::ffi::c_void, node: *mut device_node) {
    let hw = user as *mut clk_hw;
    let clk_hw = to_clk_hw_omap(hw);
    let ad = (*clk_hw).dpll_data;
    let init = (*(*clk_hw).hw.init);
    let mut clk: *mut clk = of_clk_get(node, 0);
    if IS_ERR(clk) {
        pr_debug!("clk-ref for %pOFn not ready, retry\n", node);
        if !ti_clk_retry_init(node, hw, omap_clk_register_apll) { return; }
        goto_cleanup!();
    }
    (*ad).clk_ref = __clk_get_hw(clk);
    clk = of_clk_get(node, 1);
    if IS_ERR(clk) {
        pr_debug!("clk-bypass for %pOFn not ready, retry\n", node);
        if !ti_clk_retry_init(node, hw, omap_clk_register_apll) { return; }
        goto_cleanup!();
    }
    (*ad).clk_bypass = __clk_get_hw(clk);
    let name = ti_dt_clk_name(node);
    clk = of_ti_clk_register_omap_hw(node, &mut (*clk_hw).hw, name);
    if !IS_ERR(clk) {
        of_clk_add_provider(node, of_clk_src_simple_get, clk);
        kfree((*init).parent_names);
        kfree(init);
        return;
    }
    kfree((*clk_hw).dpll_data); kfree((*init).parent_names); kfree(init); kfree(clk_hw);
}

unsafe fn of_dra7_apll_setup(node: *mut device_node) {
    let ad = kzalloc_obj::<dpll_data>();
    let clk_hw = kzalloc_obj::<clk_hw_omap>();
    let init = kzalloc_obj::<clk_init_data>();
    if ad.is_null() || clk_hw.is_null() || init.is_null() { kfree(ad); kfree(clk_hw); kfree(init); return; }
    (*clk_hw).dpll_data = ad; (*clk_hw).hw.init = init;
    (*init).name = ti_dt_clk_name(node); (*init).ops = &apll_ck_ops;
    (*init).num_parents = of_clk_get_parent_count(node);
    if (*init).num_parents < 1 { pr_err!("dra7 apll %pOFn must have parent(s)\n", node); goto_cleanup!(); }
    let parent_names = kcalloc((*init).num_parents, core::mem::size_of::<*const core::ffi::c_char>(), GFP_KERNEL);
    if parent_names.is_null() { goto_cleanup!(); }
    of_clk_parent_fill(node, parent_names, (*init).num_parents); (*init).parent_names = parent_names;
    let mut ret = ti_clk_get_reg_addr(node, 0, &mut (*ad).control_reg);
    ret |= ti_clk_get_reg_addr(node, 1, &mut (*ad).idlest_reg);
    if ret != 0 { goto_cleanup!(); }
    (*ad).idlest_mask = 0x1; (*ad).enable_mask = 0x3;
    omap_clk_register_apll(&mut (*clk_hw).hw as *mut _, node); return;
}

unsafe fn omap2_apll_is_enabled(hw: *mut clk_hw) -> i32 {
    let ad = (*to_clk_hw_omap(hw)).dpll_data;
    let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg) & (*ad).enable_mask;
    v >>= __ffs((*ad).enable_mask);
    if v == OMAP2_EN_APLL_LOCKED { 1 } else { 0 }
}

unsafe fn omap2_apll_recalc(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk = to_clk_hw_omap(hw);
    if omap2_apll_is_enabled(hw) != 0 { (*clk).fixed_rate } else { 0 }
}

unsafe fn omap2_apll_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw); let ad = (*clk).dpll_data; let mut i = 0; let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg);
    v &= !(*ad).enable_mask; v |= OMAP2_EN_APLL_LOCKED << __ffs((*ad).enable_mask); (*ti_clk_ll_ops).clk_writel(v, &mut (*ad).control_reg);
    loop { v = (*ti_clk_ll_ops).clk_readl(&(*ad).idlest_reg); if v & (*ad).idlest_mask != 0 { break; } if i > MAX_APLL_WAIT_TRIES { break; } i += 1; udelay(1); }
    if i == MAX_APLL_WAIT_TRIES { pr_warn!("failed to transition to locked\n"); -EBUSY } else { 0 }
}

unsafe fn omap2_apll_disable(hw: *mut clk_hw) { let ad = (*to_clk_hw_omap(hw)).dpll_data; let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).control_reg); v &= !(*ad).enable_mask; v |= OMAP2_EN_APLL_STOPPED << __ffs((*ad).enable_mask); (*ti_clk_ll_ops).clk_writel(v, &mut (*ad).control_reg); }

unsafe fn omap2_apll_set_autoidle(clk: *mut clk_hw_omap, val: u32) { let ad = (*clk).dpll_data; let mut v = (*ti_clk_ll_ops).clk_readl(&(*ad).autoidle_reg); v &= !(*ad).autoidle_mask; v |= val << __ffs((*ad).autoidle_mask); (*ti_clk_ll_ops).clk_writel(v, &mut (*ad).control_reg); }
unsafe fn omap2_apll_allow_idle(clk: *mut clk_hw_omap) { omap2_apll_set_autoidle(clk, OMAP2_APLL_AUTOIDLE_LOW_POWER_STOP); }
unsafe fn omap2_apll_deny_idle(clk: *mut clk_hw_omap) { omap2_apll_set_autoidle(clk, OMAP2_APLL_AUTOIDLE_DISABLE); }

// The OMAP2 setup path mirrors the C implementation; referenced allocation,
// device-tree, registration, and cleanup helpers remain external dependencies.
unsafe fn of_omap2_apll_setup(node: *mut device_node) {
    let ad = kzalloc_obj::<dpll_data>(); let clk_hw = kzalloc_obj::<clk_hw_omap>(); let init = kzalloc_obj::<clk_init_data>();
    if ad.is_null() || clk_hw.is_null() || init.is_null() { kfree(ad); kfree(clk_hw); kfree(init); return; }
    (*clk_hw).dpll_data = ad; (*clk_hw).hw.init = init; (*init).ops = &omap2_apll_ops; (*init).name = ti_dt_clk_name(node); (*clk_hw).ops = &omap2_apll_hwops;
    (*init).num_parents = of_clk_get_parent_count(node); if (*init).num_parents != 1 { pr_err!("%pOFn must have one parent\n", node); return; }
    let parent_name = of_clk_get_parent_name(node, 0); (*init).parent_names = &parent_name;
    let mut val = 0; if of_property_read_u32(node, "ti,clock-frequency", &mut val) != 0 { pr_err!("%pOFn missing clock-frequency\n", node); return; } (*clk_hw).fixed_rate = val as _;
    (*clk_hw).enable_bit = ti_clk_get_legacy_bit_shift(node); (*ad).enable_mask = 0x3 << (*clk_hw).enable_bit; (*ad).autoidle_mask = 0x3 << (*clk_hw).enable_bit;
    if of_property_read_u32(node, "ti,idlest-shift", &mut val) != 0 { pr_err!("%pOFn missing idlest-shift\n", node); return; } (*ad).idlest_mask = 1 << val;
    let mut ret = ti_clk_get_reg_addr(node, 0, &mut (*ad).control_reg); ret |= ti_clk_get_reg_addr(node, 1, &mut (*ad).autoidle_reg); ret |= ti_clk_get_reg_addr(node, 2, &mut (*ad).idlest_reg); if ret != 0 { return; }
    let clk = of_ti_clk_register_omap_hw(node, &mut (*clk_hw).hw, ti_dt_clk_name(node)); if !IS_ERR(clk) { of_clk_add_provider(node, of_clk_src_simple_get, clk); kfree(init); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
