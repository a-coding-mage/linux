// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/freezer.c - Function to freeze a process
 *
 * Originally from kernel/power/process.c
 */

/* Linux kernel dependencies supplied by other translation units. */

/* total number of freezing conditions in effect */
#[no_mangle]
pub static mut freezer_active: bool = false;

/*
 * indicate whether PM freezing is in effect, protected by
 * system_transition_mutex
 */
#[no_mangle]
pub static mut pm_freezing: bool = false;
#[no_mangle]
pub static mut pm_nosig_freezing: bool = false;

/* protects freezing and frozen transitions */
static mut freezer_lock: core::mem::MaybeUninit<spinlock_t> = core::mem::MaybeUninit::uninit();

pub unsafe fn freezing_slow_path(p: *mut task_struct) -> bool {
    if (*p).flags & (PF_NOFREEZE | PF_SUSPEND_TASK) != 0 {
        return false;
    }

    if tsk_is_oom_victim(p) {
        return false;
    }

    if pm_nosig_freezing || cgroup1_freezing(p) {
        return true;
    }

    if pm_freezing && ((*p).flags & PF_KTHREAD) == 0 {
        return true;
    }

    false
}

pub unsafe fn frozen(p: *mut task_struct) -> bool {
    core::ptr::read_volatile(&(*p).__state) & TASK_FROZEN != 0
}

/* Refrigerator is place where frozen processes are stored :-). */
pub unsafe fn __refrigerator(check_kthr_stop: bool) -> bool {
    let state: u32 = get_current_state();
    let mut was_frozen = false;

    pr_debug!("%s entered refrigerator\n", (*current).comm);

    WARN_ON_ONCE!(state != 0 && (state & TASK_NORMAL) == 0);

    loop {
        let freeze: bool;

        raw_spin_lock_irq(&(*current).pi_lock);
        core::ptr::write_volatile(&mut (*current).__state, TASK_FROZEN);
        /* unstale saved_state so that __thaw_task() will wake us up */
        (*current).saved_state = TASK_RUNNING;
        raw_spin_unlock_irq(&(*current).pi_lock);

        spin_lock_irq(&mut *freezer_lock.as_mut_ptr());
        freeze = freezing(current) && !(check_kthr_stop && kthread_should_stop());
        spin_unlock_irq(&mut *freezer_lock.as_mut_ptr());

        if !freeze {
            break;
        }

        was_frozen = true;
        schedule();
    }
    __set_current_state(TASK_RUNNING);

    pr_debug!("%s left refrigerator\n", (*current).comm);

    was_frozen
}

unsafe fn fake_signal_wake_up(p: *mut task_struct) {
    let mut flags: usize = 0;

    if lock_task_sighand(p, &mut flags) {
        signal_wake_up(p, 0);
        unlock_task_sighand(p, &mut flags);
    }
}

unsafe fn __set_task_frozen(p: *mut task_struct, _arg: *mut core::ffi::c_void) -> i32 {
    let state: u32 = core::ptr::read_volatile(&(*p).__state);

    /*
     * Allow freezing the sched_delayed tasks; they will not execute until
     * ttwu() fixes them up, so it is safe to swap their state now, instead
     * of waiting for them to get fully dequeued.
     */
    if task_is_runnable(p) {
        return 0;
    }

    if p != current && task_curr(p) {
        return 0;
    }

    if state & (TASK_FREEZABLE | __TASK_STOPPED | __TASK_TRACED) == 0 {
        return 0;
    }

    /*
     * Only TASK_NORMAL can be augmented with TASK_FREEZABLE, since they
     * can suffer spurious wakeups.
     */
    if state & TASK_FREEZABLE != 0 {
        WARN_ON_ONCE!((state & TASK_NORMAL) == 0);
    }

    /* CONFIG_LOCKDEP is a build-time condition in the original source. */
    #[cfg(feature = "CONFIG_LOCKDEP")]
    {
        /* It's dangerous to freeze with locks held; there be dragons there. */
        if state & __TASK_FREEZABLE_UNSAFE == 0 {
            WARN_ON_ONCE!(debug_locks && (*p).lockdep_depth != 0);
        }
    }

    (*p).saved_state = (*p).__state;
    core::ptr::write_volatile(&mut (*p).__state, TASK_FROZEN);
    TASK_FROZEN as i32
}

unsafe fn __freeze_task(p: *mut task_struct) -> bool {
    /* TASK_FREEZABLE|TASK_STOPPED|TASK_TRACED -> TASK_FROZEN */
    task_call_func(p, __set_task_frozen, core::ptr::null_mut())
}

pub unsafe fn freeze_task(p: *mut task_struct) -> bool {
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut *freezer_lock.as_mut_ptr(), &mut flags);
    if !freezing(p) || frozen(p) || __freeze_task(p) {
        spin_unlock_irqrestore(&mut *freezer_lock.as_mut_ptr(), flags);
        return false;
    }

    if (*p).flags & PF_KTHREAD == 0 {
        fake_signal_wake_up(p);
    } else {
        wake_up_state(p, TASK_NORMAL);
    }

    spin_unlock_irqrestore(&mut *freezer_lock.as_mut_ptr(), flags);
    true
}

/*
 * Restore the saved_state before the task entered freezer. For typical task
 * in the __refrigerator(), saved_state == TASK_RUNNING so nothing happens
 * here. For tasks which were TASK_NORMAL | TASK_FREEZABLE, their initial state
 * is restored unless they got an expected wakeup (see ttwu_state_match()).
 * Returns 1 if the task state was restored.
 */
unsafe fn __restore_freezer_state(p: *mut task_struct, _arg: *mut core::ffi::c_void) -> i32 {
    let state = (*p).saved_state;

    if state != TASK_RUNNING {
        core::ptr::write_volatile(&mut (*p).__state, state);
        (*p).saved_state = TASK_RUNNING;
        return 1;
    }

    0
}

pub unsafe fn __thaw_task(p: *mut task_struct) {
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut *freezer_lock.as_mut_ptr(), &mut flags);
    if frozen(p) && !task_call_func(p, __restore_freezer_state, core::ptr::null_mut()) {
        wake_up_state(p, TASK_FROZEN);
    }
    spin_unlock_irqrestore(&mut *freezer_lock.as_mut_ptr(), flags);
}

/*
 * thaw_process - Thaw a frozen process
 * @p: the process to be thawed
 *
 * Iterate over all threads of @p and call __thaw_task() on each.
 */
pub unsafe fn thaw_process(p: *mut task_struct) {
    let mut t: *mut task_struct = core::ptr::null_mut();

    rcu_read_lock();
    for_each_thread!(p, t, {
        __thaw_task(t);
    });
    rcu_read_unlock();
}

/**
 * set_freezable - make %current freezable
 *
 * Mark %current freezable and enter refrigerator if necessary.
 */
pub unsafe fn set_freezable() -> bool {
    might_sleep();

    /*
     * Modify flags while holding freezer_lock.  This ensures the
     * freezer notices that we aren't frozen yet or the freezing
     * condition is visible to try_to_freeze() below.
     */
    spin_lock_irq(&mut *freezer_lock.as_mut_ptr());
    (*current).flags &= !PF_NOFREEZE;
    spin_unlock_irq(&mut *freezer_lock.as_mut_ptr());

    try_to_freeze()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
