// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Kernel version string
 */

// Values supplied by the generated headers included by the C source.
pub static kernel_version: &[u8] = concat!(
    UTS_RELEASE,
    " (",
    LINUX_COMPILE_BY,
    "@",
    LINUX_COMPILE_HOST,
    ") ",
    UTS_VERSION,
    "\0",
).as_bytes();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
