// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 */

/* Dependencies supplied by the surrounding kernel translation. */
unsafe extern "C" {
    fn get_cycles() -> u64;
    fn cpu_relax();

    static lpj_fine: usize;
    static riscv_timebase: u64;
}

/* Build-time kernel constant supplied by the surrounding kernel translation. */

/*
 * This is copies from arch/arm/include/asm/delay.h
 *
 * Loop (or tick) based delay:
 *
 * loops = loops_per_jiffy * jiffies_per_sec * delay_us / us_per_sec
 *
 * where:
 *
 * jiffies_per_sec = HZ
 * us_per_sec = 1000000
 *
 * Therefore the constant part is HZ / 1000000 which is a small
 * fractional number. To make this usable with integer math, we
 * scale up this constant by 2^31, perform the actual multiplication,
 * and scale the result back down by 2^31 with a simple shift:
 *
 * loops = (loops_per_jiffy * delay_us * UDELAY_MULT) >> 31
 *
 * where:
 *
 * UDELAY_MULT = 2^31 * HZ / 1000000
 *             = (2^31 / 1000000) * HZ
 *             = 2147.483648 * HZ
 *             = 2147 * HZ + 483648 * HZ / 1000000
 *
 * 31 is the biggest scale shift value that won't overflow 32 bits for
 * delay_us * UDELAY_MULT assuming HZ <= 1000 and delay_us <= 2000.
 */
const MAX_UDELAY_US: usize = 2000;
const MAX_UDELAY_HZ: u64 = 1000;
const UDELAY_MULT: u64 = 2147 * HZ + 483648 * HZ / 1_000_000;
const UDELAY_SHIFT: u32 = 31;

/*
 * RISC-V supports both UDELAY and NDELAY.  This is largely the same as above,
 * but with different constants.  I added 10 bits to the shift to get this, but
 * the result is that I need a 64-bit multiply, which is slow on 32-bit
 * platforms.
 *
 * NDELAY_MULT = 2^41 * HZ / 1000000000
 *             = (2^41 / 1000000000) * HZ
 *             = 2199.02325555 * HZ
 *             = 2199 * HZ + 23255550 * HZ / 1000000000
 *
 * The maximum here is to avoid 64-bit overflow, but it isn't checked as it
 * won't happen.
 */
const MAX_NDELAY_NS: u64 = 1u64 << 42;
const MAX_NDELAY_HZ: u64 = MAX_UDELAY_HZ;
const NDELAY_MULT: u64 = 2199u64 * HZ + 23255550u64 * HZ / 1_000_000_000;
const NDELAY_SHIFT: u32 = 41;

pub unsafe fn __delay(cycles: usize) {
    let t0: u64 = get_cycles();

    while (get_cycles().wrapping_sub(t0) as usize) < cycles {
        cpu_relax();
    }
}

pub unsafe fn udelay(usecs: usize) {
    let ucycles: u64 = (usecs as u64)
        .wrapping_mul(lpj_fine as u64)
        .wrapping_mul(UDELAY_MULT);
    let mut n: u64;

    if usecs > MAX_UDELAY_US {
        n = (usecs as u64).wrapping_mul(riscv_timebase);
        n /= 1_000_000;

        __delay(n as usize);
        return;
    }

    __delay((ucycles >> UDELAY_SHIFT) as usize);
}

pub unsafe fn ndelay(nsecs: usize) {
    /*
     * This doesn't bother checking for overflow, as it won't happen (it's
     * an hour) of delay.
     */
    let ncycles: u64 = (nsecs as u64)
        .wrapping_mul(lpj_fine as u64)
        .wrapping_mul(NDELAY_MULT);
    __delay((ncycles >> NDELAY_SHIFT) as usize);
}

pub unsafe fn delay_read_timer(timer_val: *mut usize) -> bool {
    *timer_val = get_cycles() as usize;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
