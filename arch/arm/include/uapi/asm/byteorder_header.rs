/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  arch/arm/include/asm/byteorder.h
 *
 * ARM Endian-ness.  In little endian mode, the data bus is connected such
 * that byte accesses appear as:
 *  0 = d0...d7, 1 = d8...d15, 2 = d16...d23, 3 = d24...d31
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 *
 * When in big endian mode, byte accesses appear as:
 *  0 = d24...d31, 1 = d16...d23, 2 = d8...d15, 3 = d0...d7
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 */

// The original header selects the corresponding Linux byte-order
// declarations based on the build-time __ARMEB__ configuration.
#[cfg(target_endian = "big")]
use linux_byteorder::big_endian::*;

#[cfg(not(target_endian = "big"))]
use linux_byteorder::little_endian::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
