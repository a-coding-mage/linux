// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7790 Common Clock Framework support
 *
 * Copyright (C) 2013  Renesas Solutions Corp.
 *
 * Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

// Linux dependencies are supplied by the surrounding translation unit.

const CPG_DIV6_CKSTP: u32 = 1 << 8;
const CPG_DIV6_DIV_MASK: u32 = 0x3f;

#[inline]
const fn cpg_div6_div(d: u32) -> u32 { d & 0x3f }

/**
 * struct div6_clock - CPG 6 bit divider clock
 * @hw: handle between common and hardware-specific interfaces
 * @reg: IO-remapped register
 * @div: divisor value (1-64)
 * @src_mask: Bitmask covering the register bits to select the parent clock
 * @nb: Notifier block to save/restore clock state for system resume
 * @parents: Array to map from valid parent clocks indices to hardware indices
 */
#[repr(C)]
struct div6_clock {
    hw: clk_hw,
    reg: *mut core::ffi::c_void,
    div: u32,
    src_mask: u32,
    nb: notifier_block,
    parents: [u8; 0],
}

unsafe fn to_div6_clock<'a>(hw: *mut clk_hw) -> &'a mut div6_clock {
    &mut *((hw as *mut u8).sub(core::mem::offset_of!(div6_clock, hw)) as *mut div6_clock)
}

unsafe fn cpg_div6_clock_enable(hw: *mut clk_hw) -> i32 {
    let clock = to_div6_clock(hw);
    let val = (readl(clock.reg) & !(CPG_DIV6_DIV_MASK | CPG_DIV6_CKSTP))
        | cpg_div6_div(clock.div.wrapping_sub(1));
    writel(val, clock.reg);
    0
}

unsafe fn cpg_div6_clock_disable(hw: *mut clk_hw) {
    let clock = to_div6_clock(hw);
    let mut val = readl(clock.reg);
    val |= CPG_DIV6_CKSTP;
    /*
     * DIV6 clocks require the divisor field to be non-zero when stopping
     * the clock. However, some clocks (e.g. ZB on sh73a0) fail to be
     * re-enabled later if the divisor field is changed when stopping the
     * clock
     */
    if val & CPG_DIV6_DIV_MASK == 0 { val |= CPG_DIV6_DIV_MASK; }
    writel(val, clock.reg);
}

unsafe fn cpg_div6_clock_is_enabled(hw: *mut clk_hw) -> i32 {
    let clock = to_div6_clock(hw);
    (readl(clock.reg) & CPG_DIV6_CKSTP == 0) as i32
}

unsafe fn cpg_div6_clock_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    to_div6_clock(hw).div as c_ulong; parent_rate / to_div6_clock(hw).div as c_ulong
}

unsafe fn cpg_div6_clock_calc_div(mut rate: c_ulong, parent_rate: c_ulong) -> u32 {
    if rate == 0 { rate = 1; }
    let div = (parent_rate / rate) as u32;
    div.clamp(1, 64)
}

unsafe fn cpg_div6_clock_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let num_parents = clk_hw_get_num_parents(hw);
    let mut best_parent: *mut clk_hw = core::ptr::null_mut();
    let mut best_rate = 0 as c_ulong;
    let mut best_prate = 0 as c_ulong;
    let mut min_diff = c_ulong::MAX;
    for i in 0..num_parents {
        let parent = clk_hw_get_parent_by_index(hw, i);
        if parent.is_null() { continue; }
        let prate = clk_hw_get_rate(parent);
        if prate == 0 { continue; }
        let min_div = ((prate + (*req).max_rate - 1) / (*req).max_rate).max(1) as u32;
        let max_div = if (*req).min_rate != 0 { (prate / (*req).min_rate).min(64) as u32 } else { 64 };
        if max_div < min_div { continue; }
        let div = cpg_div6_clock_calc_div((*req).rate, prate).clamp(min_div, max_div);
        let calc_rate = prate / div as c_ulong;
        let diff = calc_rate.abs_diff((*req).rate);
        if diff < min_diff { best_rate = calc_rate; best_parent = parent; best_prate = prate; min_diff = diff; }
    }
    if best_parent.is_null() { return -22; }
    (*req).best_parent_rate = best_prate;
    (*req).best_parent_hw = best_parent;
    (*req).rate = best_rate;
    0
}

unsafe fn cpg_div6_clock_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let clock = to_div6_clock(hw);
    clock.div = cpg_div6_clock_calc_div(rate, parent_rate);
    let val = readl(clock.reg) & !CPG_DIV6_DIV_MASK;
    /* Only program the new divisor if the clock isn't stopped. */
    if val & CPG_DIV6_CKSTP == 0 { writel(val | cpg_div6_div(clock.div - 1), clock.reg); }
    0
}

unsafe fn cpg_div6_clock_get_parent(hw: *mut clk_hw) -> u8 {
    let clock = to_div6_clock(hw);
    if clock.src_mask == 0 { return 0; }
    let hw_index = field_get(clock.src_mask, readl(clock.reg)) as u8;
    for i in 0..clk_hw_get_num_parents(hw) { if *clock.parents.as_ptr().add(i as usize) == hw_index { return i as u8; } }
    pr_err!("%s: %s DIV6 clock set to invalid parent %u\n", "cpg_div6_clock_get_parent", clk_hw_get_name(hw), hw_index);
    0
}

unsafe fn cpg_div6_clock_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let clock = to_div6_clock(hw);
    if index as u32 >= clk_hw_get_num_parents(hw) { return -22; }
    let src = field_prep(clock.src_mask, *clock.parents.as_ptr().add(index as usize) as u32);
    writel((readl(clock.reg) & !clock.src_mask) | src, clock.reg);
    0
}

static cpg_div6_clock_ops: clk_ops = clk_ops {
    enable: Some(cpg_div6_clock_enable), disable: Some(cpg_div6_clock_disable),
    is_enabled: Some(cpg_div6_clock_is_enabled), get_parent: Some(cpg_div6_clock_get_parent),
    set_parent: Some(cpg_div6_clock_set_parent), recalc_rate: Some(cpg_div6_clock_recalc_rate),
    determine_rate: Some(cpg_div6_clock_determine_rate), set_rate: Some(cpg_div6_clock_set_rate),
};

unsafe fn cpg_div6_clock_notifier_call(nb: *mut notifier_block, action: c_ulong, _data: *mut core::ffi::c_void) -> i32 {
    let clock = container_of!(nb, div6_clock, nb);
    if action == PM_EVENT_RESUME {
        /*
         * TODO: This does not yet support DIV6 clocks with multiple
         * parents, as the parent selection bits are not restored.
         * Fortunately so far such DIV6 clocks are found only on
         * R/SH-Mobile SoCs, while the resume functionality is only
         * needed on R-Car Gen3.
         */
        if __clk_get_enable_count((*clock).hw.clk) != 0 { cpg_div6_clock_enable(&mut (*clock).hw); }
        else { cpg_div6_clock_disable(&mut (*clock).hw); }
        return NOTIFY_OK;
    }
    NOTIFY_DONE
}

/**
 * cpg_div6_register - Register a DIV6 clock
 * @name: Name of the DIV6 clock
 * @num_parents: Number of parent clocks of the DIV6 clock (1, 4, or 8)
 * @parent_names: Array containing the names of the parent clocks
 * @reg: Mapped register used to control the DIV6 clock
 * @notifiers: Optional notifier chain to save/restore state for system resume
 */
unsafe fn cpg_div6_register(name: *const c_char, num_parents: u32,
    parent_names: *mut *const c_char, reg: *mut core::ffi::c_void,
    notifiers: *mut raw_notifier_head) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let clock = kzalloc_flex::<div6_clock>(num_parents as usize);
    if clock.is_null() { return ERR_PTR(-12); }
    (*clock).reg = reg;
    (*clock).div = (readl(reg) & CPG_DIV6_DIV_MASK) + 1;
    (*clock).src_mask = match num_parents {
        1 => 0,
        4 => (0x3 << 6),
        8 => (0x7 << 12),
        _ => { pr_err!("%s: invalid number of parents for DIV6 clock %s\n", "cpg_div6_register", name); kfree(clock); return ERR_PTR(-22); }
    };
    let mut valid = 0;
    for i in 0..num_parents {
        let p = *parent_names.add(i as usize);
        if !p.is_null() { *parent_names.add(valid as usize) = p; *(*clock).parents.as_mut_ptr().add(valid as usize) = i as u8; valid += 1; }
    }
    init.name = name; init.ops = &cpg_div6_clock_ops; init.parent_names = parent_names; init.num_parents = valid;
    (*clock).hw.init = &init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*clock).hw);
    if IS_ERR(clk) { kfree(clock); return clk; }
    if !notifiers.is_null() { (*clock).nb.notifier_call = Some(cpg_div6_clock_notifier_call); raw_notifier_chain_register(notifiers, &mut (*clock).nb); }
    clk
}

unsafe fn cpg_div6_clock_init(np: *mut device_node) {
    let num_parents = of_clk_get_parent_count(np);
    if num_parents < 1 { pr_err!("%s: no parent found for %pOFn DIV6 clock\n", "cpg_div6_clock_init", np); return; }
    let parent_names = kmalloc_array::<*const c_char>(num_parents as usize, GFP_KERNEL);
    if parent_names.is_null() { return; }
    let reg = of_iomap(np, 0);
    if reg.is_null() { pr_err!("%s: failed to map %pOFn DIV6 clock register\n", "cpg_div6_clock_init", np); kfree(parent_names); return; }
    let mut clk_name = (*np).name;
    of_property_read_string(np, b"clock-output-names\0".as_ptr() as *const c_char, &mut clk_name);
    for i in 0..num_parents { *parent_names.add(i as usize) = of_clk_get_parent_name(np, i); }
    let clk = cpg_div6_register(clk_name, num_parents, parent_names, reg, core::ptr::null_mut());
    if IS_ERR(clk) { pr_err!("%s: failed to register %pOFn DIV6 clock\n", "cpg_div6_clock_init", np); iounmap(reg); kfree(parent_names); return; }
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    kfree(parent_names);
}

// CLK_OF_DECLARE(cpg_div6_clk, "renesas,cpg-div6-clock", cpg_div6_clock_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
