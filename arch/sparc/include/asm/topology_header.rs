/* SPDX-License-Identifier: GPL-2.0 */

// Conditional dependency from the original header:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/topology_64.h>
// #else
// #include <asm/topology_32.h>
// #endif

// When compiling for SPARC64, provide the declarations from asm/topology_64.h.
// Otherwise, provide the declarations from asm/topology_32.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
