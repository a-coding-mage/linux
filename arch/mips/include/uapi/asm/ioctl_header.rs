/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 99, 2001 Ralf Baechle <ralf@linux-mips.org>
 * Copyright (C) 2009 Wind River Systems
 * Written by Ralf Baechle <ralf@linux-mips.org>
 */

pub const _IOC_SIZEBITS: u32 = 13;
pub const _IOC_DIRBITS: u32 = 3;

/*
 * Direction bits _IOC_NONE could be 0, but OSF/1 gives it a bit.
 * And this turns out useful to catch old ioctl numbers in header
 * files for us.
 */
pub const _IOC_NONE: u32 = 1u32;
pub const _IOC_READ: u32 = 2u32;
pub const _IOC_WRITE: u32 = 4u32;

/* Declarations from <asm-generic/ioctl.h> are supplied externally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
