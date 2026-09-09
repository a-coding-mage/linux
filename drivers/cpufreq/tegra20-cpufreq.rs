// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 *	Based on arch/arm/plat-omap/cpu-omap.c, (C) 2005 Nokia Corporation
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct tegra_sku_info_type {
    pub cpu_process_id: u32,
    pub soc_speedo_id: u32,
    pub cpu_speedo_id: u32,
}

extern "C" {
    fn of_cpu_device_node_get(cpu: c_int) -> *mut device_node;
    fn of_property_present(np: *mut device_node, name: *const c_char) -> bool;
    fn of_node_put(np: *mut device_node);
    fn dev_pm_opp_put_supported_hw(token: usize);
    fn platform_device_unregister(pdev: *mut c_void);
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn get_cpu_device(cpu: c_int) -> *mut device;
    fn dev_pm_opp_set_supported_hw(dev: *mut device, versions: *const u32, count: u32) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut c_void,
        num: u32,
    ) -> *mut platform_device;
    fn ptr_err_or_zero(ptr: *mut platform_device) -> c_int;
    fn dev_err(dev: *const device, fmt: *const c_char, ...);
    fn dev_info(dev: *const device, fmt: *const c_char, ...);
    fn warn_on(condition: bool) -> bool;
}

extern "C" {
    static mut tegra_sku_info: tegra_sku_info_type;
}

const ENODEV: c_int = 19;

unsafe extern "C" fn cpu0_node_has_opp_v2_prop() -> bool {
    let np = of_cpu_device_node_get(0);
    let mut ret = false;

    if of_property_present(np, c"operating-points-v2".as_ptr()) {
        ret = true;
    }

    of_node_put(np);
    ret
}

unsafe extern "C" fn tegra20_cpufreq_put_supported_hw(opp_token: *mut c_void) {
    dev_pm_opp_put_supported_hw(opp_token as usize);
}

unsafe extern "C" fn tegra20_cpufreq_dt_unregister(cpufreq_dt: *mut c_void) {
    platform_device_unregister(cpufreq_dt);
}

unsafe extern "C" fn tegra20_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    let mut cpufreq_dt: *mut platform_device;
    let mut cpu_dev: *mut device;
    let mut versions = [0u32; 2];
    let mut err: c_int;

    if !cpu0_node_has_opp_v2_prop() {
        dev_err((&(*pdev).dev) as *const device, c"operating points not found\n".as_ptr());
        dev_err((&(*pdev).dev) as *const device, c"please update your device tree\n".as_ptr());
        return -ENODEV;
    }

    if of_machine_is_compatible(c"nvidia,tegra20".as_ptr()) {
        versions[0] = 1u32.wrapping_shl(tegra_sku_info.cpu_process_id);
        versions[1] = 1u32.wrapping_shl(tegra_sku_info.soc_speedo_id);
    } else {
        versions[0] = 1u32.wrapping_shl(tegra_sku_info.cpu_process_id);
        versions[1] = 1u32.wrapping_shl(tegra_sku_info.cpu_speedo_id);
    }

    dev_info((&(*pdev).dev) as *const device, c"hardware version 0x%x 0x%x\n".as_ptr(), versions[0], versions[1]);

    cpu_dev = get_cpu_device(0);
    if warn_on(cpu_dev.is_null()) {
        return -ENODEV;
    }

    err = dev_pm_opp_set_supported_hw(cpu_dev, versions.as_ptr(), 2);
    if err < 0 {
        dev_err((&(*pdev).dev) as *const device, c"failed to set supported hw: %d\n".as_ptr(), err);
        return err;
    }

    err = devm_add_action_or_reset(
        (&mut (*pdev).dev) as *mut device,
        Some(tegra20_cpufreq_put_supported_hw),
        err as usize as *mut c_void,
    );
    if err != 0 {
        return err;
    }

    cpufreq_dt = platform_device_register_simple(c"cpufreq-dt".as_ptr(), -1, core::ptr::null_mut(), 0);
    err = ptr_err_or_zero(cpufreq_dt);
    if err != 0 {
        dev_err((&(*pdev).dev) as *const device, c"failed to create cpufreq-dt device: %d\n".as_ptr(), err);
        return err;
    }

    err = devm_add_action_or_reset(
        (&mut (*pdev).dev) as *mut device,
        Some(tegra20_cpufreq_dt_unregister),
        cpufreq_dt as *mut c_void,
    );
    if err != 0 {
        return err;
    }

    0
}

static mut tegra20_cpufreq_driver: platform_driver = platform_driver {
    probe: Some(tegra20_cpufreq_probe),
    driver: driver {
        name: c"tegra20-cpufreq".as_ptr(),
    },
};

// module_platform_driver(tegra20_cpufreq_driver);
// MODULE_ALIAS("platform:tegra20-cpufreq");
// MODULE_AUTHOR("Colin Cross <ccross@android.com>");
// MODULE_DESCRIPTION("NVIDIA Tegra20 cpufreq driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
