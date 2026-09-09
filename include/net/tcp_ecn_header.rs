/* SPDX-License-Identifier: GPL-2.0-or-later */

#[repr(C)]
pub enum tcp_ecn_mode {
    TCP_ECN_IN_NOECN_OUT_NOECN = 0,
    TCP_ECN_IN_ECN_OUT_ECN = 1,
    TCP_ECN_IN_ECN_OUT_NOECN = 2,
    TCP_ECN_IN_ACCECN_OUT_ACCECN = 3,
    TCP_ECN_IN_ACCECN_OUT_ECN = 4,
    TCP_ECN_IN_ACCECN_OUT_NOECN = 5,
}

#[repr(C)]
pub enum tcp_accecn_option {
    TCP_ACCECN_OPTION_DISABLED = 0,
    TCP_ACCECN_OPTION_MINIMUM = 1,
    TCP_ACCECN_OPTION_FULL = 2,
    TCP_ACCECN_OPTION_PERSIST = 3,
}

pub unsafe fn INET_ECN_xmit_ect_1_negotiation(sk: *mut sock) {
    __INET_ECN_xmit(sk, tcp_ca_ect_1_negotiation(sk));
}

pub unsafe fn tcp_ecn_queue_cwr(tp: *mut tcp_sock) {
    if tcp_ecn_mode_rfc3168(tp) { (*tp).ecn_flags |= TCP_ECN_QUEUE_CWR; }
}

pub unsafe fn tcp_ecn_accept_cwr(sk: *mut sock, skb: *const sk_buff) {
    let tp = tcp_sk(sk);
    if tcp_ecn_mode_rfc3168(tp) && (*tcp_hdr(skb)).cwr != 0 {
        (*tp).ecn_flags &= !TCP_ECN_DEMAND_CWR;
        if (*TCP_SKB_CB(skb)).seq != (*TCP_SKB_CB(skb)).end_seq {
            (*inet_csk(sk)).icsk_ack.pending |= ICSK_ACK_NOW;
        }
    }
}

pub unsafe fn tcp_ecn_withdraw_cwr(tp: *mut tcp_sock) { (*tp).ecn_flags &= !TCP_ECN_QUEUE_CWR; }
pub unsafe fn tcp_accecn_ace_fail_send(tp: *const tcp_sock) -> bool { (*tp).accecn_fail_mode & TCP_ACCECN_ACE_FAIL_SEND != 0 }
pub unsafe fn tcp_accecn_ace_fail_recv(tp: *const tcp_sock) -> bool { (*tp).accecn_fail_mode & TCP_ACCECN_ACE_FAIL_RECV != 0 }
pub unsafe fn tcp_accecn_opt_fail_send(tp: *const tcp_sock) -> bool { (*tp).accecn_fail_mode & TCP_ACCECN_OPT_FAIL_SEND != 0 }
pub unsafe fn tcp_accecn_opt_fail_recv(tp: *const tcp_sock) -> bool { (*tp).accecn_fail_mode & TCP_ACCECN_OPT_FAIL_RECV != 0 }
pub unsafe fn tcp_accecn_fail_mode_set(tp: *mut tcp_sock, mode: u8) { (*tp).accecn_fail_mode |= mode; }
pub unsafe fn tcp_accecn_ace(th: *const tcphdr) -> u8 { ((*th).ae << 2) | ((*th).cwr << 1) | (*th).ece }

pub unsafe fn tcp_accecn_extract_syn_ect(ace: u8) -> i32 {
    static ACE_TO_ECN: [i32; 8] = [INET_ECN_ECT_0, INET_ECN_ECT_1, INET_ECN_NOT_ECT, INET_ECN_ECT_1, INET_ECN_ECT_0, INET_ECN_ECT_1, INET_ECN_CE, INET_ECN_ECT_1];
    ACE_TO_ECN[(ace & 7) as usize]
}

pub unsafe fn tcp_ect_transition_valid(snt: u8, rcv: u8) -> bool {
    if rcv == snt { return true; }
    if snt == INET_ECN_NOT_ECT || rcv == INET_ECN_NOT_ECT { return false; }
    if snt == INET_ECN_CE { return false; }
    true
}

pub unsafe fn tcp_accecn_validate_syn_feedback(sk: *mut sock, ace: u8, sent_ect: u8) -> bool {
    let ect = tcp_accecn_extract_syn_ect(ace) as u8;
    let tp = tcp_sk(sk);
    if !READ_ONCE((*(*sock_net(sk)).ipv4).sysctl_tcp_ecn_fallback) { return true; }
    if !tcp_ect_transition_valid(sent_ect, ect) { tcp_accecn_fail_mode_set(tp, TCP_ACCECN_ACE_FAIL_RECV); return false; }
    true
}

pub unsafe fn tcp_accecn_saw_opt_fail_recv(tp: *mut tcp_sock, saw_opt: u8) {
    (*tp).saw_accecn_opt = saw_opt;
    if (*tp).saw_accecn_opt == TCP_ACCECN_OPT_FAIL_SEEN { tcp_accecn_fail_mode_set(tp, TCP_ACCECN_OPT_FAIL_RECV); }
}

pub unsafe fn tcp_accecn_third_ack(sk: *mut sock, skb: *const sk_buff, sent_ect: u8) {
    let ace = tcp_accecn_ace(tcp_hdr(skb));
    let tp = tcp_sk(sk);
    match ace {
        0x0 => if !(*TCP_SKB_CB(skb)).sacked { tcp_accecn_fail_mode_set(tp, TCP_ACCECN_ACE_FAIL_RECV | TCP_ACCECN_OPT_FAIL_RECV); },
        0x7 | 0x5 | 0x1 => {},
        _ => if (*TCP_SKB_CB(skb)).seq == (*TCP_SKB_CB(skb)).end_seq && !(*TCP_SKB_CB(skb)).sacked && tcp_accecn_validate_syn_feedback(sk, ace, sent_ect) {
            if tcp_accecn_extract_syn_ect(ace) == INET_ECN_CE && (*tp).delivered_ce == 0 { WRITE_ONCE((*tp).delivered_ce, 1); }
        },
    }
}

pub unsafe fn tcp_accecn_opt_demand_min(sk: *mut sock, opt_demand_min: u8) { let tp = tcp_sk(sk); (*tp).accecn_opt_demand = core::cmp::max(opt_demand_min, (*tp).accecn_opt_demand); }

pub unsafe fn tcp_ecnfield_to_accecn_optfield(ecnfield: u8) -> u8 {
    match ecnfield & INET_ECN_MASK { INET_ECN_NOT_ECT => 0, INET_ECN_ECT_1 => 1, INET_ECN_CE => 2, INET_ECN_ECT_0 => 3, _ => 0 }
}

pub unsafe fn tcp_accecn_field_init_offset(ecnfield: u8) -> u32 {
    match ecnfield & INET_ECN_MASK { INET_ECN_NOT_ECT => 0, INET_ECN_ECT_1 => TCP_ACCECN_E1B_INIT_OFFSET, INET_ECN_CE => TCP_ACCECN_CEB_INIT_OFFSET, INET_ECN_ECT_0 => TCP_ACCECN_E0B_INIT_OFFSET, _ => 0 }
}

pub unsafe fn tcp_accecn_optfield_to_ecnfield(option: u32, order: bool) -> u32 {
    static LOOKUP: [[u32; 3]; 2] = [[INET_ECN_ECT_0, INET_ECN_CE, INET_ECN_ECT_1], [INET_ECN_ECT_1, INET_ECN_CE, INET_ECN_ECT_0]];
    LOOKUP[order as usize][(option % 3) as usize]
}

pub unsafe fn tcp_update_ecn_bytes(cnt: *mut u32, from: *const i8, init_offset: u32) -> i32 {
    let truncated = (get_unaligned_be32(from.offset(-1)) - init_offset) & 0xFFFFFF;
    let mut delta = (truncated - *cnt) & 0xFFFFFF;
    delta = ((delta << 8) as i32 >> 8) as u32;
    *cnt = (*cnt).wrapping_add(delta); delta as i32
}

pub unsafe fn cookie_accecn_ok(th: *const tcphdr) -> bool { tcp_accecn_ace(th) > 1 }

pub unsafe fn tcp_accecn_reflector_flags(ect: u8) -> u16 {
    static FLAGS: [u8; 4] = [0b010, 0b011, 0b100, 0b110];
    FIELD_PREP(TCPHDR_ACE, FLAGS[(ect & 3) as usize])
}

pub unsafe fn tcp_accecn_syn_requested(th: *const tcphdr) -> bool { let ace = tcp_accecn_ace(th); ace != 0 && ace != 3 }

pub unsafe fn __tcp_accecn_init_bytes_counters(counter_array: *mut i32) {
    *counter_array.add((INET_ECN_ECT_1 - 1) as usize) = 0;
    *counter_array.add((INET_ECN_ECT_0 - 1) as usize) = 0;
    *counter_array.add((INET_ECN_CE - 1) as usize) = 0;
}

pub unsafe fn tcp_accecn_init_counters(tp: *mut tcp_sock) {
    (*tp).received_ce = 0; (*tp).received_ce_pending = 0;
    __tcp_accecn_init_bytes_counters((*tp).received_ecn_bytes.as_mut_ptr());
    __tcp_accecn_init_bytes_counters((*tp).delivered_ecn_bytes.as_mut_ptr());
    (*tp).accecn_opt_sent_w_dsack = 0; (*tp).accecn_minlen = 0; (*tp).accecn_opt_demand = 0; (*tp).est_ecnfield = 0;
}

pub unsafe fn tcp_accecn_echo_syn_ect(th: *mut tcphdr, ect: u8) { (*th).ae = ((ect & INET_ECN_ECT_0) != 0) as u8; (*th).cwr = (ect != INET_ECN_ECT_0) as u8; (*th).ece = (ect == INET_ECN_ECT_1) as u8; }

pub unsafe fn tcp_accecn_set_ace(tp: *mut tcp_sock, skb: *mut sk_buff, th: *mut tcphdr) {
    if (*TCP_SKB_CB(skb)).tcp_flags & TCPHDR_ACE == 0 {
        let wire_ace = (*tp).received_ce + TCP_ACCECN_CEP_INIT_OFFSET;
        (*th).ece = (wire_ace & 1 != 0) as u8; (*th).cwr = (wire_ace & 2 != 0) as u8; (*th).ae = (wire_ace & 4 != 0) as u8;
        (*tp).received_ce_pending = 0;
    }
}

pub unsafe fn tcp_accecn_option_init(skb: *const sk_buff, opt_offset: u8) -> u8 {
    let ptr = skb_transport_header(skb).add(opt_offset as usize);
    let optlen = *ptr.add(1) as usize - 2;
    if (*ptr != TCPOPT_ACCECN0 && *ptr != TCPOPT_ACCECN1) { return TCP_ACCECN_OPT_FAIL_SEEN; }
    let ptr = ptr.add(2);
    if optlen < TCPOLEN_ACCECN_PERFIELD as usize { return TCP_ACCECN_OPT_EMPTY_SEEN; }
    if get_unaligned_be24(ptr) == 0 { return TCP_ACCECN_OPT_FAIL_SEEN; }
    if optlen < (TCPOLEN_ACCECN_PERFIELD * 3) as usize { return TCP_ACCECN_OPT_COUNTER_SEEN; }
    if get_unaligned_be24(ptr.add((TCPOLEN_ACCECN_PERFIELD * 2) as usize)) == 0 { return TCP_ACCECN_OPT_FAIL_SEEN; }
    TCP_ACCECN_OPT_COUNTER_SEEN
}

pub unsafe fn tcp_ecn_rcv_synack_accecn(sk: *mut sock, skb: *const sk_buff, dsf: u8) {
    let tp = tcp_sk(sk); tcp_ecn_mode_set(tp, TCP_ECN_MODE_ACCECN); (*tp).syn_ect_rcv = dsf & INET_ECN_MASK;
    if (*tp).rx_opt.accecn && (*tp).saw_accecn_opt < TCP_ACCECN_OPT_COUNTER_SEEN {
        let saw_opt = tcp_accecn_option_init(skb, (*tp).rx_opt.accecn); tcp_accecn_saw_opt_fail_recv(tp, saw_opt); (*tp).accecn_opt_demand = 2;
    }
}

pub unsafe fn tcp_ecn_rcv_ecn_echo(tp: *const tcp_sock, th: *const tcphdr) -> bool { (*th).ece != 0 && (*th).syn == 0 && tcp_ecn_mode_rfc3168(tp) }

pub unsafe fn tcp_ecn_clear_syn(sk: *mut sock, skb: *mut sk_buff) {
    if READ_ONCE((*(*sock_net(sk)).ipv4).sysctl_tcp_ecn_fallback) { (*TCP_SKB_CB(skb)).tcp_flags &= !TCPHDR_ACE; }
}

pub unsafe fn tcp_ecn_received_counters_payload(sk: *mut sock, skb: *const sk_buff) {
    let th = skb->data as *const tcphdr;
    tcp_ecn_received_counters(sk, skb, skb->len - ((*th).doff as u32) * 4);
}

pub unsafe fn tcp_ecn_received_counters(sk: *mut sock, skb: *const sk_buff, len: u32) {
    let ecnfield = (*TCP_SKB_CB(skb)).ip_dsfield & INET_ECN_MASK;
    let is_ce = INET_ECN_is_ce(ecnfield); let tp = tcp_sk(sk);
    if !INET_ECN_is_not_ect(ecnfield) {
        let pcount = is_ce * core::cmp::max(1, (*skb_shinfo(skb)).gso_segs as u8) as u32;
        if !tcp_ecn_mode_rfc3168(tp) { (*tp).ecn_flags |= TCP_ECN_SEEN; }
        (*tp).received_ce += pcount; (*tp).received_ce_pending = core::cmp::min((*tp).received_ce_pending + pcount, 0xf);
        if len > 0 { let minlen = tcp_ecnfield_to_accecn_optfield(ecnfield); let idx = (ecnfield - 1) as usize; let old = (*tp).received_ecn_bytes[idx]; (*tp).received_ecn_bytes[idx] += len; (*tp).accecn_minlen = core::cmp::max((*tp).accecn_minlen, minlen); if ((*tp).received_ecn_bytes[idx] ^ old) & GENMASK_U32(31,22) != 0 { tcp_accecn_opt_demand_min(sk, 1); } }
    }
    let edge = (*tp).prev_ecnfield != ecnfield;
    if edge || is_ce { (*tp).prev_ecnfield = ecnfield; if tcp_ecn_mode_accecn(tp) { if edge { (*inet_csk(sk)).icsk_ack.pending |= ICSK_ACK_NOW; } (*tp).accecn_opt_demand = 2; } }
}

pub unsafe fn tcp_ecn_send_synack(sk: *mut sock, skb: *mut sk_buff) { let tp=tcp_sk(sk); (*TCP_SKB_CB(skb)).tcp_flags &= !TCPHDR_CWR; if tcp_ecn_disabled(tp) { (*TCP_SKB_CB(skb)).tcp_flags &= !TCPHDR_ECE; } else if tcp_ca_needs_ecn(sk) || tcp_bpf_ca_needs_ecn(sk) { INET_ECN_xmit_ect_1_negotiation(sk); } if (*tp).ecn_flags & TCP_ECN_MODE_ACCECN != 0 { (*TCP_SKB_CB(skb)).tcp_flags &= !TCPHDR_ACE; (*TCP_SKB_CB(skb)).tcp_flags |= tcp_accecn_reflector_flags((*tp).syn_ect_rcv); (*tp).syn_ect_snt = (*inet_sk(sk)).tos & INET_ECN_MASK; } }

pub unsafe fn tcp_ecn_rcv_syn(sk: *mut sock, th: *const tcphdr, skb: *const sk_buff) { let tp=tcp_sk(sk); if tcp_ecn_mode_pending(tp) { if !tcp_accecn_syn_requested(th) { tcp_ecn_mode_set(tp,TCP_ECN_MODE_RFC3168); } else { (*tp).syn_ect_rcv=(*TCP_SKB_CB(skb)).ip_dsfield&INET_ECN_MASK; (*tp).prev_ecnfield=(*tp).syn_ect_rcv; tcp_ecn_mode_set(tp,TCP_ECN_MODE_ACCECN); } } if tcp_ecn_mode_rfc3168(tp) && ((*th).ece==0 || (*th).cwr==0 || tcp_ca_no_fallback_rfc3168(sk)) { tcp_ecn_mode_set(tp,TCP_ECN_DISABLED); } }

pub unsafe fn tcp_accecn_option_beacon_check(sk: *const sock) -> bool {
    let ecn_beacon = READ_ONCE((*(*sock_net(sk)).ipv4).sysctl_tcp_ecn_option_beacon); let tp = tcp_sk(sk as *mut sock);
    if ecn_beacon == 0 { return false; }
    tcp_stamp_us_delta((*tp).tcp_mstamp, (*tp).accecn_opt_tstamp) * ecn_beacon >= ((*tp).srtt_us >> 3)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
