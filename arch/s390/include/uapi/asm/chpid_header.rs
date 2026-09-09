/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *    Copyright IBM Corp. 2007, 2012
 *    Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
 */

// Dependencies supplied by the surrounding Linux UAPI translation.

pub const __MAX_CHPID: u8 = 255;

#[repr(C, packed)]
pub struct chp_id {
	pub reserved1: u8,
	pub cssid: u8,
	pub reserved2: u8,
	pub id: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
