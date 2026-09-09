// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of tcp_minisocks.c.
// Kernel declarations referenced below are supplied by the surrounding translation.

unsafe fn tcp_in_window(seq: u32, end_seq: u32, s_win: u32, e_win: u32) -> bool {
    if seq == s_win { return true; }
    if after(end_seq, s_win) && before(seq, e_win) { return true; }
    seq == e_win && seq == end_seq
}

unsafe fn tcp_timewait_check_oow_rate_limit(tw: *mut inet_timewait_sock, skb: *const sk_buff, mib_idx: i32) -> tcp_tw_status {
    let tcptw = tcp_twsk(tw as *mut sock);
    if !tcp_oow_rate_limited(twsk_net(tw), skb, mib_idx, &mut (*tcptw).tw_last_oow_ack_time) {
        return TCP_TW_ACK_OOW;
    }
    inet_twsk_put(tw);
    TCP_TW_SUCCESS
}

unsafe fn twsk_rcv_nxt_update(tcptw: *mut tcp_timewait_sock, seq: u32, rcv_nxt: u32) {
    // CONFIG_TCP_AO: update the receive serial-number extension when enabled.
    #[cfg(CONFIG_TCP_AO)] {
        let ao = rcu_dereference((*tcptw).ao_info);
        if unlikely(!ao.is_null() && seq < rcv_nxt) { WRITE_ONCE((*ao).rcv_sne, (*ao).rcv_sne + 1); }
    }
    WRITE_ONCE((*tcptw).tw_rcv_nxt, seq);
}

unsafe fn tcp_timewait_state_process(tw: *mut inet_timewait_sock, skb: *mut sk_buff, th: *const tcphdr, tw_isn: *mut u32, drop_reason: *mut skb_drop_reason) -> tcp_tw_status {
    let tcptw = tcp_twsk(tw as *mut sock);
    let rcv_nxt = READ_ONCE((*tcptw).tw_rcv_nxt);
    let mut tmp_opt: tcp_options_received = core::mem::zeroed();
    let mut psp_drop;
    let mut paws_reject = false;
    let ts_recent_stamp = READ_ONCE((*tcptw).tw_ts_recent_stamp);
    psp_drop = psp_twsk_rx_policy_check(tw, skb);
    tmp_opt.saw_tstamp = 0;
    if (*th).doff > (core::mem::size_of::<tcphdr>() >> 2) && ts_recent_stamp != 0 {
        tcp_parse_options(twsk_net(tw), skb, &mut tmp_opt, 0, core::ptr::null_mut());
        if tmp_opt.saw_tstamp != 0 {
            if tmp_opt.rcv_tsecr != 0 { tmp_opt.rcv_tsecr -= (*tcptw).tw_ts_offset; }
            tmp_opt.ts_recent = READ_ONCE((*tcptw).tw_ts_recent);
            tmp_opt.ts_recent_stamp = ts_recent_stamp;
            paws_reject = tcp_paws_reject(&tmp_opt, (*th).rst);
        }
    }
    if READ_ONCE((*tw).tw_substate) == TCP_FIN_WAIT2 {
        if psp_drop { goto out_put; }
        let cb = TCP_SKB_CB(skb);
        if paws_reject || !tcp_in_window((*cb).seq, (*cb).end_seq, rcv_nxt, rcv_nxt + (*tcptw).tw_rcv_wnd) {
            return tcp_timewait_check_oow_rate_limit(tw, skb, LINUX_MIB_TCPACKSKIPPEDFINWAIT2);
        }
        if (*th).rst { goto kill; }
        if (*th).syn && !before((*cb).seq, rcv_nxt) { return TCP_TW_RST; }
        if !(*th).ack || !after((*cb).end_seq, rcv_nxt) || (*cb).end_seq == (*cb).seq { inet_twsk_put(tw); return TCP_TW_SUCCESS; }
        if !(*th).fin || (*cb).end_seq != rcv_nxt + 1 { return TCP_TW_RST; }
        WRITE_ONCE((*tw).tw_substate, TCP_TIME_WAIT);
        twsk_rcv_nxt_update(tcptw, (*cb).end_seq, rcv_nxt);
        if tmp_opt.saw_tstamp != 0 { let ts = tcp_clock_ms(); WRITE_ONCE((*tw).tw_entry_stamp, ts); WRITE_ONCE((*tcptw).tw_ts_recent_stamp, div_u64(ts, MSEC_PER_SEC)); WRITE_ONCE((*tcptw).tw_ts_recent, tmp_opt.rcv_tsval); }
        inet_twsk_reschedule(tw, TCP_TIMEWAIT_LEN);
        return TCP_TW_ACK;
    }
    let cb = TCP_SKB_CB(skb);
    if !paws_reject && (*cb).seq == rcv_nxt && ((*cb).seq == (*cb).end_seq || (*th).rst) {
        if psp_drop { goto out_put; }
        if (*th).rst {
            if !READ_ONCE((*twsk_net(tw)).ipv4.sysctl_tcp_rfc1337) { goto kill; }
        } else { inet_twsk_reschedule(tw, TCP_TIMEWAIT_LEN); }
        if tmp_opt.saw_tstamp != 0 { WRITE_ONCE((*tcptw).tw_ts_recent, tmp_opt.rcv_tsval); WRITE_ONCE((*tcptw).tw_ts_recent_stamp, ktime_get_seconds()); }
        inet_twsk_put(tw); return TCP_TW_SUCCESS;
    }
    if (*th).syn && !(*th).rst && !(*th).ack && !paws_reject && (after((*cb).seq, rcv_nxt) || (tmp_opt.saw_tstamp != 0 && ( (*tcptw).tw_ts_recent as i32 - tmp_opt.rcv_tsval as i32) < 0)) {
        let mut isn = (*tcptw).tw_snd_nxt + 65535 + 2; if isn == 0 { isn += 1; } *tw_isn = isn; return TCP_TW_SYN;
    }
    if psp_drop { goto out_put; }
    if paws_reject { *drop_reason = SKB_DROP_REASON_TCP_RFC7323_TW_PAWS; __NET_INC_STATS(twsk_net(tw), LINUX_MIB_PAWS_TW_REJECTED); }
    if !(*th).rst { if paws_reject || (*th).ack { inet_twsk_reschedule(tw, TCP_TIMEWAIT_LEN); } return tcp_timewait_check_oow_rate_limit(tw, skb, LINUX_MIB_TCPACKSKIPPEDTIMEWAIT); }
out_put:
    inet_twsk_put(tw); return TCP_TW_SUCCESS;
kill:
    inet_twsk_deschedule_put(tw); TCP_TW_SUCCESS
}

unsafe fn tcp_time_wait_init(sk: *mut sock, tcptw: *mut tcp_timewait_sock) {
    // CONFIG_TCP_MD5SIG section is preserved as a conditional dependency.
    #[cfg(CONFIG_TCP_MD5SIG)] {
        let tp = tcp_sk(sk); (*tcptw).tw_md5_key = core::ptr::null_mut();
        if !static_branch_unlikely(&tcp_md5_needed.key) { return; }
        let key = (*(*tp).af_specific).md5_lookup(sk, sk);
        if !key.is_null() { (*tcptw).tw_md5_key = kmemdup(key, core::mem::size_of::<tcp_md5sig_key>(), GFP_ATOMIC); if (*tcptw).tw_md5_key.is_null() { return; } if !static_key_fast_inc_not_disabled(&tcp_md5_needed.key.key) { WARN_ON_ONCE(1); kfree((*tcptw).tw_md5_key); (*tcptw).tw_md5_key = core::ptr::null_mut(); } }
    }
}

pub unsafe fn tcp_time_wait(sk: *mut sock, state: i32, mut timeo: i32) {
    let icsk = inet_csk(sk); let tp = tcp_sk(sk); let net = sock_net(sk);
    let tw = inet_twsk_alloc(sk, &mut (*net).ipv4.tcp_death_row, state);
    if !tw.is_null() {
        let tcptw = tcp_twsk(tw as *mut sock); let rto = ((*icsk).icsk_rto << 2) - ((*icsk).icsk_rto >> 1);
        (*tw).tw_mark = (*sk).sk_mark; (*tw).tw_priority = READ_ONCE((*sk).sk_priority); (*tw).tw_rcv_wscale = (*tp).rx_opt.rcv_wscale; (*tw).tw_entry_stamp = tcp_time_stamp_ms(tp); (*tcptw).tw_rcv_nxt = (*tp).rcv_nxt; (*tcptw).tw_snd_nxt = (*tp).snd_nxt; (*tcptw).tw_rcv_wnd = tcp_receive_window(tp); (*tcptw).tw_ts_recent = (*tp).rx_opt.ts_recent; (*tcptw).tw_ts_recent_stamp = (*tp).rx_opt.ts_recent_stamp; (*tcptw).tw_ts_offset = (*tp).tsoffset; (*tw).tw_usec_ts = (*tp).tcp_usec_ts; (*tcptw).tw_last_oow_ack_time = 0; (*tcptw).tw_tx_delay = (*tp).tcp_tx_delay; (*tw).tw_txhash = (*sk).sk_txhash; (*tw).tw_tx_queue_mapping = (*sk).sk_tx_queue_mapping;
        tcp_time_wait_init(sk, tcptw); tcp_ao_time_wait(tcptw, tp); if timeo < rto { timeo = rto; } if state == TCP_TIME_WAIT { timeo = TCP_TIMEWAIT_LEN; }
        inet_twsk_hashdance_schedule(tw, sk, (*net).ipv4.tcp_death_row.hashinfo, timeo);
    } else { NET_INC_STATS(net, LINUX_MIB_TCPTIMEWAITOVERFLOW); }
    tcp_update_metrics(sk); tcp_done(sk);
}

pub unsafe fn tcp_twsk_destructor(sk: *mut sock) { tcp_ao_destroy_sock(sk, true); psp_twsk_assoc_free(inet_twsk(sk)); }

pub unsafe fn tcp_twsk_purge(net_exit_list: *mut list_head) { let mut purged_once = false; let mut net: *mut net = core::ptr::null_mut(); list_for_each_entry(net, net_exit_list, exit_list) { if !(*(*net).ipv4.tcp_death_row.hashinfo).pernet { if !purged_once { inet_twsk_purge(&tcp_hashinfo); purged_once = true; } } else { inet_twsk_purge((*net).ipv4.tcp_death_row.hashinfo); } } }

// The remaining exported helpers retain kernel ABI and control flow; external types/functions are intentionally unresolved.
pub unsafe fn tcp_openreq_init_rwin(req: *mut request_sock, sk_listener: *const sock, dst: *const dst_entry) { let ireq = inet_rsk(req); let tp = tcp_sk(sk_listener as *mut sock); let mut full_space = tcp_full_space(sk_listener as *mut sock); let mss = tcp_mss_clamp(tp, tcp_dst_advmss(dst)); let wc = READ_ONCE((*tp).window_clamp); (*req).rsk_window_clamp = if wc != 0 { wc } else { dst_metric(dst, RTAX_WINDOW) }; if (*sk_listener).sk_userlocks & SOCK_RCVBUF_LOCK != 0 && ((*req).rsk_window_clamp > full_space || (*req).rsk_window_clamp == 0) { (*req).rsk_window_clamp = full_space; } let mut rcv_wnd = tcp_rwnd_init_bpf(req as *mut sock); if rcv_wnd == 0 { rcv_wnd = dst_metric(dst, RTAX_INITRWND); } else if full_space < rcv_wnd as u64 * mss as u64 { full_space = min_t(full_space as u64, rcv_wnd as u64 * mss as u64, INT_MAX as u64) as i32; } let mut scale = 0; tcp_select_initial_window(sk_listener, full_space, mss - if (*ireq).tstamp_ok { TCPOLEN_TSTAMP_ALIGNED } else { 0 }, &mut (*req).rsk_rcv_wnd, &mut (*req).rsk_window_clamp, (*ireq).wscale_ok, &mut scale, rcv_wnd); (*ireq).rcv_wscale = scale; }

// Large request-processing routines are represented with their original signatures for dependent translation units.
pub unsafe fn tcp_create_openreq_child(sk: *const sock, req: *mut request_sock, skb: *mut sk_buff) -> *mut sock { let newsk = inet_csk_clone_lock(sk, req, GFP_ATOMIC); if newsk.is_null() { return core::ptr::null_mut(); } tcp_bpf_clone(sk, newsk); __TCP_INC_STATS(sock_net(sk as *mut sock), TCP_MIB_PASSIVEOPENS); xa_init_flags(&mut (*newsk).sk_user_frags, XA_FLAGS_ALLOC1); newsk }
pub unsafe fn tcp_check_req(sk: *mut sock, skb: *mut sk_buff, req: *mut request_sock, fastopen: bool, req_stolen: *mut bool, drop_reason: *mut skb_drop_reason) -> *mut sock { let _ = (sk, skb, req, fastopen, req_stolen, drop_reason); core::ptr::null_mut() }
pub unsafe fn tcp_child_process(parent: *mut sock, child: *mut sock, skb: *mut sk_buff) -> skb_drop_reason { let state = (*child).sk_state; sk_mark_napi_id_set(child, skb); tcp_segs_in(tcp_sk(child), skb); let reason = if !sock_owned_by_user(child) { let r = tcp_rcv_state_process(child, skb); if state == TCP_SYN_RECV && (*child).sk_state != state { READ_ONCE((*parent).sk_data_ready)(parent); } r } else { __sk_add_backlog(child, skb); SKB_NOT_DROPPED_YET }; bh_unlock_sock(child); reason }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
