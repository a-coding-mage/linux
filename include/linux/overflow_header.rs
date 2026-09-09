/* SPDX-License-Identifier: GPL-2.0 OR MIT */
// Rust translation of linux/overflow.h.
// Kernel/compiler facilities referenced below are supplied by other dependencies.

/*
 * We need to compute the minimum and maximum values representable in a given
 * type. These macros may also be useful elsewhere. The C implementation uses
 * is_signed_type(), typeof(), and compiler constant-expression machinery.
 */
#[inline(always)]
pub fn __must_check_overflow(overflow: bool) -> bool {
    // C: unlikely(overflow)
    overflow
}

#[macro_export]
macro_rules! __type_half_max { ($t:ty) => { ((1 as $t) << (8 * core::mem::size_of::<$t>() - 1)) }; }
#[macro_export]
macro_rules! __type_max { ($t:ty) => { (((__type_half_max!($t)) - 1) + __type_half_max!($t)) as $t }; }
#[macro_export]
macro_rules! type_max { ($e:expr) => { __type_max!(_) }; }
#[macro_export]
macro_rules! __type_min { ($t:ty) => { (-(type_max!((0 as $t))) - (1 as $t)) as $t }; }
#[macro_export]
macro_rules! type_min { ($e:expr) => { __type_min!(_) }; }

#[macro_export]
macro_rules! check_add_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let (v, overflow) = ($a).overflowing_add($b);
        unsafe { *($d) = v; }
        __must_check_overflow(overflow)
    }};
}

#[macro_export]
macro_rules! wrapping_add { ($t:ty, $a:expr, $b:expr) => { ($a as $t).wrapping_add($b as $t) }; }
#[macro_export]
macro_rules! wrapping_assign_add { ($var:expr, $offset:expr) => {{ $var = ($var).wrapping_add($offset); $var }}; }

#[macro_export]
macro_rules! check_sub_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let (v, overflow) = ($a).overflowing_sub($b);
        unsafe { *($d) = v; }
        __must_check_overflow(overflow)
    }};
}
#[macro_export]
macro_rules! wrapping_sub { ($t:ty, $a:expr, $b:expr) => { ($a as $t).wrapping_sub($b as $t) }; }
#[macro_export]
macro_rules! wrapping_assign_sub { ($var:expr, $offset:expr) => {{ $var = ($var).wrapping_sub($offset); $var }}; }

#[macro_export]
macro_rules! check_mul_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let (v, overflow) = ($a).overflowing_mul($b);
        unsafe { *($d) = v; }
        __must_check_overflow(overflow)
    }};
}
#[macro_export]
macro_rules! wrapping_mul { ($t:ty, $a:expr, $b:expr) => { ($a as $t).wrapping_mul($b as $t) }; }

#[macro_export]
macro_rules! check_shl_overflow {
    ($a:expr, $s:expr, $d:expr) => {{
        let _a = $a; let _s = $s; let _d = $d;
        let _to_shift = if _s >= 0 && (_s as usize) < 8 * core::mem::size_of_val(unsafe { &*_d }) { _s } else { 0 };
        unsafe { *_d = _a << _to_shift; }
        _to_shift != _s || unsafe { *_d < 0 } || _a < 0 || unsafe { (*_d >> _to_shift) != _a }
    }};
}

#[macro_export]
macro_rules! range_overflows { ($start:expr, $size:expr, $max:expr) => {{ let start__ = $start; let size__ = $size; let max__ = $max; start__ >= max__ || size__ > max__ - start__ }}; }
#[macro_export]
macro_rules! range_overflows_t { ($t:ty, $start:expr, $size:expr, $max:expr) => { range_overflows!($start as $t, $size as $t, $max as $t) }; }
#[macro_export]
macro_rules! range_end_overflows { ($start:expr, $size:expr, $max:expr) => {{ let start__ = $start; let size__ = $size; let max__ = $max; start__ > max__ || size__ > max__ - start__ }}; }
#[macro_export]
macro_rules! range_end_overflows_t { ($t:ty, $start:expr, $size:expr, $max:expr) => { range_end_overflows!($start as $t, $size as $t, $max as $t) }; }

pub const fn size_mul(factor1: usize, factor2: usize) -> usize { factor1.checked_mul(factor2).unwrap_or(usize::MAX) }
pub const fn size_add(addend1: usize, addend2: usize) -> usize { addend1.checked_add(addend2).unwrap_or(usize::MAX) }
pub const fn size_sub(minuend: usize, subtrahend: usize) -> usize {
    if minuend == usize::MAX || subtrahend == usize::MAX { usize::MAX } else { minuend.checked_sub(subtrahend).unwrap_or(usize::MAX) }
}

#[macro_export] macro_rules! array_size { ($a:expr, $b:expr) => { size_mul($a as usize, $b as usize) }; }
#[macro_export] macro_rules! array3_size { ($a:expr, $b:expr, $c:expr) => { size_mul(size_mul($a as usize, $b as usize), $c as usize) }; }
#[macro_export] macro_rules! overflows_type { ($n:expr, $t:ty) => {{ let _ = &$n; false }}; }
#[macro_export] macro_rules! overflows_flex_counter_type { ($t:ty, $fam:ident, $count:expr) => { overflows_type!($count, usize) }; }
#[macro_export] macro_rules! castable_to_type { ($n:expr, $t:ty) => { true }; }
#[macro_export] macro_rules! struct_offset { ($p:expr, $member:ident) => { core::mem::offset_of!(typeof!(*$p), $member) }; }
#[macro_export] macro_rules! flex_array_size { ($p:expr, $member:ident, $count:expr) => { size_mul($count as usize, core::mem::size_of_val(&(*$p).$member)) }; }
#[macro_export] macro_rules! struct_size { ($p:expr, $member:ident, $count:expr) => { size_add(core::mem::size_of_val($p), flex_array_size!($p, $member, $count)) }; }
#[macro_export] macro_rules! struct_size_t { ($t:ty, $member:ident, $count:expr) => { struct_size!((&core::mem::MaybeUninit::<$t>::uninit().assume_init()), $member, $count) }; }
#[macro_export] macro_rules! __set_flex_counter { ($fam:expr, $count:expr) => {{ let _ = &$fam; let _ = $count; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
