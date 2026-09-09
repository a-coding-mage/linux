/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Gives us 8 prio classes with 13-bits of data for each class
 */
pub const IOPRIO_CLASS_SHIFT: u32 = 13;
pub const IOPRIO_NR_CLASSES: u32 = 8;
pub const IOPRIO_CLASS_MASK: u32 = IOPRIO_NR_CLASSES - 1;
pub const IOPRIO_PRIO_MASK: u32 = (1u32 << IOPRIO_CLASS_SHIFT) - 1;

#[inline]
pub const fn IOPRIO_PRIO_CLASS(ioprio: u32) -> u32 {
    (ioprio >> IOPRIO_CLASS_SHIFT) & IOPRIO_CLASS_MASK
}

#[inline]
pub const fn IOPRIO_PRIO_DATA(ioprio: u32) -> u32 {
    ioprio & IOPRIO_PRIO_MASK
}

/*
 * These are the io priority classes as implemented by the BFQ and mq-deadline
 * schedulers. RT is the realtime class, it always gets premium service. For
 * ATA disks supporting NCQ IO priority, RT class IOs will be processed using
 * high priority NCQ commands. BE is the best-effort scheduling class, the
 * default for any process. IDLE is the idle scheduling class, it is only
 * served when no one else is using the disk.
 */
pub const IOPRIO_CLASS_NONE: u32 = 0;
pub const IOPRIO_CLASS_RT: u32 = 1;
pub const IOPRIO_CLASS_BE: u32 = 2;
pub const IOPRIO_CLASS_IDLE: u32 = 3;

/* Special class to indicate an invalid ioprio value */
pub const IOPRIO_CLASS_INVALID: u32 = 7;

/*
 * The RT and BE priority classes both support up to 8 priority levels that
 * can be specified using the lower 3-bits of the priority data.
 */
pub const IOPRIO_LEVEL_NR_BITS: u32 = 3;
pub const IOPRIO_NR_LEVELS: u32 = 1 << IOPRIO_LEVEL_NR_BITS;
pub const IOPRIO_LEVEL_MASK: u32 = IOPRIO_NR_LEVELS - 1;

#[inline]
pub const fn IOPRIO_PRIO_LEVEL(ioprio: u32) -> u32 {
    ioprio & IOPRIO_LEVEL_MASK
}

pub const IOPRIO_BE_NR: u32 = IOPRIO_NR_LEVELS;

/*
 * Possible values for the "which" argument of the ioprio_get() and
 * ioprio_set() system calls (see "man ioprio_set").
 */
pub const IOPRIO_WHO_PROCESS: u32 = 1;
pub const IOPRIO_WHO_PGRP: u32 = 2;
pub const IOPRIO_WHO_USER: u32 = 3;

/* Fallback BE class priority level. */
pub const IOPRIO_NORM: u32 = 4;
pub const IOPRIO_BE_NORM: u32 = IOPRIO_NORM;

/*
 * The 10 bits between the priority class and the priority level are used to
 * optionally define I/O hints for any combination of I/O priority class and
 * level. Depending on the kernel configuration, I/O scheduler being used and
 * the target I/O device being used, hints can influence how I/Os are processed
 * without affecting the I/O scheduling ordering defined by the I/O priority
 * class and level.
 */
pub const IOPRIO_HINT_SHIFT: u32 = IOPRIO_LEVEL_NR_BITS;
pub const IOPRIO_HINT_NR_BITS: u32 = 10;
pub const IOPRIO_NR_HINTS: u32 = 1 << IOPRIO_HINT_NR_BITS;
pub const IOPRIO_HINT_MASK: u32 = IOPRIO_NR_HINTS - 1;

#[inline]
pub const fn IOPRIO_PRIO_HINT(ioprio: u32) -> u32 {
    (ioprio >> IOPRIO_HINT_SHIFT) & IOPRIO_HINT_MASK
}

/* I/O hints. */
pub const IOPRIO_HINT_NONE: u32 = 0;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_1: u32 = 1;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_2: u32 = 2;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_3: u32 = 3;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_4: u32 = 4;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_5: u32 = 5;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_6: u32 = 6;
pub const IOPRIO_HINT_DEV_DURATION_LIMIT_7: u32 = 7;

#[inline]
pub const fn IOPRIO_BAD_VALUE(val: i32, max: i32) -> bool {
    val < 0 || val >= max
}

/*
 * Return an I/O priority value based on a class, a level and a hint.
 */
#[inline]
pub const fn ioprio_value(prioclass: i32, priolevel: i32, priohint: i32) -> u16 {
    if IOPRIO_BAD_VALUE(prioclass, IOPRIO_NR_CLASSES as i32)
        || IOPRIO_BAD_VALUE(priolevel, IOPRIO_NR_LEVELS as i32)
        || IOPRIO_BAD_VALUE(priohint, IOPRIO_NR_HINTS as i32)
    {
        (IOPRIO_CLASS_INVALID << IOPRIO_CLASS_SHIFT) as u16
    } else {
        ((prioclass as u32) << IOPRIO_CLASS_SHIFT
            | (priohint as u32) << IOPRIO_HINT_SHIFT
            | priolevel as u32) as u16
    }
}

#[inline]
pub const fn IOPRIO_PRIO_VALUE(prioclass: i32, priolevel: i32) -> u16 {
    ioprio_value(prioclass, priolevel, IOPRIO_HINT_NONE as i32)
}

#[inline]
pub const fn IOPRIO_PRIO_VALUE_HINT(prioclass: i32, priolevel: i32, priohint: i32) -> u16 {
    ioprio_value(prioclass, priolevel, priohint)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
