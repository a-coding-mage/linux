/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012-2014 The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: kernel types, timekeeping, delays, errno, and I/O accessors.

/// Periodically poll and perform an operation until a condition is met or a
/// timeout occurs.
#[macro_export]
macro_rules! poll_timeout_us {
    ($op:expr, $cond:expr, $sleep_us:expr, $timeout_us:expr, $sleep_before_op:expr) => {{
        let __timeout_us: u64 = $timeout_us;
        let __sleep_us: usize = $sleep_us;
        let __timeout = ktime_add_us(ktime_get(), __timeout_us);
        let ___ret: i32;
        might_sleep_if(__sleep_us != 0);
        if $sleep_before_op && __sleep_us != 0 {
            usleep_range((__sleep_us >> 2) + 1, __sleep_us);
        }
        loop {
            let __expired = __timeout_us != 0
                && ktime_compare(ktime_get(), __timeout) > 0;
            // Guarantee `op` and `cond` are evaluated after timeout expired.
            barrier();
            $op;
            if $cond {
                ___ret = 0;
                break;
            }
            if __expired {
                ___ret = -ETIMEDOUT;
                break;
            }
            if __sleep_us != 0 {
                usleep_range((__sleep_us >> 2) + 1, __sleep_us);
            }
            cpu_relax();
        }
        ___ret
    }};
}

/// Atomic variant of `poll_timeout_us`, using delays rather than timekeeping.
#[macro_export]
macro_rules! poll_timeout_us_atomic {
    ($op:expr, $cond:expr, $delay_us:expr, $timeout_us:expr, $delay_before_op:expr) => {{
        let __timeout_us: u64 = $timeout_us;
        let mut __left_ns: i64 = (__timeout_us as i64).wrapping_mul(NSEC_PER_USEC as i64);
        let __delay_us: usize = $delay_us;
        let __delay_ns: u64 = (__delay_us as u64).wrapping_mul(NSEC_PER_USEC as u64);
        let ___ret: i32;
        if $delay_before_op && __delay_us != 0 {
            udelay(__delay_us);
            if __timeout_us != 0 {
                __left_ns = __left_ns.wrapping_sub(__delay_ns as i64);
            }
        }
        loop {
            let __expired = __timeout_us != 0 && __left_ns < 0;
            // Guarantee `op` and `cond` are evaluated after timeout expired.
            barrier();
            $op;
            if $cond {
                ___ret = 0;
                break;
            }
            if __expired {
                ___ret = -ETIMEDOUT;
                break;
            }
            if __delay_us != 0 {
                udelay(__delay_us);
                if __timeout_us != 0 {
                    __left_ns = __left_ns.wrapping_sub(__delay_ns as i64);
                }
            }
            cpu_relax();
            if __timeout_us != 0 {
                __left_ns = __left_ns.wrapping_sub(1);
            }
        }
        ___ret
    }};
}

#[macro_export]
macro_rules! read_poll_timeout {
    ($op:ident, $val:expr, $cond:expr, $sleep_us:expr, $timeout_us:expr, $sleep_before_read:expr, $($args:expr),* $(,)?) => {
        poll_timeout_us!($val = $op($($args),*), $cond, $sleep_us, $timeout_us, $sleep_before_read)
    };
}

#[macro_export]
macro_rules! read_poll_timeout_atomic {
    ($op:ident, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr, $delay_before_read:expr, $($args:expr),* $(,)?) => {
        poll_timeout_us_atomic!($val = $op($($args),*), $cond, $delay_us, $timeout_us, $delay_before_read)
    };
}

#[macro_export]
macro_rules! readx_poll_timeout {
    ($op:ident, $addr:expr, $val:expr, $cond:expr, $sleep_us:expr, $timeout_us:expr) => {
        read_poll_timeout!($op, $val, $cond, $sleep_us, $timeout_us, false, $addr)
    };
}

#[macro_export]
macro_rules! readx_poll_timeout_atomic {
    ($op:ident, $addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => {
        read_poll_timeout_atomic!($op, $val, $cond, $delay_us, $timeout_us, false, $addr)
    };
}

#[macro_export]
macro_rules! readb_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readb, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readb_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readb, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readw_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readw, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readw_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readw, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readl_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readl, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readl_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readl, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readq_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readq, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readq_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readq, $addr, $val, $cond, $delay_us, $timeout_us) }; }

#[macro_export]
macro_rules! readb_relaxed_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readb_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readb_relaxed_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readb_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readw_relaxed_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readw_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readw_relaxed_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readw_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readl_relaxed_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readl_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readl_relaxed_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readl_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readq_relaxed_poll_timeout { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout!(readq_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }
#[macro_export]
macro_rules! readq_relaxed_poll_timeout_atomic { ($addr:expr, $val:expr, $cond:expr, $delay_us:expr, $timeout_us:expr) => { readx_poll_timeout_atomic!(readq_relaxed, $addr, $val, $cond, $delay_us, $timeout_us) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
