// SPDX-License-Identifier: GPL-2.0-only
// core.c:8287..8354 and 8898..9155; initialization order follows frozen C.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_idle(idle: *mut task_struct, cpu: c_int) {
    let mut ac = MaybeUninit::<affinity_context>::zeroed();
    let ac = ac.as_mut_ptr();
    (*ac).new_mask = lupos_core_cpumask_of(cpu);
    (*ac).flags = 0;
    let rq = lupos_core_cpu_rq(cpu);
    let flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*idle).pi_lock));
    raw_spin_rq_lock_nested(rq, 0);
    (*idle).__state = LUPOS_CORE_TASK_RUNNING;
    (*idle).se.exec_start = sched_clock();
    (*idle).flags |= LUPOS_CORE_PF_KTHREAD | LUPOS_CORE_PF_NO_SETAFFINITY;
    kthread_set_per_cpu(idle, cpu);
    set_cpus_allowed_common(idle, ac);
    lupos_core_rcu_read_lock();
    lupos_core___set_task_cpu(idle, cpu as c_uint);
    lupos_core_rcu_read_unlock();
    (*rq).idle = idle;
    lupos_core_rq_set_donor(rq, idle);
    lupos_core_rcu_assign_rq_curr(rq, idle);
    (*idle).on_rq = LUPOS_CORE_TASK_ON_RQ_QUEUED as u8;
    (*idle).on_cpu = 1;
    lupos_core_raw_spin_rq_unlock(rq);
    lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*idle).pi_lock), flags);
    lupos_core_init_idle_preempt_count(idle, cpu);
    (*idle).sched_class = addr_of!(idle_sched_class);
    lupos_core_header_ftrace_graph_init_idle_task(idle, cpu);
    lupos_core_header_vtime_init_idle(idle, cpu);
    sprintf(
        (*idle).comm.as_mut_ptr(),
        b"%s/%d\0".as_ptr().cast(),
        lupos_core_init_task_comm(),
        cpu,
    );
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_init_smp() {
    lupos_core_header_sched_init_numa(LUPOS_CORE_NUMA_NO_NODE as c_int);
    lupos_core_prandom_init_sched_once();
    sched_domains_mutex_lock();
    sched_init_domains(lupos_core_cpu_active_mask());
    sched_domains_mutex_unlock();
    let current = lupos_core_current();
    if set_cpus_allowed_ptr(
        current,
        lupos_core_housekeeping_cpumask(LUPOS_CORE_HK_TYPE_DOMAIN),
    ) < 0
    {
        lupos_core_bug_site_8913();
    }
    (*current).flags &= !LUPOS_CORE_PF_NO_SETAFFINITY;
    sched_init_granularity();
    init_sched_rt_class();
    init_sched_dl_class();
    sched_init_dl_servers();
    sched_smp_initialized = true;
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn migration_init() -> c_int {
    sched_cpu_starting(lupos_core_smp_processor_id() as c_uint);
    0
}
#[no_mangle]
pub unsafe extern "C" fn in_sched_functions(addr: c_ulong) -> c_int {
    (in_lock_functions(addr) != 0
        || (addr >= lupos_core_sched_text_start() && addr < lupos_core_sched_text_end()))
        as c_int
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_init() {
    lupos_core_bug_on_site_8967(!lupos_core_sched_class_above(
        addr_of!(stop_sched_class),
        addr_of!(dl_sched_class),
    ));
    lupos_core_bug_on_site_8968(!lupos_core_sched_class_above(
        addr_of!(dl_sched_class),
        addr_of!(rt_sched_class),
    ));
    lupos_core_bug_on_site_8969(!lupos_core_sched_class_above(
        addr_of!(rt_sched_class),
        addr_of!(fair_sched_class),
    ));
    lupos_core_bug_on_site_8970(!lupos_core_sched_class_above(
        addr_of!(fair_sched_class),
        addr_of!(idle_sched_class),
    ));
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    {
        lupos_core_bug_on_site_8972(!lupos_core_sched_class_above(
            addr_of!(fair_sched_class),
            addr_of!(ext_sched_class),
        ));
        lupos_core_bug_on_site_8973(!lupos_core_sched_class_above(
            addr_of!(ext_sched_class),
            addr_of!(idle_sched_class),
        ));
    }
    wait_bit_init();
    #[cfg(CONFIG_FAIR_GROUP_SCHED)]
    {
        root_task_group.cfs_rq = lupos_core_percpu_runqueues_cfs();
        root_task_group.shares = LUPOS_CORE_ROOT_TASK_GROUP_LOAD as c_ulong;
        init_cfs_bandwidth(addr_of_mut!(root_task_group.cfs_bandwidth), null_mut());
    }
    #[cfg(CONFIG_EXT_GROUP_SCHED)]
    lupos_core_header_scx_tg_init(addr_of_mut!(root_task_group));
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    {
        let bytes = (2 as c_ulong)
            .wrapping_mul(nr_cpu_ids as c_ulong)
            .wrapping_mul(core::mem::size_of::<*mut *mut c_void>() as c_ulong);
        let ptr = lupos_core_kzalloc(bytes as usize, LUPOS_CORE_GFP_NOWAIT);
        root_task_group.rt_se = ptr.cast();
        root_task_group.rt_rq = ptr
            .cast::<u8>()
            .wrapping_add(
                (nr_cpu_ids as usize).wrapping_mul(core::mem::size_of::<*mut *mut c_void>()),
            )
            .cast();
    }
    init_defrootdomain();
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    init_rt_bandwidth(
        addr_of_mut!(root_task_group.rt_bandwidth),
        lupos_core_global_rt_period(),
        lupos_core_global_rt_runtime(),
    );
    #[cfg(CONFIG_CGROUP_SCHED)]
    {
        *lupos_core_task_group_cache_slot() = lupos_core_kmem_cache_task_group_create();
        lupos_core_list_add(addr_of_mut!(root_task_group.list), lupos_core_task_groups());
        lupos_core_init_list_head(addr_of_mut!(root_task_group.children));
        lupos_core_init_list_head(addr_of_mut!(root_task_group.siblings));
        lupos_core_header_autogroup_init(addr_of_mut!(init_task));
    }
    let mut i = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
    while i < nr_cpu_ids as c_int {
        let rq = lupos_core_cpu_rq(i);
        lupos_core_raw_spin_lock_init_rq(addr_of_mut!((*rq).__lock));
        (*rq).nr_running = 0;
        (*rq).calc_load_active = 0;
        (*rq).calc_load_update = lupos_core_jiffies().wrapping_add(LUPOS_CORE_LOAD_FREQ as c_ulong);
        init_cfs_rq(addr_of_mut!((*rq).cfs));
        init_rt_rq(addr_of_mut!((*rq).rt));
        init_dl_rq(addr_of_mut!((*rq).dl));
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        {
            lupos_core_init_list_head(addr_of_mut!((*rq).leaf_cfs_rq_list));
            (*rq).tmp_alone_branch = addr_of_mut!((*rq).leaf_cfs_rq_list);
            init_tg_cfs_entry(
                addr_of_mut!(root_task_group),
                addr_of_mut!((*rq).cfs),
                null_mut(),
                i,
                null_mut(),
            );
        }
        #[cfg(CONFIG_RT_GROUP_SCHED)]
        {
            (*rq).rt.rt_runtime = lupos_core_global_rt_runtime() as _;
            init_tg_rt_entry(
                addr_of_mut!(root_task_group),
                addr_of_mut!((*rq).rt),
                null_mut(),
                i,
                null_mut(),
            );
        }
        (*rq).next_class = addr_of!(idle_sched_class);
        (*rq).sd = null_mut();
        (*rq).rd = null_mut();
        (*rq).cpu_capacity = LUPOS_CORE_SCHED_CAPACITY_SCALE as c_ulong;
        (*rq).balance_callback = addr_of_mut!(balance_push_callback);
        (*rq).active_balance = 0;
        (*rq).next_balance = lupos_core_jiffies();
        (*rq).push_cpu = 0;
        (*rq).cpu = i;
        (*rq).online = 0;
        (*rq).idle_stamp = 0;
        (*rq).avg_idle = (2 as c_uint).wrapping_mul(sysctl_sched_migration_cost) as u64;
        (*rq).max_idle_balance_cost = sysctl_sched_migration_cost as _;
        lupos_core_init_list_head(addr_of_mut!((*rq).cfs_tasks));
        rq_attach_root(rq, addr_of_mut!(def_root_domain));
        #[cfg(CONFIG_NO_HZ_COMMON)]
        {
            (*rq).last_blocked_load_update_tick = lupos_core_jiffies();
            lupos_core_atomic_set(addr_of_mut!((*rq).nohz_flags), 0);
            lupos_core_init_csd(addr_of_mut!((*rq).nohz_csd), Some(nohz_csd_func), rq.cast());
        }
        #[cfg(CONFIG_HOTPLUG_CPU)]
        lupos_core_rcuwait_init(addr_of_mut!((*rq).hotplug_wait));
        hrtick_rq_init(rq);
        lupos_core_atomic_set(addr_of_mut!((*rq).nr_iowait), 0);
        fair_server_init(rq);
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        ext_server_init(rq);
        #[cfg(CONFIG_SCHED_CORE)]
        {
            (*rq).core = rq;
            (*rq).core_pick = null_mut();
            (*rq).core_dl_server = null_mut();
            (*rq).core_enabled = 0;
            lupos_core_rb_root_init(addr_of_mut!((*rq).core_tree));
            (*rq).core_forceidle_count = 0;
            (*rq).core_forceidle_occupation = 0;
            (*rq).core_forceidle_start = 0;
            (*rq).core_pick_in_flight = 0;
            (*rq).core_cookie = 0;
        }
        #[cfg(CONFIG_SCHED_CACHE)]
        {
            lupos_core_raw_spin_lock_init_epoch(addr_of_mut!((*rq).cpu_epoch_lock));
            (*rq).cpu_epoch_next = lupos_core_jiffies();
        }
        lupos_core_zalloc_cpumask_var_node(
            addr_of_mut!((*rq).scratch_mask),
            LUPOS_CORE_GFP_KERNEL,
            lupos_core_cpu_to_node(i),
        );
        i = lupos_core_cpu_next(i, lupos_core_cpu_possible_mask());
    }
    set_load_weight(addr_of_mut!(init_task), false);
    init_task.se.slice = sysctl_sched_base_slice as _;
    lupos_core_mmgrab_lazy_tlb(addr_of_mut!(init_mm));
    let current = lupos_core_current();
    lupos_core_enter_lazy_tlb(addr_of_mut!(init_mm), current);
    lupos_core_warn_site_9126(!set_kthread_struct(current));
    __sched_fork(0, current);
    init_idle(current, lupos_core_smp_processor_id());
    calc_load_update = lupos_core_jiffies().wrapping_add(LUPOS_CORE_LOAD_FREQ as c_ulong);
    idle_thread_set_boot_cpu();
    balance_push_set(lupos_core_smp_processor_id(), false);
    init_sched_fair_class();
    lupos_core_header_init_sched_ext_class();
    lupos_core_header_psi_init();
    init_uclamp();
    preempt_dynamic_init();
    scheduler_running = 1;
}
