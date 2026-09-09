/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 1993 Linus Torvalds
 *
 * Delay routines, using a pre-computed "loops_per_jiffy" value.
 * Sleep routines using timer list timers or hrtimers.
 *
 * Dependencies supplied by the Linux math, scheduler, jiffies, and
 * architecture-specific delay headers are referenced but not defined here.
 */

unsafe extern "C" {
    pub static mut loops_per_jiffy: usize;

    pub fn delay_read_timer(t: *mut usize) -> bool;

    pub fn udelay(usecs: usize);

    pub static mut lpj_fine: usize;
    pub fn calibrate_delay();
    pub fn calibrate_delay_is_known() -> usize;
    pub fn calibration_delay_done();
    pub fn msleep(msecs: u32);
    pub fn msleep_interruptible(msecs: u32) -> usize;
    pub fn usleep_range_state(min: usize, max: usize, state: u32);
}

/* Architecture-specific headers may override this value. */
pub const MAX_UDELAY_MS: usize = 5;

/*
 * mdelay - Inserting a delay based on milliseconds with busy waiting
 * @n: requested delay in milliseconds
 *
 * See udelay() for basic information about mdelay() and its variants.
 * The C macro uses compile-time constant detection to optimize short delays.
 */
#[macro_export]
macro_rules! mdelay {
    ($n:expr) => {{
        let __ms: usize = $n as usize;
        if __ms <= $crate::MAX_UDELAY_MS {
            $crate::udelay(__ms.wrapping_mul(1000));
        } else {
            let mut __remaining = __ms;
            while __remaining != 0 {
                $crate::udelay(1000);
                __remaining = __remaining.wrapping_sub(1);
            }
        }
    }};
}

#[inline]
pub unsafe fn ndelay(x: usize) {
    // DIV_ROUND_UP(x, 1000), supplied by the Linux math dependency.
    udelay(x.wrapping_add(999) / 1000);
}

#[inline]
pub unsafe fn usleep_range(min: usize, max: usize) {
    usleep_range_state(min, max, TASK_UNINTERRUPTIBLE);
}

#[inline]
pub unsafe fn usleep_range_idle(min: usize, max: usize) {
    usleep_range_state(min, max, TASK_IDLE);
}

#[inline]
pub unsafe fn ssleep(seconds: u32) {
    msleep(seconds.wrapping_mul(1000));
}

pub const max_slack_shift: u32 = 2;
// USLEEP_RANGE_UPPER_BOUND = ((TICK_NSEC << max_slack_shift) / NSEC_PER_USEC)
pub const USLEEP_RANGE_UPPER_BOUND: usize =
    (TICK_NSEC << max_slack_shift) / NSEC_PER_USEC;

#[inline]
pub unsafe fn fsleep(usecs: usize) {
    if usecs <= 10 {
        udelay(usecs);
    } else if usecs < USLEEP_RANGE_UPPER_BOUND {
        usleep_range(usecs, usecs.wrapping_add(usecs >> max_slack_shift));
    } else {
        // DIV_ROUND_UP(usecs, USEC_PER_MSEC), supplied by the Linux math dependency.
        msleep(usecs.wrapping_add(USEC_PER_MSEC - 1) / USEC_PER_MSEC as u32);
    }
}

/* TASK_UNINTERRUPTIBLE, TASK_IDLE, TICK_NSEC, NSEC_PER_USEC, and
 * USEC_PER_MSEC are supplied by the scheduler and jiffies dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
