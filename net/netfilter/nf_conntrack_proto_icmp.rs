// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2006-2010 Patrick McHardy <kaber@trash.net>
 */

// External Linux kernel declarations supplied by other translation units.

static NF_CT_ICMP_TIMEOUT: u32 = 30 * HZ;

pub unsafe fn icmp_pkt_to_tuple(
    skb: *const sk_buff,
    dataoff: c_uint,
    _net: *mut net,
    tuple: *mut nf_conntrack_tuple,
) -> bool {
    let mut hdr = core::mem::MaybeUninit::<icmphdr>::uninit();
    let hp = skb_header_pointer(
        skb,
        dataoff,
        core::mem::size_of::<icmphdr>(),
        hdr.as_mut_ptr() as *mut c_void,
    );
    if hp.is_null() {
        return false;
    }
    (*tuple).dst.u.icmp.type_ = (*hp).type_;
    (*tuple).src.u.icmp.id = (*hp).un.echo.id;
    (*tuple).dst.u.icmp.code = (*hp).code;
    true
}

/* Add 1; spaces filled with 0. */
static INVMAP: [u8; 19] = [
    ICMP_ECHOREPLY + 1, 0, 0, 0, 0, 0, 0, 0,
    ICMP_ECHO + 1, 0, 0, 0, 0, ICMP_TIMESTAMPREPLY + 1,
    ICMP_TIMESTAMP + 1, ICMP_INFO_REPLY + 1, ICMP_INFO_REQUEST + 1,
    ICMP_ADDRESSREPLY + 1, ICMP_ADDRESS + 1,
];

pub unsafe fn nf_conntrack_invert_icmp_tuple(
    tuple: *mut nf_conntrack_tuple,
    orig: *const nf_conntrack_tuple,
) -> bool {
    let ty = (*orig).dst.u.icmp.type_ as usize;
    if ty >= INVMAP.len() || INVMAP[ty] == 0 {
        return false;
    }
    (*tuple).src.u.icmp.id = (*orig).src.u.icmp.id;
    (*tuple).dst.u.icmp.type_ = INVMAP[ty] - 1;
    (*tuple).dst.u.icmp.code = (*orig).dst.u.icmp.code;
    true
}

/* Returns verdict for packet, or -1 for invalid. */
pub unsafe fn nf_conntrack_icmp_packet(
    ct: *mut nf_conn,
    skb: *mut sk_buff,
    ctinfo: ip_conntrack_info,
    state: *const nf_hook_state,
) -> c_int {
    /* Do not immediately delete the connection after the first
       successful reply to avoid excessive conntrackd traffic
       and also to handle correctly ICMP echo reply duplicates. */
    let mut timeout = nf_ct_timeout_lookup(ct);
    let valid_new: [u8; 19] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0,
    ];

    if (*state).pf != NFPROTO_IPV4 {
        return -NF_ACCEPT;
    }
    let ty = (*ct).tuplehash[0].tuple.dst.u.icmp.type_ as usize;
    if ty >= valid_new.len() || valid_new[ty] == 0 {
        pr_debug!("icmp: can't create new conn with type %u\n", (*ct).tuplehash[0].tuple.dst.u.icmp.type_);
        nf_ct_dump_tuple_ip(&(*ct).tuplehash[0].tuple);
        return -NF_ACCEPT;
    }
    if timeout.is_null() {
        timeout = &mut nf_icmp_pernet(nf_ct_net(ct)).timeout;
    }
    nf_ct_refresh_acct(ct, ctinfo, skb, *timeout);
    NF_ACCEPT
}

/* Check inner header is related to any of the existing connections */
pub unsafe fn nf_conntrack_inet_error(
    tmpl: *mut nf_conn,
    skb: *mut sk_buff,
    dataoff: c_uint,
    state: *const nf_hook_state,
    l4proto: u8,
    outer_daddr: *mut nf_inet_addr,
) -> c_int {
    let mut innertuple = core::mem::zeroed::<nf_conntrack_tuple>();
    let mut origtuple = core::mem::zeroed::<nf_conntrack_tuple>();
    let mut tmp = core::mem::zeroed::<nf_conntrack_zone>();
    WARN_ON(skb_nfct(skb));
    let zone = nf_ct_zone_tmpl(tmpl, skb, &mut tmp);
    if !nf_ct_get_tuplepr(skb, dataoff, (*state).pf, (*state).net, &mut origtuple) { return -NF_ACCEPT; }
    if !nf_ct_invert_tuple(&mut innertuple, &origtuple) { return -NF_ACCEPT; }
    let h = nf_conntrack_find_get((*state).net, zone, &innertuple);
    if h.is_null() { return -NF_ACCEPT; }
    let ct = nf_ct_tuplehash_to_ctrack(h);
    let dir = NF_CT_DIRECTION(h);
    let ct_daddr = &(*ct).tuplehash[dir as usize].tuple.dst.u3;
    if !nf_inet_addr_cmp(outer_daddr, ct_daddr) {
        nf_l4proto_log_invalid(skb, state, l4proto, "outer daddr != inner daddr");
        nf_ct_put(ct);
        return -NF_ACCEPT;
    }
    let mut ctinfo = IP_CT_RELATED;
    if dir == IP_CT_DIR_REPLY { ctinfo += IP_CT_IS_REPLY; }
    nf_ct_set(skb, ct, ctinfo);
    NF_ACCEPT
}

unsafe fn icmp_error_log(skb: *const sk_buff, state: *const nf_hook_state, msg: *const c_char) {
    nf_l4proto_log_invalid(skb, state, IPPROTO_ICMP, "%s", msg);
}

/* Small and modified version of icmp_rcv */
pub unsafe fn nf_conntrack_icmpv4_error(
    tmpl: *mut nf_conn,
    skb: *mut sk_buff,
    mut dataoff: c_uint,
    state: *const nf_hook_state,
) -> c_int {
    let mut ih = core::mem::MaybeUninit::<icmphdr>::uninit();
    let icmph = skb_header_pointer(skb, dataoff, core::mem::size_of::<icmphdr>(), ih.as_mut_ptr() as *mut c_void);
    if icmph.is_null() { icmp_error_log(skb, state, c_str!("short packet")); return -NF_ACCEPT; }
    if (*state).net.ct.sysctl_checksum && (*state).hook == NF_INET_PRE_ROUTING && nf_ip_checksum(skb, (*state).hook, dataoff, IPPROTO_ICMP) { icmp_error_log(skb, state, c_str!("bad hw icmp checksum")); return -NF_ACCEPT; }
    if (*icmph).type_ > NR_ICMP_TYPES { icmp_error_log(skb, state, c_str!("invalid icmp type")); return -NF_ACCEPT; }
    if !icmp_is_err((*icmph).type_) { return NF_ACCEPT; }
    let mut outer_daddr = core::mem::zeroed::<nf_inet_addr>();
    outer_daddr.ip = ip_hdr(skb).daddr;
    dataoff += core::mem::size_of::<icmphdr>() as c_uint;
    nf_conntrack_inet_error(tmpl, skb, dataoff, state, IPPROTO_ICMP, &mut outer_daddr)
}

// CONFIG_NF_CT_NETLINK conditional section.
unsafe fn icmp_tuple_to_nlattr(skb: *mut sk_buff, t: *const nf_conntrack_tuple) -> c_int {
    if nla_put_be16(skb, CTA_PROTO_ICMP_ID, (*t).src.u.icmp.id)
        || nla_put_u8(skb, CTA_PROTO_ICMP_TYPE, (*t).dst.u.icmp.type_)
        || nla_put_u8(skb, CTA_PROTO_ICMP_CODE, (*t).dst.u.icmp.code) { return -1; }
    0
}

static ICMP_NLA_POLICY: [nla_policy; CTA_PROTO_MAX as usize + 1] = [nla_policy { type_: 0 }; CTA_PROTO_MAX as usize + 1];

unsafe fn icmp_nlattr_to_tuple(tb: *mut *mut nlattr, tuple: *mut nf_conntrack_tuple, flags: u32) -> c_int {
    if flags & CTA_FILTER_FLAG(CTA_PROTO_ICMP_TYPE) != 0 {
        if (*tb.add(CTA_PROTO_ICMP_TYPE as usize)).is_null() { return -EINVAL; }
        (*tuple).dst.u.icmp.type_ = nla_get_u8(*tb.add(CTA_PROTO_ICMP_TYPE as usize));
        let ty = (*tuple).dst.u.icmp.type_ as usize;
        if ty >= INVMAP.len() || INVMAP[ty] == 0 { return -EINVAL; }
    }
    if flags & CTA_FILTER_FLAG(CTA_PROTO_ICMP_CODE) != 0 {
        if (*tb.add(CTA_PROTO_ICMP_CODE as usize)).is_null() { return -EINVAL; }
        (*tuple).dst.u.icmp.code = nla_get_u8(*tb.add(CTA_PROTO_ICMP_CODE as usize));
    }
    if flags & CTA_FILTER_FLAG(CTA_PROTO_ICMP_ID) != 0 {
        if (*tb.add(CTA_PROTO_ICMP_ID as usize)).is_null() { return -EINVAL; }
        (*tuple).src.u.icmp.id = nla_get_be16(*tb.add(CTA_PROTO_ICMP_ID as usize));
    }
    0
}

unsafe fn icmp_nlattr_tuple_size() -> c_uint {
    static mut SIZE: c_uint = 0;
    if SIZE == 0 { SIZE = nla_policy_len(ICMP_NLA_POLICY.as_ptr(), CTA_PROTO_MAX + 1); }
    SIZE
}

// CONFIG_NF_CONNTRACK_TIMEOUT conditional section.
unsafe fn icmp_timeout_nlattr_to_obj(tb: *mut *mut nlattr, net: *mut net, data: *mut c_void) -> c_int {
    let in_ = nf_icmp_pernet(net);
    let mut timeout = data as *mut c_uint;
    if !(*tb.add(CTA_TIMEOUT_ICMP_TIMEOUT as usize)).is_null() {
        if timeout.is_null() { timeout = &mut (*in_).timeout; }
        *timeout = ntohl(nla_get_be32(*tb.add(CTA_TIMEOUT_ICMP_TIMEOUT as usize))) * HZ;
    } else if !timeout.is_null() { *timeout = (*in_).timeout; }
    0
}

unsafe fn icmp_timeout_obj_to_nlattr(skb: *mut sk_buff, data: *const c_void) -> c_int {
    let timeout = data as *const c_uint;
    if nla_put_be32(skb, CTA_TIMEOUT_ICMP_TIMEOUT, htonl(*timeout / HZ)) { return -ENOSPC; }
    0
}

pub unsafe fn nf_conntrack_icmp_init_net(net: *mut net) {
    nf_icmp_pernet(net).timeout = NF_CT_ICMP_TIMEOUT;
}

pub static nf_conntrack_l4proto_icmp: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_ICMP,
    allow_clash: true,
    #[cfg(feature = "nf_ct_netlink")]
    tuple_to_nlattr: Some(icmp_tuple_to_nlattr),
    #[cfg(feature = "nf_ct_netlink")]
    nlattr_tuple_size: Some(icmp_nlattr_tuple_size),
    #[cfg(feature = "nf_ct_netlink")]
    nlattr_to_tuple: Some(icmp_nlattr_to_tuple),
    #[cfg(feature = "nf_ct_netlink")]
    nla_policy: Some(&ICMP_NLA_POLICY),
    ..core::mem::zeroed()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
