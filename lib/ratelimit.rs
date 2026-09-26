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

use core::ffi::{c_char, c_int, c_ulong};
use core::ptr::{addr_of, addr_of_mut, read_volatile};

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
#[no_mangle]
pub unsafe extern "C" fn ___ratelimit(rs: *mut ratelimit_state, func: *const c_char) -> c_int {
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
                WARN_ONCE!(
                    interval < 0 || burst < 0,
                    c"Negative interval (%d) or burst (%d): Uninitialized ratelimit_state structure?\n",
                    interval,
                    burst
                );
                ret = (interval == 0 || burst > 0) as c_int;
                if read_volatile(addr_of!((*rs).flags)) & RATELIMIT_INITIALIZED == 0
                    || (interval == 0 && burst == 0)
                    || !raw_spin_trylock_irqsave(addr_of_mut!((*rs).lock), &mut flags)
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
            if !raw_spin_trylock_irqsave(addr_of_mut!((*rs).lock), &mut flags) {
                if read_volatile(addr_of!((*rs).flags)) & RATELIMIT_INITIALIZED != 0
                    && atomic_read(addr_of!((*rs).rs_n_left)) > 0
                    && atomic_dec_return(addr_of_mut!((*rs).rs_n_left)) >= 0
                {
                    ret = 1;
                }
                break 'nolock_ret;
            }

            if (*rs).flags & RATELIMIT_INITIALIZED == 0 {
                (*rs).begin = read_volatile(addr_of!(jiffies));
                (*rs).flags |= RATELIMIT_INITIALIZED;
                atomic_set(addr_of_mut!((*rs).rs_n_left), (*rs).burst);
            }

            if time_is_before_jiffies((*rs).begin.wrapping_add(interval as c_ulong)) {
                /*
                 * Reset rs_n_left ASAP to reduce false positives
                 * in parallel calls, see above.
                 */
                atomic_set(addr_of_mut!((*rs).rs_n_left), (*rs).burst);
                (*rs).begin = read_volatile(addr_of!(jiffies));

                if (*rs).flags & RATELIMIT_MSG_ON_RELEASE == 0 {
                    let m: c_int = ratelimit_state_reset_miss(rs);
                    if m != 0 {
                        printk_deferred(c"\x014%s: %d callbacks suppressed\n".as_ptr(), func, m);
                    }
                }
            }

            /* Note that the burst might be taken by a parallel call. */
            if atomic_read(addr_of!((*rs).rs_n_left)) > 0
                && atomic_dec_return(addr_of_mut!((*rs).rs_n_left)) >= 0
            {
                ret = 1;
            }
        }
        // unlock_ret:
        raw_spin_unlock_irqrestore(addr_of_mut!((*rs).lock), flags);
    }
    // nolock_ret:
    if ret == 0 {
        ratelimit_state_inc_miss(rs);
    }

    ret
}
// EXPORT_SYMBOL(___ratelimit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
