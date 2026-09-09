// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 *
 * Emilio López <emilio@elopez.com.ar>
 */

// Linux clock, device-tree, I/O, platform-device, and allocation APIs are
// supplied by the surrounding kernel bindings.

unsafe fn sun4i_a10_get_mod0_factors(req: *mut factors_request) {
    let mut div: u8;
    let mut calcm: u8;
    let mut calcp: u8;

    unsafe {
        if (*req).rate > (*req).parent_rate {
            (*req).rate = (*req).parent_rate;
        }
        div = ((*req).parent_rate + (*req).rate - 1) / (*req).rate;
        if div < 16 { calcp = 0; }
        else if div / 2 < 16 { calcp = 1; }
        else if div / 4 < 16 { calcp = 2; }
        else { calcp = 3; }
        calcm = (div + ((1u8 << calcp) - 1)) / (1u8 << calcp);
        (*req).rate = ((*req).parent_rate >> calcp) / calcm;
        (*req).m = calcm - 1;
        (*req).p = calcp;
    }
}

/* user manual says "n" but it's really "p" */
static sun4i_a10_mod0_config: clk_factors_config = clk_factors_config {
    mshift: 0, mwidth: 4, pshift: 16, pwidth: 2,
};

static sun4i_a10_mod0_data: factors_data = factors_data {
    enable: 31, mux: 24, muxmask: BIT(1) | BIT(0),
    table: &sun4i_a10_mod0_config, getter: sun4i_a10_get_mod0_factors,
};

static mut sun4i_a10_mod0_lock: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn sun4i_a10_mod0_setup(node: *mut device_node) {
    let reg = of_iomap(node, 0);
    if reg.is_null() { return; }
    sunxi_factors_register(node, &sun4i_a10_mod0_data, &mut sun4i_a10_mod0_lock, reg);
}

unsafe fn sun4i_a10_mod0_clk_probe(pdev: *mut platform_device) -> i32 {
    let np = (*(*pdev).dev).of_node;
    if np.is_null() { return -ENODEV; }
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    sunxi_factors_register(np, &sun4i_a10_mod0_data, &mut sun4i_a10_mod0_lock, reg);
    0
}

static sun4i_a10_mod0_clk_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: "allwinner,sun4i-a10-mod0-clk" },
    of_device_id { /* sentinel */ },
];

static mut sun4i_a10_mod0_clk_driver: platform_driver = platform_driver {
    driver: driver { name: "sun4i-a10-mod0-clk", of_match_table: sun4i_a10_mod0_clk_dt_ids.as_ptr() },
    probe: sun4i_a10_mod0_clk_probe,
};

static sun9i_a80_mod0_data: factors_data = factors_data {
    enable: 31, mux: 24, muxmask: BIT(3) | BIT(2) | BIT(1) | BIT(0),
    table: &sun4i_a10_mod0_config, getter: sun4i_a10_get_mod0_factors,
};

unsafe fn sun9i_a80_mod0_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) { pr_err!("Could not get registers for mod0-clk: %pOFn\n", node); return; }
    sunxi_factors_register(node, &sun9i_a80_mod0_data, &mut sun4i_a10_mod0_lock, reg);
}

static mut sun5i_a13_mbus_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun5i_a13_mbus_setup(node: *mut device_node) {
    let reg = of_iomap(node, 0);
    if reg.is_null() { pr_err!("Could not get registers for a13-mbus-clk\n"); return; }
    sunxi_factors_register_critical(node, &sun4i_a10_mod0_data, &mut sun5i_a13_mbus_lock, reg);
}

#[repr(C)]
struct mmc_phase { hw: clk_hw, offset: u8, reg: *mut core::ffi::c_void, lock: *mut spinlock_t }

unsafe fn mmc_get_phase(hw: *mut clk_hw) -> i32 {
    let phase = container_of!(hw, mmc_phase, hw);
    let value = readl((*phase).reg);
    let delay = ((value >> (*phase).offset) & 0x3) as u8;
    if delay == 0 { return 180; }
    let clk = (*hw).clk;
    let mmc = clk_get_parent(clk); if mmc.is_null() { return -EINVAL; }
    let mmc_rate = clk_get_rate(mmc); if mmc_rate == 0 { return -EINVAL; }
    let mmc_parent = clk_get_parent(mmc); if mmc_parent.is_null() { return -EINVAL; }
    let mmc_parent_rate = clk_get_rate(mmc_parent); if mmc_parent_rate == 0 { return -EINVAL; }
    let mmc_div = mmc_parent_rate / mmc_rate;
    delay as i32 * ((360 + mmc_div / 2) / mmc_div) as i32
}

unsafe fn mmc_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let phase = container_of!(hw, mmc_phase, hw);
    let clk = (*hw).clk;
    let mmc = clk_get_parent(clk); if mmc.is_null() { return -EINVAL; }
    let mmc_rate = clk_get_rate(mmc); if mmc_rate == 0 { return -EINVAL; }
    let mmc_parent = clk_get_parent(mmc); if mmc_parent.is_null() { return -EINVAL; }
    let mmc_parent_rate = clk_get_rate(mmc_parent); if mmc_parent_rate == 0 { return -EINVAL; }
    let delay: u8 = if degrees != 180 {
        let div = mmc_parent_rate / mmc_rate;
        let step = (360 + div / 2) / div;
        ((degrees + step / 2) / step) as u8
    } else { 0 };
    let mut flags = 0;
    spin_lock_irqsave((*phase).lock, &mut flags);
    let mut value = readl((*phase).reg);
    value &= !GENMASK((*phase).offset + 3, (*phase).offset);
    value |= (delay as u32) << (*phase).offset;
    writel(value, (*phase).reg);
    spin_unlock_irqrestore((*phase).lock, flags);
    0
}

static mmc_clk_ops: clk_ops = clk_ops { get_phase: mmc_get_phase, set_phase: mmc_set_phase };

unsafe fn sunxi_mmc_setup(node: *mut device_node, data: *const factors_data, lock: *mut spinlock_t) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) { pr_err!("Couldn't map the %pOFn clock registers\n", node); return; }
    let clk_data = kmalloc_obj::<clk_onecell_data>(); if clk_data.is_null() { return; }
    (*clk_data).clks = kzalloc_objs::<*mut clk>(3); if (*clk_data).clks.is_null() { kfree(clk_data); return; }
    (*clk_data).clk_num = 3;
    (*clk_data).clks[0] = sunxi_factors_register(node, data, lock, reg);
    if (*clk_data).clks[0].is_null() { kfree((*clk_data).clks); kfree(clk_data); return; }
    let parent = __clk_get_name((*clk_data).clks[0]);
    for i in 1..3 {
        let mut init = clk_init_data { num_parents: 1, parent_names: &parent, ops: &mmc_clk_ops, name: core::ptr::null() };
        let phase = kmalloc_obj::<mmc_phase>(); if phase.is_null() { continue; }
        (*phase).hw.init = &mut init; (*phase).reg = reg; (*phase).lock = lock; (*phase).offset = if i == 1 { 8 } else { 20 };
        if of_property_read_string_index(node, "clock-output-names", i, &mut init.name) != 0 { init.name = (*node).name; }
        (*clk_data).clks[i] = clk_register(core::ptr::null_mut(), &mut (*phase).hw);
        if IS_ERR((*clk_data).clks[i]) { kfree(phase); }
    }
    of_clk_add_provider(node, of_clk_src_onecell_get, clk_data);
}

static mut sun4i_a10_mmc_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun4i_a10_mmc_setup(node: *mut device_node) { sunxi_mmc_setup(node, &sun4i_a10_mod0_data, &mut sun4i_a10_mmc_lock); }
static mut sun9i_a80_mmc_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun9i_a80_mmc_setup(node: *mut device_node) { sunxi_mmc_setup(node, &sun9i_a80_mod0_data, &mut sun9i_a80_mmc_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
