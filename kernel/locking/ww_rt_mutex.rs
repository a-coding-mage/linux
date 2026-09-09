// SPDX-License-Identifier: GPL-2.0-only
/*
 * rtmutex API
 */

// C build configuration:
// #define RT_MUTEX_BUILD_MUTEX
// #define WW_RT
// The implementation dependencies supplied by rtmutex.c are external here.

pub unsafe fn ww_mutex_trylock(
    lock: *mut ww_mutex,
    ww_ctx: *mut ww_acquire_ctx,
) -> i32 {
    let rtm: *mut rt_mutex = unsafe { &mut (*lock).base };

    if ww_ctx.is_null() {
        return unsafe { rt_mutex_trylock(rtm) };
    }

    /*
     * Reset the wounded flag after a kill. No other process can
     * race and wound us here, since they can't have a valid owner
     * pointer if we don't have any locks held.
     */
    if unsafe { (*ww_ctx).acquired == 0 } {
        unsafe { (*ww_ctx).wounded = 0 };
    }

    if unsafe { __rt_mutex_trylock(&mut (*rtm).rtmutex) } != 0 {
        unsafe {
            ww_mutex_set_context_fastpath(lock, ww_ctx);
            mutex_acquire_nest(
                &mut (*rtm).dep_map,
                0,
                1,
                &mut (*ww_ctx).dep_map,
                _RET_IP_(),
            );
        }
        return 1;
    }

    0
}

pub unsafe fn __ww_rt_mutex_lock(
    lock: *mut ww_mutex,
    ww_ctx: *mut ww_acquire_ctx,
    state: u32,
    ip: u64,
) -> i32 {
    let mut nest_lock: *mut lockdep_map = core::ptr::null_mut();
    let rtm: *mut rt_mutex = unsafe { &mut (*lock).base };
    let ret: i32;

    unsafe { might_sleep() };

    if !ww_ctx.is_null() {
        if unsafe { ww_ctx == READ_ONCE((*lock).ctx) } {
            return -EALREADY;
        }

        /*
         * Reset the wounded flag after a kill. No other process can
         * race and wound us here, since they can't have a valid owner
         * pointer if we don't have any locks held.
         */
        if unsafe { (*ww_ctx).acquired == 0 } {
            unsafe { (*ww_ctx).wounded = 0 };
        }

        // CONFIG_DEBUG_LOCK_ALLOC
        nest_lock = unsafe { &mut (*ww_ctx).dep_map };
    }

    unsafe { mutex_acquire_nest(&mut (*rtm).dep_map, 0, 0, nest_lock, ip) };

    if unsafe { rt_mutex_try_acquire(&mut (*rtm).rtmutex) } {
        if !ww_ctx.is_null() {
            unsafe { ww_mutex_set_context_fastpath(lock, ww_ctx) };
        }
        return 0;
    }

    ret = unsafe { rt_mutex_slowlock(&mut (*rtm).rtmutex, ww_ctx, state) };

    if ret != 0 {
        unsafe { mutex_release(&mut (*rtm).dep_map, ip) };
    }
    ret
}

pub unsafe fn ww_mutex_lock(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> i32 {
    unsafe { __ww_rt_mutex_lock(lock, ctx, TASK_UNINTERRUPTIBLE, _RET_IP_()) }
}

pub unsafe fn ww_mutex_lock_interruptible(
    lock: *mut ww_mutex,
    ctx: *mut ww_acquire_ctx,
) -> i32 {
    unsafe { __ww_rt_mutex_lock(lock, ctx, TASK_INTERRUPTIBLE, _RET_IP_()) }
}

pub unsafe fn ww_mutex_unlock(lock: *mut ww_mutex) {
    let rtm: *mut rt_mutex = unsafe { &mut (*lock).base };

    unsafe {
        __ww_mutex_unlock(lock);
        mutex_release(&mut (*rtm).dep_map, _RET_IP_());
        __rt_mutex_unlock(&mut (*rtm).rtmutex);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
