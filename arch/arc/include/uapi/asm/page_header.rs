/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// C header guard: _UAPI__ASM_ARC_PAGE_H
// The C header includes <linux/const.h> for _AC and _BITUL.

/* PAGE_SHIFT determines the page size. */
// Under __KERNEL__, PAGE_SHIFT, PAGE_SIZE, and PAGE_MASK are supplied by
// the external vdso/page.h dependency.

/*
 * Default 8k
 * done this way (instead of under CONFIG_ARC_PAGE_SIZE_8K) because adhoc
 * user code (busybox appletlib.h) expects PAGE_SHIFT to be defined w/o
 * using the correct uClibc header and in their build our autoconf.h is
 * not available
 */
#[cfg(not(feature = "kernel"))]
pub const PAGE_SHIFT: usize = 13;

#[cfg(not(feature = "kernel"))]
pub const PAGE_SIZE: usize = 1usize << PAGE_SHIFT; /* Default 8K */

#[cfg(not(feature = "kernel"))]
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub const PAGE_OFFSET: usize = 0x80000000usize; /* Kernel starts at 2G onwrds */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
