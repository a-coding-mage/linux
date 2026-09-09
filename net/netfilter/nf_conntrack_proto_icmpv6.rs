// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C)2003,2004 USAGI/WIDE Project
 *
 * Author:
 *	Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 */

// Kernel dependencies supplied by the surrounding translation unit.

static NF_CT_ICMPV6_TIMEOUT: c_uint = 30 * HZ;

pub unsafe fn icmpv6_pkt_to_tuple(
    skb: *const sk_buff, dataoff: c_uint, _net: *mut net,
    tuple: *mut nf_conntrack_tuple,
) -> bool {
    let mut hdr: icmp6hdr = core::mem::zeroed();
    let hp = skb_header_pointer(skb, dataoff, core::mem::size_of::<icmp6hdr>() as c_uint, &mut hdr as *mut _ as *mut _);
    if hp.is_null() { return false; }
    (*tuple).dst.u.icmp.type_ = (*hp).icmp6_type;
    (*tuple).src.u.icmp.id = (*hp).icmp6_identifier;
    (*tuple).dst.u.icmp.code = (*hp).icmp6_code;
    true
}

/* Add 1; spaces filled with 0. */
static INV_MAP: [u8; 256] = {
    let mut a = [0u8; 256];
    a[(ICMPV6_ECHO_REQUEST - 128) as usize] = ICMPV6_ECHO_REPLY + 1;
    a[(ICMPV6_ECHO_REPLY - 128) as usize] = ICMPV6_ECHO_REQUEST + 1;
    a[(ICMPV6_NI_QUERY - 128) as usize] = ICMPV6_NI_REPLY + 1;
    a[(ICMPV6_NI_REPLY - 128) as usize] = ICMPV6_NI_QUERY + 1;
    a
};

static NOCT_VALID_NEW: [u8; 256] = {
    let mut a = [0u8; 256];
    a[(ICMPV6_MGM_QUERY - 130) as usize] = 1;
    a[(ICMPV6_MGM_REPORT - 130) as usize] = 1;
    a[(ICMPV6_MGM_REDUCTION - 130) as usize] = 1;
    a[(NDISC_ROUTER_SOLICITATION - 130) as usize] = 1;
    a[(NDISC_ROUTER_ADVERTISEMENT - 130) as usize] = 1;
    a[(NDISC_NEIGHBOUR_SOLICITATION - 130) as usize] = 1;
    a[(NDISC_NEIGHBOUR_ADVERTISEMENT - 130) as usize] = 1;
    a[(ICMPV6_MLD2_REPORT - 130) as usize] = 1;
    a[(ICMPV6_MRDISC_ADV - 130) as usize] = 1;
    a[(ICMPV6_MRDISC_SOL - 130) as usize] = 1;
    a
};

pub unsafe fn nf_conntrack_invert_icmpv6_tuple(tuple: *mut nf_conntrack_tuple, orig: *const nf_conntrack_tuple) -> bool {
    let ty = (*orig).dst.u.icmp.type_ as i32 - 128;
    if ty < 0 || ty >= INV_MAP.len() as i32 || INV_MAP[ty as usize] == 0 { return false; }
    (*tuple).src.u.icmp.id = (*orig).src.u.icmp.id;
    (*tuple).dst.u.icmp.type_ = INV_MAP[ty as usize] - 1;
    (*tuple).dst.u.icmp.code = (*orig).dst.u.icmp.code;
    true
}

unsafe fn icmpv6_get_timeouts(net: *mut net) -> *mut c_uint {
    &mut nf_icmpv6_pernet(net).timeout
}

/* Returns verdict for packet, or -1 for invalid. */
pub unsafe fn nf_conntrack_icmpv6_packet(ct: *mut nf_conn, skb: *mut sk_buff, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> c_int {
    let mut timeout = nf_ct_timeout_lookup(ct);
    let mut valid_new = [0u8; 256];
    valid_new[(ICMPV6_ECHO_REQUEST - 128) as usize] = 1;
    valid_new[(ICMPV6_NI_QUERY - 128) as usize] = 1;
    if (*state).pf != NFPROTO_IPV6 { return -NF_ACCEPT; }
    if !nf_ct_is_confirmed(ct) {
        let ty = (*ct).tuplehash[0].tuple.dst.u.icmp.type_ as i32 - 128;
        if ty < 0 || ty >= valid_new.len() as i32 || valid_new[ty as usize] == 0 {
            pr_debug!("icmpv6: can't create new conn with type %u\n", ty + 128);
            nf_ct_dump_tuple_ipv6(&(*ct).tuplehash[0].tuple);
            return -NF_ACCEPT;
        }
    }
    if timeout.is_null() { timeout = icmpv6_get_timeouts(nf_ct_net(ct)); }
    nf_ct_refresh_acct(ct, ctinfo, skb, *timeout);
    NF_ACCEPT
}

unsafe fn icmpv6_error_log(skb: *const sk_buff, state: *const nf_hook_state, msg: *const c_char) {
    nf_l4proto_log_invalid(skb, state, IPPROTO_ICMPV6, c"%s".as_ptr(), msg);
}

unsafe fn nf_conntrack_icmpv6_redirect(tmpl: *mut nf_conn, skb: *mut sk_buff, mut dataoff: c_uint, state: *const nf_hook_state) -> c_int {
    let hl = (*ipv6_hdr(skb)).hop_limit;
    let mut tmp: rd_msg = core::mem::zeroed();
    let rd = skb_header_pointer(skb, dataoff, core::mem::size_of::<rd_msg>() as c_uint, &mut tmp as *mut _ as *mut _);
    if rd.is_null() { icmpv6_error_log(skb, state, c"short redirect".as_ptr()); return -NF_ACCEPT; }
    if (*rd).icmph.icmp6_code != 0 { return NF_ACCEPT; }
    if hl != 255 || (ipv6_addr_type(&(*ipv6_hdr(skb)).saddr) & IPV6_ADDR_LINKLOCAL) == 0 { icmpv6_error_log(skb, state, c"invalid saddr or hoplimit for redirect".as_ptr()); return -NF_ACCEPT; }
    dataoff += core::mem::size_of::<rd_msg>() as c_uint;
    let nd = skb_header_pointer(skb, dataoff, core::mem::size_of::<nd_opt_hdr>() as c_uint, &mut tmp as *mut _ as *mut _);
    if nd.is_null() || (*nd).nd_opt_len == 0 { icmpv6_error_log(skb, state, c"redirect without options".as_ptr()); return -NF_ACCEPT; }
    if (*nd).nd_opt_type != ND_OPT_REDIRECT_HDR { return NF_ACCEPT; }
    let mut outer: nf_inet_addr = core::mem::zeroed();
    core::ptr::copy_nonoverlapping((&(*ipv6_hdr(skb)).daddr) as *const _, (&mut outer.ip6) as *mut _, 1);
    dataoff += 8;
    nf_conntrack_inet_error(tmpl, skb, dataoff, state, IPPROTO_ICMPV6, &outer)
}

pub unsafe fn nf_conntrack_icmpv6_error(tmpl: *mut nf_conn, skb: *mut sk_buff, mut dataoff: c_uint, state: *const nf_hook_state) -> c_int {
    let mut ih: icmp6hdr = core::mem::zeroed();
    let icmp = skb_header_pointer(skb, dataoff, core::mem::size_of::<icmp6hdr>() as c_uint, &mut ih as *mut _ as *mut _);
    if icmp.is_null() { icmpv6_error_log(skb, state, c"short packet".as_ptr()); return -NF_ACCEPT; }
    if (*state).hook == NF_INET_PRE_ROUTING && (*(*state).net).ct.sysctl_checksum != 0 && nf_ip6_checksum(skb, (*state).hook, dataoff, IPPROTO_ICMPV6) != 0 { icmpv6_error_log(skb, state, c"ICMPv6 checksum failed".as_ptr()); return -NF_ACCEPT; }
    let ty = (*icmp).icmp6_type as i32 - 130;
    if ty >= 0 && ty < NOCT_VALID_NEW.len() as i32 && NOCT_VALID_NEW[ty as usize] != 0 { nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED); return NF_ACCEPT; }
    if (*icmp).icmp6_type == NDISC_REDIRECT { return nf_conntrack_icmpv6_redirect(tmpl, skb, dataoff, state); }
    if (*icmp).icmp6_type >= 128 { return NF_ACCEPT; }
    let mut outer: nf_inet_addr = core::mem::zeroed();
    core::ptr::copy_nonoverlapping((&(*ipv6_hdr(skb)).daddr) as *const _, (&mut outer.ip6) as *mut _, 1);
    dataoff += core::mem::size_of::<icmp6hdr>() as c_uint;
    nf_conntrack_inet_error(tmpl, skb, dataoff, state, IPPROTO_ICMPV6, &outer)
}

// Netlink and timeout translations are conditionally compiled by the kernel configuration.
// Their declarations remain represented by the external protocol registration below.

pub unsafe fn nf_conntrack_icmpv6_init_net(net: *mut net) {
    nf_icmpv6_pernet(net).timeout = NF_CT_ICMPV6_TIMEOUT;
}

#[no_mangle]
pub static nf_conntrack_l4proto_icmpv6: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_ICMPV6,
    allow_clash: true,
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
