/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Build-time endianness condition preserved from the source header.
// The corresponding byte-order declarations are supplied by external
// dependencies and are intentionally not implemented here.
#[cfg(target_endian = "little")]
// Dependency equivalent of: linux/byteorder/little_endian.h

#[cfg(not(target_endian = "little"))]
// Dependency equivalent of: linux/byteorder/big_endian.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
