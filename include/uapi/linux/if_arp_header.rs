/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET        An implementation of the TCP/IP protocol suite for the LINUX
 *             operating system.  INET is implemented using the BSD Socket
 *             interface as the means of communication with the user level.
 *
 *             Global definitions for the ARP (RFC 826) protocol.
 *
 * Version:     @(#)if_arp.h  1.0.1  04/16/93
 *
 * Authors:    Original taken from Berkeley UNIX 4.3, (c) UCB 1986-1988
 *             Portions taken from the KA9Q/NOS (v2.00m PA0GRI) source.
 *             Ross Biro
 *             Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *             Florian La Roche,
 *             Jonathan Layes <layes@loran.com>
 *             Arnaldo Carvalho de Melo <acme@conectiva.com.br> ARPHRD_HWX25
 *
 *             This program is free software; you can redistribute it and/or
 *             modify it under the terms of the GNU General Public License
 *             as published by the Free Software Foundation; either version
 *             2 of the License, or (at your option) any later version.
 */

// Dependency supplied by the corresponding Linux netdevice/socket bindings:
// `sockaddr`, `__be16`, `IFNAMSIZ`, and `ETH_ALEN`.

/* ARP protocol HARDWARE identifiers. */
pub const ARPHRD_NETROM: u32 = 0;
pub const ARPHRD_ETHER: u32 = 1;
pub const ARPHRD_EETHER: u32 = 2;
pub const ARPHRD_AX25: u32 = 3;
pub const ARPHRD_PRONET: u32 = 4;
pub const ARPHRD_CHAOS: u32 = 5;
pub const ARPHRD_IEEE802: u32 = 6;
pub const ARPHRD_ARCNET: u32 = 7;
pub const ARPHRD_APPLETLK: u32 = 8;
pub const ARPHRD_DLCI: u32 = 15;
pub const ARPHRD_ATM: u32 = 19;
pub const ARPHRD_METRICOM: u32 = 23;
pub const ARPHRD_IEEE1394: u32 = 24;
pub const ARPHRD_EUI64: u32 = 27;
pub const ARPHRD_INFINIBAND: u32 = 32;

/* Dummy types for non ARP hardware */
pub const ARPHRD_SLIP: u32 = 256;
pub const ARPHRD_CSLIP: u32 = 257;
pub const ARPHRD_SLIP6: u32 = 258;
pub const ARPHRD_CSLIP6: u32 = 259;
pub const ARPHRD_RSRVD: u32 = 260;
pub const ARPHRD_ADAPT: u32 = 264;
pub const ARPHRD_ROSE: u32 = 270;
pub const ARPHRD_X25: u32 = 271;
pub const ARPHRD_HWX25: u32 = 272;
pub const ARPHRD_CAN: u32 = 280;
pub const ARPHRD_MCTP: u32 = 290;
pub const ARPHRD_PPP: u32 = 512;
pub const ARPHRD_CISCO: u32 = 513;
pub const ARPHRD_HDLC: u32 = ARPHRD_CISCO;
pub const ARPHRD_LAPB: u32 = 516;
pub const ARPHRD_DDCMP: u32 = 517;
pub const ARPHRD_RAWHDLC: u32 = 518;
pub const ARPHRD_RAWIP: u32 = 519;

pub const ARPHRD_TUNNEL: u32 = 768;
pub const ARPHRD_TUNNEL6: u32 = 769;
pub const ARPHRD_FRAD: u32 = 770;
pub const ARPHRD_SKIP: u32 = 771;
pub const ARPHRD_LOOPBACK: u32 = 772;
pub const ARPHRD_LOCALTLK: u32 = 773;
pub const ARPHRD_FDDI: u32 = 774;
pub const ARPHRD_BIF: u32 = 775;
pub const ARPHRD_SIT: u32 = 776;
pub const ARPHRD_IPDDP: u32 = 777;
pub const ARPHRD_IPGRE: u32 = 778;
pub const ARPHRD_PIMREG: u32 = 779;
pub const ARPHRD_HIPPI: u32 = 780;
pub const ARPHRD_ASH: u32 = 781;
pub const ARPHRD_ECONET: u32 = 782;
pub const ARPHRD_IRDA: u32 = 783;
/* ARP works differently on different FC media .. so */
pub const ARPHRD_FCPP: u32 = 784;
pub const ARPHRD_FCAL: u32 = 785;
pub const ARPHRD_FCPL: u32 = 786;
pub const ARPHRD_FCFABRIC: u32 = 787;
pub const ARPHRD_IEEE802_TR: u32 = 800;
pub const ARPHRD_IEEE80211: u32 = 801;
pub const ARPHRD_IEEE80211_PRISM: u32 = 802;
pub const ARPHRD_IEEE80211_RADIOTAP: u32 = 803;
pub const ARPHRD_IEEE802154: u32 = 804;
pub const ARPHRD_IEEE802154_MONITOR: u32 = 805;
pub const ARPHRD_PHONET: u32 = 820;
pub const ARPHRD_PHONET_PIPE: u32 = 821;
pub const ARPHRD_CAIF: u32 = 822;
pub const ARPHRD_IP6GRE: u32 = 823;
pub const ARPHRD_NETLINK: u32 = 824;
pub const ARPHRD_6LOWPAN: u32 = 825;
pub const ARPHRD_VSOCKMON: u32 = 826;
pub const ARPHRD_VOID: u32 = 0xFFFF;
pub const ARPHRD_NONE: u32 = 0xFFFE;

/* ARP protocol opcodes. */
pub const ARPOP_REQUEST: u32 = 1;
pub const ARPOP_REPLY: u32 = 2;
pub const ARPOP_RREQUEST: u32 = 3;
pub const ARPOP_RREPLY: u32 = 4;
pub const ARPOP_InREQUEST: u32 = 8;
pub const ARPOP_InREPLY: u32 = 9;
pub const ARPOP_NAK: u32 = 10;

/* ARP ioctl request. */
#[repr(C)]
pub struct arpreq {
    pub arp_pa: sockaddr,
    pub arp_ha: sockaddr,
    pub arp_flags: i32,
    pub arp_netmask: sockaddr,
    pub arp_dev: [::core::ffi::c_char; IFNAMSIZ],
}

#[repr(C)]
pub struct arpreq_old {
    pub arp_pa: sockaddr,
    pub arp_ha: sockaddr,
    pub arp_flags: i32,
    pub arp_netmask: sockaddr,
}

/* ARP Flag values. */
pub const ATF_COM: u32 = 0x02;
pub const ATF_PERM: u32 = 0x04;
pub const ATF_PUBL: u32 = 0x08;
pub const ATF_USETRAILERS: u32 = 0x10;
pub const ATF_NETMASK: u32 = 0x20;
pub const ATF_DONTPUB: u32 = 0x40;

/*
 * This structure defines an ethernet arp header.
 */
#[repr(C)]
pub struct arphdr {
    pub ar_hrd: __be16,
    pub ar_pro: __be16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: __be16,
}

// The C source contains an inactive #if 0 variable-sized Ethernet payload
// layout here; it is intentionally not part of the active structure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
