// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Linaro Ltd.
 * Copyright (C) 2021 Dávid Virág <virag.david003@gmail.com>
 * Author: Sam Protsenko <semen.protsenko@linaro.org>
 * Author: Dávid Virág <virag.david003@gmail.com>
 *
 * This file contains shared functions used by some arm64 Exynos SoCs,
 * such as Exynos7885 or Exynos850 to register and init CMUs.
 */

/* External kernel/framework types and functions are supplied by dependencies. */

/* PLL register bits */
const PLL_CON1_MANUAL: u32 = 1 << 1;

/* Gate register bits */
const GATE_MANUAL: u32 = 1 << 20;
const GATE_ENABLE_HWACG: u32 = 1 << 28;

/* Option register bits */
const OPT_EN_MEM_PWR_GATING: u32 = 1 << 24;
const OPT_EN_AUTO_GATING: u32 = 1 << 28;
const OPT_EN_PWR_MANAGEMENT: u32 = 1 << 29;
const OPT_EN_LAYER2_CTRL: u32 = 1 << 30;
const OPT_EN_DBG: u32 = 1 << 31;

const CMU_OPT_GLOBAL_EN_AUTO_GATING: u32 = OPT_EN_DBG
    | OPT_EN_LAYER2_CTRL
    | OPT_EN_PWR_MANAGEMENT
    | OPT_EN_AUTO_GATING
    | OPT_EN_MEM_PWR_GATING;

const PLL_CON_OFF_START: usize = 0x100;
const PLL_CON_OFF_END: usize = 0x600;
const GATE_OFF_START: usize = 0x2000;
const GATE_OFF_END: usize = 0x2fff;

#[repr(C)]
struct exynos_arm64_cmu_data {
    clk_save: *mut samsung_clk_reg_dump,
    nr_clk_save: u32,
    clk_suspend: *const samsung_clk_reg_dump,
    nr_clk_suspend: u32,
    clk_sysreg_save: *mut samsung_clk_reg_dump,
    nr_clk_sysreg: u32,
    clk: *mut clk,
    pclks: *mut *mut clk,
    nr_pclks: i32,
    ctx: *mut samsung_clk_provider,
}

fn is_gate_reg(off: usize) -> bool {
    off >= GATE_OFF_START && off <= GATE_OFF_END
}

fn is_pll_conx_reg(off: usize) -> bool {
    off >= PLL_CON_OFF_START && off <= PLL_CON_OFF_END
}

fn is_pll_con1_reg(off: usize) -> bool {
    is_pll_conx_reg(off) && (off & 0xf) == 0x4 && (off & 0x10) == 0
}

unsafe fn exynos_arm64_init_clocks(
    np: *mut device_node,
    cmu: *const samsung_cmu_info,
) {
    let reg_offs = (*cmu).clk_regs;
    let reg_offs_len = (*cmu).nr_clk_regs as usize;
    let reg_base = of_iomap(np, 0);
    if reg_base.is_null() {
        panic!("exynos_arm64_init_clocks: failed to map registers\n");
    }

    let init_auto = (*cmu).auto_clock_gate && samsung_is_auto_capable(np);

    if (*cmu).option_offset != 0 && init_auto {
        writel(CMU_OPT_GLOBAL_EN_AUTO_GATING, reg_base.add((*cmu).option_offset as usize));
    }

    for i in 0..reg_offs_len {
        let off = *reg_offs.add(i) as usize;
        let reg = reg_base.add(off);
        if (*cmu).manual_plls && is_pll_con1_reg(off) {
            writel(PLL_CON1_MANUAL, reg);
        } else if is_gate_reg(off) && !init_auto {
            let mut val = readl(reg);
            val |= GATE_MANUAL;
            val &= !GATE_ENABLE_HWACG;
            writel(val, reg);
        }
    }
    iounmap(reg_base);
}

unsafe fn exynos_arm64_enable_bus_clk(
    dev: *mut device,
    np: *mut device_node,
    cmu: *const samsung_cmu_info,
) -> i32 {
    if (*cmu).clk_name.is_null() { return 0; }
    let parent_clk;
    if !dev.is_null() {
        parent_clk = clk_get(dev, (*cmu).clk_name);
        let data = dev_get_drvdata(dev) as *mut exynos_arm64_cmu_data;
        if !data.is_null() { (*data).clk = parent_clk; }
    } else {
        parent_clk = of_clk_get_by_name(np, (*cmu).clk_name);
    }
    if IS_ERR(parent_clk) { return PTR_ERR(parent_clk); }
    clk_prepare_enable(parent_clk)
}

unsafe fn exynos_arm64_cmu_prepare_pm(dev: *mut device, cmu: *const samsung_cmu_info) -> i32 {
    let data = dev_get_drvdata(dev) as *mut exynos_arm64_cmu_data;
    let mut ret: i32;
    (*data).clk_save = samsung_clk_alloc_reg_dump((*cmu).clk_regs, (*cmu).nr_clk_regs);
    if (*data).clk_save.is_null() { return -12; }
    (*data).nr_clk_save = (*cmu).nr_clk_regs;
    if (*cmu).nr_sysreg_clk_regs != 0 {
        (*data).clk_sysreg_save = samsung_clk_alloc_reg_dump((*cmu).sysreg_clk_regs, (*cmu).nr_sysreg_clk_regs);
        if (*data).clk_sysreg_save.is_null() { ret = -12; goto free_clk_save; }
        (*data).nr_clk_sysreg = (*cmu).nr_sysreg_clk_regs;
    }
    (*data).clk_suspend = (*cmu).suspend_regs;
    (*data).nr_clk_suspend = (*cmu).nr_suspend_regs;
    (*data).nr_pclks = of_clk_get_parent_count((*dev).of_node);
    if (*data).nr_pclks == 0 { return 0; }
    (*data).pclks = devm_kcalloc(dev, core::mem::size_of::<*mut clk>(), (*data).nr_pclks as usize, GFP_KERNEL);
    if (*data).pclks.is_null() { ret = -12; goto free_sysreg_save; }
    for i in 0..(*data).nr_pclks {
        let clk = of_clk_get((*dev).of_node, i);
        if IS_ERR(clk) {
            let mut j = i;
            while j > 0 { j -= 1; clk_put(*(*data).pclks.add(j as usize)); }
            ret = PTR_ERR(clk); goto free_sysreg_save;
        }
        *(*data).pclks.add(i as usize) = clk;
    }
    return 0;
free_sysreg_save:
    kfree((*data).clk_sysreg_save as *mut core::ffi::c_void);
free_clk_save:
    kfree((*data).clk_save as *mut core::ffi::c_void);
    ret
}

pub unsafe fn exynos_arm64_register_cmu(dev: *mut device, np: *mut device_node, cmu: *const samsung_cmu_info) {
    let err = exynos_arm64_enable_bus_clk(dev, np, cmu);
    if err != 0 { pr_err("exynos_arm64_register_cmu: could not enable bus clock %s; err = %d\n", (*cmu).clk_name, err); }
    exynos_arm64_init_clocks(np, cmu);
    samsung_cmu_register_one(np, cmu);
}

pub unsafe fn exynos_arm64_register_cmu_pm(pdev: *mut platform_device, init_clk_regs: bool) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let np = (*dev).of_node;
    let cmu = of_device_get_match_data(dev) as *const samsung_cmu_info;
    let data = devm_kzalloc(dev, core::mem::size_of::<exynos_arm64_cmu_data>(), GFP_KERNEL) as *mut exynos_arm64_cmu_data;
    if data.is_null() { return -12; }
    platform_set_drvdata(pdev, data as *mut core::ffi::c_void);
    let ret = exynos_arm64_cmu_prepare_pm(dev, cmu); if ret != 0 { return ret; }
    let ret = exynos_arm64_enable_bus_clk(dev, core::ptr::null_mut(), cmu);
    if ret != 0 { dev_err(dev, "exynos_arm64_register_cmu_pm: could not enable bus clock %s; err = %d\n", (*cmu).clk_name, ret); }
    if init_clk_regs { exynos_arm64_init_clocks(np, cmu); }
    let reg_base = devm_platform_ioremap_resource(pdev, 0); if IS_ERR(reg_base) { return PTR_ERR(reg_base); }
    (*data).ctx = samsung_clk_init(dev, reg_base, (*cmu).nr_clk_ids);
    pm_runtime_get_noresume(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev);
    samsung_cmu_register_clocks((*data).ctx, cmu, np);
    samsung_clk_of_add_provider((*dev).of_node, (*data).ctx);
    samsung_en_dyn_root_clk_gating(np, (*data).ctx, cmu, true);
    pm_runtime_put_sync(dev); 0
}

pub unsafe fn exynos_arm64_cmu_suspend(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut exynos_arm64_cmu_data;
    samsung_clk_save((*(*data).ctx).reg_base, core::ptr::null_mut(), (*data).clk_save, (*data).nr_clk_save);
    samsung_clk_save(core::ptr::null_mut(), (*(*data).ctx).sysreg, (*data).clk_sysreg_save, (*data).nr_clk_sysreg);
    for i in 0..(*data).nr_pclks { clk_prepare_enable(*(*data).pclks.add(i as usize)); }
    samsung_clk_restore((*(*data).ctx).reg_base, core::ptr::null_mut(), (*data).clk_suspend, (*data).nr_clk_suspend);
    for i in 0..(*data).nr_pclks { clk_disable_unprepare(*(*data).pclks.add(i as usize)); }
    clk_disable_unprepare((*data).clk); 0
}

pub unsafe fn exynos_arm64_cmu_resume(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut exynos_arm64_cmu_data;
    clk_prepare_enable((*data).clk);
    for i in 0..(*data).nr_pclks { clk_prepare_enable(*(*data).pclks.add(i as usize)); }
    samsung_clk_restore((*(*data).ctx).reg_base, core::ptr::null_mut(), (*data).clk_save, (*data).nr_clk_save);
    if !(*(*data).ctx).sysreg.is_null() { samsung_clk_restore(core::ptr::null_mut(), (*(*data).ctx).sysreg, (*data).clk_sysreg_save, (*data).nr_clk_sysreg); }
    for i in 0..(*data).nr_pclks { clk_disable_unprepare(*(*data).pclks.add(i as usize)); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
