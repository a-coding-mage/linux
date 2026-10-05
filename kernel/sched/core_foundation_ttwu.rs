// SPDX-License-Identifier: GPL-2.0-only
unsafe fn ttwu_stat(p: *mut task_struct, cpu: c_int, flags: c_int) {
    #[cfg(CONFIG_SCHEDSTATS)]
    {
        if !lupos_core_schedstat_enabled() {
            return;
        }
        let rq = lupos_core_this_rq();
        if cpu == (*rq).cpu {
            (*rq).ttwu_local = (*rq).ttwu_local.wrapping_add(1);
            (*p).stats.nr_wakeups_local = (*p).stats.nr_wakeups_local.wrapping_add(1);
        } else {
            (*p).stats.nr_wakeups_remote = (*p).stats.nr_wakeups_remote.wrapping_add(1);
            lupos_core_rcu_read_lock();
            let mut sd = lupos_core_sched_domain_first((*rq).cpu);
            while !sd.is_null() {
                if lupos_core_cpumask_test_cpu(cpu, lupos_core_sched_domain_span(sd)) {
                    (*sd).ttwu_wake_remote = (*sd).ttwu_wake_remote.wrapping_add(1);
                    break;
                }
                sd = lupos_core_sched_domain_parent(sd);
            }
            lupos_core_rcu_read_unlock();
        }
        if flags & LUPOS_CORE_WF_MIGRATED != 0 {
            (*p).stats.nr_wakeups_migrate = (*p).stats.nr_wakeups_migrate.wrapping_add(1);
        }
        (*rq).ttwu_count = (*rq).ttwu_count.wrapping_add(1);
        (*p).stats.nr_wakeups = (*p).stats.nr_wakeups.wrapping_add(1);
        if flags & LUPOS_CORE_WF_SYNC != 0 {
            (*p).stats.nr_wakeups_sync = (*p).stats.nr_wakeups_sync.wrapping_add(1);
        }
    }
}
unsafe fn ttwu_do_wakeup(p: *mut task_struct) {
    (*p).is_blocked = 0;
    lupos_core_write_once_uint(addr_of_mut!((*p).__state), LUPOS_CORE_TASK_RUNNING);
    lupos_core_trace_wakeup(p);
}
#[no_mangle]
pub unsafe extern "C" fn update_rq_avg_idle(rq: *mut rq) {
    let delta = lupos_core_rq_clock(rq).wrapping_sub((*rq).idle_stamp);
    let max = (*rq).max_idle_balance_cost.wrapping_mul(2);
    lupos_core_update_avg(addr_of_mut!((*rq).avg_idle), delta);
    if (*rq).avg_idle > max {
        (*rq).avg_idle = max;
    }
    (*rq).idle_stamp = 0;
}
#[cfg(CONFIG_SCHED_PROXY_EXEC)]
unsafe fn proxy_reset_donor(rq: *mut rq) {
    let curr = lupos_core_rq_curr(rq);
    lupos_core_warn_proxy_donor(lupos_core_rq_donor(rq) == curr);
    lupos_core_put_prev_set_next_task(rq, lupos_core_rq_donor(rq), curr);
    lupos_core_rq_set_donor(rq, curr);
    zap_balance_callbacks(rq);
    resched_curr(rq);
}
unsafe fn proxy_needs_return(rq: *mut rq, p: *mut task_struct) -> bool {
    #[cfg(CONFIG_SCHED_PROXY_EXEC)]
    {
        if lupos_core_task_cpu(p) == (*p).wake_cpu {
            return false;
        }
        lupos_core_raw_spin_lock_nested(addr_of_mut!((*p).blocked_lock), 0);
        lupos_core_clear_task_blocked_on_locked(p);
        if lupos_core_task_current(rq, p) {
            lupos_core_raw_spin_unlock(addr_of_mut!((*p).blocked_lock));
            return false;
        }
        if lupos_core_task_current_donor(rq, p) {
            proxy_reset_donor(rq);
        }
        lupos_core_raw_spin_unlock(addr_of_mut!((*p).blocked_lock));
        block_task(rq, p, LUPOS_CORE_TASK_WAKING as c_ulong);
        true
    }
    #[cfg(not(CONFIG_SCHED_PROXY_EXEC))]
    {
        false
    }
}
unsafe fn ttwu_do_activate(rq: *mut rq, p: *mut task_struct, wake_flags: c_int, rf: *mut rq_flags) {
    let mut en_flags = LUPOS_CORE_ENQUEUE_WAKEUP | LUPOS_CORE_ENQUEUE_NOCLOCK;
    lupos_core_assert_rq_held(rq);
    if lupos_core_task_contributes_to_load(p) != 0 {
        (*rq).nr_uninterruptible = (*rq).nr_uninterruptible.wrapping_sub(1);
    }
    if wake_flags & LUPOS_CORE_WF_RQ_SELECTED != 0 {
        en_flags |= LUPOS_CORE_ENQUEUE_RQ_SELECTED;
    }
    if wake_flags & LUPOS_CORE_WF_MIGRATED != 0 {
        en_flags |= LUPOS_CORE_ENQUEUE_MIGRATED;
    } else if lupos_core_task_in_iowait(p) != 0 {
        lupos_core_delayacct_blkio_end(p);
        lupos_core_atomic_dec(addr_of_mut!((*lupos_core_task_rq(p)).nr_iowait));
    }
    activate_task(rq, p, en_flags);
    wakeup_preempt(rq, p, wake_flags);
    ttwu_do_wakeup(p);
    if let Some(task_woken) = (*(*p).sched_class).task_woken {
        lupos_core_rq_unpin_lock(rq, rf);
        task_woken(rq, p);
        lupos_core_rq_repin_lock(rq, rf);
    }
}
unsafe fn ttwu_runnable(p: *mut task_struct, wake_flags: c_int) -> c_int {
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    let rq = ___task_rq_lock(p, rf.as_mut_ptr());
    let ret = 'locked: {
        if !lupos_core_task_on_rq_queued(p) {
            break 'locked 0;
        }
        update_rq_clock(rq);
        if (*p).is_blocked != 0 {
            if (*p).se.sched_delayed != 0 {
                enqueue_task(
                    rq,
                    p,
                    LUPOS_CORE_ENQUEUE_NOCLOCK | LUPOS_CORE_ENQUEUE_DELAYED,
                );
            }
            if proxy_needs_return(rq, p) {
                break 'locked 0;
            }
        }
        if !lupos_core_task_on_cpu(rq, p) {
            wakeup_preempt(rq, p, wake_flags);
        }
        ttwu_do_wakeup(p);
        1
    };
    lupos_core_task_rq_unlock_only(rq, p, rf.as_mut_ptr());
    ret
}
#[no_mangle]
pub unsafe extern "C" fn sched_ttwu_pending(arg: *mut c_void) {
    let mut node = arg.cast::<llist_node>();
    if node.is_null() {
        return;
    }
    let rq = lupos_core_this_rq();
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    lupos_core_rq_lock_irqsave(rq, rf.as_mut_ptr());
    update_rq_clock(rq);
    while !node.is_null() {
        let p = lupos_core_wake_entry_task(node);
        node = (*node).next;
        if lupos_core_warn_pending_on_cpu((*p).on_cpu != 0) {
            lupos_core_cond_load_acquire_on_cpu(p);
        }
        if lupos_core_warn_pending_cpu(lupos_core_task_cpu(p) != lupos_core_cpu_of(rq)) {
            set_task_cpu(p, lupos_core_cpu_of(rq) as c_uint);
        }
        let flags = if lupos_core_task_remote_wakeup(p) != 0 {
            LUPOS_CORE_WF_MIGRATED
        } else {
            0
        };
        ttwu_do_activate(rq, p, flags, rf.as_mut_ptr());
    }
    lupos_core_write_once_uint(addr_of_mut!((*rq).ttwu_pending), 0);
    lupos_core_rq_unlock_irqrestore(rq, rf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn call_function_single_prep_ipi(cpu: c_int) -> bool {
    if set_nr_if_polling((*lupos_core_cpu_rq(cpu)).idle) {
        lupos_core_trace_wake_idle_without_ipi(cpu);
        return false;
    }
    true
}
unsafe fn __ttwu_queue_wakelist(p: *mut task_struct, cpu: c_int, flags: c_int) {
    let rq = lupos_core_cpu_rq(cpu);
    lupos_core_set_task_remote_wakeup(p, (flags & LUPOS_CORE_WF_MIGRATED != 0) as c_uint);
    lupos_core_write_once_uint(addr_of_mut!((*rq).ttwu_pending), 1);
    #[cfg(CONFIG_SMP)]
    __smp_call_single_queue(cpu, addr_of_mut!((*p).wake_entry.llist));
}
#[no_mangle]
pub unsafe extern "C" fn wake_up_if_idle(cpu: c_int) {
    let rq = lupos_core_cpu_rq(cpu);
    lupos_core_rcu_read_lock();
    if lupos_core_is_idle_task(lupos_core_rq_curr_rcu(rq)) {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        lupos_core_rq_lock_irqsave(rq, rf.as_mut_ptr());
        if lupos_core_is_idle_task(lupos_core_rq_curr(rq)) {
            resched_curr(rq);
        }
        lupos_core_rq_unlock_irqrestore(rq, rf.as_mut_ptr());
    }
    lupos_core_rcu_read_unlock();
}
#[no_mangle]
pub unsafe extern "C" fn cpus_equal_capacity(a: c_int, b: c_int) -> bool {
    !lupos_core_sched_asym_cpucap_active()
        || a == b
        || lupos_core_arch_scale_cpu_capacity(a) == lupos_core_arch_scale_cpu_capacity(b)
}
#[no_mangle]
pub unsafe extern "C" fn cpus_share_cache(a: c_int, b: c_int) -> bool {
    a == b || lupos_core_sd_llc_id(a) == lupos_core_sd_llc_id(b)
}
#[no_mangle]
pub unsafe extern "C" fn cpus_share_resources(a: c_int, b: c_int) -> bool {
    a == b || lupos_core_sd_share_id(a) == lupos_core_sd_share_id(b)
}
#[no_mangle]
pub unsafe extern "C" fn task_llc(p: *const task_struct) -> c_int {
    lupos_core_sd_llc_id(lupos_core_task_cpu(p))
}
unsafe fn ttwu_queue_cond(p: *mut task_struct, cpu: c_int) -> bool {
    let this_cpu = lupos_core_smp_processor_id();
    if !lupos_core_scx_allow_ttwu_queue(p) {
        return false;
    }
    #[cfg(CONFIG_SMP)]
    if (*p).sched_class == addr_of!(stop_sched_class) {
        return false;
    }
    if !lupos_core_cpu_active(cpu) || !lupos_core_cpumask_test_cpu(cpu, (*p).cpus_ptr) {
        return false;
    }
    if !cpus_share_cache(this_cpu, cpu) {
        return true;
    }
    if cpu == this_cpu {
        return false;
    }
    (*lupos_core_cpu_rq(cpu)).nr_running == 0
}
unsafe fn ttwu_queue_wakelist(p: *mut task_struct, cpu: c_int, flags: c_int) -> bool {
    if lupos_core_feat_ttwu_queue() && ttwu_queue_cond(p, cpu) {
        sched_clock_cpu(cpu);
        __ttwu_queue_wakelist(p, cpu, flags);
        return true;
    }
    false
}
unsafe fn ttwu_queue(p: *mut task_struct, cpu: c_int, flags: c_int) {
    let rq = lupos_core_cpu_rq(cpu);
    if ttwu_queue_wakelist(p, cpu, flags) {
        return;
    }
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    lupos_core_rq_lock(rq, rf.as_mut_ptr());
    update_rq_clock(rq);
    ttwu_do_activate(rq, p, flags, rf.as_mut_ptr());
    lupos_core_rq_unlock(rq, rf.as_mut_ptr());
}
#[inline(always)]
unsafe fn ttwu_state_match(p: *mut task_struct, state: c_uint, success: *mut c_int) -> bool {
    #[cfg(CONFIG_DEBUG_PREEMPT)]
    lupos_core_warn_rtlock_state(
        state & LUPOS_CORE_TASK_RTLOCK_WAIT != 0 && state != LUPOS_CORE_TASK_RTLOCK_WAIT,
    );
    let matched = __task_state_match(p, state);
    *success = (matched != 0) as c_int;
    if matched < 0 {
        (*p).saved_state = LUPOS_CORE_TASK_RUNNING;
    }
    matched > 0
}
#[no_mangle]
pub unsafe extern "C" fn try_to_wake_up(
    p: *mut task_struct,
    state: c_uint,
    mut flags: c_int,
) -> c_int {
    lupos_core_preempt_disable();
    let mut success = 0;
    flags |= LUPOS_CORE_WF_TTWU;
    if p == lupos_core_current() {
        lupos_core_warn_current_delayed((*p).se.sched_delayed != 0);
        lupos_core_warn_current_blocked((*p).is_blocked != 0);
        lupos_core_clear_task_blocked_on(p);
        if ttwu_state_match(p, state, addr_of_mut!(success)) {
            lupos_core_trace_waking(p);
            ttwu_do_wakeup(p);
        }
    } else {
        let irq_flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
        'locked: {
            lupos_core_smp_mb_after_spinlock();
            if !ttwu_state_match(p, state, addr_of_mut!(success)) {
                break 'locked;
            }
            lupos_core_trace_waking(p);
            lupos_core_smp_rmb();
            if lupos_core_read_once_u8(addr_of!((*p).on_rq)) != 0 && ttwu_runnable(p, flags) != 0 {
                break 'locked;
            }
            lupos_core_smp_acquire_after_ctrl_dep();
            lupos_core_write_once_uint(addr_of_mut!((*p).__state), LUPOS_CORE_TASK_WAKING);
            if lupos_core_load_acquire_on_cpu(p) != 0
                && ttwu_queue_wakelist(p, lupos_core_task_cpu(p), flags)
            {
                break 'locked;
            }
            lupos_core_cond_load_acquire_on_cpu(p);
            let cpu = select_task_rq(p, (*p).wake_cpu, addr_of_mut!(flags));
            if lupos_core_task_cpu(p) != cpu {
                if lupos_core_task_in_iowait(p) != 0 {
                    lupos_core_delayacct_blkio_end(p);
                    lupos_core_atomic_dec(addr_of_mut!((*lupos_core_task_rq(p)).nr_iowait));
                }
                flags |= LUPOS_CORE_WF_MIGRATED;
                lupos_core_psi_ttwu_dequeue(p);
                set_task_cpu(p, cpu as c_uint);
            } else if cpu != (*p).wake_cpu {
                (*p).wake_cpu = cpu;
            }
            ttwu_queue(p, cpu, flags);
        }
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), irq_flags);
    }
    if success != 0 {
        ttwu_stat(p, lupos_core_task_cpu(p), flags);
    }
    lupos_core_preempt_enable();
    success
}
unsafe fn __task_needs_rq_lock(p: *mut task_struct) -> bool {
    let state = lupos_core_read_once_uint(addr_of!((*p).__state));
    if state == LUPOS_CORE_TASK_RUNNING || state == LUPOS_CORE_TASK_WAKING {
        return true;
    }
    lupos_core_smp_rmb();
    if (*p).on_rq != 0 {
        return true;
    }
    lupos_core_smp_rmb();
    lupos_core_cond_load_acquire_on_cpu(p);
    false
}
#[no_mangle]
pub unsafe extern "C" fn task_call_func(
    p: *mut task_struct,
    func: task_call_f,
    arg: *mut c_void,
) -> c_int {
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    (*rf.as_mut_ptr()).flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
    let ret;
    if __task_needs_rq_lock(p) {
        let rq = ___task_rq_lock(p, rf.as_mut_ptr());
        ret = func.unwrap_unchecked()(p, arg);
        lupos_core_task_rq_unlock_only(rq, p, rf.as_mut_ptr());
    } else {
        ret = func.unwrap_unchecked()(p, arg);
    }
    lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), (*rf.as_ptr()).flags);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn cpu_curr_snapshot(cpu: c_int) -> *mut task_struct {
    let rq = lupos_core_cpu_rq(cpu);
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    lupos_core_rq_lock_irqsave(rq, rf.as_mut_ptr());
    lupos_core_smp_mb_after_spinlock();
    let task = lupos_core_rq_curr_rcu(rq);
    lupos_core_rq_unlock_irqrestore(rq, rf.as_mut_ptr());
    lupos_core_smp_mb();
    task
}
#[no_mangle]
pub unsafe extern "C" fn wake_up_process(p: *mut task_struct) -> c_int {
    try_to_wake_up(p, LUPOS_CORE_TASK_NORMAL, 0)
}
#[no_mangle]
pub unsafe extern "C" fn wake_up_state(p: *mut task_struct, state: c_uint) -> c_int {
    try_to_wake_up(p, state, 0)
}
