/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations from asm/vdso/processor.h are supplied externally.

// Preserve the source build-time selection between the 64-bit and 32-bit
// SPARC processor declarations. The corresponding declarations are supplied
// externally by asm/processor_64.h or asm/processor_32.h.
// When compiling for SPARC with 64-bit architecture, include declarations
// from asm/processor_64.h; otherwise include declarations from
// asm/processor_32.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
