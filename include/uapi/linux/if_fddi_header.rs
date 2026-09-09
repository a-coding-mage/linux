/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for the ANSI FDDI interface.
 *
 * Version:	@(#)if_fddi.h	1.0.3	Oct  6 2018
 *
 * Author:	Lawrence V. Stefani, <stefani@yahoo.com>
 * Maintainer:	Maciej W. Rozycki, <macro@orcam.me.uk>
 *
 *		if_fddi.h is based on previous if_ether.h and if_tr.h work by
 *			Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *			Donald Becker, <becker@super.org>
 *			Alan Cox, <alan@lxorguk.ukuu.org.uk>
 *			Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
 *			Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 */

/* linux/types.h dependency: __u8 is represented by u8 and __be16 by u16. */

/* Define max and min legal sizes. */
pub const FDDI_K_ALEN: u8 = 6;
pub const FDDI_K_8022_HLEN: u16 = 16;
pub const FDDI_K_SNAP_HLEN: u16 = 21;
pub const FDDI_K_8022_ZLEN: u16 = 16;
pub const FDDI_K_SNAP_ZLEN: u16 = 21;
pub const FDDI_K_8022_DLEN: u16 = 4475;
pub const FDDI_K_SNAP_DLEN: u16 = 4470;
pub const FDDI_K_LLC_ZLEN: u16 = 13;
pub const FDDI_K_LLC_LEN: u16 = 4491;
pub const FDDI_K_OUI_LEN: u8 = 3;

/* Define FDDI Frame Control (FC) Byte masks */
pub const FDDI_FC_K_CLASS_MASK: u8 = 0x80;
pub const FDDI_FC_K_CLASS_SYNC: u8 = 0x80;
pub const FDDI_FC_K_CLASS_ASYNC: u8 = 0x00;
pub const FDDI_FC_K_ALEN_MASK: u8 = 0x40;
pub const FDDI_FC_K_ALEN_48: u8 = 0x40;
pub const FDDI_FC_K_ALEN_16: u8 = 0x00;
pub const FDDI_FC_K_FORMAT_MASK: u8 = 0x30;
pub const FDDI_FC_K_FORMAT_FUTURE: u8 = 0x30;
pub const FDDI_FC_K_FORMAT_IMPLEMENTOR: u8 = 0x20;
pub const FDDI_FC_K_FORMAT_LLC: u8 = 0x10;
pub const FDDI_FC_K_FORMAT_MANAGEMENT: u8 = 0x00;
pub const FDDI_FC_K_CONTROL_MASK: u8 = 0x0f;

/* Define FDDI Frame Control (FC) Byte specific values */
pub const FDDI_FC_K_VOID: u8 = 0x00;
pub const FDDI_FC_K_NON_RESTRICTED_TOKEN: u8 = 0x80;
pub const FDDI_FC_K_RESTRICTED_TOKEN: u8 = 0xC0;
pub const FDDI_FC_K_SMT_MIN: u8 = 0x41;
pub const FDDI_FC_K_SMT_MAX: u8 = 0x4F;
pub const FDDI_FC_K_MAC_MIN: u8 = 0xC1;
pub const FDDI_FC_K_MAC_MAX: u8 = 0xCF;
pub const FDDI_FC_K_ASYNC_LLC_MIN: u8 = 0x50;
pub const FDDI_FC_K_ASYNC_LLC_DEF: u8 = 0x54;
pub const FDDI_FC_K_ASYNC_LLC_MAX: u8 = 0x5F;
pub const FDDI_FC_K_SYNC_LLC_MIN: u8 = 0xD0;
pub const FDDI_FC_K_SYNC_LLC_MAX: u8 = 0xD7;
pub const FDDI_FC_K_IMPLEMENTOR_MIN: u8 = 0x60;
pub const FDDI_FC_K_IMPLEMENTOR_MAX: u8 = 0x6F;
pub const FDDI_FC_K_RESERVED_MIN: u8 = 0x70;
pub const FDDI_FC_K_RESERVED_MAX: u8 = 0x7F;

/* Define LLC and SNAP constants */
pub const FDDI_EXTENDED_SAP: u8 = 0xAA;
pub const FDDI_UI_CMD: u8 = 0x03;

/* Define 802.2 Type 1 header */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fddi_8022_1_hdr {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
}

/* Define 802.2 Type 2 header */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fddi_8022_2_hdr {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl_1: u8,
    pub ctrl_2: u8,
}

/* Define 802.2 SNAP header */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fddi_snap_hdr {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub oui: [u8; FDDI_K_OUI_LEN as usize],
    pub ethertype: u16,
}

/* Define FDDI LLC frame header */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union fddihdr_hdr {
    pub llc_8022_1: fddi_8022_1_hdr,
    pub llc_8022_2: fddi_8022_2_hdr,
    pub llc_snap: fddi_snap_hdr,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fddihdr {
    pub fc: u8,
    pub daddr: [u8; FDDI_K_ALEN as usize],
    pub saddr: [u8; FDDI_K_ALEN as usize],
    pub hdr: fddihdr_hdr,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
