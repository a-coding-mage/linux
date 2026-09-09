/* SPDX-License-Identifier: GPL-2.0 */

// This header is intended to be included only after __LINUX_SPINLOCK_TYPES_RAW_H.

pub const TICKET_SHIFT: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __raw_tickets {
    #[cfg(target_endian = "big")]
    pub next: u16,
    #[cfg(target_endian = "big")]
    pub owner: u16,
    #[cfg(target_endian = "little")]
    pub owner: u16,
    #[cfg(target_endian = "little")]
    pub next: u16,
}

#[repr(C)]
pub union arch_spinlock_t {
    pub slock: u32,
    pub tickets: __raw_tickets,
}

pub const __ARCH_SPIN_LOCK_UNLOCKED: arch_spinlock_t = arch_spinlock_t { slock: 0 };

#[repr(C)]
pub struct arch_rwlock_t {
    pub lock: u32,
}

pub const __ARCH_RW_LOCK_UNLOCKED: arch_rwlock_t = arch_rwlock_t { lock: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
