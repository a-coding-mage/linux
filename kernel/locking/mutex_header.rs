/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Mutexes: blocking mutual exclusion locks
 *
 * started by Ingo Molnar:
 *
 *  Copyright (C) 2004, 2005, 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 */

// This file is included only when CONFIG_PREEMPT_RT is not enabled.

/*
 * This is the control structure for tasks blocked on mutex, which resides
 * on the blocked task's kernel stack:
 */
#[repr(C)]
pub struct mutex_waiter {
    pub list: list_head,
    pub task: *mut task_struct,
    pub ww_ctx: *mut ww_acquire_ctx,
    #[cfg(feature = "CONFIG_DEBUG_MUTEXES")]
    pub magic: *mut core::ffi::c_void,
}

/*
 * @owner: contains: 'struct task_struct *' to the current lock owner,
 * NULL means not owned. Since task_struct pointers are aligned at
 * at least L1_CACHE_BYTES, we have low bits to store extra state.
 *
 * Bit0 indicates a non-empty waiter list; unlock must issue a wakeup.
 * Bit1 indicates unlock needs to hand the lock to the top-waiter
 * Bit2 indicates handoff has been done and we're waiting for pickup.
 */
pub const MUTEX_FLAG_WAITERS: libc::c_ulong = 0x01;
pub const MUTEX_FLAG_HANDOFF: libc::c_ulong = 0x02;
pub const MUTEX_FLAG_PICKUP: libc::c_ulong = 0x04;

pub const MUTEX_FLAGS: libc::c_ulong = 0x07;

/*
 * Internal helper function; C doesn't allow us to hide it :/
 *
 * DO NOT USE (outside of mutex & scheduler code).
 */
#[inline]
pub unsafe fn __mutex_owner(lock: *mut mutex) -> *mut task_struct {
    if lock.is_null() {
        return core::ptr::null_mut();
    }
    (atomic_long_read(&(*lock).owner) & !(MUTEX_FLAGS as libc::c_long)) as *mut task_struct
}

#[inline]
pub unsafe fn get_task_blocked_on(p: *mut task_struct) -> *mut mutex {
    // C: guard(raw_spinlock_irqsave)(&p->blocked_lock);
    __get_task_blocked_on(p)
}

#[cfg(feature = "CONFIG_DEBUG_MUTEXES")]
extern "C" {
    pub fn debug_mutex_lock_common(lock: *mut mutex, waiter: *mut mutex_waiter);
    pub fn debug_mutex_wake_waiter(lock: *mut mutex, waiter: *mut mutex_waiter);
    pub fn debug_mutex_free_waiter(waiter: *mut mutex_waiter);
    pub fn debug_mutex_add_waiter(
        lock: *mut mutex,
        waiter: *mut mutex_waiter,
        task: *mut task_struct,
    );
    pub fn debug_mutex_remove_waiter(
        lock: *mut mutex,
        waiter: *mut mutex_waiter,
        task: *mut task_struct,
    );
    pub fn debug_mutex_unlock(lock: *mut mutex);
    pub fn debug_mutex_init(lock: *mut mutex);
}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_lock_common(_lock: *mut mutex, _waiter: *mut mutex_waiter) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_wake_waiter(_lock: *mut mutex, _waiter: *mut mutex_waiter) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_free_waiter(_waiter: *mut mutex_waiter) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_add_waiter(
    _lock: *mut mutex,
    _waiter: *mut mutex_waiter,
    _task: *mut task_struct,
) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_remove_waiter(
    _lock: *mut mutex,
    _waiter: *mut mutex_waiter,
    _task: *mut task_struct,
) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_unlock(_lock: *mut mutex) {}

#[cfg(not(feature = "CONFIG_DEBUG_MUTEXES"))]
#[inline]
pub unsafe fn debug_mutex_init(_lock: *mut mutex) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
