// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
struct SunxiCcu {
    desc: *const sunxi_ccu_desc,
    lock: spinlock_t,
    reset: ccu_reset,
}

pub unsafe extern "C" fn ccu_helper_wait_for_lock(common: *mut ccu_common, lock: u32) {
    let addr: *mut core::ffi::c_void;
    let mut reg: u32 = 0;

    if lock == 0 {
        return;
    }

    if (*common).features & CCU_FEATURE_LOCK_REG != 0 {
        addr = ((*common).base as *mut u8).add((*common).lock_reg as usize) as *mut core::ffi::c_void;
    } else {
        addr = ((*common).base as *mut u8).add((*common).reg as usize) as *mut core::ffi::c_void;
    }

    WARN_ON(readl_relaxed_poll_timeout(addr, &mut reg, reg & lock != 0, 100, 70000));
}

pub unsafe extern "C" fn ccu_is_better_rate(
    common: *mut ccu_common,
    target_rate: c_ulong,
    current_rate: c_ulong,
    best_rate: c_ulong,
) -> bool {
    let (mut min_rate, mut max_rate) = (0 as c_ulong, 0 as c_ulong);

    clk_hw_get_rate_range(&mut (*common).hw, &mut min_rate, &mut max_rate);

    if current_rate > max_rate || current_rate < min_rate {
        return false;
    }

    if (*common).features & CCU_FEATURE_CLOSEST_RATE != 0 {
        return current_rate.abs_diff(target_rate) < best_rate.abs_diff(target_rate);
    }

    current_rate <= target_rate && current_rate > best_rate
}

/*
 * This clock notifier is called when the frequency of a PLL clock is
 * changed. In common PLL designs, changes to the dividers take effect
 * almost immediately, while changes to the multipliers (implemented
 * as dividers in the feedback loop) take a few cycles to work into the
 * feedback loop for the PLL to stabilize.
 *
 * Sometimes when the PLL clock rate is changed, the decrease in the
 * divider is too much for the decrease in the multiplier to catch up.
 * The PLL clock rate will spike, and in some cases, might lock up
 * completely.
 *
 * This notifier callback will gate and then ungate the clock,
 * effectively resetting it, so it proceeds to work. Care must be
 * taken to reparent consumers to other temporary clocks during the
 * rate change, and that this notifier callback must be the first
 * to be registered.
 */
unsafe extern "C" fn ccu_pll_notifier_cb(nb: *mut notifier_block, event: c_ulong, _data: *mut core::ffi::c_void) -> c_int {
    let pll = to_ccu_pll_nb(nb);
    let mut ret: c_int = 0;

    if event != POST_RATE_CHANGE {
        return notifier_from_errno(ret);
    }

    ccu_gate_helper_disable((*pll).common, (*pll).enable);
    ret = ccu_gate_helper_enable((*pll).common, (*pll).enable);
    if ret != 0 {
        return notifier_from_errno(ret);
    }

    ccu_helper_wait_for_lock((*pll).common, (*pll).lock);
    notifier_from_errno(ret)
}

pub unsafe extern "C" fn ccu_pll_notifier_register(pll_nb: *mut ccu_pll_nb) -> c_int {
    (*pll_nb).clk_nb.notifier_call = Some(ccu_pll_notifier_cb);
    clk_notifier_register((*(*pll_nb).common).hw.clk, &mut (*pll_nb).clk_nb)
}

unsafe fn sunxi_ccu_probe(ccu: *mut SunxiCcu, dev: *mut device, node: *mut device_node, reg: *mut core::ffi::c_void, desc: *const sunxi_ccu_desc) -> c_int {
    let mut reset: *mut ccu_reset;
    let mut ret: c_int;
    (*ccu).desc = desc;
    spin_lock_init(&mut (*ccu).lock);

    for i in 0..(*desc).num_ccu_clks {
        let cclk = *(*desc).ccu_clks.add(i as usize);
        if cclk.is_null() { continue; }
        (*cclk).base = reg;
        (*cclk).lock = &mut (*ccu).lock;
    }

    let mut j: c_int = 0;
    while j < (*(*desc).hw_clks).num {
        let hw = *(*(*desc).hw_clks).hws.add(j as usize);
        j += 1;
        if hw.is_null() { continue; }
        let name = (*(*hw).init).name;
        ret = if !dev.is_null() { clk_hw_register(dev, hw) } else { of_clk_hw_register(node, hw) };
        if ret != 0 { pr_err("Couldn't register clock %d - %s\n", j - 1, name); break; }
    }
    if j <= (*(*desc).hw_clks).num {
        if ret != 0 { while j > 0 { j -= 1; let hw = *(*(*desc).hw_clks).hws.add(j as usize); if !hw.is_null() { clk_hw_unregister(hw); } } return ret; }
    }

    for i in 0..(*desc).num_ccu_clks {
        let cclk = *(*desc).ccu_clks.add(i as usize);
        if cclk.is_null() { continue; }
        if (*cclk).max_rate != 0 { clk_hw_set_rate_range(&mut (*cclk).hw, (*cclk).min_rate, (*cclk).max_rate); }
        else if (*cclk).min_rate != 0 { WARN((*cclk).min_rate, "No max_rate, ignoring min_rate of clock %d - %s\n", i, clk_hw_get_name(&mut (*cclk).hw)); }
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, (*desc).hw_clks);
    if ret != 0 { while j > 0 { j -= 1; let hw = *(*(*desc).hw_clks).hws.add(j as usize); if !hw.is_null() { clk_hw_unregister(hw); } } return ret; }
    reset = &mut (*ccu).reset;
    (*reset).rcdev.of_node = node; (*reset).rcdev.ops = &ccu_reset_ops; (*reset).rcdev.owner = if !dev.is_null() { (*(*dev).driver).owner } else { THIS_MODULE };
    (*reset).rcdev.nr_resets = (*desc).num_resets; (*reset).base = reg; (*reset).lock = &mut (*ccu).lock; (*reset).reset_map = (*desc).resets;
    ret = reset_controller_register(&mut (*reset).rcdev);
    if ret != 0 { of_clk_del_provider(node); while j > 0 { j -= 1; let hw = *(*(*desc).hw_clks).hws.add(j as usize); if !hw.is_null() { clk_hw_unregister(hw); } } }
    ret
}

unsafe extern "C" fn devm_sunxi_ccu_release(dev: *mut device, res: *mut core::ffi::c_void) {
    let ccu = res as *mut SunxiCcu;
    let desc = (*ccu).desc;
    reset_controller_unregister(&mut (*ccu).reset.rcdev);
    of_clk_del_provider((*dev).of_node);
    for i in 0..(*(*desc).hw_clks).num { let hw = *(*(*desc).hw_clks).hws.add(i as usize); if !hw.is_null() { clk_hw_unregister(hw); } }
}

pub unsafe extern "C" fn devm_sunxi_ccu_probe(dev: *mut device, reg: *mut core::ffi::c_void, desc: *const sunxi_ccu_desc) -> c_int {
    let ccu = devres_alloc(Some(devm_sunxi_ccu_release), core::mem::size_of::<SunxiCcu>(), GFP_KERNEL);
    if ccu.is_null() { return -ENOMEM; }
    let ret = sunxi_ccu_probe(ccu as *mut SunxiCcu, dev, (*dev).of_node, reg, desc);
    if ret != 0 { devres_free(ccu); return ret; }
    devres_add(dev, ccu);
    0
}

pub unsafe extern "C" fn of_sunxi_ccu_probe(node: *mut device_node, reg: *mut core::ffi::c_void, desc: *const sunxi_ccu_desc) {
    let ccu = kzalloc_obj::<SunxiCcu>();
    if ccu.is_null() { return; }
    let ret = sunxi_ccu_probe(ccu, core::ptr::null_mut(), node, reg, desc);
    if ret != 0 { pr_err("%pOF: probing clocks failed: %d\n", node, ret); kfree(ccu as *mut core::ffi::c_void); }
}

// MODULE_DESCRIPTION("Common clock support for Allwinner SoCs");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
