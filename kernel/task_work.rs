// SPDX-License-Identifier: GPL-2.0
// Continued from Rust blob 9eaf9999263e3bdd98b6ff5c8207170321c450ad.
// Native oracle: task_work.c at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
#![no_std]

compile_error!("SOURCE ONLY HOLD: task work is not admitted");

// Proposed configured declarations; no generated or hand-projected layouts.
use kernel::bindings::task_work_native::*;
use kernel::ffi::{c_int, c_void};

#[cfg(CONFIG_IRQ_WORK)]
#[export_name = "lupos_task_work_set_notify_irq"]
unsafe extern "C" fn task_work_set_notify_irq(_entry: *mut irq_work) {
    /*
     * no-op IPI
     *
     * TWA_NMI_CURRENT will already have set the TIF flag, all
     * this interrupt does it tickle the return-to-user path.
     */
}

/// Queue a caller-owned callback and request the selected notification.
///
/// # Safety
/// The task and work satisfy the original task_work_add lifetime and context
/// contract. The work is not already queued; its callback remains callable.
#[no_mangle]
pub unsafe extern "C" fn task_work_add(
    task: *mut task_struct,
    work: *mut callback_head,
    notify: task_work_notify_mode,
) -> c_int {
    // SAFETY: Native leaves own the configured fields, primitives and storage.
    // Only local expected pointers are passed by mutable address to cmpxchg.
    unsafe {
        let mut head: *mut callback_head;

        if notify.0 == TWA_NMI_CURRENT.0 {
            if lupos_task_work_warn_not_current(task) {
                return -(LUPOS_TASK_WORK_EINVAL as c_int);
            }
            #[cfg(not(CONFIG_IRQ_WORK))]
            {
                return -(LUPOS_TASK_WORK_EINVAL as c_int);
            }
        } else {
            lupos_task_work_record_aux_stack(work);
        }

        let task_works = lupos_task_work_head_slot(task);
        head = lupos_task_work_read_once(task_works);
        loop {
            if lupos_task_work_is_exited(head) {
                return -(LUPOS_TASK_WORK_ESRCH as c_int);
            }
            lupos_task_work_set_next(work, head);
            if lupos_task_work_try_cmpxchg(task_works, &raw mut head, work) {
                break;
            }
            // Failure updated head in place; do not substitute another load.
        }

        match notify.0 {
            mode if mode == TWA_NONE.0 => {}
            mode if mode == TWA_RESUME.0 => lupos_task_work_notify_resume(task),
            mode if mode == TWA_SIGNAL.0 => lupos_task_work_notify_signal(task),
            mode if mode == TWA_SIGNAL_NO_IPI.0 => {
                lupos_task_work_notify_signal_no_ipi(task);
            }
            #[cfg(CONFIG_IRQ_WORK)]
            mode if mode == TWA_NMI_CURRENT.0 => {
                lupos_task_work_notify_nmi_current();
            }
            _ => lupos_task_work_warn_unknown_notify(),
        }

        0
    }
}

/// Remove the first pending callback for which the supplied matcher succeeds.
///
/// # Safety
/// The task, matcher, data and callback lifetimes obey the original C contract.
/// The native matcher context remains live for this synchronous call; its
/// stored callback may be NULL only when no invocation is reached.
/// A matcher actually reached during traversal must be callable in this IRQ-
/// disabled, pi_lock-held context. No references to shared links are formed.
#[export_name = "lupos_task_work_cancel_match"]
pub unsafe extern "C" fn task_work_cancel_match(
    task: *mut task_struct,
    r#match: *const lupos_task_work_match_ctx,
    data: *mut c_void,
) -> *mut callback_head {
    // SAFETY: The native lock protects cancellation and synchronizes with run.
    // A failed CAS may replace work with a new head, NULL or the exit sentinel.
    unsafe {
        let mut pprev: *mut *mut callback_head = lupos_task_work_head_slot(task);
        let mut work: *mut callback_head;

        if lupos_task_work_likely_not_pending(task) {
            return core::ptr::null_mut();
        }
        let flags = lupos_task_work_lock_irqsave(task);
        work = lupos_task_work_read_once(pprev);
        while !work.is_null() {
            if !lupos_task_work_call_match(r#match, work, data) {
                pprev = lupos_task_work_next_slot(work);
                work = lupos_task_work_read_once(pprev);
            } else {
                let next = lupos_task_work_next(work);
                if lupos_task_work_try_cmpxchg(pprev, &raw mut work, next) {
                    break;
                }
                // Keep pprev: work now contains the failed CAS observation.
            }
        }
        lupos_task_work_unlock_irqrestore(task, flags);
        work
    }
}

#[export_name = "lupos_task_work_func_match"]
unsafe extern "C" fn task_work_func_match(cb: *mut callback_head, data: *mut c_void) -> bool {
    // SAFETY: Called under cancel_match's protection; native function-pointer
    // equality also preserves C's NULL callback/data comparison.
    unsafe { lupos_task_work_func_matches(cb, data) }
}

/// Cancel the last queued pending work whose callback equals func.
///
/// # Safety
/// The task and work lifetimes satisfy task_work_cancel_match's C contract.
/// The native public entry point passes func through the original C
/// function-pointer-to-void-pointer conversion, including NULL.
#[export_name = "lupos_task_work_cancel_func"]
pub unsafe extern "C" fn task_work_cancel_func(
    task: *mut task_struct,
    func: *mut c_void,
) -> *mut callback_head {
    // SAFETY: Native static storage holds the retained Rust matcher. The
    // opaque data preserves the native conversion without a Rust fn cast.
    unsafe { task_work_cancel_match(task, lupos_task_work_func_match_context(), func) }
}

#[export_name = "lupos_task_work_match"]
unsafe extern "C" fn task_work_match(cb: *mut callback_head, data: *mut c_void) -> bool {
    cb == data as *mut callback_head
}

/// Cancel the supplied callback if it is still pending on the task.
///
/// # Safety
/// The task and callback satisfy the original cancellation lifetime contract.
#[no_mangle]
pub unsafe extern "C" fn task_work_cancel(task: *mut task_struct, cb: *mut callback_head) -> bool {
    // SAFETY: The matcher compares pointers only and uses the protected walk.
    unsafe {
        task_work_cancel_match(task, lupos_task_work_match_context(), cb.cast()) == cb
    }
}

/// Drain current's pending work, installing the exit sentinel only when empty.
///
/// # Safety
/// Called for current in the original task_work_run context with IRQs enabled.
/// Each detached callback is callable and follows the native lifetime contract.
#[no_mangle]
pub unsafe extern "C" fn task_work_run() {
    // SAFETY: The successful detach and pi_lock round trip provide the original
    // exclusion from cancel. Read next before a callback can consume its work.
    unsafe {
        let task = lupos_task_work_current();
        let task_works = lupos_task_work_head_slot(task);
        let mut work: *mut callback_head;
        let mut head: *mut callback_head;
        let mut next: *mut callback_head;

        loop {
            work = lupos_task_work_read_once(task_works);
            loop {
                head = core::ptr::null_mut();
                if work.is_null() {
                    if lupos_task_work_task_exiting(task) {
                        head = lupos_task_work_exited();
                    } else {
                        break;
                    }
                }
                if lupos_task_work_try_cmpxchg(task_works, &raw mut work, head) {
                    break;
                }
                // Failure updated work, including a newly queued callback.
            }
            if work.is_null() {
                break;
            }
            lupos_task_work_lock_irq(task);
            lupos_task_work_unlock_irq(task);
            loop {
                next = lupos_task_work_next(work);
                lupos_task_work_invoke(work);
                work = next;
                lupos_task_work_cond_resched();
                if work.is_null() {
                    break;
                }
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
