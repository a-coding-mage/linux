/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Include file for the interface to IST BIOS
 * Copyright 2002 Andy Grover <andrew.grover@intel.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 */

// Dependency intent: `__u32` from <linux/types.h> is represented by `u32`.

#[repr(C)]
pub struct ist_info {
    pub signature: u32,
    pub command: u32,
    pub event: u32,
    pub perf_level: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
