/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/**
 * file phonet.h
 *
 * Phonet sockets kernel interface
 *
 * Copyright (C) 2008 Nokia Corporation. All rights reserved.
 */

/* Automatic protocol selection */
pub const PN_PROTO_TRANSPORT: i32 = 0;
/* Phonet datagram socket */
pub const PN_PROTO_PHONET: i32 = 1;
/* Phonet pipe */
pub const PN_PROTO_PIPE: i32 = 2;
pub const PHONET_NPROTO: i32 = 3;

/* Socket options for SOL_PNPIPE level */
pub const PNPIPE_ENCAP: i32 = 1;
pub const PNPIPE_IFINDEX: i32 = 2;
pub const PNPIPE_HANDLE: i32 = 3;
pub const PNPIPE_INITSTATE: i32 = 4;

pub const PNADDR_ANY: i32 = 0;
pub const PNADDR_BROADCAST: i32 = 0xfc;
pub const PNPORT_RESOURCE_ROUTING: i32 = 0;

/* Values for PNPIPE_ENCAP option */
pub const PNPIPE_ENCAP_NONE: i32 = 0;
pub const PNPIPE_ENCAP_IP: i32 = 1;

/* ioctl requests (SIOCPROTOPRIVATE is supplied externally). */
pub const SIOCPNGETOBJECT: i32 = SIOCPROTOPRIVATE + 0;
pub const SIOCPNENABLEPIPE: i32 = SIOCPROTOPRIVATE + 13;
pub const SIOCPNADDRESOURCE: i32 = SIOCPROTOPRIVATE + 14;
pub const SIOCPNDELRESOURCE: i32 = SIOCPROTOPRIVATE + 15;

/* Phonet protocol header */
#[repr(C, packed)]
pub struct phonethdr {
    pub pn_rdev: __u8,
    pub pn_sdev: __u8,
    pub pn_res: __u8,
    pub pn_length: __be16,
    pub pn_robj: __u8,
    pub pn_sobj: __u8,
}

/* Common Phonet payload header */
#[repr(C)]
pub struct phonetmsg_base {
    pub pn_submsg_id: __u8,
    pub pn_data: [__u8; 5],
}

#[repr(C)]
pub struct phonetmsg_ext {
    pub pn_e_res_id: __u16,
    pub pn_e_submsg_id: __u8,
    pub pn_e_data: [__u8; 3],
}

#[repr(C)]
pub union phonetmsg_pn_msg_u {
    pub base: phonetmsg_base,
    pub ext: phonetmsg_ext,
}

#[repr(C)]
pub struct phonetmsg {
    pub pn_trans_id: __u8, /* transaction ID */
    pub pn_msg_id: __u8, /* message type */
    pub pn_msg_u: phonetmsg_pn_msg_u,
}

pub const PN_COMMON_MESSAGE: i32 = 0xf0;
pub const PN_COMMGR: i32 = 0x10;
pub const PN_PREFIX: i32 = 0xe0;

/* C field aliases: pn_submsg_id, pn_e_submsg_id, pn_e_res_id, pn_data, pn_e_data. */

/* data for unreachable errors */
pub const PN_COMM_SERVICE_NOT_IDENTIFIED_RESP: i32 = 0x01;
pub const PN_COMM_ISA_ENTITY_NOT_REACHABLE_RESP: i32 = 0x14;
/* C field aliases: pn_orig_msg_id, pn_status, pn_e_orig_msg_id, pn_e_status. */

/* Phonet socket address structure */
#[repr(C, packed)]
pub struct sockaddr_pn {
    pub spn_family: __kernel_sa_family_t,
    pub spn_obj: __u8,
    pub spn_dev: __u8,
    pub spn_resource: __u8,
    pub spn_zero: [__u8; core::mem::size_of::<sockaddr>() - core::mem::size_of::<__kernel_sa_family_t>() - 3],
}

/* Well known address */
pub const PN_DEV_PC: i32 = 0x10;

#[inline]
pub const fn pn_object(addr: __u8, port: __u16) -> __u16 {
    ((addr as __u16) << 8) | (port & 0x3ff)
}

#[inline]
pub const fn pn_obj(handle: __u16) -> __u8 { handle as __u8 }

#[inline]
pub const fn pn_dev(handle: __u16) -> __u8 { (handle >> 8) as __u8 }

#[inline]
pub const fn pn_port(handle: __u16) -> __u16 { handle & 0x3ff }

#[inline]
pub const fn pn_addr(handle: __u16) -> __u8 { ((handle >> 8) as __u8) & 0xfc }

#[inline]
pub unsafe fn pn_sockaddr_set_addr(spn: *mut sockaddr_pn, addr: __u8) {
    (*spn).spn_dev &= 0x03;
    (*spn).spn_dev |= addr & 0xfc;
}

#[inline]
pub unsafe fn pn_sockaddr_set_port(spn: *mut sockaddr_pn, port: __u16) {
    (*spn).spn_dev &= 0xfc;
    (*spn).spn_dev |= ((port >> 8) as __u8) & 0x03;
    (*spn).spn_obj = port as __u8;
}

#[inline]
pub unsafe fn pn_sockaddr_set_object(spn: *mut sockaddr_pn, handle: __u16) {
    (*spn).spn_dev = pn_dev(handle);
    (*spn).spn_obj = pn_obj(handle);
}

#[inline]
pub unsafe fn pn_sockaddr_set_resource(spn: *mut sockaddr_pn, resource: __u8) {
    (*spn).spn_resource = resource;
}

#[inline]
pub unsafe fn pn_sockaddr_get_addr(spn: *const sockaddr_pn) -> __u8 { (*spn).spn_dev & 0xfc }

#[inline]
pub unsafe fn pn_sockaddr_get_port(spn: *const sockaddr_pn) -> __u16 {
    (((*spn).spn_dev as __u16 & 0x03) << 8) | (*spn).spn_obj as __u16
}

#[inline]
pub unsafe fn pn_sockaddr_get_object(spn: *const sockaddr_pn) -> __u16 {
    pn_object((*spn).spn_dev, (*spn).spn_obj as __u16)
}

#[inline]
pub unsafe fn pn_sockaddr_get_resource(spn: *const sockaddr_pn) -> __u8 { (*spn).spn_resource }

/* Phonet device ioctl requests */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
