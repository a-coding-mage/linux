/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 1995, 1996 by Ralf Baechle
 */

/*
 * Options for cacheflush system call
 */
pub const ICACHE: i32 = 1 << 0; /* flush instruction cache */
pub const DCACHE: i32 = 1 << 1; /* writeback and flush data cache */
pub const BCACHE: i32 = ICACHE | DCACHE; /* flush both caches */

/*
 * Caching modes for the cachectl(2) call
 *
 * cachectl(2) is currently not supported and returns ENOSYS.
 */
pub const CACHEABLE: i32 = 0; /* make pages cacheable */
pub const UNCACHEABLE: i32 = 1; /* make pages uncacheable */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
