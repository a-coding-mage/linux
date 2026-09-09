/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

/*
 * ARC ABI flags defined for Android's finegrained cacheflush requirements
 */
pub const CF_I_INV: u32 = 0x0002;
pub const CF_D_FLUSH: u32 = 0x0010;
pub const CF_D_FLUSH_INV: u32 = 0x0020;

pub const CF_DEFAULT: u32 = CF_I_INV | CF_D_FLUSH;

/*
 * Standard flags expected by cacheflush system call users
 */
pub const ICACHE: u32 = CF_I_INV;
pub const DCACHE: u32 = CF_D_FLUSH;
pub const BCACHE: u32 = CF_I_INV | CF_D_FLUSH;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
