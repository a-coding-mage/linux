/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
 */

// Declaration translated from the C header.
unsafe extern "C" {
    pub fn xterm_fd(socket: i32, pid_out: *mut i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
