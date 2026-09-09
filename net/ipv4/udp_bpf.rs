// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Cloudflare Ltd https://cloudflare.com */

// Dependencies supplied by the surrounding kernel/Rust environment:
// linux/skmsg.h, net/sock.h, net/udp.h, net/inet_common.h, asm/ioctls.h

static mut udpv6_prot_saved: *mut proto = core::ptr::null_mut();

unsafe fn sk_udp_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 {
    #[cfg(CONFIG_IPV6)]
    {
        if (*sk).sk_family == AF_INET6 {
            return ((*udpv6_prot_saved).recvmsg)(sk, msg, len, flags);
        }
    }
    (udp_prot.recvmsg)(sk, msg, len, flags)
}

unsafe fn udp_sk_has_data(sk: *mut sock) -> bool {
    !skb_queue_empty(&(*udp_sk(sk)).reader_queue) ||
        !skb_queue_empty(&(*sk).sk_receive_queue)
}

unsafe fn psock_has_data(psock: *mut sk_psock) -> bool {
    !skb_queue_empty(&(*psock).ingress_skb) || sk_psock_queue_empty(psock) == false
}

unsafe fn udp_msg_has_data(sk: *mut sock, psock: *mut sk_psock) -> bool {
    udp_sk_has_data(sk) || psock_has_data(psock)
}

unsafe fn udp_msg_wait_data(sk: *mut sock, psock: *mut sk_psock, timeo: i64) -> i32 {
    // DEFINE_WAIT_FUNC(wait, woken_wake_function)
    let mut wait = wait_queue_entry::default();
    let mut ret: i32 = 0;

    if (*sk).sk_shutdown & RCV_SHUTDOWN != 0 {
        return 1;
    }

    if timeo == 0 {
        return ret;
    }

    add_wait_queue(sk_sleep(sk), &mut wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    ret = udp_msg_has_data(sk, psock) as i32;
    if ret == 0 {
        release_sock(sk);
        wait_woken(&mut wait, TASK_INTERRUPTIBLE, timeo);
        lock_sock(sk);
        ret = udp_msg_has_data(sk, psock) as i32;
    }
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), &mut wait);
    ret
}

unsafe fn udp_bpf_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 {
    let psock: *mut sk_psock;
    let mut copied: i32;
    let mut ret: i32;

    if flags & MSG_ERRQUEUE != 0 {
        return inet_recv_error(sk, msg, len);
    }

    if len == 0 {
        return 0;
    }

    psock = sk_psock_get(sk);
    if psock.is_null() {
        return sk_udp_recvmsg(sk, msg, len, flags);
    }

    if !psock_has_data(psock) {
        ret = sk_udp_recvmsg(sk, msg, len, flags);
        sk_psock_put(sk, psock);
        return ret;
    }

    lock_sock(sk);
    'msg_bytes_ready: loop {
        copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
        if copied != 0 {
            break;
        }

        let timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT != 0);
        let data = udp_msg_wait_data(sk, psock, timeo);
        if data != 0 {
            if psock_has_data(psock) {
                continue 'msg_bytes_ready;
            }
            release_sock(sk);
            ret = sk_udp_recvmsg(sk, msg, len, flags);
            sk_psock_put(sk, psock);
            return ret;
        }
        copied = -EAGAIN;
        break;
    }

    release_sock(sk);
    ret = copied;
    sk_psock_put(sk, psock);
    ret
}

enum {
    UDP_BPF_IPV4,
    UDP_BPF_IPV6,
    UDP_BPF_NUM_PROTS,
}

static mut udpv6_prot_lock: spinlock_t = spinlock_t::new();
static mut udp_bpf_prots: [proto; UDP_BPF_NUM_PROTS] = [proto::default(); UDP_BPF_NUM_PROTS];

unsafe fn udp_bpf_ioctl(sk: *mut sock, cmd: i32, karg: *mut i32) -> i32 {
    if cmd != SIOCINQ {
        return udp_ioctl(sk, cmd, karg);
    }
    // Since we don't hold a lock, sk_receive_queue may contain data. BPF might
    // only be processing this data at the moment. We only care about the data
    // in the ingress_msg here.
    *karg = sk_msg_first_len(sk);
    0
}

unsafe fn udp_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    *prot = *base;
    (*prot).close = Some(sock_map_close);
    (*prot).recvmsg = Some(udp_bpf_recvmsg);
    (*prot).sock_is_readable = Some(sk_msg_is_readable);
    (*prot).ioctl = Some(udp_bpf_ioctl);
}

unsafe fn udp_bpf_check_v6_needs_rebuild(ops: *const proto) {
    if ops != smp_load_acquire(&udpv6_prot_saved) {
        spin_lock_bh(&mut udpv6_prot_lock);
        if ops != udpv6_prot_saved {
            udp_bpf_rebuild_protos(&mut udp_bpf_prots[UDP_BPF_IPV6], ops);
            smp_store_release(&mut udpv6_prot_saved, ops as *mut proto);
        }
        spin_unlock_bh(&mut udpv6_prot_lock);
    }
}

unsafe fn udp_bpf_v4_build_proto() -> i32 {
    udp_bpf_rebuild_protos(&mut udp_bpf_prots[UDP_BPF_IPV4], &udp_prot);
    0
}

// late_initcall(udp_bpf_v4_build_proto)

#[no_mangle]
pub unsafe extern "C" fn udp_bpf_update_proto(
    sk: *mut sock,
    psock: *mut sk_psock,
    restore: bool,
) -> i32 {
    let family = if (*sk).sk_family == AF_INET {
        UDP_BPF_IPV4
    } else {
        UDP_BPF_IPV6
    };

    if restore {
        WRITE_ONCE((*sk).sk_write_space, (*psock).saved_write_space);
        sock_replace_proto(sk, (*psock).sk_proto);
        return 0;
    }

    if (*sk).sk_family == AF_INET6 {
        udp_bpf_check_v6_needs_rebuild((*psock).sk_proto);
    }

    sock_replace_proto(sk, &udp_bpf_prots[family]);
    0
}

// EXPORT_SYMBOL_GPL(udp_bpf_update_proto)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
