/*
 * Debugging code for mutexes
 *
 * Started by Ingo Molnar:
 *
 *  Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 *
 * lock debugging, locking tree, deadlock detection started by:
 *
 *  Copyright (C) 2004, LynuxWorks, Inc., Igor Manyilov, Bill Huey
 *  Released under the General Public License (GPL).
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Must be called with lock->wait_lock held.
 */
pub unsafe fn debug_mutex_lock_common(
    lock: *mut mutex,
    waiter: *mut mutex_waiter,
) {
    let _ = lock;
    memset(waiter as *mut core::ffi::c_void, MUTEX_DEBUG_INIT, core::mem::size_of::<mutex_waiter>());
    (*waiter).magic = waiter;
    INIT_LIST_HEAD(&mut (*waiter).list);
    (*waiter).ww_ctx = MUTEX_POISON_WW_CTX;
}

pub unsafe fn debug_mutex_wake_waiter(
    lock: *mut mutex,
    waiter: *mut mutex_waiter,
) {
    lockdep_assert_held(&mut (*lock).wait_lock);
    DEBUG_LOCKS_WARN_ON(!(*lock).first_waiter);
    DEBUG_LOCKS_WARN_ON((*waiter).magic != waiter);
}

pub unsafe fn debug_mutex_free_waiter(waiter: *mut mutex_waiter) {
    DEBUG_LOCKS_WARN_ON(!list_empty(&mut (*waiter).list));
    memset(waiter as *mut core::ffi::c_void, MUTEX_DEBUG_FREE, core::mem::size_of::<mutex_waiter>());
}

pub unsafe fn debug_mutex_add_waiter(
    lock: *mut mutex,
    waiter: *mut mutex_waiter,
    task: *mut task_struct,
) {
    lockdep_assert_held(&mut (*lock).wait_lock);

    /* Current thread can't be already blocked (since it's executing!) */
    DEBUG_LOCKS_WARN_ON(!get_task_blocked_on(task).is_null());
    let _ = waiter;
}

pub unsafe fn debug_mutex_remove_waiter(
    lock: *mut mutex,
    waiter: *mut mutex_waiter,
    task: *mut task_struct,
) {
    let blocked_on = get_task_blocked_on(task);

    DEBUG_LOCKS_WARN_ON((*waiter).task != task);
    DEBUG_LOCKS_WARN_ON(!blocked_on.is_null() && blocked_on != lock);

    INIT_LIST_HEAD(&mut (*waiter).list);
    (*waiter).task = core::ptr::null_mut();
}

pub unsafe fn debug_mutex_unlock(lock: *mut mutex) {
    if likely(debug_locks) {
        DEBUG_LOCKS_WARN_ON((*lock).magic != lock);
    }
}

pub unsafe fn debug_mutex_init(lock: *mut mutex) {
    (*lock).magic = lock;
}

unsafe fn devm_mutex_release(res: *mut core::ffi::c_void) {
    mutex_destroy(res as *mut mutex);
}

pub unsafe fn __devm_mutex_init(dev: *mut device, lock: *mut mutex) -> i32 {
    devm_add_action_or_reset(dev, devm_mutex_release, lock as *mut core::ffi::c_void)
}

// EXPORT_SYMBOL_GPL(__devm_mutex_init);

/**
 * mutex_destroy - mark a mutex unusable
 * @lock: the mutex to be destroyed
 *
 * This function marks the mutex uninitialized, and any subsequent
 * use of the mutex is forbidden. The mutex must not be locked when
 * this function is called.
 */
pub unsafe fn mutex_destroy(lock: *mut mutex) {
    DEBUG_LOCKS_WARN_ON(mutex_is_locked(lock));
    (*lock).magic = core::ptr::null_mut();
}

// EXPORT_SYMBOL_GPL(mutex_destroy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
