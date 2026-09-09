// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Patrick McHardy <kaber@trash.net>
 */

// Translated dependencies:
// linux/netfilter_ipv6/ip6_tables.h
// linux/netfilter/x_tables.h
// linux/netfilter/xt_SYNPROXY.h
// net/netfilter/nf_synproxy.h

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel types and functions supplied by the surrounding tree.
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct synproxy_net;
#[repr(C)]
pub struct xt_action_param {
    pub targinfo: *const c_void,
    pub net: *mut net,
    pub hooknum: c_uint,
    pub thoff: c_uint,
}
#[repr(C)]
pub struct xt_tgchk_param {
    pub net: *mut net,
    pub entryinfo: *const ip6t_entry,
    pub family: c_uint,
}
#[repr(C)]
pub struct xt_tgdtor_param {
    pub net: *mut net,
    pub family: c_uint,
}
#[repr(C)]
pub struct ip6t_ip6 {
    pub flags: u8,
    pub proto: u8,
    pub invflags: u8,
}
#[repr(C)]
pub struct ip6t_entry {
    pub ipv6: ip6t_ip6,
}
#[repr(C)]
pub struct xt_synproxy_info {
    pub options: c_uint,
    pub mss: u16,
}
#[repr(C)]
pub struct synproxy_options {
    pub options: c_uint,
    pub mss_encode: u16,
    pub mss_option: u16,
}
#[repr(C)]
pub struct tcphdr {
    pub seq: u32,
    pub syn: bool,
    pub ack: bool,
    pub fin: bool,
    pub rst: bool,
    pub ece: bool,
    pub cwr: bool,
}
#[repr(C)]
pub struct synproxy_stats;

#[repr(C)]
pub struct xt_target {
    pub name: *const c_char,
    pub family: c_uint,
    pub hooks: c_uint,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> c_uint>,
    pub targetsize: usize,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>,
    pub me: *mut c_void,
}

extern "C" {
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn synproxy_pernet(net: *mut net) -> *mut synproxy_net;
    fn nf_ip6_checksum(skb: *mut sk_buff, hook: c_uint, thoff: c_uint, protocol: c_uint) -> c_int;
    fn skb_header_pointer(skb: *mut sk_buff, offset: c_uint, len: usize, buffer: *mut tcphdr) -> *mut tcphdr;
    fn synproxy_parse_options(skb: *mut sk_buff, thoff: c_uint, th: *const tcphdr, opts: *mut synproxy_options) -> bool;
    fn synproxy_init_timestamp_cookie(info: *const xt_synproxy_info, opts: *mut synproxy_options);
    fn synproxy_send_client_synack_ipv6(net: *mut net, skb: *mut sk_buff, th: *const tcphdr, opts: *const synproxy_options);
    fn synproxy_recv_client_ack_ipv6(net: *mut net, skb: *mut sk_buff, th: *const tcphdr, opts: *mut synproxy_options, seq: u32) -> bool;
    fn consume_skb(skb: *mut sk_buff);
    fn nf_ct_netns_get(net: *mut net, family: c_uint) -> c_int;
    fn nf_ct_netns_put(net: *mut net, family: c_uint);
    fn nf_synproxy_ipv6_init(snet: *mut synproxy_net, net: *mut net) -> c_int;
    fn nf_synproxy_ipv6_fini(snet: *mut synproxy_net, net: *mut net);
    fn xt_register_target(target: *mut xt_target) -> c_int;
    fn xt_unregister_target(target: *mut xt_target);
}

const NF_DROP: c_uint = 0;
const NF_STOLEN: c_uint = 4;
const XT_CONTINUE: c_uint = 0xffff_ffff;
const IPPROTO_TCP: c_uint = 6;
const NFPROTO_IPV6: c_uint = 10;
const IP6T_F_PROTO: u8 = 0x01;
const XT_INV_PROTO: u8 = 0x01;
const XT_SYNPROXY_OPT_ECN: c_uint = 1 << 0;
const XT_SYNPROXY_OPT_TIMESTAMP: c_uint = 1 << 1;
const XT_SYNPROXY_OPT_WSCALE: c_uint = 1 << 2;
const XT_SYNPROXY_OPT_SACK_PERM: c_uint = 1 << 3;
const NF_INET_LOCAL_IN: c_uint = 1;
const NF_INET_FORWARD: c_uint = 2;

static mut synproxy_tg6_reg: xt_target = xt_target {
    name: b"SYNPROXY\0".as_ptr() as *const c_char,
    family: NFPROTO_IPV6,
    hooks: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD),
    target: Some(synproxy_tg6),
    targetsize: core::mem::size_of::<xt_synproxy_info>(),
    checkentry: Some(synproxy_tg6_check),
    destroy: Some(synproxy_tg6_destroy),
    me: core::ptr::null_mut(), // THIS_MODULE
};

unsafe extern "C" fn synproxy_tg6(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const xt_synproxy_info;
    let net = xt_net(par);
    let snet = synproxy_pernet(net);
    let mut opts: synproxy_options = core::mem::zeroed();
    let mut th_storage: tcphdr = core::mem::zeroed();
    let th = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<tcphdr>(), &mut th_storage);

    if nf_ip6_checksum(skb, (*par).hooknum, (*par).thoff, IPPROTO_TCP) != 0 || th.is_null() {
        return NF_DROP;
    }
    if !synproxy_parse_options(skb, (*par).thoff, th, &mut opts) {
        return NF_DROP;
    }
    if (*th).syn && !((*th).ack || (*th).fin || (*th).rst) {
        // Initial SYN from client
        // this_cpu_inc(snet->stats->syn_received);
        if (*th).ece && (*th).cwr { opts.options |= XT_SYNPROXY_OPT_ECN; }
        opts.options &= (*info).options;
        opts.mss_encode = opts.mss_option;
        opts.mss_option = (*info).mss;
        if opts.options & XT_SYNPROXY_OPT_TIMESTAMP != 0 { synproxy_init_timestamp_cookie(info, &mut opts); }
        else { opts.options &= !(XT_SYNPROXY_OPT_WSCALE | XT_SYNPROXY_OPT_SACK_PERM | XT_SYNPROXY_OPT_ECN); }
        synproxy_send_client_synack_ipv6(net, skb, th, &opts);
        consume_skb(skb);
        return NF_STOLEN;
    } else if (*th).ack && !((*th).fin || (*th).rst || (*th).syn) {
        // ACK from client
        if synproxy_recv_client_ack_ipv6(net, skb, th, &mut opts, u32::from_be((*th).seq)) { consume_skb(skb); return NF_STOLEN; }
        return NF_DROP;
    }
    XT_CONTINUE
}

unsafe extern "C" fn synproxy_tg6_check(par: *const xt_tgchk_param) -> c_int {
    let snet = synproxy_pernet((*par).net);
    let e = (*par).entryinfo;
    if ((*e).ipv6.flags & IP6T_F_PROTO) == 0 || (*e).ipv6.proto as c_uint != IPPROTO_TCP || ((*e).ipv6.invflags & XT_INV_PROTO) != 0 { return -22; }
    let mut err = nf_ct_netns_get((*par).net, (*par).family);
    if err != 0 { return err; }
    err = nf_synproxy_ipv6_init(snet, (*par).net);
    if err != 0 { nf_ct_netns_put((*par).net, (*par).family); }
    err
}

unsafe extern "C" fn synproxy_tg6_destroy(par: *const xt_tgdtor_param) {
    let snet = synproxy_pernet((*par).net);
    nf_synproxy_ipv6_fini(snet, (*par).net);
    nf_ct_netns_put((*par).net, (*par).family);
}

unsafe extern "C" fn synproxy_tg6_init() -> c_int { xt_register_target(&mut synproxy_tg6_reg) }
unsafe extern "C" fn synproxy_tg6_exit() { xt_unregister_target(&mut synproxy_tg6_reg); }

// module_init(synproxy_tg6_init);
// module_exit(synproxy_tg6_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("Intercept IPv6 TCP connections and establish them using syncookies");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
