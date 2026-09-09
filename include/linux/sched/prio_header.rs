/* SPDX-License-Identifier: GPL-2.0 */

pub const MAX_NICE: i32 = 19;
pub const MIN_NICE: i32 = -20;
pub const NICE_WIDTH: i32 = MAX_NICE - MIN_NICE + 1;

/*
 * Priority of a process goes from 0..MAX_PRIO-1, valid RT
 * priority is 0..MAX_RT_PRIO-1, and SCHED_NORMAL/SCHED_BATCH
 * tasks are in the range MAX_RT_PRIO..MAX_PRIO-1. Priority
 * values are inverted: lower p->prio value means higher priority.
 */

pub const MAX_RT_PRIO: i32 = 100;
pub const MAX_DL_PRIO: i32 = 0;

pub const MAX_PRIO: i32 = MAX_RT_PRIO + NICE_WIDTH;
pub const DEFAULT_PRIO: i32 = MAX_RT_PRIO + NICE_WIDTH / 2;

/*
 * Convert user-nice values [ -20 ... 0 ... 19 ]
 * to static priority [ MAX_RT_PRIO..MAX_PRIO-1 ],
 * and back.
 */
#[inline]
pub const fn nice_to_prio(nice: i32) -> i32 {
    nice + DEFAULT_PRIO
}

#[inline]
pub const fn prio_to_nice(prio: i32) -> i32 {
    prio - DEFAULT_PRIO
}

/*
 * Convert nice value [19,-20] to rlimit style value [1,40].
 */
#[inline]
pub fn nice_to_rlimit(nice: isize) -> isize {
    MAX_NICE as isize - nice + 1
}

/*
 * Convert rlimit style value [1,40] to nice value [-20, 19].
 */
#[inline]
pub fn rlimit_to_nice(prio: isize) -> isize {
    MAX_NICE as isize - prio + 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
