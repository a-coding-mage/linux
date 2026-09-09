// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Cong Wang <cong.wang@bytedance.com> */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

macro_rules! unix_sk_has_data {
    ($sk:expr, $psock:expr) => {
        unsafe {
            !skb_queue_empty(&(*$sk).sk_receive_queue)
                || !skb_queue_empty(&(*$psock).ingress_skb)
                || !list_empty(&(*$psock).ingress_msg)
        }
    };
}

unsafe fn unix_msg_wait_data(sk: *mut sock, psock: *mut sk_psock, timeo: libc::c_long) -> libc::c_int {
    let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function);
    let u = unix_sk(sk);
    let mut ret: libc::c_int = 0;

    if (*sk).sk_shutdown & RCV_SHUTDOWN != 0 {
        return 1;
    }

    if timeo == 0 {
        return ret;
    }

    add_wait_queue(sk_sleep(sk), &mut wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    if !unix_sk_has_data!(sk, psock) {
        mutex_unlock(&mut (*u).iolock);
        wait_woken(&mut wait, TASK_INTERRUPTIBLE, timeo);
        mutex_lock(&mut (*u).iolock);
        ret = unix_sk_has_data!(sk, psock) as libc::c_int;
    }
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), &mut wait);
    ret
}

unsafe fn __unix_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: libc::c_int) -> libc::c_int {
    if (*sk).sk_type == SOCK_DGRAM {
        __unix_dgram_recvmsg(sk, msg, len, flags)
    } else {
        __unix_stream_recvmsg(sk, msg, len, flags)
    }
}

unsafe fn unix_bpf_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: libc::c_int) -> libc::c_int {
    let u = unix_sk(sk);
    let mut psock: *mut sk_psock;
    let mut copied: libc::c_int;

    if flags & MSG_OOB != 0 {
        return -EOPNOTSUPP;
    }
    if len == 0 {
        return 0;
    }

    psock = sk_psock_get(sk);
    if psock.is_null() {
        return __unix_recvmsg(sk, msg, len, flags);
    }

    mutex_lock(&mut (*u).iolock);
    if !skb_queue_empty(&(*sk).sk_receive_queue) && sk_psock_queue_empty(psock) {
        mutex_unlock(&mut (*u).iolock);
        sk_psock_put(sk, psock);
        return __unix_recvmsg(sk, msg, len, flags);
    }

    'msg_bytes_ready: loop {
        copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
        if copied != 0 {
            break;
        }
        let timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);
        let data = unix_msg_wait_data(sk, psock, timeo);
        if data != 0 {
            if !sk_psock_queue_empty(psock) {
                continue 'msg_bytes_ready;
            }
            mutex_unlock(&mut (*u).iolock);
            sk_psock_put(sk, psock);
            return __unix_recvmsg(sk, msg, len, flags);
        }
        copied = -EAGAIN;
        break;
    }
    mutex_unlock(&mut (*u).iolock);
    sk_psock_put(sk, psock);
    copied
}

static mut unix_dgram_prot_saved: *mut proto = core::ptr::null_mut();
static mut unix_dgram_prot_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut unix_dgram_bpf_prot: proto = proto::default();

static mut unix_stream_prot_saved: *mut proto = core::ptr::null_mut();
static mut unix_stream_prot_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut unix_stream_bpf_prot: proto = proto::default();

unsafe fn unix_dgram_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    *prot = *base;
    (*prot).close = Some(sock_map_close);
    (*prot).recvmsg = Some(unix_bpf_recvmsg);
    (*prot).sock_is_readable = Some(sk_msg_is_readable);
}

unsafe fn unix_stream_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    *prot = *base;
    (*prot).close = Some(sock_map_close);
    (*prot).recvmsg = Some(unix_bpf_recvmsg);
    (*prot).sock_is_readable = Some(sk_msg_is_readable);
    (*prot).unhash = Some(sock_map_unhash);
}

unsafe fn unix_dgram_bpf_check_needs_rebuild(ops: *mut proto) {
    if ops != smp_load_acquire(&unix_dgram_prot_saved) {
        spin_lock_bh(&mut unix_dgram_prot_lock);
        if ops != unix_dgram_prot_saved {
            unix_dgram_bpf_rebuild_protos(&mut unix_dgram_bpf_prot, ops);
            smp_store_release(&mut unix_dgram_prot_saved, ops);
        }
        spin_unlock_bh(&mut unix_dgram_prot_lock);
    }
}

unsafe fn unix_stream_bpf_check_needs_rebuild(ops: *mut proto) {
    if ops != smp_load_acquire(&unix_stream_prot_saved) {
        spin_lock_bh(&mut unix_stream_prot_lock);
        if ops != unix_stream_prot_saved {
            unix_stream_bpf_rebuild_protos(&mut unix_stream_bpf_prot, ops);
            smp_store_release(&mut unix_stream_prot_saved, ops);
        }
        spin_unlock_bh(&mut unix_stream_prot_lock);
    }
}

pub unsafe fn unix_dgram_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> libc::c_int {
    if (*sk).sk_type != SOCK_DGRAM { return -EOPNOTSUPP; }
    if restore {
        (*sk).sk_write_space = (*psock).saved_write_space;
        sock_replace_proto(sk, (*psock).sk_proto);
        return 0;
    }
    unix_dgram_bpf_check_needs_rebuild((*psock).sk_proto);
    sock_replace_proto(sk, &mut unix_dgram_bpf_prot);
    0
}

pub unsafe fn unix_stream_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> libc::c_int {
    let sk_pair: *mut sock;
    /* Restore does not decrement the sk_pair reference yet because we must
     * keep the a reference to the socket until after an RCU grace period
     * and any pending sends have completed.
     */
    if restore {
        (*sk).sk_write_space = (*psock).saved_write_space;
        sock_replace_proto(sk, (*psock).sk_proto);
        return 0;
    }
    /* psock_update_sk_prot can be called multiple times if psock is
     * added to multiple maps and/or slots in the same map. There is
     * also an edge case where replacing a psock with itself can trigger
     * an extra psock_update_sk_prot during the insert process. So it
     * must be safe to do multiple calls. Here we need to ensure we don't
     * increment the refcnt through sock_hold many times. There will only
     * be a single matching destroy operation.
     */
    if (*psock).sk_pair.is_null() {
        sk_pair = unix_peer(sk);
        if sk_pair.is_null() { return -EINVAL; }
        sock_hold(sk_pair);
        (*psock).sk_pair = sk_pair;
    }
    unix_stream_bpf_check_needs_rebuild((*psock).sk_proto);
    sock_replace_proto(sk, &mut unix_stream_bpf_prot);
    0
}

pub unsafe fn unix_bpf_build_proto() {
    unix_dgram_bpf_rebuild_protos(&mut unix_dgram_bpf_prot, &unix_dgram_proto);
    unix_stream_bpf_rebuild_protos(&mut unix_stream_bpf_prot, &unix_stream_proto);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
