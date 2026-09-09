/* SPDX-License-Identifier: GPL-2.0 */

// This header is intended to be included only when __LINUX_SPINLOCK_TYPES_RAW_H
// is available in the surrounding translation unit.

#[repr(C)]
pub struct arch_spinlock_t {
    // C declaration: volatile unsigned int lock;
    pub lock: u32,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t { lock: 0 };

#[repr(C)]
pub struct arch_rwlock_t {
    // C declaration: volatile unsigned int lock;
    pub lock: u32,
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t { lock: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
