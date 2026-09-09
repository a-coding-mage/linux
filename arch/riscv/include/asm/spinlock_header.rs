/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the RISC-V spinlock header.

#[cfg(feature = "CONFIG_QUEUED_SPINLOCKS")]
pub const _Q_PENDING_LOOPS: i32 = 1 << 9;

#[cfg(feature = "CONFIG_RISCV_COMBO_SPINLOCKS")]
mod combo_spinlocks {
    // TODO: Use an alternative instead of a static key when we are able to parse
    // the extensions string earlier in the boot process.

    extern "C" {
        static qspinlock_key: StaticKeyTrue;

        fn static_branch_unlikely(key: *const StaticKeyTrue) -> bool;

        fn queued_spin_lock(lock: *mut arch_spinlock_t);
        fn queued_spin_unlock(lock: *mut arch_spinlock_t);
        fn queued_spin_is_locked(lock: *mut arch_spinlock_t) -> i32;
        fn queued_spin_is_contended(lock: *mut arch_spinlock_t) -> i32;
        fn queued_spin_trylock(lock: *mut arch_spinlock_t) -> bool;
        fn queued_spin_value_unlocked(lock: arch_spinlock_t) -> i32;

        fn ticket_spin_lock(lock: *mut arch_spinlock_t);
        fn ticket_spin_unlock(lock: *mut arch_spinlock_t);
        fn ticket_spin_is_locked(lock: *mut arch_spinlock_t) -> i32;
        fn ticket_spin_is_contended(lock: *mut arch_spinlock_t) -> i32;
        fn ticket_spin_trylock(lock: *mut arch_spinlock_t) -> bool;
        fn ticket_spin_value_unlocked(lock: arch_spinlock_t) -> i32;
    }

    #[inline(always)]
    pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_lock(lock)
        } else {
            ticket_spin_lock(lock)
        }
    }

    #[inline(always)]
    pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_unlock(lock)
        } else {
            ticket_spin_unlock(lock)
        }
    }

    #[inline(always)]
    pub unsafe fn arch_spin_is_locked(lock: *mut arch_spinlock_t) -> i32 {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_is_locked(lock)
        } else {
            ticket_spin_is_locked(lock)
        }
    }

    #[inline(always)]
    pub unsafe fn arch_spin_is_contended(lock: *mut arch_spinlock_t) -> i32 {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_is_contended(lock)
        } else {
            ticket_spin_is_contended(lock)
        }
    }

    #[inline(always)]
    pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> bool {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_trylock(lock)
        } else {
            ticket_spin_trylock(lock)
        }
    }

    #[inline(always)]
    pub unsafe fn arch_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
        if static_branch_unlikely(&qspinlock_key) {
            queued_spin_value_unlocked(lock)
        } else {
            ticket_spin_value_unlocked(lock)
        }
    }
}

// CONFIG_RISCV_QUEUED_SPINLOCKS selects the qspinlock declarations;
// otherwise the ticket_spinlock declarations are supplied by dependencies.

// The qr​​wlock declarations are supplied by the external asm/qrwlock dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
