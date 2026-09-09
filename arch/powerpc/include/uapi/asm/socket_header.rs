/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

pub const SO_RCVLOWAT: i32 = 16;
pub const SO_SNDLOWAT: i32 = 17;
pub const SO_RCVTIMEO_OLD: i32 = 18;
pub const SO_SNDTIMEO_OLD: i32 = 19;
pub const SO_PASSCRED: i32 = 20;
pub const SO_PEERCRED: i32 = 21;

// Dependency intent: declarations from <asm-generic/socket.h> are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
