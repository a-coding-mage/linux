// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Davidlohr Bueso <davidlohr.bueso@hp.com>
 *
 *  Based on the shift-and-subtract algorithm for computing integer
 *  square root from Guy L. Steele.
 */

extern "C" {
    fn __fls(x: usize) -> i32;
    fn fls64(x: u64) -> i32;
}

/**
 * int_sqrt - computes the integer square root
 * @x: integer of which to calculate the sqrt
 *
 * Computes: floor(sqrt(x))
 */
pub fn int_sqrt(mut x: usize) -> usize {
    let (mut b, mut m);
    let mut y: usize = 0;

    if x <= 1 {
        return x;
    }

    m = 1usize << ((unsafe { __fls(x) } as usize) & !1usize);
    while m != 0 {
        b = y.wrapping_add(m);
        y >>= 1;

        if x >= b {
            x = x.wrapping_sub(b);
            y = y.wrapping_add(m);
        }
        m >>= 2;
    }

    y
}
// EXPORT_SYMBOL(int_sqrt);

#[cfg(target_pointer_width = "32")]
/**
 * int_sqrt64 - strongly typed int_sqrt function when minimum 64 bit input
 * is expected.
 * @x: 64bit integer of which to calculate the sqrt
 */
pub fn int_sqrt64(mut x: u64) -> u32 {
    let (mut b, mut m);
    let mut y: u64 = 0;

    if x <= usize::MAX as u64 {
        return int_sqrt(x as usize) as u32;
    }

    m = 1u64 << ((unsafe { fls64(x) } as u64 - 1) & !1u64);
    while m != 0 {
        b = y.wrapping_add(m);
        y >>= 1;

        if x >= b {
            x = x.wrapping_sub(b);
            y = y.wrapping_add(m);
        }
        m >>= 2;
    }

    y as u32
}
// EXPORT_SYMBOL(int_sqrt64);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
