/* SPDX-License-Identifier: GPL-2.0 */

/*
 * HP-PA only implements integer multiply in the FPU.  However, for
 * integer multiplies by constant, it has a number of shift-and-add
 * (but no shift-and-subtract, sigh!) instructions that a compiler
 * can synthesize a code sequence with.
 *
 * Unfortunately, GCC isn't very efficient at using them.  For example
 * it uses three instructions for "x *= 21" when only two are needed.
 * But we can find a sequence manually.
 */

pub const HAVE_ARCH__HASH_32: i32 = 1;

/*
 * This is a multiply by GOLDEN_RATIO_32 = 0x61C88647 optimized for the
 * PA7100 pairing rules.  This is an in-order 2-way superscalar processor.
 * Only one instruction in a pair may be a shift (by more than 3 bits),
 * but other than that, simple ALU ops (including shift-and-add by up
 * to 3 bits) may be paired arbitrarily.
 *
 * PA8xxx processors also dual-issue ALU instructions, although with
 * fewer constraints, so this schedule is good for them, too.
 *
 * This 6-step sequence was found by Yevgen Voronenko's implementation
 * of the Hcub algorithm at http://spiral.ece.cmu.edu/mcm/gen.html.
 */
#[inline]
pub const fn __hash_32(mut x: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    /*
     * Phase 1: Compute  a = (x << 19) + x,
     * b = (x << 9) + a, c = (x << 23) + b.
     */
    a = x << 19; // Two shifts can't be paired
    b = x << 9;
    a = a.wrapping_add(x);
    c = x << 23;
    b = b.wrapping_add(a);
    c = c.wrapping_add(b);
    /* Phase 2: Return (b<<11) + (c<<6) + (a<<3) - c */
    b <<= 11;
    a = a.wrapping_add(c << 3);
    b = b.wrapping_sub(c);
    (a << 3).wrapping_add(b)
}

/* The following declarations are conditional on BITS_PER_LONG == 64. */
#[cfg(target_pointer_width = "64")]
pub const HAVE_ARCH_HASH_64: i32 = 1;

/*
 * The C _ASSIGN macro uses inline assembly to inhibit compiler inference.
 * Rust has no direct file-local equivalent here; assignments preserve the
 * source-level operation and ordering.
 */

/*
 * Multiply by GOLDEN_RATIO_64 = 0x0x61C8864680B583EB using a heavily
 * optimized shift-and-add sequence.
 */
#[cfg(target_pointer_width = "64")]
#[inline]
pub fn hash_64(mut a: u64, mut bits: u32) -> u32 {
    let mut b: u64;
    let mut c: u64;
    let mut d: u64;

    /* The C code uses a constant-predicate compiler builtin for scheduling. */
    bits = 64u32.wrapping_sub(bits);

    b = a.wrapping_mul(5);
    c = a << 13;
    b = (b << 2).wrapping_add(a);
    d = a << 17;
    a = b.wrapping_add(a << 1);
    c = c.wrapping_add(d);
    d = a << 10;
    a = a << 19;
    d = a.wrapping_sub(d);
    a = a << 4;
    c = c.wrapping_add(b);
    a = a.wrapping_add(b);
    d = d.wrapping_sub(c);
    c = c.wrapping_add(a << 1);
    a = a.wrapping_add(c << 3);
    b = b << (7 + 31);
    a <<= 31;
    b = b.wrapping_add(d);
    a = a.wrapping_add(b);
    (a >> bits) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
