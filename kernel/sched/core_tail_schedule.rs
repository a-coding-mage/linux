// SPDX-License-Identifier: GPL-2.0-only
// core.c:7070..7578. __sched/notrace/visible/asmlinkage/nokprobe policy must be
// admitted function-by-function by the common recipe; section alone is not proof.
#[inline(never)]
#[link_section = ".sched.text"]
unsafe fn __schedule(sched_mode: c_int) {
    let mut preempt = sched_mode > LUPOS_CORE_SM_NONE;
    lupos_core_trace_sched_entry(sched_mode == LUPOS_CORE_SM_PREEMPT);
    let rq = lupos_core_cpu_rq(lupos_core_smp_processor_id());
    let prev = lupos_core_rq_curr(rq);
    schedule_debug(prev, preempt);
    lupos_core_header_klp_sched_try_switch(prev);
    lupos_core_local_irq_disable();
    lupos_core_rcu_note_context_switch(preempt);
    migrate_disable_switch(rq, prev);
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    let rf = rf.as_mut_ptr();
    lupos_core_rq_lock(rq, rf);
    lupos_core_smp_mb_after_spinlock();
    hrtick_schedule_enter(rq);
    (*rq).clock_update_flags = (*rq).clock_update_flags.wrapping_shl(1);
    update_rq_clock(rq);
    (*rq).clock_update_flags = LUPOS_CORE_RQCF_UPDATED;
    let mut switch_count = addr_of_mut!((*prev).nivcsw);
    preempt = sched_mode == LUPOS_CORE_SM_PREEMPT;
    // READ_ONCE must retain the control dependency into dequeue/block.
    let mut prev_state = lupos_core_read_once_uint(addr_of!((*prev).__state)) as c_ulong;
    let idle_fast =
        sched_mode == LUPOS_CORE_SM_IDLE && (*rq).nr_running == 0 && !lupos_core_scx_enabled();
    let mut keep_resched = false;
    let next;
    if idle_fast {
        next = prev;
        (*rq).next_class = addr_of!(idle_sched_class);
    } else {
        if sched_mode != LUPOS_CORE_SM_IDLE && !preempt && prev_state != 0 {
            try_to_block_task(rq, prev, &mut prev_state, !lupos_core_task_is_blocked(prev));
            switch_count = addr_of_mut!((*prev).nvcsw);
        }
        next = loop {
            lupos_core_header_assert_balance_callbacks_empty(rq);
            let mut picked = pick_next_task(rq, rf);
            (*rq).next_class = (*picked).sched_class;
            if lupos_core_sched_proxy_exec() {
                let prev_donor = lupos_core_rq_donor(rq);
                lupos_core_rq_set_donor(rq, picked);
                #[cfg(CONFIG_SCHED_PROXY_EXEC)]
                {
                    (*picked).blocked_donor = null_mut();
                }
                if lupos_core_task_is_blocked_flag(picked) {
                    picked = find_proxy_task(rq, picked, rf);
                    if picked.is_null() {
                        zap_balance_callbacks(rq);
                        continue;
                    }
                    if picked == (*rq).idle {
                        zap_balance_callbacks(rq);
                        keep_resched = true;
                        break picked;
                    }
                }
                if lupos_core_rq_donor(rq) == prev_donor && prev != picked {
                    let donor = lupos_core_rq_donor(rq);
                    (*(*donor).sched_class).put_prev_task.unwrap()(rq, donor, donor);
                    (*(*donor).sched_class).set_next_task.unwrap()(rq, donor, true);
                }
            } else {
                lupos_core_rq_set_donor(rq, picked);
            }
            break picked;
        };
    }
    if !keep_resched {
        lupos_core_clear_tsk_need_resched(prev);
        lupos_core_clear_preempt_need_resched();
    }
    (*rq).last_seen_need_resched_ns = 0;
    let is_switch = prev != next;
    if is_switch {
        (*rq).nr_switches = (*rq).nr_switches.wrapping_add(1);
        lupos_core_rcu_init_rq_curr(rq, next);
        *switch_count = (*switch_count).wrapping_add(1);
        lupos_core_header_psi_account_irqtime(rq, prev, next);
        lupos_core_header_psi_sched_switch(
            prev,
            next,
            !lupos_core_task_on_rq_queued(prev)
                || lupos_core_se_sched_delayed(addr_of!((*prev).se)),
        );
        lupos_core_trace_sched_switch(preempt, prev, next, prev_state);
        // context_switch owns lock release and architecture membarrier pairing.
        context_switch(rq, prev, next, rf);
    } else {
        lupos_core_rq_unpin_lock(rq, rf);
        __balance_callbacks(rq, null_mut());
        hrtick_schedule_exit(rq);
        lupos_core_raw_spin_rq_unlock_irq(rq);
    }
    lupos_core_trace_sched_exit(is_switch);
}
#[no_mangle]
pub unsafe extern "C" fn do_task_dead() -> ! {
    lupos_core_set_special_state(LUPOS_CORE_TASK_DEAD);
    (*lupos_core_current()).flags |= LUPOS_CORE_PF_NOFREEZE;
    __schedule(LUPOS_CORE_SM_NONE);
    lupos_core_bug_site_7289();
    loop {
        lupos_core_cpu_relax();
    }
}
#[inline(always)]
unsafe fn sched_submit_work(tsk: *mut task_struct) {
    lupos_core_lock_map_acquire_try(lupos_core_sched_wait_map());
    let flags = (*tsk).flags;
    if flags & LUPOS_CORE_PF_WQ_WORKER != 0 {
        wq_worker_sleeping(tsk);
    } else if flags & LUPOS_CORE_PF_IO_WORKER != 0 {
        io_wq_worker_sleeping(tsk);
    }
    lupos_core_warn_once_site_7333(
        (*lupos_core_current()).__state & LUPOS_CORE_TASK_RTLOCK_WAIT != 0,
    );
    lupos_core_header_blk_flush_plug((*tsk).plug, true);
    lupos_core_lock_map_release(lupos_core_sched_wait_map());
}
unsafe fn sched_update_worker(tsk: *mut task_struct) {
    if (*tsk).flags & (LUPOS_CORE_PF_WQ_WORKER | LUPOS_CORE_PF_IO_WORKER) != 0 {
        if (*tsk).flags & LUPOS_CORE_PF_WQ_WORKER != 0 {
            wq_worker_running(tsk);
        } else {
            io_wq_worker_running(tsk);
        }
    }
}
#[inline(always)]
unsafe fn __schedule_loop(mode: c_int) {
    loop {
        lupos_core_preempt_disable();
        __schedule(mode);
        lupos_core_sched_preempt_enable_no_resched();
        if !lupos_core_need_resched() {
            break;
        }
    }
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn schedule() {
    let tsk = lupos_core_current();
    #[cfg(CONFIG_RT_MUTEXES)]
    lupos_core_lockdep_assert_site_7370(!lupos_core_task_sched_rt_mutex(tsk));
    if !lupos_core_task_is_running(tsk) {
        sched_submit_work(tsk);
    }
    __schedule_loop(LUPOS_CORE_SM_NONE);
    sched_update_worker(tsk);
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn schedule_idle() {
    lupos_core_warn_once_site_7399((*lupos_core_current()).__state != 0);
    loop {
        __schedule(LUPOS_CORE_SM_IDLE);
        if !lupos_core_need_resched() {
            break;
        }
    }
}
#[cfg(all(
    CONFIG_CONTEXT_TRACKING_USER,
    not(CONFIG_HAVE_CONTEXT_TRACKING_USER_OFFSTACK)
))]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn schedule_user() {
    let state = lupos_core_exception_enter();
    schedule();
    lupos_core_exception_exit(state);
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn schedule_preempt_disabled() {
    lupos_core_sched_preempt_enable_no_resched();
    schedule();
    lupos_core_preempt_disable();
}
#[cfg(CONFIG_PREEMPT_RT)]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn schedule_rtlock() {
    __schedule_loop(LUPOS_CORE_SM_RTLOCK_WAIT);
}
#[link_section = ".sched.text"]
unsafe fn preempt_schedule_common() {
    loop {
        lupos_core_preempt_disable_notrace();
        preempt_latency_start(1);
        __schedule(LUPOS_CORE_SM_PREEMPT);
        preempt_latency_stop(1);
        lupos_core_preempt_enable_no_resched_notrace();
        if !lupos_core_need_resched() {
            break;
        }
    }
}
#[cfg(CONFIG_PREEMPTION)]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn preempt_schedule() {
    if !lupos_core_preemptible() {
        return;
    }
    preempt_schedule_common();
}
#[cfg(CONFIG_PREEMPTION)]
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn preempt_schedule_notrace() {
    if !lupos_core_preemptible() {
        return;
    }
    loop {
        lupos_core_preempt_disable_notrace();
        preempt_latency_start(1);
        let state = lupos_core_exception_enter();
        __schedule(LUPOS_CORE_SM_PREEMPT);
        lupos_core_exception_exit(state);
        preempt_latency_stop(1);
        lupos_core_preempt_enable_no_resched_notrace();
        if !lupos_core_need_resched() {
            break;
        }
    }
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn preempt_schedule_irq() {
    lupos_core_bug_on_site_7585(lupos_core_preempt_count() != 0 || !lupos_core_irqs_disabled());
    let state = lupos_core_exception_enter();
    loop {
        lupos_core_preempt_disable();
        lupos_core_local_irq_enable();
        __schedule(LUPOS_CORE_SM_PREEMPT);
        lupos_core_local_irq_disable();
        lupos_core_sched_preempt_enable_no_resched();
        if !lupos_core_need_resched() {
            break;
        }
    }
    lupos_core_exception_exit(state);
}
#[no_mangle]
pub unsafe extern "C" fn default_wake_function(
    curr: *mut wait_queue_entry_t,
    mode: c_uint,
    wake_flags: c_int,
    _key: *mut c_void,
) -> c_int {
    lupos_core_warn_once_site_7604(
        wake_flags & !(LUPOS_CORE_WF_SYNC | LUPOS_CORE_WF_CURRENT_CPU) as c_int != 0,
    );
    try_to_wake_up((*curr).private.cast(), mode, wake_flags)
}
#[no_mangle]
pub unsafe extern "C" fn __setscheduler_class(policy: c_int, prio: c_int) -> *const sched_class {
    if lupos_core_dl_prio(prio) {
        return addr_of!(dl_sched_class);
    }
    if lupos_core_rt_prio(prio) {
        return addr_of!(rt_sched_class);
    }
    #[cfg(CONFIG_SCHED_CLASS_EXT)]
    if lupos_core_task_should_scx(policy) {
        return addr_of!(ext_sched_class);
    }
    addr_of!(fair_sched_class)
}
#[cfg(CONFIG_RT_MUTEXES)]
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_pre_schedule() {
    let current = lupos_core_current();
    let old = lupos_core_task_sched_rt_mutex(current);
    lupos_core_set_task_sched_rt_mutex(current, true);
    lupos_core_lockdep_assert_site_7638(!old);
    sched_submit_work(current);
}
#[cfg(CONFIG_RT_MUTEXES)]
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_schedule() {
    lupos_core_lockdep_assert_site_7644(lupos_core_task_sched_rt_mutex(lupos_core_current()));
    __schedule_loop(LUPOS_CORE_SM_NONE);
}
#[cfg(CONFIG_RT_MUTEXES)]
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_post_schedule() {
    let current = lupos_core_current();
    sched_update_worker(current);
    let old = lupos_core_task_sched_rt_mutex(current);
    lupos_core_set_task_sched_rt_mutex(current, false);
    lupos_core_lockdep_assert_site_7651(old);
}
#[cfg(CONFIG_RT_MUTEXES)]
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_setprio(p: *mut task_struct, pi_task: *mut task_struct) {
    let prio = lupos_core_rt_effective_prio(pi_task, (*p).normal_prio);
    if (*p).pi_top_task == pi_task && prio == (*p).prio && !lupos_core_dl_prio(prio) {
        return;
    }
    let mut rf = MaybeUninit::<rq_flags>::uninit();
    let rf = rf.as_mut_ptr();
    let rq = ___task_rq_lock(p, rf);
    update_rq_clock(rq);
    (*p).pi_top_task = pi_task;
    (|| {
        if prio == (*p).prio && !lupos_core_dl_prio(prio) {
            return;
        }
        if p == (*rq).idle {
            lupos_core_warn_site_7717(p != lupos_core_rq_curr(rq));
            lupos_core_warn_site_7718(!(*p).pi_blocked_on.is_null());
            return;
        }
        lupos_core_trace_sched_pi_setprio(p, pi_task);
        let oldprio = (*p).prio;
        let mut flags =
            LUPOS_CORE_DEQUEUE_SAVE | LUPOS_CORE_DEQUEUE_MOVE | LUPOS_CORE_DEQUEUE_NOCLOCK;
        if oldprio == prio && !lupos_core_dl_prio(prio) {
            flags &= !LUPOS_CORE_DEQUEUE_MOVE;
        }
        let next_class = __setscheduler_class((*p).policy as c_int, prio);
        if (*p).sched_class != next_class {
            flags |= LUPOS_CORE_DEQUEUE_CLASS;
        }
        let ctx = sched_change_begin(p, flags as c_uint);
        if lupos_core_dl_prio(prio) {
            if !lupos_core_dl_prio((*p).normal_prio)
                || (!pi_task.is_null()
                    && lupos_core_dl_prio((*pi_task).prio)
                    && lupos_core_dl_entity_preempt(addr_of!((*pi_task).dl), addr_of!((*p).dl)))
            {
                (*p).dl.pi_se = (*pi_task).dl.pi_se;
                (*ctx).flags |= LUPOS_CORE_ENQUEUE_REPLENISH as c_int;
            } else {
                (*p).dl.pi_se = addr_of_mut!((*p).dl);
            }
        } else if lupos_core_rt_prio(prio) {
            if lupos_core_dl_prio(oldprio) {
                (*p).dl.pi_se = addr_of_mut!((*p).dl);
            }
            if oldprio < prio {
                (*ctx).flags |= LUPOS_CORE_ENQUEUE_HEAD as c_int;
            }
        } else {
            if lupos_core_dl_prio(oldprio) {
                (*p).dl.pi_se = addr_of_mut!((*p).dl);
            }
            if lupos_core_rt_prio(oldprio) {
                (*p).rt.timeout = 0;
            }
        }
        (*p).sched_class = next_class;
        (*p).prio = prio;
        sched_change_end(ctx);
    })();
    __balance_callbacks(rq, rf);
    lupos_core_task_rq_unlock_only(rq, p, rf);
}
