// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021 Red Hat GmbH
 *
 * Author: Florian Westphal <fw@strlen.de>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static NFNL_HOOK_NLA_POLICY: [struct_nla_policy; NFNLA_HOOK_MAX + 1] = [
    [NFNLA_HOOK_HOOKNUM] = NLA_POLICY_MAX(NLA_BE32, 255),
    [NFNLA_HOOK_PRIORITY] = struct_nla_policy { type_: NLA_U32 },
    [NFNLA_HOOK_DEV] = struct_nla_policy { type_: NLA_STRING, len: IFNAMSIZ - 1 },
    [NFNLA_HOOK_FUNCTION_NAME] = struct_nla_policy { type_: NLA_NUL_STRING, len: KSYM_NAME_LEN },
    [NFNLA_HOOK_MODULE_NAME] = struct_nla_policy { type_: NLA_NUL_STRING, len: MODULE_NAME_LEN },
    [NFNLA_HOOK_CHAIN_INFO] = struct_nla_policy { type_: NLA_NESTED },
];

unsafe fn nf_netlink_dump_start_rcu(nlsk: *mut sock, skb: *mut sk_buff,
                                    nlh: *const nlmsghdr,
                                    c: *mut netlink_dump_control) -> c_int {
    let mut err: c_int;
    if !try_module_get(THIS_MODULE) { return -EINVAL; }
    rcu_read_unlock();
    err = netlink_dump_start(nlsk, skb, nlh, c);
    rcu_read_lock();
    module_put(THIS_MODULE);
    err
}

#[repr(C)]
struct nfnl_dump_hook_data {
    devname: [c_char; IFNAMSIZ],
    headv: c_ulong,
    hook: u8,
}

unsafe fn nfnl_start_info_type(nlskb: *mut sk_buff, t: nfnl_hook_chaintype) -> *mut nlattr {
    let nest = nla_nest_start(nlskb, NFNLA_HOOK_CHAIN_INFO);
    if nest.is_null() { return core::ptr::null_mut(); }
    if nla_put_be32(nlskb, NFNLA_HOOK_INFO_TYPE, htonl(t)) == 0 { return nest; }
    nla_nest_cancel(nlskb, nest);
    core::ptr::null_mut()
}

unsafe fn nfnl_hook_put_bpf_prog_info(nlskb: *mut sk_buff, _ctx: *const nfnl_dump_hook_data,
                                      _seq: c_uint, prog: *const bpf_prog) -> c_int {
    if !IS_ENABLED(CONFIG_NETFILTER_BPF_LINK) { return 0; }
    if WARN_ON_ONCE(prog.is_null()) { return 0; }
    let nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_BPF);
    if nest.is_null() { return -EMSGSIZE; }
    let nest2 = nla_nest_start(nlskb, NFNLA_HOOK_INFO_DESC);
    if nest2.is_null() { nla_nest_cancel(nlskb, nest); return -EMSGSIZE; }
    if nla_put_be32(nlskb, NFNLA_HOOK_BPF_ID, htonl((*(*prog).aux).id)) != 0 {
        nla_nest_cancel(nlskb, nest); return -EMSGSIZE;
    }
    nla_nest_end(nlskb, nest2); nla_nest_end(nlskb, nest); 0
}

unsafe fn nfnl_hook_put_nft_info_desc(nlskb: *mut sk_buff, tname: *const c_char,
                                      name: *const c_char, family: u8) -> c_int {
    let nest = nla_nest_start(nlskb, NFNLA_HOOK_INFO_DESC);
    if nest.is_null() || nla_put_string(nlskb, NFNLA_CHAIN_TABLE, tname) != 0 ||
       nla_put_string(nlskb, NFNLA_CHAIN_NAME, name) != 0 ||
       nla_put_u8(nlskb, NFNLA_CHAIN_FAMILY, family) != 0 {
        nla_nest_cancel(nlskb, nest); return -EMSGSIZE;
    }
    nla_nest_end(nlskb, nest); 0
}

unsafe fn nfnl_hook_put_nft_chain_info(nlskb: *mut sk_buff, _ctx: *const nfnl_dump_hook_data,
                                       _seq: c_uint, chain: *mut nft_chain) -> c_int {
    if WARN_ON_ONCE(chain.is_null()) || !nft_is_active(sock_net((*nlskb).sk), chain) { return 0; }
    let nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_NFTABLES);
    if nest.is_null() { return -EMSGSIZE; }
    let ret = nfnl_hook_put_nft_info_desc(nlskb, (*(*chain).table).name, (*chain).name,
                                          (*(*chain).table).family);
    if ret != 0 { nla_nest_cancel(nlskb, nest); return ret; }
    nla_nest_end(nlskb, nest); 0
}

unsafe fn nfnl_hook_put_nft_ft_info(nlskb: *mut sk_buff, _ctx: *const nfnl_dump_hook_data,
                                    _seq: c_uint, nf_ft: *mut nf_flowtable) -> c_int {
    if WARN_ON_ONCE(nf_ft.is_null()) { return 0; }
    let ft = container_of!(nf_ft, nft_flowtable, data);
    if !nft_is_active(sock_net((*nlskb).sk), ft) { return 0; }
    let nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_NFT_FLOWTABLE);
    if nest.is_null() { return -EMSGSIZE; }
    let ret = nfnl_hook_put_nft_info_desc(nlskb, (*(*ft).table).name, (*ft).name,
                                          (*(*ft).table).family);
    if ret != 0 { nla_nest_cancel(nlskb, nest); return ret; }
    nla_nest_end(nlskb, nest); 0
}

unsafe fn nfnl_hook_entries_head(pf: u8, hook: c_uint, net: *mut net,
                                 dev: *const c_char) -> *const nf_hook_entries {
    let mut hook_head: *const nf_hook_entries = core::ptr::null();
    match pf {
        NFPROTO_IPV4 => { if hook >= ARRAY_SIZE((*net).nf.hooks_ipv4) { return ERR_PTR(-EINVAL); }
            hook_head = rcu_dereference((*net).nf.hooks_ipv4[hook as usize]); }
        NFPROTO_IPV6 => { if hook >= ARRAY_SIZE((*net).nf.hooks_ipv6) { return ERR_PTR(-EINVAL); }
            hook_head = rcu_dereference((*net).nf.hooks_ipv6[hook as usize]); }
        NFPROTO_ARP => { /* CONFIG_NETFILTER_FAMILY_ARP */
            if hook >= ARRAY_SIZE((*net).nf.hooks_arp) { return ERR_PTR(-EINVAL); }
            hook_head = rcu_dereference((*net).nf.hooks_arp[hook as usize]); }
        NFPROTO_BRIDGE => { /* CONFIG_NETFILTER_FAMILY_BRIDGE */
            if hook >= ARRAY_SIZE((*net).nf.hooks_bridge) { return ERR_PTR(-EINVAL); }
            hook_head = rcu_dereference((*net).nf.hooks_bridge[hook as usize]); }
        NFPROTO_NETDEV => { /* CONFIG_NETFILTER_INGRESS || CONFIG_NETFILTER_EGRESS */
            if hook >= NF_NETDEV_NUMHOOKS { return ERR_PTR(-EOPNOTSUPP); }
            if dev.is_null() { return ERR_PTR(-ENODEV); }
            let netdev = dev_get_by_name_rcu(net, dev);
            if netdev.is_null() { return ERR_PTR(-ENODEV); }
            if hook == NF_NETDEV_INGRESS { return rcu_dereference((*netdev).nf_hooks_ingress); }
            if hook == NF_NETDEV_EGRESS { return rcu_dereference((*netdev).nf_hooks_egress); }
        }
        _ => return ERR_PTR(-EPROTONOSUPPORT),
    }
    hook_head
}

unsafe fn nfnl_hook_dump_nat(nlskb: *mut sk_buff, ctx: *const nfnl_dump_hook_data,
                             ops: *const nf_hook_ops, family: c_int, seq: c_uint) -> c_int {
    let priv_ = (*ops).priv_ as *mut nf_nat_lookup_hook_priv;
    let e = rcu_dereference((*priv_).entries);
    if e.is_null() { return 0; }
    let nat_ops = nf_hook_entries_get_hook_ops(e);
    for i in 0..(*e).num_hook_entries {
        let err = nfnl_hook_dump_one(nlskb, ctx, *nat_ops.add(i), (*ops).priority, family, seq);
        if err != 0 { return err; }
    }
    0
}

unsafe fn nfnl_hook_dump(nlskb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let nfmsg = nlmsg_data((*cb).nlh) as *const nfgenmsg;
    let ctx = (*cb).data as *mut nfnl_dump_hook_data;
    let family = (*nfmsg).nfgen_family;
    let net = sock_net((*nlskb).sk);
    let i = (*cb).args[0];
    rcu_read_lock();
    let e = nfnl_hook_entries_head(family, (*ctx).hook, net, (*ctx).devname.as_ptr());
    if e.is_null() { rcu_read_unlock(); (*cb).args[0] = i; return (*nlskb).len; }
    if IS_ERR(e) || e as c_ulong != (*ctx).headv || i >= (*e).num_hook_entries { (*cb).seq += 1; }
    if !IS_ERR(e) {
        let ops = nf_hook_entries_get_hook_ops(e);
        let mut j = i;
        while j < (*e).num_hook_entries {
            let op = *ops.add(j);
            let err = if (*op).hook_ops_type == NF_HOOK_OP_NAT {
                nfnl_hook_dump_nat(nlskb, ctx, op, family, (*(*cb).nlh).nlmsg_seq)
            } else { nfnl_hook_dump_one(nlskb, ctx, op, (*op).priority, family, (*(*cb).nlh).nlmsg_seq) };
            if err != 0 { break; }
            j += 1;
        }
        (*cb).args[0] = j;
    }
    nl_dump_check_consistent(cb, nlmsg_hdr(nlskb));
    rcu_read_unlock(); (*nlskb).len
}

unsafe fn nfnl_hook_dump_start(cb: *mut netlink_callback) -> c_int {
    let nfmsg = nlmsg_data((*cb).nlh) as *const nfgenmsg;
    let nla = (*cb).data as *const *const nlattr;
    let net = sock_net((*(*cb).skb).sk); let family = (*nfmsg).nfgen_family;
    let hooknum = ntohl(nla_get_be32(*nla.add(NFNLA_HOOK_HOOKNUM as usize)));
    if hooknum > 255 { return -EINVAL; }
    let mut name = [0 as c_char; IFNAMSIZ];
    if family == NFPROTO_NETDEV { if (*nla.add(NFNLA_HOOK_DEV as usize)).is_null() { return -EINVAL; }
        nla_strscpy(name.as_mut_ptr(), *nla.add(NFNLA_HOOK_DEV as usize), name.len()); }
    rcu_read_lock(); let head = nfnl_hook_entries_head(family, hooknum, net, name.as_ptr()); rcu_read_unlock();
    if IS_ERR(head) { return PTR_ERR(head); }
    let ctx = kzalloc_obj::<nfnl_dump_hook_data>(); if ctx.is_null() { return -ENOMEM; }
    strscpy((*ctx).devname.as_mut_ptr(), name.as_ptr(), IFNAMSIZ); (*ctx).headv = head as c_ulong; (*ctx).hook = hooknum as u8;
    (*cb).seq = 1; (*cb).data = ctx as *mut c_void; 0
}

unsafe fn nfnl_hook_dump_stop(cb: *mut netlink_callback) -> c_int { kfree((*cb).data); 0 }

unsafe fn nfnl_hook_get(skb: *mut sk_buff, info: *const nfnl_info,
                        nla: *const *const nlattr) -> c_int {
    if (*nla.add(NFNLA_HOOK_HOOKNUM as usize)).is_null() { return -EINVAL; }
    if (*(*info).nlh).nlmsg_flags & NLM_F_DUMP != 0 {
        let c = netlink_dump_control { start: nfnl_hook_dump_start, done: nfnl_hook_dump_stop,
            dump: nfnl_hook_dump, module: THIS_MODULE, data: nla as *mut c_void };
        return nf_netlink_dump_start_rcu((*info).sk, skb, (*info).nlh, &c);
    }
    -EOPNOTSUPP
}

static nfnl_hook_cb: [nfnl_callback; NFNL_MSG_HOOK_MAX] = [nfnl_callback {
    call: nfnl_hook_get, type_: NFNL_CB_RCU, attr_count: NFNLA_HOOK_MAX,
    policy: NFNL_HOOK_NLA_POLICY.as_ptr(),
}];

unsafe fn nfnl_hook_dump_one(nlskb: *mut sk_buff, ctx: *const nfnl_dump_hook_data,
                             ops: *const nf_hook_ops, priority: c_int, family: c_int,
                             seq: c_uint) -> c_int {
    let event = nfnl_msg_type(NFNL_SUBSYS_HOOK, NFNL_MSG_HOOK_GET);
    let portid = NETLINK_CB(nlskb).portid;
    let nlh = nfnl_msg_put(nlskb, portid, seq, event, NLM_F_MULTI, family, NFNETLINK_V0, 0);
    if nlh.is_null() { return -EMSGSIZE; }
    #[cfg(CONFIG_KALLSYMS)] {
        let mut sym = [0 as c_char; KSYM_SYMBOL_LEN];
        let ret = snprintf(sym.as_mut_ptr(), sym.len(), c"%ps".as_ptr(), (*ops).hook);
        if ret >= sym.len() as c_int { nlmsg_trim(nlskb, nlh); return -EINVAL; }
        if nla_put_string(nlskb, NFNLA_HOOK_FUNCTION_NAME, sym.as_ptr()) != 0 {
            nlmsg_trim(nlskb, nlh); return -EMSGSIZE;
        }
    }
    let hooknum = if (*ops).pf == NFPROTO_INET && (*ops).hooknum == NF_INET_INGRESS {
        NF_NETDEV_INGRESS
    } else { (*ops).hooknum };
    let mut ret = nla_put_be32(nlskb, NFNLA_HOOK_HOOKNUM, htonl(hooknum));
    if ret == 0 { ret = nla_put_be32(nlskb, NFNLA_HOOK_PRIORITY, htonl(priority)); }
    if ret == 0 { ret = match (*ops).hook_ops_type {
        NF_HOOK_OP_NF_TABLES => nfnl_hook_put_nft_chain_info(nlskb, ctx, seq, (*ops).priv_ as *mut nft_chain),
        NF_HOOK_OP_BPF => nfnl_hook_put_bpf_prog_info(nlskb, ctx, seq, (*ops).priv_ as *const bpf_prog),
        NF_HOOK_OP_NFT_FT => nfnl_hook_put_nft_ft_info(nlskb, ctx, seq, (*ops).priv_ as *mut nf_flowtable),
        NF_HOOK_OP_UNDEFINED => 0,
        _ => { WARN_ON_ONCE(true); 0 }
    }; }
    if ret != 0 { nlmsg_trim(nlskb, nlh); return ret; }
    nlmsg_end(nlskb, nlh); 0
}

// Remaining declarations and module registration are kept as external kernel-facing items.
unsafe extern "C" {
    fn nfnetlink_subsys_register(s: *const nfnetlink_subsystem) -> c_int;
    fn nfnetlink_subsys_unregister(s: *const nfnetlink_subsystem);
}

unsafe fn nfnetlink_hook_init() -> c_int { nfnetlink_subsys_register(&nfhook_subsys) }
unsafe fn nfnetlink_hook_exit() { nfnetlink_subsys_unregister(&nfhook_subsys); }

static nfhook_subsys: nfnetlink_subsystem = nfnetlink_subsystem {
    name: c"nfhook".as_ptr(), subsys_id: NFNL_SUBSYS_HOOK,
    cb_count: NFNL_MSG_HOOK_MAX, cb: nfnl_hook_cb,
};

// MODULE_ALIAS_NFNL_SUBSYS(NFNL_SUBSYS_HOOK)
// module_init(nfnetlink_hook_init); module_exit(nfnetlink_hook_exit)
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Florian Westphal <fw@strlen.de>")
// MODULE_DESCRIPTION("nfnetlink_hook: list registered netfilter hooks")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
