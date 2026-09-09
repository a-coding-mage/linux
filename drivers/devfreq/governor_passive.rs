// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/devfreq/governor_passive.c
 *
 * Copyright (C) 2016 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 * Author: MyungJoo Ham <myungjoo.ham@samsung.com>
 */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */

/// struct devfreq_cpu_data - Hold the per-cpu data
/// @node: list node
/// @dev: reference to cpu device.
/// @first_cpu: the cpumask of the first cpu of a policy.
/// @opp_table: reference to cpu opp table.
/// @cur_freq: the current frequency of the cpu.
/// @min_freq: the min frequency of the cpu.
/// @max_freq: the max frequency of the cpu.
///
/// This structure stores the required cpu_data of a cpu.
/// This is auto-populated by the governor.
#[repr(C)]
struct devfreq_cpu_data {
    node: list_head,
    dev: *mut device,
    first_cpu: c_uint,
    opp_table: *mut opp_table,
    cur_freq: c_uint,
    min_freq: c_uint,
    max_freq: c_uint,
}

unsafe fn get_parent_cpu_data(
    p_data: *mut devfreq_passive_data,
    policy: *mut cpufreq_policy,
) -> *mut devfreq_cpu_data {
    let mut parent_cpu_data: *mut devfreq_cpu_data;

    if p_data.is_null() || policy.is_null() {
        return core::ptr::null_mut();
    }

    list_for_each_entry!(parent_cpu_data, (*p_data).cpu_data_list, node) {
        if (*parent_cpu_data).first_cpu == cpumask_first((*policy).related_cpus) {
            return parent_cpu_data;
        }
    }

    core::ptr::null_mut()
}

unsafe fn delete_parent_cpu_data(p_data: *mut devfreq_passive_data) {
    let mut parent_cpu_data: *mut devfreq_cpu_data;
    let mut tmp: *mut devfreq_cpu_data;

    list_for_each_entry_safe!(parent_cpu_data, tmp, (*p_data).cpu_data_list, node) {
        list_del!(&mut (*parent_cpu_data).node);

        if !(*parent_cpu_data).opp_table.is_null() {
            dev_pm_opp_put_opp_table((*parent_cpu_data).opp_table);
        }

        kfree(parent_cpu_data);
    }
}

unsafe fn get_target_freq_by_required_opp(
    p_dev: *mut device,
    p_opp_table: *mut opp_table,
    opp_table: *mut opp_table,
    freq: *mut c_ulong,
) -> c_ulong {
    let mut opp: *mut dev_pm_opp = core::ptr::null_mut();
    let mut p_opp: *mut dev_pm_opp = core::ptr::null_mut();
    let target_freq: c_ulong;

    if p_dev.is_null() || p_opp_table.is_null() || opp_table.is_null() || freq.is_null() {
        return 0;
    }

    p_opp = devfreq_recommended_opp(p_dev, freq, 0);
    if IS_ERR!(p_opp) {
        return 0;
    }

    opp = dev_pm_opp_xlate_required_opp(p_opp_table, opp_table, p_opp);
    dev_pm_opp_put(p_opp);

    if IS_ERR!(opp) {
        return 0;
    }

    target_freq = dev_pm_opp_get_freq(opp);
    dev_pm_opp_put(opp);

    target_freq
}

unsafe fn get_target_freq_with_cpufreq(
    devfreq: *mut devfreq,
    target_freq: *mut c_ulong,
) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let mut parent_cpu_data: *mut devfreq_cpu_data;
    let mut policy: *mut cpufreq_policy;
    let (mut cpu, mut cpu_cur, mut cpu_min, mut cpu_max, mut cpu_percent): (c_ulong, c_ulong, c_ulong, c_ulong, c_ulong);
    let (mut dev_min, mut dev_max): (c_ulong, c_ulong);
    let mut freq: c_ulong = 0;
    let mut ret: c_int = 0;

    for_each_online_cpu!(cpu) {
        policy = cpufreq_cpu_get(cpu);
        if policy.is_null() {
            ret = -EINVAL;
            continue;
        }

        parent_cpu_data = get_parent_cpu_data(p_data, policy);
        if parent_cpu_data.is_null() {
            cpufreq_cpu_put(policy);
            continue;
        }

        /* Get target freq via required opps */
        cpu_cur = (*parent_cpu_data).cur_freq as c_ulong * HZ_PER_KHZ;
        freq = get_target_freq_by_required_opp((*parent_cpu_data).dev,
            (*parent_cpu_data).opp_table, (*devfreq).opp_table, &mut cpu_cur);
        if freq != 0 {
            *target_freq = core::cmp::max(freq, *target_freq);
            cpufreq_cpu_put(policy);
            continue;
        }

        /* Use interpolation if required opps is not available */
        devfreq_get_freq_range(devfreq, &mut dev_min, &mut dev_max);

        cpu_min = (*parent_cpu_data).min_freq as c_ulong;
        cpu_max = (*parent_cpu_data).max_freq as c_ulong;
        cpu_cur = (*parent_cpu_data).cur_freq as c_ulong;

        cpu_percent = ((cpu_cur - cpu_min) * 100) / (cpu_max - cpu_min);
        freq = dev_min + mult_frac!(dev_max - dev_min, cpu_percent, 100);

        *target_freq = core::cmp::max(freq, *target_freq);
        cpufreq_cpu_put(policy);
    }

    ret
}

unsafe fn get_target_freq_with_devfreq(devfreq: *mut devfreq, freq: *mut c_ulong) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let parent_devfreq = (*p_data).parent as *mut devfreq;
    let mut child_freq: c_ulong = ULONG_MAX;
    let mut i: c_int = 0;
    let mut count: c_int;

    /* Get target freq via required opps */
    child_freq = get_target_freq_by_required_opp((*parent_devfreq).dev.parent,
        (*parent_devfreq).opp_table, (*devfreq).opp_table, freq);
    if child_freq != 0 {
        *freq = child_freq;
        return 0;
    }

    /* Use interpolation if required opps is not available */
    while i < (*parent_devfreq).max_state {
        if (*parent_devfreq).freq_table[i as usize] == *freq { break; }
        i += 1;
    }

    if i == (*parent_devfreq).max_state { return -EINVAL; }

    if i < (*devfreq).max_state {
        child_freq = (*devfreq).freq_table[i as usize];
    } else {
        count = (*devfreq).max_state;
        child_freq = (*devfreq).freq_table[(count - 1) as usize];
    }

    *freq = child_freq;
    0
}

unsafe fn devfreq_passive_get_target_freq(devfreq: *mut devfreq, freq: *mut c_ulong) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let ret: c_int;

    if p_data.is_null() { return -EINVAL; }

    /*
     * If the devfreq device with passive governor has the specific method
     * to determine the next frequency, should use the get_target_freq()
     * of struct devfreq_passive_data.
     */
    if let Some(get_target_freq) = (*p_data).get_target_freq {
        return get_target_freq(devfreq, freq);
    }

    match (*p_data).parent_type {
        DEVFREQ_PARENT_DEV => ret = get_target_freq_with_devfreq(devfreq, freq),
        CPUFREQ_PARENT_DEV => ret = get_target_freq_with_cpufreq(devfreq, freq),
        _ => {
            ret = -EINVAL;
            dev_err!(&(*devfreq).dev, "Invalid parent type\n");
        }
    }

    ret
}

unsafe fn cpufreq_passive_notifier_call(nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let p_data = container_of!(nb, devfreq_passive_data, nb);
    let devfreq = (*p_data).this as *mut devfreq;
    let freqs = ptr as *mut cpufreq_freqs;
    let parent_cpu_data: *mut devfreq_cpu_data;
    let cur_freq: c_uint;
    let ret: c_int;

    if event != CPUFREQ_POSTCHANGE || freqs.is_null() { return 0; }

    parent_cpu_data = get_parent_cpu_data(p_data, (*freqs).policy);
    if parent_cpu_data.is_null() || (*parent_cpu_data).cur_freq == (*freqs).new { return 0; }

    cur_freq = (*parent_cpu_data).cur_freq;
    (*parent_cpu_data).cur_freq = (*freqs).new;
    mutex_lock!(&mut (*devfreq).lock);
    ret = devfreq_update_target(devfreq, (*freqs).new as c_ulong);
    mutex_unlock!(&mut (*devfreq).lock);
    if ret != 0 {
        (*parent_cpu_data).cur_freq = cur_freq;
        dev_err!(&(*devfreq).dev, "failed to update the frequency.\n");
        return ret;
    }
    0
}

unsafe fn cpufreq_passive_unregister_notifier(devfreq: *mut devfreq) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let ret: c_int;
    if (*p_data).nb.notifier_call.is_some() {
        ret = cpufreq_unregister_notifier(&mut (*p_data).nb, CPUFREQ_TRANSITION_NOTIFIER);
        if ret < 0 { return ret; }
    }
    delete_parent_cpu_data(p_data);
    0
}

unsafe fn cpufreq_passive_register_notifier(devfreq: *mut devfreq) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let dev = (*devfreq).dev.parent;
    let mut parent_cpu_data: *mut devfreq_cpu_data;
    let mut policy: *mut cpufreq_policy;
    let mut cpu_dev: *mut device;
    let mut opp_table: *mut opp_table = core::ptr::null_mut();
    let mut cpu: c_uint;
    let mut ret: c_int;

    (*p_data).cpu_data_list = LIST_HEAD_INIT!((*p_data).cpu_data_list);
    (*p_data).nb.notifier_call = Some(cpufreq_passive_notifier_call);
    ret = cpufreq_register_notifier(&mut (*p_data).nb, CPUFREQ_TRANSITION_NOTIFIER);
    if ret != 0 {
        dev_err!(dev, "failed to register cpufreq notifier\n");
        (*p_data).nb.notifier_call = None;
        return ret;
    }

    for_each_possible_cpu!(cpu) {
        policy = cpufreq_cpu_get(cpu);
        if policy.is_null() { ret = -EPROBE_DEFER; goto!('err); }
        parent_cpu_data = get_parent_cpu_data(p_data, policy);
        if !parent_cpu_data.is_null() { cpufreq_cpu_put(policy); continue; }
        parent_cpu_data = kzalloc_obj!(*parent_cpu_data);
        if parent_cpu_data.is_null() { ret = -ENOMEM; goto!('err_put_policy); }
        cpu_dev = get_cpu_device(cpu);
        if cpu_dev.is_null() { dev_err!(dev, "failed to get cpu device\n"); ret = -ENODEV; goto!('err_free_cpu_data); }
        opp_table = dev_pm_opp_get_opp_table(cpu_dev);
        if IS_ERR!(opp_table) { dev_err!(dev, "failed to get opp_table of cpu%d\n", cpu); ret = PTR_ERR!(opp_table); goto!('err_free_cpu_data); }
        (*parent_cpu_data).dev = cpu_dev;
        (*parent_cpu_data).opp_table = opp_table;
        (*parent_cpu_data).first_cpu = cpumask_first((*policy).related_cpus);
        (*parent_cpu_data).cur_freq = (*policy).cur;
        (*parent_cpu_data).min_freq = (*policy).cpuinfo.min_freq;
        (*parent_cpu_data).max_freq = (*policy).cpuinfo.max_freq;
        list_add_tail!(&mut (*parent_cpu_data).node, &mut (*p_data).cpu_data_list);
        cpufreq_cpu_put(policy);
    }
    mutex_lock!(&mut (*devfreq).lock);
    ret = devfreq_update_target(devfreq, 0);
    mutex_unlock!(&mut (*devfreq).lock);
    if ret != 0 { dev_err!(dev, "failed to update the frequency\n"); }
    return ret;

    label!('err_free_cpu_data, kfree(parent_cpu_data));
    label!('err_put_policy, cpufreq_cpu_put(policy));
    label!('err, ret);
}

unsafe fn devfreq_passive_notifier_call(nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let data = container_of!(nb, devfreq_passive_data, nb);
    let devfreq = (*data).this as *mut devfreq;
    let parent = (*data).parent as *mut devfreq;
    let freqs = ptr as *mut devfreq_freqs;
    let freq = (*freqs).new;
    let mut ret = 0;
    mutex_lock_nested!(&mut (*devfreq).lock, SINGLE_DEPTH_NESTING);
    match event {
        DEVFREQ_PRECHANGE if (*parent).previous_freq > freq => ret = devfreq_update_target(devfreq, freq),
        DEVFREQ_POSTCHANGE if (*parent).previous_freq < freq => ret = devfreq_update_target(devfreq, freq),
        _ => {}
    }
    mutex_unlock!(&mut (*devfreq).lock);
    if ret < 0 { dev_warn!(&(*devfreq).dev, "failed to update devfreq using passive governor\n"); }
    NOTIFY_DONE
}

unsafe fn devfreq_passive_unregister_notifier(devfreq: *mut devfreq) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    devfreq_unregister_notifier((*p_data).parent as *mut devfreq, &mut (*p_data).nb, DEVFREQ_TRANSITION_NOTIFIER)
}

unsafe fn devfreq_passive_register_notifier(devfreq: *mut devfreq) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    let parent = (*p_data).parent as *mut devfreq;
    if parent.is_null() { return -EPROBE_DEFER; }
    (*p_data).nb.notifier_call = Some(devfreq_passive_notifier_call);
    devfreq_register_notifier(parent, &mut (*p_data).nb, DEVFREQ_TRANSITION_NOTIFIER)
}

unsafe fn devfreq_passive_event_handler(devfreq: *mut devfreq, event: c_uint, _data: *mut c_void) -> c_int {
    let p_data = (*devfreq).data as *mut devfreq_passive_data;
    if p_data.is_null() { return -EINVAL; }
    (*p_data).this = devfreq;
    match event {
        DEVFREQ_GOV_START if (*p_data).parent_type == DEVFREQ_PARENT_DEV => devfreq_passive_register_notifier(devfreq),
        DEVFREQ_GOV_START if (*p_data).parent_type == CPUFREQ_PARENT_DEV => cpufreq_passive_register_notifier(devfreq),
        DEVFREQ_GOV_STOP if (*p_data).parent_type == DEVFREQ_PARENT_DEV => { WARN_ON!(devfreq_passive_unregister_notifier(devfreq)); 0 },
        DEVFREQ_GOV_STOP if (*p_data).parent_type == CPUFREQ_PARENT_DEV => { WARN_ON!(cpufreq_passive_unregister_notifier(devfreq)); 0 },
        _ => 0,
    }
}

static mut devfreq_passive: devfreq_governor = devfreq_governor {
    name: DEVFREQ_GOV_PASSIVE,
    flags: DEVFREQ_GOV_FLAG_IMMUTABLE,
    get_target_freq: Some(devfreq_passive_get_target_freq),
    event_handler: Some(devfreq_passive_event_handler),
};

unsafe fn devfreq_passive_init() -> c_int {
    devfreq_add_governor(&mut devfreq_passive)
}

unsafe fn devfreq_passive_exit() {
    let ret = devfreq_remove_governor(&mut devfreq_passive);
    if ret != 0 {
        pr_err!("%s: failed remove governor %d\n", __func__, ret);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
