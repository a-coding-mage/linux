// SPDX-License-Identifier: GPL-2.0-only
/*
 * ratelimit.c - Do something with rate limit.
 *
 * Isolated from kernel/printk.c by Dave Young <hidave.darkstar@gmail.com>
 *
 * 2008-05-01 rewrite the function and use a ratelimit_state data struct as
 * parameter. Now every user can use their own standalone ratelimit_state.
 */

// Dependencies: linux/ratelimit.h, linux/jiffies.h, linux/export.h.

use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
use core::ptr::{addr_of, addr_of_mut, read_volatile};
use kernel::bindings::{atomic_t, jiffies, ratelimit_state};

/// `RATELIMIT_MSG_ON_RELEASE`; `ratelimit_helpers.c` asserts the C value.
const RATELIMIT_MSG_ON_RELEASE: c_uint = 1 << 0;
/// `RATELIMIT_INITIALIZED`; `ratelimit_helpers.c` asserts the C value.
const RATELIMIT_INITIALIZED: c_uint = 1 << 1;

extern "C" {
    fn lupos_ratelimit_trylock_irqsave(rs: *mut ratelimit_state, flags: *mut c_ulong) -> bool;
    fn lupos_ratelimit_unlock_irqrestore(rs: *mut ratelimit_state, flags: c_ulong);
    fn lupos_ratelimit_atomic_read(v: *const atomic_t) -> c_int;
    fn lupos_ratelimit_atomic_set(v: *mut atomic_t, i: c_int);
    fn lupos_ratelimit_atomic_dec_return(v: *mut atomic_t) -> c_int;
    fn lupos_ratelimit_warn_uninitialized(interval: c_int, burst: c_int);
    fn lupos_ratelimit_report_suppressed(func: *const c_char, missed: c_int);
    fn lupos_ratelimit_reset_miss(rs: *mut ratelimit_state) -> c_int;
    fn lupos_ratelimit_inc_miss(rs: *mut ratelimit_state);
}

/// `time_is_before_jiffies(a)`, i.e. `time_after(jiffies, a)` on the
/// volatile jiffies counter, with the macro's wrapping signed comparison.
///
/// # Safety
/// Reads the kernel's `jiffies` variable.
unsafe fn time_is_before_jiffies(a: c_ulong) -> bool {
    // SAFETY: jiffies is a valid, always-mapped kernel variable read volatilely.
    let now = unsafe { read_volatile(addr_of!(jiffies)) };
    (a.wrapping_sub(now) as c_long) < 0
}

/*
 * __ratelimit - rate limiting
 * @rs: ratelimit_state data
 * @func: name of calling function
 *
 * This enforces a rate limit: not more than @rs->burst callbacks
 * in every @rs->interval
 *
 * RETURNS:
 * 0 means callbacks will be suppressed.
 * 1 means go ahead and do it.
 */
/// Returns 1 if the caller may proceed, 0 if the callback is suppressed.
///
/// # Safety
/// `rs` points to a valid, initialized `struct ratelimit_state`; `func` is a
/// NUL-terminated name used only for the suppression report.
#[no_mangle]
pub unsafe extern "C" fn ___ratelimit(rs: *mut ratelimit_state, func: *const c_char) -> c_int {
    // SAFETY: rs points to a live ratelimit_state as required by the C API;
    // every native field access below uses that pointer only.
    unsafe {
        /* Paired with WRITE_ONCE() in .proc_handler().
         * Changing two values separately could be inconsistent
         * and some message could be lost.  (See: net_ratelimit_state).
         */
        let interval: c_int = read_volatile(addr_of!((*rs).interval));
        let burst: c_int = read_volatile(addr_of!((*rs).burst));
        let mut flags: c_ulong = 0;
        let mut ret: c_int = 0;

        'nolock_ret: {
            'unlock_ret: {
                /*
                 * Zero interval says never limit, otherwise, non-positive burst
                 * says always limit.
                 */
                if interval <= 0 || burst <= 0 {
                    lupos_ratelimit_warn_uninitialized(interval, burst);
                    ret = (interval == 0 || burst > 0) as c_int;
                    if read_volatile(addr_of!((*rs).flags)) & RATELIMIT_INITIALIZED == 0
                        || (interval == 0 && burst == 0)
                        || !lupos_ratelimit_trylock_irqsave(rs, &mut flags)
                    {
                        break 'nolock_ret;
                    }

                    /* Force re-initialization once re-enabled. */
                    (*rs).flags &= !RATELIMIT_INITIALIZED;
                    break 'unlock_ret;
                }

                /*
                 * If we contend on this state's lock then just check if
                 * the current burst is used or not. It might cause
                 * false positive when we are past the interval and
                 * the current lock owner is just about to reset it.
                 */
                if !lupos_ratelimit_trylock_irqsave(rs, &mut flags) {
                    if read_volatile(addr_of!((*rs).flags)) & RATELIMIT_INITIALIZED != 0
                        && lupos_ratelimit_atomic_read(addr_of!((*rs).rs_n_left)) > 0
                        && lupos_ratelimit_atomic_dec_return(addr_of_mut!((*rs).rs_n_left)) >= 0
                    {
                        ret = 1;
                    }
                    break 'nolock_ret;
                }

                if (*rs).flags & RATELIMIT_INITIALIZED == 0 {
                    (*rs).begin = read_volatile(addr_of!(jiffies));
                    (*rs).flags |= RATELIMIT_INITIALIZED;
                    lupos_ratelimit_atomic_set(addr_of_mut!((*rs).rs_n_left), (*rs).burst);
                }

                if time_is_before_jiffies((*rs).begin.wrapping_add(interval as c_ulong)) {
                    /*
                     * Reset rs_n_left ASAP to reduce false positives
                     * in parallel calls, see above.
                     */
                    lupos_ratelimit_atomic_set(addr_of_mut!((*rs).rs_n_left), (*rs).burst);
                    (*rs).begin = read_volatile(addr_of!(jiffies));

                    if (*rs).flags & RATELIMIT_MSG_ON_RELEASE == 0 {
                        let m: c_int = lupos_ratelimit_reset_miss(rs);
                        if m != 0 {
                            lupos_ratelimit_report_suppressed(func, m);
                        }
                    }
                }

                /* Note that the burst might be taken by a parallel call. */
                if lupos_ratelimit_atomic_read(addr_of!((*rs).rs_n_left)) > 0
                    && lupos_ratelimit_atomic_dec_return(addr_of_mut!((*rs).rs_n_left)) >= 0
                {
                    ret = 1;
                }
            }
            // unlock_ret:
            lupos_ratelimit_unlock_irqrestore(rs, flags);
        }
        // nolock_ret:
        if ret == 0 {
            lupos_ratelimit_inc_miss(rs);
        }

        ret
    }
}
// EXPORT_SYMBOL(___ratelimit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
