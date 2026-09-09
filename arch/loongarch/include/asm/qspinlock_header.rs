/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_LOONGARCH_QSPINLOCK_H
// Dependencies: <asm/kvm_para.h>, <asm/paravirt.h>

// CONFIG_PARAVIRT condition from the original header.
#[cfg(feature = "CONFIG_PARAVIRT")]
pub const virt_spin_lock_defined: bool = true;

#[cfg(feature = "CONFIG_PARAVIRT")]
#[inline]
pub unsafe fn virt_spin_lock(lock: *mut qspinlock) -> bool {
    let mut val: i32;

    if !static_branch_unlikely(&virt_spin_lock_key) {
        return false;
    }

    /*
     * On hypervisors without PARAVIRT_SPINLOCKS support we fall
     * back to a Test-and-Set spinlock, because fair locks have
     * horrible lock 'holder' preemption issues.
     */

    loop {
        val = atomic_read(&(*lock).val);

        if val != 0 || !atomic_try_cmpxchg(&mut (*lock).val, &mut val, _Q_LOCKED_VAL) {
            cpu_relax();
            continue;
        }

        return true;
    }
}

/*
 * Macro is better than inline function here
 * With macro, parameter cpu is parsed only when it is used.
 * With inline function, parameter cpu is parsed even though it is not used.
 * This may cause cache line thrashing across NUMA nodes.
 */
#[cfg(feature = "CONFIG_PARAVIRT")]
#[macro_export]
macro_rules! vcpu_is_preempted {
    ($cpu:expr) => {{
        let mut __val: bool;

        if !static_branch_unlikely(&virt_preempt_key) {
            __val = false;
        } else {
            let __src: *mut kvm_steal_time;
            __src = &mut per_cpu(steal_time, $cpu) as *mut _;
            __val = !!(READ_ONCE((*__src).preempted) & KVM_VCPU_PREEMPTED);
        }
        __val
    }};
}

// Declarations supplied by the corresponding architecture and generic headers.
// The original file includes <asm-generic/qspinlock.h> here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
