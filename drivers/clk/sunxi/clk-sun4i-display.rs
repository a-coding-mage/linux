// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// External Linux kernel types, functions, constants, and registration macros
// are supplied by the surrounding translation unit/dependencies.

#[repr(C)]
struct sun4i_a10_display_clk_data {
    has_div: bool,
    num_rst: u8,
    parents: u8,
    offset_en: u8,
    offset_div: u8,
    offset_mux: u8,
    offset_rst: u8,
    width_div: u8,
    width_mux: u8,
    flags: u32,
}

#[repr(C)]
struct reset_data {
    reg: *mut core::ffi::c_void,
    lock: *mut spinlock_t,
    rcdev: reset_controller_dev,
    offset: u8,
}

static mut sun4i_a10_display_lock: spinlock_t = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn rcdev_to_reset_data(rcdev: *mut reset_controller_dev) -> *mut reset_data {
    (rcdev as *mut u8).sub(core::mem::offset_of!(reset_data, rcdev)) as *mut reset_data
}

unsafe fn sun4i_a10_display_assert(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let data = rcdev_to_reset_data(rcdev);
    let mut flags: c_ulong = 0;
    let reg: u32;

    spin_lock_irqsave((*data).lock, &mut flags);
    reg = readl((*data).reg);
    writel(reg & !(1u32 << ((*data).offset as c_ulong + id)), (*data).reg);
    spin_unlock_irqrestore((*data).lock, flags);
    0
}

unsafe fn sun4i_a10_display_deassert(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let data = rcdev_to_reset_data(rcdev);
    let mut flags: c_ulong = 0;
    let reg: u32;

    spin_lock_irqsave((*data).lock, &mut flags);
    reg = readl((*data).reg);
    writel(reg | (1u32 << ((*data).offset as c_ulong + id)), (*data).reg);
    spin_unlock_irqrestore((*data).lock, flags);
    0
}

unsafe fn sun4i_a10_display_status(
    rcdev: *mut reset_controller_dev,
    id: c_ulong,
) -> c_int {
    let data = rcdev_to_reset_data(rcdev);
    (!(readl((*data).reg) & (1u32 << ((*data).offset as c_ulong + id)))) as c_int
}

static sun4i_a10_display_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(sun4i_a10_display_assert),
    deassert: Some(sun4i_a10_display_deassert),
    status: Some(sun4i_a10_display_status),
};

unsafe fn sun4i_a10_display_reset_xlate(
    _rcdev: *mut reset_controller_dev,
    _spec: *const of_phandle_args,
) -> c_int {
    // We only have a single reset signal
    0
}

unsafe fn sun4i_a10_display_init(
    node: *mut device_node,
    data: *const sun4i_a10_display_clk_data,
) {
    let mut parents: [*const c_char; 4] = [core::ptr::null(); 4];
    let mut clk_name: *const c_char = (*node).name;
    let mut reset_data: *mut reset_data;
    let mut div: *mut clk_divider = core::ptr::null_mut();
    let gate: *mut clk_gate;
    let mut res: resource = core::mem::zeroed();
    let mux: *mut clk_mux;
    let reg: *mut core::ffi::c_void;
    let clk: *mut clk;
    let mut ret: c_int;

    of_property_read_string(node, c_str!("clock-output-names"), &mut clk_name);
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if is_err(reg) {
        pr_err(c_str!("%s: Could not map the clock registers\n"), clk_name);
        return;
    }

    ret = of_clk_parent_fill(node, parents.as_mut_ptr(), (*data).parents);
    if ret != (*data).parents as c_int {
        pr_err(c_str!("%s: Could not retrieve the parents\n"), clk_name);
        goto!(unmap);
    }

    mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() { goto!(unmap); }
    (*mux).reg = reg;
    (*mux).shift = (*data).offset_mux;
    (*mux).mask = (1u32 << (*data).width_mux) - 1;
    (*mux).lock = &raw mut sun4i_a10_display_lock;

    gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() { goto!(free_mux); }
    (*gate).reg = reg;
    (*gate).bit_idx = (*data).offset_en;
    (*gate).lock = &raw mut sun4i_a10_display_lock;

    if (*data).has_div {
        div = kzalloc_obj::<clk_divider>();
        if div.is_null() { goto!(free_gate); }
        (*div).reg = reg;
        (*div).shift = (*data).offset_div;
        (*div).width = (*data).width_div;
        (*div).lock = &raw mut sun4i_a10_display_lock;
    }

    clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents.as_mut_ptr(),
        (*data).parents, &mut (*mux).hw, &clk_mux_ops,
        if (*data).has_div { &mut (*div).hw } else { core::ptr::null_mut() },
        if (*data).has_div { &clk_divider_ops } else { core::ptr::null() },
        &mut (*gate).hw, &clk_gate_ops, (*data).flags);
    if is_err(clk) { pr_err(c_str!("%s: Couldn't register the clock\n"), clk_name); goto!(free_div); }
    ret = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if ret != 0 { pr_err(c_str!("%s: Couldn't register DT provider\n"), clk_name); goto!(free_clk); }
    if (*data).num_rst == 0 { return; }

    reset_data = kzalloc_obj::<reset_data>();
    if reset_data.is_null() { goto!(free_of_clk); }
    (*reset_data).reg = reg;
    (*reset_data).offset = (*data).offset_rst;
    (*reset_data).lock = &raw mut sun4i_a10_display_lock;
    (*reset_data).rcdev.nr_resets = (*data).num_rst;
    (*reset_data).rcdev.ops = &sun4i_a10_display_reset_ops;
    (*reset_data).rcdev.of_node = node;
    if (*data).num_rst == 1 {
        (*reset_data).rcdev.of_reset_n_cells = 0;
        (*reset_data).rcdev.of_xlate = Some(sun4i_a10_display_reset_xlate);
    } else { (*reset_data).rcdev.of_reset_n_cells = 1; }
    if reset_controller_register(&mut (*reset_data).rcdev) != 0 {
        pr_err(c_str!("%s: Couldn't register the reset controller\n"), clk_name);
        goto!(free_reset);
    }
    return;

free_reset: kfree(reset_data);
free_of_clk: of_clk_del_provider(node);
free_clk: clk_unregister_composite(clk);
free_div: kfree(div);
free_gate: kfree(gate);
free_mux: kfree(mux);
unmap: iounmap(reg); of_address_to_resource(node, 0, &mut res); release_mem_region(res.start, resource_size(&res));
}

static sun4i_a10_tcon_ch0_data: sun4i_a10_display_clk_data = sun4i_a10_display_clk_data {
    has_div: false, num_rst: 2, parents: 4, offset_en: 31, offset_div: 0,
    offset_mux: 24, offset_rst: 29, width_div: 0, width_mux: 2, flags: CLK_SET_RATE_PARENT,
};

unsafe fn sun4i_a10_tcon_ch0_setup(node: *mut device_node) { sun4i_a10_display_init(node, &sun4i_a10_tcon_ch0_data); }
// CLK_OF_DECLARE(sun4i_a10_tcon_ch0, "allwinner,sun4i-a10-tcon-ch0-clk", sun4i_a10_tcon_ch0_setup);

static sun4i_a10_display_data: sun4i_a10_display_clk_data = sun4i_a10_display_clk_data {
    has_div: true, num_rst: 1, parents: 3, offset_en: 31, offset_div: 0,
    offset_mux: 24, offset_rst: 30, width_div: 4, width_mux: 2, flags: 0,
};

unsafe fn sun4i_a10_display_setup(node: *mut device_node) { sun4i_a10_display_init(node, &sun4i_a10_display_data); }
// CLK_OF_DECLARE(sun4i_a10_display, "allwinner,sun4i-a10-display-clk", sun4i_a10_display_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
