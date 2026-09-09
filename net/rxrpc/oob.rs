// SPDX-License-Identifier: GPL-2.0-or-later
/* Out of band message handling (e.g. challenge-response)
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel includes and build-time definitions are supplied by the surrounding
// translation unit.

#[repr(u8)]
enum rxrpc_oob_command {
    RXRPC_OOB_CMD_UNSET,
    RXRPC_OOB_CMD_RESPOND,
}

#[repr(C)]
struct rxrpc_oob_params {
    oob_id: u64,
    abort_code: i32,
    command: rxrpc_oob_command,
    have_oob_id: bool,
}

pub unsafe fn rxrpc_notify_socket_oob(call: *mut rxrpc_call, skb: *mut sk_buff) -> bool {
    let sp = rxrpc_skb(skb);
    let mut queued = false;
    rcu_read_lock();
    let rx = rcu_dereference((*call).socket);
    if !rx.is_null() {
        let sk = &mut (*rx).sk;
        spin_lock_irq(&mut (*rx).recvmsg_lock);
        if (*sk).sk_state < RXRPC_CLOSE {
            (*skb).skb_mstamp_ns = (*rx).oob_id_counter;
            (*rx).oob_id_counter = (*rx).oob_id_counter.wrapping_add(1);
            rxrpc_get_skb(skb, rxrpc_skb_get_post_oob);
            skb_queue_tail(&mut (*rx).recvmsg_oobq, skb);
            queued = true;
            trace_rxrpc_notify_socket((*call).debug_id, (*sp).hdr.serial);
            if !(*rx).app_ops.is_null() {
                ((*(*rx).app_ops).notify_oob)(sk, skb);
            }
        }
        spin_unlock_irq(&mut (*rx).recvmsg_lock);
        if queued && (*rx).app_ops.is_null() && !sock_flag(sk, SOCK_DEAD) {
            ((*sk).sk_data_ready)(sk);
        }
    }
    rcu_read_unlock();
    queued
}

unsafe fn rxrpc_find_pending_oob(rx: *mut rxrpc_sock, oob_id: u64) -> *mut sk_buff {
    let mut p = (*rx).pending_oobq.rb_node;
    while !p.is_null() {
        let skb = rb_entry(p);
        if oob_id < (*skb).skb_mstamp_ns {
            p = (*p).rb_left;
        } else if oob_id > (*skb).skb_mstamp_ns {
            p = (*p).rb_right;
        } else {
            return skb;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn rxrpc_add_pending_oob(rx: *mut rxrpc_sock, skb: *mut sk_buff) {
    let mut pp = &mut (*rx).pending_oobq.rb_node as *mut *mut rb_node;
    let mut p: *mut rb_node = core::ptr::null_mut();
    while !(*pp).is_null() {
        p = *pp;
        pp = &mut (**pp).rb_right;
    }
    rb_link_node(&mut (*skb).rbnode, p, pp);
    rb_insert_color(&mut (*skb).rbnode, &mut (*rx).pending_oobq);
}

unsafe fn rxrpc_sendmsg_oob_cmsg(msg: *mut msghdr, p: *mut rxrpc_oob_params) -> i32 {
    if (*msg).msg_controllen == 0 { return -EINVAL; }
    let mut cmsg = cmsg_firsthdr(msg);
    while !cmsg.is_null() {
        if !cmsg_ok(msg, cmsg) { return -EINVAL; }
        let len = (*cmsg).cmsg_len - core::mem::size_of::<cmsghdr>();
        debug_cmsg((*cmsg).cmsg_level, (*cmsg).cmsg_type, len);
        if (*cmsg).cmsg_level == SOL_RXRPC {
            match (*cmsg).cmsg_type {
                RXRPC_OOB_ID => {
                    if len != core::mem::size_of::<u64>() || (*p).have_oob_id { return -EINVAL; }
                    core::ptr::copy_nonoverlapping(cmsg_data(cmsg), &mut (*p).oob_id as *mut _ as *mut u8, len);
                    (*p).have_oob_id = true;
                }
                RXRPC_RESPOND => {
                    if !matches!((*p).command, rxrpc_oob_command::RXRPC_OOB_CMD_UNSET) { return -EINVAL; }
                    (*p).command = rxrpc_oob_command::RXRPC_OOB_CMD_RESPOND;
                }
                RXRPC_ABORT => {
                    if len != core::mem::size_of::<i32>() || (*p).abort_code != 0 { return -EINVAL; }
                    core::ptr::copy_nonoverlapping(cmsg_data(cmsg), &mut (*p).abort_code as *mut _ as *mut u8, len);
                    if (*p).abort_code == 0 { return -EINVAL; }
                }
                RXRPC_RESP_RXGK_APPDATA => {
                    if !matches!((*p).command, rxrpc_oob_command::RXRPC_OOB_CMD_RESPOND) { return -EINVAL; }
                }
                _ => return -EINVAL,
            }
        }
        cmsg = cmsg_nxthdr(msg, cmsg);
    }
    match (*p).command {
        rxrpc_oob_command::RXRPC_OOB_CMD_RESPOND if (*p).have_oob_id => 0,
        rxrpc_oob_command::RXRPC_OOB_CMD_RESPOND => -EBADSLT,
        _ => -EINVAL,
    }
}

unsafe fn rxrpc_respond_to_oob(rx: *mut rxrpc_sock, p: *mut rxrpc_oob_params, msg: *mut msghdr) -> i32 {
    let skb = rxrpc_find_pending_oob(rx, (*p).oob_id);
    if !skb.is_null() { rb_erase(&mut (*skb).rbnode, &mut (*rx).pending_oobq); }
    release_sock(&mut (*rx).sk);
    if skb.is_null() { return -EBADSLT; }
    let sp = rxrpc_skb(skb);
    let mut ret;
    match (*p).command {
        rxrpc_oob_command::RXRPC_OOB_CMD_RESPOND => {
            ret = -EPROTO;
            if (*skb).mark == RXRPC_OOB_CHALLENGE {
                let conn = (*sp).chall.conn;
                ret = -EOPNOTSUPP;
                if !(*(*conn).security).sendmsg_respond_to_challenge.is_null() {
                    if (*p).abort_code != 0 {
                        rxrpc_abort_conn(conn, core::ptr::null_mut(), (*p).abort_code as u32, -ECONNABORTED, rxrpc_abort_response_sendmsg);
                        ret = 0;
                    } else { ret = ((*(*conn).security).sendmsg_respond_to_challenge)(skb, msg); }
                }
            }
        }
        _ => ret = -EINVAL,
    }
    if (*skb).mark == RXRPC_OOB_CHALLENGE { rxrpc_put_connection((*sp).chall.conn, rxrpc_conn_put_oob); }
    rxrpc_free_skb(skb, rxrpc_skb_put_oob);
    ret
}

pub unsafe fn rxrpc_sendmsg_oob(rx: *mut rxrpc_sock, msg: *mut msghdr, _len: usize) -> i32 {
    let mut p = rxrpc_oob_params { oob_id: 0, abort_code: 0, command: rxrpc_oob_command::RXRPC_OOB_CMD_UNSET, have_oob_id: false };
    let ret = rxrpc_sendmsg_oob_cmsg(msg, &mut p);
    if ret < 0 { release_sock(&mut (*rx).sk); return ret; }
    if p.have_oob_id { return rxrpc_respond_to_oob(rx, &mut p, msg); }
    release_sock(&mut (*rx).sk);
    -EINVAL
}

pub unsafe fn rxrpc_kernel_query_oob(oob: *mut sk_buff, peer: *mut *mut rxrpc_peer, peer_appdata: *mut usize) -> rxrpc_oob_type {
    let sp = rxrpc_skb(oob);
    let ty = (*oob).mark;
    if ty == RXRPC_OOB_CHALLENGE { *peer = (*(*sp).chall.conn).peer; *peer_appdata = (*(*(*sp).chall.conn).peer).app_data; }
    else { warn_on_once(true); *peer = core::ptr::null_mut(); *peer_appdata = 0; }
    ty
}

pub unsafe fn rxrpc_kernel_dequeue_oob(sock: *mut socket, ty: *mut rxrpc_oob_type) -> *mut sk_buff {
    let rx = rxrpc_sk((*sock).sk);
    let oob = skb_dequeue(&mut (*rx).recvmsg_oobq);
    if !oob.is_null() { *ty = (*oob).mark; }
    oob
}

pub unsafe fn rxrpc_kernel_free_oob(oob: *mut sk_buff) {
    let sp = rxrpc_skb(oob);
    if (*oob).mark == RXRPC_OOB_CHALLENGE { rxrpc_put_connection((*sp).chall.conn, rxrpc_conn_put_oob); }
    rxrpc_free_skb(oob, rxrpc_skb_put_purge_oob);
}

pub unsafe fn rxrpc_kernel_query_challenge(challenge: *mut sk_buff, peer: *mut *mut rxrpc_peer, appdata: *mut usize, service_id: *mut u16, security_index: *mut u8) {
    let sp = rxrpc_skb(challenge); let conn = (*sp).chall.conn;
    *peer = (*conn).peer; *appdata = (*(*conn).peer).app_data; *service_id = (*sp).hdr.serviceId; *security_index = (*sp).hdr.securityIndex;
}

pub unsafe fn rxrpc_kernel_reject_challenge(challenge: *mut sk_buff, abort_code: u32, error: i32, why: rxrpc_abort_reason) -> i32 {
    let sp = rxrpc_skb(challenge); rxrpc_abort_conn((*sp).chall.conn, core::ptr::null_mut(), abort_code, error, why); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
