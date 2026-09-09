// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018 NXP.
 *   Dong Aisheng <aisheng.dong@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct ClkDividerGate {
    pub divider: ClkDivider,
    pub cached_val: u32,
}

#[inline]
unsafe fn to_clk_divider_gate(hw: *mut ClkHw) -> *mut ClkDividerGate {
    let div = to_clk_divider(hw);
    container_of!(div, ClkDividerGate, divider)
}

unsafe fn clk_divider_gate_recalc_rate_ro(
    hw: *mut ClkHw,
    parent_rate: c_ulong,
) -> c_ulong {
    let div = to_clk_divider(hw);
    let mut val: c_uint;

    val = readl((*div).reg) >> (*div).shift;
    val &= clk_div_mask((*div).width);
    if val == 0 {
        return 0;
    }

    divider_recalc_rate(
        hw,
        parent_rate,
        val,
        (*div).table,
        (*div).flags,
        (*div).width,
    )
}

unsafe fn clk_divider_gate_recalc_rate(
    hw: *mut ClkHw,
    parent_rate: c_ulong,
) -> c_ulong {
    let div_gate = to_clk_divider_gate(hw);
    let div = to_clk_divider(hw);
    let mut flags: c_ulong = 0;
    let val: c_uint;

    spin_lock_irqsave((*div).lock, &mut flags);

    if !clk_hw_is_enabled(hw) {
        val = (*div_gate).cached_val;
    } else {
        val = readl((*div).reg) >> (*div).shift;
        val &= clk_div_mask((*div).width);
    }

    spin_unlock_irqrestore((*div).lock, flags);

    if val == 0 {
        return 0;
    }

    divider_recalc_rate(
        hw,
        parent_rate,
        val,
        (*div).table,
        (*div).flags,
        (*div).width,
    )
}

unsafe fn clk_divider_determine_rate(
    hw: *mut ClkHw,
    req: *mut ClkRateRequest,
) -> c_int {
    ((*clk_divider_ops).determine_rate)(hw, req)
}

unsafe fn clk_divider_gate_set_rate(
    hw: *mut ClkHw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let div_gate = to_clk_divider_gate(hw);
    let div = to_clk_divider(hw);
    let mut flags: c_ulong = 0;
    let value: c_int;
    let mut val: u32;

    value = divider_get_val(rate, parent_rate, (*div).table, (*div).width, (*div).flags);
    if value < 0 {
        return value;
    }

    spin_lock_irqsave((*div).lock, &mut flags);

    if clk_hw_is_enabled(hw) {
        val = readl((*div).reg);
        val &= !(clk_div_mask((*div).width) << (*div).shift);
        val |= (value as u32) << (*div).shift;
        writel(val, (*div).reg);
    } else {
        (*div_gate).cached_val = value as u32;
    }

    spin_unlock_irqrestore((*div).lock, flags);

    0
}

unsafe fn clk_divider_enable(hw: *mut ClkHw) -> c_int {
    let div_gate = to_clk_divider_gate(hw);
    let div = to_clk_divider(hw);
    let mut flags: c_ulong = 0;
    let mut val: u32;

    if (*div_gate).cached_val == 0 {
        pr_err!("%s: no valid preset rate\n", clk_hw_get_name(hw));
        return -EINVAL;
    }

    spin_lock_irqsave((*div).lock, &mut flags);
    // restore div val
    val = readl((*div).reg);
    val |= (*div_gate).cached_val << (*div).shift;
    writel(val, (*div).reg);

    spin_unlock_irqrestore((*div).lock, flags);

    0
}

unsafe fn clk_divider_disable(hw: *mut ClkHw) {
    let div_gate = to_clk_divider_gate(hw);
    let div = to_clk_divider(hw);
    let mut flags: c_ulong = 0;
    let mut val: u32;

    spin_lock_irqsave((*div).lock, &mut flags);

    // store the current div val
    val = readl((*div).reg) >> (*div).shift;
    val &= clk_div_mask((*div).width);
    (*div_gate).cached_val = val;
    writel(0, (*div).reg);

    spin_unlock_irqrestore((*div).lock, flags);
}

unsafe fn clk_divider_is_enabled(hw: *mut ClkHw) -> c_int {
    let div = to_clk_divider(hw);
    let mut val: u32;

    val = readl((*div).reg) >> (*div).shift;
    val &= clk_div_mask((*div).width);

    if val != 0 { 1 } else { 0 }
}

static CLK_DIVIDER_GATE_RO_OPS: ClkOps = ClkOps {
    recalc_rate: Some(clk_divider_gate_recalc_rate_ro),
    determine_rate: Some(clk_divider_determine_rate),
};

static CLK_DIVIDER_GATE_OPS: ClkOps = ClkOps {
    recalc_rate: Some(clk_divider_gate_recalc_rate),
    determine_rate: Some(clk_divider_determine_rate),
    set_rate: Some(clk_divider_gate_set_rate),
    enable: Some(clk_divider_enable),
    disable: Some(clk_divider_disable),
    is_enabled: Some(clk_divider_is_enabled),
};

/*
 * NOTE: In order to reuse the most code from the common divider,
 * we also design our divider following the way that provids an extra
 * clk_divider_flags, however it's fixed to CLK_DIVIDER_ONE_BASED by
 * default as our HW is. Besides that it supports only CLK_DIVIDER_READ_ONLY
 * flag which can be specified by user flexibly.
 */
pub unsafe fn imx_clk_hw_divider_gate(
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut c_void,
    shift: u8,
    width: u8,
    clk_divider_flags: u8,
    table: *const ClkDivTable,
    lock: *mut Spinlock,
) -> *mut ClkHw {
    let mut init: ClkInitData = core::mem::zeroed();
    let div_gate: *mut ClkDividerGate;
    let hw: *mut ClkHw;
    let mut val: u32;
    let ret: c_int;

    div_gate = kzalloc_obj!();
    if div_gate.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    if clk_divider_flags & CLK_DIVIDER_READ_ONLY != 0 {
        init.ops = &CLK_DIVIDER_GATE_RO_OPS;
    } else {
        init.ops = &CLK_DIVIDER_GATE_OPS;
    }
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*div_gate).divider.reg = reg;
    (*div_gate).divider.shift = shift;
    (*div_gate).divider.width = width;
    (*div_gate).divider.lock = lock;
    (*div_gate).divider.table = table;
    (*div_gate).divider.hw.init = &init;
    (*div_gate).divider.flags = CLK_DIVIDER_ONE_BASED | clk_divider_flags;
    // cache gate status
    val = readl(reg) >> shift;
    val &= clk_div_mask(width);
    (*div_gate).cached_val = val;

    hw = &mut (*div_gate).divider.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(div_gate);
        return ERR_PTR(ret);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
