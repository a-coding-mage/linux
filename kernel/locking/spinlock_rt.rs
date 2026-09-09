// SPDX-License-Identifier: GPL-2.0-only
/*
 * PREEMPT_RT substitution for spin/rw_locks
 *
 * spinlocks and rwlocks on RT are based on rtmutexes, with a few twists to
 * resemble the non RT semantics:
 *
 * - Contrary to plain rtmutexes, spinlocks and rwlocks are state
 *   preserving. The task state is saved before blocking on the underlying
 *   rtmutex, and restored when the lock has been acquired. Regular wakeups
 *   during that time are redirected to the saved state so no wake up is
 *   missed.
 *
 * - Non RT spin/rwlocks disable preemption and eventually interrupts.
 *   Disabling preemption has the side effect of disabling migration and
 *   preventing RCU grace periods.
 *
 *   The RT substitutions explicitly disable migration and take
 *   rcu_read_lock() across the lock held section.
 */

// C includes and RT-specific build configuration are supplied by the kernel
// translation environment.

/*
 * __might_resched() skips the state check as rtlocks are state preserving. Take
 * RCU nesting into account as spin/read/write_lock() can legitimately nest into
 * an RCU read side critical section.
 */
#[inline(always)]
unsafe fn rtlock_might_resched() {
    __might_resched(file!(), line!(), rcu_preempt_depth() << MIGHT_RESCHED_RCU_SHIFT);
}

#[inline(always)]
unsafe fn rtlock_lock(rtm: *mut rt_mutex_base) {
    lockdep_assert(!(*current).pi_blocked_on);

    if !rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current) {
        rtlock_slowlock(rtm);
    }
}

#[inline(always)]
unsafe fn __rt_spin_lock(lock: *mut spinlock_t) {
    rtlock_might_resched();
    rtlock_lock(&mut (*lock).lock);
    rcu_read_lock();
    migrate_disable();
}

pub unsafe fn rt_spin_lock(lock: *mut spinlock_t) {
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_);
    __rt_spin_lock(lock);
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn rt_spin_lock_nested(lock: *mut spinlock_t, subclass: i32) {
    spin_acquire(&mut (*lock).dep_map, subclass, 0, _RET_IP_);
    __rt_spin_lock(lock);
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn rt_spin_lock_nest_lock(lock: *mut spinlock_t, nest_lock: *mut lockdep_map) {
    spin_acquire_nest(&mut (*lock).dep_map, 0, 0, nest_lock, _RET_IP_);
    __rt_spin_lock(lock);
}

pub unsafe fn rt_spin_unlock(lock: *mut spinlock_t) {
    spin_release(&mut (*lock).dep_map, _RET_IP_);
    migrate_enable();

    if !rt_mutex_cmpxchg_release(&mut (*lock).lock, current, core::ptr::null_mut()) {
        rt_mutex_slowunlock(&mut (*lock).lock);
    }

    /*
     * This must be last to prevent the following UAF:
     *
     * T1                                  T2
     * spin_lock(&p->lock);                rcu_read_lock();
     * invalidate(p);                      p = rcu_dereference(ptr);
     * rcu_assign_pointer(ptr, NULL);      if (!p) return;
     * spin_unlock(&p->lock);              spin_lock(&p->lock);
     * kfree_rcu(p);                       rcu_read_unlock();
     *                                     ....
     *                                     spin_unlock(&p->lock)
     *                                       rcu_read_unlock(); // Ends grace period
     * rcu_do_batch()
     *   kfree(p);
     *                             UAF ->  rt_mutex_cmpxchg_release(&p->lock.lock...)
     */
    rcu_read_unlock();
}

/* Wait for the lock to get unlocked by forcing the kernel to schedule if
 * there is contention. */
pub unsafe fn rt_spin_lock_unlock(lock: *mut spinlock_t) {
    spin_lock(lock);
    spin_unlock(lock);
}

#[inline(always)]
unsafe fn __rt_spin_trylock(lock: *mut spinlock_t) -> i32 {
    let mut ret = 1;

    if !rt_mutex_cmpxchg_acquire(&mut (*lock).lock, core::ptr::null_mut(), current) {
        ret = rt_mutex_slowtrylock(&mut (*lock).lock);
    }

    if ret != 0 {
        spin_acquire(&mut (*lock).dep_map, 0, 1, _RET_IP_);
        rcu_read_lock();
        migrate_disable();
    }
    ret
}

pub unsafe fn rt_spin_trylock(lock: *mut spinlock_t) -> i32 {
    __rt_spin_trylock(lock)
}

pub unsafe fn rt_spin_trylock_bh(lock: *mut spinlock_t) -> i32 {
    local_bh_disable();
    let ret = __rt_spin_trylock(lock);
    if ret == 0 {
        local_bh_enable();
    }
    ret
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn __rt_spin_lock_init(lock: *mut spinlock_t, name: *const i8,
                                   key: *mut lock_class_key, percpu: bool) {
    let ty = if percpu { LD_LOCK_PERCPU } else { LD_LOCK_NORMAL };
    debug_check_no_locks_freed(lock as *const _, core::mem::size_of::<spinlock_t>());
    lockdep_init_map_type(&mut (*lock).dep_map, name, key, 0, LD_WAIT_CONFIG,
                          LD_WAIT_INV, ty);
}

/* RT-specific reader/writer locks */

#[inline(always)]
unsafe fn rwbase_rtmutex_lock_state(rtm: *mut rt_mutex_base, _state: u32) -> i32 {
    if !rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current) {
        rtlock_slowlock(rtm);
    }
    0
}

#[inline(always)]
unsafe fn rwbase_rtmutex_slowlock_locked(rtm: *mut rt_mutex_base,
                                         _state: u32, wake_q: *mut wake_q_head) -> i32 {
    rtlock_slowlock_locked(rtm, wake_q);
    0
}

#[inline(always)]
unsafe fn rwbase_rtmutex_unlock(rtm: *mut rt_mutex_base) {
    if rt_mutex_cmpxchg_acquire(rtm, current, core::ptr::null_mut()) {
        return;
    }
    rt_mutex_slowunlock(rtm);
}

#[inline(always)]
unsafe fn rwbase_rtmutex_trylock(rtm: *mut rt_mutex_base) -> i32 {
    if rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current) {
        return 1;
    }
    rt_mutex_slowtrylock(rtm)
}

#[inline(always)]
unsafe fn rwbase_schedule() {
    schedule_rtlock();
}

pub unsafe fn rt_read_trylock(rwlock: *mut rwlock_t) -> i32 {
    let ret = rwbase_read_trylock(&mut (*rwlock).rwbase);
    if ret != 0 {
        rwlock_acquire_read(&mut (*rwlock).dep_map, 0, 1, _RET_IP_);
        rcu_read_lock();
        migrate_disable();
    }
    ret
}

pub unsafe fn rt_write_trylock(rwlock: *mut rwlock_t) -> i32 {
    let ret = rwbase_write_trylock(&mut (*rwlock).rwbase);
    if ret != 0 {
        rwlock_acquire(&mut (*rwlock).dep_map, 0, 1, _RET_IP_);
        rcu_read_lock();
        migrate_disable();
    }
    ret
}

pub unsafe fn rt_read_lock(rwlock: *mut rwlock_t) {
    rtlock_might_resched();
    rwlock_acquire_read(&mut (*rwlock).dep_map, 0, 0, _RET_IP_);
    rwbase_read_lock(&mut (*rwlock).rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
}

pub unsafe fn rt_write_lock(rwlock: *mut rwlock_t) {
    rtlock_might_resched();
    rwlock_acquire(&mut (*rwlock).dep_map, 0, 0, _RET_IP_);
    rwbase_write_lock(&mut (*rwlock).rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn rt_write_lock_nested(rwlock: *mut rwlock_t, subclass: i32) {
    rtlock_might_resched();
    rwlock_acquire(&mut (*rwlock).dep_map, subclass, 0, _RET_IP_);
    rwbase_write_lock(&mut (*rwlock).rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
}

pub unsafe fn rt_read_unlock(rwlock: *mut rwlock_t) {
    rwlock_release(&mut (*rwlock).dep_map, _RET_IP_);
    migrate_enable();
    rwbase_read_unlock(&mut (*rwlock).rwbase, TASK_RTLOCK_WAIT);
    /* This must be last. See comment in rt_spin_unlock() */
    rcu_read_unlock();
}

pub unsafe fn rt_write_unlock(rwlock: *mut rwlock_t) {
    rwlock_release(&mut (*rwlock).dep_map, _RET_IP_);
    migrate_enable();
    rwbase_write_unlock(&mut (*rwlock).rwbase);
    /* This must be last. See comment in rt_spin_unlock() */
    rcu_read_unlock();
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe fn __rt_rwlock_init(rwlock: *mut rwlock_t, name: *const i8,
                               key: *mut lock_class_key) {
    debug_check_no_locks_freed(rwlock as *const _, core::mem::size_of::<rwlock_t>());
    lockdep_init_map_wait(&mut (*rwlock).dep_map, name, key, 0, LD_WAIT_CONFIG);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
