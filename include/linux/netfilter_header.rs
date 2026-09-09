/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[inline]
pub unsafe fn NF_DROP_GETERR(verdict: i32) -> i32 {
    -(verdict >> NF_VERDICT_QBITS)
}

#[inline(always)]
pub unsafe fn NF_DROP_REASON(skb: *mut sk_buff, reason: skb_drop_reason, err: u32) -> i32 {
    assert!(err <= 0xffff);
    kfree_skb_reason(skb, reason);
    ((err << 16) | NF_STOLEN) as i32
}

#[inline]
pub unsafe fn nf_inet_addr_cmp(a1: *const nf_inet_addr, a2: *const nf_inet_addr) -> i32 {
    (*a1).all[0] == (*a2).all[0] && (*a1).all[1] == (*a2).all[1]
        && (*a1).all[2] == (*a2).all[2] && (*a1).all[3] == (*a2).all[3]
} 

#[inline]
pub unsafe fn nf_inet_addr_mask(a1: *const nf_inet_addr, result: *mut nf_inet_addr,
                                mask: *const nf_inet_addr) {
    (*result).all[0] = (*a1).all[0] & (*mask).all[0];
    (*result).all[1] = (*a1).all[1] & (*mask).all[1];
    (*result).all[2] = (*a1).all[2] & (*mask).all[2];
    (*result).all[3] = (*a1).all[3] & (*mask).all[3];
}

extern "C" { pub fn netfilter_init() -> i32; }

#[repr(C)]
pub struct nf_hook_state {
    pub hook: u8,
    pub pf: u8,
    pub r#in: *mut net_device,
    pub out: *mut net_device,
    pub sk: *mut sock,
    pub net: *mut net,
    pub okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>,
}

pub type nf_hookfn = unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff, *const nf_hook_state) -> u32;

#[repr(C)]
pub enum nf_hook_ops_type { NF_HOOK_OP_UNDEFINED, NF_HOOK_OP_NF_TABLES, NF_HOOK_OP_BPF, NF_HOOK_OP_NFT_FT, NF_HOOK_OP_NAT }

#[repr(C)]
pub struct nf_hook_ops {
    pub list: list_head,
    pub rcu: rcu_head,
    pub hook: Option<nf_hookfn>,
    pub dev: *mut net_device,
    pub priv_: *mut core::ffi::c_void,
    pub pf: u8,
    pub hook_ops_type: u8,
    pub hooknum: u32,
    pub priority: i32,
}

#[repr(C)] pub struct nf_hook_entry { pub hook: Option<nf_hookfn>, pub priv_: *mut core::ffi::c_void }
#[repr(C)] pub struct nf_hook_entries_rcu_head { pub head: rcu_head, pub allocation: *mut core::ffi::c_void }
#[repr(C)] pub struct nf_hook_entries { pub num_hook_entries: u16, pub hooks: [nf_hook_entry; 0] }
#[repr(C)] pub struct nf_nat_lookup_hook_priv { pub entries: *mut nf_hook_entries, pub rcu_head: rcu_head }

#[cfg(CONFIG_NETFILTER)]
#[inline]
pub unsafe fn nf_hook_entries_get_hook_ops(e: *const nf_hook_entries) -> *mut *mut nf_hook_ops {
    (*e).hooks.as_ptr().add((*e).num_hook_entries as usize) as *mut *mut nf_hook_ops
}

#[cfg(CONFIG_NETFILTER)]
#[inline]
pub unsafe fn nf_hook_entry_hookfn(entry: *const nf_hook_entry, skb: *mut sk_buff, state: *mut nf_hook_state) -> i32 {
    ((*entry).hook.unwrap())((*entry).priv_, skb, state)
}

#[cfg(CONFIG_NETFILTER)]
#[inline]
pub unsafe fn nf_hook_state_init(p: *mut nf_hook_state, hook: u32, pf: u8, indev: *mut net_device,
                                  outdev: *mut net_device, sk: *mut sock, n: *mut net,
                                  okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) {
    (*p).hook = hook as u8; (*p).pf = pf; (*p).r#in = indev; (*p).out = outdev;
    (*p).sk = sk; (*p).net = n; (*p).okfn = okfn;
}

#[repr(C)]
pub struct nf_sockopt_ops {
    pub list: list_head, pub pf: u8, pub set_optmin: i32, pub set_optmax: i32,
    pub set: Option<unsafe extern "C" fn(*mut sock, i32, sockptr_t, u32) -> i32>,
    pub get_optmin: i32, pub get_optmax: i32,
    pub get: Option<unsafe extern "C" fn(*mut sock, i32, *mut core::ffi::c_void, *mut i32) -> i32>,
    pub owner: *mut module,
}

extern "C" {
    pub fn nf_register_net_hook(n: *mut net, ops: *const nf_hook_ops) -> i32;
    pub fn nf_unregister_net_hook(n: *mut net, ops: *const nf_hook_ops);
    pub fn nf_register_net_hooks(n: *mut net, reg: *const nf_hook_ops, count: u32) -> i32;
    pub fn nf_unregister_net_hooks(n: *mut net, reg: *const nf_hook_ops, count: u32);
    pub fn nf_register_sockopt(reg: *mut nf_sockopt_ops) -> i32;
    pub fn nf_unregister_sockopt(reg: *mut nf_sockopt_ops);
    pub fn nf_hook_slow(skb: *mut sk_buff, state: *mut nf_hook_state, e: *const nf_hook_entries, i: u32) -> i32;
    pub fn nf_hook_slow_list(head: *mut list_head, state: *mut nf_hook_state, e: *const nf_hook_entries);
    pub fn nf_setsockopt(sk: *mut sock, pf: u8, optval: i32, opt: sockptr_t, len: u32) -> i32;
    pub fn nf_getsockopt(sk: *mut sock, pf: u8, optval: i32, opt: *mut core::ffi::c_char, len: *mut i32) -> i32;
}

#[cfg(not(CONFIG_NETFILTER))]
#[inline] pub unsafe fn nf_hook(_pf: u8, _hook: u32, _net: *mut net, _sk: *mut sock, _skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, _okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) -> i32 { 1 }

#[cfg(CONFIG_NETFILTER)]
#[inline] pub unsafe fn NF_HOOK_COND(_pf: u8, _hook: u32, n: *mut net, sk: *mut sock, skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>, cond: bool) -> i32 { if !cond || nf_hook(_pf,_hook,n,sk,skb,_in,_out,okfn)==1 { okfn.unwrap()(n,sk,skb) } else { 0 } }

#[cfg(not(CONFIG_NETFILTER))]
#[inline] pub unsafe fn NF_HOOK_COND(_pf: u8, _hook: u32, n: *mut net, sk: *mut sock, skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>, _cond: bool) -> i32 { okfn.unwrap()(n,sk,skb) }

#[cfg(not(CONFIG_NETFILTER))]
#[inline] pub unsafe fn NF_HOOK(_pf: u8, _hook: u32, n: *mut net, sk: *mut sock, skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) -> i32 { okfn.unwrap()(n,sk,skb) }

#[cfg(not(CONFIG_NETFILTER))]
#[inline] pub unsafe fn NF_HOOK_LIST(_pf: u8, _hook: u32, _n: *mut net, _sk: *mut sock, _head: *mut list_head, _in: *mut net_device, _out: *mut net_device, _okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) {}

#[cfg(not(CONFIG_NETFILTER))]
#[inline] pub unsafe fn nf_nat_decode_session(_skb: *mut sk_buff, _fl: *mut flowi, _family: u8) {}

#[cfg(CONFIG_NETFILTER)]
#[inline] pub unsafe fn nf_hook(_pf: u8, _hook: u32, _net: *mut net, _sk: *mut sock, _skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, _okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) -> i32 { 1 /* hook dispatch is supplied by the kernel implementation */ }

#[cfg(CONFIG_NETFILTER)]
#[inline] pub unsafe fn NF_HOOK(_pf: u8, _hook: u32, n: *mut net, sk: *mut sock, skb: *mut sk_buff, _in: *mut net_device, _out: *mut net_device, okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) -> i32 { let ret = nf_hook(_pf,_hook,n,sk,skb,_in,_out,okfn); if ret == 1 { okfn.unwrap()(n,sk,skb) } else { ret } }

#[cfg(CONFIG_NETFILTER)]
#[inline] pub unsafe fn NF_HOOK_LIST(_pf: u8, _hook: u32, _n: *mut net, _sk: *mut sock, _head: *mut list_head, _in: *mut net_device, _out: *mut net_device, _okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) {}

#[cfg(CONFIG_NETFILTER)]
#[inline] pub unsafe fn nf_nat_decode_session(_skb: *mut sk_buff, _fl: *mut flowi, _family: u8) {}

#[cfg(IS_ENABLED_CONFIG_NF_CONNTRACK)]
extern "C" {
    pub fn nf_ct_attach(new: *mut sk_buff, old: *const sk_buff);
    pub fn nf_ct_set_closing(nfct: *mut nf_conntrack);
    pub fn nf_ct_get_tuple_skb(dst_tuple: *mut nf_conntrack_tuple, skb: *const sk_buff) -> bool;
}
#[cfg(not(IS_ENABLED_CONFIG_NF_CONNTRACK))]
#[inline] pub unsafe fn nf_ct_attach(_new: *mut sk_buff, _old: *mut sk_buff) {}
#[cfg(not(IS_ENABLED_CONFIG_NF_CONNTRACK))]
#[inline] pub unsafe fn nf_ct_set_closing(_nfct: *mut nf_conntrack) {}
#[cfg(not(IS_ENABLED_CONFIG_NF_CONNTRACK))]
#[inline] pub unsafe fn nf_ct_get_tuple_skb(_dst_tuple: *mut nf_conntrack_tuple, _skb: *const sk_buff) -> bool { false }

extern "C" {
    pub fn nf_checksum(skb: *mut sk_buff, hook: u32, dataoff: u32, protocol: u8, family: u16) -> __sum16;
    pub fn nf_checksum_partial(skb: *mut sk_buff, hook: u32, dataoff: u32, len: u32, protocol: u8, family: u16) -> __sum16;
    pub fn nf_route(n: *mut net, dst: *mut *mut dst_entry, fl: *mut flowi, strict: bool, family: u16) -> i32;
}

#[repr(C)] pub struct nf_conntrack_tuple;
#[repr(C)] pub struct nf_conntrack;
#[repr(C)] pub struct nf_conn;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct flowi;
#[repr(C)] pub struct dst_entry;
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct net_device;
#[repr(C)] pub struct sock;
#[repr(C)] pub struct net;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct module;
#[repr(C)] pub struct sockptr_t;
#[repr(C)] pub struct nf_inet_addr { pub all: [u32; 4] }
#[repr(C)] pub struct skb_drop_reason;
#[repr(C)] pub enum nf_nat_manip_type {}
#[repr(C)] pub enum ip_conntrack_info {}
pub type __sum16 = u16;
extern "C" { pub fn kfree_skb_reason(skb: *mut sk_buff, reason: skb_drop_reason); }
extern "C" { static NF_VERDICT_QBITS: i32; static NF_STOLEN: u32; }

pub struct nf_nat_hook {
    pub parse_nat_setup: Option<unsafe extern "C" fn(*mut nf_conn, nf_nat_manip_type, *const nlattr) -> i32>,
    pub decode_session: Option<unsafe extern "C" fn(*mut sk_buff, *mut flowi)>,
    pub remove_nat_bysrc: Option<unsafe extern "C" fn(*mut nf_conn)>,
}
extern "C" { pub static mut nf_nat_hook: *const nf_nat_hook; }

pub struct nf_ct_hook {
    pub update: Option<unsafe extern "C" fn(*mut net, *mut sk_buff) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut nf_conntrack)>,
    pub get_tuple_skb: Option<unsafe extern "C" fn(*mut nf_conntrack_tuple, *const sk_buff) -> bool>,
    pub attach: Option<unsafe extern "C" fn(*mut sk_buff, *const sk_buff)>,
    pub set_closing: Option<unsafe extern "C" fn(*mut nf_conntrack)>,
    pub confirm: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub get_id: Option<unsafe extern "C" fn(*const nf_conntrack) -> u32>,
}
extern "C" { pub static mut nf_ct_hook: *const nf_ct_hook; }

pub struct nfnl_ct_hook {
    pub build_size: Option<unsafe extern "C" fn(*const nf_conn) -> usize>,
    pub build: Option<unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, u16, u16) -> i32>,
    pub parse: Option<unsafe extern "C" fn(*const nlattr, *mut nf_conn) -> i32>,
    pub attach_expect: Option<unsafe extern "C" fn(*const nlattr, *mut nf_conn, u32, u32) -> i32>,
    pub seq_adjust: Option<unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, i32)>,
}
extern "C" { pub static mut nfnl_ct_hook: *const nfnl_ct_hook; }

#[repr(C)] pub struct nf_defrag_hook { pub owner: *mut module, pub enable: Option<unsafe extern "C" fn(*mut net) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut net)> }
extern "C" { pub static mut nf_defrag_v4_hook: *const nf_defrag_hook; pub static mut nf_defrag_v6_hook: *const nf_defrag_hook; pub static mut nf_ctnetlink_has_listener: u8; }

/* External declarations and types are supplied by the included kernel headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
