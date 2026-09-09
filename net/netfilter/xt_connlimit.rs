// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * netfilter module to limit the number of parallel tcp
 * connections per IP address.
 *   (c) 2000 Gerd Knorr <kraxel@bytesex.org>
 *   Nov 2002: Martin Bene <martin.bene@icomedias.com>:
 *              only ignore TIME_WAIT or gone connections
 *   (C) CC Computer Consultants GmbH, 2007
 *
 * based on ...
 *
 * Kernel module to match connection tracking information.
 *   (C) 1999  Rusty Russell (rusty@rustcorp.com.au).
 */
// Dependency includes from the original C source are supplied by other files.
// Build-time CONFIG_IP6_NF_IPTABLES condition is preserved below.

unsafe extern "C" {
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *const nf_conn;
    fn nf_ct_zone(ct: *const nf_conn) -> *const nf_conntrack_zone;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn xt_family(par: *const xt_action_param) -> u8;
    fn nf_conncount_count_skb(
        net: *mut net,
        skb: *const sk_buff,
        family: u8,
        data: *mut core::ffi::c_void,
        key: *const u32,
    ) -> u32;
    fn nf_ct_netns_get(net: *mut net, family: u8) -> i32;
    fn nf_conncount_init(net: *mut net, keylen: usize) -> *mut core::ffi::c_void;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn nf_conncount_destroy(net: *mut net, data: *mut core::ffi::c_void);
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
}

extern "C" {
    static nf_ct_zone_dflt: nf_conntrack_zone;
}

#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct nf_conn { _private: [u8; 0] }
#[repr(C)]
pub struct ipv6hdr { pub saddr: [u32; 4], pub daddr: [u32; 4] }
#[repr(C)]
pub struct iphdr { pub saddr: u32, pub daddr: u32 }
#[repr(C)]
pub struct nf_conntrack_zone { pub id: u32 }
#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *mut core::ffi::c_void,
    pub hotdrop: bool,
}
#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *mut core::ffi::c_void,
    pub net: *mut net,
    pub family: u8,
}
#[repr(C)]
pub struct xt_mtdtor_param {
    pub matchinfo: *mut core::ffi::c_void,
    pub net: *mut net,
    pub family: u8,
}
#[repr(C)]
pub struct xt_connlimit_info {
    pub limit: u32,
    pub flags: u32,
    pub mask: nf_connlimit_mask,
    pub data: *mut core::ffi::c_void,
}
#[repr(C)]
pub union nf_connlimit_mask { pub ip: u32, pub ip6: [u32; 4] }
#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u8,
    pub checkentry: unsafe extern "C" fn(*const xt_mtchk_param) -> i32,
    pub r#match: unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool,
    pub matchsize: usize,
    pub usersize: usize,
    pub destroy: unsafe extern "C" fn(*const xt_mtdtor_param),
    pub me: *mut core::ffi::c_void,
}
pub type ip_conntrack_info = u32;

const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const XT_CONNLIMIT_DADDR: u32 = 1;
const XT_CONNLIMIT_INVERT: u32 = 2;

unsafe extern "C" fn connlimit_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let net = xt_net(par);
    let info = (*par).matchinfo as *const xt_connlimit_info;
    let mut zone = &nf_ct_zone_dflt as *const nf_conntrack_zone;
    let mut ctinfo: ip_conntrack_info = 0;
    let ct = nf_ct_get(skb, &mut ctinfo);
    if !ct.is_null() { zone = nf_ct_zone(ct); }
    let mut key = [0u32; 5];
    if xt_family(par) == NFPROTO_IPV6 {
        let iph = ipv6_hdr(skb);
        let mut addr: [u32; 4] = if ((*info).flags & XT_CONNLIMIT_DADDR) != 0 {
            (*iph).daddr
        } else { (*iph).saddr };
        for i in 0..addr.len() {
            addr[i] &= (*info).mask.ip6[i];
        }
        key[..4].copy_from_slice(&addr);
        key[4] = (*zone).id;
    } else {
        let iph = ip_hdr(skb);
        key[0] = if ((*info).flags & XT_CONNLIMIT_DADDR) != 0 { (*iph).daddr } else { (*iph).saddr };
        key[0] &= (*info).mask.ip;
        key[1] = (*zone).id;
    }
    let connections = nf_conncount_count_skb(net, skb, xt_family(par), (*info).data, key.as_ptr());
    if connections == 0 {
        (*par).hotdrop = true;
        return false;
    }
    (connections > (*info).limit) ^ (((*info).flags & XT_CONNLIMIT_INVERT) != 0)
}

unsafe extern "C" fn connlimit_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_connlimit_info;
    let mut keylen = core::mem::size_of::<u32>();
    keylen += if (*par).family == NFPROTO_IPV6 { 16 } else { 4 };
    let ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 { return ret; }
    (*info).data = nf_conncount_init((*par).net, keylen);
    if (*info).data as isize == -1 { nf_ct_netns_put((*par).net, (*par).family); }
    if (*info).data as isize == -1 { -1 } else { 0 }
}

unsafe extern "C" fn connlimit_mt_destroy(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *const xt_connlimit_info;
    nf_conncount_destroy((*par).net, (*info).data);
    nf_ct_netns_put((*par).net, (*par).family);
}

static mut connlimit_mt_reg: [xt_match; 2] = [
    xt_match { name: b"connlimit\0".as_ptr(), revision: 1, family: NFPROTO_IPV4,
        checkentry: connlimit_mt_check, r#match: connlimit_mt, matchsize: core::mem::size_of::<xt_connlimit_info>(),
        usersize: core::mem::offset_of!(xt_connlimit_info, data), destroy: connlimit_mt_destroy, me: core::ptr::null_mut() },
    xt_match { name: b"connlimit\0".as_ptr(), revision: 1, family: NFPROTO_IPV6,
        checkentry: connlimit_mt_check, r#match: connlimit_mt, matchsize: core::mem::size_of::<xt_connlimit_info>(),
        usersize: core::mem::offset_of!(xt_connlimit_info, data), destroy: connlimit_mt_destroy, me: core::ptr::null_mut() },
];

unsafe extern "C" fn connlimit_mt_init() -> i32 {
    xt_register_matches(connlimit_mt_reg.as_mut_ptr(), connlimit_mt_reg.len())
}
unsafe extern "C" fn connlimit_mt_exit() {
    xt_unregister_matches(connlimit_mt_reg.as_mut_ptr(), connlimit_mt_reg.len());
}

// module_init(connlimit_mt_init); module_exit(connlimit_mt_exit);
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("Xtables: Number of connections matching");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_connlimit");
// MODULE_ALIAS("ip6t_connlimit");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
