/* SPDX-License-Identifier: GPL-2.0 */

// Build-time conditional dependency:
// #ifdef __uClinux__
//     #include <asm/cacheflush_no.h>
// #else
//     #include <asm/cacheflush_mm.h>
// #endif
//
// The corresponding declarations are supplied by the selected dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
