/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 98, 99, 2000 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

/* C header guard: _ASM_RESOURCE_H */

/*
 * These five resource limit IDs have a MIPS/Linux-specific ordering,
 * the rest comes from the generic header:
 */
pub const RLIMIT_NOFILE: i32 = 5; /* max number of open files */
pub const RLIMIT_AS: i32 = 6; /* address space limit */
pub const RLIMIT_RSS: i32 = 7; /* max resident set size */
pub const RLIMIT_NPROC: i32 = 8; /* max number of processes */
pub const RLIMIT_MEMLOCK: i32 = 9; /* max locked-in-memory address space */

/*
 * SuS says limits have to be unsigned.
 * Which makes a ton more sense anyway,
 * but we keep the old value on MIPS32,
 * for compatibility:
 */
#[cfg(not(target_arch = "mips64"))]
pub const RLIM_INFINITY: usize = 0x7fffffffusize;

/* The declarations from <asm-generic/resource.h> are supplied externally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
