// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2008-2009 Pablo Neira Ayuso <pablo@netfilter.org>
 */
// C dependencies: linux/module.h, linux/skbuff.h, linux/jhash.h, linux/ip.h,
// net/ipv6.h, linux/netfilter/x_tables.h, net/netfilter/nf_conntrack.h,
// linux/netfilter/xt_cluster.h

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
unsafe extern "C" {
    fn jhash_1word(a: u32, initval: u32) -> u32;
    fn jhash2(k: *const u32, length: u32, initval: u32) -> u32;
    fn reciprocal_scale(val: u32, scale: u32) -> u32;
    fn nf_ct_l3num(ct: *const nf_conn) -> u16;
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *const nf_conn;
    fn nf_ct_is_template(ct: *const nf_conn) -> bool;
    fn xt_family(par: *const xt_action_param) -> u8;
    fn ipv4_is_multicast(addr: u32) -> bool;
    fn ipv6_addr_is_multicast(addr: *const in6_addr) -> bool;
    fn nf_ct_netns_get(net: *mut net, family: u8) -> i32;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
}

// Types and constants below are supplied by the kernel headers.
#[repr(C)] pub struct nf_conn { pub tuplehash: [nf_conntrack_tuple_hash; 2], pub master: *const nf_conn }
#[repr(C)] pub struct nf_conntrack_tuple_hash { pub tuple: nf_conntrack_tuple }
#[repr(C)] pub struct nf_conntrack_tuple { pub src: nf_conntrack_man_proto, }
#[repr(C)] pub struct nf_conntrack_man_proto { pub u3: nf_conntrack_address }
#[repr(C)] pub union nf_conntrack_address { pub ip: u32, pub ip6: [u32; 4] }
#[repr(C)] pub struct sk_buff { pub pkt_type: u8 }
#[repr(C)] pub struct xt_action_param { pub matchinfo: *const core::ffi::c_void }
#[repr(C)] pub struct xt_mtchk_param { pub matchinfo: *mut core::ffi::c_void, pub net: *mut net, pub family: u8 }
#[repr(C)] pub struct xt_mtdtor_param { pub net: *mut net, pub family: u8 }
#[repr(C)] pub struct xt_cluster_match_info { pub hash_seed: u32, pub total_nodes: u32, pub node_mask: u64, pub flags: u32 }
#[repr(C)] pub struct net;
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4] }
#[repr(C)] pub struct xt_match {
    pub name: *const core::ffi::c_char, pub family: u8,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub matchsize: usize,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub me: *mut core::ffi::c_void,
}
pub type ip_conntrack_info = i32;

const IP_CT_DIR_ORIGINAL: usize = 0;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const PACKET_MULTICAST: u8 =  multicast_packet_type;
const PACKET_HOST: u8 = host_packet_type;
const XT_CLUSTER_F_INV: u32 = 1;
const XT_CLUSTER_NODES_MAX: u32 = 32;
const NF_CT_TUPLE_L3SIZE: usize = 4;
const multicast_packet_type: u8 = 1;
const host_packet_type: u8 = 0;

#[inline]
unsafe fn nf_ct_orig_ipv4_src(ct: *const nf_conn) -> u32 {
    unsafe { (*(*ct).tuplehash.as_ptr().add(IP_CT_DIR_ORIGINAL)).tuple.src.u3.ip }
}

#[inline]
unsafe fn nf_ct_orig_ipv6_src(ct: *const nf_conn) -> *const u32 {
    unsafe { (*(*ct).tuplehash.as_ptr().add(IP_CT_DIR_ORIGINAL)).tuple.src.u3.ip6.as_ptr() }
}

#[inline]
unsafe fn xt_cluster_hash_ipv4(ip: u32, info: *const xt_cluster_match_info) -> u32 {
    unsafe { jhash_1word(ip, (*info).hash_seed) }
}

#[inline]
unsafe fn xt_cluster_hash_ipv6(ip: *const core::ffi::c_void, info: *const xt_cluster_match_info) -> u32 {
    unsafe { jhash2(ip as *const u32, (NF_CT_TUPLE_L3SIZE / core::mem::size_of::<u32>()) as u32, (*info).hash_seed) }
}

#[inline]
unsafe fn xt_cluster_hash(ct: *const nf_conn, info: *const xt_cluster_match_info) -> u32 {
    let mut hash = 0u32;
    unsafe { match nf_ct_l3num(ct) {
        AF_INET => hash = xt_cluster_hash_ipv4(nf_ct_orig_ipv4_src(ct), info),
        AF_INET6 => hash = xt_cluster_hash_ipv6(nf_ct_orig_ipv6_src(ct) as *const _, info),
        _ => {},
    }; reciprocal_scale(hash, (*info).total_nodes) }
}

#[inline]
unsafe fn xt_cluster_is_multicast_addr(skb: *const sk_buff, family: u8) -> bool {
    // Header accessors are supplied by the kernel networking headers.
    let mut is_multicast = false;
    unsafe { match family { NFPROTO_IPV4 => { is_multicast = ipv4_is_multicast((*skb).pkt_type as u32); }, NFPROTO_IPV6 => { is_multicast = false; }, _ => {} }; }
    is_multicast
}

unsafe extern "C" fn xt_cluster_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    unsafe {
        let pskb = skb as *mut sk_buff;
        let info = (*par).matchinfo as *const xt_cluster_match_info;
        let mut ctinfo = 0i32;
        let ct = nf_ct_get(skb, &mut ctinfo);
        if ct.is_null() || nf_ct_is_template(ct) { return false; }
        if !xt_cluster_is_multicast_addr(skb, xt_family(par)) && (*pskb).pkt_type == PACKET_MULTICAST { (*pskb).pkt_type = PACKET_HOST; }
        let hash = if !(*ct).master.is_null() { xt_cluster_hash((*ct).master, info) } else { xt_cluster_hash(ct, info) };
        (((1u64.wrapping_shl(hash)) & (*info).node_mask) != 0) ^ (((*info).flags & XT_CLUSTER_F_INV) != 0)
    }
}

unsafe extern "C" fn xt_cluster_mt_checkentry(par: *const xt_mtchk_param) -> i32 {
    unsafe { let info = (*par).matchinfo as *mut xt_cluster_match_info; if (*info).total_nodes > XT_CLUSTER_NODES_MAX { return -22; } if (*info).node_mask >= (1u64 << (*info).total_nodes) { return -33; } nf_ct_netns_get((*par).net, (*par).family) }
}
unsafe extern "C" fn xt_cluster_mt_destroy(par: *const xt_mtdtor_param) { unsafe { nf_ct_netns_put((*par).net, (*par).family); } }

static mut XT_CLUSTER_MATCH: [xt_match; 2] = [
    xt_match { name: b"cluster\0".as_ptr() as *const _, family: NFPROTO_IPV4, match_: Some(xt_cluster_mt), checkentry: Some(xt_cluster_mt_checkentry), matchsize: core::mem::size_of::<xt_cluster_match_info>(), destroy: Some(xt_cluster_mt_destroy), me: core::ptr::null_mut() },
    xt_match { name: b"cluster\0".as_ptr() as *const _, family: NFPROTO_IPV6, match_: Some(xt_cluster_mt), checkentry: Some(xt_cluster_mt_checkentry), matchsize: core::mem::size_of::<xt_cluster_match_info>(), destroy: Some(xt_cluster_mt_destroy), me: core::ptr::null_mut() },
];

unsafe extern "C" fn xt_cluster_mt_init() -> i32 {
    unsafe { xt_register_matches(XT_CLUSTER_MATCH.as_mut_ptr(), XT_CLUSTER_MATCH.len()) }
}

unsafe extern "C" fn xt_cluster_mt_fini() {
    unsafe { xt_unregister_matches(XT_CLUSTER_MATCH.as_mut_ptr(), XT_CLUSTER_MATCH.len()); }
}

// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Xtables: hash-based cluster match");
// MODULE_ALIAS("ipt_cluster");
// MODULE_ALIAS("ip6t_cluster");
// module_init(xt_cluster_mt_init);
// module_exit(xt_cluster_mt_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
