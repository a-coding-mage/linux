/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2015 Regents of the University of California
 */

// C equivalent: #define __BITS_PER_LONG (__SIZEOF_POINTER__ * 8)
pub const __BITS_PER_LONG: usize = core::mem::size_of::<*const ()>() * 8;

// C dependency: #include <asm-generic/bitsperlong.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
