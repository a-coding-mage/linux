// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Atom platform clocks driver for BayTrail and CherryTrail SoCs
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Irina Tirdea <irina.tirdea@intel.com>
 */

// Linux kernel headers and symbols are supplied by the surrounding build.

const PLT_CLK_NAME_BASE: *const u8 = b"pmc_plt_clk\0".as_ptr();

#[repr(C)]
struct clk_plt_fixed {
    clk: *mut clk_hw,
    lookup: *mut clk_lookup,
}

#[repr(C)]
struct clk_plt {
    hw: clk_hw,
    reg: *mut core::ffi::c_void,
    lookup: *mut clk_lookup,
    lock: spinlock_t,
}

#[repr(C)]
struct clk_plt_data {
    parents: *mut *mut clk_plt_fixed,
    nparents: u8,
    clks: [*mut clk_plt; PMC_CLK_NUM as usize],
    mclk_lookup: *mut clk_lookup,
    ether_clk_lookup: *mut clk_lookup,
}

unsafe fn to_clk_plt(hw: *mut clk_hw) -> *mut clk_plt {
    hw as *mut clk_plt
}

unsafe fn plt_reg_to_parent(reg: i32) -> i32 {
    match reg & PMC_MASK_CLK_FREQ {
        PMC_CLK_FREQ_PLL => 1,
        PMC_CLK_FREQ_XTAL => 0,
        _ => 0,
    }
}

unsafe fn plt_parent_to_reg(index: i32) -> i32 {
    match index {
        1 => PMC_CLK_FREQ_PLL,
        0 => PMC_CLK_FREQ_XTAL,
        _ => PMC_CLK_FREQ_XTAL,
    }
}

unsafe fn plt_reg_to_enabled(reg: i32) -> i32 {
    match reg & PMC_MASK_CLK_CTL {
        PMC_CLK_CTL_GATED_ON_D3 | PMC_CLK_CTL_FORCE_ON => 1,
        PMC_CLK_CTL_FORCE_OFF | PMC_CLK_CTL_RESERVED => 0,
        _ => 0,
    }
}

unsafe fn plt_clk_reg_update(clk: *mut clk_plt, mask: u32, val: u32) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*clk).lock, &mut flags);
    let tmp = readl((*clk).reg as *const u32);
    writel((tmp & !mask) | (val & mask), (*clk).reg as *mut u32);
    spin_unlock_irqrestore(&mut (*clk).lock, flags);
}

unsafe extern "C" fn plt_clk_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    plt_clk_reg_update(to_clk_plt(hw), PMC_MASK_CLK_FREQ as u32,
                       plt_parent_to_reg(index as i32) as u32);
    0
}

unsafe extern "C" fn plt_clk_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = to_clk_plt(hw);
    plt_reg_to_parent(readl((*clk).reg as *const u32) as i32) as u8
}

unsafe extern "C" fn plt_clk_enable(hw: *mut clk_hw) -> i32 {
    plt_clk_reg_update(to_clk_plt(hw), PMC_MASK_CLK_CTL as u32,
                       PMC_CLK_CTL_FORCE_ON as u32);
    0
}

unsafe extern "C" fn plt_clk_disable(hw: *mut clk_hw) {
    plt_clk_reg_update(to_clk_plt(hw), PMC_MASK_CLK_CTL as u32,
                       PMC_CLK_CTL_FORCE_OFF as u32);
}

unsafe extern "C" fn plt_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    plt_reg_to_enabled(readl((*to_clk_plt(hw)).reg as *const u32) as i32)
}

static plt_clk_ops: clk_ops = clk_ops {
    enable: Some(plt_clk_enable),
    disable: Some(plt_clk_disable),
    is_enabled: Some(plt_clk_is_enabled),
    get_parent: Some(plt_clk_get_parent),
    set_parent: Some(plt_clk_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

unsafe fn plt_clk_register(pdev: *mut platform_device, id: i32,
                           pmc_data: *const pmc_clk_data,
                           parent_names: *const *const u8,
                           num_parents: i32) -> *mut clk_plt {
    let mut pclk = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_plt>(), GFP_KERNEL) as *mut clk_plt;
    if pclk.is_null() { return ERR_PTR(-ENOMEM); }
    let name = kasprintf(GFP_KERNEL, b"%s_%d\0".as_ptr(), PLT_CLK_NAME_BASE, id);
    if name.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name; init.ops = &plt_clk_ops; init.flags = 0;
    init.parent_names = parent_names; init.num_parents = num_parents as u8;
    (*pclk).hw.init = &init;
    (*pclk).reg = ((*pmc_data).base as usize + PMC_CLK_CTL_OFFSET as usize + id as usize * PMC_CLK_CTL_SIZE as usize) as *mut core::ffi::c_void;
    spin_lock_init(&mut (*pclk).lock);
    if (*pmc_data).critical && plt_clk_is_enabled(&mut (*pclk).hw) != 0 { init.flags |= CLK_IS_CRITICAL; }
    let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*pclk).hw);
    if ret != 0 { pclk = ERR_PTR(ret); }
    else { (*pclk).lookup = clkdev_hw_create(&mut (*pclk).hw, init.name, core::ptr::null()); if (*pclk).lookup.is_null() { pclk = ERR_PTR(-ENOMEM); } }
    kfree(init.name as *mut core::ffi::c_void);
    pclk
}

unsafe fn plt_clk_unregister(pclk: *mut clk_plt) { clkdev_drop((*pclk).lookup); }

unsafe fn plt_clk_register_fixed_rate(pdev: *mut platform_device, name: *const u8,
                                      parent_name: *const u8, fixed_rate: c_ulong) -> *mut clk_plt_fixed {
    let pclk = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_plt_fixed>(), GFP_KERNEL) as *mut clk_plt_fixed;
    if pclk.is_null() { return ERR_PTR(-ENOMEM); }
    (*pclk).clk = clk_hw_register_fixed_rate(&mut (*pdev).dev, name, parent_name, 0, fixed_rate);
    if IS_ERR((*pclk).clk) { return ERR_CAST((*pclk).clk); }
    (*pclk).lookup = clkdev_hw_create((*pclk).clk, name, core::ptr::null());
    if (*pclk).lookup.is_null() { clk_hw_unregister_fixed_rate((*pclk).clk); return ERR_PTR(-ENOMEM); }
    pclk
}

unsafe fn plt_clk_unregister_fixed_rate(pclk: *mut clk_plt_fixed) { clkdev_drop((*pclk).lookup); clk_hw_unregister_fixed_rate((*pclk).clk); }
unsafe fn plt_clk_unregister_fixed_rate_loop(data: *mut clk_plt_data, mut i: u32) { while i != 0 { i -= 1; plt_clk_unregister_fixed_rate(*data.add(0).parents.add(i as usize)); } }
unsafe fn plt_clk_free_parent_names_loop(parent_names: *const *const u8, mut i: u32) { while i != 0 { i -= 1; kfree_const(*parent_names.add(i as usize)); } kfree(parent_names as *mut core::ffi::c_void); }
unsafe fn plt_clk_unregister_loop(data: *mut clk_plt_data, mut i: u32) { while i != 0 { i -= 1; plt_clk_unregister((*data).clks[i as usize]); } }

unsafe fn plt_clk_register_parents(pdev: *mut platform_device, data: *mut clk_plt_data, clks: *const pmc_clk) -> *const *const u8 {
    let mut nparents = 0usize;
    while !(*clks.add(nparents)).name.is_null() { nparents += 1; }
    (*data).nparents = 0;
    (*data).parents = devm_kcalloc(&mut (*pdev).dev, nparents, core::mem::size_of::<*mut clk_plt_fixed>(), GFP_KERNEL) as *mut *mut clk_plt_fixed;
    if (*data).parents.is_null() { return ERR_PTR(-ENOMEM); }
    let parent_names = kcalloc(nparents, core::mem::size_of::<*const u8>(), GFP_KERNEL) as *mut *const u8;
    if parent_names.is_null() { return ERR_PTR(-ENOMEM); }
    for i in 0..nparents {
        let c = clks.add(i);
        *(*data).parents.add(i) = plt_clk_register_fixed_rate(pdev, (*c).name, (*c).parent_name, (*c).freq);
        if IS_ERR(*(*data).parents.add(i)) {
            let err = PTR_ERR(*(*data).parents.add(i));
            plt_clk_unregister_fixed_rate_loop(data, i as u32);
            plt_clk_free_parent_names_loop(parent_names, i as u32);
            return ERR_PTR(err);
        }
        *parent_names.add(i) = kstrdup_const((*c).name, GFP_KERNEL);
    }
    (*data).nparents = nparents as u8;
    parent_names as *const *const u8
}

unsafe fn plt_clk_unregister_parents(data: *mut clk_plt_data) { plt_clk_unregister_fixed_rate_loop(data, (*data).nparents as u32); }

unsafe extern "C" fn plt_clk_probe(pdev: *mut platform_device) -> i32 {
    let pmc_data = dev_get_platdata(&mut (*pdev).dev) as *const pmc_clk_data;
    if pmc_data.is_null() || (*pmc_data).clks.is_null() { return -EINVAL; }
    let data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_plt_data>(), GFP_KERNEL) as *mut clk_plt_data;
    if data.is_null() { return -ENOMEM; }
    let parent_names = plt_clk_register_parents(pdev, data, (*pmc_data).clks);
    if IS_ERR(parent_names) { return PTR_ERR(parent_names); }
    let mut i = 0u32;
    while i < PMC_CLK_NUM as u32 {
        (*data).clks[i as usize] = plt_clk_register(pdev, i as i32, pmc_data, parent_names, (*data).nparents as i32);
        if IS_ERR((*data).clks[i as usize]) {
            let err = PTR_ERR((*data).clks[i as usize]); plt_clk_unregister_loop(data, i); plt_clk_unregister_parents(data); plt_clk_free_parent_names_loop(parent_names, (*data).nparents as u32); return err;
        }
        i += 1;
    }
    (*data).mclk_lookup = clkdev_hw_create(&mut (*(*data).clks[3]).hw, b"mclk\0".as_ptr(), core::ptr::null());
    if (*data).mclk_lookup.is_null() { plt_clk_unregister_loop(data, i); plt_clk_unregister_parents(data); plt_clk_free_parent_names_loop(parent_names, (*data).nparents as u32); return -ENOMEM; }
    (*data).ether_clk_lookup = clkdev_hw_create(&mut (*(*data).clks[4]).hw, b"ether_clk\0".as_ptr(), core::ptr::null());
    if (*data).ether_clk_lookup.is_null() { clkdev_drop((*data).mclk_lookup); plt_clk_unregister_loop(data, i); plt_clk_unregister_parents(data); plt_clk_free_parent_names_loop(parent_names, (*data).nparents as u32); return -ENOMEM; }
    plt_clk_free_parent_names_loop(parent_names, (*data).nparents as u32);
    platform_set_drvdata(pdev, data as *mut core::ffi::c_void); 0
}

unsafe extern "C" fn plt_clk_remove(pdev: *mut platform_device) {
    let data = platform_get_drvdata(pdev) as *mut clk_plt_data;
    clkdev_drop((*data).ether_clk_lookup); clkdev_drop((*data).mclk_lookup);
    plt_clk_unregister_loop(data, PMC_CLK_NUM as u32); plt_clk_unregister_parents(data);
}

static mut plt_clk_driver: platform_driver = platform_driver { driver: driver { name: b"clk-pmc-atom\0".as_ptr(), ..core::mem::zeroed() }, probe: Some(plt_clk_probe), remove: Some(plt_clk_remove), ..core::mem::zeroed() };

// External kernel types, constants, and functions are supplied by dependent translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
