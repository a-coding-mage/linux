/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * alloca() for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols */
/* C dependency intent: #include "nolibc.h" */

/* Header guard in C: _NOLIBC_ALLOCA_H */

/*
 * C macro:
 * #define alloca(size) __builtin_alloca(size)
 *
 * Rust has no direct stable source-level equivalent for the compiler builtin
 * stack allocation operation. Preserve the externally visible macro name and
 * make the missing builtin mapping explicit at the expansion site.
 */
macro_rules! alloca {
    ($size:expr) => {{
        compile_error!("alloca(size) requires the C compiler builtin __builtin_alloca(size), which has no direct Rust equivalent in this isolated translation");
    }};
}
