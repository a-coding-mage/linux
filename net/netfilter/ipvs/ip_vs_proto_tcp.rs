// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_proto_tcp.c: TCP load balancing support for IPVS
 *
 * This is a source-level Rust translation; kernel-provided declarations are
 * intentionally referenced as external dependencies.
 */

static mut TCP_STATE_OFF: [i32; IP_VS_DIR_LAST as usize] = [
    TCP_DIR_INPUT, TCP_DIR_OUTPUT, TCP_DIR_INPUT_ONLY,
];

unsafe fn tcp_csum_check(af: i32, skb: *mut sk_buff, pp: *mut ip_vs_protocol,
                         iph: *mut ip_vs_iphdr) -> i32 {
    if !ip_vs_checksum_common_check(skb, (*iph).len, IPPROTO_TCP, af) {
        IP_VS_DBG_RL_PKT(0, af, pp, skb, (*iph).off, "Failed checksum for");
        return 0;
    }
    1
}

unsafe fn tcp_conn_schedule(ipvs: *mut netns_ipvs, af: i32, skb: *mut sk_buff,
    pd: *mut ip_vs_proto_data, verdict: *mut i32, cpp: *mut *mut ip_vs_conn,
    iph: *mut ip_vs_iphdr) -> i32 {
    let mut svc: *mut ip_vs_service = core::ptr::null_mut();
    let mut _tcph: tcphdr = core::mem::zeroed();
    let mut ports: *mut __be16 = core::ptr::null_mut();
    if likely(!ip_vs_iph_icmp(iph)) {
        let th = skb_header_pointer(skb, (*iph).len, core::mem::size_of::<tcphdr>(),
                                    &mut _tcph as *mut _ as *mut _);
        if !th.is_null() {
            if (*th).rst || !(sysctl_sloppy_tcp(ipvs) || (*th).syn) { return 1; }
            ports = &mut (*th).source;
        }
    } else {
        let mut _ports: [__be16; 2] = [0; 2];
        ports = skb_header_pointer(skb, (*iph).len, core::mem::size_of_val(&_ports),
                                   _ports.as_mut_ptr() as *mut _);
    }
    if ports.is_null() { *verdict = NF_DROP; return 0; }
    if likely(!ip_vs_iph_inverse(iph)) {
        svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol,
                                  &(*iph).daddr, *ports.add(1));
    } else {
        svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol,
                                  &(*iph).saddr, *ports);
    }
    if !svc.is_null() {
        let mut ignored = 0;
        if ip_vs_todrop(ipvs) { *verdict = NF_DROP; return 0; }
        *cpp = ip_vs_schedule(svc, skb, pd, &mut ignored, iph);
        if (*cpp).is_null() && ignored <= 0 {
            *verdict = if ignored == 0 { ip_vs_leave(svc, skb, pd, iph) } else { NF_DROP };
            return 0;
        }
    }
    1
}

unsafe fn tcp_fast_csum_update(af: i32, tcph: *mut tcphdr,
    oldip: *const nf_inet_addr, newip: *const nf_inet_addr,
    oldport: __be16, newport: __be16) {
    #[cfg(feature = "CONFIG_IP_VS_IPV6")]
    if af == AF_INET6 {
        (*tcph).check = csum_fold(ip_vs_check_diff16((*oldip).ip6, (*newip).ip6,
            ip_vs_check_diff2(oldport, newport, !csum_unfold((*tcph).check))));
        return;
    }
    (*tcph).check = csum_fold(ip_vs_check_diff4((*oldip).ip, (*newip).ip,
        ip_vs_check_diff2(oldport, newport, !csum_unfold((*tcph).check))));
}

unsafe fn tcp_partial_csum_update(af: i32, tcph: *mut tcphdr,
    oldip: *const nf_inet_addr, newip: *const nf_inet_addr,
    oldlen: __be16, newlen: __be16) {
    #[cfg(feature = "CONFIG_IP_VS_IPV6")]
    if af == AF_INET6 {
        (*tcph).check = !csum_fold(ip_vs_check_diff16((*oldip).ip6, (*newip).ip6,
            ip_vs_check_diff2(oldlen, newlen, csum_unfold((*tcph).check))));
        return;
    }
    (*tcph).check = !csum_fold(ip_vs_check_diff4((*oldip).ip, (*newip).ip,
        ip_vs_check_diff2(oldlen, newlen, csum_unfold((*tcph).check))));
}

unsafe fn tcp_snat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol,
    cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    #[cfg(feature = "CONFIG_IP_VS_IPV6")]
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    let tcphoff = (*iph).len;
    let mut oldlen = (*skb).len - tcphoff;
    let mut payload_csum = false;
    if skb_ensure_writable(skb, tcphoff + core::mem::size_of::<tcphdr>()) != 0 { return 0; }
    if !(*cp).app.is_null() {
        if tcp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; }
        let ret = ip_vs_app_pkt_out(cp, skb, iph); if ret == 0 { return 0; }
        if ret == 1 { oldlen = (*skb).len - tcphoff; } else { payload_csum = true; }
    }
    let tcph = ((*skb).data.add(tcphoff)) as *mut tcphdr;
    (*tcph).source = (*cp).vport;
    if (*skb).ip_summed == CHECKSUM_PARTIAL {
        tcp_partial_csum_update((*cp).af, tcph, &(*cp).daddr, &(*cp).vaddr,
                                htons(oldlen), htons((*skb).len - tcphoff));
    } else if !payload_csum {
        tcp_fast_csum_update((*cp).af, tcph, &(*cp).daddr, &(*cp).vaddr,
                             (*cp).dport, (*cp).vport);
        if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).ip_summed = if !(*cp).app.is_null() { CHECKSUM_UNNECESSARY } else { CHECKSUM_NONE }; }
    } else {
        (*tcph).check = 0;
        (*skb).csum = skb_checksum(skb, tcphoff, (*skb).len - tcphoff, 0);
        #[cfg(feature = "CONFIG_IP_VS_IPV6")]
        if (*cp).af == AF_INET6 { (*tcph).check = csum_ipv6_magic(&(*cp).vaddr.in6, &(*cp).caddr.in6, (*skb).len - tcphoff, (*cp).protocol, (*skb).csum); }
        #[cfg(not(feature = "CONFIG_IP_VS_IPV6"))]
        { (*tcph).check = csum_tcpudp_magic((*cp).vaddr.ip, (*cp).caddr.ip, (*skb).len - tcphoff, (*cp).protocol, (*skb).csum); }
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
        IP_VS_DBG(11, "O-pkt: %s O-csum=%d (+%zd)\n", (*pp).name, (*tcph).check, (tcph as isize + 0) as isize);
    }
    1
}

unsafe fn tcp_dnat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol,
    cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    #[cfg(feature = "CONFIG_IP_VS_IPV6")]
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    let tcphoff = (*iph).len; let mut oldlen = (*skb).len - tcphoff; let mut payload_csum = false;
    if skb_ensure_writable(skb, tcphoff + core::mem::size_of::<tcphdr>()) != 0 { return 0; }
    if !(*cp).app.is_null() { if tcp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; } let ret = ip_vs_app_pkt_in(cp, skb, iph); if ret == 0 { return 0; } if ret == 1 { oldlen = (*skb).len - tcphoff; } else { payload_csum = true; } }
    let tcph = (*skb).data.add(tcphoff) as *mut tcphdr; (*tcph).dest = (*cp).dport;
    if (*skb).ip_summed == CHECKSUM_PARTIAL { tcp_partial_csum_update((*cp).af, tcph, &(*cp).vaddr, &(*cp).daddr, htons(oldlen), htons((*skb).len - tcphoff)); }
    else if !payload_csum { tcp_fast_csum_update((*cp).af, tcph, &(*cp).vaddr, &(*cp).daddr, (*cp).vport, (*cp).dport); if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).ip_summed = if !(*cp).app.is_null() { CHECKSUM_UNNECESSARY } else { CHECKSUM_NONE }; } }
    else { (*tcph).check = 0; (*skb).csum = skb_checksum(skb, tcphoff, (*skb).len - tcphoff, 0); #[cfg(feature = "CONFIG_IP_VS_IPV6")] if (*cp).af == AF_INET6 { (*tcph).check = csum_ipv6_magic(&(*cp).caddr.in6, &(*cp).daddr.in6, (*skb).len - tcphoff, (*cp).protocol, (*skb).csum); } #[cfg(not(feature = "CONFIG_IP_VS_IPV6"))] { (*tcph).check = csum_tcpudp_magic((*cp).caddr.ip, (*cp).daddr.ip, (*skb).len - tcphoff, (*cp).protocol, (*skb).csum); } (*skb).ip_summed = CHECKSUM_UNNECESSARY; }
    1
}

const TCP_DIR_INPUT: i32 = 0;
const TCP_DIR_OUTPUT: i32 = 4;
const TCP_DIR_INPUT_ONLY: i32 = 8;
static TCP_TIMEOUTS: [i32; (IP_VS_TCP_S_LAST + 1) as usize] = [2*HZ, 15*60*HZ, 2*60*HZ, 60*HZ, 2*60*HZ, 2*60*HZ, 10*HZ, 60*HZ, 30*HZ, 2*60*HZ, 120*HZ, 2*HZ];
static TCP_STATE_NAMES: [&str; (IP_VS_TCP_S_LAST + 1) as usize] = ["NONE", "ESTABLISHED", "SYN_SENT", "SYN_RECV", "FIN_WAIT", "TIME_WAIT", "CLOSE", "CLOSE_WAIT", "LAST_ACK", "LISTEN", "SYNACK", "BUG!"];
static TCP_STATE_ACTIVE: [bool; IP_VS_TCP_S_LAST as usize] = [false, true, true, true, false, false, false, false, false, false, true];

const sNO: i32 = IP_VS_TCP_S_NONE; const sES: i32 = IP_VS_TCP_S_ESTABLISHED; const sSS: i32 = IP_VS_TCP_S_SYN_SENT; const sSR: i32 = IP_VS_TCP_S_SYN_RECV; const sFW: i32 = IP_VS_TCP_S_FIN_WAIT; const sTW: i32 = IP_VS_TCP_S_TIME_WAIT; const sCL: i32 = IP_VS_TCP_S_CLOSE; const sCW: i32 = IP_VS_TCP_S_CLOSE_WAIT; const sLA: i32 = IP_VS_TCP_S_LAST_ACK; const sLI: i32 = IP_VS_TCP_S_LISTEN; const sSA: i32 = IP_VS_TCP_S_SYNACK;

#[repr(C)] pub struct tcp_states_t { pub next_state: [i32; IP_VS_TCP_S_LAST as usize] }
static mut TCP_STATES: [tcp_states_t; 12] = [
    tcp_states_t { next_state: [sSR,sES,sES,sSR,sSR,sSR,sSR,sSR,sSR,sSR,sSR] }, tcp_states_t { next_state: [sCL,sCW,sSS,sTW,sTW,sTW,sCL,sCW,sLA,sLI,sTW] }, tcp_states_t { next_state: [sES,sES,sSS,sES,sFW,sTW,sCL,sCW,sCL,sLI,sES] }, tcp_states_t { next_state: [sCL,sCL,sCL,sSR,sCL,sCL,sCL,sCL,sLA,sLI,sSR] },
    tcp_states_t { next_state: [sSS,sES,sSS,sSR,sSS,sSS,sSS,sSS,sSS,sLI,sSR] }, tcp_states_t { next_state: [sTW,sFW,sSS,sTW,sFW,sTW,sCL,sTW,sLA,sLI,sTW] }, tcp_states_t { next_state: [sES,sES,sSS,sES,sFW,sTW,sCL,sCW,sLA,sES,sES] }, tcp_states_t { next_state: [sCL,sCL,sSS,sCL,sCL,sTW,sCL,sCL,sCL,sCL,sCL] },
    tcp_states_t { next_state: [sSR,sES,sES,sSR,sSR,sSR,sSR,sSR,sSR,sSR,sSR] }, tcp_states_t { next_state: [sCL,sFW,sSS,sTW,sFW,sTW,sCL,sCW,sLA,sLI,sTW] }, tcp_states_t { next_state: [sES,sES,sSS,sES,sFW,sTW,sCL,sCW,sCL,sLI,sES] }, tcp_states_t { next_state: [sCL,sCL,sCL,sSR,sCL,sCL,sCL,sCL,sLA,sLI,sCL] },
];

unsafe fn tcp_state_name(state: i32) -> *const u8 { if state >= IP_VS_TCP_S_LAST { return b"ERR!\0".as_ptr(); } TCP_STATE_NAMES[state as usize].as_ptr() }
unsafe fn tcp_state_active(state: i32) -> bool { state < IP_VS_TCP_S_LAST && TCP_STATE_ACTIVE[state as usize] }

unsafe fn tcp_timeout_change(pd: *mut ip_vs_proto_data, flags: i32) { (*pd).tcp_state_table = if flags & 1 != 0 { TCP_STATES_DOS.as_mut_ptr() } else { TCP_STATES.as_mut_ptr() }; }
unsafe fn tcp_state_idx(th: *mut tcphdr) -> i32 { if (*th).rst {3} else if (*th).syn {0} else if (*th).fin {1} else if (*th).ack {2} else {-1} }

unsafe fn set_tcp_state(pd: *mut ip_vs_proto_data, cp: *mut ip_vs_conn,
                        direction: i32, th: *mut tcphdr) {
    let mut state_off = TCP_STATE_OFF[direction as usize];
    if (*cp).flags & IP_VS_CONN_F_NOOUTPUT != 0 {
        if state_off == TCP_DIR_OUTPUT { (*cp).flags &= !IP_VS_CONN_F_NOOUTPUT; }
        else { state_off = TCP_DIR_INPUT_ONLY; }
    }
    let state_idx = tcp_state_idx(th);
    let mut new_state = IP_VS_TCP_S_CLOSE;
    if state_idx >= 0 { new_state = (*pd).tcp_state_table[(state_off + state_idx) as usize].next_state[(*cp).state as usize]; }
    if new_state != (*cp).state {
        let dest = (*cp).dest;
        if !dest.is_null() {
            if (*cp).flags & IP_VS_CONN_F_INACTIVE == 0 && !tcp_state_active(new_state) { atomic_dec(&mut (*dest).activeconns); (*cp).flags |= IP_VS_CONN_F_INACTIVE; }
            else if (*cp).flags & IP_VS_CONN_F_INACTIVE != 0 && tcp_state_active(new_state) { atomic_inc(&mut (*dest).activeconns); (*cp).flags &= !IP_VS_CONN_F_INACTIVE; }
        }
        if new_state == IP_VS_TCP_S_ESTABLISHED { ip_vs_control_assure_ct(cp); }
    }
    (*cp).timeout = (*pd).timeout_table[ { (*cp).state = new_state; new_state as usize } ];
}

unsafe fn tcp_state_transition(cp: *mut ip_vs_conn, direction: i32,
    skb: *const sk_buff, pd: *mut ip_vs_proto_data, iph_len: u32) {
    let mut th: tcphdr = core::mem::zeroed();
    let p = skb_header_pointer(skb as *mut _, iph_len, core::mem::size_of::<tcphdr>(), &mut th as *mut _ as *mut _);
    if p.is_null() { return; }
    spin_lock_bh(&mut (*cp).lock); set_tcp_state(pd, cp, direction, &mut th); spin_unlock_bh(&mut (*cp).lock);
}

unsafe fn tcp_app_hashkey(port: __be16) -> u16 { ((((port as u16) >> TCP_APP_TAB_BITS) ^ port as u16) & TCP_APP_TAB_MASK) as u16 }
unsafe fn tcp_register_app(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) -> i32 {
    let pd = ip_vs_proto_data_get(ipvs, IPPROTO_TCP); let hash = tcp_app_hashkey((*inc).port); let mut i: *mut ip_vs_app = core::ptr::null_mut();
    list_for_each_entry(&mut i, &mut (*ipvs).tcp_apps[hash as usize], p_list) { if (*i).port == (*inc).port { return -EEXIST; } }
    list_add_rcu(&mut (*inc).p_list, &mut (*ipvs).tcp_apps[hash as usize]); atomic_inc(&mut (*pd).appcnt); 0
}
unsafe fn tcp_unregister_app(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) { let pd = ip_vs_proto_data_get(ipvs, IPPROTO_TCP); atomic_dec(&mut (*pd).appcnt); list_del_rcu(&mut (*inc).p_list); }
unsafe fn tcp_app_conn_bind(cp: *mut ip_vs_conn) -> i32 {
    if IP_VS_FWD_METHOD(cp) != IP_VS_CONN_F_MASQ { return 0; }
    let ipvs = (*cp).ipvs; let hash = tcp_app_hashkey((*cp).vport); let mut inc: *mut ip_vs_app = core::ptr::null_mut(); let mut result = 0;
    list_for_each_entry_rcu(&mut inc, &mut (*ipvs).tcp_apps[hash as usize], p_list) {
        if (*inc).port == (*cp).vport { if !ip_vs_app_inc_get(inc) { break; } (*cp).app = inc; if let Some(f) = (*inc).init_conn { result = f(inc, cp); } break; }
    } result
}
pub unsafe fn ip_vs_tcp_conn_listen(cp: *mut ip_vs_conn) { let pd = ip_vs_proto_data_get((*cp).ipvs, IPPROTO_TCP); spin_lock_bh(&mut (*cp).lock); (*cp).state = IP_VS_TCP_S_LISTEN; (*cp).timeout = if !pd.is_null() { (*pd).timeout_table[IP_VS_TCP_S_LISTEN as usize] } else { TCP_TIMEOUTS[IP_VS_TCP_S_LISTEN as usize] }; spin_unlock_bh(&mut (*cp).lock); }
unsafe fn __ip_vs_tcp_init(ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) -> i32 { ip_vs_init_hash_table((*ipvs).tcp_apps.as_mut_ptr(), TCP_APP_TAB_SIZE); (*pd).timeout_table = ip_vs_create_timeout_table(TCP_TIMEOUTS.as_ptr() as *mut i32, core::mem::size_of_val(&TCP_TIMEOUTS)); if (*pd).timeout_table.is_null() { return -ENOMEM; } (*pd).tcp_state_table = TCP_STATES.as_mut_ptr(); 0 }
unsafe fn __ip_vs_tcp_exit(_ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) { kfree((*pd).timeout_table as *mut _); }

static mut TCP_STATES_DOS: [tcp_states_t; 12] = [
    tcp_states_t { next_state: [sSR,sES,sES,sSR,sSR,sSR,sSR,sSR,sSR,sSR,sSA] }, tcp_states_t { next_state: [sCL,sCW,sSS,sTW,sTW,sTW,sCL,sCW,sLA,sLI,sSA] }, tcp_states_t { next_state: [sES,sES,sSS,sSR,sFW,sTW,sCL,sCW,sCL,sLI,sSA] }, tcp_states_t { next_state: [sCL,sCL,sCL,sSR,sCL,sCL,sCL,sCL,sLA,sLI,sCL] },
    tcp_states_t { next_state: [sSS,sES,sSS,sSA,sSS,sSS,sSS,sSS,sSS,sLI,sSA] }, tcp_states_t { next_state: [sTW,sFW,sSS,sTW,sFW,sTW,sCL,sTW,sLA,sLI,sTW] }, tcp_states_t { next_state: [sES,sES,sSS,sES,sFW,sTW,sCL,sCW,sLA,sES,sES] }, tcp_states_t { next_state: [sCL,sCL,sSS,sCL,sCL,sTW,sCL,sCL,sCL,sCL,sCL] },
    tcp_states_t { next_state: [sSA,sES,sES,sSR,sSA,sSA,sSA,sSA,sSA,sSA,sSA] }, tcp_states_t { next_state: [sCL,sFW,sSS,sTW,sFW,sTW,sCL,sCW,sLA,sLI,sTW] }, tcp_states_t { next_state: [sES,sES,sSS,sES,sFW,sTW,sCL,sCW,sCL,sLI,sES] }, tcp_states_t { next_state: [sCL,sCL,sCL,sSR,sCL,sCL,sCL,sCL,sLA,sLI,sCL] },
];

#[no_mangle]
pub static mut ip_vs_protocol_tcp: ip_vs_protocol = ip_vs_protocol {
    name: b"TCP\0".as_ptr(), protocol: IPPROTO_TCP, num_states: IP_VS_TCP_S_LAST,
    dont_defrag: 0, init: None, exit: None, init_netns: Some(__ip_vs_tcp_init),
    exit_netns: Some(__ip_vs_tcp_exit), register_app: Some(tcp_register_app),
    unregister_app: Some(tcp_unregister_app), conn_schedule: Some(tcp_conn_schedule),
    conn_in_get: Some(ip_vs_conn_in_get_proto), conn_out_get: Some(ip_vs_conn_out_get_proto),
    snat_handler: Some(tcp_snat_handler), dnat_handler: Some(tcp_dnat_handler),
    state_name: Some(tcp_state_name), state_transition: Some(tcp_state_transition),
    app_conn_bind: Some(tcp_app_conn_bind), debug_packet: Some(ip_vs_tcpudp_debug_packet),
    timeout_change: Some(tcp_timeout_change),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
