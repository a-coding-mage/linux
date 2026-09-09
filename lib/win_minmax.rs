// SPDX-License-Identifier: GPL-2.0
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

#[repr(C)]
pub struct minmax_sample {
    pub t: u32,
    pub v: u32,
}

#[repr(C)]
pub struct minmax {
    pub s: [minmax_sample; 3],
}

extern "C" {
    pub fn minmax_reset(m: *mut minmax, t: u32, meas: u32) -> u32;
}

/* As time advances, update the 1st, 2nd, and 3rd choices. */
unsafe fn minmax_subwin_update(m: *mut minmax, win: u32, val: *const minmax_sample) -> u32 {
    let dt = (*val).t.wrapping_sub((*m).s[0].t);

    if dt > win {
        /*
         * Passed entire window without a new val so make 2nd
         * choice the new val & 3rd choice the new 2nd choice.
         * we may have to iterate this since our 2nd choice
         * may also be outside the window (we checked on entry
         * that the third choice was in the window).
         */
        (*m).s[0] = (*m).s[1];
        (*m).s[1] = (*m).s[2];
        (*m).s[2] = *val;
        if (*val).t.wrapping_sub((*m).s[0].t) > win {
            (*m).s[0] = (*m).s[1];
            (*m).s[1] = (*m).s[2];
            (*m).s[2] = *val;
        }
    } else if (*m).s[1].t == (*m).s[0].t && dt > win / 4 {
        /*
         * We've passed a quarter of the window without a new val
         * so take a 2nd choice from the 2nd quarter of the window.
         */
        (*m).s[1] = *val;
        (*m).s[2] = (*m).s[1];
    } else if (*m).s[2].t == (*m).s[1].t && dt > win / 2 {
        /*
         * We've passed half the window without finding a new val
         * so take a 3rd choice from the last half of the window
         */
        (*m).s[2] = *val;
    }
    (*m).s[0].v
}

/* Check if new measurement updates the 1st, 2nd or 3rd choice max. */
pub unsafe fn minmax_running_max(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32 {
    let val = minmax_sample { t, v: meas };

    if val.v >= (*m).s[0].v || val.t.wrapping_sub((*m).s[2].t) > win {
        return minmax_reset(m, t, meas);
    }

    if val.v >= (*m).s[1].v {
        (*m).s[1] = val;
        (*m).s[2] = (*m).s[1];
    } else if val.v >= (*m).s[2].v {
        (*m).s[2] = val;
    }

    minmax_subwin_update(m, win, &val)
}

/* Check if new measurement updates the 1st, 2nd or 3rd choice min. */
pub unsafe fn minmax_running_min(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32 {
    let val = minmax_sample { t, v: meas };

    if val.v <= (*m).s[0].v || val.t.wrapping_sub((*m).s[2].t) > win {
        return minmax_reset(m, t, meas);
    }

    if val.v <= (*m).s[1].v {
        (*m).s[1] = val;
        (*m).s[2] = (*m).s[1];
    } else if val.v <= (*m).s[2].v {
        (*m).s[2] = val;
    }

    minmax_subwin_update(m, win, &val)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
