// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/freezer.c - Function to freeze a process
 * Originally from kernel/power/process.c
 */
//! Continuation of the existing freezer Rust decisions over native storage.
#![no_std]
#![cfg(CONFIG_FREEZER)]

compile_error!("SOURCE ONLY HOLD: freezer native ABI and state semantics are unqualified");

use kernel::bindings::freezer_native as b;
use kernel::ffi::{c_int, c_uint, c_ulong, c_void};
use b::task_struct;

// One actual native static key, two native booleans, and one initialized lock.
// These declarations do not create replacement Rust storage or layout copies.
pub use b::{freezer_active, pm_freezing, pm_nosig_freezing};

/// # Safety
/// p is live; the caller retains the original freezer/task-state protection.
#[no_mangle]
pub unsafe extern "C" fn freezing_slow_path(p: *mut task_struct) -> bool {
    // SAFETY: Native leaves preserve the original plain reads and inline policy.
    unsafe {
        if b::lupos_freezer_task_flags(p) & (b::LUPOS_FREEZER_PF_NOFREEZE | b::LUPOS_FREEZER_PF_SUSPEND_TASK) != 0 {
            return false;
        }
        if b::lupos_freezer_oom_victim(p) {
            return false;
        }
        if b::lupos_freezer_pm_nosig() || b::lupos_freezer_cgroup_freezing(p) {
            return true;
        }
        if b::lupos_freezer_pm() && b::lupos_freezer_task_flags(p) & b::LUPOS_FREEZER_PF_KTHREAD == 0 {
            return true;
        }
        false
    }
}

/// # Safety
/// p is live; this observation alone does not grant a task-state lifetime lock.
#[no_mangle]
pub unsafe extern "C" fn frozen(p: *mut task_struct) -> bool {
    // SAFETY: The native READ_ONCE retains the actual task-state access contract.
    unsafe { b::lupos_freezer_state_once(p) & b::LUPOS_FREEZER_TASK_FROZEN != 0 }
}

/// # Safety
/// Called in the original sleepable refrigerator context with current alive.
#[no_mangle]
pub unsafe extern "C" fn __refrigerator(check_kthr_stop: bool) -> bool {
    // SAFETY: Every lock/state/scheduling boundary retains its native primitive.
    unsafe {
        let state: c_uint = b::lupos_freezer_current_state();
        let mut was_frozen = false;
        b::lupos_freezer_debug_enter();
        b::lupos_freezer_warn_refrigerator(state);
        loop {
            let current = b::lupos_freezer_current();
            b::lupos_freezer_pi_lock_irq(current);
            b::lupos_freezer_write_state(current, b::LUPOS_FREEZER_TASK_FROZEN);
            b::lupos_freezer_write_saved(current, b::LUPOS_FREEZER_TASK_RUNNING);
            b::lupos_freezer_pi_unlock_irq(current);

            let lock = b::lupos_freezer_lock();
            b::lupos_freezer_spin_lock_irq(lock);
            let freeze = b::lupos_freezer_freezing(b::lupos_freezer_current())
                && !(check_kthr_stop && b::lupos_freezer_kthread_should_stop());
            b::lupos_freezer_spin_unlock_irq(lock);
            if !freeze {
                break;
            }
            was_frozen = true;
            b::lupos_freezer_schedule();
        }
        b::lupos_freezer_current_running();
        b::lupos_freezer_debug_leave();
        was_frozen
    }
}

// SAFETY: p remains live, and the caller holds the original freezer protection.
unsafe fn fake_signal_wake_up(p: *mut task_struct) {
    // SAFETY: Only a successful native sighand acquisition owns saved flags.
    unsafe {
        let mut flags: c_ulong = 0;
        if !b::lupos_freezer_lock_sighand(p, &mut flags).is_null() {
            b::lupos_freezer_signal_wake(p);
            b::lupos_freezer_unlock_sighand(p, &mut flags);
        }
    }
}

// SAFETY: Invoked synchronously by native task_call_func under its protection.
#[export_name = "lupos_freezer_set_task_frozen"]
unsafe extern "C" fn __set_task_frozen(p: *mut task_struct, _arg: *mut c_void) -> c_int {
    // SAFETY: Native task_call_func pins p and protects scheduler/task state.
    unsafe {
        let state: c_uint = b::lupos_freezer_state_once(p);
        // Delayed runnable entities retain the actual task_is_runnable predicate.
        if b::lupos_freezer_task_runnable(p) {
            return 0;
        }
        if p != b::lupos_freezer_current() && b::lupos_freezer_task_curr(p) {
            return 0;
        }
        if state & (b::LUPOS_FREEZER_TASK_FREEZABLE | b::LUPOS_FREEZER_TASK_STOPPED | b::LUPOS_FREEZER_TASK_TRACED) == 0 {
            return 0;
        }
        if state & b::LUPOS_FREEZER_TASK_FREEZABLE != 0 {
            b::lupos_freezer_warn_freezable(state);
        }
        #[cfg(CONFIG_LOCKDEP)]
        if state & b::LUPOS_FREEZER_TASK_FREEZABLE_UNSAFE == 0 {
            b::lupos_freezer_warn_locks(p);
        }
        // Preserve the original second, plain __state read for saved_state.
        b::lupos_freezer_save_plain_state(p);
        b::lupos_freezer_write_state(p, b::LUPOS_FREEZER_TASK_FROZEN);
        b::LUPOS_FREEZER_TASK_FROZEN as c_int
    }
}

// SAFETY: p is live; the native task_call_func supplies the callback exclusion.
unsafe fn __freeze_task(p: *mut task_struct) -> bool {
    // SAFETY: The fixed native callback signature reaches the sole Rust body.
    unsafe { b::lupos_freezer_call_set_frozen(p) != 0 }
}

/// # Safety
/// p remains live throughout the original task freezer protocol.
#[no_mangle]
pub unsafe extern "C" fn freeze_task(p: *mut task_struct) -> bool {
    // SAFETY: The real initialized lock and native IRQ-save protocol protect p.
    unsafe {
        let lock = b::lupos_freezer_lock();
        let flags = b::lupos_freezer_spin_lock_irqsave(lock);
        if !b::lupos_freezer_freezing(p) || frozen(p) || __freeze_task(p) {
            b::lupos_freezer_spin_unlock_irqrestore(lock, flags);
            return false;
        }
        if b::lupos_freezer_task_flags(p) & b::LUPOS_FREEZER_PF_KTHREAD == 0 {
            fake_signal_wake_up(p);
        } else {
            b::lupos_freezer_wake_state(p, b::LUPOS_FREEZER_TASK_NORMAL);
        }
        b::lupos_freezer_spin_unlock_irqrestore(lock, flags);
        true
    }
}

// SAFETY: Invoked synchronously by task_call_func with task state protected.
#[export_name = "lupos_freezer_restore_state"]
unsafe extern "C" fn __restore_freezer_state(p: *mut task_struct, _arg: *mut c_void) -> c_int {
    // SAFETY: The plain saved read/store and ONCE task-state write stay distinct.
    unsafe {
        let state = b::lupos_freezer_saved_state(p);
        if state != b::LUPOS_FREEZER_TASK_RUNNING {
            b::lupos_freezer_write_state(p, state);
            b::lupos_freezer_write_saved(p, b::LUPOS_FREEZER_TASK_RUNNING);
            return 1;
        }
        0
    }
}

/// # Safety
/// p is live under the caller's original task/freezer lifetime contract.
#[no_mangle]
pub unsafe extern "C" fn __thaw_task(p: *mut task_struct) {
    // SAFETY: Retain this tree's guard operations and its sole exit path.
    // guard(spinlock_irqsave) uses refcounted IRQ disable/enable on !RT.
    unsafe {
        let lock = b::lupos_freezer_lock();
        b::lupos_freezer_spin_lock_irq_disable(lock);
        if frozen(p) && b::lupos_freezer_call_restore(p) == 0 {
            b::lupos_freezer_wake_state(p, b::LUPOS_FREEZER_TASK_FROZEN);
        }
        b::lupos_freezer_spin_unlock_irq_enable(lock);
    }
}

/// # Safety
/// p and its signal/thread list are live; native RCU protects traversal.
#[no_mangle]
pub unsafe extern "C" fn thaw_process(p: *mut task_struct) {
    // SAFETY: No native task reference is manufactured or retained beyond RCU.
    unsafe {
        b::lupos_freezer_rcu_read_lock();
        let mut t = b::lupos_freezer_first_thread(p);
        while !t.is_null() {
            __thaw_task(t);
            t = b::lupos_freezer_next_thread(p, t);
        }
        b::lupos_freezer_rcu_read_unlock();
    }
}

/// # Safety
/// Current may sleep and satisfies the original try_to_freeze caller contract.
#[no_mangle]
pub unsafe extern "C" fn set_freezable() -> bool {
    // SAFETY: Flag mutation stays under freezer_lock; freezing is checked after.
    unsafe {
        b::lupos_freezer_might_sleep();
        let lock = b::lupos_freezer_lock();
        b::lupos_freezer_spin_lock_irq(lock);
        b::lupos_freezer_clear_nofreeze();
        b::lupos_freezer_spin_unlock_irq(lock);
        b::lupos_freezer_try_to_freeze()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Compatibility authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
