// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependencies supplied by the Linux clock, I/O, device-tree, reset,
// allocation, and spinlock subsystems.

static mut VE_LOCK: spinlock_t = DEFINE_SPINLOCK!();

const SUN4I_VE_ENABLE: u32 = 31;
const SUN4I_VE_DIVIDER_SHIFT: u32 = 16;
const SUN4I_VE_DIVIDER_WIDTH: u32 = 3;
const SUN4I_VE_RESET: u32 = 0;

/*
 * sunxi_ve_reset... - reset bit in ve clk registers handling
 */

#[repr(C)]
struct ve_reset_data {
    reg: *mut core::ffi::c_void,
    lock: *mut spinlock_t,
    rcdev: reset_controller_dev,
}

unsafe fn sunxi_ve_reset_assert(
    rcdev: *mut reset_controller_dev,
    _id: c_ulong,
) -> c_int {
    let data = container_of!(rcdev, ve_reset_data, rcdev);
    let mut flags: c_ulong = 0;
    let mut reg: u32;

    spin_lock_irqsave((*data).lock, &mut flags);

    reg = readl((*data).reg);
    writel(reg & !BIT(SUN4I_VE_RESET), (*data).reg);

    spin_unlock_irqrestore((*data).lock, flags);

    0
}

unsafe fn sunxi_ve_reset_deassert(
    rcdev: *mut reset_controller_dev,
    _id: c_ulong,
) -> c_int {
    let data = container_of!(rcdev, ve_reset_data, rcdev);
    let mut flags: c_ulong = 0;
    let mut reg: u32;

    spin_lock_irqsave((*data).lock, &mut flags);

    reg = readl((*data).reg);
    writel(reg | BIT(SUN4I_VE_RESET), (*data).reg);

    spin_unlock_irqrestore((*data).lock, flags);

    0
}

unsafe fn sunxi_ve_of_xlate(
    _rcdev: *mut reset_controller_dev,
    reset_spec: *const of_phandle_args,
) -> c_int {
    if WARN_ON!((*reset_spec).args_count != 0) {
        return -EINVAL;
    }

    0
}

static sunxi_ve_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(sunxi_ve_reset_assert),
    deassert: Some(sunxi_ve_reset_deassert),
};

unsafe fn sun4i_ve_clk_setup(node: *mut device_node) {
    let mut clk: *mut clk;
    let mut div: *mut clk_divider;
    let mut gate: *mut clk_gate;
    let mut reset_data: *mut ve_reset_data;
    let parent: *const c_char;
    let mut clk_name: *const c_char = (*node).name;
    let mut reg: *mut core::ffi::c_void;
    let mut err: c_int;

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR!(reg) {
        return;
    }

    div = kzalloc_obj!(clk_divider);
    if div.is_null() {
        goto!(err_unmap);
    }

    gate = kzalloc_obj!(clk_gate);
    if gate.is_null() {
        goto!(err_free_div);
    }

    of_property_read_string(node, c"clock-output-names", &mut clk_name);
    parent = of_clk_get_parent_name(node, 0);

    (*gate).reg = reg;
    (*gate).bit_idx = SUN4I_VE_ENABLE;
    (*gate).lock = &raw mut VE_LOCK;

    (*div).reg = reg;
    (*div).shift = SUN4I_VE_DIVIDER_SHIFT;
    (*div).width = SUN4I_VE_DIVIDER_WIDTH;
    (*div).lock = &raw mut VE_LOCK;

    clk = clk_register_composite(
        core::ptr::null_mut(), clk_name, &parent, 1,
        core::ptr::null_mut(), core::ptr::null(),
        &mut (*div).hw, &clk_divider_ops,
        &mut (*gate).hw, &clk_gate_ops,
        CLK_SET_RATE_PARENT,
    );
    if IS_ERR!(clk) {
        goto!(err_free_gate);
    }

    err = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if err != 0 {
        goto!(err_unregister_clk);
    }

    reset_data = kzalloc_obj!(ve_reset_data);
    if reset_data.is_null() {
        goto!(err_del_provider);
    }

    (*reset_data).reg = reg;
    (*reset_data).lock = &raw mut VE_LOCK;
    (*reset_data).rcdev.nr_resets = 1;
    (*reset_data).rcdev.ops = &sunxi_ve_reset_ops;
    (*reset_data).rcdev.of_node = node;
    (*reset_data).rcdev.of_xlate = Some(sunxi_ve_of_xlate);
    (*reset_data).rcdev.of_reset_n_cells = 0;
    err = reset_controller_register(&mut (*reset_data).rcdev);
    if err != 0 {
        goto!(err_free_reset);
    }

    return;

err_free_reset:
    kfree(reset_data);
err_del_provider:
    of_clk_del_provider(node);
err_unregister_clk:
    clk_unregister(clk);
err_free_gate:
    kfree(gate);
err_free_div:
    kfree(div);
err_unmap:
    iounmap(reg);
}

CLK_OF_DECLARE!(sun4i_ve, c"allwinner,sun4i-a10-ve-clk", sun4i_ve_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
