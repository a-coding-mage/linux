// SPDX-License-Identifier: GPL-2.0-only
/*
 * xt_conntrack - Netfilter module to match connection tracking
 * information. (Superset of Rusty's minimalistic state match.)
 *
 * (C) 2001  Marc Boucher (marc@mbsi.ca).
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 * Copyright © CC Computer Consultants GmbH, 2007 - 2008
 */
// C dependencies: linux/module.h, linux/skbuff.h, net/ipv6.h,
// linux/netfilter/x_tables.h, linux/netfilter/xt_conntrack.h,
// net/netfilter/nf_conntrack.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Marc Boucher <marc@mbsi.ca>");
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("Xtables: connection tracking state match");
// MODULE_ALIAS("ipt_conntrack");
// MODULE_ALIAS("ip6t_conntrack");

unsafe fn conntrack_addrcmp(
    kaddr: *const nf_inet_addr,
    uaddr: *const nf_inet_addr,
    umask: *const nf_inet_addr,
    l3proto: u32,
) -> bool {
    if l3proto == NFPROTO_IPV4 {
        ((*kaddr).ip ^ (*uaddr).ip) & (*umask).ip == 0
    } else if l3proto == NFPROTO_IPV6 {
        ipv6_masked_addr_cmp(&(*kaddr).in6, &(*umask).in6, &(*uaddr).in6) == 0
    } else {
        false
    }
}

unsafe fn conntrack_mt_origsrc(ct: *const nf_conn, info: *const xt_conntrack_mtinfo2, family: u8) -> bool {
    conntrack_addrcmp(&(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u3, &(*info).origsrc_addr, &(*info).origsrc_mask, family as u32)
}

unsafe fn conntrack_mt_origdst(ct: *const nf_conn, info: *const xt_conntrack_mtinfo2, family: u8) -> bool {
    conntrack_addrcmp(&(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple.dst.u3, &(*info).origdst_addr, &(*info).origdst_mask, family as u32)
}

unsafe fn conntrack_mt_replsrc(ct: *const nf_conn, info: *const xt_conntrack_mtinfo2, family: u8) -> bool {
    conntrack_addrcmp(&(*ct).tuplehash[IP_CT_DIR_REPLY].tuple.src.u3, &(*info).replsrc_addr, &(*info).replsrc_mask, family as u32)
}

unsafe fn conntrack_mt_repldst(ct: *const nf_conn, info: *const xt_conntrack_mtinfo2, family: u8) -> bool {
    conntrack_addrcmp(&(*ct).tuplehash[IP_CT_DIR_REPLY].tuple.dst.u3, &(*info).repldst_addr, &(*info).repldst_mask, family as u32)
}

unsafe fn ct_proto_port_check(info: *const xt_conntrack_mtinfo2, ct: *const nf_conn) -> bool {
    let mut tuple = &(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    if ((*info).match_flags & XT_CONNTRACK_PROTO) != 0
        && ((nf_ct_protonum(ct) == (*info).l4proto) != (((*info).invert_flags & XT_CONNTRACK_PROTO) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGSRC_PORT) != 0
        && ((tuple.src.u.all == (*info).origsrc_port) != (((*info).invert_flags & XT_CONNTRACK_ORIGSRC_PORT) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGDST_PORT) != 0
        && ((tuple.dst.u.all == (*info).origdst_port) != (((*info).invert_flags & XT_CONNTRACK_ORIGDST_PORT) == 0)) { return false; }
    tuple = &(*ct).tuplehash[IP_CT_DIR_REPLY].tuple;
    if ((*info).match_flags & XT_CONNTRACK_REPLSRC_PORT) != 0
        && ((tuple.src.u.all == (*info).replsrc_port) != (((*info).invert_flags & XT_CONNTRACK_REPLSRC_PORT) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_REPLDST_PORT) != 0
        && ((tuple.dst.u.all == (*info).repldst_port) != (((*info).invert_flags & XT_CONNTRACK_REPLDST_PORT) == 0)) { return false; }
    true
}

unsafe fn port_match(min: u16, max: u16, port: u16, invert: bool) -> bool { (port >= min && port <= max) != invert }

unsafe fn ct_proto_port_check_v3(info: *const xt_conntrack_mtinfo3, ct: *const nf_conn) -> bool {
    let mut tuple = &(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    if ((*info).match_flags & XT_CONNTRACK_PROTO) != 0
        && ((nf_ct_protonum(ct) == (*info).l4proto) != (((*info).invert_flags & XT_CONNTRACK_PROTO) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGSRC_PORT) != 0 && !port_match((*info).origsrc_port, (*info).origsrc_port_high, ntohs(tuple.src.u.all), ((*info).invert_flags & XT_CONNTRACK_ORIGSRC_PORT) != 0) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGDST_PORT) != 0 && !port_match((*info).origdst_port, (*info).origdst_port_high, ntohs(tuple.dst.u.all), ((*info).invert_flags & XT_CONNTRACK_ORIGDST_PORT) != 0) { return false; }
    tuple = &(*ct).tuplehash[IP_CT_DIR_REPLY].tuple;
    if ((*info).match_flags & XT_CONNTRACK_REPLSRC_PORT) != 0 && !port_match((*info).replsrc_port, (*info).replsrc_port_high, ntohs(tuple.src.u.all), ((*info).invert_flags & XT_CONNTRACK_REPLSRC_PORT) != 0) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_REPLDST_PORT) != 0 && !port_match((*info).repldst_port, (*info).repldst_port_high, ntohs(tuple.dst.u.all), ((*info).invert_flags & XT_CONNTRACK_REPLDST_PORT) != 0) { return false; }
    true
}

unsafe fn conntrack_mt(skb: *const sk_buff, par: *mut xt_action_param, state_mask: u16, status_mask: u16) -> bool {
    let info = (*par).matchinfo as *const xt_conntrack_mtinfo2;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    let mut statebit = if !ct.is_null() { XT_CONNTRACK_STATE_BIT(ctinfo) } else if ctinfo == IP_CT_UNTRACKED { XT_CONNTRACK_STATE_UNTRACKED } else { XT_CONNTRACK_STATE_INVALID };
    if ((*info).match_flags & XT_CONNTRACK_STATE) != 0 {
        if !ct.is_null() { if test_bit(IPS_SRC_NAT_BIT, &(*ct).status) { statebit |= XT_CONNTRACK_STATE_SNAT; } if test_bit(IPS_DST_NAT_BIT, &(*ct).status) { statebit |= XT_CONNTRACK_STATE_DNAT; } }
        if ((state_mask & statebit) != 0) != (((*info).invert_flags & XT_CONNTRACK_STATE) == 0) { return false; }
    }
    if ct.is_null() { return ((*info).match_flags & XT_CONNTRACK_STATE) != 0; }
    if ((*info).match_flags & XT_CONNTRACK_DIRECTION) != 0 && ((CTINFO2DIR(ctinfo) == IP_CT_DIR_ORIGINAL) != (((*info).invert_flags & XT_CONNTRACK_DIRECTION) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGSRC) != 0 && (conntrack_mt_origsrc(ct, info, xt_family(par)) != (((*info).invert_flags & XT_CONNTRACK_ORIGSRC) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_ORIGDST) != 0 && (conntrack_mt_origdst(ct, info, xt_family(par)) != (((*info).invert_flags & XT_CONNTRACK_ORIGDST) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_REPLSRC) != 0 && (conntrack_mt_replsrc(ct, info, xt_family(par)) != (((*info).invert_flags & XT_CONNTRACK_REPLSRC) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_REPLDST) != 0 && (conntrack_mt_repldst(ct, info, xt_family(par)) != (((*info).invert_flags & XT_CONNTRACK_REPLDST) == 0)) { return false; }
    if (*(*par).match).revision != 3 { if !ct_proto_port_check(info, ct) { return false; } } else if !ct_proto_port_check_v3((*par).matchinfo as *const xt_conntrack_mtinfo3, ct) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_STATUS) != 0 && (((status_mask as u32 & (*ct).status) != 0) != (((*info).invert_flags & XT_CONNTRACK_STATUS) == 0)) { return false; }
    if ((*info).match_flags & XT_CONNTRACK_EXPIRES) != 0 { let expires = nf_ct_expires(ct) / HZ; if ((expires >= (*info).expires_min && expires <= (*info).expires_max) != (((*info).invert_flags & XT_CONNTRACK_EXPIRES) == 0)) { return false; } }
    true
}

unsafe fn conntrack_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool { let info = (*par).matchinfo as *const xt_conntrack_mtinfo1; conntrack_mt(skb, par, (*info).state_mask, (*info).status_mask) }
unsafe fn conntrack_mt_v2(skb: *const sk_buff, par: *mut xt_action_param) -> bool { let info = (*par).matchinfo as *const xt_conntrack_mtinfo2; conntrack_mt(skb, par, (*info).state_mask, (*info).status_mask) }
unsafe fn conntrack_mt_v3(skb: *const sk_buff, par: *mut xt_action_param) -> bool { let info = (*par).matchinfo as *const xt_conntrack_mtinfo3; conntrack_mt(skb, par, (*info).state_mask, (*info).status_mask) }

unsafe fn conntrack_mt_check(par: *const xt_mtchk_param) -> i32 { let ret = nf_ct_netns_get((*par).net, (*par).family); if ret < 0 { pr_info_ratelimited!("cannot load conntrack support for proto=%u\n", (*par).family); } ret }
unsafe fn conntrack_mt_destroy(par: *const xt_mtdtor_param) { nf_ct_netns_put((*par).net, (*par).family); }

// Registration table corresponding to the three C xt_match entries.
static mut conntrack_mt_reg: [xt_match; 3] = [
    xt_match { name: "conntrack", revision: 1, family: NFPROTO_UNSPEC, matchsize: core::mem::size_of::<xt_conntrack_mtinfo1>(), match: Some(conntrack_mt_v1), checkentry: Some(conntrack_mt_check), destroy: Some(conntrack_mt_destroy), me: THIS_MODULE },
    xt_match { name: "conntrack", revision: 2, family: NFPROTO_UNSPEC, matchsize: core::mem::size_of::<xt_conntrack_mtinfo2>(), match: Some(conntrack_mt_v2), checkentry: Some(conntrack_mt_check), destroy: Some(conntrack_mt_destroy), me: THIS_MODULE },
    xt_match { name: "conntrack", revision: 3, family: NFPROTO_UNSPEC, matchsize: core::mem::size_of::<xt_conntrack_mtinfo3>(), match: Some(conntrack_mt_v3), checkentry: Some(conntrack_mt_check), destroy: Some(conntrack_mt_destroy), me: THIS_MODULE },
];

unsafe fn conntrack_mt_init() -> i32 { xt_register_matches(conntrack_mt_reg.as_mut_ptr(), conntrack_mt_reg.len()) }
unsafe fn conntrack_mt_exit() { xt_unregister_matches(conntrack_mt_reg.as_mut_ptr(), conntrack_mt_reg.len()); }

// module_init(conntrack_mt_init);
// module_exit(conntrack_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
