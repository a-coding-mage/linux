// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI Multiplexer Clock
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Dependencies supplied by the surrounding kernel clock implementation:
// linux/clk-provider.h, linux/slab.h, linux/err.h, linux/of.h,
// linux/of_address.h, linux/clk/ti.h, and clock.h.

static unsafe fn ti_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux: *mut clk_omap_mux = to_clk_omap_mux(hw);
    let num_parents: i32 = clk_hw_get_num_parents(hw);
    let mut val: u32;

    /*
     * FIXME need a mux-specific flag to determine if val is bitwise or
     * numeric. e.g. sys_clkin_ck's clksel field is 3 bits wide, but ranges
     * from 0x1 to 0x7 (index starts at one)
     * OTOH, pmd_trace_clk_mux_ck uses a separate bit for each clock, so
     * val = 0x4 really means "bit 2, index starts at bit 0"
     */
    val = ((*ti_clk_ll_ops).clk_readl)(&mut (*mux).reg) >> (*mux).shift;
    val &= (*mux).mask;

    if !(*mux).table.is_null() {
        let mut i = 0;
        while i < num_parents {
            if *(*mux).table.add(i as usize) == val {
                return i as u8;
            }
            i += 1;
        }
        return (-EINVAL) as u8;
    }

    if val != 0 && ((*mux).flags & CLK_MUX_INDEX_BIT) != 0 {
        val = (ffs(val) - 1) as u32;
    }

    if val != 0 && ((*mux).flags & CLK_MUX_INDEX_ONE) != 0 {
        val -= 1;
    }

    if val >= num_parents as u32 {
        return (-EINVAL) as u8;
    }

    val as u8
}

static unsafe fn ti_clk_mux_set_parent(hw: *mut clk_hw, mut index: u8) -> i32 {
    let mux: *mut clk_omap_mux = to_clk_omap_mux(hw);
    let mut val: u32;

    if !(*mux).table.is_null() {
        index = *(*mux).table.add(index as usize) as u8;
    } else {
        if ((*mux).flags & CLK_MUX_INDEX_BIT) != 0 {
            index = (1 << ffs(index as u32)) as u8;
        }
        if ((*mux).flags & CLK_MUX_INDEX_ONE) != 0 {
            index += 1;
        }
    }

    if ((*mux).flags & CLK_MUX_HIWORD_MASK) != 0 {
        val = (*mux).mask << ((*mux).shift + 16);
    } else {
        val = ((*ti_clk_ll_ops).clk_readl)(&mut (*mux).reg);
        val &= !((*mux).mask << (*mux).shift);
    }
    val |= (index as u32) << (*mux).shift;
    ((*ti_clk_ll_ops).clk_writel)(val, &mut (*mux).reg);
    ti_clk_latch(&mut (*mux).reg, (*mux).latch);

    0
}

/**
 * clk_mux_save_context - Save the parent selected in the mux
 * @hw: pointer  struct clk_hw
 *
 * Save the parent mux value.
 */
static unsafe fn clk_mux_save_context(hw: *mut clk_hw) -> i32 {
    let mux: *mut clk_omap_mux = to_clk_omap_mux(hw);
    (*mux).saved_parent = ti_clk_mux_get_parent(hw);
    0
}

/**
 * clk_mux_restore_context - Restore the parent in the mux
 * @hw: pointer  struct clk_hw
 *
 * Restore the saved parent mux value.
 */
static unsafe fn clk_mux_restore_context(hw: *mut clk_hw) {
    let mux: *mut clk_omap_mux = to_clk_omap_mux(hw);
    ti_clk_mux_set_parent(hw, (*mux).saved_parent);
}

const ti_clk_mux_ops: clk_ops = clk_ops {
    get_parent: Some(ti_clk_mux_get_parent),
    set_parent: Some(ti_clk_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
    save_context: Some(clk_mux_save_context),
    restore_context: Some(clk_mux_restore_context),
};

static unsafe fn _register_mux(
    node: *mut device_node,
    name: *const c_char,
    parent_data: *const clk_parent_data,
    num_parents: u8,
    flags: c_ulong,
    reg: *const clk_omap_reg,
    shift: u8,
    mask: u32,
    latch: i8,
    clk_mux_flags: u8,
    table: *mut u32,
) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let mux: *mut clk_omap_mux = kzalloc_obj::<clk_omap_mux>();
    if mux.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &ti_clk_mux_ops;
    init.flags = flags;
    init.parent_data = parent_data;
    init.num_parents = num_parents;

    /* struct clk_mux assignments */
    core::ptr::copy_nonoverlapping(reg, &mut (*mux).reg, 1);
    (*mux).shift = shift;
    (*mux).mask = mask;
    (*mux).latch = latch;
    (*mux).flags = clk_mux_flags;
    (*mux).table = table;
    (*mux).hw.init = &init;

    let clk = of_ti_clk_register(node, &mut (*mux).hw, name);
    if IS_ERR(clk) {
        kfree(mux);
    }
    clk
}

/**
 * of_mux_clk_setup - Setup function for simple mux rate clock
 * @node: DT node for the clock
 *
 * Sets up a basic clock multiplexer.
 */
static unsafe fn of_mux_clk_setup(node: *mut device_node) {
    let mut reg: clk_omap_reg = core::mem::zeroed();
    let num_parents = of_clk_get_parent_count(node);
    let mut clk_mux_flags: u8 = 0;
    let mut mask: u32 = 0;
    let mut shift: u32 = 0;
    let mut latch: i32 = -EINVAL;
    let mut flags: u32 = CLK_SET_RATE_NO_REPARENT;
    let mut parent_data: *mut clk_parent_data;

    if num_parents < 2 {
        pr_err("mux-clock %pOFn must have parents\n", node);
        return;
    }
    parent_data = kcalloc(num_parents, core::mem::size_of::<clk_parent_data>(), GFP_KERNEL);
    if parent_data.is_null() { return; }

    for i in 0..num_parents {
        (*parent_data.add(i as usize)).index = i;
    }
    if ti_clk_get_reg_addr(node, 0, &mut reg) != 0 { kfree(parent_data as *mut core::ffi::c_void); return; }
    shift = reg.bit;
    of_property_read_u32(node, "ti,latch-bit\0".as_ptr() as *const c_char, &mut latch);
    if of_property_read_bool(node, "ti,index-starts-at-one\0".as_ptr() as *const c_char) { clk_mux_flags |= CLK_MUX_INDEX_ONE; }
    if of_property_read_bool(node, "ti,set-rate-parent\0".as_ptr() as *const c_char) { flags |= CLK_SET_RATE_PARENT; }
    mask = num_parents;
    if (clk_mux_flags & CLK_MUX_INDEX_ONE) == 0 { mask -= 1; }
    mask = (1 << fls(mask)) - 1;
    let name = ti_dt_clk_name(node);
    let clk = _register_mux(node, name, parent_data, num_parents as u8, flags as c_ulong, &reg, shift as u8, mask, latch as i8, clk_mux_flags, core::ptr::null_mut());
    if !IS_ERR(clk) { of_clk_add_provider(node, of_clk_src_simple_get, clk); }
    kfree(parent_data as *mut core::ffi::c_void);
}

CLK_OF_DECLARE!(mux_clk, "ti,mux-clock", of_mux_clk_setup);

pub unsafe fn ti_clk_build_component_mux(setup: *mut ti_clk_mux) -> *mut clk_hw {
    if setup.is_null() { return core::ptr::null_mut(); }
    let mux: *mut clk_omap_mux = kzalloc_obj::<clk_omap_mux>();
    if mux.is_null() { return ERR_PTR(-ENOMEM); }
    (*mux).shift = (*setup).bit_shift;
    (*mux).latch = -EINVAL;
    (*mux).reg.index = (*setup).module;
    (*mux).reg.offset = (*setup).reg;
    if ((*setup).flags & CLKF_INDEX_STARTS_AT_ONE) != 0 { (*mux).flags |= CLK_MUX_INDEX_ONE; }
    let num_parents = (*setup).num_parents;
    (*mux).mask = num_parents - 1;
    (*mux).mask = (1 << fls((*mux).mask)) - 1;
    &mut (*mux).hw
}

static unsafe fn of_ti_composite_mux_clk_setup(node: *mut device_node) {
    let mux: *mut clk_omap_mux = kzalloc_obj::<clk_omap_mux>();
    if mux.is_null() { return; }
    if ti_clk_get_reg_addr(node, 0, &mut (*mux).reg) != 0 { kfree(mux); return; }
    (*mux).shift = (*mux).reg.bit;
    if of_property_read_bool(node, "ti,index-starts-at-one\0".as_ptr() as *const c_char) { (*mux).flags |= CLK_MUX_INDEX_ONE; }
    let num_parents = of_clk_get_parent_count(node);
    if num_parents < 2 { pr_err("%pOFn must have parents\n", node); kfree(mux); return; }
    (*mux).mask = num_parents - 1;
    (*mux).mask = (1 << fls((*mux).mask)) - 1;
    if ti_clk_add_component(node, &mut (*mux).hw, CLK_COMPONENT_TYPE_MUX) == 0 { kfree(mux); }
}

CLK_OF_DECLARE!(ti_composite_mux_clk_setup, "ti,composite-mux-clock", of_ti_composite_mux_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
