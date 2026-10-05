// SPDX-License-Identifier: GPL-2.0-only
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_core_setup_proxy_exec(str_: *mut c_char) -> c_int {
    #[cfg(CONFIG_SCHED_PROXY_EXEC)]
    {
        let mut enabled = true;
        if *str_ != 0 && kstrtobool(str_.add(1), addr_of_mut!(enabled)) != 0 {
            lupos_core_print_proxy_parse();
            return 0;
        }
        if enabled {
            lupos_core_print_proxy_enabled();
            lupos_core_proxy_key_enable();
        } else {
            lupos_core_print_proxy_disabled();
            lupos_core_proxy_key_disable();
        }
        1
    }
    #[cfg(not(CONFIG_SCHED_PROXY_EXEC))]
    {
        lupos_core_print_proxy_unavailable();
        0
    }
}
#[cfg(any(CONFIG_RT_GROUP_SCHED, CONFIG_FAIR_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn walk_tg_tree_from(
    from: *mut task_group,
    down: tg_visitor,
    up: tg_visitor,
    data: *mut c_void,
) -> c_int {
    let mut parent = from;
    loop {
        let ret = down.unwrap_unchecked()(parent, data);
        if ret != 0 {
            return ret;
        }
        let head = addr_of_mut!((*parent).children);
        lupos_core_check_tg_tree_rcu();
        let first = lupos_core_list_next_rcu(head);
        if first != head {
            parent = first
                .cast::<u8>()
                .sub(offset_of!(task_group, siblings))
                .cast();
            continue;
        }
        loop {
            let ret = up.unwrap_unchecked()(parent, data);
            if ret != 0 || parent == from {
                return ret;
            }
            let child = parent;
            parent = (*parent).parent;
            if parent.is_null() {
                return ret;
            }
            let next = lupos_core_list_next_rcu(addr_of_mut!((*child).siblings));
            if next != addr_of_mut!((*parent).children) {
                parent = next
                    .cast::<u8>()
                    .sub(offset_of!(task_group, siblings))
                    .cast();
                break;
            }
        }
    }
}
#[cfg(any(CONFIG_RT_GROUP_SCHED, CONFIG_FAIR_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn tg_nop(_tg: *mut task_group, _data: *mut c_void) -> c_int {
    0
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn __set_numabalancing_state(enabled: bool) {
    if enabled {
        lupos_core_numa_key_enable();
    } else {
        lupos_core_numa_key_disable();
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn set_numabalancing_state(enabled: bool) {
    sysctl_numa_balancing_mode = if enabled {
        LUPOS_CORE_NUMA_BALANCING_NORMAL
    } else {
        LUPOS_CORE_NUMA_BALANCING_DISABLED
    };
    __set_numabalancing_state(enabled);
}
#[cfg(all(CONFIG_NUMA_BALANCING, CONFIG_SYSCTL))]
unsafe fn reset_memory_tiering() {
    let mut pgdat = first_online_pgdat();
    while !pgdat.is_null() {
        (*pgdat).nbp_threshold = 0;
        (*pgdat).nbp_th_nr_cand = lupos_core_node_page_state(pgdat, LUPOS_CORE_PGPROMOTE_CANDIDATE);
        (*pgdat).nbp_th_start = lupos_core_jiffies_to_msecs(lupos_core_jiffies()) as c_ulong;
        pgdat = next_online_pgdat(pgdat);
    }
}
#[cfg(all(CONFIG_NUMA_BALANCING, CONFIG_SYSCTL))]
#[no_mangle]
pub unsafe extern "C" fn lupos_core_sysctl_numa_balancing(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    len: *mut usize,
    pos: *mut loff_t,
) -> c_int {
    let mut state = sysctl_numa_balancing_mode;
    if write != 0 && !lupos_core_header_capable(LUPOS_CORE_CAP_SYS_ADMIN) {
        return -LUPOS_CORE_EPERM;
    }
    let mut t = *table;
    t.data = addr_of_mut!(state).cast();
    let err = proc_dointvec_minmax(addr_of!(t), write, buffer, len, pos);
    if err < 0 {
        return err;
    }
    if write != 0 {
        if sysctl_numa_balancing_mode & LUPOS_CORE_NUMA_BALANCING_MEMORY_TIERING == 0
            && state & LUPOS_CORE_NUMA_BALANCING_MEMORY_TIERING != 0
        {
            reset_memory_tiering();
        }
        sysctl_numa_balancing_mode = state;
        __set_numabalancing_state(state != 0);
    }
    err
}
#[cfg(CONFIG_SCHEDSTATS)]
unsafe fn set_schedstats(enabled: bool) {
    if enabled {
        lupos_core_schedstats_key_enable();
    } else {
        lupos_core_schedstats_key_disable();
    }
}
#[cfg(CONFIG_SCHEDSTATS)]
#[no_mangle]
pub unsafe extern "C" fn force_schedstat_enabled() {
    if !lupos_core_schedstat_enabled() {
        lupos_core_print_schedstats_forced();
        lupos_core_schedstats_key_enable();
    }
}
#[cfg(CONFIG_SCHEDSTATS)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_core_setup_schedstats(str_: *mut c_char) -> c_int {
    let mut ret = 0;
    if !str_.is_null() {
        if strcmp(str_, c"enable".as_ptr().cast::<kernel::ffi::c_char>()) == 0 {
            set_schedstats(true);
            ret = 1;
        } else if strcmp(str_, c"disable".as_ptr().cast::<kernel::ffi::c_char>()) == 0 {
            set_schedstats(false);
            ret = 1;
        }
    }
    if ret == 0 {
        lupos_core_print_schedstats_parse();
    }
    ret
}
#[cfg(all(CONFIG_SCHEDSTATS, CONFIG_SYSCTL))]
#[no_mangle]
pub unsafe extern "C" fn lupos_core_sysctl_schedstats(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    len: *mut usize,
    pos: *mut loff_t,
) -> c_int {
    let mut state = lupos_core_schedstats_key_likely() as c_int;
    if write != 0 && !lupos_core_header_capable(LUPOS_CORE_CAP_SYS_ADMIN) {
        return -LUPOS_CORE_EPERM;
    }
    let mut t = *table;
    t.data = addr_of_mut!(state).cast();
    let err = proc_dointvec_minmax(addr_of!(t), write, buffer, len, pos);
    if err < 0 {
        return err;
    }
    if write != 0 {
        set_schedstats(state != 0);
    }
    err
}
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_core_sched_core_sysctl_init() -> c_int {
    lupos_core_register_core_sysctl();
    0
}
#[no_mangle]
pub unsafe extern "C" fn force_compatible_cpus_allowed_ptr(p: *mut task_struct) {
    let mut new_mask = MaybeUninit::<cpumask_var_t>::uninit();
    let mut override_mask = lupos_core_task_cpu_possible_mask(p);
    lupos_core_alloc_cpumask_var(new_mask.as_mut_ptr(), LUPOS_CORE_GFP_KERNEL);
    lupos_core_cpus_read_lock();
    'locked: {
        if lupos_core_cpumask_var_available(new_mask.as_mut_ptr()) {
            let mask = lupos_core_cpumask_var_ptr(new_mask.as_mut_ptr());
            if restrict_cpus_allowed_ptr(p, mask, override_mask) == 0 {
                break 'locked;
            }
            lupos_core_header_cpuset_cpus_allowed(p, mask);
            override_mask = mask;
        }
        if lupos_core_printk_ratelimit() {
            lupos_core_print_affinity_override(p, override_mask);
        }
        let err = set_cpus_allowed_ptr(p, override_mask);
        lupos_core_warn_override_affinity(err != 0);
    }
    lupos_core_cpus_read_unlock();
    lupos_core_free_cpumask_var(new_mask.as_mut_ptr());
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn __migrate_swap_task(p: *mut task_struct, cpu: c_int) {
    if lupos_core_task_on_rq_queued(p) {
        let src = lupos_core_task_rq(p);
        let dst = lupos_core_cpu_rq(cpu);
        let mut srf = MaybeUninit::<rq_flags>::uninit();
        let mut drf = MaybeUninit::<rq_flags>::uninit();
        lupos_core_rq_pin_lock(src, srf.as_mut_ptr());
        lupos_core_rq_pin_lock(dst, drf.as_mut_ptr());
        lupos_core_move_queued_task_locked(src, dst, p);
        wakeup_preempt(dst, p, 0);
        lupos_core_rq_unpin_lock(dst, drf.as_mut_ptr());
        lupos_core_rq_unpin_lock(src, srf.as_mut_ptr());
    } else {
        (*p).wake_cpu = cpu;
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe extern "C" fn migrate_swap_stop(data: *mut c_void) -> c_int {
    let arg = data.cast::<migration_swap_arg>();
    if !lupos_core_cpu_active((*arg).src_cpu) || !lupos_core_cpu_active((*arg).dst_cpu) {
        return -LUPOS_CORE_EAGAIN;
    }
    let src = lupos_core_cpu_rq((*arg).src_cpu);
    let dst = lupos_core_cpu_rq((*arg).dst_cpu);
    lupos_core_double_raw_spin_lock(
        addr_of_mut!((*(*arg).src_task).pi_lock),
        addr_of_mut!((*(*arg).dst_task).pi_lock),
    );
    double_rq_lock(src, dst);
    let ret = 'locked: {
        if lupos_core_task_cpu((*arg).dst_task) != (*arg).dst_cpu
            || lupos_core_task_cpu((*arg).src_task) != (*arg).src_cpu
            || !lupos_core_cpumask_test_cpu((*arg).dst_cpu, (*(*arg).src_task).cpus_ptr)
            || !lupos_core_cpumask_test_cpu((*arg).src_cpu, (*(*arg).dst_task).cpus_ptr)
        {
            break 'locked -LUPOS_CORE_EAGAIN;
        }
        __migrate_swap_task((*arg).src_task, (*arg).dst_cpu);
        __migrate_swap_task((*arg).dst_task, (*arg).src_cpu);
        0
    };
    lupos_core_double_rq_unlock(src, dst);
    lupos_core_double_raw_spin_unlock(
        addr_of_mut!((*(*arg).src_task).pi_lock),
        addr_of_mut!((*(*arg).dst_task).pi_lock),
    );
    ret
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn migrate_swap(
    cur: *mut task_struct,
    p: *mut task_struct,
    target: c_int,
    curr: c_int,
) -> c_int {
    let mut arg = migration_swap_arg {
        src_task: cur,
        src_cpu: curr,
        dst_task: p,
        dst_cpu: target,
    };
    if curr == target
        || !lupos_core_cpu_active(curr)
        || !lupos_core_cpu_active(target)
        || !lupos_core_cpumask_test_cpu(target, (*cur).cpus_ptr)
        || !lupos_core_cpumask_test_cpu(curr, (*p).cpus_ptr)
    {
        return -LUPOS_CORE_EINVAL;
    }
    lupos_core_trace_swap_numa(cur, curr, p, target);
    stop_two_cpus(
        target as c_uint,
        curr as c_uint,
        Some(migrate_swap_stop),
        addr_of_mut!(arg).cast(),
    )
}
