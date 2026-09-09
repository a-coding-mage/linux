// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_char;
use core::sync::atomic::{AtomicU32, Ordering};

pub type errseq_t = u32;

extern "C" {
    fn WARN(condition: bool, format: *const c_char, ...) -> bool;
}

const MAX_ERRNO: u32 = 4095;

/* The low bits are designated for error code (max of MAX_ERRNO) */
const ERRSEQ_SHIFT: u32 = 12;

/* This bit is used as a flag to indicate whether the value has been seen */
const ERRSEQ_SEEN: errseq_t = 1 << ERRSEQ_SHIFT;

/* Leverage macro ERRSEQ_SEEN to define errno mask macro here */
const ERRNO_MASK: errseq_t = ERRSEQ_SEEN - 1;

/* The lowest bit of the counter */
const ERRSEQ_CTR_INC: errseq_t = 1 << (ERRSEQ_SHIFT + 1);

#[inline]
unsafe fn read_once(value: *const errseq_t) -> errseq_t {
    core::ptr::read_volatile(value)
}

#[inline]
unsafe fn cmpxchg(value: *mut errseq_t, old: errseq_t, new: errseq_t) -> errseq_t {
    (&*(value as *mut AtomicU32)).compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap_or_else(|actual| actual)
}

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
pub unsafe fn errseq_set(eseq: *mut errseq_t, err: i32) -> errseq_t {
    let mut cur: errseq_t;
    let mut old: errseq_t;

    old = read_once(eseq);

    if err == 0 || (-(err as i64)) as u64 > MAX_ERRNO as u64 {
        WARN(true, b"err = %d\n\0".as_ptr() as *const c_char, err);
        return old;
    }

    loop {
        let mut new: errseq_t;

        /* Clear out error bits and set new error */
        new = (old & !(ERRNO_MASK | ERRSEQ_SEEN)) | (-(err as i64) as errseq_t);

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
        cur = cmpxchg(eseq, old, new);

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
    cur
}

/**
 * errseq_sample() - Grab current errseq_t value.
 * @eseq: Pointer to errseq_t to be sampled.
 */
pub unsafe fn errseq_sample(eseq: *mut errseq_t) -> errseq_t {
    let mut old = read_once(eseq);

    /* If nobody has seen this error yet, then we can be the first. */
    if old & ERRSEQ_SEEN == 0 {
        old = 0;
    }
    old
}

/**
 * errseq_check() - Has an error occurred since a particular sample point?
 */
pub unsafe fn errseq_check(eseq: *mut errseq_t, since: errseq_t) -> i32 {
    let cur = read_once(eseq);

    if cur == since {
        return 0;
    }
    -((cur & ERRNO_MASK) as i32)
}

/**
 * errseq_check_and_advance() - Check an errseq_t and advance to current value.
 */
pub unsafe fn errseq_check_and_advance(eseq: *mut errseq_t, since: *mut errseq_t) -> i32 {
    let mut err = 0;
    let old = read_once(eseq);
    if old != *since {
        let new = old | ERRSEQ_SEEN;
        if new != old {
            cmpxchg(eseq, old, new);
        }
        *since = new;
        err = -((new & ERRNO_MASK) as i32);
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
