/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * NOLIBC compiler support header
 * Copyright (C) 2023 Thomas Weißschuh <linux@weissschuh.net>
 */

/*
 * C compiler feature probes:
 *
 *   __nolibc_has_attribute(attr) -> __has_attribute(attr), or 0
 *   __nolibc_has_feature(feature) -> __has_feature(feature), or 0
 *
 * Rust has no direct file-local equivalent for these C preprocessor probes.
 * The translated macros preserve the fallback value used when the C probe is
 * unavailable.
 */
#[macro_export]
macro_rules! __nolibc_has_attribute {
    ($attr:tt) => {
        0
    };
}

#[macro_export]
macro_rules! __nolibc_has_feature {
    ($feature:tt) => {
        0
    };
}

#[macro_export]
macro_rules! __nolibc_aligned {
    ($alignment:literal) => {
        #[repr(align($alignment))]
    };
}

#[macro_export]
macro_rules! __nolibc_aligned_as {
    ($type:ty) => {
        /*
         * C used __attribute__((aligned(__alignof__(type)))).
         * Rust repr(align(N)) requires a literal power-of-two alignment, so the
         * type-dependent attribute form cannot be emitted file-locally.
         */
    };
}

/*
 * C selects either:
 *   __attribute__((naked)) with an empty epilogue, or
 *   __attribute__((optimize("Os", "omit-frame-pointer"))) with
 *   __builtin_unreachable() as the epilogue.
 *
 * File-local Rust cannot probe C attributes. Preserve the fallback behavior.
 */
#[macro_export]
macro_rules! __nolibc_entrypoint {
    () => {
        #[optimize(size)]
    };
}

#[macro_export]
macro_rules! __nolibc_entrypoint_epilogue {
    () => {
        unsafe { core::hint::unreachable_unchecked() }
    };
}

/*
 * _NOLIBC_STACKPROTECTOR is defined in C when one of:
 * __SSP__, __SSP_STRONG__, __SSP_ALL__, or __SSP_EXPLICIT__ is defined.
 * That preprocessor state has no file-local Rust equivalent.
 */

/*
 * C selects no_stack_protector when supported, otherwise an optimize attribute
 * requesting -fno-stack-protector. Rust has no stable direct equivalent here.
 */
#[macro_export]
macro_rules! __nolibc_no_stack_protector {
    () => {};
}

#[macro_export]
macro_rules! __nolibc_fallthrough {
    () => {
        ()
    };
}

/*
 * __nolibc_stdc_version maps to __STDC_VERSION__ when present, otherwise 0.
 * Rust has no C standard-version macro, so preserve the fallback value.
 */
pub const __nolibc_stdc_version: i64 = 0;

#[macro_export]
macro_rules! __nolibc_version {
    ($_major:expr, $_minor:expr, $_patch:expr) => {
        ($_major) * 10000 + ($_minor) * 100 + ($_patch)
    };
}

/*
 * __nolibc_gnuc_version and __nolibc_clang_version are computed from C
 * compiler predefined macros when present, otherwise 0. Rust has no file-local
 * equivalent for those C predefined macros, so preserve the fallback values.
 */
pub const __nolibc_gnuc_version: i32 = 0;
pub const __nolibc_clang_version: i32 = 0;

/*
 * C defines __nolibc_static_assert(_t) as _Static_assert(_t, "") when the C
 * standard or compiler version supports it, otherwise as an empty macro.
 * Rust const assertions can represent the supported form.
 */
#[macro_export]
macro_rules! __nolibc_static_assert {
    ($_t:expr) => {
        const _: () = assert!($_t);
    };
}

/* Make the optimizer believe the variable can be manipulated arbitrarily. */
#[macro_export]
macro_rules! _NOLIBC_OPTIMIZER_HIDE_VAR {
    ($var:expr) => {
        unsafe {
            core::arch::asm!("", inout(reg) $var, options(nostack, preserves_flags));
        }
    };
}

/*
 * C conditionally applies sanitizer-disabling attributes for undefined behavior
 * sanitizer builds. Rust has no file-local equivalent for that C feature probe
 * or those C attributes.
 */
#[macro_export]
macro_rules! __nolibc_no_sanitize_undefined {
    () => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
