/* SPDX-License-Identifier: GPL-2.0-or-later */
/***************************************************************************
 * Linux PPP over X - Generic PPP transport layer sockets
 * Linux PPP over Ethernet (PPPoE) Socket Implementation (RFC 2516)
 *
 * This file supplies definitions required by the PPP over Ethernet driver
 * (pppox.c).  All version information wrt this file is located in pppox.c
 */

// Dependencies supplied by the Linux networking headers are intentionally
// left as external names.

#[inline]
pub unsafe fn pppoe_hdr(skb: *const sk_buff) -> *mut pppoe_hdr {
    skb_network_header(skb) as *mut pppoe_hdr
}

#[repr(C)]
pub struct pppoe_opt {
    pub dev: *mut net_device,       // device associated with socket
    pub ifindex: ::core::ffi::c_int, // ifindex of device associated with socket
    pub pa: pppoe_addr,             // what this socket is bound to
    pub padt_work: work_struct,     // Work item for handling PADT
}

#[repr(C)]
pub struct pptp_opt {
    pub src_addr: pptp_addr,
    pub dst_addr: pptp_addr,
    pub ack_sent: u32,
    pub ack_recv: u32,
    pub seq_sent: u32,
    pub seq_recv: u32,
    pub ppp_flags: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pppox_sock {
    // struct sock must be the first member of pppox_sock
    pub sk: sock,
    pub chan: ppp_channel,
    pub next: *mut pppox_sock, // for hash table
    pub proto: pppox_sock_proto,
    pub num: __be16,
}

#[repr(C)]
pub union pppox_sock_proto {
    pub pppoe: pppoe_opt,
    pub pptp: pptp_opt,
}

impl pppox_sock {
    #[inline]
    pub unsafe fn pppoe_dev(&mut self) -> *mut net_device {
        self.proto.pppoe.dev
    }

    #[inline]
    pub unsafe fn pppoe_ifindex(&mut self) -> ::core::ffi::c_int {
        self.proto.pppoe.ifindex
    }

    #[inline]
    pub unsafe fn pppoe_pa(&mut self) -> *mut pppoe_addr {
        &mut self.proto.pppoe.pa
    }
}

#[inline]
pub unsafe fn pppox_sk(sk: *mut sock) -> *mut pppox_sock {
    container_of(sk, pppox_sock, sk)
}

pub struct module;

#[repr(C)]
pub struct pppox_proto {
    pub create: Option<unsafe extern "C" fn(net: *mut net, sock: *mut socket, kern: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub ioctl: Option<unsafe extern "C" fn(sock: *mut socket, cmd: u32, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub owner: *mut module,
}

unsafe extern "C" {
    pub fn register_pppox_proto(proto_num: ::core::ffi::c_int, pp: *const pppox_proto) -> ::core::ffi::c_int;
    pub fn unregister_pppox_proto(proto_num: ::core::ffi::c_int);
    pub fn pppox_unbind_sock(sk: *mut sock); // delete ppp-channel binding
    pub fn pppox_ioctl(sock: *mut socket, cmd: u32, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pppox_compat_ioctl(sock: *mut socket, cmd: u32, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

/* PPPoX socket states */
pub const PPPOX_NONE: i32 = 0;       /* initial state */
pub const PPPOX_CONNECTED: i32 = 1; /* connection established ==TCP_ESTABLISHED */
pub const PPPOX_BOUND: i32 = 2;      /* bound to ppp device */
pub const PPPOX_DEAD: i32 = 16;      /* dead, useless, please clean me up! */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
