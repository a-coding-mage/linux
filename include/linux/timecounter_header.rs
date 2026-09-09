/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * linux/include/linux/timecounter.h
 *
 * based on code that migrated away from
 * linux/include/linux/clocksource.h
 */

// Dependency supplied by the surrounding translation unit: linux/types.h

/* simplify initialization of mask field */
#[macro_export]
macro_rules! CYCLECOUNTER_MASK {
    ($bits:expr) => {
        if ($bits) < 64 {
            ((1u64 << ($bits)) - 1)
        } else {
            u64::MAX
        }
    };
}

/**
 * struct cyclecounter - hardware abstraction for a free running counter
 *	Provides completely state-free accessors to the underlying hardware.
 *	Depending on which hardware it reads, the cycle counter may wrap
 *	around quickly. Locking rules (if necessary) have to be defined
 *	by the implementor and user of specific instances of this API.
 */
#[repr(C)]
pub struct cyclecounter {
    pub read: Option<unsafe extern "C" fn(cc: *mut cyclecounter) -> u64>,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
}

/**
 * struct timecounter - layer above a &struct cyclecounter which counts nanoseconds
 */
#[repr(C)]
pub struct timecounter {
    pub cc: *mut cyclecounter,
    pub cycle_last: u64,
    pub nsec: u64,
    pub mask: u64,
    pub frac: u64,
}

/** cycle counter cycles converted to nanoseconds */
#[inline]
pub unsafe fn cyclecounter_cyc2ns(
    cc: *const cyclecounter,
    cycles: u64,
    mask: u64,
    frac: *mut u64,
) -> u64 {
    let mut ns = cycles;

    ns = ns
        .wrapping_mul((*cc).mult as u64)
        .wrapping_add(*frac);
    *frac = ns & mask;
    ns >> (*cc).shift
}

/** Shifts the time of the clock. */
#[inline]
pub unsafe fn timecounter_adjtime(tc: *mut timecounter, delta: i64) {
    (*tc).nsec = (*tc).nsec.wrapping_add(delta as u64);
}

/** Initialize a time counter. */
pub unsafe extern "C" fn timecounter_init(
    tc: *mut timecounter,
    cc: *mut cyclecounter,
    start_tstamp: u64,
);

/** Return nanoseconds elapsed since timecounter_init() plus the initial time stamp. */
pub unsafe extern "C" fn timecounter_read(tc: *mut timecounter) -> u64;

/* This is like cyclecounter_cyc2ns(), but computes a time previous to the
 * time stored in the cycle counter. */
#[inline]
pub unsafe fn cc_cyc2ns_backwards(
    cc: *const cyclecounter,
    cycles: u64,
    frac: u64,
) -> u64 {
    cycles
        .wrapping_mul((*cc).mult as u64)
        .wrapping_sub(frac)
        >> (*cc).shift
}

/** Convert a cycle counter to the same time base as timecounter_read(). */
#[inline]
pub unsafe fn timecounter_cyc2time(
    tc: *const timecounter,
    cycle_tstamp: u64,
) -> u64 {
    let cc = (*tc).cc as *const cyclecounter;
    let mut delta = cycle_tstamp.wrapping_sub((*tc).cycle_last) & (*cc).mask;
    let mut nsec = (*tc).nsec;
    let mut frac = (*tc).frac;

    /*
     * Instead of always treating cycle_tstamp as more recent than
     * tc->cycle_last, detect when it is too far in the future and
     * treat it as old time stamp instead.
     */
    if unlikely(delta > (*cc).mask / 2) {
        delta = (*tc).cycle_last.wrapping_sub(cycle_tstamp) & (*cc).mask;
        nsec = nsec.wrapping_sub(cc_cyc2ns_backwards(cc, delta, frac));
    } else {
        nsec = nsec.wrapping_add(cyclecounter_cyc2ns(
            cc,
            delta,
            (*tc).mask,
            &mut frac,
        ));
    }

    nsec
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
