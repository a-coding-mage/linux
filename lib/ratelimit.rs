// SPDX-License-Identifier: GPL-2.0-only
/*
 * ratelimit.c - Do something with rate limit.
 *
 * Isolated from kernel/printk.c by Dave Young <hidave.darkstar@gmail.com>
 *
 * 2008-05-01 rewrite the function and use a ratelimit_state data struct as
 * parameter. Now every user can use their own standalone ratelimit_state.
 */

// Dependencies supplied by the Linux kernel translation environment:
// linux/ratelimit.h, linux/jiffies.h, and linux/export.h.

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
pub unsafe extern "C" fn ___ratelimit(
    rs: *mut ratelimit_state,
    func: *const core::ffi::c_char,
) -> i32 {
    /* Paired with WRITE_ONCE() in .proc_handler().
     * Changing two values separately could be inconsistent
     * and some message could be lost.  (See: net_ratelimit_state).
     */
    let interval = READ_ONCE((*rs).interval);
    let burst = READ_ONCE((*rs).burst);
    let mut flags: unsigned_long = 0;
    let mut ret: i32 = 0;

    /*
     * Zero interval says never limit, otherwise, non-positive burst
     * says always limit.
     */
    if interval <= 0 || burst <= 0 {
        WARN_ONCE(
            interval < 0 || burst < 0,
            "Negative interval (%d) or burst (%d): Uninitialized ratelimit_state structure?\n",
            interval,
            burst,
        );
        ret = if interval == 0 || burst > 0 { 1 } else { 0 };
        if !(READ_ONCE((*rs).flags) & RATELIMIT_INITIALIZED != 0)
            || (!interval && !burst)
            || !raw_spin_trylock_irqsave(&mut (*rs).lock, &mut flags)
        {
            goto nolock_ret;
        }

        /* Force re-initialization once re-enabled. */
        (*rs).flags &= !RATELIMIT_INITIALIZED;
        goto unlock_ret;
    }

    /*
     * If we contend on this state's lock then just check if
     * the current burst is used or not. It might cause
     * false positive when we are past the interval and
     * the current lock owner is just about to reset it.
     */
    if !raw_spin_trylock_irqsave(&mut (*rs).lock, &mut flags) {
        if READ_ONCE((*rs).flags) & RATELIMIT_INITIALIZED != 0
            && (*rs).rs_n_left.load(core::sync::atomic::Ordering::Relaxed) > 0
            && (*rs).rs_n_left.fetch_sub(1, core::sync::atomic::Ordering::Relaxed) - 1 >= 0
        {
            ret = 1;
        }
        goto nolock_ret;
    }

    if (*rs).flags & RATELIMIT_INITIALIZED == 0 {
        (*rs).begin = jiffies;
        (*rs).flags |= RATELIMIT_INITIALIZED;
        (*rs).rs_n_left.store((*rs).burst, core::sync::atomic::Ordering::Relaxed);
    }

    if time_is_before_jiffies((*rs).begin + interval as unsigned_long) {
        let m: i32;

        /*
         * Reset rs_n_left ASAP to reduce false positives
         * in parallel calls, see above.
         */
        (*rs).rs_n_left.store((*rs).burst, core::sync::atomic::Ordering::Relaxed);
        (*rs).begin = jiffies;

        if (*rs).flags & RATELIMIT_MSG_ON_RELEASE == 0 {
            m = ratelimit_state_reset_miss(rs);
            if m != 0 {
                printk_deferred(KERN_WARNING, b"%s: %d callbacks suppressed\0".as_ptr(), func, m);
            }
        }
    }

    /* Note that the burst might be taken by a parallel call. */
    if (*rs).rs_n_left.load(core::sync::atomic::Ordering::Relaxed) > 0
        && (*rs).rs_n_left.fetch_sub(1, core::sync::atomic::Ordering::Relaxed) - 1 >= 0
    {
        ret = 1;
    }

unlock_ret:
    raw_spin_unlock_irqrestore(&mut (*rs).lock, flags);

nolock_ret:
    if ret == 0 {
        ratelimit_state_inc_miss(rs);
    }

    ret
}

// EXPORT_SYMBOL(___ratelimit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
