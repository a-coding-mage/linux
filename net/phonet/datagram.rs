// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: datagram.c
 *
 * Datagram (ISI) Phonet sockets
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Authors: Sakari Ailus <sakari.ailus@nokia.com>
 *          Rémi Denis-Courmont
 */

use core::ffi::c_void;

// Required Linux kernel, socket, and Phonet declarations are supplied by other files.

/* associated socket ceases to exist */
unsafe extern "C" fn pn_sock_close(sk: *mut sock, _timeout: i64) {
    sk_common_release(sk);
}

unsafe extern "C" fn pn_ioctl(sk: *mut sock, cmd: i32, karg: *mut i32) -> i32 {
    let mut skb: *mut sk_buff;

    match cmd {
        SIOCINQ => {
            spin_lock_bh(&mut (*(*sk).sk_receive_queue).lock);
            skb = skb_peek((*sk).sk_receive_queue);
            *karg = if !skb.is_null() { (*skb).len as i32 } else { 0 };
            spin_unlock_bh(&mut (*(*sk).sk_receive_queue).lock);
            0
        }
        SIOCPNADDRESOURCE | SIOCPNDELRESOURCE => {
            let res: u32 = *karg as u32;
            if res >= 256 {
                return -EINVAL;
            }
            if cmd == SIOCPNADDRESOURCE {
                pn_sock_bind_res(sk, res)
            } else {
                pn_sock_unbind_res(sk, res)
            }
        }
        _ => -ENOIOCTLCMD,
    }
}

/* Destroy socket. All references are gone. */
unsafe extern "C" fn pn_destruct(sk: *mut sock) {
    skb_queue_purge((*sk).sk_receive_queue);
}

unsafe extern "C" fn pn_init(sk: *mut sock) -> i32 {
    (*sk).sk_destruct = Some(pn_destruct);
    0
}

unsafe extern "C" fn pn_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> i32 {
    let target = (*msg).msg_name as *mut sockaddr_pn;
    let mut skb: *mut sk_buff;
    let mut err: i32 = 0;

    if (*msg).msg_flags & !(MSG_DONTWAIT | MSG_EOR | MSG_NOSIGNAL | MSG_CMSG_COMPAT) != 0 {
        return -EOPNOTSUPP;
    }
    if target.is_null() {
        return -EDESTADDRREQ;
    }
    if (*msg).msg_namelen < core::mem::size_of::<sockaddr_pn>() as i32 {
        return -EINVAL;
    }
    if (*target).spn_family != AF_PHONET {
        return -EAFNOSUPPORT;
    }

    skb = sock_alloc_send_skb(
        sk,
        MAX_PHONET_HEADER + len,
        (*msg).msg_flags & MSG_DONTWAIT,
        &mut err,
    );
    if skb.is_null() {
        return err;
    }
    skb_reserve(skb, MAX_PHONET_HEADER);

    err = memcpy_from_msg(skb_put(skb, len) as *mut c_void, msg, len);
    if err < 0 {
        kfree_skb(skb);
        return err;
    }

    /*
     * Fill in the Phonet header and
     * finally pass the packet forwards.
     */
    err = pn_skb_send(sk, skb, target);

    /* If ok, return len. */
    if err >= 0 { len as i32 } else { err }
}

unsafe extern "C" fn pn_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 {
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut sa: sockaddr_pn = core::mem::zeroed();
    let mut rval: i32 = -EOPNOTSUPP;
    let mut copylen: usize;

    if flags & !(MSG_PEEK | MSG_TRUNC | MSG_DONTWAIT | MSG_NOSIGNAL | MSG_CMSG_COMPAT) != 0 {
        return rval;
    }

    skb = skb_recv_datagram(sk, flags, &mut rval);
    if skb.is_null() {
        return rval;
    }

    pn_skb_get_src_sockaddr(skb, &mut sa);

    copylen = (*skb).len as usize;
    if len < copylen {
        (*msg).msg_flags |= MSG_TRUNC;
        copylen = len;
    }

    rval = skb_copy_datagram_msg(skb, 0, msg, copylen);
    if rval != 0 {
        rval = -EFAULT;
        skb_free_datagram(sk, skb);
        return rval;
    }

    rval = if flags & MSG_TRUNC != 0 { (*skb).len as i32 } else { copylen as i32 };

    if !(*msg).msg_name.is_null() {
        memcpy(
            (*msg).msg_name as *mut c_void,
            &sa as *const sockaddr_pn as *const c_void,
            core::mem::size_of::<sockaddr_pn>(),
        );
        (*msg).msg_namelen = core::mem::size_of::<sockaddr_pn>() as i32;
    }

    skb_free_datagram(sk, skb);
    rval
}

/* Queue an skb for a sock. */
unsafe extern "C" fn pn_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let err = sock_queue_rcv_skb(sk, skb);

    if err < 0 {
        kfree_skb(skb);
    }
    if err != 0 { NET_RX_DROP } else { NET_RX_SUCCESS }
}

/* Module registration */
static mut PN_PROTO: proto = proto {
    close: Some(pn_sock_close),
    ioctl: Some(pn_ioctl),
    init: Some(pn_init),
    sendmsg: Some(pn_sendmsg),
    recvmsg: Some(pn_recvmsg),
    backlog_rcv: Some(pn_backlog_rcv),
    hash: Some(pn_sock_hash),
    unhash: Some(pn_sock_unhash),
    get_port: Some(pn_sock_get_port),
    obj_size: core::mem::size_of::<pn_sock>(),
    owner: THIS_MODULE,
    name: *b"PHONET\0",
};

static PN_DGRAM_PROTO: phonet_protocol = phonet_protocol {
    ops: &phonet_dgram_ops,
    prot: unsafe { &PN_PROTO },
    sock_type: SOCK_DGRAM,
};

pub unsafe extern "C" fn isi_register() -> i32 {
    phonet_proto_register(PN_PROTO_PHONET, &PN_DGRAM_PROTO)
}

pub unsafe extern "C" fn isi_unregister() {
    phonet_proto_unregister(PN_PROTO_PHONET, &PN_DGRAM_PROTO);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
