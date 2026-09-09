/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C header note: define T (32 or 64) before including this header.  The
// following macro is the Rust equivalent of that per-width instantiation;
// callers supply the corresponding cnum type, integer types, constants, and
// generated function names from the cnum dependency.
macro_rules! cnum_defs {
    (
        $cnum_t:ty, $ut:ty, $st:ty,
        $ut_max:expr, $st_max:expr, $st_min:expr, $empty:expr,
        $from_urange:ident, $from_srange:ident, $urange_overflow:ident,
        $umin:ident, $umax:ident, $srange_overflow:ident, $smin:ident,
        $smax:ident, $intersect:ident, $intersect_with:ident,
        $intersect_with_urange:ident, $intersect_with_srange:ident,
        $normalize:ident, $add:ident, $negate:ident, $is_empty:ident,
        $contains:ident, $is_const:ident, $is_subset:ident
    ) => {
        #[inline]
        fn $from_urange(min: $ut, max: $ut) -> $cnum_t {
            $cnum_t { base: min, size: max.wrapping_sub(min) }
        }

        #[inline]
        fn $from_srange(min: $st, max: $st) -> $cnum_t {
            let size = (max as $ut).wrapping_sub(min as $ut);
            let base = if size == $ut_max { 0 as $ut } else { min as $ut };
            $cnum_t { base, size }
        }

        // True if this cnum represents two unsigned ranges.
        #[inline]
        fn $urange_overflow(cnum: $cnum_t) -> bool {
            // Same as cnum.base + cnum.size > UT_MAX but avoids overflow.
            cnum.size > $ut_max - cnum.base as $ut
        }

        fn $umin(cnum: $cnum_t) -> $ut {
            if $urange_overflow(cnum) { 0 as $ut } else { cnum.base }
        }

        fn $umax(cnum: $cnum_t) -> $ut {
            if $urange_overflow(cnum) { $ut_max } else { cnum.base.wrapping_add(cnum.size) }
        }

        // True if this cnum represents two signed ranges.
        #[inline]
        fn $srange_overflow(cnum: $cnum_t) -> bool {
            $contains(cnum, $st_max as $ut) && $contains(cnum, $st_min as $ut)
        }

        fn $smin(cnum: $cnum_t) -> $st {
            if $srange_overflow(cnum) { $st_min }
            else { (cnum.base as $st).min(cnum.base.wrapping_add(cnum.size) as $st) }
        }

        fn $smax(cnum: $cnum_t) -> $st {
            if $srange_overflow(cnum) { $st_max }
            else { (cnum.base as $st).max(cnum.base.wrapping_add(cnum.size) as $st) }
        }

        fn $intersect(mut a: $cnum_t, mut b: $cnum_t) -> $cnum_t {
            if $is_empty(a) || $is_empty(b) { return $empty; }
            if a.base > b.base { core::mem::swap(&mut a, &mut b); }
            let dbase = b.base.wrapping_sub(a.base);
            let b1 = $cnum_t { base: dbase, size: b.size };
            if $urange_overflow(b1) {
                if b1.base <= a.size {
                    if a.size <= b.size { a } else { b }
                } else {
                    $cnum_t { base: a.base, size: a.size.min(b1.base.wrapping_add(b1.size)) }
                }
            } else if a.size >= b1.base {
                $cnum_t { base: b.base, size: a.size.wrapping_sub(dbase).min(b.size) }
            } else { $empty }
        }

        fn $intersect_with(dst: &mut $cnum_t, src: $cnum_t) { *dst = $intersect(*dst, src); }
        fn $intersect_with_urange(dst: &mut $cnum_t, min: $ut, max: $ut) {
            $intersect_with(dst, $from_urange(min, max));
        }
        fn $intersect_with_srange(dst: &mut $cnum_t, min: $st, max: $st) {
            $intersect_with(dst, $from_srange(min, max));
        }

        #[inline]
        fn $normalize(mut cnum: $cnum_t) -> $cnum_t {
            if cnum.size == $ut_max && cnum.base != 0 as $ut && cnum.base != $st_max as $ut { cnum.base = 0 as $ut; }
            cnum
        }

        fn $add(a: $cnum_t, b: $cnum_t) -> $cnum_t {
            if $is_empty(a) || $is_empty(b) { return $empty; }
            if a.size > $ut_max - b.size { $cnum_t { base: 0 as $ut, size: $ut_max } }
            else { $normalize($cnum_t { base: a.base.wrapping_add(b.base), size: a.size.wrapping_add(b.size) }) }
        }

        fn $negate(a: $cnum_t) -> $cnum_t {
            if $is_empty(a) { return $empty; }
            $normalize($cnum_t { base: (0 as $ut).wrapping_sub(a.base.wrapping_add(a.size)), size: a.size })
        }

        fn $is_empty(cnum: $cnum_t) -> bool { cnum.base == $empty.base && cnum.size == $empty.size }
        fn $contains(cnum: $cnum_t, v: $ut) -> bool {
            if $is_empty(cnum) { false }
            else if $urange_overflow(cnum) { v >= cnum.base || v <= cnum.base.wrapping_add(cnum.size) }
            else { v >= cnum.base && v <= cnum.base.wrapping_add(cnum.size) }
        }
        fn $is_const(cnum: $cnum_t) -> bool { cnum.size == 0 as $ut }
        fn $is_subset(mut bigger: $cnum_t, mut smaller: $cnum_t) -> bool {
            if $is_empty(smaller) { return true; }
            if $is_empty(bigger) { return false; }
            smaller.base = smaller.base.wrapping_sub(bigger.base);
            bigger.base = 0 as $ut;
            if $urange_overflow(smaller) && bigger.size < $ut_max { return false; }
            smaller.base.wrapping_add(smaller.size) <= bigger.size
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
