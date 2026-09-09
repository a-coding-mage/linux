// SPDX-License-Identifier: GPL-2.0
/*
 * kernel/power/autosleep.c
 *
 * Opportunistic sleep support.
 *
 * Copyright (C) 2012 Rafael J. Wysocki <rjw@sisk.pl>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut autosleep_state: suspend_state_t = 0 as suspend_state_t;
static mut autosleep_wq: *mut workqueue_struct = core::ptr::null_mut();
/*
 * Note: it is only safe to mutex_lock(&autosleep_lock) if a wakeup_source
 * is active, otherwise a deadlock with try_to_suspend() is possible.
 * Alternatively mutex_lock_interruptible() can be used.  This will then fail
 * if an auto_sleep cycle tries to freeze processes.
 */
static mut autosleep_lock: mutex = DEFINE_MUTEX!();
static mut autosleep_ws: *mut wakeup_source = core::ptr::null_mut();

unsafe fn try_to_suspend(work: *mut work_struct) {
    let mut initial_count: c_uint = 0;
    let mut final_count: c_uint = 0;

    'out: {
        if !pm_get_wakeup_count(&mut initial_count, true) {
            break 'out;
        }

        mutex_lock(&mut autosleep_lock);

        if !pm_save_wakeup_count(initial_count) || system_state != SYSTEM_RUNNING {
            mutex_unlock(&mut autosleep_lock);
            break 'out;
        }

        if autosleep_state == PM_SUSPEND_ON {
            mutex_unlock(&mut autosleep_lock);
            return;
        }
        if autosleep_state >= PM_SUSPEND_MAX {
            hibernate();
        } else {
            pm_suspend(autosleep_state);
        }

        mutex_unlock(&mut autosleep_lock);

        if !pm_get_wakeup_count(&mut final_count, false) {
            break 'out;
        }

    /*
     * If the wakeup occurred for an unknown reason, wait to prevent the
     * system from trying to suspend and waking up in a tight loop.
     */
        if final_count == initial_count {
            schedule_timeout_uninterruptible(HZ / 2);
        }
    }
    queue_up_suspend_work();
}

static mut suspend_work: work_struct = DECLARE_WORK!(try_to_suspend);

pub unsafe fn queue_up_suspend_work() {
    if autosleep_state > PM_SUSPEND_ON {
        queue_work(autosleep_wq, &mut suspend_work);
    }
}

pub unsafe fn pm_autosleep_state() -> suspend_state_t {
    autosleep_state
}

pub unsafe fn pm_autosleep_lock() -> c_int {
    mutex_lock_interruptible(&mut autosleep_lock)
}

pub unsafe fn pm_autosleep_unlock() {
    mutex_unlock(&mut autosleep_lock);
}

pub unsafe fn pm_autosleep_set_state(state: suspend_state_t) -> c_int {
    // CONFIG_HIBERNATION: retain the source build-time condition.
    if state >= PM_SUSPEND_MAX {
        return -EINVAL;
    }

    __pm_stay_awake(autosleep_ws);

    mutex_lock(&mut autosleep_lock);

    autosleep_state = state;

    __pm_relax(autosleep_ws);

    if state > PM_SUSPEND_ON {
        pm_wakep_autosleep_enabled(true);
        queue_up_suspend_work();
    } else {
        pm_wakep_autosleep_enabled(false);
    }

    mutex_unlock(&mut autosleep_lock);
    0
}

pub unsafe fn pm_autosleep_init() -> c_int {
    autosleep_ws = wakeup_source_register(core::ptr::null_mut(), "autosleep\0".as_ptr() as *const c_char);
    if autosleep_ws.is_null() {
        return -ENOMEM;
    }

    autosleep_wq = alloc_ordered_workqueue("autosleep\0".as_ptr() as *const c_char, 0);
    if !autosleep_wq.is_null() {
        return 0;
    }

    wakeup_source_unregister(autosleep_ws);
    -ENOMEM
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
