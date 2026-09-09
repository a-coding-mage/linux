/* Direct Rust translation of recv.c. */

pub unsafe fn rds_inc_init(inc: *mut rds_incoming, conn: *mut rds_connection,
                           saddr: *mut in6_addr) {
    refcount_set(&mut (*inc).i_refcount, 1);
    INIT_LIST_HEAD(&mut (*inc).i_item);
    (*inc).i_conn = conn;
    (*inc).i_conn_path = core::ptr::null_mut();
    (*inc).i_saddr = *saddr;
    (*inc).i_usercopy.rdma_cookie = 0;
    (*inc).i_usercopy.rx_tstamp = ktime_set(0, 0);
    memset((*inc).i_rx_lat_trace.as_mut_ptr() as *mut _, 0,
           core::mem::size_of_val(&(*inc).i_rx_lat_trace));
}

pub unsafe fn rds_inc_path_init(inc: *mut rds_incoming, cp: *mut rds_conn_path,
                                saddr: *mut in6_addr) {
    refcount_set(&mut (*inc).i_refcount, 1);
    INIT_LIST_HEAD(&mut (*inc).i_item);
    (*inc).i_conn = (*cp).cp_conn;
    (*inc).i_conn_path = cp;
    (*inc).i_saddr = *saddr;
    (*inc).i_usercopy.rdma_cookie = 0;
    (*inc).i_usercopy.rx_tstamp = ktime_set(0, 0);
    memset((*inc).i_rx_lat_trace.as_mut_ptr() as *mut _, 0,
           core::mem::size_of_val(&(*inc).i_rx_lat_trace));
}

unsafe fn rds_inc_addref(inc: *mut rds_incoming) {
    rdsdebug!("addref inc %p ref %d\n", inc, refcount_read(&(*inc).i_refcount));
    refcount_inc(&mut (*inc).i_refcount);
}

pub unsafe fn rds_inc_put(inc: *mut rds_incoming) {
    rdsdebug!("put inc %p ref %d\n", inc, refcount_read(&(*inc).i_refcount));
    if refcount_dec_and_test(&mut (*inc).i_refcount) {
        BUG_ON(!list_empty(&(*inc).i_item));
        ((*(*inc).i_conn).c_trans).inc_free.unwrap()(inc);
    }
}

unsafe fn rds_recv_rcvbuf_delta(rs: *mut rds_sock, _sk: *mut sock,
                                map: *mut rds_cong_map, delta: i32, port: __be16) {
    if delta == 0 { return; }
    (*rs).rs_rcv_bytes += delta;
    if delta > 0 { rds_stats_add(s_recv_bytes_added_to_socket, delta); }
    else { rds_stats_add(s_recv_bytes_removed_from_socket, -delta); }
    if (*(*rs).rs_transport).t_type == RDS_TRANS_LOOP { return; }
    let now_congested = (*rs).rs_rcv_bytes > rds_sk_rcvbuf(rs);
    rdsdebug!("rs %p recv bytes %d buf %d now_cong %d delta %d\n", rs,
              (*rs).rs_rcv_bytes, rds_sk_rcvbuf(rs), now_congested, delta);
    if (*rs).rs_congested == 0 && now_congested {
        (*rs).rs_congested = 1;
        rds_cong_set_bit(map, port);
        rds_cong_queue_updates(map);
    } else if (*rs).rs_congested != 0 && (*rs).rs_rcv_bytes < rds_sk_rcvbuf(rs) / 2 {
        (*rs).rs_congested = 0;
        rds_cong_clear_bit(map, port);
        rds_cong_queue_updates(map);
    }
}

unsafe fn rds_conn_peer_gen_update(conn: *mut rds_connection, peer_gen_num: u32) {
    WARN_ON((*(*conn).c_trans).t_type != RDS_TRANS_TCP);
    if peer_gen_num != 0 {
        if (*conn).c_peer_gen_num != 0 && peer_gen_num != (*conn).c_peer_gen_num {
            for i in 0..RDS_MPATH_WORKERS {
                let cp = (*conn).c_path.as_mut_ptr().add(i as usize);
                let mut flags = 0;
                spin_lock_irqsave(&mut (*cp).cp_lock, &mut flags);
                (*cp).cp_next_tx_seq = 1;
                (*cp).cp_next_rx_seq = 0;
                let mut rm: *mut rds_message = core::ptr::null_mut();
                let mut tmp: *mut rds_message = core::ptr::null_mut();
                list_for_each_entry_safe!(rm, tmp, &mut (*cp).cp_retrans, m_conn_item, {
                    set_bit(RDS_MSG_FLUSH, &mut (*rm).m_flags);
                });
                spin_unlock_irqrestore(&mut (*cp).cp_lock, flags);
            }
        }
        (*conn).c_peer_gen_num = peer_gen_num;
    }
}

unsafe fn rds_recv_incoming_exthdrs(inc: *mut rds_incoming, rs: *mut rds_sock) {
    let hdr = &mut (*inc).i_hdr;
    let mut pos = 0u32;
    let mut buffer: rds_ext_header_buffer = core::mem::zeroed();
    loop {
        let mut len = core::mem::size_of::<rds_ext_header_buffer>() as u32;
        let typ = rds_message_next_extension(hdr, &mut pos, &mut buffer as *mut _ as *mut _, &mut len);
        if typ == RDS_EXTHDR_NONE { break; }
        match typ {
            RDS_EXTHDR_RDMA => rds_rdma_unuse(rs, be32_to_cpu(buffer.rdma.h_rdma_rkey), 0),
            RDS_EXTHDR_RDMA_DEST => (*inc).i_usercopy.rdma_cookie = rds_rdma_make_cookie(
                be32_to_cpu(buffer.rdma_dest.h_rdma_rkey),
                be32_to_cpu(buffer.rdma_dest.h_rdma_offset)),
            _ => {}
        }
    }
}

unsafe fn rds_recv_hs_exthdrs(hdr: *mut rds_header, conn: *mut rds_connection) {
    let mut pos = 0u32;
    let mut buffer: rds_hs_ext_header_buffer = core::mem::zeroed();
    let mut new_with_sport_idx = false;
    let mut new_peer_gen_num = 0u32;
    let mut new_npaths = (*conn).c_npaths;
    loop {
        let mut len = core::mem::size_of::<rds_hs_ext_header_buffer>() as u32;
        let typ = rds_message_next_extension(hdr, &mut pos, &mut buffer as *mut _ as *mut _, &mut len);
        if typ == RDS_EXTHDR_NONE { break; }
        match typ {
            RDS_EXTHDR_NPATHS => new_npaths = core::cmp::min(RDS_MPATH_WORKERS, be16_to_cpu(buffer.rds_npaths) as i32),
            RDS_EXTHDR_GEN_NUM => new_peer_gen_num = be32_to_cpu(buffer.rds_gen_num),
            RDS_EXTHDR_SPORT_IDX => new_with_sport_idx = true,
            _ => pr_warn_ratelimited!("ignoring unknown exthdr type 0x%x\n", typ),
        }
    }
    (*conn).c_with_sport_idx = new_with_sport_idx;
    let fan_out;
    if new_npaths > 1 && new_npaths != (*conn).c_npaths {
        let cp0 = (*conn).c_path;
        let mut flags = 0;
        spin_lock_irqsave(&mut (*cp0).cp_lock, &mut flags);
        (*conn).c_cp0_mprds_catchup_tx_seq = (*cp0).cp_next_tx_seq;
        spin_unlock_irqrestore(&mut (*cp0).cp_lock, flags);
        fan_out = true;
    } else { fan_out = false; }
    (*conn).c_npaths = core::cmp::max(new_npaths, 1);
    (*conn).c_ping_triggered = 0;
    rds_conn_peer_gen_update(conn, new_peer_gen_num);
    if (*conn).c_npaths > 1 && ((*(*conn).c_trans).conn_slots_available).is_some() {
        ((*(*conn).c_trans).conn_slots_available).unwrap()(conn, fan_out);
    }
}

unsafe fn rds_start_mprds(conn: *mut rds_connection) {
    if (*conn).c_npaths > 1 && rds_addr_cmp(&(*conn).c_laddr, &(*conn).c_faddr) < 0 {
        for i in 0..(*conn).c_npaths {
            rds_conn_path_connect_if_down((*conn).c_path.add(i as usize));
        }
    }
}

pub unsafe fn rds_recv_incoming(conn: *mut rds_connection, saddr: *mut in6_addr,
                                daddr: *mut in6_addr, inc: *mut rds_incoming, _gfp: gfp_t) {
    let mut rs: *mut rds_sock = core::ptr::null_mut();
    let mut cp: *mut rds_conn_path;
    (*inc).i_conn = conn;
    (*inc).i_rx_jiffies = jiffies;
    cp = if (*(*conn).c_trans).t_mp_capable { (*inc).i_conn_path } else { (*conn).c_path };
    rdsdebug!("conn %p next %llu inc %p\n", conn, (*cp).cp_next_rx_seq, inc);
    let seq = be64_to_cpu((*inc).i_hdr.h_sequence);
    if seq < (*cp).cp_next_rx_seq && ((*inc).i_hdr.h_flags & RDS_FLAG_RETRANSMITTED) != 0 {
        rds_stats_inc(s_recv_drop_old_seq); return;
    }
    (*cp).cp_next_rx_seq = seq + 1;
    if rds_sysctl_ping_enable && (*inc).i_hdr.h_dport == 0 {
        if (*inc).i_hdr.h_sport == 0 { return; }
        rds_stats_inc(s_recv_ping);
        rds_send_pong(cp, (*inc).i_hdr.h_sport);
        if RDS_HS_PROBE(be16_to_cpu((*inc).i_hdr.h_sport), be16_to_cpu((*inc).i_hdr.h_dport)) {
            rds_recv_hs_exthdrs(&mut (*inc).i_hdr, (*cp).cp_conn);
            rds_start_mprds((*cp).cp_conn);
        }
        return;
    }
    if be16_to_cpu((*inc).i_hdr.h_dport) == RDS_FLAG_PROBE_PORT && (*inc).i_hdr.h_sport == 0 {
        rds_recv_hs_exthdrs(&mut (*inc).i_hdr, (*cp).cp_conn);
        rds_start_mprds((*cp).cp_conn);
        wake_up(&mut (*(*cp).cp_conn).c_hs_waitq);
        return;
    }
    rs = rds_find_bound(daddr, (*inc).i_hdr.h_dport, (*conn).c_bound_if);
    if rs.is_null() { rds_stats_inc(s_recv_drop_no_sock); return; }
    if !net_eq(sock_net(rds_rs_to_sk(rs)), rds_conn_net(conn)) {
        rds_stats_inc(s_recv_drop_no_sock); rds_sock_put(rs); return;
    }
    rds_recv_incoming_exthdrs(inc, rs);
    let sk = rds_rs_to_sk(rs);
    let mut flags = 0;
    write_lock_irqsave(&mut (*rs).rs_recv_lock, &mut flags);
    if !sock_flag(sk, SOCK_DEAD) {
        rds_stats_inc(s_recv_queued);
        rds_recv_rcvbuf_delta(rs, sk, (*inc).i_conn.as_ref().unwrap().c_lcong,
                              be32_to_cpu((*inc).i_hdr.h_len) as i32, (*inc).i_hdr.h_dport);
        if sock_flag(sk, SOCK_RCVTSTAMP) { (*inc).i_usercopy.rx_tstamp = ktime_get_real(); }
        rds_inc_addref(inc);
        (*inc).i_rx_lat_trace[RDS_MSG_RX_END as usize] = local_clock();
        list_add_tail(&mut (*inc).i_item, &mut (*rs).rs_recv_queue);
        __rds_wake_sk_sleep(sk);
    } else { rds_stats_inc(s_recv_drop_dead_sock); }
    write_unlock_irqrestore(&mut (*rs).rs_recv_lock, flags);
    rds_sock_put(rs);
}

unsafe fn rds_next_incoming(rs: *mut rds_sock, inc: *mut *mut rds_incoming) -> bool {
    if (*inc).is_null() {
        let mut flags = 0;
        read_lock_irqsave(&mut (*rs).rs_recv_lock, &mut flags);
        if !list_empty(&(*rs).rs_recv_queue) {
            *inc = list_entry((*rs).rs_recv_queue.next, rds_incoming, i_item);
            rds_inc_addref(*inc);
        }
        read_unlock_irqrestore(&mut (*rs).rs_recv_lock, flags);
    }
    !(*inc).is_null()
}

unsafe fn rds_still_queued(rs: *mut rds_sock, inc: *mut rds_incoming, drop: bool) -> i32 {
    let sk = rds_rs_to_sk(rs); let mut ret = 0; let mut to_drop = core::ptr::null_mut(); let mut flags = 0;
    write_lock_irqsave(&mut (*rs).rs_recv_lock, &mut flags);
    if !list_empty(&(*inc).i_item) {
        ret = 1;
        if drop {
            rds_recv_rcvbuf_delta(rs, sk, (*inc).i_conn.as_ref().unwrap().c_lcong,
                                  -(be32_to_cpu((*inc).i_hdr.h_len) as i32), (*inc).i_hdr.h_dport);
            list_del_init(&mut (*inc).i_item); to_drop = inc;
        }
    }
    write_unlock_irqrestore(&mut (*rs).rs_recv_lock, flags);
    if !to_drop.is_null() { rds_inc_put(to_drop); }
    ret
}

pub unsafe fn rds_notify_queue_get(rs: *mut rds_sock, msghdr: *mut msghdr) -> i32 {
    let mut cmsg: rds_rdma_notify = core::mem::zeroed(); let mut count = 0u32;
    let mut max_messages = !0u32; let mut flags = 0; let mut copy: list_head = core::mem::zeroed();
    INIT_LIST_HEAD(&mut copy);
    if !msghdr.is_null() { max_messages = (*msghdr).msg_controllen / CMSG_SPACE(core::mem::size_of::<rds_rdma_notify>()); if max_messages == 0 { max_messages = 1; } }
    spin_lock_irqsave(&mut (*rs).rs_lock, &mut flags);
    while !list_empty(&(*rs).rs_notify_queue) && count < max_messages { let n = list_entry((*rs).rs_notify_queue.next, rds_notifier, n_list); list_move(&mut (*n).n_list, &mut copy); count += 1; }
    spin_unlock_irqrestore(&mut (*rs).rs_lock, flags);
    if count == 0 { return 0; }
    let mut err = 0;
    while !list_empty(&copy) {
        let n = list_entry(copy.next, rds_notifier, n_list);
        if !msghdr.is_null() { cmsg.user_token = (*n).n_user_token; cmsg.status = (*n).n_status; err = put_cmsg(msghdr, SOL_RDS, RDS_CMSG_RDMA_STATUS, core::mem::size_of_val(&cmsg), &cmsg as *const _ as *const _); if err != 0 { break; } }
        list_del_init(&mut (*n).n_list); kfree(n as *mut _);
    }
    if !list_empty(&copy) { spin_lock_irqsave(&mut (*rs).rs_lock, &mut flags); list_splice(&mut copy, &mut (*rs).rs_notify_queue); spin_unlock_irqrestore(&mut (*rs).rs_lock, flags); }
    err
}

unsafe fn rds_notify_cong(rs: *mut rds_sock, msg: *mut msghdr) -> i32 {
    let notify = (*rs).rs_cong_notify; let mut flags = 0;
    let err = put_cmsg(msg, SOL_RDS, RDS_CMSG_CONG_UPDATE, core::mem::size_of_val(&notify), &notify as *const _ as *const _);
    if err != 0 { return err; }
    spin_lock_irqsave(&mut (*rs).rs_lock, &mut flags); (*rs).rs_cong_notify &= !notify; spin_unlock_irqrestore(&mut (*rs).rs_lock, flags); 0
}

unsafe fn rds_cmsg_recv(inc: *mut rds_incoming, msg: *mut msghdr, rs: *mut rds_sock) -> i32 {
    let mut ret = 0;
    if (*inc).i_usercopy.rdma_cookie != 0 { ret = put_cmsg(msg, SOL_RDS, RDS_CMSG_RDMA_DEST, core::mem::size_of_val(&(*inc).i_usercopy.rdma_cookie), &(*inc).i_usercopy.rdma_cookie as *const _ as *const _); if ret != 0 { return ret; } }
    if (*inc).i_usercopy.rx_tstamp != 0 && sock_flag(rds_rs_to_sk(rs), SOCK_RCVTSTAMP) {
        let tv = ns_to_kernel_old_timeval((*inc).i_usercopy.rx_tstamp);
        if !sock_flag(rds_rs_to_sk(rs), SOCK_TSTAMP_NEW) { ret = put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMP_OLD, core::mem::size_of_val(&tv), &tv as *const _ as *const _); }
        else { let sk_tv = __kernel_sock_timeval { tv_sec: tv.tv_sec, tv_usec: tv.tv_usec }; ret = put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMP_NEW, core::mem::size_of_val(&sk_tv), &sk_tv as *const _ as *const _); }
        if ret != 0 { return ret; }
    }
    if (*rs).rs_rx_traces != 0 { let mut t: rds_cmsg_rx_trace = core::mem::zeroed(); (*inc).i_rx_lat_trace[RDS_MSG_RX_CMSG as usize] = local_clock(); t.rx_traces = (*rs).rs_rx_traces; for i in 0..(*rs).rs_rx_traces { let j = (*rs).rs_rx_trace[i as usize]; t.rx_trace_pos[i as usize] = j; t.rx_trace[i as usize] = (*inc).i_rx_lat_trace[(j + 1) as usize] - (*inc).i_rx_lat_trace[j as usize]; } ret = put_cmsg(msg, SOL_RDS, RDS_CMSG_RXPATH_LATENCY, core::mem::size_of_val(&t), &t as *const _ as *const _); }
    ret
}

unsafe fn rds_recvmsg_zcookie(rs: *mut rds_sock, msg: *mut msghdr) -> bool {
    let q = &mut (*rs).rs_zcookie_queue; let mut info: *mut rds_msg_zcopy_info = core::ptr::null_mut(); let mut flags = 0;
    if (*msg).msg_control.is_null() || !sock_flag(rds_rs_to_sk(rs), SOCK_ZEROCOPY) || (*msg).msg_controllen < CMSG_SPACE(core::mem::size_of::<rds_zcopy_cookies>()) { return false; }
    spin_lock_irqsave(&mut q.lock, &mut flags); if !list_empty(&q.zcookie_head) { info = list_entry(q.zcookie_head.next, rds_msg_zcopy_info, rs_zcookie_next); list_del(&mut (*info).rs_zcookie_next); } spin_unlock_irqrestore(&mut q.lock, flags);
    if info.is_null() { return false; }
    if put_cmsg(msg, SOL_RDS, RDS_CMSG_ZCOPY_COMPLETION, core::mem::size_of_val(&(*info).zcookies), &(*info).zcookies as *const _ as *const _) != 0 { spin_lock_irqsave(&mut q.lock, &mut flags); list_add(&mut (*info).rs_zcookie_next, &mut q.zcookie_head); spin_unlock_irqrestore(&mut q.lock, flags); return false; }
    kfree(info as *mut _); true
}

pub unsafe fn rds_recvmsg(sock: *mut socket, msg: *mut msghdr, size: usize, msg_flags: i32) -> isize {
    let sk = (*sock).sk; let rs = rds_sk_to_rs(sk); let nonblock = msg_flags & MSG_DONTWAIT != 0; let mut timeo = sock_rcvtimeo(sk, nonblock); let mut ret = 0; let mut inc: *mut rds_incoming = core::ptr::null_mut();
    if msg_flags & MSG_OOB != 0 { return ret; }
    if msg_flags & MSG_ERRQUEUE != 0 { return sock_recv_errqueue(sk, msg, size, SOL_IP, IP_RECVERR); }
    loop {
        if !list_empty(&(*rs).rs_notify_queue) { ret = rds_notify_queue_get(rs, msg); break; }
        if (*rs).rs_cong_notify != 0 { ret = rds_notify_cong(rs, msg); break; }
        if !rds_next_incoming(rs, &mut inc) {
            if nonblock { ret = if rds_recvmsg_zcookie(rs, msg) { 0 } else { -EAGAIN }; break; }
            timeo = wait_event_interruptible_timeout(*sk_sleep(sk), !list_empty(&(*rs).rs_notify_queue) || (*rs).rs_cong_notify != 0 || rds_next_incoming(rs, &mut inc), timeo);
            if timeo > 0 || timeo == MAX_SCHEDULE_TIMEOUT { continue; }
            ret = if timeo == 0 { -ETIMEDOUT } else { timeo }; break;
        }
        ret = ((*(*inc).i_conn).c_trans).inc_copy_to_user.unwrap()(inc, &mut (*msg).msg_iter);
        if ret < 0 { break; }
        if rds_still_queued(rs, inc, msg_flags & MSG_PEEK == 0) == 0 { rds_inc_put(inc); inc = core::ptr::null_mut(); rds_stats_inc(s_recv_deliver_raced); iov_iter_revert(&mut (*msg).msg_iter, ret as usize); continue; }
        if ret < be32_to_cpu((*inc).i_hdr.h_len) as isize { if msg_flags & MSG_TRUNC != 0 { ret = be32_to_cpu((*inc).i_hdr.h_len) as isize; } (*msg).msg_flags |= MSG_TRUNC; }
        if rds_cmsg_recv(inc, msg, rs) != 0 { ret = -EFAULT; break; }
        rds_recvmsg_zcookie(rs, msg); rds_stats_inc(s_recv_delivered); break;
    }
    if !inc.is_null() { rds_inc_put(inc); } ret
}

pub unsafe fn rds_clear_recv_queue(rs: *mut rds_sock) {
    let sk = rds_rs_to_sk(rs); let mut flags = 0; let mut to_drop: list_head = core::mem::zeroed(); INIT_LIST_HEAD(&mut to_drop);
    write_lock_irqsave(&mut (*rs).rs_recv_lock, &mut flags);
    let mut inc: *mut rds_incoming = core::ptr::null_mut(); let mut tmp: *mut rds_incoming = core::ptr::null_mut();
    list_for_each_entry_safe!(inc, tmp, &mut (*rs).rs_recv_queue, i_item, { rds_recv_rcvbuf_delta(rs, sk, (*inc).i_conn.as_ref().unwrap().c_lcong, -(be32_to_cpu((*inc).i_hdr.h_len) as i32), (*inc).i_hdr.h_dport); list_move(&mut (*inc).i_item, &mut to_drop); });
    write_unlock_irqrestore(&mut (*rs).rs_recv_lock, flags);
    list_for_each_entry_safe!(inc, tmp, &mut to_drop, i_item, { list_del_init(&mut (*inc).i_item); rds_inc_put(inc); });
}

pub unsafe fn rds_inc_info_copy(inc: *mut rds_incoming, iter: *mut rds_info_iterator, saddr: __be32, daddr: __be32, flip: i32) {
    let mut m: rds_info_message = core::mem::zeroed(); m.seq = be64_to_cpu((*inc).i_hdr.h_sequence); m.len = be32_to_cpu((*inc).i_hdr.h_len); m.tos = (*(*inc).i_conn).c_tos;
    if flip != 0 { m.laddr = daddr; m.faddr = saddr; m.lport = (*inc).i_hdr.h_dport; m.fport = (*inc).i_hdr.h_sport; } else { m.laddr = saddr; m.faddr = daddr; m.lport = (*inc).i_hdr.h_sport; m.fport = (*inc).i_hdr.h_dport; } m.flags = 0; rds_info_copy(iter, &m as *const _ as *const _, core::mem::size_of_val(&m));
}

// Preserved from #if IS_ENABLED(CONFIG_IPV6): IPv6-specific info export.
pub unsafe fn rds6_inc_info_copy(inc: *mut rds_incoming, iter: *mut rds_info_iterator, saddr: *mut in6_addr, daddr: *mut in6_addr, flip: i32) {
    let mut m: rds6_info_message = core::mem::zeroed(); m.seq = be64_to_cpu((*inc).i_hdr.h_sequence); m.len = be32_to_cpu((*inc).i_hdr.h_len); m.tos = (*(*inc).i_conn).c_tos;
    if flip != 0 { m.laddr = *daddr; m.faddr = *saddr; m.lport = (*inc).i_hdr.h_dport; m.fport = (*inc).i_hdr.h_sport; } else { m.laddr = *saddr; m.faddr = *daddr; m.lport = (*inc).i_hdr.h_sport; m.fport = (*inc).i_hdr.h_dport; } m.flags = 0; rds_info_copy(iter, &m as *const _ as *const _, core::mem::size_of_val(&m));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
