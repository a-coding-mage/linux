// SPDX-License-Identifier: GPL-2.0-only
// core.c:9878..10630. cgroup bandwidth/weight controls, validations and rounding.
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn tg_set_cfs_bandwidth(
    tg: *mut task_group,
    period_us: u64,
    quota_us: u64,
    burst_us: u64,
) -> c_int {
    let period = period_us.wrapping_mul(LUPOS_CORE_NSEC_PER_USEC as u64);
    let quota = if quota_us == LUPOS_CORE_RUNTIME_INF {
        LUPOS_CORE_RUNTIME_INF
    } else {
        quota_us.wrapping_mul(LUPOS_CORE_NSEC_PER_USEC as u64)
    };
    let burst = burst_us.wrapping_mul(LUPOS_CORE_NSEC_PER_USEC as u64);
    let cfs_b = addr_of_mut!((*tg).cfs_bandwidth);
    lupos_core_cpus_read_lock();
    lupos_core_mutex_lock(lupos_core_cfs_constraints_mutex());
    let ret = (|| {
        let ret = __cfs_schedulable(tg, period, quota);
        if ret != 0 {
            return ret;
        }
        let enabled = quota != LUPOS_CORE_RUNTIME_INF;
        let was_enabled = (*cfs_b).quota != LUPOS_CORE_RUNTIME_INF;
        if enabled && !was_enabled {
            cfs_bandwidth_usage_inc();
        }
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*cfs_b).lock));
        (*cfs_b).period = lupos_core_ns_to_ktime(period);
        (*cfs_b).quota = quota;
        (*cfs_b).burst = burst;
        __refill_cfs_bandwidth_runtime(cfs_b);
        if enabled {
            start_cfs_bandwidth(cfs_b);
        }
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*cfs_b).lock));
        let mut i = lupos_core_cpu_next(-1, lupos_core_cpu_online_mask());
        while i < nr_cpu_ids as c_int {
            let cfs = lupos_core_tg_cfs_rq(tg, i);
            let rq = (*cfs).rq;
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rf = rf.as_mut_ptr();
            lupos_core_rq_lock_irq(rq, rf);
            (*cfs).runtime_enabled = enabled as c_int;
            (*cfs).runtime_remaining = 1;
            if lupos_core_cfs_rq_throttled(cfs) {
                update_rq_clock(rq);
                unthrottle_cfs_rq(cfs);
            }
            lupos_core_rq_unlock_irq(rq, rf);
            i = lupos_core_cpu_next(i, lupos_core_cpu_online_mask());
        }
        if was_enabled && !enabled {
            cfs_bandwidth_usage_dec();
        }
        0
    })();
    lupos_core_mutex_unlock(lupos_core_cfs_constraints_mutex());
    lupos_core_cpus_read_unlock();
    ret
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn tg_get_cfs_period(tg: *mut task_group) -> u64 {
    lupos_core_ktime_to_ns((*tg).cfs_bandwidth.period) as u64 / LUPOS_CORE_NSEC_PER_USEC as u64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn tg_get_cfs_quota(tg: *mut task_group) -> u64 {
    let quota = (*tg).cfs_bandwidth.quota;
    if quota == LUPOS_CORE_RUNTIME_INF {
        quota
    } else {
        quota / LUPOS_CORE_NSEC_PER_USEC as u64
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn tg_get_cfs_burst(tg: *mut task_group) -> u64 {
    (*tg).cfs_bandwidth.burst / LUPOS_CORE_NSEC_PER_USEC as u64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn normalize_cfs_quota(tg: *mut task_group, d: *mut cfs_schedulable_data) -> u64 {
    let (period, quota) = if tg == (*d).tg {
        ((*d).period, (*d).quota)
    } else {
        (tg_get_cfs_period(tg), tg_get_cfs_quota(tg))
    };
    if quota == LUPOS_CORE_RUNTIME_INF || quota == u64::MAX {
        return LUPOS_CORE_RUNTIME_INF;
    }
    to_ratio(period, quota)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe extern "C" fn tg_cfs_schedulable_down(tg: *mut task_group, data: *mut c_void) -> c_int {
    let d = data.cast::<cfs_schedulable_data>();
    let cfs_b = addr_of_mut!((*tg).cfs_bandwidth);
    let mut quota: i64;
    if (*tg).parent.is_null() {
        quota = LUPOS_CORE_RUNTIME_INF as i64;
    } else {
        quota = normalize_cfs_quota(tg, d) as i64;
        let parent = (*(*tg).parent).cfs_bandwidth.hierarchical_quota as i64;
        if lupos_core_cpu_cgroup_on_dfl() {
            if quota as u64 == LUPOS_CORE_RUNTIME_INF {
                quota = parent;
            } else if parent as u64 != LUPOS_CORE_RUNTIME_INF {
                quota = quota.min(parent);
            }
        } else if quota as u64 == LUPOS_CORE_RUNTIME_INF {
            quota = parent;
        } else if parent as u64 != LUPOS_CORE_RUNTIME_INF && quota > parent {
            return -(LUPOS_CORE_EINVAL as c_int);
        }
    }
    (*cfs_b).hierarchical_quota = quota as _;
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn __cfs_schedulable(tg: *mut task_group, period: u64, quota: u64) -> c_int {
    let mut data = MaybeUninit::<cfs_schedulable_data>::zeroed();
    let data = data.as_mut_ptr();
    (*data).tg = tg;
    (*data).period = period;
    (*data).quota = quota;
    if quota != LUPOS_CORE_RUNTIME_INF {
        (*data).period /= LUPOS_CORE_NSEC_PER_USEC as u64;
        (*data).quota /= LUPOS_CORE_NSEC_PER_USEC as u64;
    }
    lupos_core_rcu_read_lock();
    let ret = lupos_core_walk_tg_tree(Some(tg_cfs_schedulable_down), Some(tg_nop), data.cast());
    lupos_core_rcu_read_unlock();
    ret
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_cfs_stat_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    let tg = lupos_core_css_tg(lupos_core_seq_css(sf));
    let cfs = addr_of!((*tg).cfs_bandwidth);
    seq_printf(sf, b"nr_periods %d\n\0".as_ptr().cast(), (*cfs).nr_periods);
    seq_printf(
        sf,
        b"nr_throttled %d\n\0".as_ptr().cast(),
        (*cfs).nr_throttled,
    );
    seq_printf(
        sf,
        b"throttled_time %llu\n\0".as_ptr().cast(),
        (*cfs).throttled_time,
    );
    if lupos_core_schedstat_enabled() && tg != addr_of_mut!(root_task_group) {
        let mut ws: u64 = 0;
        let mut i = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
        while i < nr_cpu_ids as c_int {
            let stats = lupos_core_schedstats_from_se(lupos_core_tg_se(tg, i));
            ws = ws.wrapping_add(lupos_core_schedstat_wait_sum(stats));
            i = lupos_core_cpu_next(i, lupos_core_cpu_possible_mask());
        }
        seq_printf(sf, b"wait_sum %llu\n\0".as_ptr().cast(), ws);
    }
    seq_printf(sf, b"nr_bursts %d\n\0".as_ptr().cast(), (*cfs).nr_burst);
    seq_printf(
        sf,
        b"burst_time %llu\n\0".as_ptr().cast(),
        (*cfs).burst_time,
    );
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
unsafe fn throttled_time_self(tg: *mut task_group) -> u64 {
    let mut total: u64 = 0;
    let mut i = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
    while i < nr_cpu_ids as c_int {
        total = total.wrapping_add(lupos_core_read_once_u64(addr_of!(
            (*lupos_core_tg_cfs_rq(tg, i)).throttled_clock_self_time
        )));
        i = lupos_core_cpu_next(i, lupos_core_cpu_possible_mask());
    }
    total
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_cfs_local_stat_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    seq_printf(
        sf,
        b"throttled_time %llu\n\0".as_ptr().cast(),
        throttled_time_self(lupos_core_css_tg(lupos_core_seq_css(sf))),
    );
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
unsafe fn tg_bandwidth(tg: *mut task_group, period: *mut u64, quota: *mut u64, burst: *mut u64) {
    #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
    {
        if !period.is_null() {
            *period = tg_get_cfs_period(tg);
        }
        if !quota.is_null() {
            *quota = tg_get_cfs_quota(tg);
        }
        if !burst.is_null() {
            *burst = tg_get_cfs_burst(tg);
        }
    }
    #[cfg(not(CONFIG_CFS_BANDWIDTH))]
    {
        if !period.is_null() {
            *period = (*tg).scx.bw_period_us;
        }
        if !quota.is_null() {
            *quota = (*tg).scx.bw_quota_us;
        }
        if !burst.is_null() {
            *burst = (*tg).scx.bw_burst_us;
        }
    }
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
unsafe fn tg_set_bandwidth(tg: *mut task_group, period: u64, quota: u64, burst: u64) -> c_int {
    let max_usec = u64::MAX / LUPOS_CORE_NSEC_PER_USEC as u64;
    if tg == addr_of_mut!(root_task_group) {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if period > max_usec
        || (quota != LUPOS_CORE_RUNTIME_INF && quota > max_usec)
        || burst > max_usec
    {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if quota < LUPOS_CORE_MIN_BW_QUOTA_PERIOD_US || period < LUPOS_CORE_MIN_BW_QUOTA_PERIOD_US {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if period > max_bw_quota_period_us {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if quota != LUPOS_CORE_RUNTIME_INF && quota > LUPOS_CORE_MAX_BW_RUNTIME_US {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if quota != LUPOS_CORE_RUNTIME_INF
        && (burst > quota || burst.wrapping_add(quota) > LUPOS_CORE_MAX_BW_RUNTIME_US)
    {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
    let ret = tg_set_cfs_bandwidth(tg, period, quota, burst);
    #[cfg(not(CONFIG_CFS_BANDWIDTH))]
    let ret = 0;
    if ret == 0 {
        lupos_core_header_scx_group_set_bandwidth(tg, period, quota, burst);
    }
    ret
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_period_read_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> u64 {
    let mut period = 0;
    tg_bandwidth(lupos_core_css_tg(css), &mut period, null_mut(), null_mut());
    period
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_quota_read_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> i64 {
    let mut quota = 0;
    tg_bandwidth(lupos_core_css_tg(css), null_mut(), &mut quota, null_mut());
    quota as i64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_burst_read_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> u64 {
    let mut burst = 0;
    tg_bandwidth(lupos_core_css_tg(css), null_mut(), null_mut(), &mut burst);
    burst
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_period_write_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    period: u64,
) -> c_int {
    let tg = lupos_core_css_tg(css);
    let (mut quota, mut burst) = (0, 0);
    tg_bandwidth(tg, null_mut(), &mut quota, &mut burst);
    tg_set_bandwidth(tg, period, quota, burst)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_quota_write_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    quota: i64,
) -> c_int {
    let tg = lupos_core_css_tg(css);
    let (mut period, mut burst) = (0, 0);
    let quota = if quota < 0 {
        LUPOS_CORE_RUNTIME_INF
    } else {
        quota as u64
    };
    tg_bandwidth(tg, &mut period, null_mut(), &mut burst);
    tg_set_bandwidth(tg, period, quota, burst)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_burst_write_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    burst: u64,
) -> c_int {
    let tg = lupos_core_css_tg(css);
    let (mut period, mut quota) = (0, 0);
    tg_bandwidth(tg, &mut period, &mut quota, null_mut());
    tg_set_bandwidth(tg, period, quota, burst)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn cpu_rt_runtime_write(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    val: i64,
) -> c_int {
    sched_group_set_rt_runtime(lupos_core_css_tg(css), val as c_long)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn cpu_rt_runtime_read(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> i64 {
    sched_group_rt_runtime(lupos_core_css_tg(css)) as i64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn cpu_rt_period_write_uint(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    period: u64,
) -> c_int {
    sched_group_set_rt_period(lupos_core_css_tg(css), period)
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn cpu_rt_period_read_uint(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> u64 {
    sched_group_rt_period(lupos_core_css_tg(css)) as u64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_idle_read_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> i64 {
    (*lupos_core_css_tg(css)).idle as i64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_idle_write_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    idle: i64,
) -> c_int {
    let tg = lupos_core_css_tg(css);
    let ret = lupos_core_header_sched_group_set_idle(tg, idle as c_long);
    if ret == 0 {
        lupos_core_header_scx_group_set_idle(tg, idle != 0);
    }
    ret
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_rt_group_sched(s: *mut c_char) -> c_int {
    let mut val: c_long = 0;
    if lupos_core_header_kstrtol(s, 0, &mut val) != 0 || val < 0 || val > 1 {
        lupos_core_pr_warn(b"Unable to set rt_group_sched\n\0".as_ptr().cast());
        return 1;
    }
    if val != 0 {
        lupos_core_rt_group_sched_enable();
    } else {
        lupos_core_rt_group_sched_disable();
    }
    1
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_RT_GROUP_SCHED))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn cpu_rt_group_init() -> c_int {
    if !lupos_core_rt_group_sched_enabled() {
        return 0;
    }
    lupos_core_warn_site_10370(
        cgroup_add_legacy_cftypes(addr_of_mut!(cpu_cgrp_subsys), lupos_core_rt_group_files()) != 0,
    );
    0
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_extra_stat_show(
    sf: *mut seq_file,
    css: *mut cgroup_subsys_state,
) -> c_int {
    #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
    {
        let cfs = addr_of!((*lupos_core_css_tg(css)).cfs_bandwidth);
        seq_printf(sf, b"nr_periods %d\nnr_throttled %d\nthrottled_usec %llu\nnr_bursts %d\nburst_usec %llu\n\0".as_ptr().cast(),
            (*cfs).nr_periods, (*cfs).nr_throttled, (*cfs).throttled_time / LUPOS_CORE_NSEC_PER_USEC as u64,
            (*cfs).nr_burst, (*cfs).burst_time / LUPOS_CORE_NSEC_PER_USEC as u64);
    }
    #[cfg(not(CONFIG_CFS_BANDWIDTH))]
    {
        let _ = (sf, css);
    }
    0
}
#[cfg(CONFIG_CGROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn cpu_local_stat_show(
    sf: *mut seq_file,
    css: *mut cgroup_subsys_state,
) -> c_int {
    #[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_CFS_BANDWIDTH))]
    seq_printf(
        sf,
        b"throttled_usec %llu\n\0".as_ptr().cast(),
        throttled_time_self(lupos_core_css_tg(css)) / LUPOS_CORE_NSEC_PER_USEC as u64,
    );
    #[cfg(not(CONFIG_CFS_BANDWIDTH))]
    {
        let _ = (sf, css);
    }
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_weight_read_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> u64 {
    lupos_core_sched_weight_to_cgroup(tg_weight(lupos_core_css_tg(css))) as u64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_weight_write_u64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    value: u64,
) -> c_int {
    if value < LUPOS_CORE_CGROUP_WEIGHT_MIN as u64 || value > LUPOS_CORE_CGROUP_WEIGHT_MAX as u64 {
        return -(LUPOS_CORE_ERANGE as c_int);
    }
    let tg = lupos_core_css_tg(css);
    let weight = lupos_core_sched_weight_from_cgroup(value as c_ulong);
    let ret = lupos_core_header_sched_group_set_shares(tg, lupos_core_scale_load(weight));
    if ret == 0 {
        lupos_core_header_scx_group_set_weight(tg, value as c_ulong);
    }
    ret
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_weight_nice_read_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
) -> i64 {
    let weight = tg_weight(lupos_core_css_tg(css));
    let mut last_delta = c_int::MAX;
    let mut prio: c_int = 0;
    while prio < sched_prio_to_weight.len() as c_int {
        // C promotes to unsigned long, abs() uses matching signed long, then
        // assignment narrows to int; preserve all three native-width steps.
        let diff = (sched_prio_to_weight[prio as usize] as c_ulong).wrapping_sub(weight) as c_long;
        let delta = diff.wrapping_abs() as c_int;
        if delta >= last_delta {
            break;
        }
        last_delta = delta;
        prio = prio.wrapping_add(1);
    }
    lupos_core_prio_to_nice(
        prio.wrapping_sub(1)
            .wrapping_add(LUPOS_CORE_MAX_RT_PRIO as c_int),
    ) as i64
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_WEIGHT))]
#[no_mangle]
pub unsafe extern "C" fn cpu_weight_nice_write_s64(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    nice: i64,
) -> c_int {
    if nice < LUPOS_CORE_MIN_NICE as i64 || nice > LUPOS_CORE_MAX_NICE as i64 {
        return -(LUPOS_CORE_ERANGE as c_int);
    }
    let idx = lupos_core_nice_to_prio(nice as c_int).wrapping_sub(LUPOS_CORE_MAX_RT_PRIO as c_int);
    let idx = lupos_core_array_index_nospec(idx as usize, 40);
    let weight = sched_prio_to_weight[idx] as c_ulong;
    let tg = lupos_core_css_tg(css);
    let ret = lupos_core_header_sched_group_set_shares(tg, lupos_core_scale_load(weight));
    if ret == 0 {
        lupos_core_header_scx_group_set_weight(tg, lupos_core_sched_weight_to_cgroup(weight));
    }
    ret
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn cpu_period_quota_print(sf: *mut seq_file, period: c_long, quota: c_long) {
    if quota < 0 {
        lupos_core_header_seq_puts(sf, b"max\0".as_ptr().cast());
    } else {
        seq_printf(sf, b"%ld\0".as_ptr().cast(), quota);
    }
    seq_printf(sf, b" %ld\n\0".as_ptr().cast(), period);
}
#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn cpu_period_quota_parse(buf: *mut c_char, period: *mut u64, quota: *mut u64) -> c_int {
    let mut tok = [0 as c_char; 21];
    if sscanf(
        buf,
        b"%20s %llu\0".as_ptr().cast(),
        tok.as_mut_ptr(),
        period,
    ) < 1
    {
        return -(LUPOS_CORE_EINVAL as c_int);
    }
    if sscanf(tok.as_ptr(), b"%llu\0".as_ptr().cast(), quota) < 1 {
        if strcmp(tok.as_ptr(), b"max\0".as_ptr().cast()) == 0 {
            *quota = LUPOS_CORE_RUNTIME_INF;
        } else {
            return -(LUPOS_CORE_EINVAL as c_int);
        }
    }
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_max_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    let tg = lupos_core_css_tg(lupos_core_seq_css(sf));
    let (mut period, mut quota) = (0, 0);
    tg_bandwidth(tg, &mut period, &mut quota, null_mut());
    cpu_period_quota_print(sf, period as c_long, quota as c_long);
    0
}
#[cfg(all(CONFIG_CGROUP_SCHED, CONFIG_GROUP_SCHED_BANDWIDTH))]
#[no_mangle]
pub unsafe extern "C" fn cpu_max_write(
    of: *mut kernfs_open_file,
    buf: *mut c_char,
    nbytes: usize,
    _off: loff_t,
) -> isize {
    let tg = lupos_core_css_tg(lupos_core_of_css(of));
    let (mut period, mut quota, mut burst) = (0, 0, 0);
    tg_bandwidth(tg, &mut period, null_mut(), &mut burst);
    let mut ret = cpu_period_quota_parse(buf, &mut period, &mut quota);
    if ret == 0 {
        ret = tg_set_bandwidth(tg, period, quota, burst);
    }
    if ret != 0 {
        ret as isize
    } else {
        nbytes as isize
    }
}
