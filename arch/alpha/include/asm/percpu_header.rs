/* SPDX-License-Identifier: GPL-2.0 */

/*
 * To calculate addresses of locally defined variables, GCC uses
 * 32-bit displacement from the GP. Which doesn't work for per cpu
 * variables in modules, as an offset to the kernel per cpu area is
 * way above 4G.
 *
 * Always use weak definitions for percpu variables in modules.
 * Therefore, we have enabled CONFIG_ARCH_MODULE_NEEDS_WEAK_PER_CPU
 * in the Kconfig.
 */

/* Dependency provided by asm-generic/percpu.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
