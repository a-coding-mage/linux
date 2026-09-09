/* SPDX-License-Identifier: GPL-2.0 */

// This header must not be compiled as part of the floating-point code
// compilation unit. See Documentation/core-api/floating-point.rst.
// The original C header emits an error when _LINUX_FPU_COMPILATION_UNIT is
// defined; preserve that build-time condition here for the Rust integration.

// Dependency supplied by the architecture-specific Rust translation of
// <asm/fpu.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
