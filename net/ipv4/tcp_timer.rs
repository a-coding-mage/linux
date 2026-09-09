// SPDX-License-Identifier: GPL-2.0-only
/* Literal Rust translation of tcp_timer.c. Kernel declarations and macros are
 * supplied by the surrounding translated source files. */

unsafe fn tcp_clamp_rto_to_user_timeout(sk: *const sock) -> u32 {
    let icsk = inet_csk(sk);
    let tp = tcp_sk(sk);
    let user_timeout = READ_ONCE((*icsk).icsk_user_timeout);
    if user_timeout == 0 { return (*icsk).icsk_rto; }
    let mut elapsed = tcp_time_stamp_ts(tp).wrapping_sub((*tp).retrans_stamp);
    if (*tp).tcp_usec_ts { elapsed /= USEC_PER_MSEC; }
    let remaining = user_timeout as i32 - elapsed as i32;
    if remaining <= 0 { return 1; }
    min_t((*icsk).icsk_rto, msecs_to_jiffies(remaining as u32))
}

pub unsafe fn tcp_clamp_probe0_to_user_timeout(sk: *const sock, when: u32) -> u32 {
    let icsk = inet_csk(sk);
    let user_timeout = READ_ONCE((*icsk).icsk_user_timeout);
    if user_timeout == 0 || (*icsk).icsk_probes_tstamp == 0 { return when; }
    let mut elapsed = tcp_jiffies32.wrapping_sub((*icsk).icsk_probes_tstamp) as i32;
    if elapsed < 0 { elapsed = 0; }
    let remaining = max_t(msecs_to_jiffies(user_timeout) as i32 - elapsed, TCP_TIMEOUT_MIN);
    min_t(remaining as u32, when)
}

unsafe fn tcp_write_err(sk: *mut sock) {
    tcp_done_with_error(sk, if READ_ONCE((*sk).sk_err_soft) != 0 { (*sk).sk_err_soft } else { ETIMEDOUT });
    __NET_INC_STATS(sock_net(sk), LINUX_MIB_TCPABORTONTIMEOUT);
}

unsafe fn tcp_out_of_resources(sk: *mut sock, mut do_reset: bool) -> i32 {
    let tp = tcp_sk(sk); let mut shift = 0;
    if (tcp_jiffies32.wrapping_sub((*tp).lsndtime) as i32) > 2 * tcp_rto_max(sk) as i32 || !do_reset { shift += 1; }
    if READ_ONCE((*sk).sk_err_soft) != 0 { shift += 1; }
    if tcp_check_oom(sk, shift) {
        if (tcp_jiffies32.wrapping_sub((*tp).lsndtime) as i32) <= TCP_TIMEWAIT_LEN as i32 ||
           ((*tp).snd_wnd == 0 && (*tp).packets_out == 0) { do_reset = true; }
        if do_reset { tcp_send_active_reset(sk, GFP_ATOMIC, SK_RST_REASON_TCP_ABORT_ON_MEMORY); }
        tcp_done(sk); __NET_INC_STATS(sock_net(sk), LINUX_MIB_TCPABORTONMEMORY); return 1;
    }
    if !check_net(sock_net(sk)) { tcp_done(sk); return 1; }
    0
}

unsafe fn tcp_orphan_retries(sk: *mut sock, alive: bool) -> i32 {
    let mut retries = READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_orphan_retries);
    if READ_ONCE((*sk).sk_err_soft) != 0 && !alive { retries = 0; }
    if retries == 0 && alive { retries = 8; }
    retries
}

unsafe fn tcp_mtu_probing(icsk: *mut inet_connection_sock, sk: *mut sock) {
    let net = sock_net(sk);
    if READ_ONCE((*net).ipv4.sysctl_tcp_mtu_probing) == 0 { return; }
    if !(*icsk).icsk_mtup.enabled {
        (*icsk).icsk_mtup.enabled = 1; (*icsk).icsk_mtup.probe_timestamp = tcp_jiffies32;
    } else {
        let mut mss = tcp_mtu_to_mss(sk, (*icsk).icsk_mtup.search_low) >> 1;
        mss = min(READ_ONCE((*net).ipv4.sysctl_tcp_base_mss), mss);
        mss = max(mss, READ_ONCE((*net).ipv4.sysctl_tcp_mtu_probe_floor));
        mss = max(mss, READ_ONCE((*net).ipv4.sysctl_tcp_min_snd_mss));
        (*icsk).icsk_mtup.search_low = tcp_mss_to_mtu(sk, mss);
    }
    tcp_sync_mss(sk, (*icsk).icsk_pmtu_cookie);
}

unsafe fn tcp_model_timeout(sk: *mut sock, boundary: u32, rto_base: u32) -> u32 {
    let linear = ilog2(tcp_rto_max(sk) / rto_base);
    let timeout = if boundary <= linear { ((2u32 << boundary) - 1) * rto_base }
        else { ((2u32 << linear) - 1) * rto_base + (boundary - linear) * tcp_rto_max(sk) };
    jiffies_to_msecs(timeout)
}

unsafe fn retransmits_timed_out(sk: *mut sock, boundary: u32, mut timeout: u32) -> bool {
    let tp = tcp_sk(sk); if (*inet_csk(sk)).icsk_retransmits == 0 { return false; }
    let start = (*tp).retrans_stamp;
    if timeout == 0 {
        let mut base = TCP_RTO_MIN;
        if (1u32 << (*sk).sk_state) & (TCPF_SYN_SENT | TCPF_SYN_RECV) != 0 { base = tcp_timeout_init(sk); }
        timeout = tcp_model_timeout(sk, boundary, base);
    }
    if (*tp).tcp_usec_ts {
        let delta = (*tp).tcp_mstamp.wrapping_sub(start) + jiffies_to_usecs(1);
        return (delta as i32 - (timeout * USEC_PER_MSEC) as i32) >= 0;
    }
    (tcp_time_stamp_ts(tp).wrapping_sub(start) as i32 - timeout as i32) >= 0
}

unsafe fn tcp_write_timeout(sk: *mut sock) -> i32 {
    let icsk = inet_csk(sk); let tp = tcp_sk(sk); let net = sock_net(sk);
    let mut expired = false; let mut retry_until; let mut max_retransmits;
    if (1u32 << (*sk).sk_state) & (TCPF_SYN_SENT | TCPF_SYN_RECV) != 0 {
        if (*icsk).icsk_retransmits != 0 { __dst_negative_advice(sk); }
        retry_until = if READ_ONCE((*icsk).icsk_syn_retries) != 0 { READ_ONCE((*icsk).icsk_syn_retries) } else { READ_ONCE((*net).ipv4.sysctl_tcp_syn_retries) };
        max_retransmits = retry_until;
        if (*sk).sk_state == TCP_SYN_SENT { max_retransmits += READ_ONCE((*net).ipv4.sysctl_tcp_syn_linear_timeouts); }
        expired = (*icsk).icsk_retransmits >= max_retransmits;
    } else {
        if retransmits_timed_out(sk, READ_ONCE((*net).ipv4.sysctl_tcp_retries1), 0) { tcp_mtu_probing(icsk, sk); __dst_negative_advice(sk); }
        retry_until = READ_ONCE((*net).ipv4.sysctl_tcp_retries2);
        if sock_flag(sk, SOCK_DEAD) {
            let alive = (*icsk).icsk_rto < tcp_rto_max(sk); retry_until = tcp_orphan_retries(sk, alive);
            let do_reset = alive || !retransmits_timed_out(sk, retry_until as u32, 0);
            if tcp_out_of_resources(sk, do_reset) != 0 { return 1; }
        }
    }
    if !expired { expired = retransmits_timed_out(sk, retry_until as u32, READ_ONCE((*icsk).icsk_user_timeout)); }
    tcp_fastopen_active_detect_blackhole(sk, expired); mptcp_active_detect_blackhole(sk, expired);
    if BPF_SOCK_OPS_TEST_FLAG(tp, BPF_SOCK_OPS_RTO_CB_FLAG) { tcp_call_bpf_3arg(sk, BPF_SOCK_OPS_RTO_CB, (*icsk).icsk_retransmits, (*icsk).icsk_rto, expired as i32); }
    if expired { tcp_write_err(sk); return 1; }
    if __sk_rethink_txhash_reset_dst(sk) { (*tp).timeout_rehash += 1; __NET_INC_STATS(sock_net(sk), LINUX_MIB_TCPTIMEOUTREHASH); }
    0
}

pub unsafe fn tcp_delack_timer_handler(sk: *mut sock) {
    let icsk = inet_csk(sk); let tp = tcp_sk(sk);
    if (1u32 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_LISTEN) != 0 { return; }
    if (*tp).compressed_ack != 0 { tcp_mstamp_refresh(tp); tcp_sack_compress_send_ack(sk); return; }
    if (*icsk).icsk_ack.pending & ICSK_ACK_TIMER == 0 { return; }
    if time_after(icsk_delack_timeout(icsk), jiffies) { sk_reset_timer(sk, &mut (*icsk).icsk_delack_timer, icsk_delack_timeout(icsk)); return; }
    (*icsk).icsk_ack.pending &= !ICSK_ACK_TIMER;
    if inet_csk_ack_scheduled(sk) {
        if !inet_csk_in_pingpong_mode(sk) { (*icsk).icsk_ack.ato = min3((*icsk).icsk_ack.ato << 1, (*icsk).icsk_rto, TCP_DELACK_MAX); }
        else { inet_csk_exit_pingpong_mode(sk); (*icsk).icsk_ack.ato = TCP_ATO_MIN; }
        tcp_mstamp_refresh(tp); tcp_send_ack(sk); __NET_INC_STATS(sock_net(sk), LINUX_MIB_DELAYEDACKS);
    }
}

unsafe fn tcp_update_rto_stats(sk: *mut sock) { let i = inet_csk(sk); let tp = tcp_sk(sk); if (*i).icsk_retransmits == 0 { (*tp).total_rto_recoveries += 1; (*tp).rto_stamp = tcp_time_stamp_ms(tp); } (*i).icsk_retransmits += 1; (*tp).total_rto += 1; }

unsafe fn tcp_probe_timer(sk: *mut sock) {
    let i = inet_csk(sk); let tp = tcp_sk(sk); let skb = tcp_send_head(sk);
    if (*tp).packets_out != 0 || skb.is_null() { (*i).icsk_probes_out = 0; (*i).icsk_probes_tstamp = 0; return; }
    if (*i).icsk_probes_tstamp == 0 { (*i).icsk_probes_tstamp = tcp_jiffies32; }
    else { let user = READ_ONCE((*i).icsk_user_timeout); if user != 0 && tcp_jiffies32.wrapping_sub((*i).icsk_probes_tstamp) >= msecs_to_jiffies(user) { tcp_write_err(sk); return; } }
    let mut max = READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_retries2);
    if sock_flag(sk, SOCK_DEAD) { let rto = tcp_rto_max(sk); let alive = inet_csk_rto_backoff(i, rto) < rto; max = tcp_orphan_retries(sk, alive); if !alive && (*i).icsk_backoff >= max { tcp_write_err(sk); return; } if tcp_out_of_resources(sk, true) != 0 { return; } }
    if (*i).icsk_probes_out >= max { tcp_write_err(sk); } else { tcp_send_probe0(sk); }
}

unsafe fn tcp_fastopen_synack_timer(sk: *mut sock, req: *mut request_sock) {
    let i = inet_csk(sk); let tp = tcp_sk(sk); tcp_syn_ack_timeout(req);
    let max = if READ_ONCE((*i).icsk_syn_retries) != 0 { READ_ONCE((*i).icsk_syn_retries) } else { READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_synack_retries) + 1 };
    if (*req).num_timeout >= max { tcp_write_err(sk); return; }
    if (*i).icsk_retransmits == 1 { tcp_enter_loss(sk); }
    tcp_rtx_synack(sk, req); if (*req).num_retrans > 1 && (*tcp_rsk(req)).accecn_ok { (*tcp_rsk(req)).accecn_fail_mode |= TCP_ACCECN_ACE_FAIL_SEND; }
    (*req).num_timeout += 1; tcp_update_rto_stats(sk); if (*tp).retrans_stamp == 0 { (*tp).retrans_stamp = tcp_time_stamp_ts(tp); }
    tcp_reset_xmit_timer(sk, ICSK_TIME_RETRANS, (*req).timeout << (*req).num_timeout, false);
}

pub unsafe fn tcp_retransmit_timer(sk: *mut sock) {
    let tp = tcp_sk(sk); let i = inet_csk(sk); let req = rcu_dereference_protected((*tp).fastopen_rsk, lockdep_sock_is_held(sk));
    if !req.is_null() { tcp_fastopen_synack_timer(sk, req); return; }
    if (*tp).packets_out == 0 { return; }
    let skb = tcp_rtx_queue_head(sk); if skb.is_null() { return; }
    if (*tp).snd_wnd == 0 && !sock_flag(sk, SOCK_DEAD) && (1u32 << (*sk).sk_state) & (TCPF_SYN_SENT | TCPF_SYN_RECV) == 0 {
        let delta = tcp_time_stamp_ts(tp).wrapping_sub(if (*tp).retrans_stamp != 0 { (*tp).retrans_stamp } else { tcp_skb_timestamp_ts((*tp).tcp_usec_ts, skb) });
        if tcp_rtx_probe0_timed_out(sk, skb, if (*tp).tcp_usec_ts { delta / USEC_PER_MSEC } else { delta }) { tcp_write_err(sk); return; }
        tcp_enter_loss(sk); tcp_retransmit_skb(sk, skb, 1); __sk_dst_reset(sk); return;
    }
    __NET_INC_STATS(sock_net(sk), LINUX_MIB_TCPTIMEOUTS); if tcp_write_timeout(sk) != 0 { return; }
    tcp_enter_loss(sk); tcp_update_rto_stats(sk); if tcp_retransmit_skb(sk, tcp_rtx_queue_head(sk), 1) > 0 { tcp_reset_xmit_timer(sk, ICSK_TIME_RETRANS, TCP_RESOURCE_PROBE_INTERVAL, false); return; }
    if (*sk).sk_state == TCP_ESTABLISHED && ((*tp).thin_lto || READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_thin_linear_timeouts)) && tcp_stream_is_thin(tp) && (*i).icsk_retransmits <= TCP_THIN_LINEAR_RETRIES { (*i).icsk_backoff = 0; (*i).icsk_rto = clamp(__tcp_set_rto(tp), tcp_rto_min(sk), tcp_rto_max(sk)); }
    else if (*sk).sk_state != TCP_SYN_SENT || (*tp).total_rto > READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_syn_linear_timeouts) { (*i).icsk_backoff += 1; (*i).icsk_rto = min((*i).icsk_rto << 1, tcp_rto_max(sk)); }
    tcp_reset_xmit_timer(sk, ICSK_TIME_RETRANS, tcp_clamp_rto_to_user_timeout(sk), false);
}

pub unsafe fn tcp_write_timer_handler(sk: *mut sock) { let i = inet_csk(sk); if (1u32 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_LISTEN) != 0 || (*i).icsk_pending == 0 { return; } if time_after(tcp_timeout_expires(sk), jiffies) { sk_reset_timer(sk, &mut (*sk).tcp_retransmit_timer, tcp_timeout_expires(sk)); return; } tcp_mstamp_refresh(tcp_sk(sk)); match (*i).icsk_pending { ICSK_TIME_REO_TIMEOUT => tcp_rack_reo_timeout(sk), ICSK_TIME_LOSS_PROBE => tcp_send_loss_probe(sk), ICSK_TIME_RETRANS => { (*i).icsk_pending = 0; tcp_retransmit_timer(sk); }, ICSK_TIME_PROBE0 => { (*i).icsk_pending = 0; tcp_probe_timer(sk); }, _ => {} } }

unsafe fn tcp_rtx_probe0_timed_out(sk: *const sock, _skb: *const sk_buff, rtx_delta: u32) -> bool {
    let i = inet_csk(sk); let tp = tcp_sk(sk); let user = READ_ONCE((*i).icsk_user_timeout); let mut timeout = tcp_rto_max(sk) * 2;
    if user != 0 { if rtx_delta > user { return true; } timeout = min(timeout, msecs_to_jiffies(user)); }
    let rcv_delta = tcp_timeout_expires(sk) as i32 - (*tp).rcv_tstamp as i32;
    if rcv_delta <= timeout as i32 { return false; }
    msecs_to_jiffies(rtx_delta) > timeout
}

pub unsafe fn tcp_reset_keepalive_timer(sk: *mut sock, len: u64) { sk_reset_timer(sk, &mut (*inet_csk(sk)).icsk_keepalive_timer, jiffies + len); }
unsafe fn tcp_delete_keepalive_timer(sk: *mut sock) { sk_stop_timer(sk, &mut (*inet_csk(sk)).icsk_keepalive_timer); }
pub unsafe fn tcp_set_keepalive(sk: *mut sock, val: i32) { if (1u32 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_LISTEN) != 0 { return; } if val != 0 && !sock_flag(sk, SOCK_KEEPOPEN) { tcp_reset_keepalive_timer(sk, keepalive_time_when(tcp_sk(sk)) as u64); } else if val == 0 { tcp_delete_keepalive_timer(sk); } }

pub unsafe fn tcp_syn_ack_timeout(req: *const request_sock) { let net = read_pnet(&(*inet_rsk(req)).ireq_net); __NET_INC_STATS(net, LINUX_MIB_TCPTIMEOUTS); }

pub unsafe fn tcp_init_xmit_timers(sk: *mut sock) {
    inet_csk_init_xmit_timers(sk, tcp_write_timer, tcp_delack_timer, tcp_keepalive_timer);
    hrtimer_setup(&mut (*tcp_sk(sk)).pacing_timer, tcp_pace_kick, CLOCK_MONOTONIC, HRTIMER_MODE_ABS_PINNED_SOFT);
    hrtimer_setup(&mut (*tcp_sk(sk)).compressed_ack_timer, tcp_compressed_ack_kick, CLOCK_MONOTONIC, HRTIMER_MODE_REL_PINNED_SOFT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
