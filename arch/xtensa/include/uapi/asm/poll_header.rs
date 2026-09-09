/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/poll.h
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C header guard: _XTENSA_POLL_H

pub const POLLWRNORM: i32 = POLLOUT;
pub const POLLWRBAND: i32 = 0x0100;
pub const POLLREMOVE: i32 = 0x0800;

// Dependency preserved from <asm-generic/poll.h>; its declarations and
// constants are supplied by the surrounding translated headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
