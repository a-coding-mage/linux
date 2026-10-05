// SPDX-License-Identifier: GPL-2.0-only
#[no_mangle]
pub unsafe extern "C" fn sched_task_on_rq(p: *mut task_struct) -> bool {
    // SAFETY: The caller keeps p live for the native runqueue-state snapshot.
    unsafe {
        lupos_core_task_on_rq_queued(p)
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_wchan(p: *mut task_struct) -> c_ulong {
    // SAFETY: A non-null p is kept live by the caller. Its pi lock and native
    // state ordering keep a blocked task stable while the stack walker runs.
    unsafe {
        if p.is_null() || p == lupos_core_current() {
            return 0;
        }
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*p).pi_lock));
        let state = lupos_core_read_once_uint(addr_of!((*p).__state));
        lupos_core_smp_rmb();
        let ip =
            if state != LUPOS_CORE_TASK_RUNNING && state != LUPOS_CORE_TASK_WAKING && (*p).on_rq == 0 {
                __get_wchan(p)
            } else {
                0
            };
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*p).pi_lock));
        ip
    }
}
#[no_mangle]
pub unsafe extern "C" fn enqueue_task(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    // SAFETY: The caller holds the live rq's lock and keeps p live in the
    // matching class; class, utilization, PSI and core updates share that lock.
    unsafe {
        if flags & LUPOS_CORE_ENQUEUE_NOCLOCK == 0 {
            update_rq_clock(rq);
        }
        uclamp_rq_inc(rq, p, flags);
        ((*(*p).sched_class).enqueue_task.unwrap_unchecked())(rq, p, flags);
        lupos_core_psi_enqueue(p, flags);
        if flags & LUPOS_CORE_ENQUEUE_RESTORE == 0 {
            lupos_core_sched_info_enqueue(rq, p);
        }
        if lupos_core_sched_core_enabled(rq) {
            sched_core_enqueue(rq, p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dequeue_task(rq: *mut rq, p: *mut task_struct, flags: c_int) -> bool {
    // SAFETY: The caller holds the live rq's lock and keeps p live. Class
    // dequeue may delay sleeping tasks; the native callback owns that contract.
    unsafe {
        if lupos_core_sched_core_enabled(rq) {
            sched_core_dequeue(rq, p, flags);
        }
        if flags & LUPOS_CORE_DEQUEUE_NOCLOCK == 0 {
            update_rq_clock(rq);
        }
        if flags & LUPOS_CORE_DEQUEUE_SAVE == 0 {
            lupos_core_sched_info_dequeue(rq, p);
        }
        lupos_core_psi_dequeue(p, flags);
        uclamp_rq_dec(rq, p);
        ((*(*p).sched_class).dequeue_task.unwrap_unchecked())(rq, p, flags)
    }
}
#[no_mangle]
pub unsafe extern "C" fn activate_task(rq: *mut rq, p: *mut task_struct, mut flags: c_int) {
    // SAFETY: The caller holds rq's lock and owns p's enqueue transition;
    // the native on_rq publication follows the class enqueue as required.
    unsafe {
        if lupos_core_task_on_rq_migrating(p) {
            flags |= LUPOS_CORE_ENQUEUE_MIGRATED;
        }
        enqueue_task(rq, p, flags);
        lupos_core_write_once_u8(addr_of_mut!((*p).on_rq), LUPOS_CORE_TASK_ON_RQ_QUEUED);
        lupos_core_assert_on_rq_writer(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn deactivate_task(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    // SAFETY: The caller holds rq's lock and owns p's migration transition;
    // the native migrating-state store precedes class dequeue.
    unsafe {
        lupos_core_warn_deactivate_sleep(flags & LUPOS_CORE_DEQUEUE_SLEEP != 0);
        lupos_core_write_once_u8(addr_of_mut!((*p).on_rq), LUPOS_CORE_TASK_ON_RQ_MIGRATING);
        lupos_core_assert_on_rq_writer(p);
        dequeue_task(rq, p, flags);
    }
}
unsafe fn block_task(rq: *mut rq, p: *mut task_struct, state: c_ulong) {
    // SAFETY: The caller holds rq's lock and owns p's blocked-state
    // transition; no task-state read follows the successful native block step.
    unsafe {
        let mut flags = LUPOS_CORE_DEQUEUE_NOCLOCK;
        lupos_core_set_task_contributes_to_load(
            p,
            ((state & LUPOS_CORE_TASK_UNINTERRUPTIBLE as c_ulong != 0)
                && (state & LUPOS_CORE_TASK_NOLOAD as c_ulong == 0)
                && (state & LUPOS_CORE_TASK_FROZEN as c_ulong == 0)) as c_uint,
        );
        if lupos_core_is_special_task_state(state) {
            flags |= LUPOS_CORE_DEQUEUE_SPECIAL;
        }
        if dequeue_task(rq, p, LUPOS_CORE_DEQUEUE_SLEEP | flags) {
            lupos_core_block_task(rq, p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn task_curr(p: *const task_struct) -> c_int {
    // SAFETY: The caller keeps p live; native CPU/current-task accessors
    // provide a snapshot without dereferencing the returned current pointer.
    unsafe {
        (lupos_core_cpu_curr(lupos_core_task_cpu(p)) == p.cast_mut()) as c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn wakeup_preempt(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    // SAFETY: The caller holds the live rq's lock and keeps its donor and p
    // live while invoking the selected class's wakeup/preemption callback.
    unsafe {
        let donor = lupos_core_rq_donor(rq);
        if (*p).sched_class == (*rq).next_class {
            ((*(*rq).next_class).wakeup_preempt.unwrap_unchecked())(rq, p, flags);
        } else if lupos_core_sched_class_above((*p).sched_class, (*rq).next_class) {
            ((*(*rq).next_class).wakeup_preempt.unwrap_unchecked())(rq, p, flags);
            resched_curr(rq);
            (*rq).next_class = (*p).sched_class;
        }
        if lupos_core_task_on_rq_queued(donor)
            && lupos_core_test_tsk_need_resched(lupos_core_rq_curr(rq))
        {
            lupos_core_rq_clock_skip_update(rq);
        }
    }
}
#[inline(always)]
unsafe fn __task_state_match(p: *mut task_struct, state: c_uint) -> c_int {
    // SAFETY: The caller keeps p live and serializes wake-state decisions;
    // both state fields are read through native READ_ONCE operations.
    unsafe {
        if lupos_core_read_once_uint(addr_of!((*p).__state)) & state != 0 {
            return 1;
        }
        if lupos_core_read_once_uint(addr_of!((*p).saved_state)) & state != 0 {
            return -1;
        }
        0
    }
}
#[inline(always)]
unsafe fn task_state_match(p: *mut task_struct, state: c_uint) -> c_int {
    // SAFETY: The caller keeps p live with IRQs enabled on entry; p's pi
    // lock serializes normal and saved task-state matching.
    unsafe {
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*p).pi_lock));
        let ret = __task_state_match(p, state);
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*p).pi_lock));
        ret
    }
}
#[no_mangle]
pub unsafe extern "C" fn wait_task_inactive(p: *mut task_struct, state: c_uint) -> c_ulong {
    // SAFETY: The caller keeps p live and may sleep with IRQs enabled.
    // Native snapshots precede pi/rq-locked checks; all waits occur after unlock.
    unsafe {
        loop {
            let mut rq = lupos_core_task_rq(p);
            while lupos_core_task_on_cpu(rq, p) {
                if task_state_match(p, state) == 0 {
                    return 0;
                }
                lupos_core_cpu_relax();
            }
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            rq = _task_rq_lock(p, rf.as_mut_ptr());
            if (*p).se.sched_delayed != 0 {
                dequeue_task(rq, p, LUPOS_CORE_DEQUEUE_SLEEP | LUPOS_CORE_DEQUEUE_DELAYED);
            }
            lupos_core_trace_wait_task(p);
            let running = lupos_core_task_on_cpu(rq, p);
            let mut queued = lupos_core_task_on_rq_queued(p);
            let matched = __task_state_match(p, state);
            let ncsw = if matched != 0 {
                if matched < 0 {
                    queued = true;
                }
                (*p).nvcsw | LUPOS_CORE_LONG_MIN as c_ulong
            } else {
                0
            };
            lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
            if ncsw == 0 {
                return 0;
            }
            if running {
                lupos_core_cpu_relax();
                continue;
            }
            if queued {
                let mut timeout = (LUPOS_CORE_NSEC_PER_SEC / LUPOS_CORE_HZ) as ktime_t;
                lupos_core_set_current_state(LUPOS_CORE_TASK_UNINTERRUPTIBLE);
                schedule_hrtimeout(addr_of_mut!(timeout), LUPOS_CORE_HRTIMER_MODE_REL_HARD);
                continue;
            }
            return ncsw;
        }
    }
}
unsafe fn __resched_curr(rq: *mut rq, mut tif: c_int) {
    // SAFETY: The caller holds the live rq's lock and supplies an in-range
    // native TIF bit; its current task remains live through flag/IPI operations.
    unsafe {
        let curr = lupos_core_rq_curr(rq);
        let cti = lupos_core_task_thread_info(curr);
        lupos_core_assert_rq_held(rq);
        if lupos_core_is_idle_task(curr) && tif == LUPOS_CORE_TIF_NEED_RESCHED_LAZY {
            tif = LUPOS_CORE_TIF_NEED_RESCHED;
        }
        if (*cti).flags & ((1 as c_ulong).wrapping_shl(tif as u32) | LUPOS_CORE_TIF_NEED_RESCHED_MASK)
            != 0
        {
            return;
        }
        let cpu = lupos_core_cpu_of(rq);
        lupos_core_trace_set_need_resched(curr, cpu, tif);
        if cpu == lupos_core_smp_processor_id() {
            lupos_core_set_ti_thread_flag(cti, tif);
            if tif == LUPOS_CORE_TIF_NEED_RESCHED {
                lupos_core_set_preempt_need_resched();
            }
            return;
        }
        if set_nr_and_not_polling(cti, tif) {
            if tif == LUPOS_CORE_TIF_NEED_RESCHED {
                lupos_core_smp_send_reschedule(cpu);
            }
        } else {
            lupos_core_trace_wake_idle_without_ipi(cpu);
        }
    }
}
unsafe fn set_nr_and_not_polling(ti: *mut thread_info, tif: c_int) -> bool {
    // SAFETY: ti points to live thread-info storage and tif is a native bit.
    // The native compare-exchange or configured thread-flag operation owns ordering.
    unsafe {
        #[cfg(LUPOS_CORE_HAS_TIF_POLLING_NRFLAG)]
        {
            let mut value = (*ti).flags;
            loop {
                let new = value | (1 as c_ulong).wrapping_shl(tif as u32);
                if lupos_core_try_cmpxchg_ulong(addr_of_mut!((*ti).flags), addr_of_mut!(value), new) {
                    return value & LUPOS_CORE_TIF_POLLING_NRFLAG_MASK == 0;
                }
            }
        }
        #[cfg(not(LUPOS_CORE_HAS_TIF_POLLING_NRFLAG))]
        {
            lupos_core_set_ti_thread_flag(ti, tif);
            true
        }
    }
}
#[cfg(LUPOS_CORE_HAS_TIF_POLLING_NRFLAG)]
unsafe fn set_nr_if_polling(p: *mut task_struct) -> bool {
    // SAFETY: The caller keeps p and its thread-info live. Native READ_ONCE
    // and compare-exchange implement the polling/reschedule handshake.
    unsafe {
        let ti = lupos_core_task_thread_info(p);
        let mut value = lupos_core_read_once_ulong(addr_of!((*ti).flags));
        loop {
            if value & LUPOS_CORE_TIF_POLLING_NRFLAG_MASK == 0 {
                return false;
            }
            if value & LUPOS_CORE_TIF_NEED_RESCHED_MASK != 0 {
                return true;
            }
            let new = value | LUPOS_CORE_TIF_NEED_RESCHED_MASK;
            if lupos_core_try_cmpxchg_ulong(addr_of_mut!((*ti).flags), addr_of_mut!(value), new) {
                return true;
            }
        }
    }
}
#[cfg(not(LUPOS_CORE_HAS_TIF_POLLING_NRFLAG))]
unsafe fn set_nr_if_polling(_p: *mut task_struct) -> bool { false }
#[no_mangle]
pub unsafe extern "C" fn resched_curr(rq: *mut rq) {
    // SAFETY: The caller holds the live runqueue's scheduling lock required
    // by __resched_curr and its native current-task flag update.
    unsafe {
        __resched_curr(rq, LUPOS_CORE_TIF_NEED_RESCHED);
    }
}
#[inline(always)]
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
unsafe fn dynamic_preempt_lazy() -> bool {
    // SAFETY: The native dynamic-preempt static key has permanent storage;
    // its configured accessor supplies the ordered key read.
    unsafe {
        lupos_core_dynamic_preempt_lazy()
    }
}
#[cfg(not(CONFIG_PREEMPT_DYNAMIC))]
#[inline(always)]
unsafe fn dynamic_preempt_lazy() -> bool { cfg!(CONFIG_PREEMPT_LAZY) }
#[inline(always)]
unsafe fn get_lazy_tif_bit() -> c_int {
    // SAFETY: The configured dynamic/static preemption query is valid in
    // this scheduler context; both result constants come from native headers.
    unsafe {
        if dynamic_preempt_lazy() {
            LUPOS_CORE_TIF_NEED_RESCHED_LAZY
        } else {
            LUPOS_CORE_TIF_NEED_RESCHED
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn resched_curr_lazy(rq: *mut rq) {
    // SAFETY: The caller holds the live rq's scheduling lock; the native
    // preemption mode selects the bit used by __resched_curr.
    unsafe {
        __resched_curr(rq, get_lazy_tif_bit());
    }
}
#[no_mangle]
pub unsafe extern "C" fn resched_cpu(cpu: c_int) {
    // SAFETY: cpu selects a valid permanent runqueue. The IRQ-saving rq lock
    // protects its online/current-CPU check and reschedule operation.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu);
        let flags = lupos_core_raw_spin_rq_lock_irqsave(rq);
        if lupos_core_cpu_online(cpu) || cpu == lupos_core_smp_processor_id() {
            resched_curr(rq);
        }
        lupos_core_raw_spin_rq_unlock_irqrestore(rq, flags);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __trace_set_current_state(state_value: c_int) {
    // SAFETY: The caller satisfies the enabled scheduler-state tracepoint
    // contract; current remains live through the native trace call.
    unsafe {
        lupos_core_trace_call_set_state(lupos_core_current(), state_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __trace_set_need_resched(curr: *mut task_struct, tif: c_int) {
    // SAFETY: The caller keeps curr live and guards this call with the
    // native sched_set_need_resched tracepoint-enabled check.
    unsafe {
        lupos_core_trace_call_set_need_resched(curr, lupos_core_smp_processor_id(), tif);
    }
}
