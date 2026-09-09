/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 ARM Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Using our own definitions for AArch32 (compat) support.
 */
pub const O_DIRECTORY: i32 = 1 << 14; /* must be a directory */
pub const O_NOFOLLOW: i32 = 1 << 15; /* don't follow links */
pub const O_DIRECT: i32 = 1 << 16; /* direct disk access hint - currently ignored */
pub const O_LARGEFILE: i32 = 1 << 17;

/* Dependency: declarations from <asm-generic/fcntl.h> are supplied externally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
