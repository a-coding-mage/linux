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

// Kernel declarations supplied by the surrounding translation unit.
extern "C" {
    fn swake_up_locked(wait: *mut SwaitQueueHead, wake_flags: i32);
    fn swake_up_all_locked(wait: *mut SwaitQueueHead);
    fn signal_pending_state(state: i32, task: *mut TaskStruct) -> bool;
    fn __prepare_to_swait(wait: *mut SwaitQueueHead, waiter: *mut SwaitQueue);
    fn __set_current_state(state: i32);
    fn raw_spin_unlock_irq(lock: *mut RawSpinlock);
    fn raw_spin_lock_irq(lock: *mut RawSpinlock);
    fn raw_spin_lock_irqsave(lock: *mut RawSpinlock, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinlock, flags: c_ulong);
    fn schedule_timeout(timeout: c_long) -> c_long;
    fn io_schedule_timeout(timeout: c_long) -> c_long;
    fn might_sleep();
    fn complete_acquire(x: *mut Completion);
    fn complete_release(x: *mut Completion);
    fn __finish_swait(wait: *mut SwaitQueueHead, waiter: *mut SwaitQueue);
}

type c_long = i64;
type c_ulong = u64;

#[repr(C)]
pub struct RawSpinlock { _private: [u8; 0] }
#[repr(C)]
pub struct SwaitQueue { _private: [u8; 0] }
#[repr(C)]
pub struct TaskStruct { _private: [u8; 0] }
#[repr(C)]
pub struct SwaitQueueHead { pub lock: RawSpinlock, _private: [u8; 0] }
#[repr(C)]
pub struct Completion { pub wait: SwaitQueueHead, pub done: u32 }

const UINT_MAX: u32 = u32::MAX;
const WF_CURRENT_CPU: i32 = 1;
const ERESTARTSYS: c_long = 512;
const MAX_SCHEDULE_TIMEOUT: c_long = c_long::MAX;
const TASK_UNINTERRUPTIBLE: i32 = 2;
const TASK_INTERRUPTIBLE: i32 = 1;
const TASK_KILLABLE: i32 = 4;

extern "C" {
    static mut current: *mut TaskStruct;
}

unsafe fn complete_with_flags(x: *mut Completion, wake_flags: i32) {
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*x).wait.lock, &mut flags);

    if (*x).done != UINT_MAX {
        (*x).done = (*x).done.wrapping_add(1);
    }
    swake_up_locked(&mut (*x).wait, wake_flags);
    raw_spin_unlock_irqrestore(&mut (*x).wait.lock, flags);
}

pub unsafe fn complete_on_current_cpu(x: *mut Completion) {
    complete_with_flags(x, WF_CURRENT_CPU);
}

pub unsafe fn complete(x: *mut Completion) {
    complete_with_flags(x, 0);
}

pub unsafe fn complete_all(x: *mut Completion) {
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*x).wait.lock, &mut flags);
    (*x).done = UINT_MAX;
    swake_up_all_locked(&mut (*x).wait);
    raw_spin_unlock_irqrestore(&mut (*x).wait.lock, flags);
}

unsafe fn do_wait_for_common(
    x: *mut Completion,
    action: unsafe extern "C" fn(c_long) -> c_long,
    mut timeout: c_long,
    state: i32,
) -> c_long {
    if (*x).done == 0 {
        let mut wait: SwaitQueue = core::mem::zeroed();
        loop {
            if signal_pending_state(state, current) {
                timeout = -ERESTARTSYS;
                break;
            }
            __prepare_to_swait(&mut (*x).wait, &mut wait);
            __set_current_state(state);
            raw_spin_unlock_irq(&mut (*x).wait.lock);
            timeout = action(timeout);
            raw_spin_lock_irq(&mut (*x).wait.lock);
            if (*x).done != 0 || timeout == 0 { break; }
        }
        __finish_swait(&mut (*x).wait, &mut wait);
        if (*x).done == 0 { return timeout; }
    }
    if (*x).done != UINT_MAX { (*x).done = (*x).done.wrapping_sub(1); }
    if timeout != 0 { timeout } else { 1 }
}

unsafe fn __wait_for_common(
    x: *mut Completion,
    action: unsafe extern "C" fn(c_long) -> c_long,
    timeout: c_long,
    state: i32,
) -> c_long {
    might_sleep();
    complete_acquire(x);
    raw_spin_lock_irq(&mut (*x).wait.lock);
    let timeout = do_wait_for_common(x, action, timeout, state);
    raw_spin_unlock_irq(&mut (*x).wait.lock);
    complete_release(x);
    timeout
}

unsafe fn wait_for_common(x: *mut Completion, timeout: c_long, state: i32) -> c_long {
    __wait_for_common(x, schedule_timeout, timeout, state)
}

unsafe fn wait_for_common_io(x: *mut Completion, timeout: c_long, state: i32) -> c_long {
    __wait_for_common(x, io_schedule_timeout, timeout, state)
}

pub unsafe fn wait_for_completion(x: *mut Completion) {
    wait_for_common(x, MAX_SCHEDULE_TIMEOUT, TASK_UNINTERRUPTIBLE);
}

pub unsafe fn wait_for_completion_timeout(x: *mut Completion, timeout: c_ulong) -> c_ulong {
    wait_for_common(x, timeout as c_long, TASK_UNINTERRUPTIBLE) as c_ulong
}

pub unsafe fn wait_for_completion_io(x: *mut Completion) {
    wait_for_common_io(x, MAX_SCHEDULE_TIMEOUT, TASK_UNINTERRUPTIBLE);
}

pub unsafe fn wait_for_completion_io_timeout(x: *mut Completion, timeout: c_ulong) -> c_ulong {
    wait_for_common_io(x, timeout as c_long, TASK_UNINTERRUPTIBLE) as c_ulong
}

pub unsafe fn wait_for_completion_interruptible(x: *mut Completion) -> i32 {
    let t = wait_for_common(x, MAX_SCHEDULE_TIMEOUT, TASK_INTERRUPTIBLE);
    if t == -ERESTARTSYS { t as i32 } else { 0 }
}

pub unsafe fn wait_for_completion_interruptible_timeout(x: *mut Completion, timeout: c_ulong) -> c_long {
    wait_for_common(x, timeout as c_long, TASK_INTERRUPTIBLE)
}

pub unsafe fn wait_for_completion_killable(x: *mut Completion) -> i32 {
    let t = wait_for_common(x, MAX_SCHEDULE_TIMEOUT, TASK_KILLABLE);
    if t == -ERESTARTSYS { t as i32 } else { 0 }
}

pub unsafe fn wait_for_completion_state(x: *mut Completion, state: u32) -> i32 {
    let t = wait_for_common(x, MAX_SCHEDULE_TIMEOUT, state as i32);
    if t == -ERESTARTSYS { t as i32 } else { 0 }
}

pub unsafe fn wait_for_completion_killable_timeout(x: *mut Completion, timeout: c_ulong) -> c_long {
    wait_for_common(x, timeout as c_long, TASK_KILLABLE)
}

pub unsafe fn try_wait_for_completion(x: *mut Completion) -> bool {
    let mut flags: c_ulong = 0;
    let mut ret = true;
    if core::ptr::read_volatile(&(*x).done) == 0 { return false; }
    raw_spin_lock_irqsave(&mut (*x).wait.lock, &mut flags);
    if (*x).done == 0 { ret = false; }
    else if (*x).done != UINT_MAX { (*x).done = (*x).done.wrapping_sub(1); }
    raw_spin_unlock_irqrestore(&mut (*x).wait.lock, flags);
    ret
}

pub unsafe fn completion_done(x: *mut Completion) -> bool {
    let mut flags: c_ulong = 0;
    if core::ptr::read_volatile(&(*x).done) == 0 { return false; }
    raw_spin_lock_irqsave(&mut (*x).wait.lock, &mut flags);
    raw_spin_unlock_irqrestore(&mut (*x).wait.lock, flags);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
