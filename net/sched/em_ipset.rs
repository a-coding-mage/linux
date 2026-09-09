// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/sched/em_ipset.c ipset ematch
 *
 * Copyright (c) 2012 Florian Westphal <fw@strlen.de>
 */

// The declarations below are supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    fn ip_set_nfnl_get_byindex(net: *mut net, index: ip_set_id_t) -> ip_set_id_t;
    fn ip_set_nfnl_put(net: *mut net, index: ip_set_id_t);
    fn kmemdup(src: *const c_void, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn skb_protocol(skb: *mut sk_buff, inside: bool) -> u16;
    fn pskb_network_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn ip_hdrlen(skb: *mut sk_buff) -> u8;
    fn skb_network_offset(skb: *mut sk_buff) -> i32;
    fn skb_pull(skb: *mut sk_buff, len: u32);
    fn skb_push(skb: *mut sk_buff, len: u32);
    fn dev_get_by_index_rcu(net: *mut net, ifindex: i32) -> *mut net_device;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn ip_set_test(index: ip_set_id_t, skb: *mut sk_buff,
                   par: *mut xt_action_param, opt: *mut ip_set_adt_opt) -> i32;
    fn tcf_em_register(ops: *mut tcf_ematch_ops) -> i32;
    fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
}

type ip_set_id_t = u16;

const IPSET_INVALID_ID: ip_set_id_t = 65535;
const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const GFP_KERNEL: u32 = 0x00d0;
const TCF_EM_IPSET: i32 = 2;

#[repr(C)]
struct net;
#[repr(C)]
struct net_device;
#[repr(C)]
struct iphdr;
#[repr(C)]
struct ipv6hdr;
#[repr(C)]
struct sk_buff {
    skb_iif: i32,
    dev: *mut net_device,
}
#[repr(C)]
struct xt_set_info {
    index: ip_set_id_t,
    dim: u8,
    flags: u8,
}
#[repr(C)]
struct ip_set_adt_opt {
    family: u8,
    dim: u8,
    flags: u8,
    cmdflags: u32,
    ext: ip_set_adt_opt_ext,
}
#[repr(C)]
struct ip_set_adt_opt_ext {
    timeout: u32,
}
#[repr(C)]
struct nf_hook_state {
    net: *mut net,
    pf: u8,
    input: *mut net_device,
    output: *mut net_device,
}
#[repr(C)]
struct xt_action_param {
    thoff: u8,
    state: *mut nf_hook_state,
}
#[repr(C)]
struct tcf_ematch {
    datalen: u32,
    data: usize,
    net: *mut net,
}
#[repr(C)]
struct tcf_ematch_ops {
    kind: i32,
    change: Option<unsafe extern "C" fn(*mut net, *mut c_void, i32, *mut tcf_ematch) -> i32>,
    destroy: Option<unsafe extern "C" fn(*mut tcf_ematch)>,
    r#match: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> i32>,
    owner: *mut c_void,
    link: [usize; 2],
}
#[repr(C)]
struct tcf_pkt_info;

unsafe extern "C" fn em_ipset_change(net: *mut net, data: *mut c_void, data_len: i32,
                                      em: *mut tcf_ematch) -> i32 {
    let set = data as *mut xt_set_info;
    let index: ip_set_id_t;
    if data_len as usize != core::mem::size_of::<xt_set_info>() { return -EINVAL; }
    index = ip_set_nfnl_get_byindex(net, (*set).index);
    if index == IPSET_INVALID_ID { return -ENOENT; }
    (*em).datalen = core::mem::size_of::<xt_set_info>() as u32;
    (*em).data = kmemdup(data, (*em).datalen as usize, GFP_KERNEL) as usize;
    if (*em).data != 0 { return 0; }
    ip_set_nfnl_put(net, index);
    -ENOMEM
}

unsafe extern "C" fn em_ipset_destroy(em: *mut tcf_ematch) {
    let set = (*em).data as *const xt_set_info;
    if !set.is_null() {
        ip_set_nfnl_put((*em).net, (*set).index);
        kfree((*em).data as *mut c_void);
    }
}

unsafe extern "C" fn em_ipset_match(skb: *mut sk_buff, em: *mut tcf_ematch,
                                     _info: *mut tcf_pkt_info) -> i32 {
    let mut opt = ip_set_adt_opt { family: 0, dim: 0, flags: 0, cmdflags: 0,
        ext: ip_set_adt_opt_ext { timeout: 0 } };
    let mut acpar = xt_action_param { thoff: 0, state: core::ptr::null_mut() };
    let set = (*em).data as *const xt_set_info;
    let mut state = nf_hook_state { net: (*em).net, pf: 0, input: core::ptr::null_mut(), output: core::ptr::null_mut() };
    let protocol = skb_protocol(skb, true);
    match protocol {
        ETH_P_IP => { state.pf = NFPROTO_IPV4; if !pskb_network_may_pull(skb, core::mem::size_of::<iphdr>()) { return 0; } acpar.thoff = ip_hdrlen(skb); }
        ETH_P_IPV6 => { state.pf = NFPROTO_IPV6; if !pskb_network_may_pull(skb, core::mem::size_of::<ipv6hdr>()) { return 0; } acpar.thoff = core::mem::size_of::<ipv6hdr>() as u8; }
        _ => return 0,
    }
    opt.family = state.pf; opt.dim = (*set).dim; opt.flags = (*set).flags; opt.cmdflags = 0; opt.ext.timeout = !0u32;
    let network_offset = skb_network_offset(skb); skb_pull(skb, network_offset as u32);
    let dev = (*skb).dev; rcu_read_lock();
    if (*skb).skb_iif != 0 { state.input = dev_get_by_index_rcu((*em).net, (*skb).skb_iif); }
    state.output = dev; acpar.state = &mut state;
    let ret = ip_set_test((*set).index, skb, &mut acpar, &mut opt);
    rcu_read_unlock(); skb_push(skb, network_offset as u32); ret
}

static mut em_ipset_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: TCF_EM_IPSET, change: Some(em_ipset_change), destroy: Some(em_ipset_destroy),
    r#match: Some(em_ipset_match), owner: core::ptr::null_mut(), link: [0, 0],
};

unsafe extern "C" fn init_em_ipset() -> i32 { tcf_em_register(&mut em_ipset_ops) }
unsafe extern "C" fn exit_em_ipset() { tcf_em_unregister(&mut em_ipset_ops); }

// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
// MODULE_DESCRIPTION("TC extended match for IP sets");
// module_init(init_em_ipset); module_exit(exit_em_ipset);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_IPSET);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
