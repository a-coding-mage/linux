/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Spinlock support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* C header guard: _ASM_SPINLOCK_TYPES_H */
/* This header requires __LINUX_SPINLOCK_TYPES_RAW_H. */

#[repr(C)]
pub struct arch_spinlock_t {
    /* C volatile field; access through core::ptr::{read_volatile, write_volatile}. */
    pub lock: u32,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t { lock: 0 };

#[repr(C)]
pub struct arch_rwlock_t {
    /* C volatile field; access through core::ptr::{read_volatile, write_volatile}. */
    pub lock: u32,
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t { lock: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
