/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers:
// linux/types.h, asm/byteorder.h, and asm/spinlock_types.h

/*
 * The queued read/write lock data structure
 */

#[repr(C)]
pub union QrwlockCnts {
    pub cnts: atomic_t,
    pub state: QrwlockState,
}

#[repr(C)]
pub struct QrwlockState {
    // The source selects field order with __LITTLE_ENDIAN.
    #[cfg(target_endian = "little")]
    pub wlocked: u8, /* Locked for write? */
    #[cfg(target_endian = "little")]
    pub __lstate: [u8; 3],

    #[cfg(target_endian = "big")]
    pub __lstate: [u8; 3],
    #[cfg(target_endian = "big")]
    pub wlocked: u8, /* Locked for write? */
}

#[repr(C)]
pub struct arch_rwlock_t {
    pub cnts: QrwlockCnts,
    pub wait_lock: arch_spinlock_t,
}

// Equivalent of:
// #define __ARCH_RW_LOCK_UNLOCKED {
//     { .cnts = ATOMIC_INIT(0), },
//     .wait_lock = __ARCH_SPIN_LOCK_UNLOCKED,
// }
#[macro_export]
macro_rules! __ARCH_RW_LOCK_UNLOCKED {
    () => {
        arch_rwlock_t {
            cnts: QrwlockCnts { cnts: ATOMIC_INIT(0) },
            wait_lock: __ARCH_SPIN_LOCK_UNLOCKED!(),
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
