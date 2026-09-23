// SPDX-License-Identifier: GPL-2.0-only
//! Allocation-free greatest common divisors using either kernel algorithm.

/// Returns the greatest common divisor, including `gcd(0, 0) == 0`.
///
/// The configured CPU capability selects the algorithm. Unlike the native C
/// entry point, this pure helper does not consult a mutable kernel static key.
#[inline]
pub const fn gcd(a: usize, b: usize) -> usize {
    gcd_with_ffs(a, b, !cfg!(CONFIG_CPU_NO_EFFICIENT_FFS))
}

/// Returns the greatest common divisor using the selected bit-scan strategy.
///
/// A native ABI owner may pass its runtime CPU/static-key decision here. Both
/// strategies are also available to constant evaluation and pure consumers.
#[inline]
pub const fn gcd_with_ffs(mut a: usize, mut b: usize, efficient_ffs: bool) -> usize {
    let mut r = a | b;
    if a == 0 || b == 0 {
        return r;
    }
    if efficient_ffs {
        // The rotated-out low bits are all zero, so these rotations are exactly
        // right shifts without a checked-shift panic dependency at opt-level 0.
        b = b.rotate_right(b.trailing_zeros());
        if b == 1 {
            return r & r.wrapping_neg();
        }
        loop {
            a = a.rotate_right(a.trailing_zeros());
            if a == 1 {
                return r & r.wrapping_neg();
            }
            if a == b {
                // The odd gcd times the common power of two fits in either
                // original operand. Thus no high bits wrap around this rotate.
                return a.rotate_left(r.trailing_zeros());
            }
            if a < b {
                let old_a = a;
                a = b;
                b = old_a;
            }
            a = a.wrapping_sub(b);
        }
    }

    r &= r.wrapping_neg();
    while b & r == 0 {
        b >>= 1;
    }
    if b == r {
        return r;
    }
    loop {
        while a & r == 0 {
            a >>= 1;
        }
        if a == r {
            return r;
        }
        if a == b {
            return a;
        }
        if a < b {
            let old_a = a;
            a = b;
            b = old_a;
        }
        a = a.wrapping_sub(b);
        a >>= 1;
        if a & r != 0 {
            a = a.wrapping_add(b);
        }
        a >>= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
