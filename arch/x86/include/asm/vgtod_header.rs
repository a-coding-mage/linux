/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This check is required to prevent ARCH=um from including
 * unwanted headers.
 *
 * The original C header includes the following dependencies only when
 * CONFIG_GENERIC_GETTIMEOFDAY is enabled:
 *   linux/compiler.h
 *   asm/clocksource.h
 *   vdso/datapage.h
 *   vdso/helpers.h
 *   uapi/linux/time.h
 *
 * Build-time conditional and dependency intent preserved from the source.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
