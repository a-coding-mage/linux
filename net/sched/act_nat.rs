// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Stateless NAT actions
 *
 * Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C headers and build-time kernel dependencies are supplied externally.

static mut act_nat_ops: tc_action_ops = tc_action_ops {
    kind: "nat",
    id: TCA_ID_NAT,
    owner: THIS_MODULE,
    act: Some(tcf_nat_act),
    dump: Some(tcf_nat_dump),
    init: Some(tcf_nat_init),
    cleanup: Some(tcf_nat_cleanup),
    size: core::mem::size_of::<tcf_nat>(),
    ..unsafe { core::mem::zeroed() }
};

static nat_policy: [nla_policy; TCA_NAT_MAX as usize + 1] = {
    let mut p = [nla_policy { len: 0, ..unsafe { core::mem::zeroed() } }; TCA_NAT_MAX as usize + 1];
    p[TCA_NAT_PARMS as usize].len = core::mem::size_of::<tc_nat>();
    p
};

unsafe fn tcf_nat_init(
    net: *mut net,
    nla: *mut nlattr,
    est: *mut nlattr,
    a: *mut *mut tc_action,
    tp: *mut tcf_proto,
    flags: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let tn = net_generic(net, act_nat_ops.net_id);
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut nparm: *mut tcf_nat_parms;
    let mut oparm: *mut tcf_nat_parms;
    let mut tb: [*mut nlattr; TCA_NAT_MAX as usize + 1] = [core::ptr::null_mut(); TCA_NAT_MAX as usize + 1];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_nat;
    let mut ret: i32 = 0;
    let mut err: i32;
    let p: *mut tcf_nat;
    let mut index: u32;

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_NAT_MAX, nla, nat_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_NAT_PARMS as usize].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_NAT_PARMS as usize]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err == 0 {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &raw mut act_nat_ops, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if err > 0 {
        if bind { return ACT_P_BOUND; }
        if flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    } else { return err; }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); return err; }
    nparm = kzalloc_obj::<tcf_nat_parms>();
    if nparm.is_null() { tcf_idr_release(*a, bind); return -ENOMEM; }
    (*nparm).old_addr = (*parm).old_addr;
    (*nparm).new_addr = (*parm).new_addr;
    (*nparm).mask = (*parm).mask;
    (*nparm).flags = (*parm).flags;
    (*nparm).action = (*parm).action;
    p = to_tcf_nat(*a);
    spin_lock_bh(&mut (*p).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    oparm = rcu_replace_pointer(&mut (*p).parms, nparm, lockdep_is_held(&(*p).tcf_lock));
    spin_unlock_bh(&mut (*p).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if !oparm.is_null() { kfree_rcu(oparm, rcu); }
    ret
}

unsafe fn tcf_nat_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> i32 {
    let p = to_tcf_nat(a);
    let parms = rcu_dereference_bh((*p).parms);
    tcf_lastuse_update(&mut (*p).tcf_tm);
    tcf_action_update_bstats(&mut (*p).common, skb);
    let action = (*parms).action;
    if unlikely(action == TC_ACT_SHOT) { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
    let old_addr = (*parms).old_addr;
    let new_addr = (*parms).new_addr;
    let mask = (*parms).mask;
    let egress = (*parms).flags & TCA_NAT_FLAG_EGRESS != 0;
    let noff = skb_network_offset(skb);
    if !pskb_may_pull(skb, core::mem::size_of::<iphdr>() + noff) { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
    let mut iph = ip_hdr(skb);
    let mut addr = if egress { (*iph).saddr } else { (*iph).daddr };
    let mut rewritten = false;
    if ((old_addr ^ addr) & mask) == 0 {
        if skb_try_make_writable(skb, core::mem::size_of::<iphdr>() + noff) != 0 { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
        let new_addr = (new_addr & mask) | (addr & !mask);
        iph = ip_hdr(skb);
        if egress { (*iph).saddr = new_addr; } else { (*iph).daddr = new_addr; }
        csum_replace4(&mut (*iph).check, addr, new_addr);
        addr = new_addr;
        rewritten = true;
    } else if ((*iph).frag_off & htons(IP_OFFSET)) != 0 || (*iph).protocol != IPPROTO_ICMP { return action; }
    let ihl = (*iph).ihl as usize * 4;
    let proto = if (*iph).frag_off & htons(IP_OFFSET) != 0 { 0 } else { (*iph).protocol };
    match proto {
        IPPROTO_TCP => {
            if !pskb_may_pull(skb, ihl + core::mem::size_of::<tcphdr>() + noff) || skb_try_make_writable(skb, ihl + core::mem::size_of::<tcphdr>() + noff) != 0 { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
            let tcph = (skb_network_header(skb).add(ihl)) as *mut tcphdr;
            inet_proto_csum_replace4(&mut (*tcph).check, skb, addr, if rewritten { (*parms).new_addr } else { new_addr }, true);
        }
        IPPROTO_UDP => {
            if !pskb_may_pull(skb, ihl + core::mem::size_of::<udphdr>() + noff) || skb_try_make_writable(skb, ihl + core::mem::size_of::<udphdr>() + noff) != 0 { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
            let udph = (skb_network_header(skb).add(ihl)) as *mut udphdr;
            if (*udph).check != 0 || (*skb).ip_summed == CHECKSUM_PARTIAL { inet_proto_csum_replace4(&mut (*udph).check, skb, addr, new_addr, true); if (*udph).check == 0 { (*udph).check = CSUM_MANGLED_0; } }
        }
        IPPROTO_ICMP => {
            if !pskb_may_pull(skb, ihl + core::mem::size_of::<icmphdr>() + noff) { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
            let mut icmph = (skb_network_header(skb).add(ihl)) as *mut icmphdr;
            if !icmp_is_err((*icmph).type_) { return action; }
            if !pskb_may_pull(skb, ihl + core::mem::size_of::<icmphdr>() + core::mem::size_of::<iphdr>() + noff) { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
            icmph = (skb_network_header(skb).add(ihl)) as *mut icmphdr;
            iph = icmph.add(1) as *mut iphdr;
            addr = if egress { (*iph).daddr } else { (*iph).saddr };
            if (old_addr ^ addr) & mask != 0 { return action; }
            if skb_try_make_writable(skb, ihl + core::mem::size_of::<icmphdr>() + core::mem::size_of::<iphdr>() + noff) != 0 { tcf_action_inc_drop_qstats(&mut (*p).common); return TC_ACT_SHOT; }
            icmph = (skb_network_header(skb).add(ihl)) as *mut icmphdr;
            iph = icmph.add(1) as *mut iphdr;
            let inner_new_addr = (new_addr & mask) | (addr & !mask);
            if egress { (*iph).daddr = inner_new_addr; } else { (*iph).saddr = inner_new_addr; }
            inet_proto_csum_replace4(&mut (*icmph).checksum, skb, addr, inner_new_addr, false);
        }
        _ => {}
    }
    action
}

unsafe fn tcf_nat_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, reference: i32) -> i32 {
    let b = skb_tail_pointer(skb);
    let p = to_tcf_nat(a);
    let parms = rcu_dereference((*p).parms);
    let mut opt: tc_nat = core::mem::zeroed();
    opt.index = (*p).tcf_index;
    opt.refcnt = refcount_read(&(*p).tcf_refcnt) - reference;
    opt.bindcnt = atomic_read(&(*p).tcf_bindcnt) - bind;
    opt.action = (*parms).action; opt.old_addr = (*parms).old_addr; opt.new_addr = (*parms).new_addr;
    opt.mask = (*parms).mask; opt.flags = (*parms).flags;
    if nla_put(skb, TCA_NAT_PARMS, core::mem::size_of::<tc_nat>(), &opt) != 0 { nlmsg_trim(skb, b); return -1; }
    let mut t: tcf_t = core::mem::zeroed();
    tcf_tm_dump(&mut t, &(*p).tcf_tm);
    if nla_put_64bit(skb, TCA_NAT_TM, core::mem::size_of::<tcf_t>(), &t, TCA_NAT_PAD) != 0 { nlmsg_trim(skb, b); return -1; }
    (*skb).len as i32
}
unsafe fn tcf_nat_cleanup(a: *mut tc_action) { let p = to_tcf_nat(a); let parms = rcu_dereference_protected((*p).parms, 1); if !parms.is_null() { kfree_rcu(parms, rcu); } }

unsafe fn nat_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, act_nat_ops.net_id), &raw mut act_nat_ops) }
unsafe fn nat_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_nat_ops.net_id); }

unsafe fn nat_init_module() -> i32 { tcf_register_action(&raw mut act_nat_ops, &raw mut nat_net_ops) }
unsafe fn nat_cleanup_module() { tcf_unregister_action(&raw mut act_nat_ops, &raw mut nat_net_ops); }

static mut nat_net_ops: pernet_operations = pernet_operations {
    init: Some(nat_init_net), exit_batch: Some(nat_exit_net), id: core::ptr::null_mut(),
    size: core::mem::size_of::<tc_action_net>(), ..unsafe { core::mem::zeroed() }
};

// MODULE_ALIAS_NET_ACT("nat");
// MODULE_DESCRIPTION("Stateless NAT actions");
// MODULE_LICENSE("GPL");
// module_init(nat_init_module);
// module_exit(nat_cleanup_module);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
