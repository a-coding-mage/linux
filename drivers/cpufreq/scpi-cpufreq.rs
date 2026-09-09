// SPDX-License-Identifier: GPL-2.0-only
/*
 * System Control and Power Interface (SCPI) based CPUFreq Interface driver
 *
 * Copyright (C) 2015 ARM Ltd.
 * Sudeep Holla <sudeep.holla@arm.com>
 */

// Kernel headers and build-time definitions are supplied by the surrounding
// translation unit.

#[repr(C)]
struct ScpiData {
    clk: *mut clk,
    cpu_dev: *mut device,
}

static mut scpi_ops: *mut scpi_ops = core::ptr::null_mut();

unsafe fn scpi_cpufreq_get_rate(cpu: u32) -> c_uint {
    let policy: *mut cpufreq_policy = cpufreq_cpu_get_raw(cpu);
    if policy.is_null() {
        return 0;
    }

    let priv_: *mut ScpiData = (*policy).driver_data as *mut ScpiData;
    let rate: c_ulong = clk_get_rate((*priv_).clk);
    rate / 1000
}

unsafe fn scpi_cpufreq_set_target(policy: *mut cpufreq_policy, index: u32) -> c_int {
    let freq_khz: c_ulong = (*(*policy).freq_table.add(index as usize)).frequency;
    let priv_: *mut ScpiData = (*policy).driver_data as *mut ScpiData;
    let rate: c_ulong = freq_khz * 1000;
    let ret: c_int = clk_set_rate((*priv_).clk, rate);

    if ret != 0 {
        return ret;
    }

    if clk_get_rate((*priv_).clk) / 1000 != freq_khz {
        return -EIO;
    }

    0
}

unsafe fn scpi_get_sharing_cpus(cpu_dev: *mut device, cpumask: *mut cpumask) -> c_int {
    let domain: c_int = (*scpi_ops).device_domain_id(cpu_dev);
    if domain < 0 {
        return domain;
    }

    for_each_present_cpu!(cpu, {
        if cpu == (*cpu_dev).id as u32 {
            continue;
        }

        let tcpu_dev: *mut device = get_cpu_device(cpu);
        if tcpu_dev.is_null() {
            continue;
        }

        let tdomain: c_int = (*scpi_ops).device_domain_id(tcpu_dev);
        if tdomain == domain {
            cpumask_set_cpu(cpu, cpumask);
        }
    });

    0
}

unsafe fn scpi_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    let mut ret: c_int;
    let mut latency: c_uint;
    let cpu_dev: *mut device;
    let priv_: *mut ScpiData;
    let mut freq_table: *mut cpufreq_frequency_table = core::ptr::null_mut();

    cpu_dev = get_cpu_device((*policy).cpu);
    if cpu_dev.is_null() {
        pr_err!("failed to get cpu%d device\n", (*policy).cpu);
        return -ENODEV;
    }

    ret = (*scpi_ops).add_opps_to_device(cpu_dev);
    if ret != 0 {
        dev_warn!(cpu_dev, "failed to add opps to the device\n");
        return ret;
    }

    ret = scpi_get_sharing_cpus(cpu_dev, (*policy).cpus);
    if ret != 0 {
        dev_warn!(cpu_dev, "failed to get sharing cpumask\n");
        return ret;
    }

    ret = dev_pm_opp_set_sharing_cpus(cpu_dev, (*policy).cpus);
    if ret != 0 {
        dev_err!(cpu_dev, "%s: failed to mark OPPs as shared: %d\n", __func__, ret);
        return ret;
    }

    ret = dev_pm_opp_get_opp_count(cpu_dev);
    if ret <= 0 {
        dev_dbg!(cpu_dev, "OPP table is not ready, deferring probe\n");
        ret = -EPROBE_DEFER;
        goto out_free_opp;
    }

    priv_ = kzalloc_obj!(ScpiData);
    if priv_.is_null() {
        ret = -ENOMEM;
        goto out_free_opp;
    }

    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &mut freq_table);
    if ret != 0 {
        dev_err!(cpu_dev, "failed to init cpufreq table: %d\n", ret);
        goto out_free_priv;
    }

    (*priv_).cpu_dev = cpu_dev;
    (*priv_).clk = clk_get(cpu_dev, core::ptr::null());
    if IS_ERR!((*priv_).clk) {
        dev_err!(cpu_dev, "%s: Failed to get clk for cpu: %d\n", __func__, (*cpu_dev).id);
        ret = PTR_ERR!((*priv_).clk);
        goto out_free_cpufreq_table;
    }

    (*policy).driver_data = priv_ as *mut c_void;
    (*policy).freq_table = freq_table;
    (*policy).dvfs_possible_from_any_cpu = true;

    latency = (*scpi_ops).get_transition_latency(cpu_dev);
    if latency == 0 {
        latency = CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS;
    }
    (*policy).cpuinfo.transition_latency = latency;
    (*policy).fast_switch_possible = false;
    return 0;

out_free_cpufreq_table:
    dev_pm_opp_free_cpufreq_table(cpu_dev, &mut freq_table);
out_free_priv:
    kfree(priv_ as *mut c_void);
out_free_opp:
    dev_pm_opp_remove_all_dynamic(cpu_dev);
    ret
}

unsafe fn scpi_cpufreq_exit(policy: *mut cpufreq_policy) {
    let priv_: *mut ScpiData = (*policy).driver_data as *mut ScpiData;
    clk_put((*priv_).clk);
    dev_pm_opp_free_cpufreq_table((*priv_).cpu_dev, &mut (*policy).freq_table);
    dev_pm_opp_remove_all_dynamic((*priv_).cpu_dev);
    kfree(priv_ as *mut c_void);
}

static mut scpi_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: b"scpi-cpufreq\0".as_ptr() as *const c_char,
    flags: CPUFREQ_HAVE_GOVERNOR_PER_POLICY | CPUFREQ_NEED_INITIAL_FREQ_CHECK | CPUFREQ_IS_COOLING_DEV,
    verify: Some(cpufreq_generic_frequency_table_verify),
    get: Some(scpi_cpufreq_get_rate),
    init: Some(scpi_cpufreq_init),
    exit: Some(scpi_cpufreq_exit),
    target_index: Some(scpi_cpufreq_set_target),
    register_em: Some(cpufreq_register_em_with_opp),
};

unsafe fn scpi_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    scpi_ops = get_scpi_ops();
    if scpi_ops.is_null() {
        return -EIO;
    }

    ret = cpufreq_register_driver(&mut scpi_cpufreq_driver);
    if ret != 0 {
        dev_err!(&mut (*pdev).dev, "%s: registering cpufreq failed, err: %d\n", __func__, ret);
    }
    ret
}

unsafe fn scpi_cpufreq_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut scpi_cpufreq_driver);
    scpi_ops = core::ptr::null_mut();
}

static mut scpi_cpufreq_platdrv: platform_driver = platform_driver {
    driver: driver { name: b"scpi-cpufreq\0".as_ptr() as *const c_char },
    probe: Some(scpi_cpufreq_probe),
    remove: Some(scpi_cpufreq_remove),
};

module_platform_driver!(scpi_cpufreq_platdrv);
module_alias!("platform:scpi-cpufreq");
module_author!("Sudeep Holla <sudeep.holla@arm.com>");
module_description!("ARM SCPI CPUFreq interface driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
