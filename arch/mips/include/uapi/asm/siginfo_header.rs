/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998, 1999, 2001, 2003 Ralf Baechle
 * Copyright (C) 2000, 2001 Silicon Graphics, Inc.
 */

pub const __ARCH_SIGEV_PREAMBLE_SIZE: usize =
    core::mem::size_of::<core::ffi::c_long>()
        + 2 * core::mem::size_of::<core::ffi::c_int>();

// __ARCH_HAS_SWAPPED_SIGINFO

// Definitions supplied by asm-generic/siginfo.h are external dependencies.

/*
 * si_code values
 * Again these have been chosen to be IRIX compatible.
 */
pub const SI_ASYNCIO: i32 = -2; /* sent by AIO completion */
pub const SI_TIMER: i32 = -3; /* sent by timer expiration */
pub const SI_MESGQ: i32 = -4; /* sent by real time mesq state change */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
