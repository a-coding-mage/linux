// SPDX-License-Identifier: GPL-2.0
/*
 * Split spinlock implementation out into its own file, so it can be
 * compiled in a FTRACE-compatible way.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not redefined here.

// DEFINE_STATIC_KEY_FALSE(virt_spin_lock_key);
pub static mut virt_spin_lock_key: static_key_false = static_key_false;

// #ifdef CONFIG_SMP
#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn native_pv_lock_init() {
    if boot_cpu_has(X86_FEATURE_HYPERVISOR) {
        static_branch_enable(&mut virt_spin_lock_key);
    }
}
// #endif

// #ifdef CONFIG_PARAVIRT_SPINLOCKS
#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
#[no_mangle]
pub unsafe extern "C" fn __native_queued_spin_unlock(lock: *mut qspinlock) {
    native_queued_spin_unlock(lock);
}

// PV_CALLEE_SAVE_REGS_THUNK(__native_queued_spin_unlock);

// DEFINE_STATIC_CALL(queued_spin_lock_slowpath, native_queued_spin_lock_slowpath);
// EXPORT_STATIC_CALL_TRAMP(queued_spin_lock_slowpath);
// DEFINE_STATIC_CALL(queued_spin_unlock, __raw_callee_save___native_queued_spin_unlock);
// EXPORT_STATIC_CALL_TRAMP(queued_spin_unlock);

/*
 * Traced unlock variants, swapped in via static_call while the
 * contended_release tracepoint is enabled. Two of them, so each tail calls its
 * own base directly.
 */
#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
#[no_mangle]
pub unsafe extern "C" fn native_queued_spin_unlock_traced(lock: *mut qspinlock) {
    if queued_spin_is_contended(lock) {
        trace_call__contended_release(lock);
    }
    native_queued_spin_unlock(lock);
}

// PV_CALLEE_SAVE_REGS_THUNK(native_queued_spin_unlock_traced);

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
#[no_mangle]
pub unsafe extern "C" fn pv_queued_spin_unlock_traced(lock: *mut qspinlock) {
    if queued_spin_is_contended(lock) {
        trace_call__contended_release(lock);
    }
    __raw_callee_save___pv_queued_spin_unlock(lock);
}

// PV_CALLEE_SAVE_REGS_THUNK(pv_queued_spin_unlock_traced);

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub unsafe extern "C" fn pv_is_native_spin_unlock() -> bool {
    let unlock: *mut core::ffi::c_void = static_call_query(queued_spin_unlock);

    unlock == __raw_callee_save___native_queued_spin_unlock as *mut core::ffi::c_void
        || unlock == __raw_callee_save_native_queued_spin_unlock_traced as *mut core::ffi::c_void
}

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub unsafe extern "C" fn arch_contended_release_trace_reg() -> i32 {
    let cur: *mut core::ffi::c_void = static_call_query(queued_spin_unlock);

    if cur == __raw_callee_save___native_queued_spin_unlock as *mut core::ffi::c_void {
        static_call_update(
            queued_spin_unlock,
            __raw_callee_save_native_queued_spin_unlock_traced,
        );
    } else if cur == __raw_callee_save___pv_queued_spin_unlock as *mut core::ffi::c_void {
        static_call_update(
            queued_spin_unlock,
            __raw_callee_save_pv_queued_spin_unlock_traced,
        );
    }
    0
}

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub unsafe extern "C" fn arch_contended_release_trace_unreg() {
    let cur: *mut core::ffi::c_void = static_call_query(queued_spin_unlock);

    if cur == __raw_callee_save_native_queued_spin_unlock_traced as *mut core::ffi::c_void {
        static_call_update(
            queued_spin_unlock,
            __raw_callee_save___native_queued_spin_unlock,
        );
    } else if cur == __raw_callee_save_pv_queued_spin_unlock_traced as *mut core::ffi::c_void {
        static_call_update(
            queued_spin_unlock,
            __raw_callee_save___pv_queued_spin_unlock,
        );
    }
}

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
#[no_mangle]
pub unsafe extern "C" fn __native_vcpu_is_preempted(_cpu: i64) -> bool {
    false
}

// PV_CALLEE_SAVE_REGS_THUNK(__native_vcpu_is_preempted);

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub unsafe extern "C" fn pv_is_native_vcpu_is_preempted() -> bool {
    pv_ops_lock.vcpu_is_preempted.func
        == __raw_callee_save___native_vcpu_is_preempted as *mut core::ffi::c_void
}

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub unsafe extern "C" fn paravirt_set_cap() {
    if !pv_is_native_vcpu_is_preempted() {
        setup_force_cpu_cap(X86_FEATURE_VCPUPREEMPT);
    }
}

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
pub static mut pv_ops_lock: pv_lock_ops = pv_lock_ops {
    wait: paravirt_nop,
    kick: paravirt_nop,
    vcpu_is_preempted: pv_callee_save(__native_vcpu_is_preempted),
};

// EXPORT_SYMBOL(pv_ops_lock);
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
