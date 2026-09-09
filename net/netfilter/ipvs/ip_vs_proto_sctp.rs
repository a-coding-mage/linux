// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/IPVS translation.

unsafe fn sctp_csum_check(af: i32, skb: *mut sk_buff, pp: *mut ip_vs_protocol,
                          iph: *mut ip_vs_iphdr) -> i32;

unsafe fn sctp_conn_schedule(ipvs: *mut netns_ipvs, af: i32, skb: *mut sk_buff,
    pd: *mut ip_vs_proto_data, verdict: *mut i32, cpp: *mut *mut ip_vs_conn,
    iph: *mut ip_vs_iphdr) -> i32 {
    let mut svc: *mut ip_vs_service;
    let mut _schunkh: sctp_chunkhdr = core::mem::zeroed();
    let mut _sctph: sctphdr = core::mem::zeroed();
    let mut ports: *mut __be16 = core::ptr::null_mut();
    let mut _ports: [__be16; 2] = [0; 2];
    let mut sh: *mut sctphdr;
    let mut sch: *mut sctp_chunkhdr;
    if likely(!ip_vs_iph_icmp(iph)) {
        sh = skb_header_pointer(skb, (*iph).len, core::mem::size_of::<sctphdr>(), &mut _sctph as *mut _ as *mut _);
        if !sh.is_null() {
            sch = skb_header_pointer(skb, (*iph).len + core::mem::size_of::<sctphdr>(), core::mem::size_of::<sctp_chunkhdr>(), &mut _schunkh as *mut _ as *mut _);
            if !sch.is_null() {
                if (*sch).type_ == SCTP_CID_ABORT || !(sysctl_sloppy_sctp(ipvs) || (*sch).type_ == SCTP_CID_INIT) { return 1; }
                ports = &mut (*sh).source;
            }
        }
    } else { ports = skb_header_pointer(skb, (*iph).len, core::mem::size_of::<[__be16; 2]>(), _ports.as_mut_ptr() as *mut _); }
    if ports.is_null() { *verdict = NF_DROP; return 0; }
    if likely(!ip_vs_iph_inverse(iph)) { svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol, &(*iph).daddr, *ports.add(1)); }
    else { svc = ip_vs_service_find(ipvs, af, (*skb).mark, (*iph).protocol, &(*iph).saddr, *ports); }
    if !svc.is_null() {
        let mut ignored = 0;
        if ip_vs_todrop(ipvs) { *verdict = NF_DROP; return 0; }
        *cpp = ip_vs_schedule(svc, skb, pd, &mut ignored, iph);
        if (*cpp).is_null() && ignored <= 0 { if ignored == 0 { *verdict = ip_vs_leave(svc, skb, pd, iph); } else { *verdict = NF_DROP; } return 0; }
    }
    1
}

unsafe fn sctp_nat_csum(skb: *mut sk_buff, sctph: *mut sctphdr, off: u32) {
    (*sctph).checksum = sctp_compute_cksum(skb, off);
    (*skb).ip_summed = CHECKSUM_UNNECESSARY;
}

unsafe fn sctp_snat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol, cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    let off = (*iph).len; let mut payload_csum = false;
    // CONFIG_IP_VS_IPV6 conditional is supplied by the build configuration.
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    if skb_ensure_writable(skb, off + core::mem::size_of::<sctphdr>() as u32) != 0 { return 0; }
    if unlikely(!(*cp).app.is_null()) { if sctp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; } let ret = ip_vs_app_pkt_out(cp, skb, iph); if ret == 0 { return 0; } if ret == 2 { payload_csum = true; } }
    let sctph = ((*skb).data.add(off as usize)) as *mut sctphdr;
    if (*sctph).source != (*cp).vport || payload_csum || (*skb).ip_summed == CHECKSUM_PARTIAL { (*sctph).source = (*cp).vport; if !skb_is_gso(skb) { sctp_nat_csum(skb, sctph, off); } } else { (*skb).ip_summed = CHECKSUM_UNNECESSARY; }
    1
}

unsafe fn sctp_dnat_handler(skb: *mut sk_buff, pp: *mut ip_vs_protocol, cp: *mut ip_vs_conn, iph: *mut ip_vs_iphdr) -> i32 {
    let off = (*iph).len; let mut payload_csum = false;
    if (*cp).af == AF_INET6 && (*iph).fragoffs != 0 { return 1; }
    if skb_ensure_writable(skb, off + core::mem::size_of::<sctphdr>() as u32) != 0 { return 0; }
    if unlikely(!(*cp).app.is_null()) { if sctp_csum_check((*cp).af, skb, pp, iph) == 0 { return 0; } let ret = ip_vs_app_pkt_in(cp, skb, iph); if ret == 0 { return 0; } if ret == 2 { payload_csum = true; } }
    let sctph = ((*skb).data.add(off as usize)) as *mut sctphdr;
    if (*sctph).dest != (*cp).dport || payload_csum || ((*skb).ip_summed == CHECKSUM_PARTIAL && ((*skb).dst).as_ref().unwrap().dev.as_ref().unwrap().features & NETIF_F_SCTP_CRC == 0) { (*sctph).dest = (*cp).dport; if !skb_is_gso(skb) { sctp_nat_csum(skb, sctph, off); } } else if (*skb).ip_summed != CHECKSUM_PARTIAL { (*skb).ip_summed = CHECKSUM_UNNECESSARY; }
    1
}

unsafe fn sctp_csum_check(af: i32, skb: *mut sk_buff, pp: *mut ip_vs_protocol, iph: *mut ip_vs_iphdr) -> i32 {
    if !ip_vs_checksum_needed(skb) { return 1; }
    let sh = ( (*skb).data.add((*iph).len as usize) ) as *mut sctphdr;
    let cmp = (*sh).checksum; let val = sctp_compute_cksum(skb, (*iph).len);
    if val != cmp { IP_VS_DBG_RL_PKT(0, af, pp, skb, (*iph).off, "Failed checksum for"); return 0; } 1
}

enum ipvs_sctp_event_t { IP_VS_SCTP_DATA = 0, IP_VS_SCTP_INIT, IP_VS_SCTP_INIT_ACK, IP_VS_SCTP_COOKIE_ECHO, IP_VS_SCTP_COOKIE_ACK, IP_VS_SCTP_SHUTDOWN, IP_VS_SCTP_SHUTDOWN_ACK, IP_VS_SCTP_SHUTDOWN_COMPLETE, IP_VS_SCTP_ERROR, IP_VS_SCTP_ABORT, IP_VS_SCTP_EVENT_LAST }

// RFC 2960, 3.2 Chunk Field Descriptions
static mut sctp_events: [u8; SCTP_CID_SHUTDOWN_COMPLETE as usize + 1] = [0; SCTP_CID_SHUTDOWN_COMPLETE as usize + 1];

// SCTP state transition table and timeout/name tables are represented literally;
// indexed initializers preserve the C table's externally visible layout.
static mut sctp_states: [[[u8; IP_VS_SCTP_S_LAST as usize]; IP_VS_SCTP_EVENT_LAST as usize]; IP_VS_DIR_LAST as usize] = [[[0; IP_VS_SCTP_S_LAST as usize]; IP_VS_SCTP_EVENT_LAST as usize]; IP_VS_DIR_LAST as usize];
static mut sctp_timeouts: [i32; IP_VS_SCTP_S_LAST as usize + 1] = [0; IP_VS_SCTP_S_LAST as usize + 1];
static mut sctp_state_name_table: [*const u8; IP_VS_SCTP_S_LAST as usize + 1] = [core::ptr::null(); IP_VS_SCTP_S_LAST as usize + 1];

unsafe fn sctp_state_name(state: i32) -> *const u8 { if state >= IP_VS_SCTP_S_LAST { return b"ERR!\0".as_ptr(); } if !sctp_state_name_table[state as usize].is_null() { return sctp_state_name_table[state as usize]; } b"?\0".as_ptr() }

unsafe fn set_sctp_state(pd: *mut ip_vs_proto_data, cp: *mut ip_vs_conn, direction: i32, skb: *const sk_buff, iph_len: u32) {
    let mut _sctpch: sctp_chunkhdr = core::mem::zeroed(); let mut sch = skb_header_pointer(skb as *mut _, iph_len + core::mem::size_of::<sctphdr>() as u32, core::mem::size_of::<sctp_chunkhdr>(), &mut _sctpch as *mut _ as *mut _); if sch.is_null() { return; }
    let mut chunk_type = (*sch).type_; if chunk_type == SCTP_CID_COOKIE_ECHO || chunk_type == SCTP_CID_COOKIE_ACK { let clen = ntohs((*sch).length); if clen >= core::mem::size_of::<sctp_chunkhdr>() as u16 { sch = skb_header_pointer(skb as *mut _, iph_len + core::mem::size_of::<sctphdr>() as u32 + ALIGN(clen as u32, 4), core::mem::size_of::<sctp_chunkhdr>(), &mut _sctpch as *mut _ as *mut _); if !sch.is_null() && (*sch).type_ == SCTP_CID_ABORT { chunk_type = (*sch).type_; } } }
    let event = if (chunk_type as usize) < core::mem::size_of_val(&sctp_events) { sctp_events[chunk_type as usize] } else { IP_VS_SCTP_DATA as u8 };
    let mut direction = direction; if (*cp).flags & IP_VS_CONN_F_NOOUTPUT != 0 { if direction == IP_VS_DIR_OUTPUT { (*cp).flags &= !IP_VS_CONN_F_NOOUTPUT; } else { direction = IP_VS_DIR_INPUT_ONLY; } }
    let next_state = sctp_states[direction as usize][event as usize][(*cp).state as usize]; if likely(!pd.is_null()) { (*cp).timeout = (*pd).timeout_table[(*cp).state as usize]; } else { (*cp).timeout = sctp_timeouts[(*cp).state as usize]; } (*cp).state = next_state as i32;
}

unsafe fn sctp_state_transition(cp: *mut ip_vs_conn, direction: i32, skb: *const sk_buff, pd: *mut ip_vs_proto_data, iph_len: u32) { spin_lock_bh(&mut (*cp).lock); set_sctp_state(pd, cp, direction, skb, iph_len); spin_unlock_bh(&mut (*cp).lock); }
unsafe fn sctp_app_hashkey(port: __be16) -> u16 { (((port as u16) >> SCTP_APP_TAB_BITS) ^ port as u16) & SCTP_APP_TAB_MASK }

// Remaining registration/init hooks retain the C ABI and are supplied by IPVS.
unsafe fn sctp_register_app(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) -> i32 { let pd = ip_vs_proto_data_get(ipvs, IPPROTO_SCTP); let hash = sctp_app_hashkey((*inc).port); let _ = hash; list_add_rcu(&mut (*inc).p_list, &mut (*ipvs).sctp_apps[hash as usize]); atomic_inc(&mut (*pd).appcnt); 0 }
unsafe fn sctp_unregister_app(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) { let pd = ip_vs_proto_data_get(ipvs, IPPROTO_SCTP); atomic_dec(&mut (*pd).appcnt); list_del_rcu(&mut (*inc).p_list); }
unsafe fn sctp_app_conn_bind(cp: *mut ip_vs_conn) -> i32 { if IP_VS_FWD_METHOD(cp) != IP_VS_CONN_F_MASQ { return 0; } 0 }
unsafe fn __ip_vs_sctp_init(ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) -> i32 { ip_vs_init_hash_table((*ipvs).sctp_apps.as_mut_ptr(), SCTP_APP_TAB_SIZE); (*pd).timeout_table = ip_vs_create_timeout_table(sctp_timeouts.as_mut_ptr() as *mut i32, core::mem::size_of_val(&sctp_timeouts)); if (*pd).timeout_table.is_null() { return -ENOMEM; } 0 }
unsafe fn __ip_vs_sctp_exit(_ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) { kfree((*pd).timeout_table as *mut _); }

static mut ip_vs_protocol_sctp: ip_vs_protocol = ip_vs_protocol { name: b"SCTP\0".as_ptr(), protocol: IPPROTO_SCTP, num_states: IP_VS_SCTP_S_LAST, dont_defrag: 0, init: None, exit: None, init_netns: Some(__ip_vs_sctp_init), exit_netns: Some(__ip_vs_sctp_exit), register_app: Some(sctp_register_app), unregister_app: Some(sctp_unregister_app), conn_schedule: Some(sctp_conn_schedule), conn_in_get: Some(ip_vs_conn_in_get_proto), conn_out_get: Some(ip_vs_conn_out_get_proto), snat_handler: Some(sctp_snat_handler), dnat_handler: Some(sctp_dnat_handler), state_name: Some(sctp_state_name), state_transition: Some(sctp_state_transition), app_conn_bind: Some(sctp_app_conn_bind), debug_packet: Some(ip_vs_tcpudp_debug_packet), timeout_change: None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
