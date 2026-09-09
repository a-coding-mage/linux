/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust equivalents of the Linux build-time assertion helpers.  The
 * compiler-specific declarations used by the C header are supplied by the
 * surrounding translation unit.
 */

/* Force a compilation error if condition is true, while producing a value
 * of type i32 and value 0 when the condition is false. */
#[macro_export]
macro_rules! BUILD_BUG_ON_ZERO {
    ($e:expr $(, $msg:expr)?) => {{
        const _: () = assert!(!$e);
        0i32
    }};
}

/* Force a compilation error if a constant expression is not a power of 2. */
#[macro_export]
macro_rules! __BUILD_BUG_ON_NOT_POWER_OF_2 {
    ($n:expr) => {
        BUILD_BUG_ON!((($n) & (($n) - 1)) != 0)
    };
}

#[macro_export]
macro_rules! BUILD_BUG_ON_NOT_POWER_OF_2 {
    ($n:expr) => {
        BUILD_BUG_ON!(($n) == 0 || ((($n) & (($n) - 1)) != 0))
    };
}

/* Check the validity of the expression without generating code for it. */
#[macro_export]
macro_rules! BUILD_BUG_ON_INVALID {
    ($e:expr) => {{
        let _ = core::mem::size_of_val(&$e);
    }};
}

/** BUILD_BUG_ON_MSG - break compile if a condition is true and emit a message. */
#[macro_export]
macro_rules! BUILD_BUG_ON_MSG {
    ($cond:expr, $msg:expr) => {{
        const _: () = assert!(!$cond, $msg);
    }};
}

/** BUILD_BUG_ON - break compile if a condition is true. */
#[macro_export]
macro_rules! BUILD_BUG_ON {
    ($condition:expr) => {
        BUILD_BUG_ON_MSG!($condition, concat!("BUILD_BUG_ON failed: ", stringify!($condition)))
    };
}

/** BUILD_BUG - break compile if used. */
#[macro_export]
macro_rules! BUILD_BUG {
    () => {
        BUILD_BUG_ON_MSG!(true, "BUILD_BUG failed")
    };
}

/* Check an integer constant expression at build time. */
#[macro_export]
macro_rules! static_assert {
    ($expr:expr $(, $msg:expr)?) => {
        const _: () = assert!($expr $(, $msg)?);
    };
}

/* Compile-time check that a field has an expected offset. */
#[macro_export]
macro_rules! ASSERT_STRUCT_OFFSET {
    ($type:ty, $field:tt, $expected_offset:expr) => {
        BUILD_BUG_ON_MSG!(
            core::mem::offset_of!($type, $field) != ($expected_offset),
            concat!(
                "Offset of ", stringify!($field), " in ", stringify!($type),
                " has changed."
            )
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
