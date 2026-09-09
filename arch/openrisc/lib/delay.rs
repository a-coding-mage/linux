// SPDX-License-Identifier: GPL-2.0-only
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * Precise Delay Loops
 */

// Dependencies supplied by the surrounding kernel translation.
extern "Rust" {
    static loops_per_jiffy: usize;
    static HZ: usize;
    fn get_cycles() -> u64;
    fn cpu_relax();
}

pub unsafe fn delay_read_timer(timer_value: *mut usize) -> bool {
    *timer_value = get_cycles() as usize;
    true
}

pub unsafe fn __delay(cycles: usize) {
    let start: u64 = get_cycles();

    while get_cycles().wrapping_sub(start) < cycles as u64 {
        cpu_relax();
    }
}

#[inline]
pub unsafe fn __const_udelay(xloops: usize) {
    let loops: u64;

    loops = (xloops as u64)
        .wrapping_mul(loops_per_jiffy as u64)
        .wrapping_mul(HZ as u64);

    __delay((loops >> 32) as usize);
}

pub unsafe fn __udelay(usecs: usize) {
    __const_udelay(usecs.wrapping_mul(0x10C7usize)); /* 2**32 / 1000000 (rounded up) */
}

pub unsafe fn __ndelay(nsecs: usize) {
    __const_udelay(nsecs.wrapping_mul(0x5usize)); /* 2**32 / 1000000000 (rounded up) */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
