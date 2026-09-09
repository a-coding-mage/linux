// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Linux kernel dependencies supplied by the surrounding translation.

const SUN9I_MMC_WIDTH: usize = 4;
const SUN9I_MMC_GATE_BIT: u32 = 16;
const SUN9I_MMC_RESET_BIT: u32 = 18;

#[repr(C)]
struct Sun9iMmcClkData {
    lock: spinlock_t,
    membase: *mut core::ffi::c_void,
    clk: *mut clk,
    reset: *mut reset_control,
    clk_data: clk_onecell_data,
    rcdev: reset_controller_dev,
}

unsafe fn sun9i_mmc_reset_assert(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    let data = container_of!(rcdev, Sun9iMmcClkData, rcdev);
    let mut flags: c_ulong = 0;
    let reg = ( (*data).membase as *mut u8)
        .add(SUN9I_MMC_WIDTH.wrapping_mul(id as usize)) as *mut u32;
    let val: u32;

    clk_prepare_enable((*data).clk);
    spin_lock_irqsave(&mut (*data).lock, &mut flags);

    val = readl(reg);
    writel(val & !(1u32 << SUN9I_MMC_RESET_BIT), reg);

    spin_unlock_irqrestore(&mut (*data).lock, flags);
    clk_disable_unprepare((*data).clk);

    0
}

unsafe fn sun9i_mmc_reset_deassert(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    let data = container_of!(rcdev, Sun9iMmcClkData, rcdev);
    let mut flags: c_ulong = 0;
    let reg = ((*data).membase as *mut u8)
        .add(SUN9I_MMC_WIDTH.wrapping_mul(id as usize)) as *mut u32;
    let val: u32;

    clk_prepare_enable((*data).clk);
    spin_lock_irqsave(&mut (*data).lock, &mut flags);

    val = readl(reg);
    writel(val | (1u32 << SUN9I_MMC_RESET_BIT), reg);

    spin_unlock_irqrestore(&mut (*data).lock, flags);
    clk_disable_unprepare((*data).clk);

    0
}

unsafe fn sun9i_mmc_reset_reset(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    sun9i_mmc_reset_assert(rcdev, id);
    udelay(10);
    sun9i_mmc_reset_deassert(rcdev, id);
    0
}

static SUN9I_MMC_RESET_OPS: reset_control_ops = reset_control_ops {
    assert: Some(sun9i_mmc_reset_assert),
    deassert: Some(sun9i_mmc_reset_deassert),
    reset: Some(sun9i_mmc_reset_reset),
};

unsafe fn sun9i_a80_mmc_config_clk_probe(pdev: *mut platform_device) -> c_int {
    let np = (*(*pdev).dev.of_node);
    let mut data: *mut Sun9iMmcClkData;
    let mut clk_data: *mut clk_onecell_data;
    let mut clk_name = np.name;
    let mut clk_parent: *const c_char;
    let mut r: *mut resource = core::ptr::null_mut();
    let (mut count, mut i, mut ret): (c_int, c_int, c_int);

    data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Sun9iMmcClkData>(), GFP_KERNEL);
    if data.is_null() { return -ENOMEM; }

    spin_lock_init(&mut (*data).lock);
    (*data).membase = devm_platform_get_and_ioremap_resource(pdev, 0, &mut r);
    if IS_ERR!((*data).membase) { return PTR_ERR!((*data).membase); }

    // one clock/reset pair per word
    count = DIV_ROUND_UP!(resource_size!(r), SUN9I_MMC_WIDTH as c_int);
    clk_data = &mut (*data).clk_data;
    (*clk_data).clk_num = count;
    (*clk_data).clks = devm_kcalloc(&mut (*pdev).dev, count as usize,
                                     core::mem::size_of::<*mut clk>(), GFP_KERNEL);
    if (*clk_data).clks.is_null() { return -ENOMEM; }

    (*data).clk = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!((*data).clk) { dev_err!(&mut (*pdev).dev, "Could not get clock\n"); return PTR_ERR!((*data).clk); }
    (*data).reset = devm_reset_control_get_exclusive(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!((*data).reset) { dev_err!(&mut (*pdev).dev, "Could not get reset control\n"); return PTR_ERR!((*data).reset); }

    ret = reset_control_deassert((*data).reset);
    if ret != 0 { dev_err!(&mut (*pdev).dev, "Reset deassert err %d\n", ret); return ret; }

    clk_parent = __clk_get_name((*data).clk);
    i = 0;
    while i < count {
        of_property_read_string_index(np, "clock-output-names", i, &mut clk_name);
        *(*clk_data).clks.add(i as usize) = clk_register_gate(
            &mut (*pdev).dev, clk_name, clk_parent, 0,
            ((*data).membase as *mut u8).add(SUN9I_MMC_WIDTH * i as usize) as *mut c_void,
            SUN9I_MMC_GATE_BIT, 0, &mut (*data).lock);
        if IS_ERR!(*(*clk_data).clks.add(i as usize)) {
            ret = PTR_ERR!(*(*clk_data).clks.add(i as usize));
            goto!(err_clk_register);
        }
        i += 1;
    }

    ret = of_clk_add_provider(np, of_clk_src_onecell_get, clk_data);
    if ret != 0 { goto!(err_clk_provider); }
    (*data).rcdev.owner = THIS_MODULE;
    (*data).rcdev.nr_resets = count as usize;
    (*data).rcdev.ops = &SUN9I_MMC_RESET_OPS;
    (*data).rcdev.of_node = (*pdev).dev.of_node;
    ret = reset_controller_register(&mut (*data).rcdev);
    if ret != 0 { goto!(err_rc_reg); }
    platform_set_drvdata(pdev, data);
    return 0;

err_rc_reg:
    of_clk_del_provider(np);
err_clk_provider:
    i = 0;
    while i < count { clk_unregister(*(*clk_data).clks.add(i as usize)); i += 1; }
err_clk_register:
    reset_control_assert((*data).reset);
    ret
}

static SUN9I_A80_MMC_CONFIG_CLK_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "allwinner,sun9i-a80-mmc-config-clk\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut SUN9I_A80_MMC_CONFIG_CLK_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: "sun9i-a80-mmc-config-clk\0".as_ptr() as *const c_char,
        suppress_bind_attrs: true,
        of_match_table: SUN9I_A80_MMC_CONFIG_CLK_DT_IDS.as_ptr(),
    },
    probe: Some(sun9i_a80_mmc_config_clk_probe),
};

builtin_platform_driver!(SUN9I_A80_MMC_CONFIG_CLK_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
