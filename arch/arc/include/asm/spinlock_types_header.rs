/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

#[repr(C)]
pub struct arch_spinlock_t {
    pub slock: core::cell::UnsafeCell<u32>,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED__: u32 = 0;
pub const __ARCH_SPIN_LOCK_LOCKED__: u32 = 1;

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t {
    slock: core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_UNLOCKED__),
};
pub const __ARCH_SPIN_LOCK_LOCKED: arch_spinlock_t = arch_spinlock_t {
    slock: core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_LOCKED__),
};

/*
 * Unlocked     : 0x0100_0000
 * Read lock(s) : 0x00FF_FFFF to 0x01  (Multiple Readers decrement it)
 * Write lock   : 0x0, but only if prior value is "unlocked" 0x0100_0000
 */
#[repr(C)]
pub struct arch_rwlock_t {
    pub counter: core::cell::UnsafeCell<u32>,
    /* Preserves the source condition: this field is absent when CONFIG_ARC_HAS_LLSC is defined. */
    #[cfg(not(CONFIG_ARC_HAS_LLSC))]
    pub lock_mutex: arch_spinlock_t,
}

pub const __ARCH_RW_LOCK_UNLOCKED__: u32 = 0x01000000;
pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t {
    counter: core::cell::UnsafeCell::new(__ARCH_RW_LOCK_UNLOCKED__),
    #[cfg(not(CONFIG_ARC_HAS_LLSC))]
    lock_mutex: __ARCH_SPIN_LOCK_UNLOCKED,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
