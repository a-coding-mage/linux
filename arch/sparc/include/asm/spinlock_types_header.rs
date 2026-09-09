/* SPDX-License-Identifier: GPL-2.0 */

// When CONFIG_QUEUED_SPINLOCKS is enabled, the definitions are supplied by
// the asm-generic qspinlock types header.
#[cfg(not(CONFIG_QUEUED_SPINLOCKS))]
#[repr(C)]
pub struct arch_spinlock_t {
    // C volatile unsigned char lock;
    pub lock: core::cell::UnsafeCell<u8>,
}

#[cfg(not(CONFIG_QUEUED_SPINLOCKS))]
pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t {
    lock: core::cell::UnsafeCell::new(0),
};

// When CONFIG_QUEUED_RWLOCKS is enabled, the definitions are supplied by
// the asm-generic qrwlock types header.
#[cfg(not(CONFIG_QUEUED_RWLOCKS))]
#[repr(C)]
pub struct arch_rwlock_t {
    // C volatile unsigned int lock;
    pub lock: core::cell::UnsafeCell<u32>,
}

#[cfg(not(CONFIG_QUEUED_RWLOCKS))]
pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t {
    lock: core::cell::UnsafeCell::new(0),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
