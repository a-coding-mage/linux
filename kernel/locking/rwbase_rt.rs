// SPDX-License-Identifier: GPL-2.0-only

/*
 * RT-specific reader/writer semaphores and reader/writer locks
 *
 * down_write/write_lock()
 *  1) Lock rtmutex
 *  2) Remove the reader BIAS to force readers into the slow path
 *  3) Wait until all readers have left the critical section
 *  4) Mark it write locked
 *
 * up_write/write_unlock()
 *  1) Remove the write locked marker
 *  2) Set the reader BIAS, so readers can use the fast path again
 *  3) Unlock rtmutex, to release blocked readers
 *
 * down_read/read_lock()
 *  1) Try fast path acquisition (reader BIAS is set)
 *  2) Take tmutex::wait_lock, which protects the writelocked flag
 *  3) If !writelocked, acquire it for read
 *  4) If writelocked, block on tmutex
 *  5) unlock rtmutex, goto 1)
 *
 * up_read/read_unlock()
 *  1) Try fast path release (reader count != 1)
 *  2) Wake the writer waiting in down_write()/write_lock() #3
 *
 * down_read/read_lock()#3 has the consequence, that rw semaphores and rw
 * locks on RT are not writer fair, but writers, which should be avoided in
 * RT tasks (think mmap_sem), are subject to the rtmutex priority/DL
 * inheritance mechanism.
 *
 * It's possible to make the rw primitives writer fair by keeping a list of
 * active readers. A blocked writer would force all newly incoming readers to
 * block on the rtmutex, but the rtmutex would have to be proxy locked
 * for one reader after the other. We can't use multi-reader inheritance
 * because there is no way to support that with SCHED_DEADLINE.
 * Implementing the one by one reader boosting/handover mechanism is a
 * major surgery for a very dubious value.
 *
 * The risk of writer starvation is there, but the pathological use cases
 * which trigger it are not necessarily the typical RT workloads.
 *
 * Fast-path orderings:
 * The lock/unlock of readers can run in fast paths: lock and unlock are only
 * atomic ops, and there is no inner lock to provide ACQUIRE and RELEASE
 * semantics of rwbase_rt. Atomic ops should thus provide _acquire()
 * and _release() (or stronger).
 *
 * Common code shared between RT rw_semaphore and rwlock
 */

#[inline(always)]
unsafe fn rwbase_read_trylock(rwb: *mut rwbase_rt) -> i32 {
    let mut r: i32;
    r = atomic_read(unsafe { &(*rwb).readers });
    while r < 0 {
        if likely(atomic_try_cmpxchg_acquire(unsafe { &(*rwb).readers }, &mut r, r + 1)) {
            return 1;
        }
    }
    0
}

unsafe fn __rwbase_read_lock(rwb: *mut rwbase_rt, state: u32) -> i32 {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let mut wake_q = WakeQ::default();
    let ret: i32;

    rwbase_pre_schedule();
    raw_spin_lock_irq(&mut (*rtm).wait_lock);

    trace_contention_begin(rwb, LCB_F_RT | LCB_F_READ);
    ret = rwbase_rtmutex_slowlock_locked(rtm, state, &mut wake_q);
    if ret == 0 {
        atomic_inc(&mut (*rwb).readers);
    }

    preempt_disable();
    raw_spin_unlock_irq(&mut (*rtm).wait_lock);
    wake_up_q(&mut wake_q);
    preempt_enable();

    if ret == 0 {
        rwbase_rtmutex_unlock(rtm);
    }

    trace_contention_end(rwb, ret);
    rwbase_post_schedule();
    ret
}

#[inline(always)]
unsafe fn rwbase_read_lock(rwb: *mut rwbase_rt, state: u32) -> i32 {
    lockdep_assert(!(*current).pi_blocked_on);
    if rwbase_read_trylock(rwb) != 0 {
        return 0;
    }
    __rwbase_read_lock(rwb, state)
}

unsafe fn __rwbase_read_unlock(rwb: *mut rwbase_rt, state: u32) {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let owner: *mut task_struct;
    let mut wqh = RtWakeQ::default();

    raw_spin_lock_irq(&mut (*rtm).wait_lock);
    owner = rt_mutex_owner(rtm);
    if !owner.is_null() {
        rt_mutex_wake_q_add_task(&mut wqh, owner, state);
    }
    preempt_disable();
    raw_spin_unlock_irq(&mut (*rtm).wait_lock);
    rt_mutex_wake_up_q(&mut wqh);
}

#[inline(always)]
unsafe fn rwbase_read_unlock(rwb: *mut rwbase_rt, state: u32) {
    if trace_contended_release_enabled() && !rt_mutex_owner(&mut (*rwb).rtmutex).is_null() {
        trace_call__contended_release(rwb);
    }
    if unlikely(atomic_dec_and_test(&mut (*rwb).readers)) {
        __rwbase_read_unlock(rwb, state);
    }
}

#[inline]
unsafe fn __rwbase_write_unlock(rwb: *mut rwbase_rt, bias: i32, flags: usize) {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let _ = atomic_add_return_release(READER_BIAS - bias, &mut (*rwb).readers);
    raw_spin_unlock_irqrestore(&mut (*rtm).wait_lock, flags);
    rwbase_rtmutex_unlock(rtm);
}

#[inline]
unsafe fn rwbase_write_unlock(rwb: *mut rwbase_rt) {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*rtm).wait_lock, &mut flags);
    if trace_contended_release_enabled() && rt_mutex_has_waiters(rtm) {
        trace_call__contended_release(rwb);
    }
    __rwbase_write_unlock(rwb, WRITER_BIAS, flags);
}

#[inline]
unsafe fn rwbase_write_downgrade(rwb: *mut rwbase_rt) {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*rtm).wait_lock, &mut flags);
    if trace_contended_release_enabled() && rt_mutex_has_waiters(rtm) {
        trace_call__contended_release(rwb);
    }
    __rwbase_write_unlock(rwb, WRITER_BIAS - 1, flags);
}

#[inline]
unsafe fn __rwbase_write_trylock(rwb: *mut rwbase_rt) -> bool {
    lockdep_assert_held(&(*rwb).rtmutex.wait_lock);
    if atomic_read_acquire(&(*rwb).readers) == 0 {
        atomic_set(&mut (*rwb).readers, WRITER_BIAS);
        return true;
    }
    false
}

unsafe fn rwbase_write_lock(rwb: *mut rwbase_rt, state: u32) -> i32 {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let mut flags: usize = 0;

    if rwbase_rtmutex_lock_state(rtm, state) {
        return -EINTR;
    }
    atomic_sub(READER_BIAS, &mut (*rwb).readers);
    rwbase_pre_schedule();
    raw_spin_lock_irqsave(&mut (*rtm).wait_lock, &mut flags);
    if __rwbase_write_trylock(rwb) {
        raw_spin_unlock_irqrestore(&mut (*rtm).wait_lock, flags);
        rwbase_post_schedule();
        return 0;
    }

    rwbase_set_and_save_current_state(state);
    trace_contention_begin(rwb, LCB_F_RT | LCB_F_WRITE);
    loop {
        if rwbase_signal_pending_state(state, current) {
            rwbase_restore_current_state();
            __rwbase_write_unlock(rwb, 0, flags);
            rwbase_post_schedule();
            trace_contention_end(rwb, -EINTR);
            return -EINTR;
        }
        if __rwbase_write_trylock(rwb) {
            break;
        }
        raw_spin_unlock_irqrestore(&mut (*rtm).wait_lock, flags);
        rwbase_schedule();
        raw_spin_lock_irqsave(&mut (*rtm).wait_lock, &mut flags);
        set_current_state(state);
    }
    rwbase_restore_current_state();
    trace_contention_end(rwb, 0);
    raw_spin_unlock_irqrestore(&mut (*rtm).wait_lock, flags);
    rwbase_post_schedule();
    0
}

#[inline]
unsafe fn rwbase_write_trylock(rwb: *mut rwbase_rt) -> i32 {
    let rtm: *mut rt_mutex_base = &mut (*rwb).rtmutex;
    let mut flags: usize = 0;
    if !rwbase_rtmutex_trylock(rtm) {
        return 0;
    }
    atomic_sub(READER_BIAS, &mut (*rwb).readers);
    raw_spin_lock_irqsave(&mut (*rtm).wait_lock, &mut flags);
    if __rwbase_write_trylock(rwb) {
        raw_spin_unlock_irqrestore(&mut (*rtm).wait_lock, flags);
        return 1;
    }
    __rwbase_write_unlock(rwb, 0, flags);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
