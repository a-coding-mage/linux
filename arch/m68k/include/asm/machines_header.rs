/* SPDX-License-Identifier: GPL-2.0 */
/*
 * machines.h: Defines for taking apart the machine type value in the
 *             idprom and determining the kind of machine we are on.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Sun3/3x models added by David Monro (davidm@psrg.cs.usyd.edu.au)
 */

#[repr(C)]
pub struct Sun_Machine_Models {
    pub name: *mut core::ffi::c_char,
    pub id_machtype: u8,
}

/* Current number of machines we know about that has an IDPROM
 * machtype entry including one entry for the 0x80 OBP machines.
 */
// reduced along with table in arch/m68k/sun3/idprom.c
// sun3 port doesn't need to know about sparc machines.
// #define NUM_SUN_MACHINES 23
pub const NUM_SUN_MACHINES: u32 = 8;

/* The machine type in the idprom area looks like this:
 *
 * ---------------
 * | ARCH | MACH |
 * ---------------
 *  7    4 3    0
 *
 * The ARCH field determines the architecture line (sun4, sun4c, etc).
 * The MACH field determines the machine make within that architecture.
 */

pub const SM_ARCH_MASK: u8 = 0xf0;
pub const SM_SUN3: u8 = 0x10;
pub const SM_SUN4: u8 = 0x20;
pub const SM_SUN3X: u8 = 0x40;
pub const SM_SUN4C: u8 = 0x50;
pub const SM_SUN4M: u8 = 0x70;
pub const SM_SUN4M_OBP: u8 = 0x80;

pub const SM_TYP_MASK: u8 = 0x0f;
/* Sun3 machines */
pub const SM_3_160: u8 = 0x01; /* Sun 3/160 series */
pub const SM_3_50: u8 = 0x02; /* Sun 3/50 series */
pub const SM_3_260: u8 = 0x03; /* Sun 3/260 series */
pub const SM_3_110: u8 = 0x04; /* Sun 3/110 series */
pub const SM_3_60: u8 = 0x07; /* Sun 3/60 series */
pub const SM_3_E: u8 = 0x08; /* Sun 3/E series */

/* Sun3x machines */
pub const SM_3_460: u8 = 0x01; /* Sun 3/460 (460,470,480) series */
pub const SM_3_80: u8 = 0x02; /* Sun 3/80 series */

/* Sun4 machines */
pub const SM_4_260: u8 = 0x01; /* Sun 4/200 series */
pub const SM_4_110: u8 = 0x02; /* Sun 4/100 series */
pub const SM_4_330: u8 = 0x03; /* Sun 4/300 series */
pub const SM_4_470: u8 = 0x04; /* Sun 4/400 series */

/* Sun4c machines                Full Name              - PROM NAME */
pub const SM_4C_SS1: u8 = 0x01; /* Sun4c SparcStation 1   - Sun 4/60  */
pub const SM_4C_IPC: u8 = 0x02; /* Sun4c SparcStation IPC - Sun 4/40  */
pub const SM_4C_SS1PLUS: u8 = 0x03; /* Sun4c SparcStation 1+  - Sun 4/65  */
pub const SM_4C_SLC: u8 = 0x04; /* Sun4c SparcStation SLC - Sun 4/20  */
pub const SM_4C_SS2: u8 = 0x05; /* Sun4c SparcStation 2   - Sun 4/75  */
pub const SM_4C_ELC: u8 = 0x06; /* Sun4c SparcStation ELC - Sun 4/25  */
pub const SM_4C_IPX: u8 = 0x07; /* Sun4c SparcStation IPX - Sun 4/50  */

/* Sun4m machines, these predate the OpenBoot.  These values only mean
 * something if the value in the ARCH field is SM_SUN4M, if it is
 * SM_SUN4M_OBP then you have the following situation:
 * 1) You either have a sun4d, a sun4e, or a recently made sun4m.
 * 2) You have to consult OpenBoot to determine which machine this is.
 */
pub const SM_4M_SS60: u8 = 0x01; /* Sun4m SparcSystem 600                  */
pub const SM_4M_SS50: u8 = 0x02; /* Sun4m SparcStation 10                  */
pub const SM_4M_SS40: u8 = 0x03; /* Sun4m SparcStation 5                   */

/* Sun4d machines -- N/A */
/* Sun4e machines -- N/A */
/* Sun4u machines -- N/A */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
