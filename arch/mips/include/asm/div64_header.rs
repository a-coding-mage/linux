/*
 * Copyright (C) 2000, 2004, 2021  Maciej W. Rozycki
 * Copyright (C) 2003, 07 Ralf Baechle (ralf@linux-mips.org)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// The following declarations correspond to the BITS_PER_LONG == 32 branch.

/// Divide a 64-bit value, supplied as high and low 32-bit words, by a
/// 32-bit base.  The quotient is written to `res` and the remainder returned.
#[inline]
pub unsafe fn do_div64_32(res: *mut u32, high: u32, low: u32, base: u32) -> u32 {
    let mut mod32 = high;
    let mut tmp = low;
    let mut quot32: u32 = 0;

    // This is the restoring-division sequence implemented by the original
    // MIPS inline assembly.  All operations intentionally wrap at 32 bits.
    for _ in 0..32 {
        let carry = tmp >> 31;
        mod32 = mod32.wrapping_shl(1) | carry;
        tmp = tmp.wrapping_shl(1);
        quot32 = quot32.wrapping_shl(1);
        if mod32 >= base {
            mod32 = mod32.wrapping_sub(base);
            quot32 = quot32.wrapping_add(1);
        }
    }

    *res = quot32;
    mod32
}

/// Divide the 64-bit integer pointed to by `n` by a 32-bit base.
#[inline]
pub unsafe fn __div64_32(n: *mut u64, base: u32) -> u32 {
    let div = *n;
    let radix = base as u64;

    let mut high = (div >> 32) as u32;
    let mut low = div as u32;
    let upper: u32;

    if high < base {
        upper = high;
        high = 0;
    } else {
        upper = (high as u64 % radix) as u32;
        high = (high as u64 / radix) as u32;
    }

    let modulus = do_div64_32(&mut low as *mut u32, upper, low, base);

    let quot = ((high as u64) << 32) | low as u64;
    *n = quot;
    modulus
}

// The original header also includes <asm-generic/div64.h>; its declarations
// and definitions are supplied by the corresponding external dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
