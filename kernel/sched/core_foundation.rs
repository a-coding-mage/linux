// SPDX-License-Identifier: GPL-2.0-only
// Rust algorithm owner for core.c:1..5524. Source phase; not accepted/buildable.
include!("core_foundation_locks.rs");
include!("core_foundation_wake.rs");

// core.c:646. The preemption disable spans lock selection/retry and pairs with
// synchronize_rcu in __sched_core_enable. It must not be folded into each leaf.
#[no_mangle]
pub unsafe extern "C" fn raw_spin_rq_lock_nested(rq: *mut rq, subclass: c_int) {
    lupos_core_preempt_disable();
    if lupos_core_sched_core_disabled() {
        lupos_core_raw_spin_lock_nested(addr_of_mut!((*rq).__lock), subclass);
        lupos_core_preempt_enable_no_resched();
        return;
    }
    loop {
        let lock = lupos_core_rq_lockp(rq);
        lupos_core_raw_spin_lock_nested(lock, subclass);
        if lock == lupos_core_rq_lockp(rq) {
            lupos_core_preempt_enable_no_resched();
            return;
        }
        lupos_core_raw_spin_unlock(lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn raw_spin_rq_trylock(rq: *mut rq) -> bool {
    lupos_core_preempt_disable();
    if lupos_core_sched_core_disabled() {
        let ret = lupos_core_raw_spin_trylock(addr_of_mut!((*rq).__lock));
        lupos_core_preempt_enable();
        return ret;
    }
    loop {
        let lock = lupos_core_rq_lockp(rq);
        let ret = lupos_core_raw_spin_trylock(lock);
        if !ret || lock == lupos_core_rq_lockp(rq) {
            lupos_core_preempt_enable();
            return ret;
        }
        lupos_core_raw_spin_unlock(lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn double_rq_lock(mut rq1: *mut rq, mut rq2: *mut rq) {
    lupos_core_assert_irqs_disabled();
    if lupos_core_rq_order_less(rq2, rq1) {
        core::mem::swap(&mut rq1, &mut rq2);
    }
    raw_spin_rq_lock_nested(rq1, 0);
    if lupos_core_rq_lockp(rq1) != lupos_core_rq_lockp(rq2) {
        raw_spin_rq_lock_nested(rq2, LUPOS_CORE_SINGLE_DEPTH_NESTING as c_int);
    } else {
        // Native sparse context bookkeeping only; does not acquire twice.
        lupos_core_acquire_ctx_lock(lupos_core_rq_lockp(rq2));
    }
    lupos_core_double_rq_clock_clear_update(rq1, rq2);
}
#[no_mangle]
pub unsafe extern "C" fn ___task_rq_lock(p: *mut task_struct, rf: *mut rq_flags) -> *mut rq {
    lupos_core_assert_pi_lock(p);
    loop {
        let rq = lupos_core_task_rq(p);
        raw_spin_rq_lock_nested(rq, 0);
        if rq == lupos_core_task_rq(p) && !lupos_core_task_on_rq_migrating(p) {
            lupos_core_rq_pin_lock(rq, rf);
            return rq;
        }
        lupos_core_raw_spin_rq_unlock(rq);
        while lupos_core_task_on_rq_migrating(p) {
            lupos_core_cpu_relax();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _task_rq_lock(p: *mut task_struct, rf: *mut rq_flags) -> *mut rq {
    loop {
        (*rf).flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
        let rq = lupos_core_task_rq(p);
        raw_spin_rq_lock_nested(rq, 0);
        // Retain the address dependency headed by task_rq and the lock's
        // acquire against __set_task_cpu's WMB and ON_RQ_MIGRATING stores.
        if rq == lupos_core_task_rq(p) && !lupos_core_task_on_rq_migrating(p) {
            lupos_core_rq_pin_lock(rq, rf);
            return rq;
        }
        lupos_core_raw_spin_rq_unlock(rq);
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), (*rf).flags);
        while lupos_core_task_on_rq_migrating(p) {
            lupos_core_cpu_relax();
        }
    }
}
include!("core_foundation_queue.rs");
include!("core_foundation_ttwu.rs");
include!("core_foundation_switch.rs");
include!("core_foundation_affinity.rs");
include!("core_foundation_smt.rs");
include!("core_foundation_uclamp.rs");
include!("core_foundation_fork.rs");
include!("core_foundation_timer.rs");
include!("core_foundation_config.rs");
