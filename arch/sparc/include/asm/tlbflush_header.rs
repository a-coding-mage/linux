/* SPDX-License-Identifier: GPL-2.0 */

// Source conditional: on 64-bit SPARC (__sparc__ && __arch64__), use
// <asm/tlbflush_64.h>; otherwise, use <asm/tlbflush_32.h>.
// The selected architecture-specific declarations are supplied by future
// dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
