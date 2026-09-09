/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Arch-specific trace clocks.
 */

/*
 * Additional trace clocks added to the trace_clocks
 * array in kernel/trace/trace.c
 * None if the architecture has not defined it.
 *
 * The C header defines ARCH_TRACE_CLOCKS as an empty macro when the
 * architecture has not provided a definition. Rust has no direct equivalent
 * for an empty preprocessor macro.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
