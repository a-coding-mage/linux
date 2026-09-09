/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acgcc.h - GCC specific defines, etc.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* C header dependencies: stdarg.h or linux/stdarg.h provide va_arg. */

/// GCC's inline specifier. Rust functions are declared with `#[inline]` where
/// applicable; this marker preserves the source-level macro intent.
pub const ACPI_INLINE: &str = "__inline__";

/* Function name is used for debug output. Non-ANSI, compiler-dependent. */
#[macro_export]
macro_rules! ACPI_GET_FUNCTION_NAME {
    () => {{
        module_path!()
    }};
}

/*
 * This macro is used to tag functions as "printf-like" because
 * some compilers (like GCC) can catch printf format string problems.
 * Rust has no direct local equivalent for GCC's format attribute.
 */
#[macro_export]
macro_rules! ACPI_PRINTF_LIKE {
    ($c:expr) => {};
}

/*
 * Some compilers complain about unused variables. Sometimes we don't want to
 * use all the variables (for example, _acpi_module_name). This allows us to
 * tell the compiler warning in a per-variable manner that a variable is
 * unused.
 * Rust's `#[allow(unused_variables)]` is the corresponding attribute.
 */
#[macro_export]
macro_rules! ACPI_UNUSED_VAR {
    ($item:item) => {
        #[allow(unused_variables)]
        $item
    };
}

/* GCC supports __VA_ARGS__ in macros. */
pub const COMPILER_VA_MACRO: i32 = 1;

/* GCC supports native multiply/shift on 32-bit platforms. */
pub const ACPI_USE_NATIVE_MATH64: bool = true;

/* GCC did not support __has_attribute until 5.1. */
#[macro_export]
macro_rules! __has_attribute {
    ($x:ident) => { 0 };
}

/*
 * Explicitly mark intentional explicit fallthrough to silence
 * -Wimplicit-fallthrough in GCC 7.1+.
 * The condition is compiler-specific and has no direct Rust attribute.
 */
#[macro_export]
macro_rules! ACPI_FALLTHROUGH {
    () => {};
}

/*
 * Flexible array members are not allowed to be part of a union under
 * C99, but this is not for any technical reason. Work around the limitation.
 * Rust uses a zero-length trailing array to preserve the declaration's
 * flexible-array layout intent.
 */
#[macro_export]
macro_rules! ACPI_FLEX_ARRAY {
    ($TYPE:ty, $NAME:ident) => {
        #[repr(C)]
        pub struct $NAME {
            pub __Empty_$NAME: (),
            pub $NAME: [$TYPE; 0],
        }
    };
}

/*
 * Explicitly mark strings that lack a terminating NUL character so
 * that ACPICA can be built with -Wunterminated-string-initialization.
 * Rust byte strings carry their length explicitly and need no attribute.
 */
#[macro_export]
macro_rules! ACPI_NONSTRING {
    ($item:item) => { $item };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
