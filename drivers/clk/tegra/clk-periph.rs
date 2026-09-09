// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel clock implementation.

unsafe fn clk_periph_get_parent(hw: *mut clk_hw) -> u8 {
    let periph = to_clk_periph(hw);
    let mux_ops = (*periph).mux_ops;
    let mux_hw = &mut (*periph).mux.hw as *mut clk_hw;

    __clk_hw_set_clk(mux_hw, hw);
    ((*mux_ops).get_parent)(mux_hw)
}

unsafe fn clk_periph_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let periph = to_clk_periph(hw);
    let mux_ops = (*periph).mux_ops;
    let mux_hw = &mut (*periph).mux.hw as *mut clk_hw;

    __clk_hw_set_clk(mux_hw, hw);
    ((*mux_ops).set_parent)(mux_hw, index)
}

unsafe fn clk_periph_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let periph = to_clk_periph(hw);
    let div_ops = (*periph).div_ops;
    let div_hw = &mut (*periph).divider.hw as *mut clk_hw;

    __clk_hw_set_clk(div_hw, hw);
    ((*div_ops).recalc_rate)(div_hw, parent_rate)
}

unsafe fn clk_periph_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let periph = to_clk_periph(hw);
    let div_ops = (*periph).div_ops;
    let div_hw = &mut (*periph).divider.hw as *mut clk_hw;

    __clk_hw_set_clk(div_hw, hw);
    ((*div_ops).determine_rate)(div_hw, req)
}

unsafe fn clk_periph_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let periph = to_clk_periph(hw);
    let div_ops = (*periph).div_ops;
    let div_hw = &mut (*periph).divider.hw as *mut clk_hw;

    __clk_hw_set_clk(div_hw, hw);
    ((*div_ops).set_rate)(div_hw, rate, parent_rate)
}

unsafe fn clk_periph_is_enabled(hw: *mut clk_hw) -> i32 {
    let periph = to_clk_periph(hw);
    let gate_ops = (*periph).gate_ops;
    let gate_hw = &mut (*periph).gate.hw as *mut clk_hw;

    __clk_hw_set_clk(gate_hw, hw);
    ((*gate_ops).is_enabled)(gate_hw)
}

unsafe fn clk_periph_enable(hw: *mut clk_hw) -> i32 {
    let periph = to_clk_periph(hw);
    let gate_ops = (*periph).gate_ops;
    let gate_hw = &mut (*periph).gate.hw as *mut clk_hw;

    __clk_hw_set_clk(gate_hw, hw);
    ((*gate_ops).enable)(gate_hw)
}

unsafe fn clk_periph_disable(hw: *mut clk_hw) {
    let periph = to_clk_periph(hw);
    let gate_ops = (*periph).gate_ops;
    let gate_hw = &mut (*periph).gate.hw as *mut clk_hw;
    ((*gate_ops).disable)(gate_hw);
}

unsafe fn clk_periph_disable_unused(hw: *mut clk_hw) {
    let periph = to_clk_periph(hw);
    let gate_ops = (*periph).gate_ops;
    let gate_hw = &mut (*periph).gate.hw as *mut clk_hw;
    ((*gate_ops).disable_unused)(gate_hw);
}

unsafe fn clk_periph_restore_context(hw: *mut clk_hw) {
    let periph = to_clk_periph(hw);
    let div_ops = (*periph).div_ops;
    let div_hw = &mut (*periph).divider.hw as *mut clk_hw;
    let parent_id = clk_hw_get_parent_index(hw);

    if WARN_ON(parent_id < 0) {
        return;
    }
    if ((*periph).gate.flags & TEGRA_PERIPH_NO_DIV) == 0 {
        ((*div_ops).restore_context)(div_hw);
    }
    clk_periph_set_parent(hw, parent_id as u8);
}

static tegra_clk_periph_ops: clk_ops = clk_ops {
    get_parent: Some(clk_periph_get_parent), set_parent: Some(clk_periph_set_parent),
    recalc_rate: Some(clk_periph_recalc_rate), determine_rate: Some(clk_periph_determine_rate),
    set_rate: Some(clk_periph_set_rate), is_enabled: Some(clk_periph_is_enabled),
    enable: Some(clk_periph_enable), disable: Some(clk_periph_disable),
    disable_unused: Some(clk_periph_disable_unused), restore_context: Some(clk_periph_restore_context),
};

static tegra_clk_periph_nodiv_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent), get_parent: Some(clk_periph_get_parent),
    set_parent: Some(clk_periph_set_parent), is_enabled: Some(clk_periph_is_enabled),
    enable: Some(clk_periph_enable), disable: Some(clk_periph_disable),
    disable_unused: Some(clk_periph_disable_unused), restore_context: Some(clk_periph_restore_context),
};

static tegra_clk_periph_no_gate_ops: clk_ops = clk_ops {
    get_parent: Some(clk_periph_get_parent), set_parent: Some(clk_periph_set_parent),
    recalc_rate: Some(clk_periph_recalc_rate), determine_rate: Some(clk_periph_determine_rate),
    set_rate: Some(clk_periph_set_rate), restore_context: Some(clk_periph_restore_context),
};

unsafe fn _tegra_clk_register_periph(name: *const c_char, parent_names: *const *const c_char,
    num_parents: i32, periph: *mut tegra_clk_periph, clk_base: *mut c_void,
    offset: u32, mut flags: c_ulong) -> *mut clk {
    let div = ((*periph).gate.flags & TEGRA_PERIPH_NO_DIV) == 0;
    if ((*periph).gate.flags & TEGRA_PERIPH_NO_DIV) != 0 {
        flags |= CLK_SET_RATE_PARENT;
        (*periph).hw.init.ops = &tegra_clk_periph_nodiv_ops;
    } else if ((*periph).gate.flags & TEGRA_PERIPH_NO_GATE) != 0 {
        (*periph).hw.init.ops = &tegra_clk_periph_no_gate_ops;
    } else {
        (*periph).hw.init.ops = &tegra_clk_periph_ops;
    }
    (*periph).hw.init.name = name;
    (*periph).hw.init.flags = flags;
    (*periph).hw.init.parent_names = parent_names;
    (*periph).hw.init.num_parents = num_parents;
    let bank = get_reg_bank((*periph).gate.clk_num);
    if bank.is_null() { return ERR_PTR(-EINVAL); }
    (*periph).hw.init = &mut (*periph).init;
    (*periph).magic = TEGRA_CLK_PERIPH_MAGIC;
    (*periph).mux.reg = (clk_base as *mut u8).add(offset as usize) as *mut c_void;
    (*periph).divider.reg = if div { (*periph).mux.reg } else { core::ptr::null_mut() };
    (*periph).gate.clk_base = clk_base;
    (*periph).gate.regs = bank;
    (*periph).gate.enable_refcnt = periph_clk_enb_refcnt;
    let clk = clk_register(core::ptr::null_mut(), &mut (*periph).hw);
    if IS_ERR(clk) { return clk; }
    (*periph).mux.hw.clk = clk;
    (*periph).divider.hw.clk = if div { clk } else { core::ptr::null_mut() };
    (*periph).gate.hw.clk = clk;
    clk
}

pub unsafe fn tegra_clk_register_periph(name: *const c_char, parent_names: *const *const c_char,
    num_parents: i32, periph: *mut tegra_clk_periph, clk_base: *mut c_void,
    offset: u32, flags: c_ulong) -> *mut clk {
    _tegra_clk_register_periph(name, parent_names, num_parents, periph, clk_base, offset, flags)
}

pub unsafe fn tegra_clk_register_periph_nodiv(name: *const c_char, parent_names: *const *const c_char,
    num_parents: i32, periph: *mut tegra_clk_periph, clk_base: *mut c_void, offset: u32) -> *mut clk {
    (*periph).gate.flags |= TEGRA_PERIPH_NO_DIV;
    _tegra_clk_register_periph(name, parent_names, num_parents, periph, clk_base, offset, CLK_SET_RATE_PARENT)
}

pub unsafe fn tegra_clk_register_periph_data(clk_base: *mut c_void, init: *mut tegra_periph_init_data) -> *mut clk {
    _tegra_clk_register_periph((*init).name, (*init).p.parent_names, (*init).num_parents,
        &mut (*init).periph, clk_base, (*init).offset, (*init).flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
