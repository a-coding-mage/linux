/* SPDX-License-Identifier: GPL-2.0 */
/*
 * idprom.h: Macros and defines for idprom routines
 *
 * Copyright (C) 1995,1996 David S. Miller (davem@caip.rutgers.edu)
 */

#[repr(C)]
pub struct idprom {
    pub id_format: u8,      /* Format identifier (always 0x01) */
    pub id_machtype: u8,    /* Machine type */
    pub id_ethaddr: [u8; 6], /* Hardware ethernet address */
    pub id_date: i32,       /* Date of manufacture */
    /* Unique serial number; only the low 24 bits are meaningful. */
    pub id_sernum: u32,
    pub id_cksum: u8,       /* Checksum - xor of the data bytes */
    pub reserved: [u8; 16],
}

extern "C" {
    pub static mut idprom: *mut idprom;
    pub fn idprom_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
