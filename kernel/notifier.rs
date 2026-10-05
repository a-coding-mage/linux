// SPDX-License-Identifier: GPL-2.0-only
// Continued from the existing notifier.rs at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
#![no_std]

compile_error!("SOURCE ONLY HOLD: notifier ABI and instrumentation are not qualified");

use kernel::bindings::notifier_native::*;
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};

/*
 * Notifier chain core routines. The exported routines below are layered on
 * top of these, with appropriate locking added. Native leaves retain actual
 * configured fields, primitives, diagnostics and function-pointer operations.
 */

/// # Safety
/// The caller protects list mutation and supplies live native blocks. The new
/// block is owned until publication; registered blocks remain live as required
/// by the original chain contract.
unsafe fn notifier_chain_register(
    mut nl: *mut *mut notifier_block,
    n: *mut notifier_block,
    unique_priority: bool,
) -> c_int {
    // SAFETY: The caller's writer protection covers the native slot accesses.
    // Loads remain at their original expression sites; no shared Rust reference
    // or Rust value of the potentially invalid native callback is constructed.
    unsafe {
        while !lupos_notifier_load(nl).is_null() {
            if lupos_notifier_duplicate(nl, n) {
                lupos_notifier_warn_duplicate(n);
                return -(LUPOS_NOTIFIER_EEXIST as c_int);
            }
            if lupos_notifier_priority(n)
                > lupos_notifier_priority(lupos_notifier_load(nl))
            {
                break;
            }
            if lupos_notifier_priority(n)
                == lupos_notifier_priority(lupos_notifier_load(nl))
                && unique_priority
            {
                return -(LUPOS_NOTIFIER_EBUSY as c_int);
            }
            nl = lupos_notifier_next_slot(lupos_notifier_load(nl));
        }
        lupos_notifier_set_next(n, lupos_notifier_load(nl));
        lupos_notifier_assign(nl, n);
        lupos_notifier_trace_register(n);
        0
    }
}

/// # Safety
/// The caller protects mutation and keeps all inspected native slots alive.
unsafe fn notifier_chain_unregister(
    mut nl: *mut *mut notifier_block,
    n: *mut notifier_block,
) -> c_int {
    // SAFETY: Writer protection covers each slot; only a found block's next is
    // read. Reclamation remains the caller's responsibility after unlinking.
    unsafe {
        while !lupos_notifier_load(nl).is_null() {
            if lupos_notifier_load(nl) == n {
                lupos_notifier_assign(nl, lupos_notifier_next(n));
                lupos_notifier_trace_unregister(n);
                return 0;
            }
            nl = lupos_notifier_next_slot(lupos_notifier_load(nl));
        }
        -(LUPOS_NOTIFIER_ENOENT as c_int)
    }
}

/// Informs the registered notifiers about an event.
///
/// # Safety
/// The caller supplies the original chain read-side lifetime protection and
/// context. Reached callbacks obey their native contract; in DEBUG_NOTIFIERS,
/// invalid kernel-text addresses are skipped. A non-null nr_calls points to a
/// writable local count, and val/v satisfy the reached callbacks' contracts.
// The stable C name lets native NOKPROBE metadata refer to the Rust body.
// This does not prove generated blacklist coverage of inlining or clones.
#[export_name = "lupos_notifier_call_chain"]
unsafe extern "C" fn notifier_call_chain(
    nl: *mut *mut notifier_block,
    val: c_ulong,
    v: *mut c_void,
    mut nr_to_call: c_int,
    nr_calls: *mut c_int,
) -> c_int {
    // SAFETY: Callers hold the appropriate RCU/SRCU/rwsem or raw protection.
    // Native invocation uses notifier_fn_t directly, with no nullable Rust
    // callback encoding or extra null-skip policy.
    unsafe {
        let mut ret = LUPOS_NOTIFIER_DONE as c_int;
        let mut nb = lupos_notifier_dereference_raw(nl);

        while !nb.is_null() && nr_to_call != 0 {
            let next_nb = lupos_notifier_dereference_raw(lupos_notifier_next_slot(nb));
            #[cfg(CONFIG_DEBUG_NOTIFIERS)]
            if lupos_notifier_invalid_callback(nb) {
                lupos_notifier_warn_invalid();
                nb = next_nb;
                // C's continue does not trace, invoke, count or consume budget.
                continue;
            }
            lupos_notifier_trace_run(nb);
            ret = lupos_notifier_invoke(nb, val, v);

            if !nr_calls.is_null() {
                *nr_calls += 1;
            }
            if ret & (LUPOS_NOTIFIER_STOP_MASK as c_int) != 0 {
                break;
            }
            nb = next_nb;
            nr_to_call -= 1;
        }
        ret
    }
}

/// # Safety
/// The chain is unchanged across both walks; RCU alone is insufficient. The
/// caller satisfies notifier_call_chain's callback and data lifetime contract.
unsafe fn notifier_call_chain_robust(
    nl: *mut *mut notifier_block,
    val_up: c_ulong,
    val_down: c_ulong,
    v: *mut c_void,
) -> c_int {
    // SAFETY: The caller holds stable-chain protection across both invocations;
    // nr is a live local and counts only callbacks actually invoked.
    unsafe {
        let mut nr = 0;
        let ret = notifier_call_chain(nl, val_up, v, -1, &raw mut nr);
        if ret & (LUPOS_NOTIFIER_STOP_MASK as c_int) != 0 {
            notifier_call_chain(nl, val_down, v, nr - 1, core::ptr::null_mut());
        }
        ret
    }
}

/// # Safety
/// nh is initialized and n obeys the atomic notifier registration contract.
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_register(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> c_int {
    // SAFETY: Native irqsave/restore protect mutation of this native head.
    unsafe {
        let flags = lupos_notifier_atomic_lock(nh);
        let ret = notifier_chain_register(lupos_notifier_atomic_head(nh), n, false);
        lupos_notifier_atomic_unlock(nh, flags);
        ret
    }
}

/// # Safety
/// nh is initialized and n obeys the atomic notifier registration contract.
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_register_unique_prio(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> c_int {
    // SAFETY: Native irqsave/restore protect the same registration walk.
    unsafe {
        let flags = lupos_notifier_atomic_lock(nh);
        let ret = notifier_chain_register(lupos_notifier_atomic_head(nh), n, true);
        lupos_notifier_atomic_unlock(nh, flags);
        ret
    }
}

/// # Safety
/// nh and n remain live, and this is not called from within the call chain.
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_unregister(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> c_int {
    // SAFETY: The spinlock protects unlinking. The unconditional native grace
    // period follows unlock even on ENOENT, before caller reclamation.
    unsafe {
        let flags = lupos_notifier_atomic_lock(nh);
        let ret = notifier_chain_unregister(lupos_notifier_atomic_head(nh), n);
        lupos_notifier_atomic_unlock(nh, flags);
        lupos_notifier_synchronize_rcu();
        ret
    }
}

/// # Safety
/// nh is initialized; callback/data lifetimes and atomic-context rules hold.
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_call_chain(
    nh: *mut atomic_notifier_head, val: c_ulong, v: *mut c_void,
) -> c_int {
    // SAFETY: Native RCU read-side protection encloses the complete walk.
    unsafe {
        lupos_notifier_rcu_read_lock();
        let ret = notifier_call_chain(lupos_notifier_atomic_head(nh), val, v, -1, core::ptr::null_mut());
        lupos_notifier_rcu_read_unlock();
        ret
    }
}

/// # Safety
/// nh is a live initialized atomic notifier head.
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_call_chain_is_empty(nh: *mut atomic_notifier_head) -> bool {
    // SAFETY: rcu_access_pointer only inspects the head; it does not dereference
    // the returned block or extend its lifetime.
    unsafe { lupos_notifier_access(lupos_notifier_atomic_head(nh)).is_null() }
}

/// # Safety
/// nh/n satisfy the blocking registration lifetime and process-context rules.
unsafe fn __blocking_notifier_chain_register(
    nh: *mut blocking_notifier_head, n: *mut notifier_block, unique_priority: bool,
) -> c_int {
    // SAFETY: Early boot retains its original no-scheduling path; otherwise the
    // native write semaphore protects every registration access.
    unsafe {
        if lupos_notifier_blocking_register_booting() {
            return notifier_chain_register(lupos_notifier_blocking_head(nh), n, unique_priority);
        }
        lupos_notifier_down_write(nh);
        let ret = notifier_chain_register(lupos_notifier_blocking_head(nh), n, unique_priority);
        lupos_notifier_up_write(nh);
        ret
    }
}

/// # Safety
/// nh/n satisfy __blocking_notifier_chain_register's contract.
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_register(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: Forward the original registration contract and priority policy.
    unsafe { __blocking_notifier_chain_register(nh, n, false) }
}

/// # Safety
/// nh/n satisfy __blocking_notifier_chain_register's contract.
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_register_unique_prio(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: Forward the same contract with original unique-priority checking.
    unsafe { __blocking_notifier_chain_register(nh, n, true) }
}

/// # Safety
/// nh/n remain live, process context is valid, and the call is outside the chain.
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_unregister(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: Early boot bypasses scheduling; otherwise write locking excludes
    // readers until unlinking completes, as in the original owner.
    unsafe {
        if lupos_notifier_blocking_unregister_booting() {
            return notifier_chain_unregister(lupos_notifier_blocking_head(nh), n);
        }
        lupos_notifier_down_write(nh);
        let ret = notifier_chain_unregister(lupos_notifier_blocking_head(nh), n);
        lupos_notifier_up_write(nh);
        ret
    }
}

/// # Safety
/// nh is initialized and val_up/val_down/v obey each reached callback contract.
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_call_chain_robust(nh: *mut blocking_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> c_int {
    // SAFETY: The preliminary head access does not dereference a block. Read
    // locking keeps the chain stable across the forward and rollback walks.
    unsafe {
        let mut ret = LUPOS_NOTIFIER_DONE as c_int;
        if !lupos_notifier_access(lupos_notifier_blocking_head(nh)).is_null() {
            lupos_notifier_down_read(nh);
            ret = notifier_call_chain_robust(lupos_notifier_blocking_head(nh), val_up, val_down, v);
            lupos_notifier_up_read(nh);
        }
        ret
    }
}

/// # Safety
/// nh is initialized and val/v obey each reached callback's process-context contract.
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_call_chain(nh: *mut blocking_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    // SAFETY: As above, the native rwsem protects all dereferenced blocks.
    unsafe {
        let mut ret = LUPOS_NOTIFIER_DONE as c_int;
        if !lupos_notifier_access(lupos_notifier_blocking_head(nh)).is_null() {
            lupos_notifier_down_read(nh);
            ret = notifier_call_chain(lupos_notifier_blocking_head(nh), val, v, -1, core::ptr::null_mut());
            lupos_notifier_up_read(nh);
        }
        ret
    }
}

/// # Safety
/// The caller provides all raw-chain writer protection and block lifetimes.
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_chain_register(nh: *mut raw_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: The caller, rather than this raw API, protects mutation.
    unsafe { notifier_chain_register(lupos_notifier_raw_head(nh), n, false) }
}

/// # Safety
/// The caller provides all raw-chain writer protection and reclamation ordering.
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_chain_unregister(nh: *mut raw_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: The caller owns writer exclusion and block lifetimes.
    unsafe { notifier_chain_unregister(lupos_notifier_raw_head(nh), n) }
}

/// # Safety
/// The caller prevents mutation across both walks and satisfies callback contracts.
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_call_chain_robust(nh: *mut raw_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> c_int {
    // SAFETY: Stable-chain and callback protection are provided by the caller.
    unsafe { notifier_call_chain_robust(lupos_notifier_raw_head(nh), val_up, val_down, v) }
}

/// # Safety
/// The caller supplies all raw-chain read-side and callback/data protection.
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_call_chain(nh: *mut raw_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    // SAFETY: The original raw API imposes protection on its caller.
    unsafe { notifier_call_chain(lupos_notifier_raw_head(nh), val, v, -1, core::ptr::null_mut()) }
}

/// # Safety
/// nh is initialized and nh/n obey the SRCU registration/process-context contract.
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_chain_register(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: Early boot avoids scheduling; otherwise the native mutex covers
    // publication while readers retain the original SRCU lifetime protection.
    unsafe {
        if lupos_notifier_srcu_register_booting() {
            return notifier_chain_register(lupos_notifier_srcu_head(nh), n, false);
        }
        lupos_notifier_mutex_lock(nh);
        let ret = notifier_chain_register(lupos_notifier_srcu_head(nh), n, false);
        lupos_notifier_mutex_unlock(nh);
        ret
    }
}

/// # Safety
/// nh/n remain live, process context is valid, and this is outside the call chain.
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_chain_unregister(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> c_int {
    // SAFETY: Preserve boot bypass and, otherwise, unlink under the mutex then
    // wait for SRCU readers only after unlocking, even if unlink returned ENOENT.
    unsafe {
        if lupos_notifier_srcu_unregister_booting() {
            return notifier_chain_unregister(lupos_notifier_srcu_head(nh), n);
        }
        lupos_notifier_mutex_lock(nh);
        let ret = notifier_chain_unregister(lupos_notifier_srcu_head(nh), n);
        lupos_notifier_mutex_unlock(nh);
        lupos_notifier_synchronize_srcu(nh);
        ret
    }
}

/// # Safety
/// nh is initialized; val/v and reached callbacks obey the SRCU chain contract.
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_call_chain(nh: *mut srcu_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    // SAFETY: The native SRCU index is kept unchanged until matching unlock.
    unsafe {
        let idx = lupos_notifier_srcu_read_lock(nh);
        let ret = notifier_call_chain(lupos_notifier_srcu_head(nh), val, v, -1, core::ptr::null_mut());
        lupos_notifier_srcu_read_unlock(nh, idx);
        ret
    }
}

/// # Safety
/// nh is live, writable, unshared during initialization, and later cleaned up
/// using the original srcu_cleanup_notifier_head contract before deallocation.
#[no_mangle]
pub unsafe extern "C" fn srcu_init_notifier_head(nh: *mut srcu_notifier_head) {
    // SAFETY: Initialization occurs before publication. Native macros retain
    // their actual lockdep key/storage and original fatal failure behavior.
    unsafe {
        lupos_notifier_mutex_init(nh);
        if lupos_notifier_init_srcu(nh) < 0 {
            lupos_notifier_bug_init_srcu();
        }
        lupos_notifier_store(lupos_notifier_srcu_head(nh), core::ptr::null_mut());
    }
}

/// # Safety
/// The original notify_die context and argument/callback lifetime rules apply.
// Native metadata requires notrace here. No Rust-body implementation of that
// policy is asserted: the unconditional source hold must remain until qualified.
#[no_mangle]
pub unsafe extern "C" fn notify_die(val: die_val, str_: *const c_char, regs: *mut pt_regs, err: c_long, trap: c_int, sig: c_int) -> c_int {
    // SAFETY: die_args uses the configured native layout and remains mutable
    // and live through the synchronous call. The diagnostic evaluates its RCU
    // predicate inside the native macro, retaining configuration short-circuiting.
    unsafe {
        let mut args = die_args { regs, str: str_, err, trapnr: trap, signr: sig };
        lupos_notifier_warn_die_rcu();
        atomic_notifier_call_chain(lupos_notifier_die_head(), val.0 as c_ulong, (&raw mut args).cast())
    }
}

/// # Safety
/// nb obeys atomic_notifier_chain_register's native block/callback contract.
#[no_mangle]
pub unsafe extern "C" fn register_die_notifier(nb: *mut notifier_block) -> c_int {
    // SAFETY: The native die head has its real static spinlock initializer.
    unsafe { atomic_notifier_chain_register(lupos_notifier_die_head(), nb) }
}

/// # Safety
/// nb obeys atomic_notifier_chain_unregister's lifetime and context contract.
#[no_mangle]
pub unsafe extern "C" fn unregister_die_notifier(nb: *mut notifier_block) -> c_int {
    // SAFETY: The atomic unregister path retains the required RCU grace period.
    unsafe { atomic_notifier_chain_unregister(lupos_notifier_die_head(), nb) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
