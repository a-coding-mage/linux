// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hisilicon clock driver
 *
 * Copyright (c) 2012-2013 Hisilicon Limited.
 * Copyright (c) 2012-2013 Linaro Limited.
 *
 * Author: Haojian Zhuang <haojian.zhuang@linaro.org>
 *         Xin Li <li.xin@linaro.org>
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut hisi_clk_lock: Spinlock = Spinlock::new();

pub unsafe fn hisi_clk_alloc(pdev: *mut platform_device, nr_clks: i32) -> *mut hisi_clock_data {
    let mut clk_data: *mut hisi_clock_data;
    let mut res: *mut resource;
    let mut clk_table: *mut *mut clk;

    clk_data = devm_kmalloc(&mut (*pdev).dev, core::mem::size_of::<hisi_clock_data>(), GFP_KERNEL);
    if clk_data.is_null() {
        return core::ptr::null_mut();
    }

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return core::ptr::null_mut();
    }
    (*clk_data).base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*clk_data).base.is_null() {
        return core::ptr::null_mut();
    }

    clk_table = devm_kmalloc_array(&mut (*pdev).dev, nr_clks as usize,
                                   core::mem::size_of::<*mut clk>(), GFP_KERNEL);
    if clk_table.is_null() {
        return core::ptr::null_mut();
    }

    (*clk_data).clk_data.clks = clk_table;
    (*clk_data).clk_data.clk_num = nr_clks;

    clk_data
}

pub unsafe fn hisi_clk_init(np: *mut device_node, nr_clks: i32) -> *mut hisi_clock_data {
    let mut clk_data: *mut hisi_clock_data;
    let mut clk_table: *mut *mut clk;
    let base: *mut core::ffi::c_void;

    base = of_iomap(np, 0);
    if base.is_null() {
        pr_err!("{}: failed to map clock registers\n", "hisi_clk_init");
        return core::ptr::null_mut();
    }

    clk_data = kzalloc_obj::<hisi_clock_data>();
    if clk_data.is_null() {
        iounmap(base);
        return core::ptr::null_mut();
    }

    (*clk_data).base = base;
    clk_table = kzalloc_objs::<*mut clk>(nr_clks as usize);
    if clk_table.is_null() {
        kfree(clk_data as *mut core::ffi::c_void);
        iounmap(base);
        return core::ptr::null_mut();
    }

    (*clk_data).clk_data.clks = clk_table;
    (*clk_data).clk_data.clk_num = nr_clks;
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*clk_data).clk_data);
    clk_data
}

pub unsafe fn hisi_clk_register_fixed_rate(clks: *const hisi_fixed_rate_clock, nums: i32,
                                           data: *mut hisi_clock_data) -> i32 {
    let mut clk: *mut clk = core::ptr::null_mut();
    let mut i = 0;
    while i < nums {
        let c = &*clks.add(i as usize);
        clk = clk_register_fixed_rate(core::ptr::null_mut(), c.name, c.parent_name, c.flags, c.fixed_rate);
        if IS_ERR(clk) {
            pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_fixed_rate", c.name);
            while i > 0 { i -= 1; clk_unregister_fixed_rate(*(*data).clk_data.clks.add((*clks.add(i as usize)).id as usize)); }
            return PTR_ERR(clk);
        }
        *(*data).clk_data.clks.add(c.id as usize) = clk;
        i += 1;
    }
    0
}

pub unsafe fn hisi_clk_register_fixed_factor(clks: *const hisi_fixed_factor_clock, nums: i32,
                                              data: *mut hisi_clock_data) -> i32 {
    let mut clk: *mut clk = core::ptr::null_mut();
    let mut i = 0;
    while i < nums {
        let c = &*clks.add(i as usize);
        clk = clk_register_fixed_factor(core::ptr::null_mut(), c.name, c.parent_name, c.flags, c.mult, c.div);
        if IS_ERR(clk) {
            pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_fixed_factor", c.name);
            while i > 0 { i -= 1; clk_unregister_fixed_factor(*(*data).clk_data.clks.add((*clks.add(i as usize)).id as usize)); }
            return PTR_ERR(clk);
        }
        *(*data).clk_data.clks.add(c.id as usize) = clk;
        i += 1;
    }
    0
}

pub unsafe fn hisi_clk_register_mux(clks: *const hisi_mux_clock, nums: i32, data: *mut hisi_clock_data) -> i32 {
    let base = (*data).base;
    let mut i = 0;
    while i < nums {
        let c = &*clks.add(i as usize);
        let mask = (1u32 << c.width) - 1;
        let clk = clk_register_mux_table(core::ptr::null_mut(), c.name, c.parent_names, c.num_parents, c.flags,
                                         base.add(c.offset as usize), c.shift, mask, c.mux_flags, c.table, &mut hisi_clk_lock);
        if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_mux", c.name); while i > 0 { i -= 1; clk_unregister_mux(*(*data).clk_data.clks.add((*clks.add(i as usize)).id as usize)); } return PTR_ERR(clk); }
        if !c.alias.is_null() { clk_register_clkdev(clk, c.alias, core::ptr::null()); }
        *(*data).clk_data.clks.add(c.id as usize) = clk; i += 1;
    }
    0
}

pub unsafe fn hisi_clk_register_phase(dev: *mut device, clks: *const hisi_phase_clock, nums: i32, data: *mut hisi_clock_data) -> i32 {
    let base = (*data).base; let mut i = 0;
    while i < nums { let c = &*clks.add(i as usize); let clk = clk_register_hisi_phase(dev, c, base, &mut hisi_clk_lock); if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_phase", c.name); return PTR_ERR(clk); } *(*data).clk_data.clks.add(c.id as usize) = clk; i += 1; } 0
}

pub unsafe fn hisi_clk_register_divider(clks: *const hisi_divider_clock, nums: i32, data: *mut hisi_clock_data) -> i32 {
    let base = (*data).base; let mut i = 0;
    while i < nums { let c = &*clks.add(i as usize); let clk = clk_register_divider_table(core::ptr::null_mut(), c.name, c.parent_name, c.flags, base.add(c.offset as usize), c.shift, c.width, c.div_flags, c.table, &mut hisi_clk_lock); if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_divider", c.name); while i > 0 { i -= 1; clk_unregister_divider(*(*data).clk_data.clks.add((*clks.add(i as usize)).id as usize)); } return PTR_ERR(clk); } if !c.alias.is_null() { clk_register_clkdev(clk, c.alias, core::ptr::null()); } *(*data).clk_data.clks.add(c.id as usize) = clk; i += 1; } 0
}

pub unsafe fn hisi_clk_register_gate(clks: *const hisi_gate_clock, nums: i32, data: *mut hisi_clock_data) -> i32 {
    let base = (*data).base; let mut i = 0;
    while i < nums { let c = &*clks.add(i as usize); let clk = clk_register_gate(core::ptr::null_mut(), c.name, c.parent_name, c.flags, base.add(c.offset as usize), c.bit_idx, c.gate_flags, &mut hisi_clk_lock); if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_gate", c.name); while i > 0 { i -= 1; clk_unregister_gate(*(*data).clk_data.clks.add((*clks.add(i as usize)).id as usize)); } return PTR_ERR(clk); } if !c.alias.is_null() { clk_register_clkdev(clk, c.alias, core::ptr::null()); } *(*data).clk_data.clks.add(c.id as usize) = clk; i += 1; } 0
}

pub unsafe fn hisi_clk_register_gate_sep(clks: *const hisi_gate_clock, nums: i32, data: *mut hisi_clock_data) {
    let base = (*data).base; for i in 0..nums { let c = &*clks.add(i as usize); let clk = hisi_register_clkgate_sep(core::ptr::null_mut(), c.name, c.parent_name, c.flags, base.add(c.offset as usize), c.bit_idx, c.gate_flags, &mut hisi_clk_lock); if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hisi_clk_register_gate_sep", c.name); continue; } if !c.alias.is_null() { clk_register_clkdev(clk, c.alias, core::ptr::null()); } *(*data).clk_data.clks.add(c.id as usize) = clk; }
}

pub unsafe fn hi6220_clk_register_divider(clks: *const hi6220_divider_clock, nums: i32, data: *mut hisi_clock_data) {
    let base = (*data).base; for i in 0..nums { let c = &*clks.add(i as usize); let clk = hi6220_register_clkdiv(core::ptr::null_mut(), c.name, c.parent_name, c.flags, base.add(c.offset as usize), c.shift, c.width, c.mask_bit, &mut hisi_clk_lock); if IS_ERR(clk) { pr_err!("{}: failed to register clock {}\n", "hi6220_clk_register_divider", c.name); continue; } if !c.alias.is_null() { clk_register_clkdev(clk, c.alias, core::ptr::null()); } *(*data).clk_data.clks.add(c.id as usize) = clk; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
