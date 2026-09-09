/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h and asm/byteorder.h

#[repr(C)]
pub union qspinlock {
    pub val: u32,
    #[cfg(target_endian = "little")]
    pub little_endian: qspinlock_little_endian,
    #[cfg(not(target_endian = "little"))]
    pub big_endian: qspinlock_big_endian,
}

#[repr(C)]
pub struct qspinlock_little_endian {
    pub locked: u16,
    pub reserved: [u8; 2],
}

#[repr(C)]
pub struct qspinlock_big_endian {
    pub reserved: [u8; 2],
    pub locked: u16,
}

pub type arch_spinlock_t = qspinlock;

pub const __ARCH_SPIN_LOCK_UNLOCKED: qspinlock = qspinlock { val: 0 };

/*
 * Bitfields in the lock word:
 *
 *     0: locked bit
 *  1-14: lock holder cpu
 *    15: lock owner or queuer vcpus observed to be preempted bit
 *    16: must queue bit
 * 17-31: tail cpu (+1)
 */
#[inline]
pub const fn _Q_SET_MASK(bits: u32, offset: u32) -> u32 {
    ((1u32 << bits).wrapping_sub(1)) << offset
}

/* 0x00000001 */
pub const _Q_LOCKED_OFFSET: u32 = 0;
pub const _Q_LOCKED_BITS: u32 = 1;
pub const _Q_LOCKED_VAL: u32 = 1u32 << _Q_LOCKED_OFFSET;

/* 0x00007ffe */
pub const _Q_OWNER_CPU_OFFSET: u32 = 1;
pub const _Q_OWNER_CPU_BITS: u32 = 14;
pub const _Q_OWNER_CPU_MASK: u32 =
    _Q_SET_MASK(_Q_OWNER_CPU_BITS, _Q_OWNER_CPU_OFFSET);

// C preprocessor condition: CONFIG_NR_CPUS > (1U << _Q_OWNER_CPU_BITS)
// produces: "qspinlock does not support such large CONFIG_NR_CPUS"

/* 0x00008000 */
pub const _Q_SLEEPY_OFFSET: u32 = 15;
pub const _Q_SLEEPY_BITS: u32 = 1;
pub const _Q_SLEEPY_VAL: u32 = 1u32 << _Q_SLEEPY_OFFSET;

/* 0x00010000 */
pub const _Q_MUST_Q_OFFSET: u32 = 16;
pub const _Q_MUST_Q_BITS: u32 = 1;
pub const _Q_MUST_Q_VAL: u32 = 1u32 << _Q_MUST_Q_OFFSET;

/* 0xfffe0000 */
pub const _Q_TAIL_CPU_OFFSET: u32 = 17;
pub const _Q_TAIL_CPU_BITS: u32 = 15;
pub const _Q_TAIL_CPU_MASK: u32 =
    _Q_SET_MASK(_Q_TAIL_CPU_BITS, _Q_TAIL_CPU_OFFSET);

// C preprocessor condition: CONFIG_NR_CPUS >= (1U << _Q_TAIL_CPU_BITS)
// produces: "qspinlock does not support such large CONFIG_NR_CPUS"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
