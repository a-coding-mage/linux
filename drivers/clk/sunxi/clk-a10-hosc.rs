// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 *
 * Emilio López <emilio@elopez.com.ar>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/of.h, linux/of_address.h, linux/slab.h

const SUNXI_OSC24M_GATE: u32 = 0;

static mut hosc_lock: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn sun4i_osc_clk_setup(node: *mut device_node) {
    let mut clk: *mut clk;
    let fixed: *mut clk_fixed_rate;
    let gate: *mut clk_gate;
    let mut clk_name: *const ::core::ffi::c_char = (*node).name;
    let mut rate: u32 = 0;

    if of_property_read_u32(node, b"clock-frequency\0".as_ptr() as *const _, &mut rate) != 0 {
        return;
    }

    /* allocate fixed-rate and gate clock structs */
    fixed = kzalloc_obj::<clk_fixed_rate>();
    if fixed.is_null() {
        return;
    }
    gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() {
        goto_err_free_fixed(fixed);
        return;
    }

    of_property_read_string(
        node,
        b"clock-output-names\0".as_ptr() as *const _,
        &mut clk_name,
    );

    /* set up gate and fixed rate properties */
    (*gate).reg = of_iomap(node, 0);
    (*gate).bit_idx = SUNXI_OSC24M_GATE;
    (*gate).lock = &raw mut hosc_lock;
    (*fixed).fixed_rate = rate;

    clk = clk_register_composite(
        core::ptr::null_mut(),
        clk_name,
        core::ptr::null(),
        0,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut (*fixed).hw,
        &clk_fixed_rate_ops,
        &mut (*gate).hw,
        &clk_gate_ops,
        0,
    );

    if IS_ERR(clk) {
        kfree(gate);
        kfree(fixed);
        return;
    }

    of_clk_add_provider(node, of_clk_src_simple_get, clk);
}

unsafe fn goto_err_free_fixed(fixed: *mut clk_fixed_rate) {
    kfree(fixed);
}

// CLK_OF_DECLARE(sun4i_osc, "allwinner,sun4i-a10-osc-clk", sun4i_osc_clk_setup);
CLK_OF_DECLARE!(sun4i_osc, "allwinner,sun4i-a10-osc-clk", sun4i_osc_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
