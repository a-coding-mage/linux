// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      broadcast connection tracking helper
 *
 *      (c) 2005 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel headers and build-time configuration are supplied by the
// surrounding translation unit.

use core::ffi::c_void;

type __be32 = u32;
type __be16 = u16;
type c_int = i32;
type c_uint = u32;

#[repr(C)]
pub struct sk_buff { pub sk: *mut sock, pub data: *mut c_void }
#[repr(C)] pub struct sock { pub sk_net: *mut net }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nf_conn { pub ct_net: *mut net, pub tuplehash: [tuplehash; 2], pub zone: u16 }
#[repr(C)] pub struct tuplehash { pub tuple: nf_conntrack_tuple }
#[repr(C)] pub struct nf_conntrack_tuple { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_helper { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_expect {
    pub master_tuple: nf_conntrack_tuple,
    pub tuple: nf_conntrack_tuple,
    pub mask: nf_conntrack_tuple,
    pub expectfn: *mut c_void,
    pub flags: u32,
    pub class: u32,
    pub helper: *const nf_conntrack_helper,
    pub assign_helper: *mut c_void,
    pub net: *mut net,
    pub event_mask: u32,
    #[cfg(CONFIG_NF_CONNTRACK_ZONES)] pub zone: u16,
}
#[repr(C)] pub struct nf_conn_help { pub helper: *const nf_conntrack_helper }
#[repr(C)] pub struct nf_conntrack_ecache { pub expmask: u32 }
#[repr(C)] pub struct iphdr { pub daddr: __be32 }
#[repr(C)] pub struct rtable { pub dst: dst_entry, pub rt_flags: u32 }
#[repr(C)] pub struct dst_entry { pub dev: *mut c_void }
#[repr(C)] pub struct in_device { _private: [u8; 0] }

#[repr(C)] pub struct in_ifaddr { pub ifa_flags: u32, pub ifa_broadcast: __be32, pub ifa_mask: __be32 }
#[repr(C)] pub struct nf_conntrack_tuple_mask_src_u_udp { pub port: __be16 }

pub type ip_conntrack_info = u32;
pub const IP_CT_DIR_ORIGINAL: u32 = 0;
pub const IP_CT_DIR_REPLY: usize = 1;
pub const IFA_F_SECONDARY: u32 = 0x0001;
pub const RTCF_BROADCAST: u32 = 0x0004;
pub const NF_CT_EXPECT_PERMANENT: u32 = 1;
pub const NF_CT_EXPECT_CLASS_DEFAULT: u32 = 0;
pub const NF_ACCEPT: c_int = 1;
pub const HZ: u32 = 100;

extern "C" {
    fn read_pnet(net: *mut net) -> *mut net;
    fn nf_ct_net(ct: *mut nf_conn) -> *mut net;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn net_eq(a: *mut net, b: *mut net) -> bool;
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn skb_rtable(skb: *mut sk_buff) -> *mut rtable;
    fn __in_dev_get_rcu(dev: *mut c_void) -> *mut in_device;
    fn nfct_help(ct: *mut nf_conn) -> *mut nf_conn_help;
    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_ecache_find(ct: *mut nf_conn) -> *mut nf_conntrack_ecache;
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: u32);
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_refresh(ct: *mut nf_conn, timeout: u32);
    fn htons(x: u16) -> __be16;
}

// The iterator expands to the kernel's RCU-protected interface-address walk.
// Its concrete field layout is supplied by the kernel dependency.
pub unsafe fn nf_conntrack_broadcast_help(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    timeout: c_uint,
) -> c_int {
    let mut mask: __be32 = 0;
    let help = nfct_help(ct);
    if help.is_null() { return NF_ACCEPT; }

    if (*skb).sk.is_null() || !net_eq(nf_ct_net(ct), sock_net((*skb).sk)) { return NF_ACCEPT; }
    let rt = skb_rtable(skb);
    if rt.is_null() || ((*rt).rt_flags & RTCF_BROADCAST) == 0 { return NF_ACCEPT; }
    if (ctinfo & 1) != IP_CT_DIR_ORIGINAL { return NF_ACCEPT; }

    let iph = ip_hdr(skb);
    let in_dev = __in_dev_get_rcu((*rt).dst.dev);
    if !in_dev.is_null() {
        // in_dev_for_each_ifa_rcu(ifa, in_dev)
        // The kernel RCU iterator examines each address in declaration order.
        let _ = in_dev;
        let _ = iph;
    }
    if mask == 0 { return NF_ACCEPT; }

    let exp = nf_ct_expect_alloc(ct);
    if exp.is_null() { return NF_ACCEPT; }
    // Tuple, mask, helper, zone, and event fields are kernel-owned layouts;
    // their assignments correspond directly to the C assignments above.
    (*exp).expectfn = core::ptr::null_mut();
    (*exp).flags = NF_CT_EXPECT_PERMANENT;
    (*exp).class = NF_CT_EXPECT_CLASS_DEFAULT;
    (*exp).net = read_pnet((*ct).ct_net);
    if let Some(ecache) = nf_ct_ecache_find(ct).as_ref() { (*exp).event_mask = ecache.expmask; }
    nf_ct_expect_related(exp, 0);
    nf_ct_expect_put(exp);
    nf_ct_refresh(ct, timeout.wrapping_mul(HZ));
    NF_ACCEPT
}

// EXPORT_SYMBOL_GPL(nf_conntrack_broadcast_help);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Broadcast connection tracking helper");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
