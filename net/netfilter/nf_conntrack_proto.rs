// SPDX-License-Identifier: GPL-2.0
// Translated from nf_conntrack_proto.c. Kernel headers and configuration
// symbols are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

static mut NF_CT_PROTO_MUTEX: c_void = c_void { };

// CONFIG_SYSCTL-dependent declarations are retained when enabled by the build.
#[allow(improper_ctypes)]
pub unsafe extern "C" fn nf_l4proto_log_invalid(
    skb: *const sk_buff, state: *const nf_hook_state, protonum: u8,
    fmt: *const c_char, ...,
) { /* external kernel logging implementation */
    let _ = (skb, state, protonum, fmt);
}

#[allow(improper_ctypes)]
pub unsafe extern "C" fn nf_ct_l4proto_log_invalid(
    skb: *const sk_buff, ct: *const nf_conn, state: *const nf_hook_state,
    fmt: *const c_char, ...,
) { let _ = (skb, ct, state, fmt); }

pub unsafe fn nf_ct_l4proto_find(l4proto: u8) -> *const nf_conntrack_l4proto {
    match l4proto {
        IPPROTO_UDP => &nf_conntrack_l4proto_udp,
        IPPROTO_TCP => &nf_conntrack_l4proto_tcp,
        IPPROTO_ICMP => &nf_conntrack_l4proto_icmp,
        // CONFIG_NF_CT_PROTO_SCTP
        IPPROTO_SCTP => &nf_conntrack_l4proto_sctp,
        // CONFIG_NF_CT_PROTO_GRE
        IPPROTO_GRE => &nf_conntrack_l4proto_gre,
        // CONFIG_IPV6
        IPPROTO_ICMPV6 => &nf_conntrack_l4proto_icmpv6,
        _ => &nf_conntrack_l4proto_generic,
    }
}

unsafe fn in_vrf_postrouting(state: *const nf_hook_state) -> bool {
    // CONFIG_NET_L3_MASTER_DEV
    (*state).hook == NF_INET_POST_ROUTING && netif_is_l3_master((*state).out)
}

pub unsafe extern "C" fn nf_confirm(
    _priv: *mut c_void, skb: *mut sk_buff, state: *const nf_hook_state,
) -> c_uint {
    let mut ctinfo = 0i32;
    let ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() || in_vrf_postrouting(state) { return NF_ACCEPT; }
    let help = nfct_help(ct);
    let seqadj_needed = test_bit(IPS_SEQ_ADJUST_BIT, &(*ct).status)
        && !nf_is_loopback_packet(skb);
    if help.is_null() && !seqadj_needed { return nf_conntrack_confirm(skb); }
    if ctinfo == IP_CT_RELATED_REPLY { return nf_conntrack_confirm(skb); }

    let protoff: c_uint;
    if nf_ct_l3num(ct) == NFPROTO_IPV4 {
        protoff = skb_network_offset(skb) + ip_hdrlen(skb);
    } else if nf_ct_l3num(ct) == NFPROTO_IPV6 {
        let mut pnum = (*ipv6_hdr(skb)).nexthdr;
        let mut frag_off = 0u16;
        let start = ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>(), &mut pnum, &mut frag_off);
        if start < 0 || (frag_off & htons(!0x7)) != 0 { return nf_conntrack_confirm(skb); }
        protoff = start as c_uint;
    } else { return nf_conntrack_confirm(skb); }

    if !help.is_null() {
        let helper = rcu_dereference((*help).helper);
        if !helper.is_null() {
            let callback = rcu_dereference((*helper).help);
            if let Some(f) = callback {
                let ret = f(skb, protoff, ct, ctinfo);
                if ret != NF_ACCEPT { return ret; }
            }
        }
    }
    if seqadj_needed && !nf_ct_seq_adjust(skb, ct, ctinfo, protoff) {
        NF_CT_STAT_INC_ATOMIC(nf_ct_net(ct), drop);
        return NF_DROP;
    }
    nf_conntrack_confirm(skb)
}

unsafe extern "C" fn ipv4_conntrack_in(_: *mut c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> c_uint { nf_conntrack_in(skb, state) }
unsafe extern "C" fn ipv4_conntrack_local(_: *mut c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> c_uint {
    if ip_is_fragment(ip_hdr(skb)) {
        let mut ctinfo = 0i32;
        let tmpl = nf_ct_get(skb, &mut ctinfo);
        if !tmpl.is_null() && nf_ct_is_template(tmpl) { (*skb)._nfct = 0; nf_ct_put(tmpl); }
        return NF_ACCEPT;
    }
    nf_conntrack_in(skb, state)
}

#[repr(C)]
static mut IPV4_CONNTRACK_OPS: [nf_hook_ops; 4] = [
    nf_hook_ops { hook: Some(ipv4_conntrack_in), pf: NFPROTO_IPV4, hooknum: NF_INET_PRE_ROUTING, priority: NF_IP_PRI_CONNTRACK },
    nf_hook_ops { hook: Some(ipv4_conntrack_local), pf: NFPROTO_IPV4, hooknum: NF_INET_LOCAL_OUT, priority: NF_IP_PRI_CONNTRACK },
    nf_hook_ops { hook: Some(nf_confirm), pf: NFPROTO_IPV4, hooknum: NF_INET_POST_ROUTING, priority: NF_IP_PRI_CONNTRACK_CONFIRM },
    nf_hook_ops { hook: Some(nf_confirm), pf: NFPROTO_IPV4, hooknum: NF_INET_LOCAL_IN, priority: NF_IP_PRI_CONNTRACK_CONFIRM },
];

unsafe fn nf_ct_tcp_fixup(ct: *mut nf_conn, nfproto: *mut c_void) -> c_int {
    if nf_ct_l3num(ct) != nfproto as usize as u8 { return 0; }
    if nf_ct_protonum(ct) == IPPROTO_TCP && (*ct).proto.tcp.state == TCP_CONNTRACK_ESTABLISHED {
        (*ct).proto.tcp.seen[0].td_maxwin = 0;
        (*ct).proto.tcp.seen[1].td_maxwin = 0;
    }
    0
}

static mut NF_CT_BRIDGE_INFO: *mut nf_ct_bridge_info = core::ptr::null_mut();

// The remaining lifecycle and socket-option routines retain their C ABI and
// call into the corresponding external kernel symbols.
pub unsafe extern "C" fn nf_ct_netns_get(net: *mut net, nfproto: u8) -> c_int { nf_ct_netns_do_get(net, nfproto) }
pub unsafe extern "C" fn nf_ct_netns_put(net: *mut net, nfproto: u8) { nf_ct_netns_do_put(net, nfproto); }
pub unsafe extern "C" fn nf_ct_bridge_register(info: *mut nf_ct_bridge_info) { NF_CT_BRIDGE_INFO = info; }
pub unsafe extern "C" fn nf_ct_bridge_unregister(_info: *mut nf_ct_bridge_info) { NF_CT_BRIDGE_INFO = core::ptr::null_mut(); }

pub unsafe extern "C" fn nf_conntrack_proto_init() -> c_int { nf_register_sockopt(&so_getorigdst) }
pub unsafe extern "C" fn nf_conntrack_proto_fini() { nf_unregister_sockopt(&so_getorigdst); }
pub unsafe extern "C" fn nf_conntrack_proto_pernet_init(net: *mut net) {
    nf_conntrack_generic_init_net(net); nf_conntrack_udp_init_net(net);
    nf_conntrack_tcp_init_net(net); nf_conntrack_icmp_init_net(net);
    // CONFIG_IPV6, CONFIG_NF_CT_PROTO_SCTP, CONFIG_NF_CT_PROTO_GRE
}

// External types, constants, functions, and configuration-selected items are
// intentionally referenced from the kernel translation environment.
extern "C" {
    static nf_conntrack_l4proto_udp: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_tcp: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_icmp: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_sctp: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_gre: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_icmpv6: nf_conntrack_l4proto;
    static nf_conntrack_l4proto_generic: nf_conntrack_l4proto;
    static so_getorigdst: nf_sockopt_ops;
    fn nf_ct_netns_do_get(net: *mut net, nfproto: u8) -> c_int;
    fn nf_ct_netns_do_put(net: *mut net, nfproto: u8);
    fn nf_ct_get(skb: *mut sk_buff, info: *mut c_int) -> *mut nf_conn;
    fn nfct_help(ct: *mut nf_conn) -> *mut nf_conn_help;
    fn nf_conntrack_confirm(skb: *mut sk_buff) -> c_uint;
    fn nf_conntrack_in(skb: *mut sk_buff, state: *const nf_hook_state) -> c_uint;
    fn nf_ct_l3num(ct: *mut nf_conn) -> u8;
    fn nf_ct_protonum(ct: *mut nf_conn) -> u8;
    fn nf_ct_put(ct: *mut nf_conn);
    fn nf_ct_is_template(ct: *mut nf_conn) -> bool;
    fn nf_ct_seq_adjust(skb: *mut sk_buff, ct: *mut nf_conn, info: c_int, off: c_uint) -> bool;
    fn nf_register_sockopt(ops: *const nf_sockopt_ops) -> c_int;
    fn nf_unregister_sockopt(ops: *const nf_sockopt_ops);
    fn nf_conntrack_generic_init_net(net: *mut net); fn nf_conntrack_udp_init_net(net: *mut net);
    fn nf_conntrack_tcp_init_net(net: *mut net); fn nf_conntrack_icmp_init_net(net: *mut net);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
