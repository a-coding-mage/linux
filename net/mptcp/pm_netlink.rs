// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP
 *
 * Copyright (c) 2020, Red Hat, Inc.
 */

// Translated from pm_netlink.c. Kernel-provided declarations and macros are
// intentionally referenced but not reimplemented here.

const MPTCP_PM_CMD_GRP_OFFSET: usize = 0;
const MPTCP_PM_EV_GRP_OFFSET: usize = 1;

static mut mptcp_pm_mcgrps: [genl_multicast_group; 2] = [
    genl_multicast_group { name: MPTCP_PM_CMD_GRP_NAME, flags: 0 },
    genl_multicast_group { name: MPTCP_PM_EV_GRP_NAME, flags: GENL_MCAST_CAP_NET_ADMIN },
];

unsafe fn mptcp_pm_family_to_addr(family: i32) -> i32 {
    #[cfg(CONFIG_MPTCP_IPV6)]
    if family == AF_INET6 { return MPTCP_PM_ADDR_ATTR_ADDR6; }
    MPTCP_PM_ADDR_ATTR_ADDR4
}

unsafe fn mptcp_pm_parse_pm_addr_attr(
    tb: *mut *mut nlattr, attr: *const nlattr, info: *mut genl_info,
    addr: *mut mptcp_addr_info, require_family: bool,
) -> i32 {
    if attr.is_null() { GENL_SET_ERR_MSG(info, "missing address info"); return -EINVAL; }
    let err = nla_parse_nested_deprecated(tb, MPTCP_PM_ADDR_ATTR_MAX, attr,
        mptcp_pm_address_nl_policy, (*info).extack);
    if err != 0 { return err; }
    if !(*tb.add(MPTCP_PM_ADDR_ATTR_ID as usize)).is_null() { (*addr).id = nla_get_u8(*tb.add(MPTCP_PM_ADDR_ATTR_ID as usize)); }
    if (*tb.add(MPTCP_PM_ADDR_ATTR_FAMILY as usize)).is_null() {
        if !require_family { return 0; }
        NL_SET_ERR_MSG_ATTR((*info).extack, attr, "missing family"); return -EINVAL;
    }
    (*addr).family = nla_get_u16(*tb.add(MPTCP_PM_ADDR_ATTR_FAMILY as usize));
    if (*addr).family != AF_INET
        && { #[cfg(CONFIG_MPTCP_IPV6)] { (*addr).family != AF_INET6 } #[cfg(not(CONFIG_MPTCP_IPV6))] { true } } {
        NL_SET_ERR_MSG_ATTR((*info).extack, attr, "unknown address family"); return -EINVAL;
    }
    let addr_addr = mptcp_pm_family_to_addr((*addr).family as i32) as usize;
    if (*tb.add(addr_addr)).is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, attr, "missing address data"); return -EINVAL; }
    #[cfg(CONFIG_MPTCP_IPV6)]
    if (*addr).family == AF_INET6 { (*addr).addr6 = nla_get_in6_addr(*tb.add(addr_addr)); }
    else { (*addr).addr.s_addr = nla_get_in_addr(*tb.add(addr_addr)); }
    #[cfg(not(CONFIG_MPTCP_IPV6))]
    { (*addr).addr.s_addr = nla_get_in_addr(*tb.add(addr_addr)); }
    if !(*tb.add(MPTCP_PM_ADDR_ATTR_PORT as usize)).is_null() { (*addr).port = htons(nla_get_u16(*tb.add(MPTCP_PM_ADDR_ATTR_PORT as usize))); }
    0
}

pub unsafe fn mptcp_pm_parse_addr(attr: *mut nlattr, info: *mut genl_info, addr: *mut mptcp_addr_info) -> i32 {
    let mut tb: [*mut nlattr; (MPTCP_PM_ADDR_ATTR_MAX + 1) as usize] = [core::ptr::null_mut(); (MPTCP_PM_ADDR_ATTR_MAX + 1) as usize];
    core::ptr::write_bytes(addr as *mut u8, 0, core::mem::size_of::<mptcp_addr_info>());
    mptcp_pm_parse_pm_addr_attr(tb.as_mut_ptr(), attr, info, addr, true)
}

pub unsafe fn mptcp_pm_parse_entry(attr: *mut nlattr, info: *mut genl_info, require_family: bool, entry: *mut mptcp_pm_addr_entry) -> i32 {
    let mut tb: [*mut nlattr; (MPTCP_PM_ADDR_ATTR_MAX + 1) as usize] = [core::ptr::null_mut(); (MPTCP_PM_ADDR_ATTR_MAX + 1) as usize];
    core::ptr::write_bytes(entry as *mut u8, 0, core::mem::size_of::<mptcp_pm_addr_entry>());
    let err = mptcp_pm_parse_pm_addr_attr(tb.as_mut_ptr(), attr, info, &mut (*entry).addr, require_family);
    if err != 0 { return err; }
    if !(*tb.add(MPTCP_PM_ADDR_ATTR_IF_IDX as usize)).is_null() { (*entry).ifindex = nla_get_s32(*tb.add(MPTCP_PM_ADDR_ATTR_IF_IDX as usize)); }
    if !(*tb.add(MPTCP_PM_ADDR_ATTR_FLAGS as usize)).is_null() { (*entry).flags = nla_get_u32(*tb.add(MPTCP_PM_ADDR_ATTR_FLAGS as usize)) & MPTCP_PM_ADDR_FLAGS_MASK; }
    if !(*tb.add(MPTCP_PM_ADDR_ATTR_PORT as usize)).is_null() { (*entry).addr.port = htons(nla_get_u16(*tb.add(MPTCP_PM_ADDR_ATTR_PORT as usize))); }
    0
}

unsafe fn mptcp_nl_fill_addr(skb: *mut sk_buff, entry: *mut mptcp_pm_addr_entry) -> i32 {
    let addr = &mut (*entry).addr;
    let attr = nla_nest_start(skb, MPTCP_PM_ATTR_ADDR);
    if attr.is_null() { return -EMSGSIZE; }
    if nla_put_u16(skb, MPTCP_PM_ADDR_ATTR_FAMILY, addr.family) != 0
        || nla_put_u16(skb, MPTCP_PM_ADDR_ATTR_PORT, ntohs(addr.port)) != 0
        || nla_put_u8(skb, MPTCP_PM_ADDR_ATTR_ID, addr.id) != 0
        || nla_put_u32(skb, MPTCP_PM_ADDR_ATTR_FLAGS, (*entry).flags) != 0
        || ((*entry).ifindex != 0 && nla_put_s32(skb, MPTCP_PM_ADDR_ATTR_IF_IDX, (*entry).ifindex) != 0) { nla_nest_cancel(skb, attr); return -EMSGSIZE; }
    if addr.family == AF_INET && nla_put_in_addr(skb, MPTCP_PM_ADDR_ATTR_ADDR4, addr.addr.s_addr) != 0 { nla_nest_cancel(skb, attr); return -EMSGSIZE; }
    #[cfg(CONFIG_MPTCP_IPV6)]
    if addr.family == AF_INET6 && nla_put_in6_addr(skb, MPTCP_PM_ADDR_ATTR_ADDR6, &addr.addr6) != 0 { nla_nest_cancel(skb, attr); return -EMSGSIZE; }
    nla_nest_end(skb, attr); 0
}

unsafe fn mptcp_pm_get_addr(id: u8, addr: *mut mptcp_pm_addr_entry, info: *mut genl_info) -> i32 {
    if !(*info).attrs[MPTCP_PM_ATTR_TOKEN as usize].is_null() { mptcp_userspace_pm_get_addr(id, addr, info) } else { mptcp_pm_nl_get_addr(id, addr, info) }
}

pub unsafe fn mptcp_pm_nl_get_addr_doit(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut addr: mptcp_pm_addr_entry = core::mem::zeroed();
    if GENL_REQ_ATTR_CHECK(info, MPTCP_PM_ENDPOINT_ADDR) { return -EINVAL; }
    let attr = (*info).attrs[MPTCP_PM_ENDPOINT_ADDR as usize];
    let mut ret = mptcp_pm_parse_entry(attr, info, false, &mut addr);
    if ret < 0 { return ret; }
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return -ENOMEM; }
    let reply = genlmsg_put_reply(msg, info, &mptcp_genl_family, 0, (*info).genlhdr.cmd);
    if reply.is_null() { GENL_SET_ERR_MSG(info, "not enough space in Netlink message"); nlmsg_free(msg); return -EMSGSIZE; }
    ret = mptcp_pm_get_addr(addr.addr.id, &mut addr, info);
    if ret != 0 { NL_SET_ERR_MSG_ATTR((*info).extack, attr, "address not found"); nlmsg_free(msg); return ret; }
    ret = mptcp_nl_fill_addr(msg, &mut addr); if ret != 0 { nlmsg_free(msg); return ret; }
    genlmsg_end(msg, reply); genlmsg_reply(msg, info)
}

pub unsafe fn mptcp_pm_genl_fill_addr(msg: *mut sk_buff, cb: *mut netlink_callback, entry: *mut mptcp_pm_addr_entry) -> i32 {
    let hdr = genlmsg_put(msg, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq, &mptcp_genl_family, NLM_F_MULTI, MPTCP_PM_CMD_GET_ADDR);
    if hdr.is_null() { return -EINVAL; }
    if mptcp_nl_fill_addr(msg, entry) < 0 { genlmsg_cancel(msg, hdr); return -EINVAL; }
    genlmsg_end(msg, hdr); 0
}

unsafe fn mptcp_pm_dump_addr(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let info = genl_info_dump(cb);
    if !(*info).attrs[MPTCP_PM_ATTR_TOKEN as usize].is_null() { mptcp_userspace_pm_dump_addr(msg, cb) } else { mptcp_pm_nl_dump_addr(msg, cb) }
}

pub unsafe fn mptcp_pm_nl_get_addr_dumpit(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32 { mptcp_pm_dump_addr(msg, cb) }

unsafe fn mptcp_pm_set_flags(info: *mut genl_info) -> i32 {
    let mut loc: mptcp_pm_addr_entry = core::mem::zeroed(); loc.addr.family = AF_UNSPEC;
    if GENL_REQ_ATTR_CHECK(info, MPTCP_PM_ATTR_ADDR) { return -EINVAL; }
    let attr = (*info).attrs[MPTCP_PM_ATTR_ADDR as usize];
    let ret = mptcp_pm_parse_entry(attr, info, false, &mut loc); if ret < 0 { return ret; }
    if !(*info).attrs[MPTCP_PM_ATTR_TOKEN as usize].is_null() { mptcp_userspace_pm_set_flags(&mut loc, info) } else { mptcp_pm_nl_set_flags(&mut loc, info) }
}

pub unsafe fn mptcp_pm_nl_set_flags_doit(_skb: *mut sk_buff, info: *mut genl_info) -> i32 { mptcp_pm_set_flags(info) }

unsafe fn mptcp_nl_mcast_send(net: *mut net, nlskb: *mut sk_buff, gfp: gfp_t) { genlmsg_multicast_netns(&mptcp_genl_family, net, nlskb, 0, MPTCP_PM_EV_GRP_OFFSET, gfp); }

unsafe fn mptcp_event_add_subflow(skb: *mut sk_buff, ssk: *const sock) -> i32 {
    let issk = inet_sk(ssk); let sf = mptcp_subflow_ctx(ssk);
    if nla_put_u16(skb, MPTCP_ATTR_FAMILY, (*ssk).sk_family) != 0 { return -EMSGSIZE; }
    if (*ssk).sk_family == AF_INET { if nla_put_in_addr(skb, MPTCP_ATTR_SADDR4, (*issk).inet_saddr) != 0 || nla_put_in_addr(skb, MPTCP_ATTR_DADDR4, (*issk).inet_daddr) != 0 { return -EMSGSIZE; } }
    #[cfg(CONFIG_MPTCP_IPV6)]
    else if (*ssk).sk_family == AF_INET6 { if nla_put_in6_addr(skb, MPTCP_ATTR_SADDR6, &(*issk).pinet6.saddr) != 0 || nla_put_in6_addr(skb, MPTCP_ATTR_DADDR6, &(*ssk).sk_v6_daddr) != 0 { return -EMSGSIZE; } }
    else { WARN_ON_ONCE(1); return -EMSGSIZE; }
    if nla_put_be16(skb, MPTCP_ATTR_SPORT, (*issk).inet_sport) != 0 || nla_put_be16(skb, MPTCP_ATTR_DPORT, (*issk).inet_dport) != 0 || sf.is_null() { return -EMSGSIZE; }
    if nla_put_u8(skb, MPTCP_ATTR_LOC_ID, subflow_get_local_id(sf)) != 0 || nla_put_u8(skb, MPTCP_ATTR_REM_ID, (*sf).remote_id) != 0 { return -EMSGSIZE; } 0
}

unsafe fn mptcp_event_put_token_and_ssk(skb: *mut sk_buff, msk: *const mptcp_sock, ssk: *const sock) -> i32 {
    if nla_put_u32(skb, MPTCP_ATTR_TOKEN, READ_ONCE((*msk).token)) != 0 || mptcp_event_add_subflow(skb, ssk) != 0 { return -EMSGSIZE; }
    let sf = mptcp_subflow_ctx(ssk); if sf.is_null() { return -EINVAL; }
    if nla_put_u8(skb, MPTCP_ATTR_BACKUP, (*sf).backup) != 0 { return -EMSGSIZE; }
    if (*ssk).sk_bound_dev_if != 0 && nla_put_s32(skb, MPTCP_ATTR_IF_IDX, (*ssk).sk_bound_dev_if) != 0 { return -EMSGSIZE; } 0
}

unsafe fn mptcp_event_sub_established(skb: *mut sk_buff, msk: *const mptcp_sock, ssk: *const sock) -> i32 { mptcp_event_put_token_and_ssk(skb, msk, ssk) }
unsafe fn mptcp_event_sub_closed(skb: *mut sk_buff, msk: *const mptcp_sock, ssk: *const sock) -> i32 { mptcp_event_put_token_and_ssk(skb, msk, ssk) }
unsafe fn mptcp_event_created(skb: *mut sk_buff, msk: *const mptcp_sock, ssk: *const sock) -> i32 { if nla_put_u32(skb, MPTCP_ATTR_TOKEN, READ_ONCE((*msk).token)) != 0 { return -EMSGSIZE; } mptcp_event_add_subflow(skb, ssk) }

pub unsafe fn mptcp_userspace_pm_active(msk: *const mptcp_sock) -> bool { genl_has_listeners(&mptcp_genl_family, sock_net(msk as *const sock), MPTCP_PM_EV_GRP_OFFSET) }

// The remaining event builders preserve the original kernel netlink operations.
pub unsafe fn mptcp_event_addr_removed(msk: *const mptcp_sock, id: u8) { let net = sock_net(msk as *const sock); if !genl_has_listeners(&mptcp_genl_family, net, MPTCP_PM_EV_GRP_OFFSET) { return; } let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC); if skb.is_null() { return; } let nlh = genlmsg_put(skb, 0, 0, &mptcp_genl_family, 0, MPTCP_EVENT_REMOVED); if nlh.is_null() || nla_put_u32(skb, MPTCP_ATTR_TOKEN, READ_ONCE((*msk).token)) != 0 || nla_put_u8(skb, MPTCP_ATTR_REM_ID, id) != 0 { nlmsg_free(skb); return; } genlmsg_end(skb, nlh); mptcp_nl_mcast_send(net, skb, GFP_ATOMIC); }

pub unsafe fn mptcp_event_addr_announced(ssk: *const sock, info: *const mptcp_addr_info) {
    let subflow = mptcp_subflow_ctx(ssk); let msk = mptcp_sk((*subflow).conn); let net = sock_net(ssk);
    if !genl_has_listeners(&mptcp_genl_family, net, MPTCP_PM_EV_GRP_OFFSET) { return; }
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC); if skb.is_null() { return; }
    let nlh = genlmsg_put(skb, 0, 0, &mptcp_genl_family, 0, MPTCP_EVENT_ANNOUNCED);
    if nlh.is_null() || nla_put_u32(skb, MPTCP_ATTR_TOKEN, READ_ONCE((*msk).token)) != 0 || nla_put_u8(skb, MPTCP_ATTR_REM_ID, (*info).id) != 0 || nla_put_be16(skb, MPTCP_ATTR_DPORT, if (*info).port == 0 { inet_sk(ssk).inet_dport } else { (*info).port }) != 0 { nlmsg_free(skb); return; }
    if (*info).family == AF_INET { if nla_put_in_addr(skb, MPTCP_ATTR_DADDR4, (*info).addr.s_addr) != 0 { nlmsg_free(skb); return; } }
    #[cfg(CONFIG_MPTCP_IPV6)]
    else if (*info).family == AF_INET6 { if nla_put_in6_addr(skb, MPTCP_ATTR_DADDR6, &(*info).addr6) != 0 { nlmsg_free(skb); return; } }
    else { WARN_ON_ONCE(1); nlmsg_free(skb); return; }
    genlmsg_end(skb, nlh); mptcp_nl_mcast_send(net, skb, GFP_ATOMIC);
}

pub unsafe fn mptcp_event_pm_listener(ssk: *const sock, event: mptcp_event_type) {
    let issk = inet_sk(ssk); let net = sock_net(ssk); if !genl_has_listeners(&mptcp_genl_family, net, MPTCP_PM_EV_GRP_OFFSET) { return; }
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if skb.is_null() { return; }
    let nlh = genlmsg_put(skb, 0, 0, &mptcp_genl_family, 0, event);
    if nlh.is_null() || nla_put_u16(skb, MPTCP_ATTR_FAMILY, (*ssk).sk_family) != 0 || nla_put_be16(skb, MPTCP_ATTR_SPORT, (*issk).inet_sport) != 0 || ((*ssk).sk_family == AF_INET && nla_put_in_addr(skb, MPTCP_ATTR_SADDR4, (*issk).inet_saddr) != 0) { nlmsg_free(skb); return; }
    #[cfg(CONFIG_MPTCP_IPV6)]
    if (*ssk).sk_family == AF_INET6 && nla_put_in6_addr(skb, MPTCP_ATTR_SADDR6, &(*issk).pinet6.saddr) != 0 { nlmsg_free(skb); return; }
    genlmsg_end(skb, nlh); mptcp_nl_mcast_send(net, skb, GFP_KERNEL);
}

pub unsafe fn mptcp_event(typ: mptcp_event_type, msk: *const mptcp_sock, ssk: *const sock, gfp: gfp_t) {
    let net = sock_net(msk as *const sock); if !genl_has_listeners(&mptcp_genl_family, net, MPTCP_PM_EV_GRP_OFFSET) { return; }
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, gfp); if skb.is_null() { return; }
    let nlh = genlmsg_put(skb, 0, 0, &mptcp_genl_family, 0, typ); if nlh.is_null() { nlmsg_free(skb); return; }
    match typ { MPTCP_EVENT_UNSPEC => { WARN_ON_ONCE(1); }, MPTCP_EVENT_CLOSED => { if nla_put_u32(skb, MPTCP_ATTR_TOKEN, READ_ONCE((*msk).token)) != 0 { nlmsg_free(skb); return; } }, MPTCP_EVENT_ANNOUNCED | MPTCP_EVENT_REMOVED => { WARN_ON_ONCE(1); }, _ => {} }
    genlmsg_end(skb, nlh); mptcp_nl_mcast_send(net, skb, gfp);
}

pub static mut mptcp_genl_family: genl_family = genl_family {
    name: MPTCP_PM_NAME, version: MPTCP_PM_VER, netnsok: true, module: THIS_MODULE,
    ops: mptcp_pm_nl_ops, n_ops: ARRAY_SIZE(mptcp_pm_nl_ops),
    resv_start_op: MPTCP_PM_CMD_SUBFLOW_DESTROY + 1,
    mcgrps: mptcp_pm_mcgrps.as_ptr(), n_mcgrps: ARRAY_SIZE(mptcp_pm_mcgrps),
};

pub unsafe fn mptcp_pm_nl_init() { if genl_register_family(&mut mptcp_genl_family) != 0 { panic!("Failed to register MPTCP PM netlink family\n"); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
