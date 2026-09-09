/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the UDP protocol.
 *
 * Version:	@(#)udp.h	1.0.2	04/28/93
 *
 * Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 */

// Dependency: __be16 and __sum16 are supplied by linux/types.h.

#[repr(C)]
pub struct udphdr {
	pub source: __be16,
	pub dest: __be16,
	pub len: __be16,
	pub check: __sum16,
}

/* UDP socket options */
pub const UDP_CORK: i32 = 1; /* Never send partially complete segments */
/* Deprecated, reserved for UDPLITE_SEND_CSCOV 10 */
/* Deprecated, reserved for UDPLITE_RECV_CSCOV 11 */
pub const UDP_ENCAP: i32 = 100; /* Set the socket to accept encapsulated packets */
pub const UDP_NO_CHECK6_TX: i32 = 101; /* Disable sending checksum for UDP6X */
pub const UDP_NO_CHECK6_RX: i32 = 102; /* Disable accepting checksum for UDP6 */
pub const UDP_SEGMENT: i32 = 103; /* Set GSO segmentation size */
pub const UDP_GRO: i32 = 104; /* This socket can receive UDP GRO packets */

/* UDP encapsulation types */
pub const UDP_ENCAP_ESPINUDP_NON_IKE: i32 = 1; /* unused  draft-ietf-ipsec-nat-t-ike-00/01 */
pub const UDP_ENCAP_ESPINUDP: i32 = 2; /* draft-ietf-ipsec-udp-encaps-06 */
pub const UDP_ENCAP_L2TPINUDP: i32 = 3; /* rfc2661 */
pub const UDP_ENCAP_GTP0: i32 = 4; /* GSM TS 09.60 */
pub const UDP_ENCAP_GTP1U: i32 = 5; /* 3GPP TS 29.060 */
pub const UDP_ENCAP_RXRPC: i32 = 6;
pub const TCP_ENCAP_ESPINTCP: i32 = 7; /* Yikes, this is really xfrm encap types. */
pub const UDP_ENCAP_OVPNINUDP: i32 = 8; /* OpenVPN traffic */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
