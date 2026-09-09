/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET  An implementation of the TCP/IP protocol suite for the LINUX
 *       operating system.  INET is implemented using the BSD Socket
 *       interface as the means of communication with the user level.
 *
 *       Definitions for the protocol dispatcher.
 *
 * Version:     @(#)protocol.h  1.0.2  05/07/93
 *
 * Author: Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 *       Changes:
 *               Alan Cox       : Added a name field and a frag handler
 *                                 field for later.
 *               Alan Cox       : Cleaned up, and sorted types.
 *               Pedro Roque    : inet6 protocols
 */

// C dependencies supplied by other translated headers.

/* This is one larger than the largest protocol value that can be
 * found in an ipv4 or ipv6 header.  Since in both cases the protocol
 * value is presented in a __u8, this is defined to be 256.
 */
pub const MAX_INET_PROTOS: usize = 256;

/* This is used to register protocols. */
#[repr(C)]
pub struct net_protocol {
    pub handler: Option<unsafe extern "C" fn(skb: *mut sk_buff) -> ::core::ffi::c_int>,

    /* This returns an error if we weren't able to handle the error. */
    pub err_handler: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        info: u32,
    ) -> ::core::ffi::c_int>,

    /* C bit-fields are represented by their underlying storage unit. */
    pub no_policy: u32,
    pub icmp_strict_tag_validation: u32,
    pub secret: u32,
}

/* Enabled when CONFIG_IPV6 is enabled in the C build. */
#[cfg(feature = "CONFIG_IPV6")]
#[repr(C)]
pub struct inet6_protocol {
    pub handler: Option<unsafe extern "C" fn(skb: *mut sk_buff) -> ::core::ffi::c_int>,

    /* This returns an error if we weren't able to handle the error. */
    pub err_handler: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        opt: *mut inet6_skb_parm,
        type_: u8,
        code: u8,
        offset: ::core::ffi::c_int,
        info: __be32,
    ) -> ::core::ffi::c_int>,

    pub flags: u32, /* INET6_PROTO_xxx */
    pub secret: u32,
}

#[cfg(feature = "CONFIG_IPV6")]
pub const INET6_PROTO_NOPOLICY: u32 = 0x1;
#[cfg(feature = "CONFIG_IPV6")]
pub const INET6_PROTO_FINAL: u32 = 0x2;

#[repr(C)]
pub struct net_offload {
    pub callbacks: offload_callbacks,
    pub flags: u32, /* Flags used by IPv6 for now */
    pub secret: u32,
}

/* This should be set for any extension header which is compatible with GSO. */
pub const INET6_PROTO_GSO_EXTHDR: u32 = 0x1;

/* This is used to register socket interfaces for IP protocols. */
#[repr(C)]
pub struct inet_protosw {
    pub list: list_head,
    /* These two fields form the lookup key. */
    pub type_: u16,       /* This is the 2nd argument to socket(2). */
    pub protocol: u16,   /* This is the L4 protocol number. */
    pub prot: *mut proto,
    pub ops: *const proto_ops,
    pub flags: u8,        /* See INET_PROTOSW_* below. */
}

pub const INET_PROTOSW_REUSE: u8 = 0x01;     /* Are ports automatically reusable? */
pub const INET_PROTOSW_PERMANENT: u8 = 0x02; /* Permanent protocols are unremovable. */
pub const INET_PROTOSW_ICSK: u8 = 0x04;      /* Is this an inet_connection_sock? */

extern "C" {
    pub static mut inet_protos: [*mut net_protocol; MAX_INET_PROTOS];
    pub static inet_offloads: [*const net_offload; MAX_INET_PROTOS];
    pub static inet6_offloads: [*const net_offload; MAX_INET_PROTOS];

    #[cfg(feature = "CONFIG_IPV6")]
    pub static mut inet6_protos: [*mut inet6_protocol; MAX_INET_PROTOS];

    pub fn inet_add_protocol(prot: *const net_protocol, num: u8) -> ::core::ffi::c_int;
    pub fn inet_del_protocol(prot: *const net_protocol, num: u8) -> ::core::ffi::c_int;
    pub fn inet_add_offload(prot: *const net_offload, num: u8) -> ::core::ffi::c_int;
    pub fn inet_del_offload(prot: *const net_offload, num: u8) -> ::core::ffi::c_int;
    pub fn inet_register_protosw(p: *mut inet_protosw);
    pub fn inet_unregister_protosw(p: *mut inet_protosw);

    #[cfg(feature = "CONFIG_IPV6")]
    pub fn inet6_add_protocol(prot: *const inet6_protocol, num: u8) -> ::core::ffi::c_int;
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn inet6_del_protocol(prot: *const inet6_protocol, num: u8) -> ::core::ffi::c_int;
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn inet6_register_protosw(p: *mut inet_protosw);
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn inet6_unregister_protosw(p: *mut inet_protosw);

    pub fn inet6_add_offload(prot: *const net_offload, num: u8) -> ::core::ffi::c_int;
    pub fn inet6_del_offload(prot: *const net_offload, num: u8) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
