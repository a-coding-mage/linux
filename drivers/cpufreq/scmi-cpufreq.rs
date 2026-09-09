// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) based CPUFreq Interface driver
 *
 * Copyright (C) 2018-2021 ARM Ltd.
 * Sudeep Holla <sudeep.holla@arm.com>
 */

// Translated from C. Kernel-provided types, constants, macros, and functions
// referenced below are supplied by the surrounding kernel bindings.

#[repr(C)]
struct scmi_data {
    domain_id: i32,
    nr_opp: i32,
    cpu_dev: *mut device,
    opp_shared_cpus: cpumask_var_t,
    limit_notify_nb: notifier_block,
    limits_freq_req: freq_qos_request,
}

static mut ph: *mut scmi_protocol_handle = core::ptr::null_mut();
static mut perf_ops: *const scmi_perf_proto_ops = core::ptr::null();
static mut scmi_cpufreq_driver: cpufreq_driver = cpufreq_driver::zeroed();

unsafe fn scmi_cpufreq_get_rate(cpu: u32) -> u32 {
    let policy: *mut cpufreq_policy;
    let priv_: *mut scmi_data;
    let mut rate: u64 = 0;
    let ret: i32;

    policy = cpufreq_cpu_get_raw(cpu);
    if policy.is_null() {
        return 0;
    }

    priv_ = (*policy).driver_data as *mut scmi_data;
    ret = ((*perf_ops).freq_get)(ph, (*priv_).domain_id, &mut rate, false);
    if ret != 0 {
        return 0;
    }
    (rate / 1000) as u32
}

/*
 * perf_ops->freq_set is not a synchronous, the actual OPP change will
 * happen asynchronously and can get notified if the events are
 * subscribed for by the SCMI firmware
 */
unsafe fn scmi_cpufreq_set_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let priv_ = (*policy).driver_data as *mut scmi_data;
    let freq: u64 = (*(*policy).freq_table.add(index as usize)).frequency as u64;
    ((*perf_ops).freq_set)(ph, (*priv_).domain_id, freq * 1000, false)
}

unsafe fn scmi_cpufreq_fast_switch(policy: *mut cpufreq_policy, target_freq: u32) -> u32 {
    let priv_ = (*policy).driver_data as *mut scmi_data;
    let freq = target_freq as u64;
    if ((*perf_ops).freq_set)(ph, (*priv_).domain_id, freq * 1000, true) == 0 {
        target_freq
    } else {
        0
    }
}

unsafe fn scmi_cpu_domain_id(cpu_dev: *mut device) -> i32 {
    let np = (*cpu_dev).of_node;
    let mut domain_id: of_phandle_args = core::mem::zeroed();
    let index: i32;

    if of_parse_phandle_with_args(np, c"clocks".as_ptr(), c"#clock-cells".as_ptr(), 0, &mut domain_id) != 0 {
        index = of_property_match_string(np, c"power-domain-names".as_ptr(), c"perf".as_ptr());
        if index < 0 {
            return -EINVAL;
        }
        if of_parse_phandle_with_args(np, c"power-domains".as_ptr(), c"#power-domain-cells".as_ptr(), index as u32, &mut domain_id) != 0 {
            return -EINVAL;
        }
    }
    of_node_put(domain_id.np);
    domain_id.args[0]
}

unsafe fn scmi_get_sharing_cpus(cpu_dev: *mut device, domain: i32, cpumask: *mut cpumask) -> i32 {
    for_each_present_cpu!(cpu, {
        if cpu == (*cpu_dev).id { continue; }
        let tcpu_dev = get_cpu_device(cpu);
        if tcpu_dev.is_null() { continue; }
        let tdomain = scmi_cpu_domain_id(tcpu_dev);
        if tdomain == domain { cpumask_set_cpu(cpu, cpumask); }
    });
    0
}

unsafe fn scmi_get_cpu_power(cpu_dev: *mut device, power: *mut u64, khz: *mut u64) -> i32 {
    let power_scale = ((*perf_ops).power_scale_get)(ph);
    let mut hz: u64;
    let domain = scmi_cpu_domain_id(cpu_dev);
    if domain < 0 { return domain; }
    hz = *khz * 1000;
    let ret = ((*perf_ops).est_power_get)(ph, domain, &mut hz, power);
    if ret != 0 { return ret; }
    if power_scale == SCMI_POWER_MILLIWATTS { *power *= MICROWATT_PER_MILLIWATT; }
    *khz = hz / 1000;
    0
}

unsafe fn scmi_get_rate_limit(domain: u32, has_fast_switch: bool) -> i32 {
    let mut rate_limit = 0;
    if has_fast_switch {
        let ret = ((*perf_ops).fast_switch_rate_limit)(ph, domain, &mut rate_limit);
        if ret == 0 && rate_limit != 0 { return rate_limit; }
    }
    let ret = ((*perf_ops).rate_limit_get)(ph, domain, &mut rate_limit);
    if ret != 0 { return 0; }
    rate_limit
}

unsafe fn scmi_limit_notify_cb(nb: *mut notifier_block, _event: u64, data: *mut core::ffi::c_void) -> i32 {
    let priv_ = container_of!(nb, scmi_data, limit_notify_nb);
    let limit_notify = data as *mut scmi_perf_limits_report;
    let limit_freq_khz = (*limit_notify).range_max_freq / HZ_PER_KHZ;
    let ret = freq_qos_update_request(&mut (*priv_).limits_freq_req, limit_freq_khz);
    if ret < 0 { pr_warn!("failed to update freq constraint: {}\n", ret); }
    NOTIFY_OK
}

unsafe fn scmi_cpufreq_init(policy: *mut cpufreq_policy) -> i32 {
    let cpu_dev = get_cpu_device((*policy).cpu);
    if cpu_dev.is_null() { pr_err!("failed to get cpu{} device\n", (*policy).cpu); return -ENODEV; }
    let domain = scmi_cpu_domain_id(cpu_dev);
    if domain < 0 { return domain; }
    let priv_ = kzalloc_obj::<scmi_data>();
    if priv_.is_null() { return -ENOMEM; }
    if !zalloc_cpumask_var(&mut (*priv_).opp_shared_cpus, GFP_KERNEL) {
        kfree(priv_ as *mut core::ffi::c_void); return -ENOMEM;
    }
    let mut ret = scmi_get_sharing_cpus(cpu_dev, domain, (*policy).cpus);
    if ret != 0 { dev_warn!(cpu_dev, "failed to get sharing cpumask\n"); goto!(out_free_cpumask); }
    ret = dev_pm_opp_of_get_sharing_cpus(cpu_dev, (*priv_).opp_shared_cpus);
    if ret != 0 || cpumask_empty((*priv_).opp_shared_cpus) { cpumask_copy((*priv_).opp_shared_cpus, (*policy).cpus); }
    let mut nr_opp = dev_pm_opp_get_opp_count(cpu_dev);
    if nr_opp <= 0 {
        ret = ((*perf_ops).device_opps_add)(ph, cpu_dev, domain);
        if ret != 0 { dev_warn!(cpu_dev, "failed to add opps to the device\n"); goto!(out_free_cpumask); }
        nr_opp = dev_pm_opp_get_opp_count(cpu_dev);
        if nr_opp <= 0 { dev_err!(cpu_dev, "{}: No OPPs for this device: {}\n", __func__, nr_opp); ret = -ENODEV; goto!(out_free_opp); }
        ret = dev_pm_opp_set_sharing_cpus(cpu_dev, (*priv_).opp_shared_cpus);
        if ret != 0 { dev_err!(cpu_dev, "{}: failed to mark OPPs as shared: {}\n", __func__, ret); goto!(out_free_opp); }
        (*priv_).nr_opp = nr_opp;
    }
    let mut freq_table: *mut cpufreq_frequency_table = core::ptr::null_mut();
    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &mut freq_table);
    if ret != 0 { dev_err!(cpu_dev, "failed to init cpufreq table: {}\n", ret); goto!(out_free_opp); }
    (*priv_).cpu_dev = cpu_dev; (*priv_).domain_id = domain;
    (*policy).driver_data = priv_ as *mut core::ffi::c_void; (*policy).freq_table = freq_table;
    (*policy).dvfs_possible_from_any_cpu = true;
    let mut latency = ((*perf_ops).transition_latency_get)(ph, domain);
    if latency == 0 { latency = CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS; }
    (*policy).cpuinfo.transition_latency = latency;
    (*policy).fast_switch_possible = ((*perf_ops).fast_switch_possible)(ph, domain);
    (*policy).transition_delay_us = scmi_get_rate_limit(domain as u32, (*policy).fast_switch_possible);
    ret = freq_qos_add_request(&mut (*policy).constraints, &mut (*priv_).limits_freq_req, FREQ_QOS_MAX, FREQ_QOS_MAX_DEFAULT_VALUE);
    if ret < 0 { dev_err!(cpu_dev, "failed to add qos limits request: {}\n", ret); goto!(out_free_table); }
    (*priv_).limit_notify_nb.notifier_call = Some(scmi_limit_notify_cb);
    let sdev = cpufreq_get_driver_data() as *mut scmi_device;
    ret = ((*(*sdev).handle).notify_ops.event_notifier_register)((*sdev).handle, SCMI_PROTOCOL_PERF, SCMI_EVENT_PERFORMANCE_LIMITS_CHANGED, &mut (*priv_).domain_id, &mut (*priv_).limit_notify_nb);
    if ret != 0 { dev_warn!(&mut (*sdev).dev, "failed to register for limits change notifier for domain {}\n", (*priv_).domain_id); }
    return 0;
    out_free_table: dev_pm_opp_free_cpufreq_table(cpu_dev, &mut freq_table);
    out_free_opp: dev_pm_opp_remove_all_dynamic(cpu_dev);
    out_free_cpumask: free_cpumask_var((*priv_).opp_shared_cpus);
    kfree(priv_ as *mut core::ffi::c_void); ret
}

unsafe fn scmi_cpufreq_exit(policy: *mut cpufreq_policy) {
    let priv_ = (*policy).driver_data as *mut scmi_data;
    let sdev = cpufreq_get_driver_data() as *mut scmi_device;
    ((*(*sdev).handle).notify_ops.event_notifier_unregister)((*sdev).handle, SCMI_PROTOCOL_PERF, SCMI_EVENT_PERFORMANCE_LIMITS_CHANGED, &mut (*priv_).domain_id, &mut (*priv_).limit_notify_nb);
    freq_qos_remove_request(&mut (*priv_).limits_freq_req);
    dev_pm_opp_free_cpufreq_table((*priv_).cpu_dev, &mut (*policy).freq_table);
    dev_pm_opp_remove_all_dynamic((*priv_).cpu_dev);
    free_cpumask_var((*priv_).opp_shared_cpus); kfree(priv_ as *mut core::ffi::c_void);
}

unsafe fn scmi_cpufreq_register_em(policy: *mut cpufreq_policy) {
    let priv_ = (*policy).driver_data as *mut scmi_data;
    if (*priv_).nr_opp == 0 { return; }
    let power_scale = ((*perf_ops).power_scale_get)(ph);
    let em_power_scale = power_scale == SCMI_POWER_MILLIWATTS || power_scale == SCMI_POWER_MICROWATTS;
    let mut em_cb = EM_DATA_CB!(scmi_get_cpu_power);
    em_dev_register_perf_domain(get_cpu_device((*policy).cpu), (*priv_).nr_opp, &mut em_cb, (*priv_).opp_shared_cpus, em_power_scale);
}

static mut scmi_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: "scmi", flags: CPUFREQ_HAVE_GOVERNOR_PER_POLICY | CPUFREQ_NEED_INITIAL_FREQ_CHECK | CPUFREQ_IS_COOLING_DEV,
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(scmi_cpufreq_set_target),
    fast_switch: Some(scmi_cpufreq_fast_switch), get: Some(scmi_cpufreq_get_rate), init: Some(scmi_cpufreq_init),
    exit: Some(scmi_cpufreq_exit), register_em: Some(scmi_cpufreq_register_em), set_boost: Some(cpufreq_boost_set_sw),
};

unsafe fn scmi_dev_used_by_cpus(scmi_dev: *mut device) -> bool {
    let scmi_np = dev_of_node(scmi_dev); if scmi_np.is_null() { return false; }
    for_each_possible_cpu!(cpu, {
        let cpu_dev = get_cpu_device(cpu); if cpu_dev.is_null() { continue; }
        let cpu_np = dev_of_node(cpu_dev); let np = of_parse_phandle(cpu_np, c"clocks".as_ptr(), 0); of_node_put(np);
        if np == scmi_np { return true; }
        let idx = of_property_match_string(cpu_np, c"power-domain-names".as_ptr(), c"perf".as_ptr());
        let np = of_parse_phandle(cpu_np, c"power-domains".as_ptr(), idx); of_node_put(np);
        if np == scmi_np { return true; }
    });
    if of_machine_is_compatible(c"brcm,brcmstb".as_ptr()) { return true; }
    false
}

unsafe fn scmi_cpufreq_probe(sdev: *mut scmi_device) -> i32 {
    let dev = &mut (*sdev).dev; let handle = (*sdev).handle;
    if handle.is_null() || !scmi_dev_used_by_cpus(dev) { return -ENODEV; }
    scmi_cpufreq_driver.driver_data = sdev as *mut core::ffi::c_void;
    perf_ops = ((*handle).devm_protocol_get)(sdev, SCMI_PROTOCOL_PERF, &mut ph);
    if IS_ERR(perf_ops) { return PTR_ERR(perf_ops); }
    // CONFIG_COMMON_CLK: add the dummy clock provider when the clocks property is used.
    let ret = cpufreq_register_driver(&mut scmi_cpufreq_driver);
    if ret != 0 { dev_err!(dev, "{}: registering cpufreq failed, err: {}\n", __func__, ret); }
    ret
}

unsafe fn scmi_cpufreq_remove(_sdev: *mut scmi_device) { cpufreq_unregister_driver(&mut scmi_cpufreq_driver); }

static scmi_id_table: [scmi_device_id; 2] = [
    scmi_device_id { protocol_id: SCMI_PROTOCOL_PERF, name: "cpufreq" },
    scmi_device_id::default(),
];

static mut scmi_cpufreq_drv: scmi_driver = scmi_driver {
    name: "scmi-cpufreq", probe: Some(scmi_cpufreq_probe), remove: Some(scmi_cpufreq_remove), id_table: scmi_id_table.as_ptr(),
};

// MODULE_DEVICE_TABLE(scmi, scmi_id_table);
// module_scmi_driver(scmi_cpufreq_drv);
// MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
// MODULE_DESCRIPTION("ARM SCMI CPUFreq interface driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
