// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Bobby Eshleman <bobby.eshleman@bytedance.com>
 *
 * Based off of net/unix/unix_bpf.c
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this file.

unsafe fn vsock_sk_has_data(__sk: *mut sock, __psock: *mut sk_psock) -> bool {
    !skb_queue_empty(unsafe { &mut (*__sk).sk_receive_queue })
        || !skb_queue_empty(unsafe { &mut (*__psock).ingress_skb })
        || !list_empty(unsafe { &mut (*__psock).ingress_msg })
}

static mut vsock_prot_saved: *mut proto = core::ptr::null_mut();
static mut vsock_prot_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut vsock_bpf_prot: proto = unsafe { core::mem::zeroed() };

unsafe fn vsock_has_data(sk: *mut sock, psock: *mut sk_psock) -> bool {
    let vsk: *mut vsock_sock = vsock_sk(sk);
    let ret: i64 = vsock_connectible_has_data(vsk);

    if ret > 0 {
        return true;
    }

    vsock_sk_has_data(sk, psock)
}

unsafe fn vsock_msg_wait_data(sk: *mut sock, psock: *mut sk_psock, timeo: c_long) -> bool {
    let mut ret: bool;
    let mut wait: wait_queue_entry = WakerWaitFunc::new();

    if (*sk).sk_shutdown & RCV_SHUTDOWN != 0 {
        return true;
    }

    if timeo == 0 {
        return false;
    }

    add_wait_queue(sk_sleep(sk), &mut wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    ret = vsock_has_data(sk, psock);
    if !ret {
        wait_woken(&mut wait, TASK_INTERRUPTIBLE, timeo);
        ret = vsock_has_data(sk, psock);
    }
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), &mut wait);
    ret
}

unsafe fn __vsock_recvmsg(
    sk: *mut sock,
    msg: *mut msghdr,
    len: usize,
    flags: c_int,
) -> c_int {
    let sock = (*sk).sk_socket;
    let err: c_int;

    if (*sk).sk_type == SOCK_STREAM || (*sk).sk_type == SOCK_SEQPACKET {
        err = __vsock_connectible_recvmsg(sock, msg, len, flags);
    } else if (*sk).sk_type == SOCK_DGRAM {
        err = __vsock_dgram_recvmsg(sock, msg, len, flags);
    } else {
        err = -EPROTOTYPE;
    }

    err
}

unsafe fn vsock_bpf_recvmsg(
    sk: *mut sock,
    msg: *mut msghdr,
    len: usize,
    flags: c_int,
) -> c_int {
    let psock: *mut sk_psock = sk_psock_get(sk);
    let vsk: *mut vsock_sock;
    let mut copied: c_int;

    if psock.is_null() {
        return __vsock_recvmsg(sk, msg, len, flags);
    }

    lock_sock(sk);
    vsk = vsock_sk(sk);

    if WARN_ON_ONCE((*vsk).transport.is_null()) {
        copied = -ENODEV;
        goto_out!(out);
    }

    if vsock_has_data(sk, psock) && sk_psock_queue_empty(psock) {
        release_sock(sk);
        sk_psock_put(sk, psock);
        return __vsock_recvmsg(sk, msg, len, flags);
    }

    copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
    while copied == 0 {
        let timeo: c_long = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);

        if !vsock_msg_wait_data(sk, psock, timeo) {
            copied = -EAGAIN;
            break;
        }

        if sk_psock_queue_empty(psock) {
            release_sock(sk);
            sk_psock_put(sk, psock);
            return __vsock_recvmsg(sk, msg, len, flags);
        }

        copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
    }

    out:
    release_sock(sk);
    sk_psock_put(sk, psock);

    copied
}

unsafe fn vsock_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    *prot = *base;
    (*prot).close = Some(sock_map_close);
    (*prot).recvmsg = Some(vsock_bpf_recvmsg);
    (*prot).sock_is_readable = Some(sk_msg_is_readable);
}

unsafe fn vsock_bpf_check_needs_rebuild(ops: *mut proto) {
    // Paired with the smp_store_release() below.
    if unlikely(ops != smp_load_acquire(&vsock_prot_saved)) {
        spin_lock_bh(&mut vsock_prot_lock);
        if likely(ops != vsock_prot_saved) {
            vsock_bpf_rebuild_protos(&mut vsock_bpf_prot, ops);
            // Make sure proto function pointers are updated before publishing
            // the pointer to the struct.
            smp_store_release(&mut vsock_prot_saved, ops);
        }
        spin_unlock_bh(&mut vsock_prot_lock);
    }
}

unsafe fn vsock_bpf_update_proto(
    sk: *mut sock,
    psock: *mut sk_psock,
    restore: bool,
) -> c_int {
    let vsk: *mut vsock_sock;

    if restore {
        (*sk).sk_write_space = (*psock).saved_write_space;
        sock_replace_proto(sk, (*psock).sk_proto);
        return 0;
    }

    vsk = vsock_sk(sk);
    if (*vsk).transport.is_null() {
        return -ENODEV;
    }

    if (*(*vsk).transport).read_skb.is_none() {
        return -EOPNOTSUPP;
    }

    vsock_bpf_check_needs_rebuild((*psock).sk_proto);
    sock_replace_proto(sk, &mut vsock_bpf_prot);
    0
}

pub unsafe fn vsock_bpf_build_proto() {
    vsock_bpf_rebuild_protos(&mut vsock_bpf_prot, &vsock_proto);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
