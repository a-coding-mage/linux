/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for Fibre Channel.
 *
 * Version:	@(#)if_fc.h	0.0	11/20/98
 *
 * Author: Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *		Donald Becker, <becker@super.org>
 *    Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
 *	  Vineet Abraham, <vma@iol.unh.edu>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		of the License, or (at your option) any later version.
 */

// Dependency supplied by linux/types.h in the original header:
// __u8 and __be16 are referenced here as external Rust types.

pub const FC_ALEN: usize = 6; /* Octets in one ethernet addr */
pub const FC_ID_LEN: usize = 3; /* Octets in a Fibre Channel Address */

/* LLC and SNAP constants */
pub const EXTENDED_SAP: u32 = 0xAA;
pub const UI_CMD: u32 = 0x03;

/* This is NOT the Fibre Channel frame header. The FC frame header is
 * constructed in the driver as the Tachyon needs certain fields in
 * certains positions. So, it can't be generalized here. */
#[repr(C)]
pub struct fch_hdr {
	pub daddr: [__u8; FC_ALEN], /* destination address */
	pub saddr: [__u8; FC_ALEN], /* source address */
}

/* This is a Fibre Channel LLC structure */
#[repr(C)]
pub struct fcllc {
	pub dsap: __u8,       /* destination SAP */
	pub ssap: __u8,       /* source SAP */
	pub llc: __u8,        /* LLC control field */
	pub protid: [__u8; 3], /* protocol id */
	pub ethertype: __be16, /* ether type field */
}

pub const FC_HLEN: usize = core::mem::size_of::<fch_hdr>() + core::mem::size_of::<fcllc>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
