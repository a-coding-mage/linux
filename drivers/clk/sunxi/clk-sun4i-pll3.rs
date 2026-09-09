// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel bindings.

const SUN4I_A10_PLL3_GATE_BIT: u32 = 31;
const SUN4I_A10_PLL3_DIV_WIDTH: u32 = 7;
const SUN4I_A10_PLL3_DIV_SHIFT: u32 = 0;

static mut SUN4I_A10_PLL3_LOCK: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn sun4i_a10_pll3_setup(node: *mut device_node) {
    let mut clk_name: *const c_char = (*node).name;
    let parent: *const c_char;
    let mult: *mut clk_multiplier;
    let gate: *mut clk_gate;
    let mut res: resource = core::mem::zeroed();
    let reg: *mut core::ffi::c_void;
    let clk: *mut clk;
    let ret: i32;

    of_property_read_string(node, c"clock-output-names".as_ptr(), &mut clk_name);
    parent = of_clk_get_parent_name(node, 0);

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) {
        pr_err(c"%s: Could not map the clock registers\n".as_ptr(), clk_name);
        return;
    }

    gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() {
        goto_err_unmap(reg, node, &mut res);
        return;
    }

    (*gate).reg = reg;
    (*gate).bit_idx = SUN4I_A10_PLL3_GATE_BIT;
    (*gate).lock = &raw mut SUN4I_A10_PLL3_LOCK;

    mult = kzalloc_obj::<clk_multiplier>();
    if mult.is_null() {
        kfree(gate);
        goto_err_unmap(reg, node, &mut res);
        return;
    }

    (*mult).reg = reg;
    (*mult).shift = SUN4I_A10_PLL3_DIV_SHIFT;
    (*mult).width = SUN4I_A10_PLL3_DIV_WIDTH;
    (*mult).lock = &raw mut SUN4I_A10_PLL3_LOCK;

    clk = clk_register_composite(
        core::ptr::null_mut(),
        clk_name,
        &parent,
        1,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut (*mult).hw,
        &clk_multiplier_ops,
        &mut (*gate).hw,
        &clk_gate_ops,
        0,
    );
    if IS_ERR(clk) {
        pr_err(c"%s: Couldn't register the clock\n".as_ptr(), clk_name);
        kfree(mult);
        kfree(gate);
        goto_err_unmap(reg, node, &mut res);
        return;
    }

    ret = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if ret != 0 {
        pr_err(c"%s: Couldn't register DT provider\n".as_ptr(), clk_name);
        clk_unregister_composite(clk);
        kfree(mult);
        kfree(gate);
        goto_err_unmap(reg, node, &mut res);
    }
}

unsafe fn goto_err_unmap(
    reg: *mut core::ffi::c_void,
    node: *mut device_node,
    res: *mut resource,
) {
    iounmap(reg);
    of_address_to_resource(node, 0, res);
    release_mem_region((*res).start, resource_size(res));
}

// CLK_OF_DECLARE(sun4i_a10_pll3, "allwinner,sun4i-a10-pll3-clk",
//                sun4i_a10_pll3_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
