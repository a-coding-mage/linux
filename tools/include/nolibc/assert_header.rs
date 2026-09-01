/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Assert for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols */
/* C dependency: "nolibc.h" */

/* C dependencies inside _NOLIBC_ASSERT_H: "errno.h", "stdio.h", "stdlib.h" */

/* NDEBUG needs to be evaluated on *each* inclusion */
/*
 * C macro translation:
 *
 * #ifndef NDEBUG
 * #define assert(expr) ({ ... })
 * #else
 * #define assert(expr) ((void)0)
 * #endif
 *
 * Rust has no per-inclusion preprocessor evaluation. This macro preserves the
 * non-NDEBUG assertion behavior; builds that define the equivalent of NDEBUG
 * should replace or cfg-disable this macro at inclusion time.
 */
#[macro_export]
macro_rules! assert {
    ($expr:expr) => {{
        if !$expr {
            unsafe {
                fprintf(
                    stderr,
                    c"%s: %s:%d: %s: Assertion `%s' failed.\n".as_ptr(),
                    program_invocation_short_name,
                    c"%s".as_ptr(),
                    line!() as core::ffi::c_int,
                    c"%s".as_ptr(),
                    stringify!($expr).as_ptr(),
                );
                abort();
            }
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
