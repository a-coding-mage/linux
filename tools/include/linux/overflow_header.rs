/* SPDX-License-Identifier: GPL-2.0 OR MIT */

/* Translated from include/linux/overflow.h. */
/* Original C dependencies: <stdint.h>, <linux/compiler.h>. */

pub const SIZE_MAX: usize = usize::MAX;

/*
 * We need to compute the minimum and maximum values representable in a given
 * type. These macros may also be useful elsewhere. It would seem more obvious
 * to do something like:
 *
 * #define type_min(T) (T)(is_signed_type(T) ? (T)1 << (8*sizeof(T)-1) : 0)
 * #define type_max(T) (T)(is_signed_type(T) ? ((T)1 << (8*sizeof(T)-1)) - 1 : ~(T)0)
 *
 * Unfortunately, the middle expressions, strictly speaking, have
 * undefined behaviour, and at least some versions of gcc warn about
 * the type_max expression (but not if -fsanitize=undefined is in
 * effect; in that case, the warning is deferred to runtime...).
 *
 * The slightly excessive casting in type_min is to make sure the
 * macros also produce sensible values for the exotic type _Bool. [The
 * overflow checkers only almost work for _Bool, but that's
 * a-feature-not-a-bug, since people shouldn't be doing arithmetic on
 * _Bools. Besides, the gcc builtins don't allow _Bool* as third
 * argument.]
 *
 * Idea stolen from
 * https://mail-index.netbsd.org/tech-misc/2007/02/05/0000.html -
 * credit to Christian Biere.
 */
#[macro_export]
macro_rules! is_signed_type {
    ($type:ty) => {
        ((-1i8 as $type) < (1i8 as $type))
    };
}

#[macro_export]
macro_rules! __type_half_max {
    ($type:ty) => {
        ((1i8 as $type) << (8usize * ::core::mem::size_of::<$type>() - 1usize - ($crate::is_signed_type!($type) as usize)))
    };
}

#[macro_export]
macro_rules! type_max {
    ($type:ty) => {
        (((($crate::__type_half_max!($type) - (1i8 as $type)) + $crate::__type_half_max!($type))) as $type)
    };
}

#[macro_export]
macro_rules! type_min {
    ($type:ty) => {
        (((0i8 as $type) - $crate::type_max!($type) - (1i8 as $type)) as $type)
    };
}

/*
 * For simplicity and code hygiene, the fallback code below insists on
 * a, b and *d having the same type (similar to the min() and max()
 * macros), whereas gcc's type-generic overflow checkers accept
 * different types. Hence we don't just make check_add_overflow an
 * alias for __builtin_add_overflow, but add type checks similar to
 * below.
 */
#[macro_export]
macro_rules! check_add_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let __a = $a;
        let __b = $b;
        let __d = $d;
        let (__result, __overflow) = __a.overflowing_add(__b);
        unsafe {
            *__d = __result;
        }
        __overflow
    }};
}

#[macro_export]
macro_rules! check_sub_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let __a = $a;
        let __b = $b;
        let __d = $d;
        let (__result, __overflow) = __a.overflowing_sub(__b);
        unsafe {
            *__d = __result;
        }
        __overflow
    }};
}

#[macro_export]
macro_rules! check_mul_overflow {
    ($a:expr, $b:expr, $d:expr) => {{
        let __a = $a;
        let __b = $b;
        let __d = $d;
        let (__result, __overflow) = __a.overflowing_mul(__b);
        unsafe {
            *__d = __result;
        }
        __overflow
    }};
}

/**
 * size_mul() - Calculate size_t multiplication with saturation at SIZE_MAX
 * @factor1: first factor
 * @factor2: second factor
 *
 * Returns: calculate @factor1 * @factor2, both promoted to size_t,
 * with any overflow causing the return value to be SIZE_MAX. The
 * lvalue must be size_t to avoid implicit type conversion.
 */
#[must_use]
pub fn size_mul(factor1: usize, factor2: usize) -> usize {
    let mut bytes: usize = 0;

    if check_mul_overflow!(factor1, factor2, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }

    bytes
}

/**
 * array_size() - Calculate size of 2-dimensional array.
 *
 * @a: dimension one
 * @b: dimension two
 *
 * Calculates size of 2-dimensional array: @a * @b.
 *
 * Returns: number of bytes needed to represent the array or SIZE_MAX on
 * overflow.
 */
#[must_use]
pub fn array_size(a: usize, b: usize) -> usize {
    let mut bytes: usize = 0;

    if check_mul_overflow!(a, b, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }

    bytes
}

/**
 * array3_size() - Calculate size of 3-dimensional array.
 *
 * @a: dimension one
 * @b: dimension two
 * @c: dimension three
 *
 * Calculates size of 3-dimensional array: @a * @b * @c.
 *
 * Returns: number of bytes needed to represent the array or SIZE_MAX on
 * overflow.
 */
#[must_use]
pub fn array3_size(a: usize, b: usize, c: usize) -> usize {
    let mut bytes: usize = 0;

    if check_mul_overflow!(a, b, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }
    if check_mul_overflow!(bytes, c, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }

    bytes
}

#[must_use]
pub fn __ab_c_size(n: usize, size: usize, c: usize) -> usize {
    let mut bytes: usize = 0;

    if check_mul_overflow!(n, size, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }
    if check_add_overflow!(bytes, c, &mut bytes as *mut usize) {
        return SIZE_MAX;
    }

    bytes
}

/**
 * struct_size() - Calculate size of structure with trailing array.
 * @p: Pointer to the structure.
 * @member: Name of the array member.
 * @n: Number of elements in the array.
 *
 * Calculates size of memory needed for structure @p followed by an
 * array of @n @member elements.
 *
 * Return: number of bytes needed or SIZE_MAX on overflow.
 */
#[macro_export]
macro_rules! struct_size {
    ($p:expr, $member:ident, $n:expr) => {
        $crate::__ab_c_size(
            $n,
            ::core::mem::size_of_val(unsafe { &(*(*$p).$member.as_ptr()) })
                + __must_be_array!((*$p).$member),
            ::core::mem::size_of_val(unsafe { &*$p }),
        )
    };
}
