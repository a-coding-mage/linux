/* SPDX-License-Identifier: GPL-2.0 */

pub const __ARCH_SPIN_LOCK_UNLOCKED_VAL: u32 = 0x1a46;

pub const SPINLOCK_BREAK_INSN: u32 = 0x0000c006; /* break 6,6 */

#[repr(C)]
pub struct arch_spinlock_t {
    /* C: volatile unsigned int lock[4]; */
    pub lock: [core::cell::UnsafeCell<u32>; 4],
}

impl arch_spinlock_t {
    pub const fn new_unlocked() -> Self {
        Self {
            lock: [
                core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_UNLOCKED_VAL),
                core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_UNLOCKED_VAL),
                core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_UNLOCKED_VAL),
                core::cell::UnsafeCell::new(__ARCH_SPIN_LOCK_UNLOCKED_VAL),
            ],
        }
    }
}

/* __ARCH_SPIN_LOCK_UNLOCKED: { { value, value, value, value } } */
pub const fn __ARCH_SPIN_LOCK_UNLOCKED() -> arch_spinlock_t {
    arch_spinlock_t::new_unlocked()
}

/* counter:
 * Unlocked     : 0x0100_0000
 * Read lock(s) : 0x00FF_FFFF to 0x01  (Multiple Readers decrement it)
 * Write lock   : 0x0, but only if prior value is "unlocked" 0x0100_0000
 */
#[repr(C)]
pub struct arch_rwlock_t {
    pub lock_mutex: arch_spinlock_t,
    /* C: volatile unsigned int counter; */
    pub counter: core::cell::UnsafeCell<u32>,
}

pub const __ARCH_RW_LOCK_UNLOCKED__: u32 = 0x01000000;

/* __ARCH_RW_LOCK_UNLOCKED:
 * { .lock_mutex = __ARCH_SPIN_LOCK_UNLOCKED,
 *   .counter = __ARCH_RW_LOCK_UNLOCKED__ }
 */
pub const fn __ARCH_RW_LOCK_UNLOCKED() -> arch_rwlock_t {
    arch_rwlock_t {
        lock_mutex: __ARCH_SPIN_LOCK_UNLOCKED(),
        counter: core::cell::UnsafeCell::new(__ARCH_RW_LOCK_UNLOCKED__),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
