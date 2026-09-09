// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 Syncookies implementation for the Linux kernel. */

const COOKIEBITS: u32 = 24;
const COOKIEMASK: u32 = (1u32 << COOKIEBITS) - 1;

static mut syncookie6_secret: [siphash_aligned_key_t; 2] = [
    unsafe { core::mem::zeroed() },
    unsafe { core::mem::zeroed() },
];

static msstab: [u16; 4] = [1280 - 60, 1480 - 60, 1500 - 60, 9000 - 60];

unsafe fn cookie_hash(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    count: u32,
    c: i32,
) -> u32 {
    #[repr(C, align(8))]
    struct Combined {
        saddr: in6_addr,
        daddr: in6_addr,
        count: u32,
        sport: __be16,
        dport: __be16,
    }
    let combined = Combined {
        saddr: *saddr,
        daddr: *daddr,
        count,
        sport,
        dport,
    };
    net_get_random_once(
        syncookie6_secret.as_mut_ptr() as *mut _,
        core::mem::size_of_val(&syncookie6_secret),
    );
    siphash(
        &combined as *const _ as *const _,
        core::mem::offset_of!(Combined, dport) + core::mem::size_of::<__be16>(),
        &syncookie6_secret[c as usize],
    )
}

unsafe fn secure_tcp_syn_cookie(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    sseq: u32,
    data: u32,
) -> u32 {
    let count = tcp_cookie_time();
    cookie_hash(saddr, daddr, sport, dport, 0, 0)
        .wrapping_add(sseq)
        .wrapping_add(count << COOKIEBITS)
        .wrapping_add(cookie_hash(saddr, daddr, sport, dport, count, 1).wrapping_add(data) & COOKIEMASK)
}

unsafe fn check_tcp_syn_cookie(
    mut cookie: u32,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    sseq: u32,
) -> u32 {
    let count = tcp_cookie_time();
    cookie = cookie.wrapping_sub(cookie_hash(saddr, daddr, sport, dport, 0, 0).wrapping_add(sseq));
    let diff = count.wrapping_sub(cookie >> COOKIEBITS) & (u32::MAX >> COOKIEBITS);
    if diff >= MAX_SYNCOOKIE_AGE {
        return u32::MAX;
    }
    cookie.wrapping_sub(cookie_hash(saddr, daddr, sport, dport, count.wrapping_sub(diff), 1)) & COOKIEMASK
}

pub unsafe fn __cookie_v6_init_sequence(iph: *const ipv6hdr, th: *const tcphdr, mssp: *mut u16) -> u32 {
    let mut mssind: i32;
    let mss = *mssp;
    mssind = msstab.len() as i32 - 1;
    while mssind != 0 {
        if mss >= msstab[mssind as usize] { break; }
        mssind -= 1;
    }
    *mssp = msstab[mssind as usize];
    secure_tcp_syn_cookie(&(*iph).saddr, &(*iph).daddr, (*th).source, (*th).dest, ntohl((*th).seq), mssind as u32)
}

pub unsafe fn cookie_v6_init_sequence(skb: *const sk_buff, mssp: *mut u16) -> u32 {
    let iph = ipv6_hdr(skb);
    let th = tcp_hdr(skb);
    __cookie_v6_init_sequence(iph, th, mssp)
}

pub unsafe fn __cookie_v6_check(iph: *const ipv6hdr, th: *const tcphdr) -> u16 {
    let cookie = ntohl((*th).ack_seq).wrapping_sub(1);
    let seq = ntohl((*th).seq).wrapping_sub(1);
    let mssind = check_tcp_syn_cookie(cookie, &(*iph).saddr, &(*iph).daddr, (*th).source, (*th).dest, seq);
    if mssind < msstab.len() as u32 { msstab[mssind as usize] } else { 0 }
}

unsafe fn cookie_tcp_check(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> *mut request_sock {
    let mut tcp_opt: tcp_options_received = core::mem::zeroed();
    let mut tsoff: u32 = 0;
    let mut mss: i32;
    if tcp_synq_no_recent_overflow(sk) { return ERR_PTR(-EINVAL); }
    mss = __cookie_v6_check(ipv6_hdr(skb), tcp_hdr(skb)) as i32;
    if mss == 0 { __NET_INC_STATS(net, LINUX_MIB_SYNCOOKIESFAILED); return ERR_PTR(-EINVAL); }
    __NET_INC_STATS(net, LINUX_MIB_SYNCOOKIESRECV);
    tcp_parse_options(net, skb, &mut tcp_opt, 0, core::ptr::null_mut());
    if tcp_opt.saw_tstamp && tcp_opt.rcv_tsecr != 0 {
        let st = secure_tcpv6_seq_and_ts_off(net, (*ipv6_hdr(skb)).daddr.s6_addr32, (*ipv6_hdr(skb)).saddr.s6_addr32, (*tcp_hdr(skb)).dest, (*tcp_hdr(skb)).source);
        tsoff = st.ts_off;
        tcp_opt.rcv_tsecr = tcp_opt.rcv_tsecr.wrapping_sub(tsoff);
    }
    if !cookie_timestamp_decode(net, &mut tcp_opt) { return ERR_PTR(-EINVAL); }
    cookie_tcp_reqsk_alloc(&tcp6_request_sock_ops, sk, skb, &mut tcp_opt, mss, tsoff)
}

pub unsafe fn cookie_v6_check(sk: *mut sock, skb: *mut sk_buff) -> *mut sock {
    let th = tcp_hdr(skb);
    let np = inet6_sk(sk);
    let tp = tcp_sk(sk);
    let net = sock_net(sk);
    let mut req: *mut request_sock;
    let mut ret = sk;
    let mut reason: skb_drop_reason = 0;
    if !READ_ONCE((*(*net).ipv4).sysctl_tcp_syncookies) || !(*th).ack || (*th).rst { return ret; }
    req = if cookie_bpf_ok(skb) { cookie_bpf_check(sk, skb) } else { cookie_tcp_check(net, sk, skb) };
    if IS_ERR(req) { return ret; }
    if req.is_null() { SKB_DR_SET(reason, NO_SOCKET); sk_skb_reason_drop(sk, skb, reason); return core::ptr::null_mut(); }
    let ireq = inet_rsk(req);
    (*ireq).ir_v6_rmt_addr = (*ipv6_hdr(skb)).saddr;
    (*ireq).ir_v6_loc_addr = (*ipv6_hdr(skb)).daddr;
    if security_inet_conn_request(sk, skb, req) { SKB_DR_SET(reason, SECURITY_HOOK); reqsk_free(req); sk_skb_reason_drop(sk, skb, reason); return core::ptr::null_mut(); }
    if ipv6_opt_accepted(sk, skb, &mut (*TCP_SKB_CB(skb)).header.h6) || (*np).rxopt.bits.rxinfo || (*np).rxopt.bits.rxoinfo || (*np).rxopt.bits.rxhlim || (*np).rxopt.bits.rxohlim {
        refcount_inc(&mut (*skb).users);
        (*ireq).pktopts = skb;
    }
    if (*sk).sk_bound_dev_if == 0 && ipv6_addr_type(&(*ireq).ir_v6_rmt_addr) & IPV6_ADDR_LINKLOCAL != 0 { (*ireq).ir_iif = tcp_v6_iif(skb); }
    tcp_ao_syncookie(sk, skb, req, AF_INET6);
    let dst = ip6_dst_lookup_flow(net, sk, core::ptr::null_mut(), core::ptr::null());
    if IS_ERR(dst) { SKB_DR_SET(reason, IP_OUTNOROUTES); reqsk_free(req); sk_skb_reason_drop(sk, skb, reason); return core::ptr::null_mut(); }
    (*req).rsk_window_clamp = if READ_ONCE((*tp).window_clamp) != 0 { (*tp).window_clamp } else { dst_metric(dst, RTAX_WINDOW) };
    let full_space = tcp_full_space(sk);
    if (*sk).sk_userlocks & SOCK_RCVBUF_LOCK != 0 && ((*req).rsk_window_clamp > full_space || (*req).rsk_window_clamp == 0) { (*req).rsk_window_clamp = full_space; }
    let mut rcv_wscale = 0u8;
    tcp_select_initial_window(sk, full_space, (*req).mss, &mut (*req).rsk_rcv_wnd, &mut (*req).rsk_window_clamp, (*ireq).wscale_ok, &mut rcv_wscale, dst_metric(dst, RTAX_INITRWND));
    if !(*req).syncookie { (*ireq).rcv_wscale = rcv_wscale; }
    (*ireq).ecn_ok &= cookie_ecn_ok(net, dst);
    (*tcp_rsk(req)).accecn_ok = (*ireq).ecn_ok && cookie_accecn_ok(th);
    ret = tcp_get_cookie_sock(sk, skb, req, dst);
    if ret.is_null() { SKB_DR_SET(reason, NO_SOCKET); sk_skb_reason_drop(sk, skb, reason); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
