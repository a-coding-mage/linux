// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

const CCU_MUX_KEY_VALUE: u32 = 0x16aa0000;

unsafe fn ccu_mux_get_prediv(
    common: *mut ccu_common,
    cm: *mut ccu_mux_internal,
    mut parent_index: i32,
) -> u16 {
    let mut prediv: u16 = 1;
    let mut reg: u32;

    if !((*common).features & CCU_FEATURE_FIXED_PREDIV != 0
        || (*common).features & CCU_FEATURE_VARIABLE_PREDIV != 0
        || (*common).features & CCU_FEATURE_ALL_PREDIV != 0)
    {
        return 1;
    }

    if (*common).features & CCU_FEATURE_ALL_PREDIV != 0 {
        return (*common).prediv;
    }

    reg = readl((*common).base.add((*common).reg as usize));
    if parent_index < 0 {
        parent_index = (reg >> (*cm).shift) as i32;
        parent_index &= ((1u32 << (*cm).width) - 1) as i32;
    }

    if (*common).features & CCU_FEATURE_FIXED_PREDIV != 0 {
        for i in 0..(*cm).n_predivs {
            if parent_index == (*cm).fixed_predivs.add(i as usize).read().index as i32 {
                prediv = (*cm).fixed_predivs.add(i as usize).read().div;
            }
        }
    }

    if (*common).features & CCU_FEATURE_VARIABLE_PREDIV != 0 {
        for i in 0..(*cm).n_var_predivs {
            let var_prediv = (*cm).var_predivs.add(i as usize).read();
            if parent_index == var_prediv.index as i32 {
                let mut div: u8 = (reg >> var_prediv.shift) as u8;
                div &= ((1u32 << var_prediv.width) - 1) as u8;
                prediv = div.wrapping_add(1) as u16;
            }
        }
    }

    prediv
}

pub unsafe fn ccu_mux_helper_apply_prediv(
    common: *mut ccu_common,
    cm: *mut ccu_mux_internal,
    parent_index: i32,
    parent_rate: usize,
) -> usize {
    parent_rate / ccu_mux_get_prediv(common, cm, parent_index) as usize
}

unsafe fn ccu_mux_helper_unapply_prediv(
    common: *mut ccu_common,
    cm: *mut ccu_mux_internal,
    parent_index: i32,
    parent_rate: usize,
) -> usize {
    parent_rate * ccu_mux_get_prediv(common, cm, parent_index) as usize
}

pub unsafe fn ccu_mux_helper_determine_rate(
    common: *mut ccu_common,
    cm: *mut ccu_mux_internal,
    req: *mut clk_rate_request,
    round: Option<unsafe extern "C" fn(*mut ccu_mux_internal, *mut clk_rate_request, *mut core::ffi::c_void) -> i32>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut best_parent_rate: usize = 0;
    let mut best_rate: usize = 0;
    let mut best_parent: *mut clk_hw;
    let hw: *mut clk_hw = &mut (*common).hw;
    let mut ret: i32;

    if clk_hw_get_flags(hw) & CLK_SET_RATE_NO_REPARENT != 0 {
        let mut adj_req = req.read();
        best_parent = clk_hw_get_parent(hw);
        adj_req.best_parent_rate = clk_hw_get_rate(best_parent);
        adj_req.best_parent_hw = best_parent;
        adj_req.rate = ccu_mux_helper_unapply_prediv(common, cm, -1, (*req).rate);
        ret = round.unwrap()(cm, &mut adj_req, data);
        if ret != 0 { return ret; }
        best_parent_rate = adj_req.best_parent_rate;
        best_rate = ccu_mux_helper_apply_prediv(common, cm, -1, adj_req.rate);
    } else {
        best_parent = core::ptr::null_mut();
        for i in 0..clk_hw_get_num_parents(hw) {
            let mut tmp_req = req.read();
            let parent = clk_hw_get_parent_by_index(hw, i);
            if parent.is_null() { continue; }
            tmp_req.best_parent_hw = parent;
            tmp_req.best_parent_rate = clk_hw_get_rate(parent);
            tmp_req.rate = ccu_mux_helper_unapply_prediv(common, cm, i as i32, (*req).rate);
            ret = round.unwrap()(cm, &mut tmp_req, data);
            if ret != 0 { continue; }
            let rate = ccu_mux_helper_apply_prediv(common, cm, i as i32, tmp_req.rate);
            if rate == (*req).rate {
                best_parent = parent;
                best_parent_rate = tmp_req.best_parent_rate;
                best_rate = rate;
                break;
            }
            if ccu_is_better_rate(common, (*req).rate, rate, best_rate) != 0 {
                best_rate = rate;
                best_parent_rate = tmp_req.best_parent_rate;
                best_parent = parent;
            }
        }
        if best_rate == 0 { return -EINVAL; }
    }

    (*req).best_parent_hw = best_parent;
    (*req).best_parent_rate = best_parent_rate;
    (*req).rate = best_rate;
    0
}

pub unsafe fn ccu_mux_helper_get_parent(common: *mut ccu_common, cm: *mut ccu_mux_internal) -> u8 {
    let reg = readl((*common).base.add((*common).reg as usize));
    let mut parent = (reg >> (*cm).shift) as u8;
    parent &= ((1u32 << (*cm).width) - 1) as u8;
    if !(*cm).table.is_null() {
        let num_parents = clk_hw_get_num_parents(&mut (*common).hw);
        for i in 0..num_parents {
            if (*cm).table.add(i as usize).read() == parent { return i as u8; }
        }
    }
    parent
}

pub unsafe fn ccu_mux_helper_set_parent(common: *mut ccu_common, cm: *mut ccu_mux_internal, mut index: u8) -> i32 {
    let mut flags: usize = 0;
    if !(*cm).table.is_null() { index = (*cm).table.add(index as usize).read(); }
    spin_lock_irqsave((*common).lock, &mut flags);
    let mut reg = readl((*common).base.add((*common).reg as usize));
    if (*common).features & CCU_FEATURE_KEY_FIELD != 0 { reg |= CCU_MUX_KEY_VALUE; }
    if (*common).features & CCU_FEATURE_UPDATE_BIT != 0 { reg |= CCU_SUNXI_UPDATE_BIT; }
    reg &= !genmask((*cm).width + (*cm).shift - 1, (*cm).shift);
    writel(reg | ((index as u32) << (*cm).shift), (*common).base.add((*common).reg as usize));
    spin_unlock_irqrestore((*common).lock, flags);
    0
}

unsafe fn ccu_mux_disable(hw: *mut clk_hw) {
    let cm = hw_to_ccu_mux(hw);
    ccu_gate_helper_disable(&mut (*cm).common, (*cm).enable);
}

unsafe fn ccu_mux_enable(hw: *mut clk_hw) -> i32 {
    let cm = hw_to_ccu_mux(hw);
    ccu_gate_helper_enable(&mut (*cm).common, (*cm).enable)
}

unsafe fn ccu_mux_is_enabled(hw: *mut clk_hw) -> i32 {
    let cm = hw_to_ccu_mux(hw);
    ccu_gate_helper_is_enabled(&mut (*cm).common, (*cm).enable)
}

unsafe fn ccu_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let cm = hw_to_ccu_mux(hw);
    ccu_mux_helper_get_parent(&mut (*cm).common, &mut (*cm).mux)
}

unsafe fn ccu_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let cm = hw_to_ccu_mux(hw);
    ccu_mux_helper_set_parent(&mut (*cm).common, &mut (*cm).mux, index)
}

unsafe fn ccu_mux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let cm = hw_to_ccu_mux(hw);
    if (*cm).common.features & CCU_FEATURE_CLOSEST_RATE != 0 {
        return clk_mux_determine_rate_flags(hw, req, CLK_MUX_ROUND_CLOSEST);
    }
    clk_mux_determine_rate_flags(hw, req, 0)
}

unsafe fn ccu_mux_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let cm = hw_to_ccu_mux(hw);
    ccu_mux_helper_apply_prediv(&mut (*cm).common, &mut (*cm).mux, -1, parent_rate)
}

pub static ccu_mux_ops: clk_ops = clk_ops {
    disable: Some(ccu_mux_disable), enable: Some(ccu_mux_enable), is_enabled: Some(ccu_mux_is_enabled),
    get_parent: Some(ccu_mux_get_parent), set_parent: Some(ccu_mux_set_parent),
    determine_rate: Some(ccu_mux_determine_rate), recalc_rate: Some(ccu_mux_recalc_rate),
};

/* This clock notifier temporarily switches to a stable parent while a PLL stabilizes. */
unsafe fn ccu_mux_notifier_cb(nb: *mut notifier_block, event: usize, _data: *mut core::ffi::c_void) -> i32 {
    let mux = to_ccu_mux_nb(nb);
    let mut ret = 0;
    if event == PRE_RATE_CHANGE {
        (*mux).original_index = ccu_mux_helper_get_parent((*mux).common, (*mux).cm);
        ret = ccu_mux_helper_set_parent((*mux).common, (*mux).cm, (*mux).bypass_index);
    } else if event == POST_RATE_CHANGE {
        ret = ccu_mux_helper_set_parent((*mux).common, (*mux).cm, (*mux).original_index);
    }
    udelay((*mux).delay_us);
    notifier_from_errno(ret)
}

pub unsafe fn ccu_mux_notifier_register(clk: *mut clk, mux_nb: *mut ccu_mux_nb) -> i32 {
    (*mux_nb).clk_nb.notifier_call = Some(ccu_mux_notifier_cb);
    clk_notifier_register(clk, &mut (*mux_nb).clk_nb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
