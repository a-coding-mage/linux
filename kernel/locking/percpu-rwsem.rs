// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn __percpu_init_rwsem(
    sem: *mut percpu_rw_semaphore,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
) -> i32 {
    unsafe {
        (*sem).read_count = alloc_percpu::<i32>();
        if unlikely((*sem).read_count.is_null()) {
            return -ENOMEM;
        }

        rcu_sync_init(&mut (*sem).rss);
        rcuwait_init(&mut (*sem).writer);
        init_waitqueue_head(&mut (*sem).waiters);
        atomic_set(&mut (*sem).block, 0);
        // CONFIG_DEBUG_LOCK_ALLOC conditionally includes the following checks.
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            debug_check_no_locks_freed(sem as *mut core::ffi::c_void, core::mem::size_of::<percpu_rw_semaphore>());
            lockdep_init_map(&mut (*sem).dep_map, name, key, 0);
        }
        0
    }
}

pub unsafe fn percpu_free_rwsem(sem: *mut percpu_rw_semaphore) {
    unsafe {
        /*
         * XXX: temporary kludge. The error path in alloc_super()
         * assumes that percpu_free_rwsem() is safe after kzalloc().
         */
        if (*sem).read_count.is_null() {
            return;
        }

        rcu_sync_dtor(&mut (*sem).rss);
        free_percpu((*sem).read_count);
        (*sem).read_count = core::ptr::null_mut(); /* catch use after free bugs */
    }
}

unsafe fn __percpu_down_read_trylock(sem: *mut percpu_rw_semaphore) -> bool {
    unsafe {
        this_cpu_inc((*sem).read_count);

        /*
         * Due to having preemption disabled the decrement happens on
         * the same CPU as the increment, avoiding the
         * increment-on-one-CPU-and-decrement-on-another problem.
         *
         * If the reader misses the writer's assignment of sem->block, then the
         * writer is guaranteed to see the reader's increment.
         *
         * Conversely, any readers that increment their sem->read_count after
         * the writer looks are guaranteed to see the sem->block value, which
         * in turn means that they are guaranteed to immediately decrement
         * their sem->read_count, so that it doesn't matter that the writer
         * missed them.
         */
        smp_mb(); /* A matches D */

        /* If !sem->block the critical section starts here, matched by the release in percpu_up_write(). */
        if likely(!atomic_read_acquire(&(*sem).block)) {
            return true;
        }

        this_cpu_dec((*sem).read_count);
        /* Prod writer to re-evaluate readers_active_check() */
        rcuwait_wake_up(&mut (*sem).writer);
        false
    }
}

unsafe fn __percpu_down_write_trylock(sem: *mut percpu_rw_semaphore) -> bool {
    unsafe {
        if atomic_read(&(*sem).block) != 0 {
            return false;
        }
        atomic_xchg(&mut (*sem).block, 1) == 0
    }
}

unsafe fn __percpu_rwsem_trylock(sem: *mut percpu_rw_semaphore, reader: bool) -> bool {
    unsafe {
        if reader {
            preempt_disable();
            let ret = __percpu_down_read_trylock(sem);
            preempt_enable();
            ret
        } else {
            __percpu_down_write_trylock(sem)
        }
    }
}

/*
 * The return value of wait_queue_entry::func means:
 *
 *  <0 - error, wakeup is terminated and the error is returned
 *   0 - no wakeup, a next waiter is tried
 *  >0 - woken, if EXCLUSIVE, counted towards @nr_exclusive.
 *
 * We use EXCLUSIVE for both readers and writers to preserve FIFO order,
 * and play games with the return value to allow waking multiple readers.
 * Specifically, we wake readers until we've woken a single writer, or until a trylock fails.
 */
unsafe fn percpu_rwsem_wake_function(
    wq_entry: *mut wait_queue_entry,
    _mode: u32,
    _wake_flags: i32,
    key: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        let reader = ((*wq_entry).flags & WQ_FLAG_CUSTOM) != 0;
        let sem = key as *mut percpu_rw_semaphore;
        if !__percpu_rwsem_trylock(sem, reader) {
            return 1;
        }

        let p = get_task_struct((*wq_entry).private);
        list_del_init(&mut (*wq_entry).entry);
        smp_store_release(&mut (*wq_entry).private, core::ptr::null_mut());
        wake_up_process(p);
        put_task_struct(p);
        if reader { 0 } else { 1 } /* wake (readers until) 1 writer */
    }
}

unsafe fn percpu_rwsem_wait(sem: *mut percpu_rw_semaphore, reader: bool, freeze: bool) {
    unsafe {
        let mut wq_entry = DEFINE_WAIT_FUNC!(percpu_rwsem_wake_function);
        let mut wait;

        spin_lock_irq(&mut (*sem).waiters.lock);
        wait = !__percpu_rwsem_trylock(sem, reader);
        if wait {
            wq_entry.flags |= WQ_FLAG_EXCLUSIVE | if reader { WQ_FLAG_CUSTOM } else { 0 };
            __add_wait_queue_entry_tail(&mut (*sem).waiters, &mut wq_entry);
        }
        spin_unlock_irq(&mut (*sem).waiters.lock);

        while wait {
            set_current_state(TASK_UNINTERRUPTIBLE | if freeze { TASK_FREEZABLE } else { 0 });
            if !smp_load_acquire(&wq_entry.private) {
                break;
            }
            schedule();
        }
        __set_current_state(TASK_RUNNING);
    }
}

pub unsafe fn __percpu_down_read(sem: *mut percpu_rw_semaphore, try_: bool, freeze: bool) -> bool {
    unsafe {
        if __percpu_down_read_trylock(sem) {
            return true;
        }
        if try_ {
            return false;
        }
        trace_contention_begin(sem, LCB_F_PERCPU | LCB_F_READ);
        preempt_enable();
        percpu_rwsem_wait(sem, true, freeze);
        preempt_disable();
        trace_contention_end(sem, 0);
        true
    }
}

unsafe fn per_cpu_sum(sem: *mut percpu_rw_semaphore) -> i32 {
    unsafe {
        let mut sum: i32 = 0;
        for_each_possible_cpu!(|cpu| { sum = sum.wrapping_add(per_cpu((*sem).read_count, cpu)); });
        sum
    }
}

pub unsafe fn percpu_is_read_locked(sem: *mut percpu_rw_semaphore) -> bool {
    unsafe { per_cpu_sum(sem) != 0 && atomic_read(&(*sem).block) == 0 }
}

unsafe fn readers_active_check(sem: *mut percpu_rw_semaphore) -> bool {
    unsafe {
        if data_race(per_cpu_sum(sem)) != 0 {
            return false;
        }
        /* If we observed the decrement; ensure we see the entire critical section. */
        smp_mb(); /* C matches B */
        true
    }
}

pub unsafe fn percpu_down_write(sem: *mut percpu_rw_semaphore) {
    unsafe {
        let mut contended = false;
        might_sleep();
        rwsem_acquire(&mut (*sem).dep_map, 0, 0, _RET_IP_);
        rcu_sync_enter(&mut (*sem).rss);
        if !__percpu_down_write_trylock(sem) {
            trace_contention_begin(sem, LCB_F_PERCPU | LCB_F_WRITE);
            percpu_rwsem_wait(sem, false, false);
            contended = true;
        }
        rcuwait_wait_event!(&mut (*sem).writer, readers_active_check(sem), TASK_UNINTERRUPTIBLE);
        if contended {
            trace_contention_end(sem, 0);
        }
    }
}

pub unsafe fn percpu_up_write(sem: *mut percpu_rw_semaphore) {
    unsafe {
        rwsem_release(&mut (*sem).dep_map, _RET_IP_);
        if trace_contended_release_enabled() && wq_has_sleeper(&(*sem).waiters) {
            trace_call__contended_release(sem);
        }
        atomic_set_release(&mut (*sem).block, 0);
        __wake_up(&mut (*sem).waiters, TASK_NORMAL, 1, sem as *mut core::ffi::c_void);
        rcu_sync_exit(&mut (*sem).rss);
    }
}

pub unsafe fn __percpu_up_read(sem: *mut percpu_rw_semaphore) {
    unsafe {
        lockdep_assert_preemption_disabled();
        if trace_contended_release_enabled() && rcuwait_active(&(*sem).writer) {
            trace_call__contended_release(sem);
        }
        smp_mb(); /* B matches C */
        this_cpu_dec((*sem).read_count);
        rcuwait_wake_up(&mut (*sem).writer);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
