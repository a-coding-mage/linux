// SPDX-License-Identifier: GPL-2.0
//! Constant-space windowed minimum and maximum tracking.
/*
 * lib/minmax.c: windowed min/max tracker
 *
 * Kathleen Nichols' algorithm for tracking the minimum (or maximum)
 * value of a data stream over some fixed time interval.  (E.g.,
 * the minimum RTT over the past five minutes.) It uses constant
 * space and constant time per update yet almost always delivers the
 * same minimum as an implementation that has to keep all the data in
 * the window.
 *
 * The algorithm keeps track of the best, 2nd best & 3rd best min
 * values, maintaining an invariant that the measurement time of
 * the n'th best >= n-1'th best. It also makes sure that the three
 * values are widely separated in the time window since that bounds
 * the worse case error when that data is monotonically increasing
 * over the window.
 */

#[path = "../include/linux/win_minmax_header.rs"]
/// Original generated C types, inline operations and declarations.
pub mod declarations;
use declarations::{minmax, minmax_sample};
use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut};

// C struct assignment may transport indeterminate fields without reading their
// scalar values. MaybeUninit preserves that domain even before a later overwrite.
unsafe fn copy_sample(dst: *mut minmax_sample, src: *const minmax_sample) {
    // SAFETY: caller supplies aligned readable/writable sample storage; neither
    // the raw projections nor this MaybeUninit copy require initialized fields.
    unsafe { *dst.cast::<MaybeUninit<minmax_sample>>() = *src.cast::<MaybeUninit<minmax_sample>>() };
}

/* As time advances, update the 1st, 2nd, and 3rd choices. */
unsafe fn minmax_subwin_update(m: *mut minmax, win: u32, val: minmax_sample) -> u32 {
    // SAFETY: the caller guarantees storage/access and initialization of fields
    // actually read. Raw projections never form a reference to the whole tracker;
    // raw sample copies preserve possibly uninitialized fields, as in C.
    unsafe {
        let dt = val.t.wrapping_sub((*m).s[0].t);

        if dt > win {
            /*
             * Passed entire window without a new val so make 2nd
             * choice the new val & 3rd choice the new 2nd choice.
             * we may have to iterate this since our 2nd choice
             * may also be outside the window (we checked on entry
             * that the third choice was in the window).
             */
            copy_sample(addr_of_mut!((*m).s[0]), addr_of!((*m).s[1]));
            copy_sample(addr_of_mut!((*m).s[1]), addr_of!((*m).s[2]));
            (*m).s[2] = val;
            if val.t.wrapping_sub((*m).s[0].t) > win {
                copy_sample(addr_of_mut!((*m).s[0]), addr_of!((*m).s[1]));
                copy_sample(addr_of_mut!((*m).s[1]), addr_of!((*m).s[2]));
                (*m).s[2] = val;
            }
        } else if (*m).s[1].t == (*m).s[0].t && dt > win / 4 {
            /*
             * We've passed a quarter of the window without a new val
             * so take a 2nd choice from the 2nd quarter of the window.
             */
            (*m).s[1] = val;
            (*m).s[2] = val;
        } else if (*m).s[2].t == (*m).s[1].t && dt > win / 2 {
            /*
             * We've passed half the window without finding a new val
             * so take a 3rd choice from the last half of the window
             */
            (*m).s[2] = val;
        }
        (*m).s[0].v
    }
}

/* Check if new measurement updates the 1st, 2nd or 3rd choice max. */
/// Updates the running maximum, preserving unsigned timestamp wraparound.
///
/// # Safety
/// `m` must point to aligned writable tracker storage, exclusively accessible
/// for this call. Only fields read along the executed C control-flow path need
/// initialization. A value reset reads only `s[0].v`; an expiry reset also
/// reads `s[2].t`. Fields overwritten before use need not be initialized.
#[no_mangle]
pub unsafe extern "C" fn minmax_running_max(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32 {
    // SAFETY: the caller guarantees storage/access and initialization of fields
    // actually read. Raw projections never form a reference to the whole tracker;
    // raw sample copies preserve possibly uninitialized fields, as in C.
    unsafe {
        let val = minmax_sample { t, v: meas };

        if val.v >= (*m).s[0].v || val.t.wrapping_sub((*m).s[2].t) > win {
            (*m).s = [val; 3];
            return meas;
        }

        if val.v >= (*m).s[1].v {
            (*m).s[1] = val;
            (*m).s[2] = val;
        } else if val.v >= (*m).s[2].v {
            (*m).s[2] = val;
        }

        minmax_subwin_update(m, win, val)
    }
}

/* Check if new measurement updates the 1st, 2nd or 3rd choice min. */
/// Updates the running minimum, preserving unsigned timestamp wraparound.
///
/// # Safety
/// `m` must point to aligned writable tracker storage, exclusively accessible
/// for this call. Only fields read along the executed C control-flow path need
/// initialization. A value reset reads only `s[0].v`; an expiry reset also
/// reads `s[2].t`. Fields overwritten before use need not be initialized.
#[no_mangle]
pub unsafe extern "C" fn minmax_running_min(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32 {
    // SAFETY: the caller guarantees storage/access and initialization of fields
    // actually read. Raw projections never form a reference to the whole tracker;
    // raw sample copies preserve possibly uninitialized fields, as in C.
    unsafe {
        let val = minmax_sample { t, v: meas };

        if val.v <= (*m).s[0].v || val.t.wrapping_sub((*m).s[2].t) > win {
            (*m).s = [val; 3];
            return meas;
        }

        if val.v <= (*m).s[1].v {
            (*m).s[1] = val;
            (*m).s[2] = val;
        } else if val.v <= (*m).s[2].v {
            (*m).s[2] = val;
        }

        minmax_subwin_update(m, win, val)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
