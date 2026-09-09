// SPDX-License-Identifier: GPL-2.0-only
/*
 * Tegra 124 cpufreq driver
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
pub struct platform_driver { _private: [u8; 0] }

extern "C" {
    fn clk_set_rate(clk: *mut clk, rate: u64) -> i32;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> i32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn of_cpu_device_node_get(cpu: u32) -> *mut device_node;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn get_cpu_device(cpu: u32) -> *mut device;
    fn of_clk_get_by_name(np: *mut device_node, name: *const i8) -> *mut clk;
    fn cpufreq_dt_pdev_register(dev: *mut device) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn clk_put(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn disable_cpufreq();
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(drv: *mut platform_driver) -> i32;
    fn platform_driver_unregister(drv: *mut platform_driver);
    fn platform_device_register_simple(name: *const i8, id: i32, data: *mut c_void, size: u32) -> *mut platform_device;
    fn of_machine_is_compatible(compat: *const i8) -> bool;
}

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0x400cc0;

static mut tegra124_cpufreq_pdev: *mut platform_device = core::ptr::null_mut();

#[repr(C)]
pub struct tegra124_cpufreq_priv {
    cpu_clk: *mut clk,
    pllp_clk: *mut clk,
    pllx_clk: *mut clk,
    dfll_clk: *mut clk,
    cpufreq_dt_pdev: *mut platform_device,
}

unsafe fn tegra124_cpu_switch_to_dfll(priv_: *mut tegra124_cpufreq_priv) -> i32 {
    let orig_parent: *mut clk;
    let ret = clk_set_rate((*priv_).dfll_clk, clk_get_rate((*priv_).cpu_clk));
    if ret != 0 { return ret; }

    orig_parent = clk_get_parent((*priv_).cpu_clk);
    clk_set_parent((*priv_).cpu_clk, (*priv_).pllp_clk);

    let ret = clk_prepare_enable((*priv_).dfll_clk);
    if ret != 0 {
        clk_set_parent((*priv_).cpu_clk, orig_parent);
        return ret;
    }

    clk_set_parent((*priv_).cpu_clk, (*priv_).dfll_clk);
    0
}

unsafe fn tegra124_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    let np = of_cpu_device_node_get(0);
    let mut priv_: *mut tegra124_cpufreq_priv;
    let cpu_dev: *mut device;
    let ret: i32;

    if np.is_null() { return -ENODEV; }
    priv_ = devm_kzalloc(pdev as *mut device, core::mem::size_of::<tegra124_cpufreq_priv>(), GFP_KERNEL) as *mut tegra124_cpufreq_priv;
    if priv_.is_null() { return -ENOMEM; }
    cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() { return -ENODEV; }

    (*priv_).cpu_clk = of_clk_get_by_name(np, b"cpu_g\0".as_ptr() as *const i8);
    if (*priv_).cpu_clk.is_null() { return -ENODEV; }
    (*priv_).dfll_clk = of_clk_get_by_name(np, b"dfll\0".as_ptr() as *const i8);
    if (*priv_).dfll_clk.is_null() { clk_put((*priv_).cpu_clk); return -ENODEV; }
    (*priv_).pllx_clk = of_clk_get_by_name(np, b"pll_x\0".as_ptr() as *const i8);
    if (*priv_).pllx_clk.is_null() { clk_put((*priv_).dfll_clk); clk_put((*priv_).cpu_clk); return -ENODEV; }
    (*priv_).pllp_clk = of_clk_get_by_name(np, b"pll_p\0".as_ptr() as *const i8);
    if (*priv_).pllp_clk.is_null() { clk_put((*priv_).pllx_clk); clk_put((*priv_).dfll_clk); clk_put((*priv_).cpu_clk); return -ENODEV; }

    ret = tegra124_cpu_switch_to_dfll(priv_);
    if ret != 0 { clk_put((*priv_).pllp_clk); clk_put((*priv_).pllx_clk); clk_put((*priv_).dfll_clk); clk_put((*priv_).cpu_clk); return ret; }
    (*priv_).cpufreq_dt_pdev = cpufreq_dt_pdev_register(cpu_dev);
    if (*priv_).cpufreq_dt_pdev.is_null() { clk_put((*priv_).pllp_clk); clk_put((*priv_).pllx_clk); clk_put((*priv_).dfll_clk); clk_put((*priv_).cpu_clk); return -ENODEV; }
    platform_set_drvdata(pdev, priv_ as *mut c_void);
    0
}

unsafe fn tegra124_cpufreq_suspend(dev: *mut device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut tegra124_cpufreq_priv;
    let err = clk_set_parent((*priv_).cpu_clk, (*priv_).pllp_clk);
    if err < 0 { dev_err(dev, b"failed to reparent to PLLP: %d\n\0".as_ptr() as *const i8, err); return err; }
    clk_disable_unprepare((*priv_).dfll_clk); 0
}

unsafe fn tegra124_cpufreq_resume(dev: *mut device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut tegra124_cpufreq_priv;
    let err = clk_prepare_enable((*priv_).dfll_clk);
    if err < 0 { dev_err(dev, b"failed to enable DFLL clock for CPU: %d\n\0".as_ptr() as *const i8, err); disable_cpufreq(); return err; }
    let err = clk_set_parent((*priv_).cpu_clk, (*priv_).dfll_clk);
    if err < 0 { dev_err(dev, b"failed to reparent to DFLL clock: %d\n\0".as_ptr() as *const i8, err); clk_disable_unprepare((*priv_).dfll_clk); disable_cpufreq(); return err; }
    0
}

unsafe fn tegra124_cpufreq_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(pdev as *mut device) as *mut tegra124_cpufreq_priv;
    if !(*priv_).cpufreq_dt_pdev.is_null() { platform_device_unregister((*priv_).cpufreq_dt_pdev); (*priv_).cpufreq_dt_pdev = core::ptr::null_mut(); }
    clk_put((*priv_).pllp_clk); clk_put((*priv_).pllx_clk); clk_put((*priv_).dfll_clk); clk_put((*priv_).cpu_clk);
}

unsafe fn tegra_cpufreq_init() -> i32 {
    if !(of_machine_is_compatible(b"nvidia,tegra114\0".as_ptr() as *const i8) || of_machine_is_compatible(b"nvidia,tegra124\0".as_ptr() as *const i8) || of_machine_is_compatible(b"nvidia,tegra210\0".as_ptr() as *const i8)) { return -ENODEV; }
    let ret = platform_driver_register(core::ptr::null_mut());
    if ret != 0 { return ret; }
    tegra124_cpufreq_pdev = platform_device_register_simple(b"cpufreq-tegra124\0".as_ptr() as *const i8, -1, core::ptr::null_mut(), 0);
    if tegra124_cpufreq_pdev.is_null() { platform_driver_unregister(core::ptr::null_mut()); return -ENODEV; }
    0
}

unsafe fn tegra_cpufreq_module_exit() {
    if !tegra124_cpufreq_pdev.is_null() { platform_device_unregister(tegra124_cpufreq_pdev); }
    platform_driver_unregister(core::ptr::null_mut());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
