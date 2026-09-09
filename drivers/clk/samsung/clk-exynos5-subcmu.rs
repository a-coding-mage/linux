// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 Samsung Electronics Co., Ltd.
// Author: Marek Szyprowski <m.szyprowski@samsung.com>
// Common Clock Framework support for Exynos5 power-domain dependent clocks

// Linux headers and local clock headers provide the external types, functions,
// constants, and macros referenced by this translation.

static mut ctx: *mut samsung_clk_provider = core::ptr::null_mut();
static mut cmu: *const *const exynos5_subcmu_info = core::ptr::null();
static mut nr_cmus: i32 = 0;

unsafe fn exynos5_subcmu_clk_save(
    base: *mut core::ffi::c_void,
    mut rd: *mut exynos5_subcmu_reg_dump,
    mut num_regs: u32,
) {
    while num_regs > 0 {
        (*rd).save = readl(base.add((*rd).offset as usize));
        writel(
            ((*rd).save & !(*rd).mask) | (*rd).value,
            base.add((*rd).offset as usize),
        );
        (*rd).save &= (*rd).mask;
        num_regs -= 1;
        rd = rd.add(1);
    }
}

unsafe fn exynos5_subcmu_clk_restore(
    base: *mut core::ffi::c_void,
    mut rd: *mut exynos5_subcmu_reg_dump,
    mut num_regs: u32,
) {
    while num_regs > 0 {
        writel(
            (readl(base.add((*rd).offset as usize)) & !(*rd).mask) | (*rd).save,
            base.add((*rd).offset as usize),
        );
        num_regs -= 1;
        rd = rd.add(1);
    }
}

unsafe fn exynos5_subcmu_defer_gate(
    provider: *mut samsung_clk_provider,
    mut list: *const samsung_gate_clock,
    mut nr_clk: i32,
) {
    while nr_clk != 0 {
        samsung_clk_add_lookup(provider, ERR_PTR(-EPROBE_DEFER), (*list).id);
        list = list.add(1);
        nr_clk -= 1;
    }
}

/*
 * Pass the needed clock provider context and register sub-CMU clocks
 *
 * NOTE: This function has to be called from the main, CLK_OF_DECLARE-
 * initialized clock provider driver. This happens very early during boot
 * process. Then this driver, during core_initcall registers two platform
 * drivers: one which binds to the same device-tree node as CLK_OF_DECLARE
 * driver and second, for handling its per-domain child-devices. Those
 * platform drivers are bound to their devices a bit later in arch_initcall,
 * when OF-core populates all device-tree nodes.
 */
pub unsafe fn exynos5_subcmus_init(
    _ctx: *mut samsung_clk_provider,
    mut _nr_cmus: i32,
    mut _cmu: *const *const exynos5_subcmu_info,
) {
    ctx = _ctx;
    cmu = _cmu;
    nr_cmus = _nr_cmus;

    while _nr_cmus != 0 {
        exynos5_subcmu_defer_gate(_ctx, (**_cmu).gate_clks, (**_cmu).nr_gate_clks);
        exynos5_subcmu_clk_save(
            (*_ctx).reg_base,
            (**_cmu).suspend_regs,
            (**_cmu).nr_suspend_regs,
        );
        _nr_cmus -= 1;
        _cmu = _cmu.add(1);
    }
}

unsafe fn exynos5_subcmu_suspend(dev: *mut device) -> i32 {
    let info = dev_get_drvdata(dev) as *mut exynos5_subcmu_info;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*ctx).lock, &mut flags);
    exynos5_subcmu_clk_save((*ctx).reg_base, (*info).suspend_regs, (*info).nr_suspend_regs);
    spin_unlock_irqrestore(&mut (*ctx).lock, flags);
    0
}

unsafe fn exynos5_subcmu_resume(dev: *mut device) -> i32 {
    let info = dev_get_drvdata(dev) as *mut exynos5_subcmu_info;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*ctx).lock, &mut flags);
    exynos5_subcmu_clk_restore((*ctx).reg_base, (*info).suspend_regs, (*info).nr_suspend_regs);
    spin_unlock_irqrestore(&mut (*ctx).lock, flags);
    0
}

unsafe fn exynos5_subcmu_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let info = dev_get_drvdata(dev) as *mut exynos5_subcmu_info;

    pm_runtime_set_suspended(dev);
    pm_runtime_enable(dev);
    pm_runtime_get(dev);
    (*ctx).dev = dev;
    samsung_clk_register_div(ctx, (*info).div_clks, (*info).nr_div_clks);
    samsung_clk_register_gate(ctx, (*info).gate_clks, (*info).nr_gate_clks);
    (*ctx).dev = core::ptr::null_mut();
    pm_runtime_put_sync(dev);
    0
}

// SET_RUNTIME_PM_OPS(exynos5_subcmu_suspend, exynos5_subcmu_resume, NULL)
// SET_LATE_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static exynos5_subcmu_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(exynos5_subcmu_suspend),
    runtime_resume: Some(exynos5_subcmu_resume),
    ..Default::default()
};

static exynos5_subcmu_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "exynos5-subcmu",
        suppress_bind_attrs: true,
        pm: &exynos5_subcmu_pm_ops,
        ..Default::default()
    },
    probe: Some(exynos5_subcmu_probe),
    ..Default::default()
};

unsafe fn exynos5_clk_register_subcmu(
    parent: *mut device,
    info: *const exynos5_subcmu_info,
    pd_node: *mut device_node,
) -> i32 {
    let genpdspec = of_phandle_args { np: pd_node, ..Default::default() };
    let pdev = platform_device_alloc("exynos5-subcmu", PLATFORM_DEVID_AUTO);
    if pdev.is_null() { return -ENOMEM; }
    (*pdev).dev.parent = parent;
    platform_set_drvdata(pdev, info as *mut core::ffi::c_void);
    of_genpd_add_device(&genpdspec, &mut (*pdev).dev);
    let ret = platform_device_add(pdev);
    if ret != 0 { platform_device_put(pdev); }
    ret
}

unsafe fn exynos5_clk_probe(pdev: *mut platform_device) -> i32 {
    // for_each_compatible_node(np, NULL, "samsung,exynos4210-pd")
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut name: *const i8 = core::ptr::null();
    while (np = for_each_compatible_node(np, core::ptr::null_mut(), "samsung,exynos4210-pd"), !np.is_null()) {
        if of_property_read_string(np, "label", &mut name) < 0 { continue; }
        for i in 0..nr_cmus {
            if strcmp((**cmu.add(i as usize)).pd_name, name) == 0 {
                exynos5_clk_register_subcmu(&mut (*pdev).dev, *cmu.add(i as usize), np);
            }
        }
    }
    0
}

static exynos5_clk_of_match: [of_device_id; 4] = [
    of_device_id { compatible: "samsung,exynos5250-clock", ..Default::default() },
    of_device_id { compatible: "samsung,exynos5420-clock", ..Default::default() },
    of_device_id { compatible: "samsung,exynos5800-clock", ..Default::default() },
    of_device_id { ..Default::default() },
];

static exynos5_clk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "exynos5-clock",
        of_match_table: exynos5_clk_of_match.as_ptr(),
        suppress_bind_attrs: true,
        ..Default::default()
    },
    probe: Some(exynos5_clk_probe),
    ..Default::default()
};

unsafe fn exynos5_clk_drv_init() -> i32 {
    platform_driver_register(&exynos5_clk_driver);
    platform_driver_register(&exynos5_subcmu_driver);
    0
}

// core_initcall(exynos5_clk_drv_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
