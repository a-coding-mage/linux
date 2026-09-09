/* SPDX-License-Identifier: GPL-2.0 */
/*
 * machines.h:  Defines for taking apart the machine type value in the
 *              idprom and determining the kind of machine we are on.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

#[repr(C)]
pub struct Sun_Machine_Models {
    pub name: *mut core::ffi::c_char,
    pub id_machtype: u8,
}

/* The machine type in the idprom area looks like this:
 *
 * ---------------
 * | ARCH | MACH |
 * ---------------
 *  7    4 3    0
 *
 * The ARCH field determines the architecture line (sun4m, etc).
 * The MACH field determines the machine make within that architecture.
 */

pub const SM_ARCH_MASK: u8 = 0xf0;
pub const M_LEON: u8 = 0x30;
pub const SM_SUN4M: u8 = 0x70;
pub const SM_SUN4M_OBP: u8 = 0x80;

pub const SM_TYP_MASK: u8 = 0x0f;

/* Leon machines */
pub const M_LEON3_SOC: u8 = 0x02; /* Leon3 SoC */

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
