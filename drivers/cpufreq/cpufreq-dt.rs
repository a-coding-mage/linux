// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 *
 * Copyright (C) 2014 Linaro.
 * Viresh Kumar <viresh.kumar@linaro.org>
 */

// Dependency intent: Linux kernel headers and "cpufreq-dt.h" provide the
// types, constants, functions, and macros referenced below.

#[repr(C)]
struct private_data {
    node: list_head,
    cpus: cpumask_var_t,
    cpu_dev: *mut device,
    freq_table: *mut cpufreq_frequency_table,
    have_static_opps: bool,
    opp_token: i32,
}

static mut priv_list: list_head = LIST_HEAD_INIT;

unsafe fn cpufreq_dt_find_data(cpu: i32) -> *mut private_data {
    let mut priv_: *mut private_data;
    list_for_each_entry!(priv_, &mut priv_list, node) {
        if cpumask_test_cpu(cpu, (*priv_).cpus) {
            return priv_;
        }
    }
    core::ptr::null_mut()
}

unsafe fn set_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let priv_: *mut private_data = (*policy).driver_data as *mut private_data;
    let freq: usize = (*(*policy).freq_table.add(index as usize)).frequency as usize;
    dev_pm_opp_set_rate((*priv_).cpu_dev, freq.wrapping_mul(1000))
}

/*
 * An earlier version of opp-v1 bindings used to name the regulator
 * "cpu0-supply", we still need to handle that for backwards compatibility.
 */
unsafe fn find_supply_name(dev: *mut device) -> *const i8 {
    let np: *mut device_node = of_node_get((*dev).of_node);
    let cpu: i32 = (*dev).id;

    /* This must be valid for sure */
    if WARN_ON(np.is_null()) {
        return core::ptr::null();
    }

    /* Try "cpu0" for older DTs */
    if cpu == 0 && of_property_present(np, c"cpu0-supply".as_ptr()) {
        return c"cpu0".as_ptr();
    }

    if of_property_present(np, c"cpu-supply".as_ptr()) {
        return c"cpu".as_ptr();
    }

    dev_dbg(dev, c"no regulator for cpu%d\n".as_ptr(), cpu);
    core::ptr::null()
}

unsafe fn cpufreq_init(policy: *mut cpufreq_policy) -> i32 {
    let priv_: *mut private_data;
    let cpu_dev: *mut device;
    let cpu_clk: *mut clk;
    let mut transition_latency: u32;
    let ret: i32;

    priv_ = cpufreq_dt_find_data((*policy).cpu);
    if priv_.is_null() {
        pr_err!("failed to find data for cpu{}\n", (*policy).cpu);
        return -ENODEV;
    }
    cpu_dev = (*priv_).cpu_dev;

    cpu_clk = clk_get(cpu_dev, core::ptr::null());
    if IS_ERR(cpu_clk) {
        ret = PTR_ERR(cpu_clk);
        dev_err(cpu_dev, c"%s: failed to get clk: %d\n".as_ptr(), __func__, ret);
        return ret;
    }

    transition_latency = dev_pm_opp_get_max_transition_latency(cpu_dev);
    if transition_latency == 0 {
        transition_latency = CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS;
    }

    cpumask_copy((*policy).cpus, (*priv_).cpus);
    (*policy).driver_data = priv_ as *mut core::ffi::c_void;
    (*policy).clk = cpu_clk;
    (*policy).freq_table = (*priv_).freq_table;
    (*policy).suspend_freq = dev_pm_opp_get_suspend_opp_freq(cpu_dev) / 1000;
    (*policy).cpuinfo.transition_latency = transition_latency;
    (*policy).dvfs_possible_from_any_cpu = true;
    0
}

unsafe fn cpufreq_online(_policy: *mut cpufreq_policy) -> i32 { 0 }
unsafe fn cpufreq_offline(_policy: *mut cpufreq_policy) -> i32 { 0 }

unsafe fn cpufreq_exit(policy: *mut cpufreq_policy) {
    clk_put((*policy).clk);
}

static mut dt_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK | CPUFREQ_IS_COOLING_DEV,
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(set_target),
    get: Some(cpufreq_generic_get),
    init: Some(cpufreq_init),
    exit: Some(cpufreq_exit),
    online: Some(cpufreq_online),
    offline: Some(cpufreq_offline),
    register_em: Some(cpufreq_register_em_with_opp),
    name: c"cpufreq-dt".as_ptr(),
    set_boost: Some(cpufreq_boost_set_sw),
    suspend: Some(cpufreq_generic_suspend),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn dt_cpufreq_early_init(dev: *mut device, cpu: i32) -> i32 {
    let mut priv_: *mut private_data;
    let cpu_dev: *mut device;
    let mut fallback = false;
    let mut reg_name: [*const i8; 2] = [core::ptr::null(), core::ptr::null()];
    let mut ret: i32;

    if !cpufreq_dt_find_data(cpu).is_null() { return 0; }
    cpu_dev = get_cpu_device(cpu);
    if cpu_dev.is_null() { return -EPROBE_DEFER; }
    priv_ = devm_kzalloc(dev, core::mem::size_of::<private_data>(), GFP_KERNEL) as *mut private_data;
    if priv_.is_null() { return -ENOMEM; }
    if !zalloc_cpumask_var(&mut (*priv_).cpus, GFP_KERNEL) { return -ENOMEM; }
    cpumask_set_cpu(cpu, (*priv_).cpus);
    (*priv_).cpu_dev = cpu_dev;
    reg_name[0] = find_supply_name(cpu_dev);
    if !reg_name[0].is_null() {
        (*priv_).opp_token = dev_pm_opp_set_regulators(cpu_dev, reg_name.as_mut_ptr());
        if (*priv_).opp_token < 0 {
            ret = dev_err_probe(cpu_dev, (*priv_).opp_token, c"failed to set regulators\n".as_ptr());
            goto free_cpumask;
        }
    }
    ret = dev_pm_opp_of_get_sharing_cpus(cpu_dev, (*priv_).cpus);
    if ret != 0 {
        if ret != -ENOENT { goto out; }
        if dev_pm_opp_get_sharing_cpus(cpu_dev, (*priv_).cpus) != 0 { fallback = true; }
    }
    ret = dev_pm_opp_of_cpumask_add_table((*priv_).cpus);
    if ret == 0 { (*priv_).have_static_opps = true; }
    else if ret == -EPROBE_DEFER { goto out; }
    ret = dev_pm_opp_get_opp_count(cpu_dev);
    if ret <= 0 {
        dev_err(cpu_dev, c"OPP table can't be empty\n".as_ptr());
        ret = -ENODEV;
        goto out;
    }
    if fallback {
        cpumask_setall((*priv_).cpus);
        ret = dev_pm_opp_set_sharing_cpus(cpu_dev, (*priv_).cpus);
        if ret != 0 { dev_err(cpu_dev, c"%s: failed to mark OPPs as shared: %d\n".as_ptr(), __func__, ret); }
    }
    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &mut (*priv_).freq_table);
    if ret != 0 { dev_err(cpu_dev, c"failed to init cpufreq table: %d\n".as_ptr(), ret); goto out; }
    list_add(&mut (*priv_).node, &mut priv_list);
    return 0;
out:
    if (*priv_).have_static_opps { dev_pm_opp_of_cpumask_remove_table((*priv_).cpus); }
    dev_pm_opp_put_regulators((*priv_).opp_token);
free_cpumask:
    free_cpumask_var((*priv_).cpus);
    ret
}

unsafe fn dt_cpufreq_release() {
    let mut priv_: *mut private_data;
    let mut tmp: *mut private_data;
    list_for_each_entry_safe!(priv_, tmp, &mut priv_list, node) {
        dev_pm_opp_free_cpufreq_table((*priv_).cpu_dev, &mut (*priv_).freq_table);
        if (*priv_).have_static_opps { dev_pm_opp_of_cpumask_remove_table((*priv_).cpus); }
        dev_pm_opp_put_regulators((*priv_).opp_token);
        free_cpumask_var((*priv_).cpus);
        list_del(&mut (*priv_).node);
    }
}

unsafe fn dt_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    let data: *mut cpufreq_dt_platform_data = dev_get_platdata(&mut (*pdev).dev);
    let mut ret: i32 = 0;
    let mut cpu: i32;
    for_each_present_cpu!(cpu) {
        ret = dt_cpufreq_early_init(&mut (*pdev).dev, cpu);
        if ret != 0 { goto err; }
    }
    if !data.is_null() {
        if (*data).have_governor_per_policy { dt_cpufreq_driver.flags |= CPUFREQ_HAVE_GOVERNOR_PER_POLICY; }
        dt_cpufreq_driver.resume = (*data).resume;
        if !(*data).suspend.is_none() { dt_cpufreq_driver.suspend = (*data).suspend; }
        if !(*data).get_intermediate.is_none() {
            dt_cpufreq_driver.target_intermediate = (*data).target_intermediate;
            dt_cpufreq_driver.get_intermediate = (*data).get_intermediate;
        }
    }
    ret = cpufreq_register_driver(&mut dt_cpufreq_driver);
    if ret != 0 { dev_err(&mut (*pdev).dev, c"failed register driver: %d\n".as_ptr(), ret); goto err; }
    return 0;
err:
    dt_cpufreq_release();
    ret
}

unsafe fn dt_cpufreq_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut dt_cpufreq_driver);
    dt_cpufreq_release();
}

static mut dt_cpufreq_platdrv: platform_driver = platform_driver {
    driver: driver { name: c"cpufreq-dt".as_ptr(), ..unsafe { core::mem::zeroed() } },
    probe: Some(dt_cpufreq_probe),
    remove: Some(dt_cpufreq_remove),
    ..unsafe { core::mem::zeroed() }
};

// Equivalent to module_platform_driver(dt_cpufreq_platdrv).
module_platform_driver!(dt_cpufreq_platdrv);

unsafe fn cpufreq_dt_pdev_register(dev: *mut device) -> *mut platform_device {
    let mut cpufreq_dt_devinfo: platform_device_info = core::mem::zeroed();
    cpufreq_dt_devinfo.name = c"cpufreq-dt".as_ptr();
    cpufreq_dt_devinfo.parent = dev;
    platform_device_register_full(&cpufreq_dt_devinfo)
}

// EXPORT_SYMBOL_GPL(cpufreq_dt_pdev_register);
// MODULE_ALIAS("platform:cpufreq-dt");
// MODULE_AUTHOR("Viresh Kumar <viresh.kumar@linaro.org>");
// MODULE_AUTHOR("Shawn Guo <shawn.guo@linaro.org>");
// MODULE_DESCRIPTION("Generic cpufreq driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
