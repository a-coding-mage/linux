// SPDX-License-Identifier: GPL-2.0-only
unsafe fn hrtick_clear(rq: *mut rq) {
    #[cfg(CONFIG_SCHED_HRTICK)]
    if lupos_core_hrtimer_active(addr_of!((*rq).hrtick_timer)) {
        hrtimer_cancel(addr_of_mut!((*rq).hrtick_timer));
    }
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe extern "C" fn hrtick(timer: *mut hrtimer) -> hrtimer_restart {
    let rq = timer
        .cast::<u8>()
        .sub(offset_of!(rq, hrtick_timer))
        .cast::<rq>();
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    lupos_core_warn_hrtick_cpu(lupos_core_cpu_of(rq) != lupos_core_smp_processor_id());
    lupos_core_rq_lock(rq, rf.as_mut_ptr());
    update_rq_clock(rq);
    let donor = lupos_core_rq_donor(rq);
    ((*(*donor).sched_class).task_tick.unwrap_unchecked())(rq, donor, 1);
    lupos_core_rq_unlock(rq, rf.as_mut_ptr());
    LUPOS_CORE_HRTIMER_NORESTART
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe fn hrtick_needs_rearm(timer: *mut hrtimer, expires: ktime_t) -> bool {
    !lupos_core_hrtimer_is_queued(timer)
        || expires
            .wrapping_sub(lupos_core_hrtimer_get_expires(timer))
            .wrapping_abs()
            > 5000
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe fn hrtick_cond_restart(rq: *mut rq) {
    let timer = addr_of_mut!((*rq).hrtick_timer);
    let time = (*rq).hrtick_time;
    if hrtick_needs_rearm(timer, time) {
        lupos_core_hrtimer_start(timer, time, LUPOS_CORE_HRTIMER_MODE_ABS_PINNED_HARD);
    }
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe extern "C" fn __hrtick_start(arg: *mut c_void) {
    let rq = arg.cast::<rq>();
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    lupos_core_rq_lock(rq, rf.as_mut_ptr());
    hrtick_cond_restart(rq);
    lupos_core_rq_unlock(rq, rf.as_mut_ptr());
}
#[cfg(CONFIG_SCHED_HRTICK)]
#[no_mangle]
pub unsafe extern "C" fn hrtick_start(rq: *mut rq, delay: u64) {
    let delta = core::cmp::max(delay as i64, 10000);
    if (*rq).hrtick_sched != 0 {
        (*rq).hrtick_sched |= LUPOS_CORE_HRTICK_SCHED_START;
        (*rq).hrtick_delay = delta;
        return;
    }
    (*rq).hrtick_time = lupos_core_ktime_add_ns(ktime_get(), delta as u64);
    if !hrtick_needs_rearm(addr_of_mut!((*rq).hrtick_timer), (*rq).hrtick_time) {
        return;
    }
    if rq == lupos_core_this_rq() {
        lupos_core_hrtimer_start(
            addr_of_mut!((*rq).hrtick_timer),
            (*rq).hrtick_time,
            LUPOS_CORE_HRTIMER_MODE_ABS_PINNED_HARD,
        );
    } else {
        smp_call_function_single_async(lupos_core_cpu_of(rq), addr_of_mut!((*rq).hrtick_csd));
    }
}
unsafe fn hrtick_schedule_enter(rq: *mut rq) {
    #[cfg(CONFIG_SCHED_HRTICK)]
    {
        (*rq).hrtick_sched = LUPOS_CORE_HRTICK_SCHED_DEFER;
        if lupos_core_hrtimer_test_and_clear_rearm_deferred() {
            (*rq).hrtick_sched |= LUPOS_CORE_HRTICK_SCHED_REARM_HRTIMER;
        }
    }
}
unsafe fn hrtick_schedule_exit(rq: *mut rq) {
    #[cfg(CONFIG_SCHED_HRTICK)]
    {
        if (*rq).hrtick_sched & LUPOS_CORE_HRTICK_SCHED_START != 0 {
            (*rq).hrtick_time = lupos_core_ktime_add_ns(ktime_get(), (*rq).hrtick_delay as u64);
            hrtick_cond_restart(rq);
        } else if lupos_core_idle_rq(rq)
            && lupos_core_hrtimer_is_queued(addr_of!((*rq).hrtick_timer))
        {
            hrtimer_cancel(addr_of_mut!((*rq).hrtick_timer));
        }
        if (*rq).hrtick_sched & LUPOS_CORE_HRTICK_SCHED_REARM_HRTIMER != 0 {
            lupos_core_header___hrtimer_rearm_deferred();
        }
        (*rq).hrtick_sched = LUPOS_CORE_HRTICK_SCHED_NONE;
    }
}
unsafe fn hrtick_rq_init(rq: *mut rq) {
    #[cfg(CONFIG_SCHED_HRTICK)]
    {
        lupos_core_init_csd(
            addr_of_mut!((*rq).hrtick_csd),
            Some(__hrtick_start),
            rq.cast(),
        );
        (*rq).hrtick_sched = LUPOS_CORE_HRTICK_SCHED_NONE;
        lupos_core_hrtimer_setup(
            addr_of_mut!((*rq).hrtick_timer),
            Some(hrtick),
            LUPOS_CORE_CLOCK_MONOTONIC,
            LUPOS_CORE_HRTIMER_MODE_REL_HARD | LUPOS_CORE_HRTIMER_MODE_LAZY_REARM,
        );
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
#[no_mangle]
pub unsafe extern "C" fn get_nohz_timer_target() -> c_int {
    let cpu = lupos_core_smp_processor_id();
    let mut default_cpu = -1;
    if lupos_core_housekeeping_cpu(cpu, LUPOS_CORE_HK_TYPE_KERNEL_NOISE) {
        if idle_cpu(cpu) == 0 {
            return cpu;
        }
        default_cpu = cpu;
    }
    let hk = lupos_core_housekeeping_cpumask(LUPOS_CORE_HK_TYPE_KERNEL_NOISE);
    lupos_core_rcu_read_lock();
    let mut sd = lupos_core_sched_domain_first(cpu);
    while !sd.is_null() {
        let mask = lupos_core_sched_domain_span(sd);
        let mut i = lupos_core_cpu_next(-1, mask);
        while i < lupos_core_nr_cpu_ids() {
            if i != cpu && lupos_core_cpumask_test_cpu(i, hk) && idle_cpu(i) == 0 {
                lupos_core_rcu_read_unlock();
                return i;
            }
            i = lupos_core_cpu_next(i, mask);
        }
        sd = lupos_core_sched_domain_parent(sd);
    }
    if default_cpu == -1 {
        default_cpu = lupos_core_housekeeping_any_cpu(LUPOS_CORE_HK_TYPE_KERNEL_NOISE);
    }
    lupos_core_rcu_read_unlock();
    default_cpu
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn wake_up_idle_cpu(cpu: c_int) {
    let rq = lupos_core_cpu_rq(cpu);
    if cpu == lupos_core_smp_processor_id() {
        return;
    }
    if set_nr_and_not_polling(
        lupos_core_task_thread_info((*rq).idle),
        LUPOS_CORE_TIF_NEED_RESCHED,
    ) {
        lupos_core_smp_send_reschedule(cpu);
    } else {
        lupos_core_trace_wake_idle_without_ipi(cpu);
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn wake_up_full_nohz_cpu(cpu: c_int) -> bool {
    if lupos_core_cpu_is_offline(cpu) {
        return true;
    }
    if lupos_core_tick_nohz_full_cpu(cpu) {
        if cpu != lupos_core_smp_processor_id() || lupos_core_tick_nohz_tick_stopped() {
            lupos_core_tick_nohz_full_kick_cpu(cpu);
        }
        return true;
    }
    false
}
#[cfg(CONFIG_NO_HZ_COMMON)]
#[no_mangle]
pub unsafe extern "C" fn wake_up_nohz_cpu(cpu: c_int) {
    if !wake_up_full_nohz_cpu(cpu) {
        wake_up_idle_cpu(cpu);
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe extern "C" fn nohz_csd_func(info: *mut c_void) {
    let rq = info.cast::<rq>();
    let cpu = lupos_core_cpu_of(rq);
    let flags = lupos_core_atomic_fetch_andnot(
        LUPOS_CORE_NOHZ_KICK_MASK | LUPOS_CORE_NOHZ_NEWILB_KICK,
        lupos_core_nohz_flags(cpu),
    ) as c_uint;
    lupos_core_warn_nohz_kick(flags & LUPOS_CORE_NOHZ_KICK_MASK as c_uint == 0);
    (*rq).idle_balance = idle_cpu(cpu) as u8;
    if (*rq).idle_balance != 0 {
        (*rq).nohz_idle_balance = flags as u8;
        lupos_core_raise_softirq_irqoff(LUPOS_CORE_SCHED_SOFTIRQ);
    }
}
#[cfg(CONFIG_NO_HZ_FULL)]
unsafe fn __need_bw_check(rq: *mut rq, p: *mut task_struct) -> bool {
    (*rq).nr_running == 1
        && (*p).sched_class == addr_of!(fair_sched_class)
        && lupos_core_task_on_rq_queued(p)
}
#[cfg(CONFIG_NO_HZ_FULL)]
#[no_mangle]
pub unsafe extern "C" fn sched_can_stop_tick(rq: *mut rq) -> bool {
    if (*rq).dl.dl_nr_running != 0 {
        return false;
    }
    if (*rq).rt.rr_nr_running != 0 {
        return (*rq).rt.rr_nr_running == 1;
    }
    let fifo = (*rq).rt.rt_nr_running.wrapping_sub((*rq).rt.rr_nr_running) as c_int;
    if fifo != 0 {
        return true;
    }
    if lupos_core_scx_enabled() && !lupos_core_scx_can_stop_tick(rq) {
        return false;
    }
    if (*rq).cfs.h_nr_queued > 1 {
        return false;
    }
    if __need_bw_check(rq, lupos_core_rq_curr(rq))
        && lupos_core_cfs_task_bw_constrained(lupos_core_rq_curr(rq))
    {
        return false;
    }
    true
}
