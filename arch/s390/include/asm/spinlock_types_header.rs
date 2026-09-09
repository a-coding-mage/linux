/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header requires __LINUX_SPINLOCK_TYPES_RAW_H to be defined by the
 * including environment; the original C header emits an error otherwise.
 */

#[repr(C)]
pub struct arch_spinlock_t {
    pub lock: i32,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t { lock: 0 };

#[repr(C)]
pub struct arch_rwlock_t {
    pub cnts: i32,
    pub wait: arch_spinlock_t,
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t {
    cnts: 0,
    wait: arch_spinlock_t { lock: 0 },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
