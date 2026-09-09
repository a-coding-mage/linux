/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// MicroBlaze byte order selection.
// When the target is little-endian, use the Linux little-endian byte-order
// definitions; otherwise, use the Linux big-endian byte-order definitions.
// These external header dependencies are intentionally not implemented here.
#[cfg(__MICROBLAZEEL__)]
// Dependency equivalent: linux/byteorder/little_endian.h

#[cfg(not(__MICROBLAZEEL__))]
// Dependency equivalent: linux/byteorder/big_endian.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
