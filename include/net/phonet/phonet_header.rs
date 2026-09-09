/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * File: af_phonet.h
 *
 * Phonet sockets kernel definitions
 *
 * Copyright (C) 2008 Nokia Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/phonet.h, linux/skbuff.h, and net/sock.h.

/*
 * The lower layers may not require more space, ever. Make sure it's
 * enough.
 */
pub const MAX_PHONET_HEADER: usize = 8 + MAX_HEADER;

/*
 * Every Phonet* socket has this structure first in its
 * protocol-specific structure under name c.
 */
#[repr(C)]
pub struct pn_sock {
    pub sk: sock,
    pub sobject: u16,
    pub dobject: u16,
    pub resource: u8,
}

pub unsafe fn pn_sk(sk: *mut sock) -> *mut pn_sock {
    sk as *mut pn_sock
}

extern "C" {
    pub static phonet_dgram_ops: proto_ops;

    pub fn pn_sock_init();
    pub fn pn_find_sock_by_sa(net: *mut net, sa: *const sockaddr_pn) -> *mut sock;
    pub fn pn_deliver_sock_broadcast(net: *mut net, skb: *mut sk_buff);
    pub fn phonet_get_local_port_range(min: *mut i32, max: *mut i32);
    pub fn pn_sock_hash(sk: *mut sock) -> i32;
    pub fn pn_sock_unhash(sk: *mut sock);
    pub fn pn_sock_get_port(sk: *mut sock, sport: u16) -> i32;

    pub fn pn_find_sock_by_res(net: *mut net, res: u8) -> *mut sock;
    pub fn pn_sock_bind_res(sock: *mut sock, res: u8) -> i32;
    pub fn pn_sock_unbind_res(sk: *mut sock, res: u8) -> i32;
    pub fn pn_sock_unbind_all_res(sk: *mut sock);

    pub fn pn_skb_send(
        sk: *mut sock,
        skb: *mut sk_buff,
        target: *const sockaddr_pn,
    ) -> i32;

    pub fn phonet_proto_register(protocol: u32, pp: *const phonet_protocol) -> i32;
    pub fn phonet_proto_unregister(protocol: u32, pp: *const phonet_protocol);

    pub fn phonet_sysctl_init() -> i32;
    pub fn phonet_sysctl_exit();
    pub fn isi_register() -> i32;
    pub fn isi_unregister();
}

pub unsafe fn pn_hdr(skb: *mut sk_buff) -> *mut phonethdr {
    skb_network_header(skb) as *mut phonethdr
}

pub unsafe fn pn_msg(skb: *mut sk_buff) -> *mut phonetmsg {
    skb_transport_header(skb) as *mut phonetmsg
}

/*
 * Get the other party's sockaddr from received skb. The skb begins
 * with a Phonet header.
 */
pub unsafe fn pn_skb_get_src_sockaddr(skb: *mut sk_buff, sa: *mut sockaddr_pn) {
    let ph = pn_hdr(skb);
    let obj: u16 = pn_object((*ph).pn_sdev, (*ph).pn_sobj);

    (*sa).spn_family = AF_PHONET;
    pn_sockaddr_set_object(sa, obj);
    pn_sockaddr_set_resource(sa, (*ph).pn_res);
    memset((*sa).spn_zero.as_mut_ptr(), 0, core::mem::size_of_val(&(*sa).spn_zero));
}

pub unsafe fn pn_skb_get_dst_sockaddr(skb: *mut sk_buff, sa: *mut sockaddr_pn) {
    let ph = pn_hdr(skb);
    let obj: u16 = pn_object((*ph).pn_rdev, (*ph).pn_robj);

    (*sa).spn_family = AF_PHONET;
    pn_sockaddr_set_object(sa, obj);
    pn_sockaddr_set_resource(sa, (*ph).pn_res);
    memset((*sa).spn_zero.as_mut_ptr(), 0, core::mem::size_of_val(&(*sa).spn_zero));
}

/* Protocols in Phonet protocol family. */
#[repr(C)]
pub struct phonet_protocol {
    pub ops: *const proto_ops,
    pub prot: *mut proto,
    pub sock_type: i32,
}

pub unsafe fn sk_is_phonet(sk: *mut sock) -> bool {
    (*sk).sk_family == PF_PHONET
}

pub unsafe fn phonet_sk_ioctl(sk: *mut sock, cmd: u32, arg: *mut core::ffi::c_void) -> i32 {
    let mut karg: i32 = 0;

    match cmd {
        SIOCPNADDRESOURCE | SIOCPNDELRESOURCE => {
            if get_user(&mut karg, arg as *mut i32) != 0 {
                return -EFAULT;
            }

            return ((*(*sk).sk_prot).ioctl)(sk, cmd, &mut karg as *mut i32 as *mut core::ffi::c_void);
        }
        _ => {}
    }
    /* A positive return value means that the ioctl was not processed */
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
