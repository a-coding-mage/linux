/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET\tAn implementation of the TCP/IP protocol suite for the LINUX
 *\t\toperating system.  INET is implemented using the  BSD Socket
 *\t\tinterface as the means of communication with the user level.
 *
 *\t\tDefinitions for the IP protocol.
 *
 * Version:\t@(#)ip.h\t1.0.2\t04/28/93
 *
 * Authors:\tFred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

/* Dependencies supplied by linux/skbuff.h and uapi/linux/ip.h. */

extern "C" {
    fn skb_network_header(skb: *const sk_buff) -> *mut core::ffi::c_void;
    fn skb_inner_network_header(skb: *const sk_buff) -> *mut core::ffi::c_void;
    fn skb_transport_header(skb: *const sk_buff) -> *mut core::ffi::c_void;
    fn skb_network_header_len(skb: *const sk_buff) -> u32;
    fn skb_is_gso(skb: *const sk_buff) -> bool;
    fn skb_is_gso_tcp(skb: *const sk_buff) -> bool;
    fn skb_network_offset(skb: *const sk_buff) -> u32;
    fn ntohs(value: u16) -> u16;
    fn htons(value: u16) -> u16;
}

pub unsafe fn ip_hdr(skb: *const sk_buff) -> *mut iphdr {
    skb_network_header(skb) as *mut iphdr
}

pub unsafe fn inner_ip_hdr(skb: *const sk_buff) -> *mut iphdr {
    skb_inner_network_header(skb) as *mut iphdr
}

pub unsafe fn ipip_hdr(skb: *const sk_buff) -> *mut iphdr {
    skb_transport_header(skb) as *mut iphdr
}

pub unsafe fn ip_transport_len(skb: *const sk_buff) -> u32 {
    ntohs((*ip_hdr(skb)).tot_len) as u32 - skb_network_header_len(skb)
}

pub unsafe fn iph_totlen(skb: *const sk_buff, iph: *const iphdr) -> u32 {
    let len: u32 = ntohs((*iph).tot_len) as u32;

    if len != 0 || !skb_is_gso(skb) || !skb_is_gso_tcp(skb) {
        len
    } else {
        (*skb).len - skb_network_offset(skb)
    }
}

pub unsafe fn skb_ip_totlen(skb: *const sk_buff) -> u32 {
    iph_totlen(skb, ip_hdr(skb))
}

/* IPv4 datagram length is stored into 16bit field (tot_len) */
pub const IP_MAX_MTU: u32 = 0xFFFF;

pub unsafe fn iph_set_totlen(iph: *mut iphdr, len: u32) {
    (*iph).tot_len = if len <= IP_MAX_MTU {
        htons(len as u16)
    } else {
        0
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
