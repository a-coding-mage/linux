// SPDX-License-Identifier: GPL-2.0

// Rust translation of include/linux/build_bug.h.
// C header guard and include of <linux/compiler.h> are intentionally omitted.
// The compiletime_assert! dependency is expected to be supplied by translated
// compiler support code.

/*
 * Force a compilation error if condition is true, but also produce a
 * result (of value 0 and type int), so the expression can be used
 * e.g. in a structure initializer (or where-ever else comma expressions
 * aren't permitted).
 *
 * Take an error message as an optional second argument. If omitted,
 * default to the stringification of the tested expression.
 */
#[macro_export]
macro_rules! BUILD_BUG_ON_ZERO {
    ($e:expr $(,)?) => {{
        $crate::__BUILD_BUG_ON_ZERO_MSG!($e, concat!(stringify!($e), " is true"))
    }};
    ($e:expr, $msg:expr $(,)?) => {{
        $crate::__BUILD_BUG_ON_ZERO_MSG!($e, $msg)
    }};
}

/* Force a compilation error if a constant expression is not a power of 2 */
#[macro_export]
macro_rules! __BUILD_BUG_ON_NOT_POWER_OF_2 {
    ($n:expr $(,)?) => {{
        $crate::BUILD_BUG_ON!((($n) & (($n) - 1)) != 0)
    }};
}

#[macro_export]
macro_rules! BUILD_BUG_ON_NOT_POWER_OF_2 {
    ($n:expr $(,)?) => {{
        $crate::BUILD_BUG_ON!(($n) == 0 || ((($n) & (($n) - 1)) != 0))
    }};
}

/*
 * BUILD_BUG_ON_INVALID() permits the compiler to check the validity of the
 * expression but avoids the generation of any code, even if that expression
 * has side-effects.
 */
#[macro_export]
macro_rules! BUILD_BUG_ON_INVALID {
    ($e:expr $(,)?) => {{
        let _ = ::core::mem::size_of_val(&($e as isize));
    }};
}

/**
 * BUILD_BUG_ON_MSG - break compile if a condition is true & emit supplied
 *		      error message.
 * @cond: the condition which the compiler should know is false.
 * @msg: build-time error message
 *
 * See BUILD_BUG_ON for description.
 */
#[macro_export]
macro_rules! BUILD_BUG_ON_MSG {
    ($cond:expr, $msg:expr $(,)?) => {{
        $crate::compiletime_assert!(!($cond), $msg)
    }};
}

/**
 * BUILD_BUG_ON - break compile if a condition is true.
 * @condition: the condition which the compiler should know is false.
 *
 * If you have some code which relies on certain constants being equal, or
 * some other compile-time-evaluated condition, you should use BUILD_BUG_ON to
 * detect if someone changes it.
 */
#[macro_export]
macro_rules! BUILD_BUG_ON {
    ($condition:expr $(,)?) => {{
        $crate::BUILD_BUG_ON_MSG!(
            $condition,
            concat!("BUILD_BUG_ON failed: ", stringify!($condition))
        )
    }};
}

/**
 * BUILD_BUG - break compile if used.
 *
 * If you have some code that you expect the compiler to eliminate at
 * build time, you should use BUILD_BUG to detect if it is
 * unexpectedly used.
 */
#[macro_export]
macro_rules! BUILD_BUG {
    () => {{
        $crate::BUILD_BUG_ON_MSG!(true, "BUILD_BUG failed")
    }};
}

/**
 * static_assert - check integer constant expression at build time
 * @expr: expression to be checked
 *
 * static_assert() is a wrapper for the C11 _Static_assert, with a
 * little macro magic to make the message optional (defaulting to the
 * stringification of the tested expression).
 *
 * Contrary to BUILD_BUG_ON(), static_assert() can be used at global
 * scope, but requires the expression to be an integer constant
 * expression (i.e., it is not enough that __builtin_constant_p() is
 * true for expr).
 *
 * Also note that BUILD_BUG_ON() fails the build if the condition is
 * true, while static_assert() fails the build if the expression is
 * false.
 */
#[macro_export]
macro_rules! static_assert {
    ($expr:expr $(,)?) => {
        $crate::__static_assert!($expr, stringify!($expr))
    };
    ($expr:expr, $msg:expr $(,)?) => {
        $crate::__static_assert!($expr, $msg)
    };
}

#[macro_export]
macro_rules! __static_assert {
    ($expr:expr, $msg:expr $(,)?) => {
        const _: () = ::core::assert!($expr, $msg);
    };
}

/*
 * Compile time check that field has an expected offset
 */
#[macro_export]
macro_rules! ASSERT_STRUCT_OFFSET {
    ($type:ty, $field:tt, $expected_offset:expr $(,)?) => {{
        $crate::BUILD_BUG_ON_MSG!(
            ::core::mem::offset_of!($type, $field) != ($expected_offset),
            concat!(
                "Offset of ",
                stringify!($field),
                " in ",
                stringify!($type),
                " has changed."
            )
        )
    }};
}
