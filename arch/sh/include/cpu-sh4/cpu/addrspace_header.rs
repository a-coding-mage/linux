/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 1999 by Kaz Kojima
 *
 * Defitions for the address spaces of the SH-4 CPUs.
 */

pub const P0SEG: u32 = 0x00000000;
pub const P1SEG: u32 = 0x80000000;
pub const P2SEG: u32 = 0xa0000000;
pub const P3SEG: u32 = 0xc0000000;
pub const P4SEG: u32 = 0xe0000000;

/* Detailed P4SEG  */
pub const P4SEG_STORE_QUE: u32 = P4SEG;
pub const P4SEG_IC_ADDR: u32 = 0xf0000000;
pub const P4SEG_IC_DATA: u32 = 0xf1000000;
pub const P4SEG_ITLB_ADDR: u32 = 0xf2000000;
pub const P4SEG_ITLB_DATA: u32 = 0xf3000000;
pub const P4SEG_OC_ADDR: u32 = 0xf4000000;
pub const P4SEG_OC_DATA: u32 = 0xf5000000;
pub const P4SEG_TLB_ADDR: u32 = 0xf6000000;
pub const P4SEG_TLB_DATA: u32 = 0xf7000000;
pub const P4SEG_REG_BASE: u32 = 0xff000000;

pub const PA_AREA0: u32 = 0x00000000;
pub const PA_AREA1: u32 = 0x04000000;
pub const PA_AREA2: u32 = 0x08000000;
pub const PA_AREA3: u32 = 0x0c000000;
pub const PA_AREA4: u32 = 0x10000000;
pub const PA_AREA5: u32 = 0x14000000;
pub const PA_AREA6: u32 = 0x18000000;
pub const PA_AREA7: u32 = 0x1c000000;

pub const PA_AREA5_IO: u32 = 0xb4000000; /* Area 5 IO Memory */
pub const PA_AREA6_IO: u32 = 0xb8000000; /* Area 6 IO Memory */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
