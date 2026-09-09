// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/act_pedit.c - Generic packet editor */

// Kernel headers and symbols used by this translation are supplied by the
// surrounding kernel/Rust bindings.

static mut act_pedit_ops: tc_action_ops = tc_action_ops::ZERO;

static pedit_policy: [nla_policy; TCA_PEDIT_MAX as usize + 1] = [nla_policy::ZERO; TCA_PEDIT_MAX as usize + 1];
static pedit_key_ex_policy: [nla_policy; TCA_PEDIT_KEY_EX_MAX as usize + 1] = [nla_policy::ZERO; TCA_PEDIT_KEY_EX_MAX as usize + 1];

unsafe fn tcf_pedit_keys_ex_parse(nla: *mut nlattr, mut n: u8,
                                  extack: *mut netlink_ext_ack) -> *mut tcf_pedit_key_ex {
    if nla.is_null() { return core::ptr::null_mut(); }
    let keys_ex = kzalloc_objs::<tcf_pedit_key_ex>(n);
    if keys_ex.is_null() { return ERR_PTR(-ENOMEM); }
    let mut k = keys_ex;
    let mut ka: *const nlattr;
    let mut rem: i32 = 0;
    nla_for_each_nested!(ka, nla, rem) {
        let mut tb: [*mut nlattr; TCA_PEDIT_KEY_EX_MAX as usize + 1] = [core::ptr::null_mut(); TCA_PEDIT_KEY_EX_MAX as usize + 1];
        if n == 0 { NL_SET_ERR_MSG_MOD!(extack, "Can't parse more extended keys than requested"); goto_err!(keys_ex, -EINVAL); }
        n -= 1;
        if nla_type(ka) != TCA_PEDIT_KEY_EX { NL_SET_ERR_MSG_ATTR!(extack, ka, "Unknown attribute, expected extended key"); goto_err!(keys_ex, -EINVAL); }
        let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_PEDIT_KEY_EX_MAX, ka, pedit_key_ex_policy.as_ptr(), core::ptr::null_mut());
        if err != 0 { goto_err!(keys_ex, err); }
        if NL_REQ_ATTR_CHECK!(extack, nla, tb.as_mut_ptr(), TCA_PEDIT_KEY_EX_HTYPE) || NL_REQ_ATTR_CHECK!(extack, nla, tb.as_mut_ptr(), TCA_PEDIT_KEY_EX_CMD) {
            NL_SET_ERR_MSG!(extack, "Missing required attribute"); goto_err!(keys_ex, -EINVAL);
        }
        (*k).htype = nla_get_u16(tb[TCA_PEDIT_KEY_EX_HTYPE as usize]);
        (*k).cmd = nla_get_u16(tb[TCA_PEDIT_KEY_EX_CMD as usize]);
        k = k.add(1);
    }
    if n != 0 { NL_SET_ERR_MSG_MOD!(extack, "Not enough extended keys to parse"); goto_err!(keys_ex, -EINVAL); }
    keys_ex
}

unsafe fn tcf_pedit_key_ex_dump(skb: *mut sk_buff, mut keys_ex: *mut tcf_pedit_key_ex, mut n: i32) -> i32 {
    let keys_start = nla_nest_start_noflag(skb, TCA_PEDIT_KEYS_EX);
    if keys_start.is_null() { return -EINVAL; }
    while n > 0 {
        let key_start = nla_nest_start_noflag(skb, TCA_PEDIT_KEY_EX);
        if key_start.is_null() || nla_put_u16(skb, TCA_PEDIT_KEY_EX_HTYPE, (*keys_ex).htype) != 0 || nla_put_u16(skb, TCA_PEDIT_KEY_EX_CMD, (*keys_ex).cmd) != 0 {
            nla_nest_cancel(skb, keys_start); return -EINVAL;
        }
        nla_nest_end(skb, key_start); keys_ex = keys_ex.add(1); n -= 1;
    }
    nla_nest_end(skb, keys_start); 0
}

unsafe fn tcf_pedit_cleanup_rcu(head: *mut rcu_head) {
    let parms = container_of!(head, tcf_pedit_parms, rcu);
    kfree((*parms).tcfp_keys_ex); kfree((*parms).tcfp_keys); kfree(parms);
}

unsafe fn tcf_pedit_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr, a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32, extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic::<tc_action_net>(net, (*act_pedit_ops).net_id);
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut tb: [*mut nlattr; TCA_PEDIT_MAX as usize + 1] = [core::ptr::null_mut(); TCA_PEDIT_MAX as usize + 1];
    if nla.is_null() { NL_SET_ERR_MSG_MOD!(extack, "Pedit requires attributes to be passed"); return -EINVAL; }
    let mut err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_PEDIT_MAX, nla, pedit_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    let mut pattr = tb[TCA_PEDIT_PARMS as usize]; if pattr.is_null() { pattr = tb[TCA_PEDIT_PARMS_EX as usize]; }
    if pattr.is_null() { NL_SET_ERR_MSG_MOD!(extack, "Missing required TCA_PEDIT_PARMS or TCA_PEDIT_PARMS_EX pedit attribute"); return -EINVAL; }
    let parm = nla_data::<tc_pedit>(pattr); let index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &index as *const _ as *mut u32, a, bind);
    let mut ret = 0;
    if err == 0 { ret = tcf_idr_create_from_flags(tn, index, est, a, &act_pedit_ops, bind, flags); if ret != 0 { tcf_idr_cleanup(tn, index); return ret; } ret = ACT_P_CREATED; }
    else if err > 0 { if bind { return ACT_P_BOUND; } if flags & TCA_ACT_FLAGS_REPLACE == 0 { ret = -EEXIST; goto_release!(); } }
    else { return err; }
    if (*parm).nkeys == 0 { NL_SET_ERR_MSG_MOD!(extack, "Pedit requires keys to be passed"); ret = -EINVAL; goto_release!(); }
    let ksize = (*parm).nkeys as usize * core::mem::size_of::<tc_pedit_key>();
    if nla_len(pattr) < core::mem::size_of::<tc_pedit>() + ksize { NL_SET_ERR_MSG_ATTR!(extack, pattr, "Length of pedit attribute is invalid"); ret = -EINVAL; goto_release!(); }
    let nparms = kzalloc_obj::<tcf_pedit_parms>(); if nparms.is_null() { ret = -ENOMEM; goto_release!(); }
    (*nparms).tcfp_keys_ex = tcf_pedit_keys_ex_parse(tb[TCA_PEDIT_KEYS_EX as usize], (*parm).nkeys, extack);
    if IS_ERR!((*nparms).tcfp_keys_ex) { ret = PTR_ERR!((*nparms).tcfp_keys_ex); goto_free!(); }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack); if err < 0 { ret = err; goto_free_ex!(); }
    (*nparms).tcfp_flags = (*parm).flags; (*nparms).tcfp_nkeys = (*parm).nkeys;
    (*nparms).tcfp_keys = kmemdup((*parm).keys, ksize, GFP_KERNEL); if (*nparms).tcfp_keys.is_null() { ret = -ENOMEM; goto_chain!(); }
    for i in 0..(*nparms).tcfp_nkeys as usize { let key = &mut *(*nparms).tcfp_keys.add(i); let offmask = key.offmask; let cur = key.off; if offmask == 0 && cur % 4 != 0 { NL_SET_ERR_MSG_MOD!(extack, "Offsets must be on 32bit boundaries"); ret = -EINVAL; goto_keys!(); } key.shift = core::cmp::min(core::mem::size_of::<i32>() * 8 - 1, key.shift as usize) as _; }
    let p = to_pedit(*a); (*nparms).action = (*parm).action; spin_lock_bh!((*p).tcf_lock); goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch); let oparms = rcu_replace_pointer!((*p).parms, nparms, 1); spin_unlock_bh!((*p).tcf_lock); if !oparms.is_null() { call_rcu!(&mut (*oparms).rcu, tcf_pedit_cleanup_rcu); } if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } return ret;
}

unsafe fn tcf_pedit_cleanup(a: *mut tc_action) { let p = to_pedit(a); let parms = rcu_dereference_protected!((*p).parms, 1); if !parms.is_null() { call_rcu!(&mut (*parms).rcu, tcf_pedit_cleanup_rcu); } }
unsafe fn offset_valid(skb: *mut sk_buff, offset: i32, len: i32) -> bool { offset >= -(skb_headroom(skb) as i32) && offset <= (*skb).len as i32 - len }

unsafe fn pedit_l4_skb_offset(skb: *mut sk_buff, hoffset: *mut i32, header_type: i32) -> i32 { let noff = skb_network_offset(skb) as i32; let mut iph = core::mem::MaybeUninit::<iphdr>::uninit(); match (*skb).protocol { x if x == htons(ETH_P_IP) => { let p = skb_header_pointer(skb, noff, core::mem::size_of::<iphdr>(), iph.as_mut_ptr()); if p.is_null() { return -EINVAL; } let ip = &*p.cast::<iphdr>(); if ip.ihl < 5 || ip.protocol as i32 != header_type || ip.frag_off & htons(IP_OFFSET) != 0 { return -EINVAL; } *hoffset = noff + ip.ihl as i32 * 4; 0 }, x if x == htons(ETH_P_IPV6) => if ipv6_find_hdr(skb, hoffset, header_type, core::ptr::null_mut(), core::ptr::null_mut()) == header_type { 0 } else { -EINVAL }, _ => -EINVAL } }

unsafe fn pedit_skb_hdr_offset(skb: *mut sk_buff, htype: pedit_header_type, hoffset: *mut i32) -> i32 { match htype { TCA_PEDIT_KEY_EX_HDR_TYPE_ETH => { if skb_mac_header_was_set(skb) { *hoffset = skb_mac_offset(skb); 0 } else { -EINVAL } }, TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK | TCA_PEDIT_KEY_EX_HDR_TYPE_IP4 | TCA_PEDIT_KEY_EX_HDR_TYPE_IP6 => { *hoffset = skb_network_offset(skb) as i32; 0 }, TCA_PEDIT_KEY_EX_HDR_TYPE_TCP => pedit_l4_skb_offset(skb, hoffset, IPPROTO_TCP), TCA_PEDIT_KEY_EX_HDR_TYPE_UDP => pedit_l4_skb_offset(skb, hoffset, IPPROTO_UDP), _ => -EINVAL } }

unsafe fn tcf_pedit_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> i32 {
    let p = to_pedit(a); let parms = rcu_dereference_bh!((*p).parms);
    tcf_lastuse_update!(&mut (*p).tcf_tm); tcf_action_update_bstats!(&mut (*p).common, skb);
    let mut tkey = (*parms).tcfp_keys; let mut tkey_ex = (*parms).tcfp_keys_ex;
    let mut htype = TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK; let mut cmd = TCA_PEDIT_KEY_EX_CMD_SET;
    for _ in 0..(*parms).tcfp_nkeys { let key = &*tkey; let mut hoffset = 0; if !tkey_ex.is_null() { htype = (*tkey_ex).htype; cmd = (*tkey_ex).cmd; tkey_ex = tkey_ex.add(1); }
        if pedit_skb_hdr_offset(skb, htype, &mut hoffset) != 0 { goto_bad!(); }
        let mut offset = key.off as i32; if key.offmask != 0 { let at = hoffset.wrapping_add(key.at as i32); if !offset_valid(skb, at, 1) { goto_bad!(); } let mut d = 0u8; if skb_header_pointer(skb, at, 1, &mut d) .is_null() { goto_bad!(); } offset = offset.wrapping_add(((d as u32 & key.offmask) >> key.shift) as i32); if offset % 4 != 0 { goto_bad!(); } }
        let write_offset = hoffset.wrapping_add(offset); if !offset_valid(skb, write_offset, 4) { goto_bad!(); }
        if write_offset < 0 { if skb_cow(skb, -write_offset) != 0 { goto_bad!(); } } else if skb_ensure_writable(skb, core::cmp::min((*skb).len as i32, write_offset + 4)) != 0 { goto_bad!(); }
        let ptr = (*skb).data.offset(write_offset as isize) as *mut u32; let cur = get_unaligned(ptr); let val = match cmd { TCA_PEDIT_KEY_EX_CMD_SET => key.val, TCA_PEDIT_KEY_EX_CMD_ADD => cur.wrapping_add(key.val) & !key.mask, _ => { goto_bad!(); 0 } }; put_unaligned((cur & key.mask) ^ val, ptr); tkey = tkey.add(1);
    }
    return (*parms).action;
    goto_bad!(); tcf_action_inc_overlimit_qstats!(&mut (*p).common); (*parms).action
}

unsafe fn tcf_pedit_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) { let d = to_pedit(a); tcf_action_update_stats(a, bytes, packets, drops, hw); (*d).tcf_tm.lastuse = core::cmp::max((*d).tcf_tm.lastuse, lastuse); }
unsafe fn tcf_pedit_get_fill_size(act: *const tc_action) -> usize { let p = to_pedit(act); let parms = rcu_dereference!((*p).parms); let mut size = nla_total_size(struct_size_t!(tc_pedit, keys, (*parms).tcfp_nkeys)); if !(*parms).tcfp_keys_ex.is_null() { size += nla_total_size(0) + (*parms).tcfp_nkeys as usize * (nla_total_size(0) + nla_total_size(2) + nla_total_size(2)); } size }

static mut act_pedit_ops: tc_action_ops = tc_action_ops { kind: "pedit", id: TCA_ID_PEDIT, owner: THIS_MODULE, act: Some(tcf_pedit_act), stats_update: Some(tcf_pedit_stats_update), cleanup: Some(tcf_pedit_cleanup), init: Some(tcf_pedit_init), get_fill_size: Some(tcf_pedit_get_fill_size), size: core::mem::size_of::<tcf_pedit>(), ..tc_action_ops::ZERO };
unsafe fn pedit_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, act_pedit_ops.net_id), &act_pedit_ops) }
unsafe fn pedit_exit_net(list: *mut list_head) { tc_action_net_exit(list, act_pedit_ops.net_id); }
unsafe fn pedit_init_module() -> i32 { tcf_register_action(&act_pedit_ops, core::ptr::null_mut()) }
unsafe fn pedit_cleanup_module() { tcf_unregister_action(&act_pedit_ops, core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
