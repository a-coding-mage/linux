// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_skbmod.c  skb data modifier
 *
 * Copyright (c) 2016 Jamal Hadi Salim <jhs@mojatatu.com>
 */

// Linux kernel headers and symbols are supplied by the surrounding translation.

static mut ACT_SKBMOD_OPS: tc_action_ops = tc_action_ops { };

unsafe fn tcf_skbmod_act(
    skb: *mut sk_buff,
    a: *const tc_action,
    res: *mut tcf_result,
) -> i32 {
    let d: *mut tcf_skbmod = to_skbmod(a);
    let mut p: *mut tcf_skbmod_params;
    let mut max_edit_len: i32;
    let mut err: i32;
    let flags: u64;

    tcf_lastuse_update(&mut (*d).tcf_tm);
    bstats_update(this_cpu_ptr((*d).common.cpu_bstats), skb);

    p = rcu_dereference_bh((*d).skbmod_p);
    if unlikely((*p).action == TC_ACT_SHOT) {
        goto_drop!(drop);
    }

    flags = (*p).flags;

    /* tcf_skbmod_init() guarantees "flags" to be one of the following:
     * 1. a combination of SKBMOD_F_{DMAC,SMAC,ETYPE}
     * 2. SKBMOD_F_SWAPMAC
     * 3. SKBMOD_F_ECN
     * SKBMOD_F_ECN only works with IP packets; all other flags only work with Ethernet
     * packets.
     */
    if flags == SKBMOD_F_ECN as u64 {
        match skb_protocol(skb, true) {
            x if x == cpu_to_be16(ETH_P_IP) => {
                max_edit_len = core::mem::size_of::<iphdr>() as i32;
            }
            x if x == cpu_to_be16(ETH_P_IPV6) => {
                max_edit_len = core::mem::size_of::<ipv6hdr>() as i32;
            }
            _ => goto_out!(out),
        }
        max_edit_len += skb_network_offset(skb);
    } else {
        if (*skb).dev.is_null() || (*(*skb).dev).type_ != ARPHRD_ETHER {
            goto_out!(out);
        }
        max_edit_len = ETH_HLEN as i32;
    }

    err = skb_ensure_writable(skb, max_edit_len);
    if unlikely(err != 0) {
        goto_drop!(drop);
    }

    if flags & SKBMOD_F_DMAC as u64 != 0 {
        ether_addr_copy((*eth_hdr(skb)).h_dest.as_mut_ptr(), (*p).eth_dst.as_ptr());
    }
    if flags & SKBMOD_F_SMAC as u64 != 0 {
        ether_addr_copy((*eth_hdr(skb)).h_source.as_mut_ptr(), (*p).eth_src.as_ptr());
    }
    if flags & SKBMOD_F_ETYPE as u64 != 0 {
        (*eth_hdr(skb)).h_proto = (*p).eth_type;
    }

    if flags & SKBMOD_F_SWAPMAC as u64 != 0 {
        let mut tmpaddr: [u16; ETH_ALEN / 2] = [0; ETH_ALEN / 2];
        /*XXX: I am sure we can come up with more efficient swapping*/
        ether_addr_copy(tmpaddr.as_mut_ptr() as *mut u8, (*eth_hdr(skb)).h_dest.as_ptr());
        ether_addr_copy((*eth_hdr(skb)).h_dest.as_mut_ptr(), (*eth_hdr(skb)).h_source.as_ptr());
        ether_addr_copy((*eth_hdr(skb)).h_source.as_mut_ptr(), tmpaddr.as_ptr() as *const u8);
    }

    if flags & SKBMOD_F_ECN as u64 != 0 {
        INET_ECN_set_ce(skb);
    }

out:
    return (*p).action;

drop:
    qstats_cpu_overlimit_inc((*d).common.cpu_qstats);
    return TC_ACT_SHOT;
}

static skbmod_policy: [nla_policy; TCA_SKBMOD_MAX + 1] = [nla_policy { len: core::mem::size_of::<tc_skbmod>() as u16 }, nla_policy { len: ETH_ALEN as u16 }, nla_policy { len: ETH_ALEN as u16 }, nla_policy { type_: NLA_U16 }];

unsafe fn tcf_skbmod_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
                          a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32,
                          extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic(net, ACT_SKBMOD_OPS.net_id);
    let ovr = flags & TCA_ACT_FLAGS_REPLACE != 0;
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut tb: [*mut nlattr; TCA_SKBMOD_MAX + 1] = [core::ptr::null_mut(); TCA_SKBMOD_MAX + 1];
    let mut lflags: u32 = 0;
    let mut index: u32;
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut daddr: *mut u8 = core::ptr::null_mut();
    let mut saddr: *mut u8 = core::ptr::null_mut();
    let mut eth_type: u16 = 0;
    let mut ret: i32 = 0;
    let mut err: i32;

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_SKBMOD_MAX, nla, skbmod_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_SKBMOD_PARMS].is_null() { return -EINVAL; }
    if !tb[TCA_SKBMOD_DMAC].is_null() { daddr = nla_data(tb[TCA_SKBMOD_DMAC]); lflags |= SKBMOD_F_DMAC; }
    if !tb[TCA_SKBMOD_SMAC].is_null() { saddr = nla_data(tb[TCA_SKBMOD_SMAC]); lflags |= SKBMOD_F_SMAC; }
    if !tb[TCA_SKBMOD_ETYPE].is_null() { eth_type = nla_get_u16(tb[TCA_SKBMOD_ETYPE]); lflags |= SKBMOD_F_ETYPE; }

    let parm: *mut tc_skbmod = nla_data(tb[TCA_SKBMOD_PARMS]);
    index = (*parm).index;
    if (*parm).flags & SKBMOD_F_SWAPMAC != 0 { lflags = SKBMOD_F_SWAPMAC; }
    if (*parm).flags & SKBMOD_F_ECN != 0 { lflags = SKBMOD_F_ECN; }
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err < 0 { return err; }
    let exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }
    if lflags == 0 { if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); } return -EINVAL; }
    if !exists { ret = tcf_idr_create(tn, index, est, a, &ACT_SKBMOD_OPS, bind, true, flags); if ret != 0 { tcf_idr_cleanup(tn, index); return ret; } ret = ACT_P_CREATED; } else if !ovr { tcf_idr_release(*a, bind); return -EEXIST; }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack); if err < 0 { goto release_idr; }
    let d: *mut tcf_skbmod = to_skbmod(*a);
    let p: *mut tcf_skbmod_params = kzalloc_obj(); if p.is_null() { err = -ENOMEM; goto put_chain; }
    (*p).flags = lflags; (*p).action = (*parm).action;
    if ovr { spin_lock_bh(&mut (*d).tcf_lock); }
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    let p_old = rcu_dereference_protected((*d).skbmod_p, 1);
    if lflags & SKBMOD_F_DMAC != 0 { ether_addr_copy((*p).eth_dst.as_mut_ptr(), daddr); }
    if lflags & SKBMOD_F_SMAC != 0 { ether_addr_copy((*p).eth_src.as_mut_ptr(), saddr); }
    if lflags & SKBMOD_F_ETYPE != 0 { (*p).eth_type = htons(eth_type); }
    rcu_assign_pointer(&mut (*d).skbmod_p, p);
    if ovr { spin_unlock_bh(&mut (*d).tcf_lock); }
    if !p_old.is_null() { kfree_rcu(p_old, rcu); }
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    return ret;
put_chain:
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
release_idr:
    tcf_idr_release(*a, bind); err
}

unsafe fn tcf_skbmod_cleanup(a: *mut tc_action) {
    let d = to_skbmod(a);
    let p = rcu_dereference_protected((*d).skbmod_p, 1);
    if !p.is_null() { kfree_rcu(p, rcu); }
}

unsafe fn tcf_skbmod_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let d = to_skbmod(a);
    let b = skb_tail_pointer(skb);
    let mut opt: tc_skbmod = core::mem::zeroed();
    let mut t: tcf_t = core::mem::zeroed();
    opt.index = (*d).tcf_index;
    opt.refcnt = refcount_read(&(*d).tcf_refcnt) - ref_;
    opt.bindcnt = atomic_read(&(*d).tcf_bindcnt) - bind;
    rcu_read_lock();
    let p = rcu_dereference((*d).skbmod_p);
    opt.action = (*p).action; opt.flags = (*p).flags;
    if nla_put(skb, TCA_SKBMOD_PARMS, core::mem::size_of::<tc_skbmod>() as i32, &opt as *const _ as *const core::ffi::c_void) != 0 { goto nla_put_failure; }
    if (*p).flags & SKBMOD_F_DMAC != 0 && nla_put(skb, TCA_SKBMOD_DMAC, ETH_ALEN as i32, (*p).eth_dst.as_ptr() as *const _) != 0 { goto nla_put_failure; }
    if (*p).flags & SKBMOD_F_SMAC != 0 && nla_put(skb, TCA_SKBMOD_SMAC, ETH_ALEN as i32, (*p).eth_src.as_ptr() as *const _) != 0 { goto nla_put_failure; }
    if (*p).flags & SKBMOD_F_ETYPE != 0 && nla_put_u16(skb, TCA_SKBMOD_ETYPE, ntohs((*p).eth_type)) != 0 { goto nla_put_failure; }
    tcf_tm_dump(&mut t, &(*d).tcf_tm);
    if nla_put_64bit(skb, TCA_SKBMOD_TM, core::mem::size_of::<tcf_t>() as i32, &t, TCA_SKBMOD_PAD) != 0 { goto nla_put_failure; }
    rcu_read_unlock(); return (*skb).len;
nla_put_failure:
    rcu_read_unlock(); nlmsg_trim(skb, b); -1
}

static mut skbmod_net_ops: pernet_operations = pernet_operations { init: Some(skbmod_init_net), exit_batch: Some(skbmod_exit_net), id: unsafe { &mut ACT_SKBMOD_OPS.net_id }, size: core::mem::size_of::<tc_action_net>() };

unsafe fn skbmod_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, ACT_SKBMOD_OPS.net_id), &ACT_SKBMOD_OPS) }
unsafe fn skbmod_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, ACT_SKBMOD_OPS.net_id); }
unsafe fn skbmod_init_module() -> i32 { tcf_register_action(&ACT_SKBMOD_OPS, &skbmod_net_ops) }
unsafe fn skbmod_cleanup_module() { tcf_unregister_action(&ACT_SKBMOD_OPS, &skbmod_net_ops); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
