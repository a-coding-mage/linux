/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Delay routines using pre computed loops_per_jiffy value.
 *
 * vineetg: Feb 2012
 *  -Rewrote in "C" to avoid dealing with availability of H/w MPY
 *  -Also reduced the num of MPY operations from 3 to 2
 *
 * Amit Bhor: Codito Technologies 2004
 */

// Dependency intent: the original header includes asm-generic/types.h and
// asm/param.h for the integer types and HZ constant.

unsafe extern "C" {
    pub static mut loops_per_jiffy: usize;
    pub fn __bad_udelay();
}

#[inline]
pub unsafe fn __delay(loops: usize) {
    // ARC inline assembly: load the loop counter, execute one loop iteration,
    // and retain the clobber of the processor's lp_count register.
    core::arch::asm!(
        "mov lp_count, {loops}",
        "lp 1f",
        "nop",
        "1:",
        loops = in(reg) loops,
        clobber_abi("C"),
    );
}

/*
 * Normal Math for computing loops in "N" usecs
 *  -we have precomputed @loops_per_jiffy
 *  -1 sec has HZ jiffies
 * loops per "N" usecs = ((loops_per_jiffy * HZ / 1000000) * N)
 *
 * Approximate Division by multiplication:
 *  -Mathematically if we multiply and divide a number by same value the
 *   result remains unchanged:  In this case, we use 2^32
 *  -> (loops_per_N_usec * 2^32 ) / 2^32
 *  -> (((loops_per_jiffy * HZ / 1000000) * N) * 2^32) / 2^32
 *  -> (loops_per_jiffy * HZ * N * 4295) / 2^32
 *
 *  -Divide by 2^32 is very simply right shift by 32
 *  -We simply need to ensure that the multiply per above eqn happens in
 *   64-bit precision (if CPU doesn't support it - gcc can emaulate it)
 */

#[inline]
pub unsafe fn __udelay(usecs: usize) {
    let loops: usize;

    /* (u64) cast ensures 64 bit MPY - real or emulated
     * HZ * 4295 is pre-evaluated by gcc - hence only 2 mpy ops
     */
    loops = ((usecs as u64 * 4295u64 * HZ as u64 * loops_per_jiffy as u64) >> 32) as usize;

    __delay(loops);
}

// HZ is supplied by the original asm/param.h dependency.
#[macro_export]
macro_rules! udelay {
    ($n:expr) => {{
        if $n > 20000 {
            unsafe { $crate::__bad_udelay() }
        } else {
            unsafe { $crate::__udelay($n as usize) }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
