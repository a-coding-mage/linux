// SPDX-License-Identifier: GPL-2.0+
/*
 * Based on clocksource code. See commit 74d23cc704d1
 */

// Types and symbols provided by the corresponding Linux timecounter headers
// are intentionally left as external dependencies.

pub unsafe fn timecounter_init(
    tc: *mut timecounter,
    cc: *mut cyclecounter,
    start_tstamp: u64,
) {
    (*tc).cc = cc;
    (*tc).cycle_last = ((*cc).read)(cc);
    (*tc).nsec = start_tstamp;
    (*tc).mask = (1u64.wrapping_shl((*cc).shift)) .wrapping_sub(1);
    (*tc).frac = 0;
}

/**
 * timecounter_read_delta - get nanoseconds since last call of this function
 * @tc:         Pointer to time counter
 *
 * When the underlying cycle counter runs over, this will be handled
 * correctly as long as it does not run over more than once between
 * calls.
 *
 * The first call to this function for a new time counter initializes
 * the time tracking and returns an undefined result.
 */
unsafe fn timecounter_read_delta(tc: *mut timecounter) -> u64 {
    let cycle_now: u64;
    let cycle_delta: u64;
    let ns_offset: u64;

    /* read cycle counter: */
    cycle_now = ((*(*tc).cc).read)((*tc).cc);

    /* calculate the delta since the last timecounter_read_delta(): */
    cycle_delta = cycle_now
        .wrapping_sub((*tc).cycle_last)
        & (*(*tc).cc).mask;

    /* convert to nanoseconds: */
    ns_offset = cyclecounter_cyc2ns(
        (*tc).cc,
        cycle_delta,
        (*tc).mask,
        &mut (*tc).frac,
    );

    /* update time stamp of timecounter_read_delta() call: */
    (*tc).cycle_last = cycle_now;

    ns_offset
}

pub unsafe fn timecounter_read(tc: *mut timecounter) -> u64 {
    let mut nsec: u64;

    /* increment time by nanoseconds since last call */
    nsec = timecounter_read_delta(tc);
    nsec = nsec.wrapping_add((*tc).nsec);
    (*tc).nsec = nsec;

    nsec
}

// External dependency supplied by the cyclecounter implementation.
extern "C" {
    fn cyclecounter_cyc2ns(
        cc: *mut cyclecounter,
        cycles: u64,
        mask: u64,
        frac: *mut u64,
    ) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
