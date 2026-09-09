/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-apollo.h -- Apollo-specific boot information definitions
*/

/* Apollo-specific tags */

pub const BI_APOLLO_MODEL: u32 = 0x8000; /* model (__be32) */

/* Apollo models (BI_APOLLO_MODEL) */

pub const APOLLO_UNKNOWN: u32 = 0;
pub const APOLLO_DN3000: u32 = 1;
pub const APOLLO_DN3010: u32 = 2;
pub const APOLLO_DN3500: u32 = 3;
pub const APOLLO_DN4000: u32 = 4;
pub const APOLLO_DN4500: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
