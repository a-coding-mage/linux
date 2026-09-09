/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for the Ethernet IEEE 802.3 interface.
 *
 * Version:	@(#)if_ether.h	1.0.1a	02/08/94
 *
 * Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *		Donald Becker, <becker@super.org>
 *		Alan Cox, <alan@lxorguk.ukuu.org.uk>
 *		Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
 */

// Dependencies supplied by linux/skbuff.h and uapi/linux/if_ether.h.
extern "C" {
    pub fn skb_mac_header(skb: *const sk_buff) -> *mut u8;
    pub fn skb_inner_mac_header(skb: *const sk_buff) -> *mut u8;
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
}

#[repr(C)]
pub struct ethhdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub static ETH_ALEN: i32;
}

/* XX:XX:XX:XX:XX:XX */
pub const MAC_ADDR_STR_LEN: i32 = 3 * 6 - 1;

pub unsafe fn eth_hdr(skb: *const sk_buff) -> *mut ethhdr {
    skb_mac_header(skb) as *mut ethhdr
}

/* Prefer this version in TX path, instead of
 * skb_reset_mac_header() + eth_hdr()
 */
pub unsafe fn skb_eth_hdr(skb: *const sk_buff) -> *mut ethhdr {
    (*skb).data as *mut ethhdr
}

pub unsafe fn inner_eth_hdr(skb: *const sk_buff) -> *mut ethhdr {
    skb_inner_mac_header(skb) as *mut ethhdr
}

extern "C" {
    pub fn eth_header_parse(
        skb: *const sk_buff,
        dev: *const net_device,
        haddr: *mut u8,
    ) -> i32;

    pub fn sysfs_format_mac(buf: *mut i8, addr: *const u8, len: i32) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
