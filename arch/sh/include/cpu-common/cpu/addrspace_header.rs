/* SPDX-License-Identifier: GPL-2.0
 *
 * Definitions for the address spaces of the SH-2 CPUs.
 *
 * Copyright (C) 2003  Paul Mundt
 */

// Translated from the C header guard: __ASM_CPU_SH2_ADDRSPACE_H.

pub const P0SEG: u32 = 0x00000000;
pub const P1SEG: u32 = 0x80000000;
pub const P2SEG: u32 = 0xa0000000;
pub const P3SEG: u32 = 0xc0000000;
pub const P4SEG: u32 = 0xe0000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
