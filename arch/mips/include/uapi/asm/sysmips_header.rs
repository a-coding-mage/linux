/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for the MIPS sysmips(2) call
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995 by Ralf Baechle
 */

/*
 * Commands for the sysmips(2) call
 *
 * sysmips(2) is deprecated - though some existing software uses it.
 * We only support the following commands.
 */
pub const SETNAME: i32 = 1; /* set hostname */
pub const FLUSH_CACHE: i32 = 3; /* writeback and invalidate caches */
pub const MIPS_FIXADE: i32 = 7; /* control address error fixing */
pub const MIPS_RDNVRAM: i32 = 10; /* read NVRAM */
pub const MIPS_ATOMIC_SET: i32 = 2001; /* atomically set variable */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
