// SPDX-License-Identifier: GPL-2.0-only
// core.c:5532..5957. Per-CPU storage/exports live in the native declaration unit.
#[no_mangle]
pub unsafe extern "C" fn nr_running() -> c_uint {
    let mut sum: c_uint = 0;
    let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_online_mask());
    while cpu < nr_cpu_ids as c_int {
        sum = sum.wrapping_add((*lupos_core_cpu_rq(cpu)).nr_running);
        cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_online_mask());
    }
    sum
}
#[no_mangle]
pub unsafe extern "C" fn single_task_running() -> bool {
    (*lupos_core_raw_rq()).nr_running == 1
}
#[no_mangle]
pub unsafe extern "C" fn nr_context_switches_cpu(cpu: c_int) -> u64 {
    (*lupos_core_cpu_rq(cpu)).nr_switches
}
#[no_mangle]
pub unsafe extern "C" fn nr_context_switches() -> u64 {
    let mut sum = 0u64;
    let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
    while cpu < nr_cpu_ids as c_int {
        sum = sum.wrapping_add((*lupos_core_cpu_rq(cpu)).nr_switches);
        cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
    }
    sum
}
#[no_mangle]
pub unsafe extern "C" fn nr_iowait_cpu(cpu: c_int) -> c_uint {
    lupos_core_atomic_read(addr_of!((*lupos_core_cpu_rq(cpu)).nr_iowait)) as c_uint
}
#[no_mangle]
pub unsafe extern "C" fn nr_iowait() -> c_uint {
    let mut sum: c_uint = 0;
    let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
    while cpu < nr_cpu_ids as c_int {
        sum = sum.wrapping_add(nr_iowait_cpu(cpu));
        cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
    }
    sum
}
#[no_mangle]
pub unsafe extern "C" fn sched_exec() {
    let p = lupos_core_current();
    let mut arg = MaybeUninit::<migration_arg>::zeroed();
    let arg = arg.as_mut_ptr();
    let flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
    let cpu = (*(*p).sched_class).select_task_rq.unwrap()(
        p,
        lupos_core_task_cpu(p),
        LUPOS_CORE_WF_EXEC as c_int,
    );
    if cpu == lupos_core_smp_processor_id() || !lupos_core_cpu_active(cpu) {
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), flags);
        return;
    }
    (*arg).task = p;
    (*arg).dest_cpu = cpu;
    lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), flags);
    lupos_core_header_stop_one_cpu(
        lupos_core_task_cpu(p) as c_uint,
        Some(migration_cpu_stop),
        arg.cast(),
    );
}
#[inline(always)]
unsafe fn prefetch_curr_exec_start(p: *mut task_struct) {
    let curr = (*lupos_core_task_rq(p)).cfs.curr;
    lupos_core_prefetch(curr.cast());
    lupos_core_prefetch(addr_of!((*curr).exec_start).cast());
}
#[no_mangle]
pub unsafe extern "C" fn task_sched_runtime(p: *mut task_struct) -> u64 {
    #[cfg(CONFIG_64BIT)]
    if (*p).on_cpu == 0 || !lupos_core_task_on_rq_queued(p) {
        return (*p).se.sum_exec_runtime;
    }
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    let rf = rf.as_mut_ptr();
    let rq = _task_rq_lock(p, rf);
    if lupos_core_task_current_donor(rq, p) && lupos_core_task_on_rq_queued(p) {
        prefetch_curr_exec_start(p);
        update_rq_clock(rq);
        (*(*p).sched_class).update_curr.unwrap()(rq);
    }
    let ns = (*p).se.sum_exec_runtime;
    lupos_core_task_rq_unlock(rq, p, rf);
    ns
}
unsafe fn cpu_resched_latency(rq: *mut rq) -> u64 {
    // Storage retains original function-static identity in native declaration unit.
    let latency_warn_ms = lupos_core_read_once_int(addr_of!(sysctl_resched_latency_warn_ms));
    let now = lupos_core_rq_clock(rq);
    if sysctl_resched_latency_warn_once != 0 && *lupos_core_resched_warned_once() {
        return 0;
    }
    if !lupos_core_need_resched()
        || latency_warn_ms == 0
        || system_state == LUPOS_CORE_SYSTEM_BOOTING
    {
        return 0;
    }
    if (*rq).last_seen_need_resched_ns == 0 {
        (*rq).last_seen_need_resched_ns = now;
        (*rq).ticks_without_resched = 0;
        return 0;
    }
    (*rq).ticks_without_resched = (*rq).ticks_without_resched.wrapping_add(1);
    let latency = now.wrapping_sub((*rq).last_seen_need_resched_ns);
    if latency
        <= (latency_warn_ms as c_long).wrapping_mul(LUPOS_CORE_NSEC_PER_MSEC as c_long) as u64
    {
        return 0;
    }
    *lupos_core_resched_warned_once() = true;
    latency
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_resched_latency_warn_ms(s: *mut c_char) -> c_int {
    let mut val: c_long = 0;
    if lupos_core_header_kstrtol(s, 0, &mut val) != 0 {
        lupos_core_pr_warn(b"Unable to set resched_latency_warn_ms\n\0".as_ptr().cast());
        return 1;
    }
    sysctl_resched_latency_warn_ms = val as c_int;
    1
}
#[no_mangle]
pub unsafe extern "C" fn sched_tick() {
    let cpu = lupos_core_smp_processor_id();
    let rq = lupos_core_cpu_rq(cpu);
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    let rf = rf.as_mut_ptr();
    if lupos_core_housekeeping_cpu(cpu, LUPOS_CORE_HK_TYPE_KERNEL_NOISE) {
        lupos_core_arch_scale_freq_tick();
    }
    lupos_core_header_sched_clock_tick();
    lupos_core_rq_lock(rq, rf);
    let donor = lupos_core_rq_donor(rq);
    lupos_core_header_psi_account_irqtime(rq, donor, null_mut());
    update_rq_clock(rq);
    let pressure = lupos_core_arch_scale_hw_pressure(lupos_core_cpu_of(rq));
    lupos_core_header_update_hw_load_avg(lupos_core_rq_clock_task(rq), rq, pressure);
    if dynamic_preempt_lazy() && lupos_core_tif_test_bit(LUPOS_CORE_TIF_NEED_RESCHED_LAZY)
    {
        resched_curr(rq);
    }
    (*(*donor).sched_class).task_tick.unwrap()(rq, donor, 0);
    let latency = if lupos_core_sched_feat_latency_warn() {
        cpu_resched_latency(rq)
    } else {
        0
    };
    calc_global_load_tick(rq);
    lupos_core_header_sched_core_tick(rq);
    lupos_core_header_scx_tick(rq);
    lupos_core_rq_unlock(rq, rf);
    if lupos_core_sched_feat_latency_warn() && latency != 0 {
        resched_latency_warn(cpu, latency);
    }
    lupos_core_header_perf_event_task_tick();
    if (*donor).flags & LUPOS_CORE_PF_WQ_WORKER != 0 {
        wq_worker_tick(donor);
    }
    if !lupos_core_header_scx_switched_all() {
        (*rq).idle_balance = idle_cpu(cpu);
        sched_balance_trigger(rq);
    }
}
#[cfg(CONFIG_NO_HZ_FULL)]
unsafe extern "C" fn sched_tick_remote(work: *mut work_struct) {
    let dwork = lupos_core_to_delayed_work(work);
    let twork = dwork
        .cast::<u8>()
        .sub(offset_of!(tick_work, work))
        .cast::<tick_work>();
    let cpu = (*twork).cpu;
    let rq = lupos_core_cpu_rq(cpu);
    if lupos_core_header_tick_nohz_tick_stopped_cpu(cpu) != 0 {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        lupos_core_rq_lock_irq(rq, rf);
        let curr = lupos_core_rq_curr(rq);
        if lupos_core_cpu_online(cpu) {
            lupos_core_warn_once_site_5876(curr != lupos_core_rq_donor(rq));
            update_rq_clock(rq);
            if !lupos_core_is_idle_task(curr) {
                let delta = lupos_core_rq_clock_task(rq).wrapping_sub((*curr).se.exec_start);
                lupos_core_warn_once_site_5885(
                    delta > (LUPOS_CORE_NSEC_PER_SEC as u64).wrapping_mul(30),
                );
            }
            (*(*curr).sched_class).task_tick.unwrap()(rq, curr, 0);
            lupos_core_header_calc_load_nohz_remote(rq);
        }
        lupos_core_rq_unlock_irq(rq, rf);
    }
    let old = lupos_core_atomic_fetch_add_unless(
        addr_of_mut!((*twork).state),
        -1,
        LUPOS_CORE_TICK_SCHED_REMOTE_RUNNING,
    );
    lupos_core_warn_once_site_5905(old == LUPOS_CORE_TICK_SCHED_REMOTE_OFFLINE);
    if old == LUPOS_CORE_TICK_SCHED_REMOTE_RUNNING {
        queue_delayed_work_on(
            LUPOS_CORE_WORK_CPU_UNBOUND as c_int,
            system_dfl_wq,
            dwork,
            LUPOS_CORE_HZ as c_ulong,
        );
    }
}
#[cfg(CONFIG_NO_HZ_FULL)]
unsafe fn sched_tick_start(cpu: c_int) {
    if lupos_core_housekeeping_cpu(cpu, LUPOS_CORE_HK_TYPE_KERNEL_NOISE) {
        return;
    }
    lupos_core_warn_once_site_5918(lupos_core_tick_work_base_is_null());
    let twork = lupos_core_tick_work_cpu(cpu);
    let old = lupos_core_atomic_xchg(
        addr_of_mut!((*twork).state),
        LUPOS_CORE_TICK_SCHED_REMOTE_RUNNING,
    );
    lupos_core_warn_once_site_5922(old == LUPOS_CORE_TICK_SCHED_REMOTE_RUNNING);
    if old == LUPOS_CORE_TICK_SCHED_REMOTE_OFFLINE {
        (*twork).cpu = cpu;
        lupos_core_init_delayed_work(addr_of_mut!((*twork).work), Some(sched_tick_remote));
        queue_delayed_work_on(
            LUPOS_CORE_WORK_CPU_UNBOUND as c_int,
            system_dfl_wq,
            addr_of_mut!((*twork).work),
            LUPOS_CORE_HZ as c_ulong,
        );
    }
}
#[cfg(not(CONFIG_NO_HZ_FULL))]
unsafe fn sched_tick_start(_cpu: c_int) {}
#[cfg(all(CONFIG_NO_HZ_FULL, CONFIG_HOTPLUG_CPU))]
unsafe fn sched_tick_stop(cpu: c_int) {
    if lupos_core_housekeeping_cpu(cpu, LUPOS_CORE_HK_TYPE_KERNEL_NOISE) {
        return;
    }
    lupos_core_warn_once_site_5938(lupos_core_tick_work_base_is_null());
    let twork = lupos_core_tick_work_cpu(cpu);
    let old = lupos_core_atomic_xchg(
        addr_of_mut!((*twork).state),
        LUPOS_CORE_TICK_SCHED_REMOTE_OFFLINING,
    );
    lupos_core_warn_once_site_5943(old != LUPOS_CORE_TICK_SCHED_REMOTE_RUNNING);
}
#[cfg(not(CONFIG_NO_HZ_FULL))]
unsafe fn sched_tick_stop(_cpu: c_int) {}
#[cfg(CONFIG_NO_HZ_FULL)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_tick_offload_init() -> c_int {
    lupos_core_alloc_tick_work_percpu();
    lupos_core_bug_on_site_5951(lupos_core_tick_work_base_is_null());
    0
}
