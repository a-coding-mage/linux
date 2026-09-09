// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Michal Ludvig <michal@logix.cz>
 */

// Linux kernel headers supplied by the surrounding build provide these types,
// constants, and functions.

use core::ffi::{c_char, c_int, c_void};

pub const NFPROTO_UNSPEC: u8 = 0;
pub const NFPROTO_IPV4: u8 = 2;
pub const NFPROTO_IPV6: u8 = 10;
pub const PACKET_LOOPBACK: u8 = 5;
pub const PACKET_MULTICAST: u8 =  multicast_packet_type();
pub const PACKET_BROADCAST: u8 =  broadcast_packet_type();

const fn multicast_packet_type() -> u8 { 1 }
const fn broadcast_packet_type() -> u8 { 2 }

#[repr(C)]
pub struct sk_buff {
    pub pkt_type: u8,
}

#[repr(C)]
pub struct xt_pkttype_info {
    pub pkttype: u8,
    pub invert: bool,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
}

pub type XtMatchFn = unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool;

#[repr(C)]
pub struct xt_match {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u8,
    pub match_fn: Option<XtMatchFn>,
    pub matchsize: usize,
    pub me: *mut c_void,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    fn xt_family(par: *const xt_action_param) -> u8;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn ipv4_is_multicast(addr: u32) -> bool;
    fn xt_register_match(m: *mut xt_match) -> c_int;
    fn xt_unregister_match(m: *mut xt_match);
}

#[repr(C)]
struct iphdr {
    _private: [u8; 0],
}

// The IPv4 header's daddr field is supplied by the kernel definition.
unsafe fn ipv4_daddr(hdr: *const iphdr) -> u32 {
    *(hdr as *const u8).add(16) as u32
        | ((*hdr as *const iphdr as *const u8).add(17).read() as u32) << 8
        | ((*hdr as *const iphdr as *const u8).add(18).read() as u32) << 16
        | ((*hdr as *const iphdr as *const u8).add(19).read() as u32) << 24
}

static mut pkttype_mt_reg: xt_match = xt_match {
    name: b"pkttype\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_fn: Some(pkttype_mt),
    matchsize: core::mem::size_of::<xt_pkttype_info>(),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn pkttype_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const xt_pkttype_info;
    let packet_type: u8;

    if (*skb).pkt_type != PACKET_LOOPBACK {
        packet_type = (*skb).pkt_type;
    } else if xt_family(par) == NFPROTO_IPV4
        && ipv4_is_multicast(ipv4_daddr(ip_hdr(skb)))
    {
        packet_type = PACKET_MULTICAST;
    } else if xt_family(par) == NFPROTO_IPV6 {
        packet_type = PACKET_MULTICAST;
    } else {
        packet_type = PACKET_BROADCAST;
    }

    (packet_type == (*info).pkttype) ^ (*info).invert
}

unsafe extern "C" fn pkttype_mt_init() -> c_int {
    xt_register_match(&raw mut pkttype_mt_reg)
}

unsafe extern "C" fn pkttype_mt_exit() {
    xt_unregister_match(&raw mut pkttype_mt_reg);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
