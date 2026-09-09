/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: __ASM_SH_SPINLOCK_TYPES_H

// This header must be included only after __LINUX_SPINLOCK_TYPES_RAW_H.
// The original C header emits an error otherwise.

#[repr(C)]
pub struct arch_spinlock_t {
    pub lock: core::cell::UnsafeCell<u32>,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t {
    lock: core::cell::UnsafeCell::new(1),
};

#[repr(C)]
pub struct arch_rwlock_t {
    pub lock: core::cell::UnsafeCell<u32>,
}

pub const RW_LOCK_BIAS: u32 = 0x01000000;

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t {
    lock: core::cell::UnsafeCell::new(RW_LOCK_BIAS),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
