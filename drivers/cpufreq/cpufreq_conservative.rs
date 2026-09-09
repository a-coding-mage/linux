// SPDX-License-Identifier: GPL-2.0-only
/*
 *  drivers/cpufreq/cpufreq_conservative.c
 *
 *  Copyright (C)  2001 Russell King
 *            (C)  2003 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
 *                      Jun Nakajima <jun.nakajima@intel.com>
 *            (C)  2009 Alexander Clouter <alex@digriz.org.uk>
 */

#[repr(C)]
struct CsPolicyDbsInfo {
    policy_dbs: PolicyDbsInfo,
    down_skip: u32,
    requested_freq: u32,
}

unsafe fn to_dbs_info(policy_dbs: *mut PolicyDbsInfo) -> *mut CsPolicyDbsInfo {
    container_of!(policy_dbs, CsPolicyDbsInfo, policy_dbs)
}

#[repr(C)]
struct CsDbsTuners {
    down_threshold: u32,
    freq_step: u32,
}

const DEF_FREQUENCY_UP_THRESHOLD: u32 = 80;
const DEF_FREQUENCY_DOWN_THRESHOLD: u32 = 20;
const DEF_FREQUENCY_STEP: u32 = 5;
const DEF_SAMPLING_DOWN_FACTOR: u32 = 1;
const MAX_SAMPLING_DOWN_FACTOR: u32 = 10;

unsafe fn get_freq_step(cs_tuners: *mut CsDbsTuners, policy: *mut CpufreqPolicy) -> u32 {
    let mut freq_step = ((*cs_tuners).freq_step * (*policy).max) / 100;
    if unlikely!(freq_step == 0) {
        freq_step = DEF_FREQUENCY_STEP;
    }
    freq_step
}

unsafe fn cs_dbs_update(policy: *mut CpufreqPolicy) -> u32 {
    let policy_dbs = (*policy).governor_data as *mut PolicyDbsInfo;
    let dbs_info = to_dbs_info(policy_dbs);
    let mut requested_freq = (*dbs_info).requested_freq;
    let dbs_data = (*policy_dbs).dbs_data;
    let cs_tuners = (*dbs_data).tuners as *mut CsDbsTuners;
    let load = dbs_update(policy);
    let freq_step: u32;

    if (*cs_tuners).freq_step == 0 {
        return (*dbs_data).sampling_rate;
    }

    if requested_freq > (*policy).max || requested_freq < (*policy).min {
        requested_freq = (*policy).cur;
        (*dbs_info).requested_freq = requested_freq;
    }

    freq_step = get_freq_step(cs_tuners, policy);

    if (*policy_dbs).idle_periods < UINT_MAX {
        let freq_steps = (*policy_dbs).idle_periods * freq_step;
        if requested_freq > (*policy).min + freq_steps {
            requested_freq -= freq_steps;
        } else {
            requested_freq = (*policy).min;
        }
        (*policy_dbs).idle_periods = UINT_MAX;
    }

    if load > (*dbs_data).up_threshold {
        (*dbs_info).down_skip = 0;
        requested_freq += freq_step;
        if requested_freq > (*policy).max {
            requested_freq = (*policy).max;
        }
        __cpufreq_driver_target(policy, requested_freq, CPUFREQ_RELATION_HE);
        (*dbs_info).requested_freq = requested_freq;
        return (*dbs_data).sampling_rate;
    }

    (*dbs_info).down_skip += 1;
    if (*dbs_info).down_skip < (*dbs_data).sampling_down_factor {
        return (*dbs_data).sampling_rate;
    }
    (*dbs_info).down_skip = 0;

    if load < (*cs_tuners).down_threshold {
        if requested_freq > (*policy).min + freq_step {
            requested_freq -= freq_step;
        } else {
            requested_freq = (*policy).min;
        }
        __cpufreq_driver_target(policy, requested_freq, CPUFREQ_RELATION_LE);
        (*dbs_info).requested_freq = requested_freq;
    }

    (*dbs_data).sampling_rate
}

unsafe fn sampling_down_factor_store(attr_set: *mut GovAttrSet, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let mut input: u32 = 0;
    let ret = kstrtouint(buf, 0, &mut input);
    if ret != 0 || input > MAX_SAMPLING_DOWN_FACTOR || input < 1 { return -EINVAL; }
    (*dbs_data).sampling_down_factor = input;
    count as isize
}

unsafe fn up_threshold_store(attr_set: *mut GovAttrSet, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let cs_tuners = (*dbs_data).tuners as *mut CsDbsTuners;
    let mut input: u32 = 0;
    let ret = kstrtouint(buf, 0, &mut input);
    if ret != 0 || input > 100 || input <= (*cs_tuners).down_threshold { return -EINVAL; }
    (*dbs_data).up_threshold = input;
    count as isize
}

unsafe fn down_threshold_store(attr_set: *mut GovAttrSet, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let cs_tuners = (*dbs_data).tuners as *mut CsDbsTuners;
    let mut input: u32 = 0;
    let ret = kstrtouint(buf, 0, &mut input);
    // cannot be lower than 1 otherwise freq will not fall
    if ret != 0 || input < 1 || input >= (*dbs_data).up_threshold { return -EINVAL; }
    (*cs_tuners).down_threshold = input;
    count as isize
}

unsafe fn ignore_nice_load_store(attr_set: *mut GovAttrSet, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let mut input: u32 = 0;
    let ret = kstrtouint(buf, 0, &mut input);
    if ret != 0 { return ret as isize; }
    if input > 1 { input = 1; }
    if input == (*dbs_data).ignore_nice_load { return count as isize; }
    (*dbs_data).ignore_nice_load = input;
    gov_update_cpu_data(dbs_data);
    count as isize
}

unsafe fn freq_step_store(attr_set: *mut GovAttrSet, buf: *const i8, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let cs_tuners = (*dbs_data).tuners as *mut CsDbsTuners;
    let mut input: u32 = 0;
    let ret = kstrtouint(buf, 0, &mut input);
    if ret != 0 { return ret as isize; }
    if input > 100 { input = 100; }
    (*cs_tuners).freq_step = input;
    count as isize
}

gov_show_one_common!(sampling_rate);
gov_show_one_common!(sampling_down_factor);
gov_show_one_common!(up_threshold);
gov_show_one_common!(ignore_nice_load);
gov_show_one!(cs, down_threshold);
gov_show_one!(cs, freq_step);

gov_attr_rw!(sampling_rate);
gov_attr_rw!(sampling_down_factor);
gov_attr_rw!(up_threshold);
gov_attr_rw!(ignore_nice_load);
gov_attr_rw!(down_threshold);
gov_attr_rw!(freq_step);

static mut CS_ATTRS: [*mut Attribute; 7] = [
    &mut sampling_rate.attr, &mut sampling_down_factor.attr, &mut up_threshold.attr,
    &mut down_threshold.attr, &mut ignore_nice_load.attr, &mut freq_step.attr, core::ptr::null_mut(),
];
attribute_groups!(cs);

unsafe fn cs_alloc() -> *mut PolicyDbsInfo {
    let dbs_info = kzalloc_obj!(CsPolicyDbsInfo);
    if !dbs_info.is_null() { &mut (*dbs_info).policy_dbs } else { core::ptr::null_mut() }
}

unsafe fn cs_free(policy_dbs: *mut PolicyDbsInfo) { kfree(to_dbs_info(policy_dbs)); }

unsafe fn cs_init(dbs_data: *mut DbsData) -> i32 {
    let tuners = kzalloc_obj!(CsDbsTuners);
    if tuners.is_null() { return -ENOMEM; }
    (*tuners).down_threshold = DEF_FREQUENCY_DOWN_THRESHOLD;
    (*tuners).freq_step = DEF_FREQUENCY_STEP;
    (*dbs_data).up_threshold = DEF_FREQUENCY_UP_THRESHOLD;
    (*dbs_data).sampling_down_factor = DEF_SAMPLING_DOWN_FACTOR;
    (*dbs_data).ignore_nice_load = 0;
    (*dbs_data).tuners = tuners as *mut core::ffi::c_void;
    0
}

unsafe fn cs_exit(dbs_data: *mut DbsData) { kfree((*dbs_data).tuners); }

unsafe fn cs_start(policy: *mut CpufreqPolicy) {
    let dbs_info = to_dbs_info((*policy).governor_data as *mut PolicyDbsInfo);
    (*dbs_info).down_skip = 0;
    (*dbs_info).requested_freq = (*policy).cur;
}

unsafe fn cs_limits(policy: *mut CpufreqPolicy) {
    let dbs_info = to_dbs_info((*policy).governor_data as *mut PolicyDbsInfo);
    (*dbs_info).requested_freq = (*policy).cur;
}

static mut CS_GOVERNOR: DbsGovernor = dbs_governor_initializer!("conservative", cs_dbs_update, cs_alloc, cs_free, cs_init, cs_exit, cs_start, cs_limits);

// CPU_FREQ_GOV_CONSERVATIVE expands to (cs_governor.gov).
const CPU_FREQ_GOV_CONSERVATIVE: *mut CpufreqGovernor = unsafe { &mut CS_GOVERNOR.gov };

// MODULE_AUTHOR, MODULE_DESCRIPTION, MODULE_LICENSE, and governor init/exit are
// retained as build-system metadata supplied by the surrounding kernel.
#[cfg(CONFIG_CPU_FREQ_DEFAULT_GOV_CONSERVATIVE)]
unsafe fn cpufreq_default_governor() -> *mut CpufreqGovernor { CPU_FREQ_GOV_CONSERVATIVE }

cpufreq_governor_init!(CPU_FREQ_GOV_CONSERVATIVE);
cpufreq_governor_exit!(CPU_FREQ_GOV_CONSERVATIVE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
