// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel Rust environment.

/*
 *	Notifier chain core routines.  The exported routines below
 *	are layered on top of these, with appropriate locking added.
 */

unsafe fn notifier_chain_register(
    mut nl: *mut *mut notifier_block,
    n: *mut notifier_block,
    unique_priority: bool,
) -> i32 {
    while !(*nl).is_null() {
        if unlikely((*nl) == n) {
            WARN!(1, "notifier callback already registered");
            return -EEXIST;
        }
        if (*n).priority > (**nl).priority {
            break;
        }
        if (*n).priority == (**nl).priority && unique_priority {
            return -EBUSY;
        }
        nl = &mut (**nl).next;
    }
    (*n).next = *nl;
    rcu_assign_pointer!(*nl, n);
    trace_notifier_register(n as *const _ as *mut core::ffi::c_void);
    0
}

unsafe fn notifier_chain_unregister(
    mut nl: *mut *mut notifier_block,
    n: *mut notifier_block,
) -> i32 {
    while !(*nl).is_null() {
        if *nl == n {
            rcu_assign_pointer!(*nl, (*n).next);
            trace_notifier_unregister(n as *const _ as *mut core::ffi::c_void);
            return 0;
        }
        nl = &mut (**nl).next;
    }
    -ENOENT
}

/**
 * notifier_call_chain - Informs the registered notifiers about an event.
 */
unsafe fn notifier_call_chain(
    nl: *mut *mut notifier_block,
    val: c_ulong,
    v: *mut c_void,
    mut nr_to_call: i32,
    nr_calls: *mut i32,
) -> i32 {
    let mut ret = NOTIFY_DONE;
    let mut nb = rcu_dereference_raw!(*nl);

    while !nb.is_null() && nr_to_call != 0 {
        let next_nb = rcu_dereference_raw!((*nb).next);
        trace_notifier_run((*nb).notifier_call as *const _ as *mut c_void);
        ret = ((*nb).notifier_call)(nb, val, v);

        if !nr_calls.is_null() {
            *nr_calls += 1;
        }
        if ret & NOTIFY_STOP_MASK != 0 {
            break;
        }
        nb = next_nb;
        nr_to_call -= 1;
    }
    ret
}

unsafe fn notifier_call_chain_robust(
    nl: *mut *mut notifier_block,
    val_up: c_ulong,
    val_down: c_ulong,
    v: *mut c_void,
) -> i32 {
    let mut nr = 0;
    let ret = notifier_call_chain(nl, val_up, v, -1, &mut nr);
    if ret & NOTIFY_STOP_MASK != 0 {
        notifier_call_chain(nl, val_down, v, nr - 1, core::ptr::null_mut());
    }
    ret
}

pub unsafe fn atomic_notifier_chain_register(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> i32 {
    let mut flags = 0;
    spin_lock_irqsave!((*nh).lock, flags);
    let ret = notifier_chain_register(&mut (*nh).head, n, false);
    spin_unlock_irqrestore!((*nh).lock, flags);
    ret
}

pub unsafe fn atomic_notifier_chain_register_unique_prio(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> i32 {
    let mut flags = 0;
    spin_lock_irqsave!((*nh).lock, flags);
    let ret = notifier_chain_register(&mut (*nh).head, n, true);
    spin_unlock_irqrestore!((*nh).lock, flags);
    ret
}

pub unsafe fn atomic_notifier_chain_unregister(
    nh: *mut atomic_notifier_head, n: *mut notifier_block,
) -> i32 {
    let mut flags = 0;
    spin_lock_irqsave!((*nh).lock, flags);
    let ret = notifier_chain_unregister(&mut (*nh).head, n);
    spin_unlock_irqrestore!((*nh).lock, flags);
    synchronize_rcu();
    ret
}

pub unsafe fn atomic_notifier_call_chain(
    nh: *mut atomic_notifier_head, val: c_ulong, v: *mut c_void,
) -> i32 {
    rcu_read_lock();
    let ret = notifier_call_chain(&mut (*nh).head, val, v, -1, core::ptr::null_mut());
    rcu_read_unlock();
    ret
}

pub unsafe fn atomic_notifier_call_chain_is_empty(nh: *mut atomic_notifier_head) -> bool {
    rcu_access_pointer!((*nh).head).is_null()
}

unsafe fn __blocking_notifier_chain_register(
    nh: *mut blocking_notifier_head, n: *mut notifier_block, unique_priority: bool,
) -> i32 {
    if unlikely(system_state == SYSTEM_BOOTING) {
        return notifier_chain_register(&mut (*nh).head, n, unique_priority);
    }
    down_write!((*nh).rwsem);
    let ret = notifier_chain_register(&mut (*nh).head, n, unique_priority);
    up_write!((*nh).rwsem);
    ret
}

pub unsafe fn blocking_notifier_chain_register(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> i32 {
    __blocking_notifier_chain_register(nh, n, false)
}

pub unsafe fn blocking_notifier_chain_register_unique_prio(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> i32 {
    __blocking_notifier_chain_register(nh, n, true)
}

pub unsafe fn blocking_notifier_chain_unregister(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> i32 {
    if unlikely(system_state == SYSTEM_BOOTING) {
        return notifier_chain_unregister(&mut (*nh).head, n);
    }
    down_write!((*nh).rwsem);
    let ret = notifier_chain_unregister(&mut (*nh).head, n);
    up_write!((*nh).rwsem);
    ret
}

pub unsafe fn blocking_notifier_call_chain_robust(nh: *mut blocking_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> i32 {
    let mut ret = NOTIFY_DONE;
    if !rcu_access_pointer!((*nh).head).is_null() {
        down_read!((*nh).rwsem);
        ret = notifier_call_chain_robust(&mut (*nh).head, val_up, val_down, v);
        up_read!((*nh).rwsem);
    }
    ret
}

pub unsafe fn blocking_notifier_call_chain(nh: *mut blocking_notifier_head, val: c_ulong, v: *mut c_void) -> i32 {
    let mut ret = NOTIFY_DONE;
    if !rcu_access_pointer!((*nh).head).is_null() {
        down_read!((*nh).rwsem);
        ret = notifier_call_chain(&mut (*nh).head, val, v, -1, core::ptr::null_mut());
        up_read!((*nh).rwsem);
    }
    ret
}

pub unsafe fn raw_notifier_chain_register(nh: *mut raw_notifier_head, n: *mut notifier_block) -> i32 {
    notifier_chain_register(&mut (*nh).head, n, false)
}
pub unsafe fn raw_notifier_chain_unregister(nh: *mut raw_notifier_head, n: *mut notifier_block) -> i32 {
    notifier_chain_unregister(&mut (*nh).head, n)
}
pub unsafe fn raw_notifier_call_chain_robust(nh: *mut raw_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> i32 {
    notifier_call_chain_robust(&mut (*nh).head, val_up, val_down, v)
}
pub unsafe fn raw_notifier_call_chain(nh: *mut raw_notifier_head, val: c_ulong, v: *mut c_void) -> i32 {
    notifier_call_chain(&mut (*nh).head, val, v, -1, core::ptr::null_mut())
}

pub unsafe fn srcu_notifier_chain_register(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> i32 {
    if unlikely(system_state == SYSTEM_BOOTING) { return notifier_chain_register(&mut (*nh).head, n, false); }
    mutex_lock!((*nh).mutex);
    let ret = notifier_chain_register(&mut (*nh).head, n, false);
    mutex_unlock!((*nh).mutex);
    ret
}
pub unsafe fn srcu_notifier_chain_unregister(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> i32 {
    if unlikely(system_state == SYSTEM_BOOTING) { return notifier_chain_unregister(&mut (*nh).head, n); }
    mutex_lock!((*nh).mutex);
    let ret = notifier_chain_unregister(&mut (*nh).head, n);
    mutex_unlock!((*nh).mutex);
    synchronize_srcu!((*nh).srcu);
    ret
}
pub unsafe fn srcu_notifier_call_chain(nh: *mut srcu_notifier_head, val: c_ulong, v: *mut c_void) -> i32 {
    let idx = srcu_read_lock!((*nh).srcu);
    let ret = notifier_call_chain(&mut (*nh).head, val, v, -1, core::ptr::null_mut());
    srcu_read_unlock!((*nh).srcu, idx);
    ret
}
pub unsafe fn srcu_init_notifier_head(nh: *mut srcu_notifier_head) {
    mutex_init!((*nh).mutex);
    if init_srcu_struct!((*nh).srcu) < 0 { BUG!(); }
    (*nh).head = core::ptr::null_mut();
}

static mut die_chain: atomic_notifier_head = ATOMIC_NOTIFIER_HEAD!();

pub unsafe fn notify_die(val: die_val, str_: *const c_char, regs: *mut pt_regs, err: c_long, trap: i32, sig: i32) -> i32 {
    let args = die_args { regs, str: str_, err, trapnr: trap, signr: sig };
    RCU_LOCKDEP_WARN!(!rcu_is_watching(), "notify_die called but RCU thinks we're quiescent");
    atomic_notifier_call_chain(&mut die_chain, val as c_ulong, &args as *const _ as *mut c_void)
}
pub unsafe fn register_die_notifier(nb: *mut notifier_block) -> i32 { atomic_notifier_chain_register(&mut die_chain, nb) }
pub unsafe fn unregister_die_notifier(nb: *mut notifier_block) -> i32 { atomic_notifier_chain_unregister(&mut die_chain, nb) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
