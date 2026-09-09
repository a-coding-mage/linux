/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header includes <linux/byteorder/little_endian.h> when
// __LITTLE_ENDIAN__ is defined; otherwise it includes
// <linux/byteorder/big_endian.h>. These dependencies are supplied externally.
//
// Conditional intent preserved from the source:
// #ifdef __LITTLE_ENDIAN__
// use linux::byteorder::little_endian;
// #else
// use linux::byteorder::big_endian;
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
