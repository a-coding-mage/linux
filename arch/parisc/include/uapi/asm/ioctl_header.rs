/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *    Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *    Copyright (C) 1999,2003 Matthew Wilcox < willy at debian . org >
 *    portions from "linux/ioctl.h for Linux" by H.H. Bergman.
 *
 *    This program is free software; you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation; either version 2 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with this program; if not, write to the Free Software
 *    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// ioctl command encoding: 32 bits total, command in lower 16 bits,
// size of the parameter structure in the lower 14 bits of the
// upper 16 bits.
// Encoding the size of the parameter structure in the ioctl request
// is useful for catching programs compiled with old versions
// and to avoid overwriting user space outside the user buffer area.
// The highest 2 bits are reserved for indicating the ``access mode''.
// NOTE: This limits the max parameter size to 16kB -1 !

// Direction bits.
pub const _IOC_NONE: u32 = 0U32;
pub const _IOC_WRITE: u32 = 2U32;
pub const _IOC_READ: u32 = 1U32;

// The declarations from <asm-generic/ioctl.h> are supplied by another
// translation unit and are intentionally not reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
