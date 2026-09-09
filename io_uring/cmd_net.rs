// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding kernel headers and source files.

unsafe fn io_uring_cmd_get_sock_ioctl(sock: *mut socket, op: c_int) -> c_int {
    let sk = (*sock).sk;
    let prot = READ_ONCE((*sk).sk_prot);
    let mut arg: c_int = 0;

    if prot.is_null() || (*prot).ioctl.is_none() {
        return -EOPNOTSUPP;
    }

    let ret = ((*prot).ioctl.unwrap())(sk, op, &mut arg);
    if ret != 0 {
        return ret;
    }
    arg
}

unsafe fn io_uring_cmd_getsockopt(
    sock: *mut socket,
    cmd: *mut io_uring_cmd,
    issue_flags: c_uint,
) -> c_int {
    let sqe = (*cmd).sqe;
    let compat = (issue_flags & IO_URING_F_COMPAT) != 0;
    let level = READ_ONCE((*sqe).level);
    if level != SOL_SOCKET {
        return -EOPNOTSUPP;
    }

    let optval = u64_to_user_ptr(READ_ONCE((*sqe).optval));
    let optname = READ_ONCE((*sqe).optname);
    let mut optlen = READ_ONCE((*sqe).optlen);

    let err = do_sock_getsockopt(
        sock,
        compat,
        level,
        optname,
        USER_SOCKPTR(optval),
        KERNEL_SOCKPTR(&mut optlen),
    );
    if err != 0 {
        return err;
    }

    // On success, return optlen
    optlen
}

unsafe fn io_uring_cmd_setsockopt(
    sock: *mut socket,
    cmd: *mut io_uring_cmd,
    issue_flags: c_uint,
) -> c_int {
    let sqe = (*cmd).sqe;
    let compat = (issue_flags & IO_URING_F_COMPAT) != 0;
    let optval = u64_to_user_ptr(READ_ONCE((*sqe).optval));
    let optname = READ_ONCE((*sqe).optname);
    let optlen = READ_ONCE((*sqe).optlen);
    let level = READ_ONCE((*sqe).level);
    let optval_s = USER_SOCKPTR(optval);

    do_sock_setsockopt(sock, compat, level, optname, optval_s, optlen)
}

unsafe fn io_process_timestamp_skb(
    cmd: *mut io_uring_cmd,
    sk: *mut sock,
    skb: *mut sk_buff,
    issue_flags: c_uint,
) -> bool {
    let serr = SKB_EXT_ERR(skb);
    let mut cqe: [io_uring_cqe; 2] = core::mem::zeroed();
    let mut ts: timespec64 = core::mem::zeroed();

    // BUILD_BUG_ON(sizeof(struct io_uring_cqe) != sizeof(struct io_timespec));
    let ret = skb_get_tx_timestamp(skb, sk, &mut ts);
    if ret < 0 {
        return false;
    }

    let tskey = (*serr).ee.ee_data;
    let tstype = (*serr).ee.ee_info;

    cqe[0].user_data = 0;
    cqe[0].res = tskey;
    cqe[0].flags = IORING_CQE_F_MORE | ctx_cqe32_flags((*cmd_to_io_kiocb(cmd)).ctx);
    cqe[0].flags |= tstype << IORING_TIMESTAMP_TYPE_SHIFT;
    if ret == SOF_TIMESTAMPING_TX_HARDWARE {
        cqe[0].flags |= IORING_CQE_F_TSTAMP_HW;
    }

    let iots = &mut *(&mut cqe[1] as *mut io_uring_cqe as *mut io_timespec);
    iots.tv_sec = ts.tv_sec;
    iots.tv_nsec = ts.tv_nsec;
    io_uring_cmd_post_mshot_cqe32(cmd, issue_flags, cqe.as_mut_ptr())
}

unsafe fn io_uring_cmd_timestamp(
    sock: *mut socket,
    cmd: *mut io_uring_cmd,
    issue_flags: c_uint,
) -> c_int {
    let sk = (*sock).sk;
    let q = &mut (*sk).sk_error_queue;
    let mut skb: *mut sk_buff;
    let mut tmp: *mut sk_buff;
    let mut list: sk_buff_head = core::mem::zeroed();

    if issue_flags & IO_URING_F_CQE32 == 0 {
        return -EINVAL;
    }
    let ret = io_cmd_poll_multishot(cmd, issue_flags, EPOLLERR);
    if unlikely(ret != 0) {
        return ret;
    }

    if skb_queue_empty_lockless(q) {
        return -EAGAIN;
    }
    __skb_queue_head_init(&mut list);

    // scoped_guard(spinlock_irq, &q->lock)
    {
        skb_queue_walk_safe(q, skb, tmp) {
            // don't support skbs with payload
            if !skb_has_tx_timestamp(skb, sk) || (*skb).len != 0 {
                continue;
            }
            __skb_unlink(skb, q);
            __skb_queue_tail(&mut list, skb);
        }
    }

    loop {
        skb = skb_peek(&mut list);
        if skb.is_null() {
            break;
        }
        if !io_process_timestamp_skb(cmd, sk, skb, issue_flags) {
            break;
        }
        __skb_dequeue(&mut list);
        consume_skb(skb);
    }

    if !unlikely(skb_queue_empty(&mut list)) {
        // scoped_guard(spinlock_irqsave, &q->lock)
        skb_queue_splice(&mut list, q);
    }
    -EAGAIN
}

unsafe fn io_uring_cmd_getsockname(
    sock: *mut socket,
    cmd: *mut io_uring_cmd,
    _issue_flags: c_uint,
) -> c_int {
    let sqe = (*cmd).sqe;
    if (*sqe).ioprio != 0 || (*sqe).__pad1 != 0 || (*sqe).len != 0 || (*sqe).rw_flags != 0 {
        return -EINVAL;
    }

    let uaddr = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    let ulen = u64_to_user_ptr(READ_ONCE((*sqe).addr3));
    let peer = READ_ONCE((*sqe).optlen);
    if peer > 1 {
        return -EINVAL;
    }
    do_getsockname(sock, peer, uaddr, ulen)
}

pub unsafe fn io_uring_cmd_sock(cmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    let sock = (*(*cmd).file).private_data as *mut socket;

    match (*cmd).cmd_op {
        SOCKET_URING_OP_SIOCINQ => io_uring_cmd_get_sock_ioctl(sock, SIOCINQ),
        SOCKET_URING_OP_SIOCOUTQ => io_uring_cmd_get_sock_ioctl(sock, SIOCOUTQ),
        SOCKET_URING_OP_GETSOCKOPT => io_uring_cmd_getsockopt(sock, cmd, issue_flags),
        SOCKET_URING_OP_SETSOCKOPT => io_uring_cmd_setsockopt(sock, cmd, issue_flags),
        SOCKET_URING_OP_TX_TIMESTAMP => io_uring_cmd_timestamp(sock, cmd, issue_flags),
        SOCKET_URING_OP_GETSOCKNAME => io_uring_cmd_getsockname(sock, cmd, issue_flags),
        _ => -EOPNOTSUPP,
    }
}

// EXPORT_SYMBOL_GPL(io_uring_cmd_sock);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
