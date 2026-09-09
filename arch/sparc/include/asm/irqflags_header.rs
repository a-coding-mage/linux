/* SPDX-License-Identifier: GPL-2.0 */

// C source conditional:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/irqflags_64.h>
// #else
// #include <asm/irqflags_32.h>
// #endif
//
// The selected architecture-specific declarations are supplied by the
// corresponding dependency in the translated Rust build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
