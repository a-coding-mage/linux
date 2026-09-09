// SPDX-License-Identifier: GPL-2.0
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Manage RMBE
 * copy new RMBE data into user space
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn smc_rx_wake_up(sk: *mut sock) {
    let wq: *mut socket_wq;

    trace_sk_data_ready(sk);
    rcu_read_lock();
    wq = rcu_dereference((*sk).sk_wq);
    if skwq_has_sleeper(wq) {
        wake_up_interruptible_sync_poll(&mut (*wq).wait,
                                        EPOLLIN | EPOLLPRI | EPOLLRDNORM | EPOLLRDBAND);
    }
    sk_wake_async_rcu(sk, SOCK_WAKE_WAITD, POLL_IN);
    if (*sk).sk_shutdown == SHUTDOWN_MASK || (*sk).sk_state == SMC_CLOSED {
        sk_wake_async_rcu(sk, SOCK_WAKE_WAITD, POLL_HUP);
    }
    rcu_read_unlock();
}

unsafe fn smc_rx_update_consumer(smc: *mut smc_sock, mut cons: smc_host_cursor, len: usize) -> i32 {
    let conn = &mut (*smc).conn;
    let sk = &mut (*smc).sk;
    let mut force = false;
    let mut diff: i32;
    let mut rc = 0;

    smc_curs_add((*conn.rmb_desc).len, &mut cons, len);
    if conn.urg_state == SMC_URG_VALID || conn.urg_rx_skip_pend {
        diff = smc_curs_comp((*conn.rmb_desc).len, &cons, &conn.urg_curs);
        if sock_flag(sk, SOCK_URGINLINE) {
            if diff == 0 {
                force = true;
                rc = 1;
                conn.urg_state = SMC_URG_READ;
            }
        } else if diff == 1 {
            force = true;
            smc_curs_add((*conn.rmb_desc).len, &mut cons, 1);
            conn.urg_rx_skip_pend = false;
        } else if diff < -1 {
            conn.urg_state = SMC_URG_READ;
        }
    }
    smc_curs_copy(&mut conn.local_tx_ctrl.cons, &cons, conn);
    smc_tx_consumer_update(conn, force);
    rc
}

unsafe fn smc_rx_update_cons(smc: *mut smc_sock, len: usize) {
    let conn = &mut (*smc).conn;
    let mut cons: smc_host_cursor = core::mem::zeroed();
    smc_curs_copy(&mut cons, &conn.local_tx_ctrl.cons, conn);
    smc_rx_update_consumer(smc, cons, len);
}

#[repr(C)]
struct smc_spd_priv {
    smc: *mut smc_sock,
    len: usize,
}

unsafe fn smc_rx_pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) {
    let priv_ = (*buf).private as *mut smc_spd_priv;
    let conn = &mut (*(*priv_).smc).conn;
    let smc = (*priv_).smc;
    let sk = &mut (*smc).sk;
    lock_sock(sk);
    if conn.freed {
        release_sock(sk);
        kfree(priv_ as *mut core::ffi::c_void);
        put_page((*buf).page);
        sock_put(sk);
        return;
    }
    smc_rx_update_cons(smc, (*priv_).len);
    release_sock(sk);
    if atomic_sub_and_test((*priv_).len, &mut conn.splice_pending) {
        smc_rx_wake_up(sk);
    }
    kfree(priv_ as *mut core::ffi::c_void);
    put_page((*buf).page);
    sock_put(sk);
}

unsafe fn smc_rx_pipe_buf_get(_pipe: *mut pipe_inode_info, _buf: *mut pipe_buffer) -> bool { false }

static smc_pipe_ops: pipe_buf_operations = pipe_buf_operations {
    release: Some(smc_rx_pipe_buf_release),
    get: Some(smc_rx_pipe_buf_get),
};

unsafe fn smc_rx_spd_release(spd: *mut splice_pipe_desc, i: u32) {
    let priv_ = (*spd).partial.add(i as usize).private as *mut smc_spd_priv;
    let sk = &mut (*(*priv_).smc).sk;
    kfree(priv_ as *mut core::ffi::c_void);
    put_page(*(*spd).pages.add(i as usize));
    sock_put(sk);
}

unsafe fn smc_rx_splice(pipe: *mut pipe_inode_info, src: *mut i8, len: usize, smc: *mut smc_sock) -> i32 {
    let lgr = (*smc).conn.lgr;
    let mut offset = offset_in_page(src);
    let nr_pages = if !(*lgr).is_smcd && (*(*smc).conn.rmb_desc).is_vm {
        page_align(len + offset) / PAGE_SIZE
    } else { 1 };
    let pages = kzalloc_pages(nr_pages);
    if pages.is_null() { return -ENOMEM; }
    let partial = kzalloc_partial_pages(nr_pages);
    if partial.is_null() { kfree(pages as *mut _); return -ENOMEM; }
    let privs = kzalloc_priv_pages(nr_pages);
    if privs.is_null() { kfree(partial as *mut _); kfree(pages as *mut _); return -ENOMEM; }
    let mut i = 0;
    while i < nr_pages {
        *privs.add(i) = kzalloc_priv();
        if (*privs.add(i)).is_null() {
            while i > 0 { i -= 1; kfree(*privs.add(i) as *mut _); }
            kfree(privs as *mut _); kfree(partial as *mut _); kfree(pages as *mut _); return -ENOMEM;
        }
        i += 1;
    }
    if (*lgr).is_smcd || !(*(*smc).conn.rmb_desc).is_vm {
        (*(*privs).as_mut()).len = len;
        (*(*privs).as_mut()).smc = smc;
        (*partial).offset = src.offset_from((*(*smc).conn.rmb_desc).cpu_addr as *mut i8) as usize;
        (*partial).len = len;
        (*partial).private = *privs as *mut smc_spd_priv as usize;
        *pages = (*smc).conn.rmb_desc.pages;
    } else {
        let mut left = len;
        let mut buf = src;
        i = 0;
        while i < nr_pages {
            let size = core::cmp::min(PAGE_SIZE - offset, left);
            (*(*privs.add(i))).len = size;
            (*(*privs.add(i))).smc = smc;
            *pages.add(i) = vmalloc_to_page(buf as *mut _);
            (*partial.add(i)).offset = offset;
            (*partial.add(i)).len = size;
            (*partial.add(i)).private = *privs.add(i) as *mut smc_spd_priv as usize;
            buf = buf.add(size); left -= size; offset = 0; i += 1;
        }
    }
    i = 0;
    while i < nr_pages { get_page(*pages.add(i)); sock_hold(&mut (*smc).sk); i += 1; }
    let mut spd: splice_pipe_desc = core::mem::zeroed();
    spd.nr_pages_max = nr_pages as u32; spd.nr_pages = nr_pages as u32;
    spd.pages = pages; spd.partial = partial; spd.ops = &smc_pipe_ops; spd.spd_release = Some(smc_rx_spd_release);
    let bytes = splice_to_pipe(pipe, &mut spd);
    if bytes > 0 { atomic_add(bytes as usize, &mut (*smc).conn.splice_pending); }
    kfree(privs as *mut _); kfree(partial as *mut _); kfree(pages as *mut _); bytes
}

unsafe fn smc_rx_data_available_and_no_splice_pend(conn: *mut smc_connection, peeked: usize) -> i32 {
    (smc_rx_data_available(conn, peeked) && atomic_read(&mut (*conn).splice_pending) == 0) as i32
}

unsafe fn smc_rx_recv_urg(smc: *mut smc_sock, msg: *mut msghdr, mut len: i32, flags: i32) -> i32 {
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk; let mut cons: smc_host_cursor = core::mem::zeroed();
    if sock_flag(sk, SOCK_URGINLINE) || conn.urg_state != SMC_URG_VALID || conn.urg_state == SMC_URG_READ { return -EINVAL; }
    SMC_STAT_INC(smc, urg_data_cnt);
    if !(flags & MSG_PEEK) != 0 { conn.urg_state = SMC_URG_READ; }
    (*msg).msg_flags |= MSG_OOB;
    if len > 0 {
        let mut rc = 0;
        if flags & MSG_TRUNC == 0 { rc = memcpy_to_msg(msg, &conn.urg_rx_byte as *const _ as *const _, 1); }
        len = 1; smc_curs_copy(&mut cons, &conn.local_tx_ctrl.cons, conn);
        if smc_curs_diff((*conn.rmb_desc).len, &cons, &conn.urg_curs) > 1 { conn.urg_rx_skip_pend = true; }
        if flags & MSG_PEEK == 0 { smc_rx_update_consumer(smc, cons, 0); }
        return if rc != 0 { -EFAULT } else { len };
    }
    (*msg).msg_flags |= MSG_TRUNC;
    if sk.sk_state == SMC_CLOSED || sk.sk_shutdown & RCV_SHUTDOWN != 0 { return 0; }
    -EAGAIN
}

unsafe fn smc_rx_recvmsg_data_available(smc: *mut smc_sock, peeked: usize) -> bool {
    let conn = &mut (*smc).conn;
    if smc_rx_data_available(conn, peeked) { true } else { if conn.urg_state == SMC_URG_VALID { smc_rx_update_cons(smc, 0); } false }
}

// The main receive loop is kept in source order; kernel helper declarations and
// structure definitions are supplied by the surrounding translation unit.
unsafe fn smc_rx_recvmsg(smc: *mut smc_sock, msg: *mut msghdr, pipe: *mut pipe_inode_info, len: usize, flags: i32) -> isize {
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk;
    if flags & MSG_ERRQUEUE != 0 { return -EINVAL as isize; }
    if sk.sk_state == SMC_LISTEN { return -ENOTCONN as isize; }
    if flags & MSG_OOB != 0 { return smc_rx_recv_urg(smc, msg, len as i32, flags) as isize; }
    let mut timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT != 0);
    let target = sock_rcvlowat(sk, flags & MSG_WAITALL != 0, len);
    let mut read_done = 0usize; let mut read_remaining = len; let mut peeked_bytes = 0usize;
    let rcvbuf_base = conn.rx_off.add((*conn.rmb_desc).cpu_addr as usize) as *mut i8;
    loop {
        if read_done >= target || (!pipe.is_null() && read_done != 0) || conn.killed { break; }
        if !smc_rx_recvmsg_data_available(smc, peeked_bytes) {
            if sk.sk_shutdown & RCV_SHUTDOWN != 0 { break; }
            if read_done != 0 && (sk.sk_err != 0 || sk.sk_state == SMC_CLOSED || timeo == 0 || signal_pending(current())) { break; }
            if read_done == 0 { if sk.sk_err != 0 { read_done = sock_error(sk) as usize; break; } if sk.sk_state == SMC_CLOSED { if !sock_flag(sk, SOCK_DONE) { return -ENOTCONN as isize; } break; } if timeo == 0 { return -EAGAIN as isize; } if signal_pending(current()) { return sock_intr_errno(timeo) as isize; } }
            smc_rx_wait(smc, &mut timeo, peeked_bytes, Some(smc_rx_data_available)); continue;
        }
        let readable = smc_rx_data_available(conn, peeked_bytes); let splbytes = atomic_read(&mut conn.splice_pending) as usize;
        if !readable || (!msg.is_null() && splbytes != 0) { smc_rx_wait(smc, &mut timeo, peeked_bytes, if splbytes != 0 { Some(smc_rx_data_available_and_no_splice_pend) } else { Some(smc_rx_data_available) }); continue; }
        let mut cons: smc_host_cursor = core::mem::zeroed(); smc_curs_copy(&mut cons, &conn.local_tx_ctrl.cons, conn);
        if flags & MSG_PEEK != 0 && peeked_bytes != 0 { smc_curs_add((*conn.rmb_desc).len, &mut cons, peeked_bytes); } if splbytes != 0 { smc_curs_add((*conn.rmb_desc).len, &mut cons, splbytes); }
        let copylen = core::cmp::min(read_remaining, readable as usize); let mut chunk_len = core::cmp::min(copylen, (*conn.rmb_desc).len - cons.count); let mut chunk_off = cons.count; let mut sum = chunk_len;
        smc_rmb_sync_sg_for_cpu(conn);
        for _ in 0..2 { if flags & MSG_TRUNC == 0 { let rc = if !msg.is_null() { memcpy_to_msg(msg, rcvbuf_base.add(chunk_off) as *const _, chunk_len) } else { smc_rx_splice(pipe, rcvbuf_base.add(chunk_off), chunk_len, smc) }; if rc < 0 { if read_done == 0 { read_done = -EFAULT as usize; } return read_done as isize; } } read_remaining -= chunk_len; read_done += chunk_len; if flags & MSG_PEEK != 0 { peeked_bytes += chunk_len; } if sum == copylen { break; } chunk_len = copylen - chunk_len; sum += chunk_len; chunk_off = 0; }
        if flags & MSG_PEEK == 0 { smp_mb__before_atomic(); atomic_sub(copylen, &mut conn.bytes_to_rcv); smp_mb__after_atomic(); if !msg.is_null() && smc_rx_update_consumer(smc, cons, copylen) != 0 { break; } }
        trace_smc_rx_recvmsg(smc, copylen);
        if read_remaining == 0 { break; }
    }
    read_done as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
