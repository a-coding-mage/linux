// SPDX-License-Identifier: GPL-2.0

/*
 * Generic wait-for-completion handler;
 *
 * It differs from semaphores in that their default case is the opposite,
 * wait_for_completion default blocks whereas semaphore default non-block. The
 * interface also makes it easy to 'complete' multiple waiting threads,
 * something which isn't entirely natural for semaphores.
 *
 * But more importantly, the primitive documents the usage. Semaphores would
 * typically be used for exclusion which gives rise to priority inversion.
 * Waiting for completion is a typically sync point, but not an exclusion point.
 */

use kernel::ffi::{c_long, c_ulong};
use kernel::bindings::sched_waiting_native::*;
use kernel::bindings::sched_waiting_native::{
    completion as Completion, swait_queue as SwaitQueue,
};
use super::swait::{
    __finish_swait, __prepare_to_swait, swake_up_all_locked, swake_up_locked,
};

unsafe fn complete_with_flags(x: *mut Completion, wake_flags: i32) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*x).wait.lock, &mut flags);

    if lupos_waiting_completion_done_locked(x) != LUPOS_WAITING_UINT_MAX {
        lupos_waiting_set_completion_done_locked(x, lupos_waiting_completion_done_locked(x).wrapping_add(1));
    }
    swake_up_locked(&raw mut (*x).wait, wake_flags);
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*x).wait.lock, flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn complete_on_current_cpu(x: *mut Completion) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    complete_with_flags(x, LUPOS_WAITING_WF_CURRENT_CPU);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn complete(x: *mut Completion) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    complete_with_flags(x, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn complete_all(x: *mut Completion) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_assert_rt_threaded();
    let mut flags: c_ulong = 0;
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*x).wait.lock, &mut flags);
    lupos_waiting_set_completion_done_locked(x, LUPOS_WAITING_UINT_MAX);
    swake_up_all_locked(&raw mut (*x).wait);
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*x).wait.lock, flags);
    }
}

#[unsafe(link_section = ".sched.text")]
unsafe fn do_wait_for_common(
    x: *mut Completion,
    action: unsafe extern "C" fn(c_long) -> c_long,
    mut timeout: c_long,
    state: i32,
) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    if lupos_waiting_completion_done_locked(x) == 0 {
        let mut wait = core::mem::MaybeUninit::<SwaitQueue>::uninit();
        let wait = wait.as_mut_ptr();
        lupos_waiting_init_swait_entry(wait);
        loop {
            if lupos_waiting_signal_pending_state(state, lupos_waiting_current()) {
                timeout = -LUPOS_WAITING_ERESTARTSYS;
                break;
            }
            __prepare_to_swait(&raw mut (*x).wait, wait);
            lupos_waiting_completion_wait_state(state);
            lupos_waiting_raw_spin_unlock_irq(&raw mut (*x).wait.lock);
            timeout = action(timeout);
            lupos_waiting_raw_spin_lock_irq(&raw mut (*x).wait.lock);
            if lupos_waiting_completion_done_locked(x) != 0 || timeout == 0 { break; }
        }
        __finish_swait(&raw mut (*x).wait, wait);
        if lupos_waiting_completion_done_locked(x) == 0 { return timeout; }
    }
    if lupos_waiting_completion_done_locked(x) != LUPOS_WAITING_UINT_MAX { lupos_waiting_set_completion_done_locked(x, lupos_waiting_completion_done_locked(x).wrapping_sub(1)); }
    if timeout != 0 { timeout } else { 1 }
    }
}

#[unsafe(link_section = ".sched.text")]
unsafe fn __wait_for_common(
    x: *mut Completion,
    action: unsafe extern "C" fn(c_long) -> c_long,
    timeout: c_long,
    state: i32,
) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_might_sleep();
    lupos_waiting_complete_acquire(x);
    lupos_waiting_raw_spin_lock_irq(&raw mut (*x).wait.lock);
    let timeout = do_wait_for_common(x, action, timeout, state);
    lupos_waiting_raw_spin_unlock_irq(&raw mut (*x).wait.lock);
    lupos_waiting_complete_release(x);
    timeout
    }
}

#[unsafe(link_section = ".sched.text")]
unsafe fn wait_for_common(x: *mut Completion, timeout: c_long, state: i32) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    __wait_for_common(x, schedule_timeout, timeout, state)
    }
}

#[unsafe(link_section = ".sched.text")]
unsafe fn wait_for_common_io(x: *mut Completion, timeout: c_long, state: i32) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    __wait_for_common(x, io_schedule_timeout, timeout, state)
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion(x: *mut Completion) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common(x, LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT, LUPOS_WAITING_TASK_UNINTERRUPTIBLE);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_timeout(x: *mut Completion, timeout: c_ulong) -> c_ulong {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common(x, timeout as c_long, LUPOS_WAITING_TASK_UNINTERRUPTIBLE) as c_ulong
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_io(x: *mut Completion) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common_io(x, LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT, LUPOS_WAITING_TASK_UNINTERRUPTIBLE);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_io_timeout(x: *mut Completion, timeout: c_ulong) -> c_ulong {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common_io(x, timeout as c_long, LUPOS_WAITING_TASK_UNINTERRUPTIBLE) as c_ulong
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_interruptible(x: *mut Completion) -> i32 {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let t = wait_for_common(x, LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT, LUPOS_WAITING_TASK_INTERRUPTIBLE);
    if t == -LUPOS_WAITING_ERESTARTSYS { t as i32 } else { 0 }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_interruptible_timeout(x: *mut Completion, timeout: c_ulong) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common(x, timeout as c_long, LUPOS_WAITING_TASK_INTERRUPTIBLE)
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_killable(x: *mut Completion) -> i32 {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let t = wait_for_common(x, LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT, LUPOS_WAITING_TASK_KILLABLE);
    if t == -LUPOS_WAITING_ERESTARTSYS { t as i32 } else { 0 }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_state(x: *mut Completion, state: u32) -> i32 {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let t = wait_for_common(x, LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT, state as i32);
    if t == -LUPOS_WAITING_ERESTARTSYS { t as i32 } else { 0 }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn wait_for_completion_killable_timeout(x: *mut Completion, timeout: c_ulong) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    wait_for_common(x, timeout as c_long, LUPOS_WAITING_TASK_KILLABLE)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn try_wait_for_completion(x: *mut Completion) -> bool {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    let mut ret = true;
    if lupos_waiting_read_completion_done(x) == 0 { return false; }
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*x).wait.lock, &mut flags);
    if lupos_waiting_completion_done_locked(x) == 0 { ret = false; }
    else if lupos_waiting_completion_done_locked(x) != LUPOS_WAITING_UINT_MAX { lupos_waiting_set_completion_done_locked(x, lupos_waiting_completion_done_locked(x).wrapping_sub(1)); }
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*x).wait.lock, flags);
    ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn completion_done(x: *mut Completion) -> bool {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    if lupos_waiting_read_completion_done(x) == 0 { return false; }
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*x).wait.lock, &mut flags);
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*x).wait.lock, flags);
    true
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
