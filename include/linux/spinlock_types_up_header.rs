// include/linux/spinlock_types_up.h - spinlock type definitions for UP
//
// portions Copyright 2005, Red Hat, Inc., Ingo Molnar
// Released under the General Public License (GPL).

// The C header requires __LINUX_SPINLOCK_TYPES_RAW_H to be included first.

#[cfg(CONFIG_DEBUG_SPINLOCK)]
#[repr(C)]
pub struct arch_spinlock_t {
    pub slock: core::cell::UnsafeCell<core::ffi::c_uint>,
}

#[cfg(CONFIG_DEBUG_SPINLOCK)]
pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t {
    slock: core::cell::UnsafeCell::new(1),
};

#[cfg(not(CONFIG_DEBUG_SPINLOCK))]
#[repr(C)]
pub struct arch_spinlock_t {}

#[cfg(not(CONFIG_DEBUG_SPINLOCK))]
pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t {};

#[repr(C)]
pub struct arch_rwlock_t {
    // no debug version on UP
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
