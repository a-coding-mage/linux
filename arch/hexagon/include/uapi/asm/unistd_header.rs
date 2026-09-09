/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Syscall support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 and
 * only version 2 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

/*
 *  The kernel pulls this unistd.h in three different ways:
 *  1.  the "normal" way which gets all the __NR defines
 *  2.  with __SYSCALL defined to produce function declarations
 *  3.  with __SYSCALL defined to produce syscall table initialization
 *  See also:  syscalltab.c
 */

// C dependency: declarations from <asm/unistd_32.h> are supplied externally.

pub const __NR_sync_file_range2: i32 = 84;

// C preprocessor directive: #undef __NR_sync_file_range.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
