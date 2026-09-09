/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent of the C header guard: ___ASM_SPARC_SFP_MACHINE_H.
//
// Build-time dependency selection preserved from the source:
// - when compiling for SPARC with the 64-bit architecture extension, use
//   asm/sfp-machine_64.h;
// - otherwise, use asm/sfp-machine_32.h.
//
// The selected architecture-specific header is supplied as an external
// dependency and is intentionally not implemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
