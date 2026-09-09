// SPDX-License-Identifier: GPL-2.0-only
/*
 *  drivers/cpufreq/cpufreq_ondemand.c
 *
 *  Copyright (C)  2001 Russell King
 *            (C)  2003 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
 *                      Jun Nakajima <jun.nakajima@intel.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

const DEF_FREQUENCY_UP_THRESHOLD: u32 = 80;
const DEF_SAMPLING_DOWN_FACTOR: u32 = 1;
const MAX_SAMPLING_DOWN_FACTOR: u32 = 100000;
const MICRO_FREQUENCY_UP_THRESHOLD: u32 = 95;
const MIN_FREQUENCY_UP_THRESHOLD: u32 = 1;
const MAX_FREQUENCY_UP_THRESHOLD: u32 = 100;

#[repr(C)]
struct od_ops { powersave_bias_target: unsafe fn(*mut cpufreq_policy, u32, u32) -> u32 }
static mut OD_OPS: od_ops = od_ops { powersave_bias_target: generic_powersave_bias_target };
static mut default_powersave_bias: u32 = 0;

unsafe fn generic_powersave_bias_target(policy: *mut cpufreq_policy, freq_next: u32, relation: u32) -> u32 {
    let policy_dbs = (*policy).governor_data;
    let dbs_info = to_dbs_info(policy_dbs);
    let dbs_data = (*policy_dbs).dbs_data;
    let od_tuners = (*dbs_data).tuners;
    let freq_table = (*policy).freq_table;
    if freq_table.is_null() {
        (*dbs_info).freq_lo = 0;
        (*dbs_info).freq_lo_delay_us = 0;
        return freq_next;
    }
    let mut index = cpufreq_frequency_table_target(policy, freq_next, (*policy).min, (*policy).max, relation);
    let freq_req = (*freq_table.add(index as usize)).frequency;
    let freq_reduc = freq_req * (*od_tuners).powersave_bias / 1000;
    let freq_avg = freq_req - freq_reduc;
    index = cpufreq_table_find_index_h(policy, freq_avg, relation & CPUFREQ_RELATION_E);
    let freq_lo = (*freq_table.add(index as usize)).frequency;
    index = cpufreq_table_find_index_l(policy, freq_avg, relation & CPUFREQ_RELATION_E);
    let freq_hi = (*freq_table.add(index as usize)).frequency;
    if freq_hi == freq_lo {
        (*dbs_info).freq_lo = 0;
        (*dbs_info).freq_lo_delay_us = 0;
        return freq_lo;
    }
    let mut delay_hi_us = (freq_avg - freq_lo) * (*dbs_data).sampling_rate;
    delay_hi_us += (freq_hi - freq_lo) / 2;
    delay_hi_us /= freq_hi - freq_lo;
    (*dbs_info).freq_hi_delay_us = delay_hi_us;
    (*dbs_info).freq_lo = freq_lo;
    (*dbs_info).freq_lo_delay_us = (*dbs_data).sampling_rate - delay_hi_us;
    freq_hi
}

unsafe fn ondemand_powersave_bias_init(policy: *mut cpufreq_policy) {
    (*to_dbs_info((*policy).governor_data)).freq_lo = 0;
}

unsafe fn dbs_freq_increase(policy: *mut cpufreq_policy, mut freq: u32) {
    let policy_dbs = (*policy).governor_data;
    let dbs_data = (*policy_dbs).dbs_data;
    let od_tuners = (*dbs_data).tuners;
    if (*od_tuners).powersave_bias != 0 {
        freq = (OD_OPS.powersave_bias_target)(policy, freq, CPUFREQ_RELATION_HE);
    } else if (*policy).cur == (*policy).max { return; }
    __cpufreq_driver_target(policy, freq, if (*od_tuners).powersave_bias != 0 { CPUFREQ_RELATION_LE } else { CPUFREQ_RELATION_HE });
}

unsafe fn od_update(policy: *mut cpufreq_policy) {
    let policy_dbs = (*policy).governor_data;
    let dbs_info = to_dbs_info(policy_dbs);
    let dbs_data = (*policy_dbs).dbs_data;
    let od_tuners = (*dbs_data).tuners;
    let load = dbs_update(policy);
    (*dbs_info).freq_lo = 0;
    if load > (*dbs_data).up_threshold {
        if (*policy).cur < (*policy).max { (*policy_dbs).rate_mult = (*dbs_data).sampling_down_factor; }
        dbs_freq_increase(policy, (*policy).max);
    } else {
        let min_f = (*policy).cpuinfo.min_freq;
        let max_f = (*policy).cpuinfo.max_freq;
        let mut freq_next = min_f + load * (max_f - min_f) / 100;
        (*policy_dbs).rate_mult = 1;
        if (*od_tuners).powersave_bias != 0 { freq_next = (OD_OPS.powersave_bias_target)(policy, freq_next, CPUFREQ_RELATION_LE); }
        __cpufreq_driver_target(policy, freq_next, CPUFREQ_RELATION_CE);
    }
}

unsafe fn od_dbs_update(policy: *mut cpufreq_policy) -> u32 {
    let policy_dbs = (*policy).governor_data;
    let dbs_data = (*policy_dbs).dbs_data;
    let dbs_info = to_dbs_info(policy_dbs);
    let sample_type = (*dbs_info).sample_type;
    (*dbs_info).sample_type = OD_NORMAL_SAMPLE;
    if sample_type == OD_SUB_SAMPLE && (*policy_dbs).sample_delay_ns > 0 {
        __cpufreq_driver_target(policy, (*dbs_info).freq_lo, CPUFREQ_RELATION_HE);
        return (*dbs_info).freq_lo_delay_us;
    }
    od_update(policy);
    if (*dbs_info).freq_lo != 0 {
        (*dbs_info).sample_type = OD_SUB_SAMPLE;
        return (*dbs_info).freq_hi_delay_us;
    }
    (*dbs_data).sampling_rate * (*policy_dbs).rate_mult
}

unsafe fn io_is_busy_store(attr_set: *mut gov_attr_set, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set); let mut input = 0u32;
    if sscanf(buf, "%u", &mut input) != 1 { return -EINVAL; }
    (*dbs_data).io_is_busy = (input != 0) as u32; gov_update_cpu_data(dbs_data); count as isize
}

unsafe fn up_threshold_store(attr_set: *mut gov_attr_set, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set); let mut input = 0u32;
    if sscanf(buf, "%u", &mut input) != 1 || input > MAX_FREQUENCY_UP_THRESHOLD || input < MIN_FREQUENCY_UP_THRESHOLD { return -EINVAL; }
    (*dbs_data).up_threshold = input; count as isize
}

unsafe fn sampling_down_factor_store(attr_set: *mut gov_attr_set, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set); let mut input = 0u32;
    if sscanf(buf, "%u", &mut input) != 1 || input > MAX_SAMPLING_DOWN_FACTOR || input < 1 { return -EINVAL; }
    (*dbs_data).sampling_down_factor = input;
    // list_for_each_entry(policy_dbs, &attr_set->policy_list, list)
    let mut policy_dbs = (*attr_set).policy_list_head;
    while !policy_dbs.is_null() {
        mutex_lock(&mut (*policy_dbs).update_mutex); (*policy_dbs).rate_mult = 1; mutex_unlock(&mut (*policy_dbs).update_mutex);
        policy_dbs = (*policy_dbs).next;
    }
    count as isize
}

unsafe fn ignore_nice_load_store(attr_set: *mut gov_attr_set, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set); let mut input = 0u32;
    if sscanf(buf, "%u", &mut input) != 1 { return -EINVAL; }
    if input > 1 { input = 1; }
    if input == (*dbs_data).ignore_nice_load { return count as isize; }
    (*dbs_data).ignore_nice_load = input; gov_update_cpu_data(dbs_data); count as isize
}

unsafe fn powersave_bias_store(attr_set: *mut gov_attr_set, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set); let od_tuners = (*dbs_data).tuners; let mut input = 0u32;
    if sscanf(buf, "%u", &mut input) != 1 { return -EINVAL; }
    if input > 1000 { input = 1000; }
    (*od_tuners).powersave_bias = input;
    let mut policy_dbs = (*attr_set).policy_list_head;
    while !policy_dbs.is_null() { ondemand_powersave_bias_init((*policy_dbs).policy); policy_dbs = (*policy_dbs).next; }
    count as isize
}

// gov_show_one_common(sampling_rate), gov_show_one_common(up_threshold),
// gov_show_one_common(sampling_down_factor), gov_show_one_common(ignore_nice_load),
// gov_show_one_common(io_is_busy), and gov_show_one(od, powersave_bias) generate
// the corresponding sysfs show functions and attributes.
// gov_attr_rw(...) and ATTRIBUTE_GROUPS(od) generate the read/write attributes and group.
static mut od_attrs: [*mut attribute; 7] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
];

unsafe fn od_alloc() -> *mut policy_dbs_info {
    let dbs_info = kzalloc_obj::<od_policy_dbs_info>();
    if dbs_info.is_null() { core::ptr::null_mut() } else { &mut (*dbs_info).policy_dbs }
}
unsafe fn od_free(policy_dbs: *mut policy_dbs_info) { kfree(to_dbs_info(policy_dbs)); }

unsafe fn od_init(dbs_data: *mut dbs_data) -> i32 {
    let tuners = kzalloc_obj::<od_dbs_tuners>(); if tuners.is_null() { return -ENOMEM; }
    (*dbs_data).up_threshold = if tick_nohz_is_active() { MICRO_FREQUENCY_UP_THRESHOLD } else { DEF_FREQUENCY_UP_THRESHOLD };
    (*dbs_data).sampling_down_factor = DEF_SAMPLING_DOWN_FACTOR; (*dbs_data).ignore_nice_load = 0;
    (*tuners).powersave_bias = default_powersave_bias; (*dbs_data).io_is_busy = od_should_io_be_busy(); (*dbs_data).tuners = tuners; 0
}
unsafe fn od_exit(dbs_data: *mut dbs_data) { kfree((*dbs_data).tuners); }
unsafe fn od_start(policy: *mut cpufreq_policy) { let dbs_info = to_dbs_info((*policy).governor_data); (*dbs_info).sample_type = OD_NORMAL_SAMPLE; ondemand_powersave_bias_init(policy); }

// static struct dbs_governor od_dbs_gov = {
//     .gov = CPUFREQ_DBS_GOVERNOR_INITIALIZER("ondemand"),
//     .kobj_type = { .default_groups = od_groups },
//     .gov_dbs_update = od_dbs_update, .alloc = od_alloc, .free = od_free,
//     .init = od_init, .exit = od_exit, .start = od_start,
// };
// #define CPU_FREQ_GOV_ONDEMAND (od_dbs_gov.gov)

unsafe fn od_set_powersave_bias(powersave_bias: u32) {
    let done = match alloc_cpumask_var(GFP_KERNEL) { Some(p) => p, None => return };
    default_powersave_bias = powersave_bias; cpumask_clear(done); cpus_read_lock();
    for_each_online_cpu(|cpu| {
        if cpumask_test_cpu(cpu, done) { return; }
        let policy = cpufreq_cpu_get_raw(cpu); if policy.is_null() || (*policy).governor != &CPU_FREQ_GOV_ONDEMAND { return; }
        let policy_dbs = (*policy).governor_data; if policy_dbs.is_null() { return; }
        cpumask_or(done, done, (*policy).cpus); (*(*policy_dbs).dbs_data).tuners.powersave_bias = default_powersave_bias;
    });
    cpus_read_unlock(); free_cpumask_var(done);
}

pub unsafe fn od_register_powersave_bias_handler(f: unsafe fn(*mut cpufreq_policy, u32, u32) -> u32, powersave_bias: u32) { OD_OPS.powersave_bias_target = f; od_set_powersave_bias(powersave_bias); }
pub unsafe fn od_unregister_powersave_bias_handler() { OD_OPS.powersave_bias_target = generic_powersave_bias_target; od_set_powersave_bias(0); }

// Module metadata retained from MODULE_AUTHOR, MODULE_DESCRIPTION, and MODULE_LICENSE.
// CONFIG_CPU_FREQ_DEFAULT_GOV_ONDEMAND conditionally provides cpufreq_default_governor.
// cpufreq_governor_init(CPU_FREQ_GOV_ONDEMAND); cpufreq_governor_exit(CPU_FREQ_GOV_ONDEMAND);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
