// SPDX-License-Identifier: GPL-2.0-or-later
/* Syncookies implementation for the Linux kernel. */

static mut syncookie_secret: [siphash_aligned_key_t; 2] = [unsafe { core::mem::zeroed() }; 2];

const COOKIEBITS: u32 = 24;
const COOKIEMASK: u32 = (1u32 << COOKIEBITS) - 1;
const TS_OPT_WSCALE_MASK: u32 = 0xf;
const TS_OPT_SACK: u32 = 1 << 4;
const TS_OPT_ECN: u32 = 1 << 5;
const TSBITS: u32 = 6;

unsafe fn cookie_hash(saddr: __be32, daddr: __be32, sport: __be16, dport: __be16,
                     count: u32, c: i32) -> u32 {
    net_get_random_once(syncookie_secret.as_mut_ptr() as *mut _, core::mem::size_of_val(&syncookie_secret));
    siphash_4u32(saddr as u32, daddr as u32,
                 ((sport as u32) << 16) | dport as u32, count,
                 &syncookie_secret[c as usize])
}

pub unsafe fn cookie_init_timestamp(req: *mut request_sock, now: u64) -> u64 {
    let ireq = inet_rsk(req);
    let ts_now = tcp_ns_to_ts(false, now);
    let mut options: u32 = if (*ireq).wscale_ok { (*ireq).snd_wscale as u32 } else { TS_OPT_WSCALE_MASK };
    if (*ireq).sack_ok != 0 { options |= TS_OPT_SACK; }
    if (*ireq).ecn_ok != 0 { options |= TS_OPT_ECN; }
    let mut ts = (ts_now >> TSBITS) << TSBITS;
    ts |= options as u64;
    if ts > ts_now { ts -= 1u64 << TSBITS; }
    if (*tcp_rsk(req)).req_usec_ts { ts * NSEC_PER_USEC as u64 } else { ts * NSEC_PER_MSEC as u64 }
}

unsafe fn secure_tcp_syn_cookie(saddr: __be32, daddr: __be32, sport: __be16,
                                dport: __be16, sseq: u32, data: u32) -> u32 {
    let count = tcp_cookie_time();
    cookie_hash(saddr, daddr, sport, dport, 0, 0)
        .wrapping_add(sseq).wrapping_add(count << COOKIEBITS)
        .wrapping_add(cookie_hash(saddr, daddr, sport, dport, count, 1).wrapping_add(data) & COOKIEMASK)
}

unsafe fn check_tcp_syn_cookie(mut cookie: u32, saddr: __be32, daddr: __be32,
                               sport: __be16, dport: __be16, sseq: u32) -> u32 {
    let count = tcp_cookie_time();
    cookie = cookie.wrapping_sub(cookie_hash(saddr, daddr, sport, dport, 0, 0).wrapping_add(sseq));
    let diff = (count.wrapping_sub(cookie >> COOKIEBITS)) & (u32::MAX >> COOKIEBITS);
    if diff >= MAX_SYNCOOKIE_AGE { return u32::MAX; }
    cookie.wrapping_sub(cookie_hash(saddr, daddr, sport, dport, count.wrapping_sub(diff), 1)) & COOKIEMASK
}

static msstab: [u16; 4] = [536, 1300, 1440, 1460];

pub unsafe fn __cookie_v4_init_sequence(iph: *const iphdr, th: *const tcphdr, mssp: *mut u16) -> u32 {
    let mss = *mssp;
    let mut mssind = msstab.len() - 1;
    while mssind != 0 && mss < msstab[mssind] { mssind -= 1; }
    *mssp = msstab[mssind];
    secure_tcp_syn_cookie((*iph).saddr, (*iph).daddr, (*th).source, (*th).dest,
                          ntohl((*th).seq), mssind as u32)
}

pub unsafe fn cookie_v4_init_sequence(skb: *const sk_buff, mssp: *mut u16) -> u32 {
    __cookie_v4_init_sequence(ip_hdr(skb), tcp_hdr(skb), mssp)
}

pub unsafe fn __cookie_v4_check(iph: *const iphdr, th: *const tcphdr) -> i32 {
    let cookie = ntohl((*th).ack_seq).wrapping_sub(1);
    let seq = ntohl((*th).seq).wrapping_sub(1);
    let mssind = check_tcp_syn_cookie(cookie, (*iph).saddr, (*iph).daddr, (*th).source, (*th).dest, seq);
    if (mssind as usize) < msstab.len() { msstab[mssind as usize] as i32 } else { 0 }
}

pub unsafe fn cookie_timestamp_decode(net: *const net, tcp_opt: *mut tcp_options_received) -> bool {
    let options = (*tcp_opt).rcv_tsecr;
    if !(*tcp_opt).saw_tstamp { tcp_clear_options(tcp_opt); return true; }
    if READ_ONCE((*net).ipv4.sysctl_tcp_timestamps) == 0 { return false; }
    (*tcp_opt).sack_ok = if options & TS_OPT_SACK != 0 { TCP_SACK_SEEN } else { 0 };
    if (*tcp_opt).sack_ok != 0 && READ_ONCE((*net).ipv4.sysctl_tcp_sack) == 0 { return false; }
    if options & TS_OPT_WSCALE_MASK == TS_OPT_WSCALE_MASK { return true; }
    (*tcp_opt).wscale_ok = 1;
    (*tcp_opt).snd_wscale = (options & TS_OPT_WSCALE_MASK) as _;
    READ_ONCE((*net).ipv4.sysctl_tcp_window_scaling) != 0
}

// The remaining request-socket routines retain the kernel ABI and operations.
// Their declarations are translated below; dependent kernel types/functions are external.

pub unsafe fn cookie_tcp_reqsk_alloc(ops: *const request_sock_ops, sk: *mut sock, skb: *mut sk_buff,
                                     tcp_opt: *const tcp_options_received, mss: i32, tsoff: u32) -> *mut request_sock {
    let req = if sk_is_mptcp(sk) { mptcp_subflow_reqsk_alloc(ops, sk, false) } else { inet_reqsk_alloc(ops, sk, false) };
    if req.is_null() { return core::ptr::null_mut(); }
    if cookie_tcp_reqsk_init(sk, skb, req) != 0 { reqsk_free(req); return core::ptr::null_mut(); }
    let ireq = inet_rsk(req); let treq = tcp_rsk(req);
    (*req).mss = mss; (*req).ts_recent = if (*tcp_opt).saw_tstamp { (*tcp_opt).rcv_tsval } else { 0 };
    (*ireq).snd_wscale = (*tcp_opt).snd_wscale; (*ireq).tstamp_ok = (*tcp_opt).saw_tstamp;
    (*ireq).sack_ok = (*tcp_opt).sack_ok; (*ireq).wscale_ok = (*tcp_opt).wscale_ok;
    (*ireq).ecn_ok = if (*tcp_opt).rcv_tsecr & TS_OPT_ECN != 0 { 1 } else { 0 };
    (*treq).req_usec_ts = false; (*treq).ts_off = tsoff; req
}

unsafe fn cookie_tcp_reqsk_init(sk: *mut sock, skb: *mut sk_buff, req: *mut request_sock) -> i32 {
    let ireq = inet_rsk(req); let treq = tcp_rsk(req); let th = tcp_hdr(skb);
    (*req).num_retrans = 0; (*ireq).ir_num = ntohs((*th).dest); (*ireq).ir_rmt_port = (*th).source;
    (*ireq).ir_iif = inet_request_bound_dev_if(sk, skb); (*ireq).ir_mark = inet_request_mark(sk, skb);
    (*treq).snt_synack = 0; (*treq).snt_tsval_first = 0; (*treq).tfo_listener = false;
    (*treq).rcv_isn = ntohl((*th).seq).wrapping_sub(1); (*treq).snt_isn = ntohl((*th).ack_seq).wrapping_sub(1);
    (*treq).txhash = (*treq).snt_isn; (*treq).syn_tos = TCP_SKB_CB(skb).ip_dsfield;
    if IS_ENABLED(CONFIG_MPTCP) { (*treq).is_mptcp = sk_is_mptcp(sk); if (*treq).is_mptcp { return mptcp_subflow_init_cookie_req(req, sk, skb); } }
    0
}

pub unsafe fn tcp_get_cookie_sock(sk: *mut sock, skb: *mut sk_buff, req: *mut request_sock, dst: *mut dst_entry) -> *mut sock {
    let icsk = inet_csk(sk); let mut own_req = false;
    let child = ((*(*icsk).icsk_af_ops).syn_recv_sock)(sk, skb, req, dst, core::ptr::null_mut(), &mut own_req, core::ptr::null_mut());
    if !child.is_null() { refcount_set(&mut (*req).rsk_refcnt, 1); sock_rps_save_rxhash(child, skb);
        if rsk_drop_req(req) { reqsk_put(req); return child; }
        if inet_csk_reqsk_queue_add(sk, req, child) { return child; }
        bh_unlock_sock(child); sock_put(child);
    }
    __reqsk_free(req); core::ptr::null_mut()
}

unsafe fn cookie_tcp_check(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> *mut request_sock {
    let mut tcp_opt: tcp_options_received = core::mem::zeroed(); let mut tsoff = 0;
    if tcp_synq_no_recent_overflow(sk) { return ERR_PTR(-EINVAL); }
    let mss = __cookie_v4_check(ip_hdr(skb), tcp_hdr(skb));
    if mss == 0 { __NET_INC_STATS(net, LINUX_MIB_SYNCOOKIESFAILED); return ERR_PTR(-EINVAL); }
    __NET_INC_STATS(net, LINUX_MIB_SYNCOOKIESRECV); tcp_parse_options(net, skb, &mut tcp_opt, 0, core::ptr::null_mut());
    if tcp_opt.saw_tstamp && tcp_opt.rcv_tsecr != 0 { let st = secure_tcp_seq_and_ts_off(net, (*ip_hdr(skb)).daddr, (*ip_hdr(skb)).saddr, (*tcp_hdr(skb)).dest, (*tcp_hdr(skb)).source); tsoff = st.ts_off; tcp_opt.rcv_tsecr -= tsoff; }
    if !cookie_timestamp_decode(net, &mut tcp_opt) { return ERR_PTR(-EINVAL); }
    cookie_tcp_reqsk_alloc(&tcp_request_sock_ops, sk, skb, &tcp_opt, mss, tsoff)
}

pub unsafe fn cookie_v4_check(sk: *mut sock, skb: *mut sk_buff) -> *mut sock {
    let net = sock_net(sk); let th = tcp_hdr(skb); let mut ret = sk;
    if READ_ONCE((*net).ipv4.sysctl_tcp_syncookies) == 0 || !(*th).ack || (*th).rst { return ret; }
    let req = cookie_tcp_check(net, sk, skb); if req.is_null() || IS_ERR(req) { return ret; }
    let ireq = inet_rsk(req); let treq = tcp_rsk(req);
    sk_rcv_saddr_set(req_to_sk(req), (*ip_hdr(skb)).daddr); sk_daddr_set(req_to_sk(req), (*ip_hdr(skb)).saddr);
    if security_inet_conn_request(sk, skb, req) != 0 { reqsk_free(req); return core::ptr::null_mut(); }
    tcp_ao_syncookie(sk, skb, req, AF_INET);
    let mut fl4: flowi4 = core::mem::zeroed(); flowi4_init_output(&mut fl4, (*ireq).ir_iif, (*ireq).ir_mark, ip_sock_rt_tos(sk), ip_sock_rt_scope(sk), IPPROTO_TCP, inet_sk_flowi_flags(sk), (*ireq).ir_rmt_addr, (*ireq).ir_loc_addr, (*th).source, (*th).dest, sk_uid(sk));
    let rt = ip_route_output_key(net, &mut fl4); if IS_ERR(rt) { reqsk_free(req); return core::ptr::null_mut(); }
    let tp = tcp_sk(sk); (*req).rsk_window_clamp = if READ_ONCE((*tp).window_clamp) != 0 { (*tp).window_clamp } else { dst_metric(&(*rt).dst, RTAX_WINDOW) };
    let full_space = tcp_full_space(sk); tcp_select_initial_window(sk, full_space, (*req).mss, &mut (*req).rsk_rcv_wnd, &mut (*req).rsk_window_clamp, (*ireq).wscale_ok, &mut (*ireq).rcv_wscale, dst_metric(&(*rt).dst, RTAX_INITRWND));
    (*ireq).ecn_ok &= cookie_ecn_ok(net, &(*rt).dst); (*treq).accecn_ok = (*ireq).ecn_ok && cookie_accecn_ok(th);
    ret = tcp_get_cookie_sock(sk, skb, req, &mut (*rt).dst); if ret.is_null() { return core::ptr::null_mut(); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
