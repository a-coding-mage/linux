/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Common user-facing libbpf helpers.
 *
 * Copyright (c) 2019 Facebook
 */

/* C dependencies removed from executable Rust:
 * - <string.h> for memset() and memcpy()
 * - "libbpf_version.h" for LIBBPF_MAJOR_VERSION and LIBBPF_MINOR_VERSION
 */

/* In C, LIBBPF_API defaults to __attribute__((visibility("default"))).
 * Rust symbol visibility/export has to be expressed on each exported item.
 */

/* C: #define LIBBPF_DEPRECATED(msg) __attribute__((deprecated(msg))) */
#[macro_export]
macro_rules! LIBBPF_DEPRECATED {
    ($msg:expr) => {
        #[deprecated(note = $msg)]
    };
}

/* Mark a symbol as deprecated when libbpf version is >= {major}.{minor}.
 *
 * The C macro constructs a version-specific preprocessor symbol name:
 * __LIBBPF_MARK_DEPRECATED_{major}_{minor}(LIBBPF_DEPRECATED(...)).
 * Rust cannot paste attribute macro names in the same way, so supported
 * version gates are translated explicitly below.
 */
#[macro_export]
macro_rules! LIBBPF_DEPRECATED_SINCE {
    (1, 0, $msg:expr) => {
        $crate::__LIBBPF_MARK_DEPRECATED_1_0!(
            #[deprecated(note = concat!("libbpf v", "1", ".", "0", "+: ", $msg))]
        )
    };
    ($major:literal, $minor:literal, $msg:expr) => {
        /* No file-local Rust equivalent for the C preprocessor token-pasting
         * dispatch to an undeclared __LIBBPF_MARK_DEPRECATED_{major}_{minor}.
         */
    };
}

#[macro_export]
macro_rules! __LIBBPF_CURRENT_VERSION_GEQ {
    ($major:expr, $minor:expr) => {
        (LIBBPF_MAJOR_VERSION > ($major)
            || (LIBBPF_MAJOR_VERSION == ($major) && LIBBPF_MINOR_VERSION >= ($minor)))
    };
}

/* Add checks for other versions below when planning deprecation of API symbols
 * with the LIBBPF_DEPRECATED_SINCE macro.
 *
 * C condition:
 * #if __LIBBPF_CURRENT_VERSION_GEQ(1, 0)
 */
#[cfg(any(
    all(libbpf_major_version = "1", any(libbpf_minor_version = "0", libbpf_minor_version = "1")),
    libbpf_major_version_ge_2
))]
#[macro_export]
macro_rules! __LIBBPF_MARK_DEPRECATED_1_0 {
    ($x:meta) => {
        #[$x]
    };
}

#[cfg(not(any(
    all(libbpf_major_version = "1", any(libbpf_minor_version = "0", libbpf_minor_version = "1")),
    libbpf_major_version_ge_2
)))]
#[macro_export]
macro_rules! __LIBBPF_MARK_DEPRECATED_1_0 {
    ($x:meta) => {};
}

/* This set of internal macros allows to do "function overloading" based on
 * number of arguments provided by used in backwards-compatible way during the
 * transition to libbpf 1.0
 * It's ugly but necessary evil that will be cleaned up when we get to 1.0.
 * See bpf_prog_load() overload for example.
 */
#[macro_export]
macro_rules! ___libbpf_cat {
    ($a:ident, $b:ident) => {
        concat!(stringify!($a), stringify!($b))
    };
}

#[macro_export]
macro_rules! ___libbpf_select {
    ($name:ident, $num:ident) => {
        ___libbpf_cat!($name, $num)
    };
}

#[macro_export]
macro_rules! ___libbpf_nth {
    ($_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $n:tt $(, $rest:tt)*) => {
        $n
    };
}

#[macro_export]
macro_rules! ___libbpf_cnt {
    ($($args:tt),*) => {
        ___libbpf_nth!($($args,)* 6, 5, 4, 3, 2, 1)
    };
}

/* Rust macro_rules! cannot form and invoke a new macro identifier by token
 * pasting the way the C preprocessor can with NAME ## NUM.
 */
#[macro_export]
macro_rules! ___libbpf_overload {
    ($name:ident, $($args:tt),*) => {
        compile_error!("___libbpf_overload requires C preprocessor token-pasting semantics")
    };
}

/* Helper macro to declare and initialize libbpf options struct
 *
 * This dance with uninitialized declaration, followed by memset to zero,
 * followed by assignment using compound literal syntax is done to preserve
 * ability to use a nice struct field initialization syntax and **hopefully**
 * have all the padding bytes initialized to zero. It's not guaranteed though,
 * when copying literal, that compiler won't copy garbage in literal's padding
 * bytes, but that's the best way I've found and it seems to work in practice.
 *
 * Macro declares opts struct of given type and name, zero-initializes,
 * including any extra padding, it with memset() and then assigns initial
 * values provided by users in struct initializer-syntax as varargs.
 */
#[macro_export]
macro_rules! LIBBPF_OPTS {
    ($type:ty, $name:ident $(, $field:ident : $value:expr)* $(,)?) => {
        let mut $name: $type = unsafe { ::core::mem::zeroed() };
        $name.sz = ::core::mem::size_of::<$type>();
        $(
            $name.$field = $value;
        )*
    };
}

/* Helper macro to clear and optionally reinitialize libbpf options struct
 *
 * Small helper macro to reset all fields and to reinitialize the common
 * structure size member. Values provided by users in struct initializer-
 * syntax as varargs can be provided as well to reinitialize options struct
 * specific members.
 */
#[macro_export]
macro_rules! LIBBPF_OPTS_RESET {
    ($name:ident $(, $field:ident : $value:expr)* $(,)?) => {
        unsafe {
            ::core::ptr::write_bytes(
                (&mut $name as *mut _ as *mut u8),
                0,
                ::core::mem::size_of_val(&$name),
            );
        }
        $name.sz = ::core::mem::size_of_val(&$name);
        $(
            $name.$field = $value;
        )*
    };
}
