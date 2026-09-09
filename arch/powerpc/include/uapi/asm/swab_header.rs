/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// The original header includes <linux/types.h> and <linux/compiler.h>.
// The declarations supplied by those headers are dependencies of this file.

// The original definition is enabled for GCC builds and omitted for
// __powerpc64__ builds. Rust's target configuration preserves that intent;
// compiler-specific selection remains a build-time condition from the source.
#[cfg(not(target_arch = "powerpc64"))]
pub const __SWAB_64_THRU_32__: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
