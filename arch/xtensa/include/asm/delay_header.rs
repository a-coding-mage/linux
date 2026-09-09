/*
 * include/asm-xtensa/delay.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 *
 */

// C dependencies: <asm/timex.h> and <asm/param.h>.

extern "C" {
    pub static mut loops_per_jiffy: ::core::ffi::c_ulong;
    pub static mut ccount_freq: ::core::ffi::c_ulong;
    pub fn get_ccount() -> ::core::ffi::c_ulong;
    pub fn cpu_relax();
    pub fn __bad_udelay();
    pub fn __bad_ndelay();
}

pub unsafe fn __delay(mut loops: ::core::ffi::c_ulong) {
    // __builtin_constant_p(loops) is a C compile-time predicate.  Its
    // constant-only branch is retained here by the same loop threshold.
    if loops < 2 {
        ::core::arch::asm!("nop", options(nomem, nostack, preserves_flags));
    } else if loops >= 2 {
        // 2 cycles per loop.
        ::core::arch::asm!(
            "1: addi {0}, {0}, -2; bgeui {0}, 2, 1b",
            inout(reg) loops,
            options(nostack)
        );
    }
}

pub const __MAX_UDELAY: ::core::ffi::c_ulong = 30000;
pub const __MAX_NDELAY: ::core::ffi::c_ulong = 30000;

pub unsafe fn __udelay(usecs: ::core::ffi::c_ulong) {
    let start = get_ccount();
    let cycles = (usecs * (ccount_freq >> 15)) >> 5;

    /* Note: all variables are unsigned (can wrap around)! */
    while (get_ccount() as ::core::ffi::c_ulong).wrapping_sub(start) < cycles {
        cpu_relax();
    }
}

pub unsafe fn udelay(usec: ::core::ffi::c_ulong) {
    // __builtin_constant_p(usec) cannot be represented directly in Rust;
    // retain the source limit check for the equivalent externally visible call.
    if usec >= __MAX_UDELAY {
        __bad_udelay();
    } else {
        __udelay(usec);
    }
}

pub unsafe fn __ndelay(nsec: ::core::ffi::c_ulong) {
    /*
     * Inner shift makes sure multiplication doesn't overflow
     * for legitimate nsec values
     */
    let cycles = (nsec * (ccount_freq >> 15)) >> 15;
    __delay(cycles);
}

pub unsafe fn ndelay(nsec: ::core::ffi::c_ulong) {
    // __builtin_constant_p(nsec) cannot be represented directly in Rust;
    // retain the source limit check for the equivalent externally visible call.
    if nsec >= __MAX_NDELAY {
        __bad_ndelay();
    } else {
        __ndelay(nsec);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
