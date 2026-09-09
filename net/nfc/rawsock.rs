// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 *
 * Authors:
 *    Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
 *    Lauro Ramos Venancio <lauro.venancio@openbossa.org>
 */

// Dependencies supplied by the surrounding kernel/NFC implementation.

static mut RAW_SK_LIST: nfc_sock_list = nfc_sock_list { lock: __RW_LOCK_UNLOCKED!() };

unsafe fn nfc_sock_link(l: *mut nfc_sock_list, sk: *mut sock) {
    write_lock(&mut (*l).lock);
    sk_add_node(sk, &mut (*l).head);
    write_unlock(&mut (*l).lock);
}

unsafe fn nfc_sock_unlink(l: *mut nfc_sock_list, sk: *mut sock) {
    write_lock(&mut (*l).lock);
    sk_del_node_init(sk);
    write_unlock(&mut (*l).lock);
}

unsafe fn rawsock_write_queue_purge(sk: *mut sock) {
    pr_debug!("sk={:p}\n", sk);
    spin_lock_bh(&mut (*sk).sk_write_queue.lock);
    __skb_queue_purge(&mut (*sk).sk_write_queue);
    nfc_rawsock(sk).tx_work_scheduled = false;
    spin_unlock_bh(&mut (*sk).sk_write_queue.lock);
}

unsafe fn rawsock_report_error(sk: *mut sock, err: i32) {
    pr_debug!("sk={:p} err={}\n", sk, err);
    (*sk).sk_shutdown = SHUTDOWN_MASK;
    (*sk).sk_err = -err;
    sk_error_report(sk);
    rawsock_write_queue_purge(sk);
}

unsafe fn rawsock_release(sock: *mut socket) -> i32 {
    let sk = (*sock).sk;
    pr_debug!("sock={:p} sk={:p}\n", sock, sk);
    if sk.is_null() { return 0; }
    if (*sock).type_ == SOCK_RAW { nfc_sock_unlink(&mut RAW_SK_LIST, sk); }
    if (*sk).sk_state == TCP_ESTABLISHED {
        (*sk).sk_shutdown |= SEND_SHUTDOWN;
        cancel_work_sync(&mut nfc_rawsock(sk).tx_work);
        rawsock_write_queue_purge(sk);
    }
    sock_orphan(sk);
    sock_put(sk);
    0
}

unsafe fn rawsock_connect(sock: *mut socket, _addr: *mut sockaddr_unsized, len: i32, flags: i32) -> i32 {
    let sk = (*sock).sk;
    let addr = _addr as *mut sockaddr_nfc;
    let mut rc = 0;
    pr_debug!("sock={:p} sk={:p} flags={}\n", sock, sk, flags);
    if addr.is_null() || len < core::mem::size_of::<sockaddr_nfc>() as i32 || (*addr).sa_family != AF_NFC { return -EINVAL; }
    pr_debug!("addr dev_idx={} target_idx={} protocol={}\n", (*addr).dev_idx, (*addr).target_idx, (*addr).nfc_protocol);
    lock_sock(sk);
    if (*sock).state == SS_CONNECTED { rc = -EISCONN; goto_error(sk, rc); return rc; }
    let dev = nfc_get_device((*addr).dev_idx);
    if dev.is_null() { rc = -ENODEV; release_sock(sk); return rc; }
    if (*addr).target_idx > (*dev).target_next_idx - 1 || (*addr).target_idx < (*dev).target_next_idx - (*dev).n_targets {
        rc = -EINVAL; nfc_put_device(dev); release_sock(sk); return rc;
    }
    rc = nfc_activate_target(dev, (*addr).target_idx, (*addr).nfc_protocol);
    if rc != 0 { nfc_put_device(dev); release_sock(sk); return rc; }
    nfc_rawsock(sk).dev = dev;
    nfc_rawsock(sk).target_idx = (*addr).target_idx;
    (*sock).state = SS_CONNECTED;
    (*sk).sk_state = TCP_ESTABLISHED;
    ((*sk).sk_state_change)(sk);
    release_sock(sk);
    0
}

unsafe fn rawsock_add_header(skb: *mut sk_buff) -> i32 {
    *(skb_push(skb, NFC_HEADER_SIZE) as *mut u8) = 0;
    0
}

unsafe fn rawsock_data_exchange_complete(context: *mut core::ffi::c_void, skb: *mut sk_buff, mut err: i32) {
    let sk = context as *mut sock;
    BUG_ON(in_hardirq());
    pr_debug!("sk={:p} err={}\n", sk, err);
    if err != 0 { rawsock_report_error(sk, err); sock_put(sk); return; }
    err = rawsock_add_header(skb);
    if err != 0 { kfree_skb(skb); rawsock_report_error(sk, err); sock_put(sk); return; }
    err = sock_queue_rcv_skb(sk, skb);
    if err != 0 { kfree_skb(skb); rawsock_report_error(sk, err); sock_put(sk); return; }
    spin_lock_bh(&mut (*sk).sk_write_queue.lock);
    if !skb_queue_empty(&(*sk).sk_write_queue) { schedule_work(&mut nfc_rawsock(sk).tx_work); }
    else { nfc_rawsock(sk).tx_work_scheduled = false; }
    spin_unlock_bh(&mut (*sk).sk_write_queue.lock);
    sock_put(sk);
}

unsafe fn rawsock_tx_work(work: *mut work_struct) {
    let sk = to_rawsock_sk(work);
    let dev = nfc_rawsock(sk).dev;
    let target_idx = nfc_rawsock(sk).target_idx;
    pr_debug!("sk={:p} target_idx={}\n", sk, target_idx);
    if (*sk).sk_shutdown & SEND_SHUTDOWN != 0 { rawsock_write_queue_purge(sk); return; }
    let skb = skb_dequeue(&mut (*sk).sk_write_queue);
    kcov_remote_start_common(skb_get_kcov_handle(skb));
    sock_hold(sk);
    let rc = nfc_data_exchange(dev, target_idx, skb, rawsock_data_exchange_complete, sk as *mut _);
    if rc != 0 { rawsock_report_error(sk, rc); sock_put(sk); }
    kcov_remote_stop();
}

unsafe fn rawsock_sendmsg(sock: *mut socket, msg: *mut msghdr, len: usize) -> isize {
    let sk = (*sock).sk; let dev = nfc_rawsock(sk).dev; let mut rc;
    pr_debug!("sock={:p} sk={:p} len={}\n", sock, sk, len);
    if (*msg).msg_namelen != 0 { return -EOPNOTSUPP as isize; }
    if (*sock).state != SS_CONNECTED { return -ENOTCONN as isize; }
    let skb = nfc_alloc_send_skb(dev, sk, (*msg).msg_flags, len, &mut rc);
    if skb.is_null() { return rc as isize; }
    rc = memcpy_from_msg(skb_put(skb, len), msg, len);
    if rc < 0 { kfree_skb(skb); return rc as isize; }
    spin_lock_bh(&mut (*sk).sk_write_queue.lock);
    __skb_queue_tail(&mut (*sk).sk_write_queue, skb);
    if !nfc_rawsock(sk).tx_work_scheduled { schedule_work(&mut nfc_rawsock(sk).tx_work); nfc_rawsock(sk).tx_work_scheduled = true; }
    spin_unlock_bh(&mut (*sk).sk_write_queue.lock);
    len as isize
}

unsafe fn rawsock_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: i32) -> isize {
    let sk = (*sock).sk; let mut rc = 0; pr_debug!("sock={:p} sk={:p} len={} flags={}\n", sock, sk, len, flags);
    let skb = skb_recv_datagram(sk, flags, &mut rc); if skb.is_null() { return rc as isize; }
    let mut copied = (*skb).len; if len < copied { (*msg).msg_flags |= MSG_TRUNC; copied = len; }
    rc = skb_copy_datagram_msg(skb, 0, msg, copied); skb_free_datagram(sk, skb);
    if rc != 0 { rc as isize } else { copied as isize }
}

// The following operation tables and protocol registrations preserve the C ABI layout;
// their field types and supplied callbacks are defined by the surrounding kernel bindings.
static rawsock_ops: proto_ops = proto_ops { family: PF_NFC, owner: THIS_MODULE, release: rawsock_release, bind: sock_no_bind, connect: rawsock_connect, socketpair: sock_no_socketpair, accept: sock_no_accept, getname: sock_no_getname, poll: datagram_poll, ioctl: sock_no_ioctl, listen: sock_no_listen, shutdown: sock_no_shutdown, sendmsg: rawsock_sendmsg, recvmsg: rawsock_recvmsg, mmap: sock_no_mmap };
static rawsock_raw_ops: proto_ops = proto_ops { family: PF_NFC, owner: THIS_MODULE, release: rawsock_release, bind: sock_no_bind, connect: sock_no_connect, socketpair: sock_no_socketpair, accept: sock_no_accept, getname: sock_no_getname, poll: datagram_poll, ioctl: sock_no_ioctl, listen: sock_no_listen, shutdown: sock_no_shutdown, sendmsg: sock_no_sendmsg, recvmsg: rawsock_recvmsg, mmap: sock_no_mmap };

unsafe fn rawsock_destruct(sk: *mut sock) {
    pr_debug!("sk={:p}\n", sk);
    if (*sk).sk_state == TCP_ESTABLISHED { nfc_deactivate_target(nfc_rawsock(sk).dev, nfc_rawsock(sk).target_idx, NFC_TARGET_MODE_IDLE); nfc_put_device(nfc_rawsock(sk).dev); }
    skb_queue_purge(&mut (*sk).sk_receive_queue);
    if !sock_flag(sk, SOCK_DEAD) { pr_err!("Freeing alive NFC raw socket {:p}\n", sk); }
}

unsafe fn rawsock_create(net: *mut net, sock: *mut socket, nfc_proto: *const nfc_protocol, kern: i32) -> i32 {
    pr_debug!("sock={:p}\n", sock);
    if (*sock).type_ != SOCK_SEQPACKET && (*sock).type_ != SOCK_RAW { return -ESOCKTNOSUPPORT; }
    if (*sock).type_ == SOCK_RAW { if !ns_capable((*net).user_ns, CAP_NET_RAW) { return -EPERM; } (*sock).ops = &rawsock_raw_ops; } else { (*sock).ops = &rawsock_ops; }
    let sk = sk_alloc(net, PF_NFC, GFP_ATOMIC, (*nfc_proto).proto, kern); if sk.is_null() { return -ENOMEM; }
    sock_init_data(sock, sk); (*sk).sk_protocol = (*nfc_proto).id; (*sk).sk_destruct = Some(rawsock_destruct); (*sock).state = SS_UNCONNECTED;
    if (*sock).type_ == SOCK_RAW { nfc_sock_link(&mut RAW_SK_LIST, sk); } else { INIT_WORK!(&mut nfc_rawsock(sk).tx_work, rawsock_tx_work); nfc_rawsock(sk).tx_work_scheduled = false; }
    0
}

pub unsafe fn nfc_send_to_raw_sock(dev: *mut nfc_dev, skb: *mut sk_buff, payload_type: u8, direction: u8) {
    let mut skb_copy: *mut sk_buff = core::ptr::null_mut(); let mut nskb; let mut data;
    read_lock(&RAW_SK_LIST.lock);
    sk_for_each!(sk, &RAW_SK_LIST.head, { if skb_copy.is_null() { skb_copy = __pskb_copy_fclone(skb, NFC_RAW_HEADER_SIZE, GFP_ATOMIC, true); if skb_copy.is_null() { continue; } data = skb_push(skb_copy, NFC_RAW_HEADER_SIZE); *data.add(0) = if !dev.is_null() { (*dev).idx } else { 0xFF }; *data.add(1) = direction & 0x01; *data.add(1) |= payload_type << 1; } nskb = skb_clone(skb_copy, GFP_ATOMIC); if !nskb.is_null() && sock_queue_rcv_skb(sk, nskb) != 0 { kfree_skb(nskb); } });
    read_unlock(&RAW_SK_LIST.lock); kfree_skb(skb_copy);
}

static mut rawsock_proto: proto = proto { name: "NFC_RAW", owner: THIS_MODULE, obj_size: core::mem::size_of::<nfc_rawsock>() };
static rawsock_nfc_proto: nfc_protocol = nfc_protocol { id: NFC_SOCKPROTO_RAW, proto: &mut rawsock_proto, owner: THIS_MODULE, create: rawsock_create };

pub unsafe fn rawsock_init() -> i32 { nfc_proto_register(&rawsock_nfc_proto) }
pub unsafe fn rawsock_exit() { nfc_proto_unregister(&rawsock_nfc_proto); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
