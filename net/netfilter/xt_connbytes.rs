// SPDX-License-Identifier: GPL-2.0
/* Kernel module to match connection tracking byte counter.
 *  (C) 2002 Martin Devera (devik@cdi.cz).
 */

// C dependencies supplied by the surrounding kernel/Rust bindings.

const MODULE_LICENSE: &str = "GPL";
const MODULE_AUTHOR: &str = "Harald Welte <laforge@netfilter.org>";
const MODULE_DESCRIPTION: &str = "Xtables: Number of packets/bytes per connection matching";
const MODULE_ALIAS_IPT_CONNBYTES: &str = "ipt_connbytes";
const MODULE_ALIAS_IP6T_CONNBYTES: &str = "ip6t_connbytes";

extern "C" {
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *const nf_conn;
    fn nf_conn_acct_find(ct: *const nf_conn) -> *const nf_conn_acct;
    fn atomic64_read(v: *const atomic64_t) -> u64;
    fn div64_u64(dividend: u64, divisor: u64) -> u64;
    fn nf_ct_netns_get(net: *mut net, family: u8) -> i32;
    fn nf_ct_acct_enabled(net: *mut net) -> bool;
    fn nf_ct_set_acct(net: *mut net, enabled: bool);
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct nf_conn;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct atomic64_t;
#[repr(C)]
pub struct xt_connbytes_info {
    pub what: u32,
    pub direction: u32,
    pub count: xt_connbytes_count,
}
#[repr(C)]
pub struct xt_connbytes_count {
    pub from: u64,
    pub to: u64,
}
#[repr(C)]
pub struct nf_conn_counter {
    pub packets: atomic64_t,
    pub bytes: atomic64_t,
}
#[repr(C)]
pub struct nf_conn_acct {
    pub counter: *const nf_conn_counter,
}
#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const core::ffi::c_void,
}
#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const core::ffi::c_void,
    pub net: *mut net,
    pub family: u8,
}
#[repr(C)]
pub struct xt_mtdtor_param {
    pub net: *mut net,
    pub family: u8,
}
#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub me: *mut core::ffi::c_void,
}

pub type ip_conntrack_info = u32;

const XT_CONNBYTES_PKTS: u32 = 0;
const XT_CONNBYTES_BYTES: u32 = 1;
const XT_CONNBYTES_AVGPKT: u32 = 2;
const XT_CONNBYTES_DIR_ORIGINAL: u32 = 0;
const XT_CONNBYTES_DIR_REPLY: u32 = 1;
const XT_CONNBYTES_DIR_BOTH: u32 = 2;
const IP_CT_DIR_ORIGINAL: usize = 0;
const IP_CT_DIR_REPLY: usize = 1;
const NFPROTO_UNSPEC: u16 = 0;

unsafe extern "C" fn connbytes_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let sinfo = (*par).matchinfo as *const xt_connbytes_info;
    let mut ctinfo: ip_conntrack_info = 0;
    let ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() { return false; }
    let acct = nf_conn_acct_find(ct);
    if acct.is_null() { return false; }
    let counters = (*acct).counter;
    let mut what: u64 = 0;
    let mut bytes: u64 = 0;
    let mut pkts: u64 = 0;
    match (*sinfo).what {
        XT_CONNBYTES_PKTS => match (*sinfo).direction {
            XT_CONNBYTES_DIR_ORIGINAL => what = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).packets),
            XT_CONNBYTES_DIR_REPLY => what = atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).packets),
            XT_CONNBYTES_DIR_BOTH => { what = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).packets); what = what.wrapping_add(atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).packets)); }
            _ => {}
        },
        XT_CONNBYTES_BYTES => match (*sinfo).direction {
            XT_CONNBYTES_DIR_ORIGINAL => what = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).bytes),
            XT_CONNBYTES_DIR_REPLY => what = atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).bytes),
            XT_CONNBYTES_DIR_BOTH => { what = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).bytes); what = what.wrapping_add(atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).bytes)); }
            _ => {}
        },
        XT_CONNBYTES_AVGPKT => { match (*sinfo).direction {
            XT_CONNBYTES_DIR_ORIGINAL => { bytes = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).bytes); pkts = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).packets); }
            XT_CONNBYTES_DIR_REPLY => { bytes = atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).bytes); pkts = atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).packets); }
            XT_CONNBYTES_DIR_BOTH => { bytes = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).bytes).wrapping_add(atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).bytes)); pkts = atomic64_read(&(*counters.add(IP_CT_DIR_ORIGINAL)).packets).wrapping_add(atomic64_read(&(*counters.add(IP_CT_DIR_REPLY)).packets)); }
            _ => {}
        } if pkts != 0 { what = div64_u64(bytes, pkts); } },
        _ => {}
    }
    if (*sinfo).count.to >= (*sinfo).count.from { what <= (*sinfo).count.to && what >= (*sinfo).count.from } else { what < (*sinfo).count.to || what > (*sinfo).count.from }
}

unsafe extern "C" fn connbytes_mt_check(par: *const xt_mtchk_param) -> i32 {
    let sinfo = (*par).matchinfo as *const xt_connbytes_info;
    if (*sinfo).what != XT_CONNBYTES_PKTS && (*sinfo).what != XT_CONNBYTES_BYTES && (*sinfo).what != XT_CONNBYTES_AVGPKT { return -22; }
    if (*sinfo).direction != XT_CONNBYTES_DIR_ORIGINAL && (*sinfo).direction != XT_CONNBYTES_DIR_REPLY && (*sinfo).direction != XT_CONNBYTES_DIR_BOTH { return -22; }
    let ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 { return ret; }
    if !nf_ct_acct_enabled((*par).net) { nf_ct_set_acct((*par).net, true); }
    ret
}

unsafe extern "C" fn connbytes_mt_destroy(par: *const xt_mtdtor_param) { nf_ct_netns_put((*par).net, (*par).family); }

static mut CONNBYTES_MT_REG: xt_match = xt_match {
    name: b"connbytes\0".as_ptr(), revision: 0, family: NFPROTO_UNSPEC,
    checkentry: Some(connbytes_mt_check), r#match: Some(connbytes_mt), destroy: Some(connbytes_mt_destroy),
    matchsize: core::mem::size_of::<xt_connbytes_info>(), me: core::ptr::null_mut(),
};

pub unsafe extern "C" fn connbytes_mt_init() -> i32 { xt_register_match(&mut CONNBYTES_MT_REG) }
pub unsafe extern "C" fn connbytes_mt_exit() { xt_unregister_match(&mut CONNBYTES_MT_REG); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
