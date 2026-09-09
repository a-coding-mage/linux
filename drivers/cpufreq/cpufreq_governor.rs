// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/cpufreq/cpufreq_governor.c
 *
 * CPUFREQ governors common code
 *
 * Copyright (C) 2001 Russell King
 * Copyright (C) 2003 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
 * Copyright (C) 2003 Jun Nakajima <jun.nakajima@intel.com>
 * Copyright (C) 2009 Alexander Clouter <alex@digriz.org.uk>
 * Copyright (c) 2012 Viresh Kumar <viresh.kumar@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const CPUFREQ_DBS_MIN_SAMPLING_INTERVAL: c_uint = 2 * TICK_NSEC / NSEC_PER_USEC;

static mut CPU_DBS: PerCpu<cpu_dbs_info> = DEFINE_PER_CPU();
static GOV_DBS_DATA_MUTEX: Mutex = DEFINE_MUTEX();

pub unsafe fn sampling_rate_store(attr_set: *mut gov_attr_set, buf: *const c_char, count: usize) -> isize {
    let dbs_data = to_dbs_data(attr_set);
    let mut policy_dbs: *mut policy_dbs_info;
    let mut sampling_interval: c_uint = 0;
    let ret = sscanf(buf, "%u\0", &mut sampling_interval);
    if ret != 1 || sampling_interval < CPUFREQ_DBS_MIN_SAMPLING_INTERVAL { return -EINVAL; }
    (*dbs_data).sampling_rate = sampling_interval;
    list_for_each_entry!(policy_dbs, &mut (*attr_set).policy_list, list, {
        mutex_lock(&mut (*policy_dbs).update_mutex);
        gov_update_sample_delay(policy_dbs, 0);
        mutex_unlock(&mut (*policy_dbs).update_mutex);
    });
    count as isize
}

pub unsafe fn gov_update_cpu_data(dbs_data: *mut dbs_data) {
    let mut policy_dbs: *mut policy_dbs_info;
    list_for_each_entry!(policy_dbs, &mut (*dbs_data).attr_set.policy_list, list, {
        mutex_lock(&mut (*policy_dbs).update_mutex);
        for_each_cpu!(j, (*policy_dbs).policy.cpus, {
            let j_cdbs = &mut per_cpu!(CPU_DBS, j);
            j_cdbs.prev_cpu_idle = get_cpu_idle_time(j, &mut j_cdbs.prev_update_time, (*dbs_data).io_is_busy);
            j_cdbs.prev_cpu_nice = kcpustat_field(CPUTIME_NICE, j);
        });
        mutex_unlock(&mut (*policy_dbs).update_mutex);
    });
}

pub unsafe fn dbs_update(policy: *mut cpufreq_policy) -> c_uint {
    let policy_dbs = (*policy).governor_data as *mut policy_dbs_info;
    let dbs_data = (*policy_dbs).dbs_data;
    let ignore_nice = (*dbs_data).ignore_nice_load;
    let mut max_load: c_uint = 0;
    let mut idle_periods: c_uint = UINT_MAX;
    let sampling_rate = (*dbs_data).sampling_rate * (*policy_dbs).rate_mult;
    let io_busy = (*dbs_data).io_is_busy;
    for_each_cpu!(j, (*policy).cpus, {
        let j_cdbs = &mut per_cpu!(CPU_DBS, j);
        let mut update_time: u64 = 0;
        let cur_idle_time = get_cpu_idle_time(j, &mut update_time, io_busy);
        let time_elapsed = update_time - j_cdbs.prev_update_time;
        j_cdbs.prev_update_time = update_time;
        let mut idle_time = if cur_idle_time > j_cdbs.prev_cpu_idle { (cur_idle_time - j_cdbs.prev_cpu_idle) as c_uint } else { 0 };
        j_cdbs.prev_cpu_idle = cur_idle_time;
        let cur_nice = kcpustat_field(CPUTIME_NICE, j);
        if ignore_nice != 0 { idle_time += div_u64(cur_nice - j_cdbs.prev_cpu_nice, NSEC_PER_USEC) as c_uint; }
        j_cdbs.prev_cpu_nice = cur_nice;
        let load;
        if time_elapsed == 0 { load = j_cdbs.prev_load; }
        else if idle_time > 2 * sampling_rate && j_cdbs.prev_load != 0 {
            load = j_cdbs.prev_load;
            j_cdbs.prev_load = 0;
        } else {
            load = if time_elapsed > idle_time as u64 { 100 * (time_elapsed - idle_time as u64) as c_uint / time_elapsed as c_uint } else { 0 };
            j_cdbs.prev_load = load;
        }
        if idle_time > 2 * sampling_rate {
            let periods = idle_time / sampling_rate;
            if periods < idle_periods { idle_periods = periods; }
        }
        if load > max_load { max_load = load; }
    });
    (*policy_dbs).idle_periods = idle_periods;
    max_load
}

unsafe fn dbs_work_handler(work: *mut work_struct) {
    let policy_dbs = container_of!(work, policy_dbs_info, work);
    let policy = (*policy_dbs).policy;
    let gov = dbs_governor_of(policy);
    mutex_lock(&mut (*policy_dbs).update_mutex);
    gov_update_sample_delay(policy_dbs, ((*gov).gov_dbs_update)(policy));
    mutex_unlock(&mut (*policy_dbs).update_mutex);
    atomic_set(&mut (*policy_dbs).work_count, 0);
    smp_wmb();
    (*policy_dbs).work_in_progress = false;
}

unsafe fn dbs_irq_work(irq_work: *mut irq_work) {
    let policy_dbs = container_of!(irq_work, policy_dbs_info, irq_work);
    schedule_work_on(smp_processor_id(), &mut (*policy_dbs).work);
}

unsafe fn dbs_update_util_handler(data: *mut update_util_data, time: u64, _flags: c_uint) {
    let cdbs = container_of!(data, cpu_dbs_info, update_util);
    let policy_dbs = (*cdbs).policy_dbs;
    if !cpufreq_this_cpu_can_update((*policy_dbs).policy) || (*policy_dbs).work_in_progress { return; }
    smp_rmb();
    let lst = READ_ONCE!((*policy_dbs).last_sample_time);
    let delta_ns = time - lst;
    if (delta_ns as i64) < (*policy_dbs).sample_delay_ns { return; }
    if (*policy_dbs).is_shared {
        if !atomic_add_unless(&mut (*policy_dbs).work_count, 1, 1) { return; }
        if lst != READ_ONCE!((*policy_dbs).last_sample_time) { atomic_set(&mut (*policy_dbs).work_count, 0); return; }
    }
    (*policy_dbs).last_sample_time = time;
    (*policy_dbs).work_in_progress = true;
    irq_work_queue(&mut (*policy_dbs).irq_work);
}

unsafe fn gov_set_update_util(policy_dbs: *mut policy_dbs_info, delay_us: c_uint) {
    let policy = (*policy_dbs).policy;
    gov_update_sample_delay(policy_dbs, delay_us);
    (*policy_dbs).last_sample_time = 0;
    for_each_cpu!(cpu, (*policy).cpus, {
        let cdbs = &mut per_cpu!(CPU_DBS, cpu);
        cpufreq_add_update_util_hook(cpu, &mut cdbs.update_util, dbs_update_util_handler);
    });
}

unsafe fn gov_clear_update_util(policy: *mut cpufreq_policy) {
    for_each_cpu!(i, (*policy).cpus, { cpufreq_remove_update_util_hook(i); });
    synchronize_rcu();
}

unsafe fn alloc_policy_dbs_info(policy: *mut cpufreq_policy, gov: *mut dbs_governor) -> *mut policy_dbs_info {
    let policy_dbs = ((*gov).alloc)();
    if policy_dbs.is_null() { return core::ptr::null_mut(); }
    (*policy_dbs).policy = policy;
    mutex_init(&mut (*policy_dbs).update_mutex);
    atomic_set(&mut (*policy_dbs).work_count, 0);
    init_irq_work(&mut (*policy_dbs).irq_work, dbs_irq_work);
    INIT_WORK!(&mut (*policy_dbs).work, dbs_work_handler);
    for_each_cpu!(j, (*policy).related_cpus, { per_cpu!(CPU_DBS, j).policy_dbs = policy_dbs; });
    policy_dbs
}

unsafe fn free_policy_dbs_info(policy_dbs: *mut policy_dbs_info, gov: *mut dbs_governor) {
    mutex_destroy(&mut (*policy_dbs).update_mutex);
    for_each_cpu!(j, (*policy_dbs).policy.related_cpus, { per_cpu!(CPU_DBS, j).policy_dbs = core::ptr::null_mut(); per_cpu!(CPU_DBS, j).update_util.func = None; });
    ((*gov).free)(policy_dbs);
}

unsafe fn cpufreq_dbs_data_release(kobj: *mut kobject) {
    let dbs_data = to_dbs_data(to_gov_attr_set(kobj));
    let gov = (*dbs_data).gov;
    ((*gov).exit)(dbs_data);
    kfree(dbs_data as *mut c_void);
}

pub unsafe fn cpufreq_dbs_governor_init(policy: *mut cpufreq_policy) -> c_int {
    let gov = dbs_governor_of(policy);
    if !(*policy).governor_data.is_null() { return -EBUSY; }
    let policy_dbs = alloc_policy_dbs_info(policy, gov);
    if policy_dbs.is_null() { return -ENOMEM; }
    mutex_lock(&mut GOV_DBS_DATA_MUTEX);
    let mut dbs_data = (*gov).gdbs_data;
    if !dbs_data.is_null() {
        if WARN_ON(have_governor_per_policy()) { mutex_unlock(&mut GOV_DBS_DATA_MUTEX); free_policy_dbs_info(policy_dbs, gov); return -EINVAL; }
        (*policy_dbs).dbs_data = dbs_data; (*policy).governor_data = policy_dbs;
        gov_attr_set_get(&mut (*dbs_data).attr_set, &mut (*policy_dbs).list);
        mutex_unlock(&mut GOV_DBS_DATA_MUTEX); return 0;
    }
    dbs_data = kzalloc_obj!();
    if dbs_data.is_null() { mutex_unlock(&mut GOV_DBS_DATA_MUTEX); free_policy_dbs_info(policy_dbs, gov); return -ENOMEM; }
    (*dbs_data).gov = gov;
    gov_attr_set_init(&mut (*dbs_data).attr_set, &mut (*policy_dbs).list);
    let mut ret = ((*gov).init)(dbs_data);
    if ret != 0 { kfree(dbs_data as *mut c_void); mutex_unlock(&mut GOV_DBS_DATA_MUTEX); free_policy_dbs_info(policy_dbs, gov); return ret; }
    (*dbs_data).sampling_rate = max_t(CPUFREQ_DBS_MIN_SAMPLING_INTERVAL, cpufreq_policy_transition_delay_us(policy));
    if !have_governor_per_policy() { (*gov).gdbs_data = dbs_data; }
    (*policy_dbs).dbs_data = dbs_data; (*policy).governor_data = policy_dbs;
    (*gov).kobj_type.sysfs_ops = &governor_sysfs_ops; (*gov).kobj_type.release = cpufreq_dbs_data_release;
    ret = kobject_init_and_add(&mut (*dbs_data).attr_set.kobj, &mut (*gov).kobj_type, get_governor_parent_kobj(policy), "%s\0", (*gov).gov.name);
    if ret != 0 { (*policy).governor_data = core::ptr::null_mut(); if !have_governor_per_policy() { (*gov).gdbs_data = core::ptr::null_mut(); } kobject_put(&mut (*dbs_data).attr_set.kobj); free_policy_dbs_info(policy_dbs, gov); }
    mutex_unlock(&mut GOV_DBS_DATA_MUTEX); ret
}

pub unsafe fn cpufreq_dbs_governor_exit(policy: *mut cpufreq_policy) {
    let gov = dbs_governor_of(policy); let policy_dbs = (*policy).governor_data as *mut policy_dbs_info; let dbs_data = (*policy_dbs).dbs_data;
    mutex_lock(&mut GOV_DBS_DATA_MUTEX);
    let count = gov_attr_set_put(&mut (*dbs_data).attr_set, &mut (*policy_dbs).list);
    (*policy).governor_data = core::ptr::null_mut();
    if count == 0 && !have_governor_per_policy() { (*gov).gdbs_data = core::ptr::null_mut(); }
    free_policy_dbs_info(policy_dbs, gov); mutex_unlock(&mut GOV_DBS_DATA_MUTEX);
}

pub unsafe fn cpufreq_dbs_governor_start(policy: *mut cpufreq_policy) -> c_int {
    let gov = dbs_governor_of(policy); let policy_dbs = (*policy).governor_data as *mut policy_dbs_info; let dbs_data = (*policy_dbs).dbs_data;
    if (*policy).cur.is_null() { return -EINVAL; }
    (*policy_dbs).is_shared = policy_is_shared(policy); (*policy_dbs).rate_mult = 1;
    mutex_lock(&mut (*policy_dbs).update_mutex);
    let io_busy = (*dbs_data).io_is_busy;
    for_each_cpu!(j, (*policy).cpus, { let cdbs = &mut per_cpu!(CPU_DBS, j); cdbs.prev_cpu_idle = get_cpu_idle_time(j, &mut cdbs.prev_update_time, io_busy); cdbs.prev_load = 0; cdbs.prev_cpu_nice = kcpustat_field(CPUTIME_NICE, j); });
    mutex_unlock(&mut (*policy_dbs).update_mutex);
    ((*gov).start)(policy); gov_set_update_util(policy_dbs, (*dbs_data).sampling_rate); 0
}

pub unsafe fn cpufreq_dbs_governor_stop(policy: *mut cpufreq_policy) {
    let policy_dbs = (*policy).governor_data as *mut policy_dbs_info;
    gov_clear_update_util((*policy_dbs).policy); irq_work_sync(&mut (*policy_dbs).irq_work); cancel_work_sync(&mut (*policy_dbs).work); atomic_set(&mut (*policy_dbs).work_count, 0); (*policy_dbs).work_in_progress = false;
}

pub unsafe fn cpufreq_dbs_governor_limits(policy: *mut cpufreq_policy) {
    let gov = dbs_governor_of(policy); mutex_lock(&mut GOV_DBS_DATA_MUTEX); let policy_dbs = (*policy).governor_data as *mut policy_dbs_info;
    if policy_dbs.is_null() { mutex_unlock(&mut GOV_DBS_DATA_MUTEX); return; }
    mutex_lock(&mut (*policy_dbs).update_mutex); cpufreq_policy_apply_limits(policy); gov_update_sample_delay(policy_dbs, 0); if let Some(limits) = (*gov).limits { limits(policy); } mutex_unlock(&mut (*policy_dbs).update_mutex); mutex_unlock(&mut GOV_DBS_DATA_MUTEX);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
