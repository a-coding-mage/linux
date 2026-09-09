/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Queued spinlock
 *
 * (C) Copyright 2013-2015 Hewlett-Packard Development Company, L.P.
 *
 * Authors: Waiman Long <waiman.long@hp.com>
 */

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct qspinlock_le_bytes {
    pub locked: u8,
    pub pending: u8,
}

#[repr(C)]
pub struct qspinlock_le_words {
    pub locked_pending: u16,
    pub tail: u16,
}

#[repr(C)]
pub struct qspinlock_be_words {
    pub tail: u16,
    pub locked_pending: u16,
}

#[repr(C)]
pub struct qspinlock_be_bytes {
    pub reserved: [u8; 2],
    pub pending: u8,
    pub locked: u8,
}

#[repr(C)]
pub union qspinlock_union {
    pub val: atomic_t,
    /* By using the whole 2nd least significant byte for the pending bit,
     * we can allow better optimization of the lock acquisition for the
     * pending bit holder. */
    pub little_bytes: qspinlock_le_bytes,
    pub little_words: qspinlock_le_words,
    pub big_words: qspinlock_be_words,
    pub big_bytes: qspinlock_be_bytes,
}

#[repr(C)]
pub struct qspinlock {
    pub data: qspinlock_union,
}

pub type arch_spinlock_t = qspinlock;

/* Initializier */
#[macro_export]
macro_rules! __ARCH_SPIN_LOCK_UNLOCKED {
    () => {
        qspinlock {
            data: qspinlock_union { val: ATOMIC_INIT!(0) },
        }
    };
}

/*
 * Bitfields in the atomic value:
 *
 * When NR_CPUS < 16K
 *  0- 7: locked byte
 *  8-15: pending byte
 * 16-17: tail index
 * 18-31: tail cpu (+1)
 *
 * When NR_CPUS >= 16K
 *  0- 7: locked byte
 *     8: pending
 *  9-10: tail index
 * 11-31: tail cpu (+1)
 */

/* C preprocessor token-pasting macro represented with explicit bit values. */
#[macro_export]
macro_rules! _Q_SET_MASK {
    ($bits:expr, $offset:expr) => {
        (((1u32 << $bits) - 1) << $offset)
    };
}

pub const _Q_LOCKED_OFFSET: u32 = 0;
pub const _Q_LOCKED_BITS: u32 = 8;
pub const _Q_LOCKED_MASK: u32 = (1u32 << _Q_LOCKED_BITS) - 1;

pub const _Q_PENDING_OFFSET: u32 = _Q_LOCKED_OFFSET + _Q_LOCKED_BITS;
/* CONFIG_NR_CPUS < (1U << 14) selects 8 bits; otherwise it selects 1 bit. */
#[cfg(not(CONFIG_NR_CPUS_GE_16384))]
pub const _Q_PENDING_BITS: u32 = 8;
#[cfg(CONFIG_NR_CPUS_GE_16384)]
pub const _Q_PENDING_BITS: u32 = 1;
pub const _Q_PENDING_MASK: u32 = ((1u32 << _Q_PENDING_BITS) - 1) << _Q_PENDING_OFFSET;

pub const _Q_TAIL_IDX_OFFSET: u32 = _Q_PENDING_OFFSET + _Q_PENDING_BITS;
pub const _Q_TAIL_IDX_BITS: u32 = 2;
pub const _Q_TAIL_IDX_MASK: u32 = ((1u32 << _Q_TAIL_IDX_BITS) - 1) << _Q_TAIL_IDX_OFFSET;

pub const _Q_TAIL_CPU_OFFSET: u32 = _Q_TAIL_IDX_OFFSET + _Q_TAIL_IDX_BITS;
pub const _Q_TAIL_CPU_BITS: u32 = 32 - _Q_TAIL_CPU_OFFSET;
pub const _Q_TAIL_CPU_MASK: u32 = ((1u32 << _Q_TAIL_CPU_BITS) - 1) << _Q_TAIL_CPU_OFFSET;

pub const _Q_TAIL_OFFSET: u32 = _Q_TAIL_IDX_OFFSET;
pub const _Q_TAIL_MASK: u32 = _Q_TAIL_IDX_MASK | _Q_TAIL_CPU_MASK;

pub const _Q_LOCKED_VAL: u32 = 1u32 << _Q_LOCKED_OFFSET;
pub const _Q_PENDING_VAL: u32 = 1u32 << _Q_PENDING_OFFSET;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
