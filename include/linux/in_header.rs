/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions of the Internet Protocol.
 *
 * Version:	@(#)in.h	1.0.1	04/21/93
 *
 * Authors:	Original taken from the GNU Project <netinet/in.h> file.
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

// Dependencies supplied by the surrounding Linux headers:
// linux/errno.h and uapi/linux/in.h.

#[inline]
fn proto_ports_offset(proto: i32) -> i32 {
    match proto {
        IPPROTO_TCP | IPPROTO_UDP | IPPROTO_DCCP
        | IPPROTO_ESP /* SPI */
        | IPPROTO_SCTP | IPPROTO_UDPLITE => 0,
        IPPROTO_AH /* SPI */ => 4,
        _ => -(EINVAL as i32),
    }
}

#[inline]
fn ipv4_is_loopback(addr: __be32) -> bool {
    (addr & htonl(0xff000000)) == htonl(0x7f000000)
}

#[inline]
fn ipv4_is_multicast(addr: __be32) -> bool {
    (addr & htonl(0xf0000000)) == htonl(0xe0000000)
}

#[inline]
fn ipv4_is_local_multicast(addr: __be32) -> bool {
    (addr & htonl(0xffffff00)) == htonl(0xe0000000)
}

#[inline]
fn ipv4_is_lbcast(addr: __be32) -> bool {
    /* limited broadcast */
    addr == htonl(INADDR_BROADCAST)
}

#[inline]
fn ipv4_is_all_snoopers(addr: __be32) -> bool {
    addr == htonl(INADDR_ALLSNOOPERS_GROUP)
}

#[inline]
fn ipv4_is_zeronet(addr: __be32) -> bool {
    addr == 0
}

/* Special-Use IPv4 Addresses (RFC3330) */

#[inline]
fn ipv4_is_private_10(addr: __be32) -> bool {
    (addr & htonl(0xff000000)) == htonl(0x0a000000)
}

#[inline]
fn ipv4_is_private_172(addr: __be32) -> bool {
    (addr & htonl(0xfff00000)) == htonl(0xac100000)
}

#[inline]
fn ipv4_is_private_192(addr: __be32) -> bool {
    (addr & htonl(0xffff0000)) == htonl(0xc0a80000)
}

#[inline]
fn ipv4_is_linklocal_169(addr: __be32) -> bool {
    (addr & htonl(0xffff0000)) == htonl(0xa9fe0000)
}

#[inline]
fn ipv4_is_anycast_6to4(addr: __be32) -> bool {
    (addr & htonl(0xffffff00)) == htonl(0xc0586300)
}

#[inline]
fn ipv4_is_test_192(addr: __be32) -> bool {
    (addr & htonl(0xffffff00)) == htonl(0xc0000200)
}

#[inline]
fn ipv4_is_test_198(addr: __be32) -> bool {
    (addr & htonl(0xfffe0000)) == htonl(0xc6120000)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
