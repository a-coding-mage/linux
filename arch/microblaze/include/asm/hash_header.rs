/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Fortunately, most people who want to run Linux on Microblaze enable
 * both multiplier and barrel shifter, but omitting them is technically a
 * supported configuration.
 *
 * With just a barrel shifter, a constant multiply can be implemented using
 * shifts and adds.  Without even a shifter, any hash function will suck.
 *
 * The original implementation is selected when
 * CONFIG_XILINX_MICROBLAZE0_USE_HW_MUL == 0.
 */

#[cfg(not(CONFIG_XILINX_MICROBLAZE0_USE_HW_MUL))]
pub const HAVE_ARCH__HASH_32: u32 = 1;

/* Multiply by GOLDEN_RATIO_32 = 0x61C88647. */
#[cfg(all(
    not(CONFIG_XILINX_MICROBLAZE0_USE_HW_MUL),
    CONFIG_XILINX_MICROBLAZE0_USE_BARREL
))]
#[inline]
pub const fn __hash_32(mut a: u32) -> u32 {
    let mut b: u32;
    let mut c: u32;

    /* Phase 1: Compute three intermediate values. */
    b = a.wrapping_shl(23);
    c = a.wrapping_shl(19).wrapping_add(a);
    a = a.wrapping_shl(9).wrapping_add(c);
    b = b.wrapping_add(a);

    /* Phase 2: Compute (a << 11) + (b << 6) + (c << 3) - b. */
    a = a.wrapping_shl(5);
    a = a.wrapping_add(b); /* (a << 5) + b */
    a = a.wrapping_shl(3);
    a = a.wrapping_add(c); /* (a << 8) + (b << 3) + c */
    a = a.wrapping_shl(3);
    a.wrapping_sub(b) /* (a << 11) + (b << 6) + (c << 3) - b */
}

/*
 * Without a barrel shifter, left shifts are implemented as repeated
 * additions.  This is an addition-subtraction chain for the multiplier.
 */
#[cfg(all(
    not(CONFIG_XILINX_MICROBLAZE0_USE_HW_MUL),
    not(CONFIG_XILINX_MICROBLAZE0_USE_BARREL)
))]
#[inline]
pub const fn __hash_32(mut a: u32) -> u32 {
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;

    b = a.wrapping_shl(4); /* 4 */
    c = b.wrapping_shl(1); /* 1  5 */
    b = b.wrapping_add(a); /* 1  6 */
    c = c.wrapping_add(b); /* 1  7 */
    c = c.wrapping_shl(3); /* 3 10 */
    c = c.wrapping_sub(a); /* 1 11 */
    d = c.wrapping_shl(7); /* 7 18 */
    d = d.wrapping_add(b); /* 1 19 */
    d = d.wrapping_shl(8); /* 8 27 */
    d = d.wrapping_add(a); /* 1 28 */
    d = d.wrapping_shl(1); /* 1 29 */
    d = d.wrapping_add(b); /* 1 30 */
    d = d.wrapping_shl(6); /* 6 36 */
    d.wrapping_add(c) /* 1 37 total instructions */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
