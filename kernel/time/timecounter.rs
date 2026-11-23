// SPDX-License-Identifier: GPL-2.0+
/*
 * Based on clocksource code. See commit 74d23cc704d1
 */

// Dependencies: linux/timecounter.h.

/// `struct cyclecounter` from linux/timecounter.h, which the generated
/// bindings do not include; kernel/time/timecounter_helpers.c asserts the
/// layout.
#[repr(C)]
pub struct Cyclecounter {
    read: Option<unsafe extern "C" fn(*mut Cyclecounter) -> u64>,
    mask: u64,
    mult: u32,
    shift: u32,
}

/// `struct timecounter` from linux/timecounter.h.
#[repr(C)]
pub struct Timecounter {
    cc: *mut Cyclecounter,
    cycle_last: u64,
    nsec: u64,
    mask: u64,
    frac: u64,
}

/// cc->read(cc): the cycle counter's mandatory read callback.
///
/// # Safety
/// `cc` is a valid cycle counter with its read callback set.
unsafe fn cc_read(cc: *mut Cyclecounter) -> u64 {
    // SAFETY: the C API requires a read callback, which C calls unchecked.
    unsafe { (*cc).read.unwrap_unchecked()(cc) }
}

/// cyclecounter_cyc2ns() from linux/timecounter.h.
///
/// # Safety
/// `cc` is a valid cycle counter and `frac` points to writable storage.
unsafe fn cyclecounter_cyc2ns(cc: *const Cyclecounter, cycles: u64, mask: u64, frac: *mut u64) -> u64 {
    // SAFETY: forwarded caller contract.
    unsafe {
        let mut ns: u64 = cycles;

        ns = ns.wrapping_mul(u64::from((*cc).mult)).wrapping_add(*frac);
        *frac = ns & mask;
        // C's >> by an out-of-range shift is undefined; x86 masks it.
        ns.wrapping_shr((*cc).shift)
    }
}

/**
 * timecounter_init - initialize a time counter
 * @tc:			Pointer to time counter which is to be initialized/reset
 * @cc:			A cycle counter, ready to be used.
 * @start_tstamp:	Arbitrary initial time stamp.
 *
 * After this call the current cycle register (roughly) corresponds to
 * the initial time stamp. Every call to timecounter_read() increments
 * the time stamp counter by the number of elapsed nanoseconds.
 */
/// # Safety
/// `tc` is writable and `cc` a ready cycle counter.
#[no_mangle]
pub unsafe extern "C" fn timecounter_init(tc: *mut Timecounter, cc: *mut Cyclecounter, start_tstamp: u64) {
    // SAFETY: forwarded caller contract.
    unsafe {
        (*tc).cc = cc;
        (*tc).cycle_last = cc_read(cc);
        (*tc).nsec = start_tstamp;
        (*tc).mask = 1u64.wrapping_shl((*cc).shift).wrapping_sub(1);
        (*tc).frac = 0;
    }
}
// EXPORT_SYMBOL_GPL(timecounter_init);

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
/// # Safety
/// `tc` is an initialized time counter.
unsafe fn timecounter_read_delta(tc: *mut Timecounter) -> u64 {
    // SAFETY: forwarded caller contract.
    unsafe {
        /* read cycle counter: */
        let cycle_now = cc_read((*tc).cc);

        /* calculate the delta since the last timecounter_read_delta(): */
        let cycle_delta = cycle_now.wrapping_sub((*tc).cycle_last) & (*(*tc).cc).mask;

        /* convert to nanoseconds: */
        let ns_offset = cyclecounter_cyc2ns((*tc).cc, cycle_delta, (*tc).mask, &mut (*tc).frac);

        /* update time stamp of timecounter_read_delta() call: */
        (*tc).cycle_last = cycle_now;

        ns_offset
    }
}

/// Returns the time stamp advanced by the nanoseconds since the last call.
///
/// # Safety
/// `tc` is an initialized time counter.
#[no_mangle]
pub unsafe extern "C" fn timecounter_read(tc: *mut Timecounter) -> u64 {
    // SAFETY: forwarded caller contract.
    unsafe {
        /* increment time by nanoseconds since last call */
        let mut nsec = timecounter_read_delta(tc);
        nsec = nsec.wrapping_add((*tc).nsec);
        (*tc).nsec = nsec;

        nsec
    }
}
// EXPORT_SYMBOL_GPL(timecounter_read);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
