/* SPDX-License-Identifier: GPL-2.0 */

// C source conditional:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/pgtable_64.h>
// #else
// #include <asm/pgtable_32.h>
// #endif
//
// The included architecture-specific declarations are supplied by the
// corresponding Rust translation unit. This header contains no declarations
// of its own beyond selecting that dependency.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
