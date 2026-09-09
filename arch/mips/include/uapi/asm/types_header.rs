/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 1995, 1996, 1999 by Ralf Baechle
 * Copyright (C) 2008 Wind River Systems,
 *   written by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

/*
 * We don't use int-l64.h for the kernel anymore but still use it for
 * userspace to avoid code changes.
 *
 * However, some user programs (e.g. perf) may not want this. They can
 * flag __SANE_USERSPACE_TYPES__ to get int-ll64.h here.
 *
 * The original header selects one of the asm-generic integer type headers
 * for non-kernel builds:
 * - _MIPS_SZLONG == 64 && !defined(__SANE_USERSPACE_TYPES__): int-l64.h
 * - otherwise: int-ll64.h
 * These dependencies are supplied externally.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
