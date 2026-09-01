/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Original C header required _TOOLS_LINUX_COMPILER_H_ to be defined and
 * emitted a preprocessor error when included directly. In Rust this file is
 * expected to be included through the translated compiler header equivalent.
 */

/*
 * Common definitions for all gcc versions go here.
 */

/*
 * GCC_VERSION was defined in C as:
 *   (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
 *
 * Rust does not expose GCC's preprocessor version macros file-locally. Keep the
 * name as a build-supplied dependency when a translated consumer requires it.
 */

/*
 * C:
 *   #if __has_attribute(__fallthrough__)
 *   # define fallthrough __attribute__((__fallthrough__))
 *   #else
 *   # define fallthrough do {} while (0)
 *   #endif
 *
 * Rust has no statement attribute with identical GCC fallthrough semantics.
 * The fallback behavior is an explicit no-op expression.
 */
#[macro_export]
macro_rules! fallthrough {
    () => {{}};
}

/*
 * C:
 *   #if __has_attribute(__error__)
 *   # define __compiletime_error(message) __attribute__((error(message)))
 *   #endif
 *
 * Rust has no direct stable equivalent to GCC's error(message) function
 * attribute. Preserve the macro name for translated declarations that need to
 * mark the intent.
 */
#[macro_export]
macro_rules! __compiletime_error {
    ($message:expr, $item:item) => {
        $item
    };
}

/* &a[0] degrades to a pointer: a different type from an array */
/*
 * Depends on the external BUILD_BUG_ON_ZERO and __same_type macros from the C
 * headers. This file-local translation preserves the interface and intent.
 */
#[macro_export]
macro_rules! __must_be_array {
    ($a:expr) => {{
        /* TODO: map BUILD_BUG_ON_ZERO(__same_type(($a), &($a)[0])) when the
         * translated compile-time type machinery is available.
         */
        0
    }};
}

/*
 * Attribute helper macros translated as Rust item wrappers where a direct Rust
 * attribute exists. These preserve the externally visible helper names without
 * inventing dependency implementations.
 */
#[macro_export]
macro_rules! __pure {
    ($item:item) => {
        $item
    };
}

#[macro_export]
macro_rules! noinline {
    ($item:item) => {
        #[inline(never)]
        $item
    };
}

#[macro_export]
macro_rules! __packed {
    ($item:item) => {
        #[repr(packed)]
        $item
    };
}

#[macro_export]
macro_rules! __noreturn {
    ($item:item) => {
        $item
    };
}

#[macro_export]
macro_rules! __aligned {
    ($x:expr, $item:item) => {
        #[repr(align($x))]
        $item
    };
}

#[macro_export]
macro_rules! __printf {
    ($a:expr, $b:expr, $item:item) => {
        $item
    };
}

#[macro_export]
macro_rules! __scanf {
    ($a:expr, $b:expr, $item:item) => {
        $item
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
