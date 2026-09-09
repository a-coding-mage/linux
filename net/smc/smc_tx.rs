// SPDX-License-Identifier: GPL-2.0
/* Shared Memory Communications over RDMA (SMC-R) and RoCE.
 * Manage send buffer. Producer/consumer implementation. */

const SMC_TX_WORK_DELAY: u64 = 0;

unsafe fn smc_tx_write_space(sk: *mut sock) {
    let sock = (*sk).sk_socket;
    let smc = smc_sk(sk);
    if atomic_read(&(*smc).conn.sndbuf_space) != 0 && !sock.is_null() {
        if test_bit(SOCK_NOSPACE, &(*sock).flags) { SMC_STAT_RMB_TX_FULL(smc, (*smc).conn.lnk.is_null()); }
        clear_bit(SOCK_NOSPACE, &mut (*sock).flags);
        rcu_read_lock();
        let wq = rcu_dereference((*sk).sk_wq);
        if skwq_has_sleeper(wq) { wake_up_interruptible_poll(&mut (*wq).wait, EPOLLOUT | EPOLLWRNORM | EPOLLWRBAND); }
        if !wq.is_null() && !(*wq).fasync_list.is_null() && ((*sk).sk_shutdown & SEND_SHUTDOWN) == 0 { sock_wake_async(wq, SOCK_WAKE_SPACE, POLL_OUT); }
        rcu_read_unlock();
    }
}

pub unsafe fn smc_tx_sndbuf_nonfull(smc: *mut smc_sock) {
    if !(*smc).sk.sk_socket.is_null() && test_bit(SOCK_NOSPACE, &(*(*smc).sk.sk_socket).flags) { ((*smc).sk.sk_write_space)(&mut (*smc).sk); }
}

unsafe fn smc_tx_wait(smc: *mut smc_sock, flags: c_int) -> c_int {
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk; let mut timeo = sock_sndtimeo(sk, flags & MSG_DONTWAIT); let mut rc = 0;
    let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function); add_wait_queue(sk_sleep(sk), &mut wait);
    loop {
        sk_set_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        if (*sk).sk_err != 0 || ((*sk).sk_shutdown & SEND_SHUTDOWN) != 0 || conn.killed || conn.local_tx_ctrl.conn_state_flags.peer_done_writing { rc = -EPIPE; break; }
        if smc_cdc_rxed_any_close(conn) { rc = -ECONNRESET; break; }
        if timeo == 0 { set_bit(SOCK_NOSPACE, &mut (*(*sk).sk_socket).flags); rc = -EAGAIN; break; }
        if signal_pending(current) { rc = sock_intr_errno(timeo); break; }
        sk_clear_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        if atomic_read(&conn.sndbuf_space) != 0 && !conn.urg_tx_pend { break; }
        set_bit(SOCK_NOSPACE, &mut (*(*sk).sk_socket).flags);
        sk_wait_event(sk, &mut timeo, READ_ONCE((*sk).sk_err) != 0 || (READ_ONCE((*sk).sk_shutdown) & SEND_SHUTDOWN) != 0 || smc_cdc_rxed_any_close(conn) || (atomic_read(&conn.sndbuf_space) != 0 && !conn.urg_tx_pend), &mut wait);
    }
    remove_wait_queue(sk_sleep(sk), &mut wait); rc
}

unsafe fn smc_tx_is_corked(smc: *mut smc_sock) -> bool { (tcp_sk((*smc).clcsock.sk).nonagle & TCP_NAGLE_CORK) != 0 }

unsafe fn smc_should_autocork(smc: *mut smc_sock) -> bool {
    let conn = &mut (*smc).conn;
    let corking_size = min((*conn).sndbuf_desc.len >> 1, sock_net(&mut (*smc).sk).smc.sysctl_autocorking_size);
    if atomic_read(&conn.cdc_pend_tx_wr) == 0 || smc_tx_prepared_sends(conn) > corking_size { return false; } true
}

unsafe fn smc_tx_should_cork(smc: *mut smc_sock, msg: *mut msghdr) -> bool {
    let conn = &mut (*smc).conn;
    if smc_should_autocork(smc) { return true; }
    if (((*msg).msg_flags & MSG_MORE) != 0 || smc_tx_is_corked(smc)) && atomic_read(&conn.sndbuf_space) != 0 { return true; } false
}

pub unsafe fn smc_tx_sendmsg(smc: *mut smc_sock, msg: *mut msghdr, len: usize) -> isize {
    let conn = &mut (*smc).conn; let sk = &mut (*smc).sk; let mut send_done = 0usize; let mut send_remaining = len; let mut rc: c_int;
    sk_clear_bit(SOCKWQ_ASYNC_NOSPACE, sk);
    if (*sk).sk_err != 0 || ((*sk).sk_shutdown & SEND_SHUTDOWN) != 0 { rc = -EPIPE; return sk_stream_error(sk, (*msg).msg_flags, rc) as isize; }
    if (*sk).sk_state == SMC_INIT { return -ENOTCONN as isize; }
    if len > conn.sndbuf_desc.len { SMC_STAT_RMB_TX_SIZE_SMALL(smc, conn.lnk.is_null()); }
    if len > conn.peer_rmbe_size { SMC_STAT_RMB_TX_PEER_SIZE_SMALL(smc, conn.lnk.is_null()); }
    if ((*msg).msg_flags & MSG_OOB) != 0 { SMC_STAT_INC(smc, urg_data_cnt); }
    while msg_data_left(msg) {
        if ((*smc).sk.sk_shutdown & SEND_SHUTDOWN) != 0 || (*smc).sk.sk_err == ECONNABORTED || conn.killed { return -EPIPE as isize; }
        if smc_cdc_rxed_any_close(conn) { return if send_done != 0 { send_done as isize } else { -ECONNRESET as isize }; }
        if ((*msg).msg_flags & MSG_OOB) != 0 { conn.local_tx_ctrl.prod_flags.urg_data_pending = 1; }
        if atomic_read(&conn.sndbuf_space) == 0 || conn.urg_tx_pend { if send_done != 0 { return send_done as isize; } rc = smc_tx_wait(smc, (*msg).msg_flags); if rc != 0 { return sk_stream_error(sk, (*msg).msg_flags, rc) as isize; } continue; }
        let writespace = atomic_read(&conn.sndbuf_space) as usize; let copylen = min(send_remaining, writespace); let sndbuf_base = conn.sndbuf_desc.cpu_addr; let mut prep = zeroed::<smc_host_cursor>(); smc_curs_copy(&mut prep, &conn.tx_curs_prep, conn); let mut chunk_len = min(copylen, conn.sndbuf_desc.len - prep.count); let mut chunk_len_sum = chunk_len; let mut chunk_off = prep.count;
        for _ in 0..2 { rc = memcpy_from_msg(sndbuf_base.add(chunk_off), msg, chunk_len); if rc != 0 { smc_sndbuf_sync_sg_for_device(conn); if send_done != 0 { return send_done as isize; } return sk_stream_error(sk, (*msg).msg_flags, rc) as isize; } send_done += chunk_len; send_remaining -= chunk_len; if chunk_len_sum == copylen { break; } chunk_len = copylen - chunk_len; chunk_len_sum += chunk_len; chunk_off = 0; }
        smc_sndbuf_sync_sg_for_device(conn); smc_curs_add(conn.sndbuf_desc.len, &mut prep, copylen); smc_curs_copy(&mut conn.tx_curs_prep, &prep, conn); smp_mb__before_atomic(); atomic_sub(copylen, &mut conn.sndbuf_space); smp_mb__after_atomic(); if ((*msg).msg_flags & MSG_OOB) != 0 && send_remaining == 0 { conn.urg_tx_pend = true; } if !smc_tx_should_cork(smc, msg) { smc_tx_sndbuf_nonempty(conn); } trace_smc_tx_sendmsg(smc, copylen);
    } send_done as isize
}

pub unsafe fn smcd_tx_ism_write(conn: *mut smc_connection, data: *mut c_void, len: usize, offset: u32, signal: c_int) -> c_int { let rc = smc_ism_write((*conn).lgr.smcd, (*conn).peer_token, (*conn).peer_rmbe_idx, signal, (*conn).tx_off + offset, data, len); if rc != 0 { (*conn).local_tx_ctrl.conn_state_flags.peer_conn_abort = 1; } rc }

unsafe fn smc_tx_advance_cursors(conn: *mut smc_connection, prod: *mut smc_host_cursor, sent: *mut smc_host_cursor, len: usize) { smc_curs_add((*conn).peer_rmbe_size, prod, len); smp_mb__before_atomic(); atomic_sub(len, &mut (*conn).peer_rmbe_space); smp_mb__after_atomic(); smc_curs_add((*conn).sndbuf_desc.len, sent, len); }

unsafe fn smcr_tx_rdma_writes(conn: *mut smc_connection, len: usize, mut src_off: usize, mut src_len: usize, mut dst_off: usize, mut dst_len: usize, wr: *mut smc_rdma_wr) -> c_int {
    let link = (*conn).lnk; let dma_addr = sg_dma_address((*conn).sndbuf_desc.sgt[(*link).link_idx].sgl); let virt_addr = (*conn).sndbuf_desc.cpu_addr as u64; let mut src_len_sum = src_len; let mut dst_len_sum = dst_len; let mut sent_count = src_off;
    for _ in 0..2 { let rd = &mut (*wr).wr_tx_rdma[0]; let mut num_sges = 0; let mut base = dma_addr; if dst_len < (*link).qp_attr.cap.max_inline_data { base = virt_addr; rd.wr.send_flags |= IB_SEND_INLINE; } else { rd.wr.send_flags &= !IB_SEND_INLINE; }
        for i in 0..2 { rd.wr.sg_list[i].addr = if (*conn).sndbuf_desc.is_vm { virt_addr + src_off as u64 } else { base + src_off as u64 }; rd.wr.sg_list[i].length = src_len; if (*conn).sndbuf_desc.is_vm { rd.wr.sg_list[i].lkey = (*conn).sndbuf_desc.mr[(*link).link_idx].lkey; } num_sges += 1; src_off += src_len; if src_off >= (*conn).sndbuf_desc.len { src_off -= (*conn).sndbuf_desc.len; } if src_len_sum == dst_len { break; } src_len = dst_len - src_len; src_len_sum += src_len; }
        let rc = smc_tx_rdma_write(conn, dst_off as c_int, num_sges, rd); if rc != 0 { return rc; } if dst_len_sum == len { break; } dst_off = 0; dst_len = len - dst_len; dst_len_sum += dst_len; src_len = min(dst_len, (*conn).sndbuf_desc.len - sent_count); src_len_sum = src_len;
    } 0
}

unsafe fn smcd_tx_rdma_writes(conn: *mut smc_connection, len: usize, mut src_off: usize, mut src_len: usize, mut dst_off: usize, mut dst_len: usize) -> c_int {
    if (*conn).sndbuf_desc.is_attached { return 0; } let mut src_sum = src_len; let mut dst_sum = dst_len;
    for _ in 0..2 { for _ in 0..2 { let data = (*conn).sndbuf_desc.cpu_addr.add(src_off); let rc = smcd_tx_ism_write(conn, data, src_len, (dst_off + size_of::<smcd_cdc_msg>()) as u32, 0); if rc != 0 { return rc; } dst_off += src_len; src_off += src_len; if src_off >= (*conn).sndbuf_desc.len { src_off -= (*conn).sndbuf_desc.len; } if src_sum == dst_len { break; } src_len = dst_len - src_len; src_sum += src_len; } if dst_sum == len { break; } dst_off = 0; dst_len = len - dst_len; dst_sum += dst_len; src_len = min(dst_len, (*conn).sndbuf_desc.len - src_off); src_sum = src_len; } 0
}

unsafe fn smc_tx_rdma_write(conn: *mut smc_connection, off: c_int, n: c_int, wr: *mut ib_rdma_wr) -> c_int { let link = (*conn).lnk; (*wr).wr.wr_id = smc_wr_tx_get_next_wr_id(link); (*wr).wr.num_sge = n; (*wr).remote_addr = (*conn).lgr.rtokens[(*conn).rtoken_idx][(*link).link_idx].dma_addr + (*conn).tx_off as u64 + off as u64; (*wr).rkey = (*conn).lgr.rtokens[(*conn).rtoken_idx][(*link).link_idx].rkey; let rc = ib_post_send((*link).roce_qp, &mut (*wr).wr, core::ptr::null_mut()); if rc != 0 { smcr_link_down_cond_sched(link); } rc }

unsafe fn smc_tx_rdma_writes(conn: *mut smc_connection, wr: *mut smc_rdma_wr) -> c_int {
    let mut sent = zeroed::<smc_host_cursor>(); let mut prep = zeroed::<smc_host_cursor>(); let mut prod = zeroed::<smc_host_cursor>(); let mut cons = zeroed::<smc_host_cursor>(); smc_curs_copy(&mut sent, &(*conn).tx_curs_sent, conn); smc_curs_copy(&mut prep, &(*conn).tx_curs_prep, conn); let to_send = smc_curs_diff((*conn).sndbuf_desc.len, &sent, &prep); if to_send <= 0 { return 0; } let space = atomic_read(&(*conn).peer_rmbe_space); if space <= 0 { return 0; } smc_curs_copy(&mut prod, &(*conn).local_tx_ctrl.prod, conn); smc_curs_copy(&mut cons, &(*conn).local_rx_ctrl.cons, conn); (*conn).local_tx_ctrl.prod_flags.write_blocked = to_send >= space; let len = min(to_send as usize, space as usize); let dst_off = prod.count; let dst_len = if prod.wrap == cons.wrap { min((*conn).peer_rmbe_size - prod.count, len) } else { len }; let src_len = if sent.count + dst_len <= (*conn).sndbuf_desc.len { dst_len } else { (*conn).sndbuf_desc.len - sent.count }; let rc = if (*conn).lgr.is_smcd { smcd_tx_rdma_writes(conn, len, sent.count, src_len, dst_off, dst_len) } else { smcr_tx_rdma_writes(conn, len, sent.count, src_len, dst_off, dst_len, wr) }; if rc == 0 { smc_tx_advance_cursors(conn, &mut prod, &mut sent, len); smc_curs_copy(&mut (*conn).local_tx_ctrl.prod, &prod, conn); smc_curs_copy(&mut (*conn).tx_curs_sent, &sent, conn); } rc
}

pub unsafe fn smc_tx_sndbuf_nonempty(conn: *mut smc_connection) -> c_int { if smc_tx_prepared_sends(conn) <= 0 || atomic_read(&(*conn).peer_rmbe_space) <= 0 { return 0; } if (*conn).killed || (*conn).local_rx_ctrl.conn_state_flags.peer_conn_abort != 0 { return -EPIPE; } let rc = smc_tx_rdma_writes(conn, core::ptr::null_mut()); if rc == 0 { smc_close_wake_tx_prepared(container_of!(conn, smc_sock, conn)); } rc }
pub unsafe fn smc_tx_pending(conn: *mut smc_connection) { if (*container_of!(conn, smc_sock, conn)).sk.sk_err == 0 { smc_tx_sndbuf_nonempty(conn); } }
pub unsafe fn smc_tx_work(work: *mut work_struct) { let conn = container_of!(work, smc_connection, tx_work); smc_tx_pending(conn); }

pub unsafe fn smc_tx_consumer_update(conn: *mut smc_connection, force: bool) { let mut cfed = zeroed::<smc_host_cursor>(); let mut cons = zeroed::<smc_host_cursor>(); smc_curs_copy(&mut cons, &(*conn).local_tx_ctrl.cons, conn); smc_curs_copy(&mut cfed, &(*conn).rx_curs_confirmed, conn); let to_confirm = smc_curs_diff((*conn).rmb_desc.len, &cfed, &cons); if ((*conn).local_rx_ctrl.prod_flags.cons_curs_upd_req || force || to_confirm > (*conn).rmbe_update_limit) && !(*conn).killed && (*conn).local_rx_ctrl.conn_state_flags.peer_conn_abort == 0 { smc_cdc_get_slot_and_msg_send(conn); } }

pub unsafe fn smc_tx_init(smc: *mut smc_sock) { (*smc).sk.sk_write_space = Some(smc_tx_write_space); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
