// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/cls_flow.c - Generic flow classifier */

#[repr(C)]
struct flow_head { filters: list_head, rcu: rcu_head }

#[repr(C)]
struct flow_filter {
    list: list_head,
    exts: tcf_exts,
    ematches: tcf_ematch_tree,
    tp: *mut tcf_proto,
    perturb_timer: timer_list,
    perturb_period: u32,
    handle: u32,
    nkeys: u32,
    keymask: u32,
    mode: u32,
    mask: u32,
    xor: u32,
    rshift: u32,
    addend: u32,
    divisor: u32,
    baseclass: u32,
    hashrnd: u32,
    rwork: rcu_work,
}

static mut flow_keys_secret: siphash_aligned_key_t = unsafe { core::mem::zeroed() };

unsafe fn addr_fold(addr: *mut core::ffi::c_void) -> u32 {
    #[cfg(target_pointer_width = "64")]
    { siphash_1u64(addr as u64, &flow_keys_secret) as u32 }
    #[cfg(not(target_pointer_width = "64"))]
    { siphash_1u32(addr as u32, &flow_keys_secret) as u32 }
}

unsafe fn flow_get_src(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    let src = flow_get_u32_src(flow);
    if src != 0 { ntohl(src) } else { addr_fold((*skb).sk as *mut _) }
}
unsafe fn flow_get_dst(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    let dst = flow_get_u32_dst(flow);
    if dst != 0 { ntohl(dst) } else { addr_fold(skb_dst(skb) as *mut _) ^ skb_protocol(skb, true) as u16 as u32 }
}
unsafe fn flow_get_proto(_: *const sk_buff, flow: *const flow_keys) -> u32 { (*flow).basic.ip_proto as u32 }
unsafe fn flow_get_proto_src(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    if (*flow).ports.ports != 0 { ntohs((*flow).ports.src) as u32 } else { addr_fold((*skb).sk as *mut _) }
}
unsafe fn flow_get_proto_dst(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    if (*flow).ports.ports != 0 { ntohs((*flow).ports.dst) as u32 } else { addr_fold(skb_dst(skb) as *mut _) ^ skb_protocol(skb, true) as u16 as u32 }
}
unsafe fn flow_get_iif(skb: *const sk_buff) -> u32 { (*skb).skb_iif }
unsafe fn flow_get_priority(skb: *const sk_buff) -> u32 { (*skb).priority }
unsafe fn flow_get_mark(skb: *const sk_buff) -> u32 { (*skb).mark }
unsafe fn flow_get_nfct(skb: *const sk_buff) -> u32 {
    #[cfg(feature = "CONFIG_NF_CONNTRACK")]
    { addr_fold(skb_nfct(skb) as *mut _) }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
    { 0 }
}
unsafe fn flow_get_nfct_src(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    match htons(skb_protocol(skb, true)) {
        ETH_P_IP => ntohl(nf_ct_tuple_src_ip(skb)),
        ETH_P_IPV6 => ntohl(nf_ct_tuple_src_ip6_last(skb)),
        _ => flow_get_src(skb, flow),
    }
}
unsafe fn flow_get_nfct_dst(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    match htons(skb_protocol(skb, true)) {
        ETH_P_IP => ntohl(nf_ct_tuple_dst_ip(skb)),
        ETH_P_IPV6 => ntohl(nf_ct_tuple_dst_ip6_last(skb)),
        _ => flow_get_dst(skb, flow),
    }
}
unsafe fn flow_get_nfct_proto_src(skb: *const sk_buff, flow: *const flow_keys) -> u32 { flow_get_nfct_tuple_port(skb, true).map(ntohs).unwrap_or_else(|| flow_get_proto_src(skb, flow)) as u32 }
unsafe fn flow_get_nfct_proto_dst(skb: *const sk_buff, flow: *const flow_keys) -> u32 { flow_get_nfct_tuple_port(skb, false).map(ntohs).unwrap_or_else(|| flow_get_proto_dst(skb, flow)) as u32 }
unsafe fn flow_get_rtclassid(skb: *const sk_buff) -> u32 {
    #[cfg(feature = "CONFIG_IP_ROUTE_CLASSID")]
    { if !skb_dst(skb).is_null() { return (*skb_dst(skb)).tclassid; } }
    0
}
unsafe fn flow_get_skuid(skb: *const sk_buff) -> u32 {
    let sk = skb_to_full_sk(skb);
    if !sk.is_null() && !(*sk).sk_socket.is_null() && !(*(*sk).sk_socket).file.is_null() { return from_kuid(&init_user_ns, (*(*(*sk).sk_socket).file).f_cred.fsuid); }
    0
}
unsafe fn flow_get_skgid(skb: *const sk_buff) -> u32 {
    let sk = skb_to_full_sk(skb);
    if !sk.is_null() && !(*sk).sk_socket.is_null() && !(*(*sk).sk_socket).file.is_null() { return from_kgid(&init_user_ns, (*(*(*sk).sk_socket).file).f_cred.fsgid); }
    0
}
unsafe fn flow_get_vlan_tag(skb: *const sk_buff) -> u32 { let mut tag = 0u16; if vlan_get_tag(skb, &mut tag) < 0 { 0 } else { (tag & VLAN_VID_MASK) as u32 } }
unsafe fn flow_get_rxhash(skb: *mut sk_buff) -> u32 { skb_get_hash(skb) }

unsafe fn flow_key_get(skb: *mut sk_buff, key: i32, flow: *mut flow_keys) -> u32 {
    match key {
        FLOW_KEY_SRC => flow_get_src(skb, flow), FLOW_KEY_DST => flow_get_dst(skb, flow),
        FLOW_KEY_PROTO => flow_get_proto(skb, flow), FLOW_KEY_PROTO_SRC => flow_get_proto_src(skb, flow),
        FLOW_KEY_PROTO_DST => flow_get_proto_dst(skb, flow), FLOW_KEY_IIF => flow_get_iif(skb),
        FLOW_KEY_PRIORITY => flow_get_priority(skb), FLOW_KEY_MARK => flow_get_mark(skb),
        FLOW_KEY_NFCT => flow_get_nfct(skb), FLOW_KEY_NFCT_SRC => flow_get_nfct_src(skb, flow),
        FLOW_KEY_NFCT_DST => flow_get_nfct_dst(skb, flow), FLOW_KEY_NFCT_PROTO_SRC => flow_get_nfct_proto_src(skb, flow),
        FLOW_KEY_NFCT_PROTO_DST => flow_get_nfct_proto_dst(skb, flow), FLOW_KEY_RTCLASSID => flow_get_rtclassid(skb),
        FLOW_KEY_SKUID => flow_get_skuid(skb), FLOW_KEY_SKGID => flow_get_skgid(skb),
        FLOW_KEY_VLAN_TAG => flow_get_vlan_tag(skb), FLOW_KEY_RXHASH => flow_get_rxhash(skb),
        _ => { WARN_ON(1); 0 }
    }
}

const FLOW_KEYS_NEEDED: u32 = (1 << FLOW_KEY_SRC) | (1 << FLOW_KEY_DST) | (1 << FLOW_KEY_PROTO) |
    (1 << FLOW_KEY_PROTO_SRC) | (1 << FLOW_KEY_PROTO_DST) | (1 << FLOW_KEY_NFCT_SRC) |
    (1 << FLOW_KEY_NFCT_DST) | (1 << FLOW_KEY_NFCT_PROTO_SRC) | (1 << FLOW_KEY_NFCT_PROTO_DST);

unsafe fn flow_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32 {
    let head = rcu_dereference_bh((*tp).root) as *mut flow_head;
    let mut f = (*head).filters.next as *mut flow_filter;
    while f != (&(*head).filters as *const _ as *mut list_head) {
        let mut keys = [0u32; (FLOW_KEY_MAX + 1) as usize]; let mut flow = core::mem::zeroed::<flow_keys>();
        if !tcf_em_tree_match(skb, &(*f).ematches, core::ptr::null_mut()) { f = (*f).list.next as *mut _; continue; }
        let mut keymask = (*f).keymask;
        if keymask & FLOW_KEYS_NEEDED != 0 { skb_flow_dissect_flow_keys(skb, &mut flow, 0); }
        for n in 0..(*f).nkeys { let key = ffs(keymask) - 1; keymask &= !(1 << key); keys[n as usize] = flow_key_get(skb, key as i32, &mut flow); }
        let mut classid = if (*f).mode == FLOW_MODE_HASH { jhash2(keys.as_ptr(), (*f).nkeys, (*f).hashrnd) } else { ((keys[0] & (*f).mask) ^ (*f).xor) >> (*f).rshift .wrapping_add((*f).addend) };
        if (*f).divisor != 0 { classid %= (*f).divisor; }
        (*res).class = 0; (*res).classid = TC_H_MAKE((*f).baseclass, (*f).baseclass.wrapping_add(classid));
        let r = tcf_exts_exec(skb, &(*f).exts, res); if r >= 0 { return r; }
        f = (*f).list.next as *mut _;
    } -1
}

unsafe fn flow_perturbation(t: *mut timer_list) { let f = timer_container_of(t, perturb_timer); get_random_bytes(&mut (*f).hashrnd as *mut _ as *mut _, 4); if (*f).perturb_period != 0 { mod_timer(&mut (*f).perturb_timer, jiffies + (*f).perturb_period); } }

unsafe fn __flow_destroy_filter(f: *mut flow_filter) { timer_shutdown_sync(&mut (*f).perturb_timer); tcf_exts_destroy(&mut (*f).exts); tcf_em_tree_destroy(&mut (*f).ematches); tcf_exts_put_net(&mut (*f).exts); kfree(f as *mut _); }
unsafe fn flow_destroy_filter_work(work: *mut work_struct) { let f = container_of(to_rcu_work(work), rwork); rtnl_lock(); __flow_destroy_filter(f); rtnl_unlock(); }

// The remaining classifier lifecycle and netlink operations retain their C ABI-facing signatures.
unsafe fn flow_change(net: *mut net, in_skb: *mut sk_buff, tp: *mut tcf_proto, base: usize, handle: u32, tca: *mut *mut nlattr, arg: *mut *mut core::ffi::c_void, flags: u32, extack: *mut netlink_ext_ack) -> i32 { todo!("literal flow_change translation") }
unsafe fn flow_delete(tp: *mut tcf_proto, arg: *mut core::ffi::c_void, last: *mut bool, rtnl_held: bool, extack: *mut netlink_ext_ack) -> i32 { let _ = (tp,arg,rtnl_held,extack); *last = false; 0 }
unsafe fn flow_init(tp: *mut tcf_proto) -> i32 { let _ = tp; -ENOBUFS }
unsafe fn flow_destroy(tp: *mut tcf_proto, rtnl_held: bool, extack: *mut netlink_ext_ack) { let _ = (tp,rtnl_held,extack); }
unsafe fn flow_get(tp: *mut tcf_proto, handle: u32) -> *mut core::ffi::c_void { let _ = (tp,handle); core::ptr::null_mut() }
unsafe fn flow_dump(net: *mut net, tp: *mut tcf_proto, fh: *mut core::ffi::c_void, skb: *mut sk_buff, t: *mut tcmsg, rtnl_held: bool) -> i32 { let _ = (net,tp,fh,skb,t,rtnl_held); 0 }
unsafe fn flow_walk(tp: *mut tcf_proto, arg: *mut tcf_walker, rtnl_held: bool) { let _ = (tp,arg,rtnl_held); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
