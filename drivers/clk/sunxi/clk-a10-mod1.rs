// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 *
 * Emilio López <emilio@elopez.com.ar>
 */

// C dependencies supplied by the surrounding kernel translation.

static mod1_lock: Spinlock = Spinlock::new();

const SUN4I_MOD1_ENABLE: u32 = 31;
const SUN4I_MOD1_MUX: u32 = 16;
const SUN4I_MOD1_MUX_WIDTH: u32 = 2;
const SUN4I_MOD1_MAX_PARENTS: usize = 4;

unsafe fn sun4i_mod1_clk_setup(node: *mut device_node) {
    let mut clk: *mut clk = core::ptr::null_mut();
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut parents: [*const core::ffi::c_char; 4] = [core::ptr::null(); 4];
    let mut clk_name: *const core::ffi::c_char = (*node).name;
    let mut reg: *mut core::ffi::c_void;
    let mut i: i32;

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) {
        return;
    }

    mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() {
        goto_err_unmap(reg);
        return;
    }

    gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() {
        kfree(mux as *mut core::ffi::c_void);
        goto_err_unmap(reg);
        return;
    }

    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const _, &mut clk_name);
    i = of_clk_parent_fill(node, parents.as_mut_ptr(), SUN4I_MOD1_MAX_PARENTS);

    (*gate).reg = reg;
    (*gate).bit_idx = SUN4I_MOD1_ENABLE;
    (*gate).lock = &mod1_lock;
    (*mux).reg = reg;
    (*mux).shift = SUN4I_MOD1_MUX;
    (*mux).mask = (1u32 << SUN4I_MOD1_MUX_WIDTH) - 1;
    (*mux).lock = &mod1_lock;

    clk = clk_register_composite(
        core::ptr::null_mut(),
        clk_name,
        parents.as_ptr(),
        i,
        &mut (*mux).hw,
        &clk_mux_ops,
        core::ptr::null_mut(),
        core::ptr::null(),
        &mut (*gate).hw,
        &clk_gate_ops,
        CLK_SET_RATE_PARENT,
    );
    if IS_ERR(clk) {
        kfree(gate as *mut core::ffi::c_void);
        kfree(mux as *mut core::ffi::c_void);
        goto_err_unmap(reg);
        return;
    }

    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
}

unsafe fn goto_err_unmap(reg: *mut core::ffi::c_void) {
    iounmap(reg);
}

// CLK_OF_DECLARE(sun4i_mod1, "allwinner,sun4i-a10-mod1-clk",
//                sun4i_mod1_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
