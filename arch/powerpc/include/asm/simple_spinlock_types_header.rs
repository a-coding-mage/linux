/* SPDX-License-Identifier: GPL-2.0 */

// This header requires __LINUX_SPINLOCK_TYPES_RAW_H to be defined by the
// including translation unit.  Preserve that build-time dependency here.

#[repr(C)]
pub struct arch_spinlock_t {
    pub slock: u32, // volatile unsigned int
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t { slock: 0 };

#[repr(C)]
pub struct arch_rwlock_t {
    pub lock: i32, // volatile signed int
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t { lock: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
