// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/err.h, linux/bug.h, linux/atomic.h, linux/errseq.h,
// linux/log2.h.


/*
 * An errseq_t is a way of recording errors in one place, and allowing any
 * number of "subscribers" to tell whether it has changed since a previous
 * point where it was sampled.
 *
 * It's implemented as an unsigned 32-bit value. The low order bits are
 * designated to hold an error code (between 0 and -MAX_ERRNO). The upper bits
 * are used as a counter. This is done with atomics instead of locking so that
 * these functions can be called from any context.
 *
 * The general idea is for consumers to sample an errseq_t value. That value
 * can later be used to tell whether any new errors have occurred since that
 * sampling was done.
 *
 * Note that there is a risk of collisions if new errors are being recorded
 * frequently, since we have so few bits to use as a counter.
 *
 * To mitigate this, one bit is used as a flag to tell whether the value has
 * been sampled since a new value was recorded. That allows us to avoid bumping
 * the counter if no one has sampled it since the last time an error was
 * recorded.
 *
 * A new errseq_t should always be zeroed out.  A errseq_t value of all zeroes
 * is the special (but common) case where there has never been an error. An all
 * zero value thus serves as the "epoch" if one wishes to know whether there
 * has ever been an error set since it was first initialized.
 */

use core::ptr::read_volatile;
use kernel::ffi::c_int;

/// `errseq_t`, an unsigned 32-bit value.
#[allow(non_camel_case_types)]
type errseq_t = u32;

extern "C" {
    fn lupos_errseq_cmpxchg(eseq: *mut errseq_t, old: errseq_t, new: errseq_t) -> errseq_t;
    fn lupos_errseq_warn_bad_err(err: c_int);
}

/// MAX_ERRNO from linux/err.h; lib/errseq_helpers.c asserts the derived
/// ERRSEQ_* values.
const MAX_ERRNO: u32 = 4095;

/* The low bits are designated for error code (max of MAX_ERRNO) */
const ERRSEQ_SHIFT: u32 = (u32::BITS - 1 - MAX_ERRNO.leading_zeros()) + 1;

/* This bit is used as a flag to indicate whether the value has been seen */
const ERRSEQ_SEEN: errseq_t = 1 << ERRSEQ_SHIFT;

/* Leverage macro ERRSEQ_SEEN to define errno mask macro here */
const ERRNO_MASK: errseq_t = ERRSEQ_SEEN - 1;

/* The lowest bit of the counter */
const ERRSEQ_CTR_INC: errseq_t = 1 << (ERRSEQ_SHIFT + 1);

/**
 * errseq_set - set a errseq_t for later reporting
 * @eseq: errseq_t field that should be set
 * @err: error to set (must be between -1 and -MAX_ERRNO)
 *
 * This function sets the error in @eseq, and increments the sequence counter
 * if the last sequence was sampled at some point in the past.
 *
 * Any error set will always overwrite an existing error.
 *
 * Return: The previous value, primarily for debugging purposes. The
 * return value should not be used as a previously sampled value in later
 * calls as it will not have the SEEN flag set.
 */
/// # Safety
/// `eseq` points to a valid errseq_t.
#[no_mangle]
pub unsafe extern "C" fn errseq_set(eseq: *mut errseq_t, err: c_int) -> errseq_t {
    let mut cur: errseq_t;

    // SAFETY: eseq is valid; updates use the native fully ordered cmpxchg().
    unsafe {
        /*
         * Ensure the error code actually fits where we want it to go. If it
         * doesn't then just throw a warning and don't record anything. We
         * also don't accept zero here as that would effectively clear a
         * previous error.
         */
        let mut old: errseq_t = read_volatile(eseq);

        if err == 0 || err.wrapping_neg() as u32 > MAX_ERRNO {
            lupos_errseq_warn_bad_err(err);
            return old;
        }

        loop {
            /* Clear out error bits and set new error */
            let mut new: errseq_t = (old & !(ERRNO_MASK | ERRSEQ_SEEN)) | err.wrapping_neg() as u32;

            /* Only increment if someone has looked at it */
            if old & ERRSEQ_SEEN != 0 {
                new = new.wrapping_add(ERRSEQ_CTR_INC);
            }

            /* If there would be no change, then call it done */
            if new == old {
                cur = new;
                break;
            }

            /* Try to swap the new value into place */
            cur = lupos_errseq_cmpxchg(eseq, old, new);

            /*
             * Call it success if we did the swap or someone else beat us
             * to it for the same value.
             */
            if cur == old || cur == new {
                break;
            }

            /* Raced with an update, try again */
            old = cur;
        }
    }
    cur
}
// EXPORT_SYMBOL(errseq_set);

/**
 * errseq_sample() - Grab current errseq_t value.
 * @eseq: Pointer to errseq_t to be sampled.
 *
 * This function allows callers to initialise their errseq_t variable.
 * If the error has been "seen", new callers will not see an old error.
 * If there is an unseen error in @eseq, the caller of this function will
 * see it the next time it checks for an error.
 *
 * Context: Any context.
 * Return: The current errseq value.
 */
/// # Safety
/// `eseq` points to a valid errseq_t.
#[no_mangle]
pub unsafe extern "C" fn errseq_sample(eseq: *mut errseq_t) -> errseq_t {
    // SAFETY: eseq is valid; READ_ONCE() is a volatile load.
    let mut old = unsafe { read_volatile(eseq) };

    /* If nobody has seen this error yet, then we can be the first. */
    if old & ERRSEQ_SEEN == 0 {
        old = 0;
    }
    old
}
// EXPORT_SYMBOL(errseq_sample);

/**
 * errseq_check() - Has an error occurred since a particular sample point?
 * @eseq: Pointer to errseq_t value to be checked.
 * @since: Previously-sampled errseq_t from which to check.
 *
 * Grab the value that eseq points to, and see if it has changed @since
 * the given value was sampled. The @since value is not advanced, so there
 * is no need to mark the value as seen.
 *
 * Return: The latest error set in the errseq_t or 0 if it hasn't changed.
 */
/// # Safety
/// `eseq` points to a valid errseq_t.
#[no_mangle]
pub unsafe extern "C" fn errseq_check(eseq: *mut errseq_t, since: errseq_t) -> c_int {
    // SAFETY: eseq is valid; READ_ONCE() is a volatile load.
    let cur = unsafe { read_volatile(eseq) };

    if cur == since {
        return 0;
    }
    (cur & ERRNO_MASK).wrapping_neg() as c_int
}
// EXPORT_SYMBOL(errseq_check);

/**
 * errseq_check_and_advance() - Check an errseq_t and advance to current value.
 * @eseq: Pointer to value being checked and reported.
 * @since: Pointer to previously-sampled errseq_t to check against and advance.
 *
 * Grab the eseq value, and see whether it matches the value that @since
 * points to. If it does, then just return 0.
 *
 * If it doesn't, then the value has changed. Set the "seen" flag, and try to
 * swap it into place as the new eseq value. Then, set that value as the new
 * "since" value, and return whatever the error portion is set to.
 *
 * Note that no locking is provided here for concurrent updates to the "since"
 * value. The caller must provide that if necessary. Because of this, callers
 * may want to do a lockless errseq_check before taking the lock and calling
 * this.
 *
 * Return: Negative errno if one has been stored, or 0 if no new error has
 * occurred.
 */
/// # Safety
/// `eseq` and `since` point to valid errseq_t values.
#[no_mangle]
pub unsafe extern "C" fn errseq_check_and_advance(eseq: *mut errseq_t, since: *mut errseq_t) -> c_int {
    let mut err: c_int = 0;

    // SAFETY: both pointers are valid; the swap uses the native cmpxchg().
    unsafe {
        /*
         * Most callers will want to use the inline wrapper to check this,
         * so that the common case of no error is handled without needing
         * to take the lock that protects the "since" value.
         */
        let old = read_volatile(eseq);
        if old != *since {
            /*
             * Set the flag and try to swap it into place if it has
             * changed.
             *
             * We don't care about the outcome of the swap here. If the
             * swap doesn't occur, then it has either been updated by a
             * writer who is altering the value in some way (updating
             * counter or resetting the error), or another reader who is
             * just setting the "seen" flag. Either outcome is OK, and we
             * can advance "since" and return an error based on what we
             * have.
             */
            let new = old | ERRSEQ_SEEN;
            if new != old {
                lupos_errseq_cmpxchg(eseq, old, new);
            }
            *since = new;
            err = (new & ERRNO_MASK).wrapping_neg() as c_int;
        }
    }
    err
}
// EXPORT_SYMBOL(errseq_check_and_advance);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
