// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// C kernel dependencies are supplied by the surrounding translation unit.

static mut tegra_car_np: *mut device_node = core::ptr::null_mut();
static mut dummy_car_ops: tegra_cpu_car_ops = tegra_cpu_car_ops {};
pub static mut tegra_cpu_car_ops: *mut tegra_cpu_car_ops = unsafe { &raw mut dummy_car_ops };

pub static mut periph_clk_enb_refcnt: *mut i32 = core::ptr::null_mut();
static mut periph_banks: i32 = 0;
static mut periph_state_ctx: *mut u32 = core::ptr::null_mut();
static mut clks: *mut *mut clk = core::ptr::null_mut();
static mut clk_num: i32 = 0;
static mut clk_data: clk_onecell_data = clk_onecell_data {};

static mut special_reset_assert: Option<unsafe extern "C" fn(usize) -> i32> = None;
static mut special_reset_deassert: Option<unsafe extern "C" fn(usize) -> i32> = None;
static mut num_special_reset: u32 = 0;

static periph_regs: [tegra_clk_periph_regs; 7] = [
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_L, enb_set_reg: CLK_OUT_ENB_SET_L, enb_clr_reg: CLK_OUT_ENB_CLR_L, rst_reg: RST_DEVICES_L, rst_set_reg: RST_DEVICES_SET_L, rst_clr_reg: RST_DEVICES_CLR_L },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_H, enb_set_reg: CLK_OUT_ENB_SET_H, enb_clr_reg: CLK_OUT_ENB_CLR_H, rst_reg: RST_DEVICES_H, rst_set_reg: RST_DEVICES_SET_H, rst_clr_reg: RST_DEVICES_CLR_H },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_U, enb_set_reg: CLK_OUT_ENB_SET_U, enb_clr_reg: CLK_OUT_ENB_CLR_U, rst_reg: RST_DEVICES_U, rst_set_reg: RST_DEVICES_SET_U, rst_clr_reg: RST_DEVICES_CLR_U },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_V, enb_set_reg: CLK_OUT_ENB_SET_V, enb_clr_reg: CLK_OUT_ENB_CLR_V, rst_reg: RST_DEVICES_V, rst_set_reg: RST_DEVICES_SET_V, rst_clr_reg: RST_DEVICES_CLR_V },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_W, enb_set_reg: CLK_OUT_ENB_SET_W, enb_clr_reg: CLK_OUT_ENB_CLR_W, rst_reg: RST_DEVICES_W, rst_set_reg: RST_DEVICES_SET_W, rst_clr_reg: RST_DEVICES_CLR_W },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_X, enb_set_reg: CLK_OUT_ENB_SET_X, enb_clr_reg: CLK_OUT_ENB_CLR_X, rst_reg: RST_DEVICES_X, rst_set_reg: RST_DEVICES_SET_X, rst_clr_reg: RST_DEVICES_CLR_X },
    tegra_clk_periph_regs { enb_reg: CLK_OUT_ENB_Y, enb_set_reg: CLK_OUT_ENB_SET_Y, enb_clr_reg: CLK_OUT_ENB_CLR_Y, rst_reg: RST_DEVICES_Y, rst_set_reg: RST_DEVICES_SET_Y, rst_clr_reg: RST_DEVICES_CLR_Y },
];

static mut clk_base: *mut u8 = core::ptr::null_mut();

unsafe fn tegra_clk_rst_assert(_rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    tegra_read_chipid();
    if id < (periph_banks as usize) * 32 {
        writel_relaxed(BIT(id % 32), clk_base.add(periph_regs[id / 32].rst_set_reg as usize));
        0
    } else if id < (periph_banks as usize) * 32 + num_special_reset as usize {
        special_reset_assert.unwrap()(id)
    } else { -EINVAL }
}

unsafe fn tegra_clk_rst_deassert(_rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    if id < (periph_banks as usize) * 32 {
        writel_relaxed(BIT(id % 32), clk_base.add(periph_regs[id / 32].rst_clr_reg as usize));
        0
    } else if id < (periph_banks as usize) * 32 + num_special_reset as usize {
        special_reset_deassert.unwrap()(id)
    } else { -EINVAL }
}

unsafe fn tegra_clk_rst_reset(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    let err = tegra_clk_rst_assert(rcdev, id);
    if err != 0 { return err; }
    udelay(1);
    tegra_clk_rst_deassert(rcdev, id)
}

pub unsafe fn get_reg_bank(clkid: i32) -> *const tegra_clk_periph_regs {
    let reg_bank = clkid / 32;
    if reg_bank < periph_banks { &periph_regs[reg_bank as usize] } else { WARN_ON(1); core::ptr::null() }
}

pub unsafe fn tegra_clk_set_pllp_out_cpu(enable: bool) {
    let mut val = readl_relaxed(clk_base.add(CLK_OUT_ENB_Y as usize));
    if enable { val |= CLK_ENB_PLLP_OUT_CPU; } else { val &= !CLK_ENB_PLLP_OUT_CPU; }
    writel_relaxed(val, clk_base.add(CLK_OUT_ENB_Y as usize));
}

pub unsafe fn tegra_clk_periph_suspend() {
    let mut idx = 0;
    for i in 0..periph_banks as usize { *periph_state_ctx.add(idx) = readl_relaxed(clk_base.add(periph_regs[i].enb_reg as usize)); idx += 1; }
    for i in 0..periph_banks as usize { *periph_state_ctx.add(idx) = readl_relaxed(clk_base.add(periph_regs[i].rst_reg as usize)); idx += 1; }
}

pub unsafe fn tegra_clk_periph_resume() {
    let mut idx = 0;
    for i in 0..periph_banks as usize { writel_relaxed(*periph_state_ctx.add(idx), clk_base.add(periph_regs[i].enb_reg as usize)); idx += 1; }
    fence_udelay(5, clk_base);
    for i in 0..periph_banks as usize { writel_relaxed(*periph_state_ctx.add(idx), clk_base.add(periph_regs[i].rst_reg as usize)); idx += 1; }
    fence_udelay(2, clk_base);
}

unsafe fn tegra_clk_periph_ctx_init(banks: i32) -> i32 {
    periph_state_ctx = kcalloc((2 * banks) as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
    if periph_state_ctx.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn tegra_clk_init(regs: *mut u8, num: i32, banks: i32) -> *mut *mut clk {
    clk_base = regs;
    if WARN_ON((banks as usize) > periph_regs.len()) { return core::ptr::null_mut(); }
    periph_clk_enb_refcnt = kzalloc((32 * banks) as usize * core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
    if periph_clk_enb_refcnt.is_null() { return core::ptr::null_mut(); }
    periph_banks = banks;
    clks = kzalloc((num as usize) * core::mem::size_of::<*mut clk>(), GFP_KERNEL) as *mut *mut clk;
    if clks.is_null() { kfree(periph_clk_enb_refcnt as *mut u8); return core::ptr::null_mut(); }
    clk_num = num;
    // CONFIG_PM_SLEEP is a build-time condition preserved from the C source.
    if IS_ENABLED_CONFIG_PM_SLEEP && tegra_clk_periph_ctx_init(banks) != 0 { kfree(periph_clk_enb_refcnt as *mut u8); kfree(clks as *mut u8); return core::ptr::null_mut(); }
    clks
}

pub unsafe fn tegra_init_dup_clks(mut dup_list: *mut tegra_clk_duplicate, clks_: *mut *mut clk, clk_max: i32) {
    while (*dup_list).clk_id < clk_max { let c = *clks_.add((*dup_list).clk_id as usize); (*dup_list).lookup.clk = c; clkdev_add(&mut (*dup_list).lookup); dup_list = dup_list.add(1); }
}

pub unsafe fn tegra_init_from_table(mut tbl: *mut tegra_clk_init_table, clks_: *mut *mut clk, clk_max: i32) {
    while (*tbl).clk_id < clk_max {
        let c = *clks_.add((*tbl).clk_id as usize);
        if IS_ERR_OR_NULL(c) { pr_err("%s: invalid entry %ld in clks array for id %d\n", __func__, PTR_ERR(c), (*tbl).clk_id); WARN_ON(1); tbl = tbl.add(1); continue; }
        if (*tbl).parent_id < clk_max { let p = *clks_.add((*tbl).parent_id as usize); if clk_set_parent(c, p) != 0 { pr_err("%s: Failed to set parent %s of %s\n", __func__, __clk_get_name(p), __clk_get_name(c)); WARN_ON(1); } }
        if (*tbl).rate != 0 && clk_set_rate(c, (*tbl).rate) != 0 { pr_err("%s: Failed to set rate %lu of %s\n", __func__, (*tbl).rate, __clk_get_name(c)); WARN_ON(1); }
        if (*tbl).state != 0 && clk_prepare_enable(c) != 0 { pr_err("%s: Failed to enable %s\n", __func__, __clk_get_name(c)); WARN_ON(1); }
        tbl = tbl.add(1);
    }
}

static mut rst_ops: reset_control_ops = reset_control_ops { assert: Some(tegra_clk_rst_assert), deassert: Some(tegra_clk_rst_deassert), reset: Some(tegra_clk_rst_reset) };
static mut rst_ctlr: reset_controller_dev = reset_controller_dev { ops: &raw const rst_ops, owner: THIS_MODULE, of_node: core::ptr::null_mut(), nr_resets: 0, of_reset_n_cells: 1 };

pub unsafe fn tegra_add_of_provider(np: *mut device_node, clk_src_onecell_get: *mut core::ffi::c_void) {
    tegra_car_np = np;
    for i in 0..clk_num as usize { let c = *clks.add(i); if IS_ERR(c) { pr_err("Tegra clk %d: register failed with %ld\n", i as i32, PTR_ERR(c)); } if c.is_null() { *clks.add(i) = ERR_PTR(-EINVAL); } }
    clk_data.clks = clks; clk_data.clk_num = clk_num; of_clk_add_provider(np, clk_src_onecell_get, &mut clk_data);
    rst_ctlr.of_node = np; rst_ctlr.nr_resets = periph_banks * 32 + num_special_reset as i32; reset_controller_register(&mut rst_ctlr);
}

pub unsafe fn tegra_init_special_resets(num: u32, assert: Option<unsafe extern "C" fn(usize) -> i32>, deassert: Option<unsafe extern "C" fn(usize) -> i32>) { num_special_reset = num; special_reset_assert = assert; special_reset_deassert = deassert; }

pub unsafe fn tegra_register_devclks(mut dev_clks: *mut tegra_devclk, num: i32) {
    for _ in 0..num { clk_register_clkdev(*clks.add((*dev_clks).dt_id as usize), (*dev_clks).con_id, (*dev_clks).dev_id); dev_clks = dev_clks.add(1); }
    for i in 0..clk_num as usize { let c = *clks.add(i); if !IS_ERR_OR_NULL(c) { clk_register_clkdev(c, __clk_get_name(c), "tegra-clk-debug" as *const _); } }
}

pub unsafe fn tegra_lookup_dt_id(clk_id: i32, tegra_clk: *mut tegra_clk) -> *mut *mut clk { if (*tegra_clk.add(clk_id as usize)).present { &mut *clks.add((*tegra_clk.add(clk_id as usize)).dt_id as usize) } else { core::ptr::null_mut() } }

unsafe fn tegra_clk_get_of_node(hw: *mut clk_hw) -> *mut device_node { let node_name = kstrdup_and_replace((*(*hw).init).name, b'_', b'-', GFP_KERNEL); if node_name.is_null() { return core::ptr::null_mut(); } let mut np = core::ptr::null_mut(); for_each_child_of_node(tegra_car_np, np) { if strcmp((*np).name, node_name) == 0 { break; } } kfree(node_name as *mut u8); np }

pub unsafe fn tegra_clk_dev_register(hw: *mut clk_hw) -> *mut clk { let np = tegra_clk_get_of_node(hw); if !of_device_is_available(np) { of_node_put(np); return clk_register(core::ptr::null_mut(), hw); } let dev_name = kasprintf(GFP_KERNEL, "tegra_clk_%s", (*(*hw).init).name); if dev_name.is_null() { of_node_put(np); return clk_register(core::ptr::null_mut(), hw); } let parent = of_find_device_by_node(tegra_car_np); let dev = if !parent.is_null() { let pdev = of_platform_device_create(np, dev_name, &mut (*parent).dev); put_device(&mut (*parent).dev); if pdev.is_null() { pr_err("%s: failed to create device for %pOF\n", __func__, np); core::ptr::null_mut() } else { pm_runtime_enable(&mut (*pdev).dev); &mut (*pdev).dev } } else { WARN(1, "failed to find device for %pOF\n", tegra_car_np); core::ptr::null_mut() }; kfree(dev_name as *mut u8); of_node_put(np); clk_register(dev, hw) }

pub static mut tegra_clk_apply_init_table: Option<unsafe extern "C" fn()> = None;
unsafe fn tegra_clocks_apply_init_table() -> i32 { if let Some(f) = tegra_clk_apply_init_table { f(); } 0 }
// arch_initcall(tegra_clocks_apply_init_table)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
