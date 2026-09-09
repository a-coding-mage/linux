/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * This file contains the system call numbers.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Conditional dependency preserved from the C header:
// when building for 32-bit PowerPC, use the declarations from asm/unistd_32.h;
// otherwise, use the declarations from asm/unistd_64.h.
#[cfg(not(target_arch = "powerpc64"))]
use crate::asm::unistd_32::*;

#[cfg(target_arch = "powerpc64")]
use crate::asm::unistd_64::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
