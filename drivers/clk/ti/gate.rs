// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP gate clock support
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Linux clock-provider, slab, I/O, device-tree, and TI clock interfaces are
// supplied by the surrounding kernel translation unit.

static mut OMAP_GATE_CLKDM_CLK_OPS: clk_ops = clk_ops {
    init: Some(omap2_init_clk_clkdm),
    enable: Some(omap2_clkops_enable_clkdm),
    disable: Some(omap2_clkops_disable_clkdm),
    restore_context: Some(clk_gate_restore_context),
    ..clk_ops::ZERO
};

pub static mut OMAP_GATE_CLK_OPS: clk_ops = clk_ops {
    init: Some(omap2_init_clk_clkdm),
    enable: Some(omap2_dflt_clk_enable),
    disable: Some(omap2_dflt_clk_disable),
    is_enabled: Some(omap2_dflt_clk_is_enabled),
    restore_context: Some(clk_gate_restore_context),
    ..clk_ops::ZERO
};

static mut OMAP_GATE_CLK_HSDIV_RESTORE_OPS: clk_ops = clk_ops {
    init: Some(omap2_init_clk_clkdm),
    enable: Some(omap36xx_gate_clk_enable_with_hsdiv_restore),
    disable: Some(omap2_dflt_clk_disable),
    is_enabled: Some(omap2_dflt_clk_is_enabled),
    restore_context: Some(clk_gate_restore_context),
    ..clk_ops::ZERO
};

/*
 * omap36xx_gate_clk_enable_with_hsdiv_restore - enable clocks suffering
 * from HSDivider PWRDN problem. Implements Errata ID: i556.
 */
unsafe extern "C" fn omap36xx_gate_clk_enable_with_hsdiv_restore(
    hw: *mut clk_hw,
) -> i32 {
    let parent_hw: *mut clk_hw;
    let parent: *mut clk_omap_divider;
    let dummy_v: u32;
    let orig_v: u32;
    let ret: i32;

    /* Clear PWRDN bit of HSDIVIDER */
    ret = omap2_dflt_clk_enable(hw);

    /* Parent is the x2 node, get parent of parent for the m2 div */
    parent_hw = clk_hw_get_parent(clk_hw_get_parent(hw));
    parent = to_clk_omap_divider(parent_hw);

    /* Restore the dividers */
    if ret == 0 {
        orig_v = (*ti_clk_ll_ops).clk_readl(&(*parent).reg);
        dummy_v = orig_v ^ (1u32 << (*parent).shift);

        /* Write any other value different from the Read value */
        (*ti_clk_ll_ops).clk_writel(dummy_v, &(*parent).reg);

        /* Write the original divider */
        (*ti_clk_ll_ops).clk_writel(orig_v, &(*parent).reg);
    }

    ret
}

unsafe fn _register_gate(
    node: *mut device_node,
    name: *const i8,
    parent_name: *const i8,
    flags: c_ulong,
    reg: *const clk_omap_reg,
    bit_idx: u8,
    clk_gate_flags: u8,
    ops: *const clk_ops,
    hw_ops: *const clk_hw_omap_ops,
) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let clk_hw = kzalloc::<clk_hw_omap>();
    if clk_hw.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*clk_hw).hw.init = &mut init;
    init.name = name;
    init.ops = ops;
    core::ptr::copy_nonoverlapping(reg, &mut (*clk_hw).enable_reg, 1);
    (*clk_hw).enable_bit = bit_idx;
    (*clk_hw).ops = hw_ops;
    (*clk_hw).flags = clk_gate_flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = flags;

    let clk = of_ti_clk_register_omap_hw(node, &mut (*clk_hw).hw, name);
    if IS_ERR(clk) {
        kfree(clk_hw);
    }
    clk
}

unsafe fn _of_ti_gate_clk_setup(
    node: *mut device_node,
    ops: *const clk_ops,
    hw_ops: *const clk_hw_omap_ops,
) {
    let mut reg: clk_omap_reg = core::mem::zeroed();
    let mut enable_bit: u8 = 0;
    let mut flags: u32 = 0;
    let mut clk_gate_flags: u8 = 0;

    if ops != &OMAP_GATE_CLKDM_CLK_OPS as *const _ {
        if ti_clk_get_reg_addr(node, 0, &mut reg) != 0 { return; }
        enable_bit = reg.bit;
    }
    if of_clk_get_parent_count(node) != 1 {
        pr_err!("%pOFn must have 1 parent\n", node);
        return;
    }
    let parent_name = of_clk_get_parent_name(node, 0);
    if of_property_read_bool(node, c"ti,set-rate-parent".as_ptr()) { flags |= CLK_SET_RATE_PARENT; }
    if of_property_read_bool(node, c"ti,set-bit-to-disable".as_ptr()) { clk_gate_flags |= INVERT_ENABLE; }
    let name = ti_dt_clk_name(node);
    let clk = _register_gate(node, name, parent_name, flags as c_ulong, &reg, enable_bit, clk_gate_flags, ops, hw_ops);
    if !IS_ERR(clk) { of_clk_add_provider(node, of_clk_src_simple_get, clk); }
}

unsafe fn _of_ti_composite_gate_clk_setup(node: *mut device_node, hw_ops: *const clk_hw_omap_ops) {
    let gate = kzalloc::<clk_hw_omap>();
    if gate.is_null() { return; }
    if ti_clk_get_reg_addr(node, 0, &mut (*gate).enable_reg) != 0 { kfree(gate); return; }
    (*gate).enable_bit = (*gate).enable_reg.bit;
    (*gate).ops = hw_ops;
    if ti_clk_add_component(node, &mut (*gate).hw, CLK_COMPONENT_TYPE_GATE) == 0 { kfree(gate); }
}

unsafe extern "C" fn of_ti_composite_no_wait_gate_clk_setup(node: *mut device_node) {
    _of_ti_composite_gate_clk_setup(node, core::ptr::null());
}
// CLK_OF_DECLARE(ti_composite_no_wait_gate_clk, "ti,composite-no-wait-gate-clock", ...)

#[cfg(any(CONFIG_ARCH_OMAP2, CONFIG_ARCH_OMAP3))]
unsafe extern "C" fn of_ti_composite_interface_clk_setup(node: *mut device_node) {
    _of_ti_composite_gate_clk_setup(node, &clkhwops_iclk_wait);
}
// CLK_OF_DECLARE(ti_composite_interface_clk, "ti,composite-interface-clock", ...)

unsafe extern "C" fn of_ti_composite_gate_clk_setup(node: *mut device_node) {
    _of_ti_composite_gate_clk_setup(node, &clkhwops_wait);
}
// CLK_OF_DECLARE(ti_composite_gate_clk, "ti,composite-gate-clock", ...)

unsafe extern "C" fn of_ti_clkdm_gate_clk_setup(node: *mut device_node) { _of_ti_gate_clk_setup(node, &OMAP_GATE_CLKDM_CLK_OPS, core::ptr::null()); }
// CLK_OF_DECLARE(ti_clkdm_gate_clk, "ti,clkdm-gate-clock", ...)
unsafe extern "C" fn of_ti_hsdiv_gate_clk_setup(node: *mut device_node) { _of_ti_gate_clk_setup(node, &OMAP_GATE_CLK_HSDIV_RESTORE_OPS, &clkhwops_wait); }
// CLK_OF_DECLARE(ti_hsdiv_gate_clk, "ti,hsdiv-gate-clock", ...)
unsafe extern "C" fn of_ti_gate_clk_setup(node: *mut device_node) { _of_ti_gate_clk_setup(node, &OMAP_GATE_CLK_OPS, core::ptr::null()); }
// CLK_OF_DECLARE(ti_gate_clk, "ti,gate-clock", ...)
unsafe extern "C" fn of_ti_wait_gate_clk_setup(node: *mut device_node) { _of_ti_gate_clk_setup(node, &OMAP_GATE_CLK_OPS, &clkhwops_wait); }
// CLK_OF_DECLARE(ti_wait_gate_clk, "ti,wait-gate-clock", ...)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
