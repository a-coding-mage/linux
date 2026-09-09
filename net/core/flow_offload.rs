/* SPDX-License-Identifier: GPL-2.0 */
/* Kernel headers and externally supplied types/functions are dependencies of this translation. */

use core::ffi::c_void;

extern "C" {
    fn kzalloc_flex<T>(n: usize) -> *mut T;
    fn kzalloc_obj<T>() -> *mut T;
    fn kmalloc(size: usize, gfp: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize);
    fn skb_flow_dissector_target(d: *mut flow_dissector, ty: u32, p: *mut c_void) -> *mut c_void;
    fn flow_block_cb_add(cb: *mut flow_block_cb, f: *mut flow_block_offload);
    fn flow_block_cb_remove(cb: *mut flow_block_cb, f: *mut flow_block_offload);
    fn tcf_action_reoffload_cb(cb: flow_indr_block_bind_cb_t, priv_: *mut c_void, add: bool);
}

#[repr(C)] pub struct flow_rule { pub action: flow_action, pub match_: flow_match }
#[repr(C)] pub struct flow_offload_action { pub action: flow_action }
#[repr(C)] pub struct flow_action { pub num_entries: u32, pub entries: *mut flow_action_entry }
#[repr(C)] pub struct flow_action_entry { pub hw_stats: u32 }
#[repr(C)] pub struct flow_match { pub dissector: *mut flow_dissector, pub key: *mut c_void, pub mask: *mut c_void }
#[repr(C)] pub struct flow_dissector;
#[repr(C)] pub struct flow_action_cookie { pub cookie_len: u32, pub cookie: [u8; 0] }
#[repr(C)] pub struct flow_block_cb { pub cb: flow_setup_cb_t, pub cb_ident: *mut c_void, pub cb_priv: *mut c_void, pub release: Option<unsafe extern "C" fn(*mut c_void)>, pub refcnt: u32, pub list: list_head, pub driver_list: list_head, pub indr: flow_block_cb_indr }
#[repr(C)] pub struct flow_block_cb_indr { pub list: list_head, pub binder_type: u32, pub data: *mut c_void, pub cb_priv: *mut c_void, pub dev: *mut net_device, pub sch: *mut Qdisc, pub cleanup: Option<unsafe extern "C" fn(*mut flow_block_cb)> }
#[repr(C)] pub struct flow_block { pub cb_list: list_head }
#[repr(C)] pub struct flow_block_offload { pub command: u32, pub binder_type: u32, pub driver_block_list: *mut list_head, pub block: *mut flow_block, pub cb_list: *mut list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct net_device; #[repr(C)] pub struct Qdisc;
#[repr(C)] pub struct flow_indr_dev { pub list: list_head, pub cb: flow_indr_block_bind_cb_t, pub cb_priv: *mut c_void, pub refcnt: u32 }
#[repr(C)] pub struct flow_indir_dev_info { pub data: *mut c_void, pub dev: *mut net_device, pub sch: *mut Qdisc, pub type_: u32, pub cleanup: Option<unsafe extern "C" fn(*mut flow_block_cb)>, pub list: list_head, pub command: u32, pub binder_type: u32, pub cb_list: *mut list_head }

pub type flow_setup_cb_t = Option<unsafe extern "C" fn(*mut c_void)>;
pub type flow_indr_block_bind_cb_t = unsafe extern "C" fn(*mut net_device, *mut Qdisc, *mut c_void, u32, *mut flow_block_offload, *mut c_void, Option<unsafe extern "C" fn(*mut flow_block_cb)> ) -> i32;

pub const FLOW_ACTION_HW_STATS_DONT_CARE: u32 = 0;
pub const FLOW_BLOCK_BIND: u32 = 0; pub const FLOW_BLOCK_UNBIND: u32 = 1;
pub const FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS: u32 = 0;
pub const EINVAL: i32 = 22; pub const ENOMEM: i32 = 12; pub const EOPNOTSUPP: i32 = 95; pub const EBUSY: i32 = 16; pub const ENOENT: i32 = 2; pub const EEXIST: i32 = 17;

macro_rules! flow_dissector_match { ($rule:expr, $ty:expr, $out:expr) => {{ let m = &(*$rule).match_; (*$out).key = skb_flow_dissector_target((*m).dissector, $ty, (*m).key); (*$out).mask = skb_flow_dissector_target((*m).dissector, $ty, (*m).mask); }} }

pub unsafe fn flow_rule_alloc(num_actions: u32) -> *mut flow_rule { let rule = kzalloc_flex::<flow_rule>(num_actions as usize); if rule.is_null() { return core::ptr::null_mut(); } (*rule).action.num_entries = num_actions; for i in 0..num_actions { (*(*rule).action.entries.add(i as usize)).hw_stats = FLOW_ACTION_HW_STATS_DONT_CARE; } rule }
pub unsafe fn offload_action_alloc(num_actions: u32) -> *mut flow_offload_action { let a = kzalloc_flex::<flow_offload_action>(num_actions as usize); if a.is_null() { return core::ptr::null_mut(); } (*a).action.num_entries = num_actions; for i in 0..num_actions { (*(*a).action.entries.add(i as usize)).hw_stats = FLOW_ACTION_HW_STATS_DONT_CARE; } a }

macro_rules! match_fn { ($name:ident, $out:ty, $ty:expr) => { pub unsafe fn $name(rule: *const flow_rule, out: *mut $out) { flow_dissector_match!(rule as *mut flow_rule, $ty, out); } }; }
/* Dissector key constants are provided by the kernel dependency. */
extern "C" { static FLOW_DISSECTOR_KEY_META: u32; static FLOW_DISSECTOR_KEY_BASIC: u32; static FLOW_DISSECTOR_KEY_CONTROL: u32; static FLOW_DISSECTOR_KEY_ETH_ADDRS: u32; static FLOW_DISSECTOR_KEY_VLAN: u32; static FLOW_DISSECTOR_KEY_CVLAN: u32; static FLOW_DISSECTOR_KEY_ARP: u32; static FLOW_DISSECTOR_KEY_IPV4_ADDRS: u32; static FLOW_DISSECTOR_KEY_IPV6_ADDRS: u32; static FLOW_DISSECTOR_KEY_IP: u32; static FLOW_DISSECTOR_KEY_PORTS: u32; static FLOW_DISSECTOR_KEY_PORTS_RANGE: u32; static FLOW_DISSECTOR_KEY_TCP: u32; static FLOW_DISSECTOR_KEY_IPSEC: u32; static FLOW_DISSECTOR_KEY_ICMP: u32; static FLOW_DISSECTOR_KEY_MPLS: u32; static FLOW_DISSECTOR_KEY_ENC_CONTROL: u32; static FLOW_DISSECTOR_KEY_ENC_IPV4_ADDRS: u32; static FLOW_DISSECTOR_KEY_ENC_IPV6_ADDRS: u32; static FLOW_DISSECTOR_KEY_ENC_IP: u32; static FLOW_DISSECTOR_KEY_ENC_PORTS: u32; static FLOW_DISSECTOR_KEY_ENC_KEYID: u32; static FLOW_DISSECTOR_KEY_ENC_OPTS: u32; static FLOW_DISSECTOR_KEY_CT: u32; static FLOW_DISSECTOR_KEY_PPPOE: u32; static FLOW_DISSECTOR_KEY_L2TPV3: u32; }
#[repr(C)] pub struct flow_match_meta { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_basic { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_control { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_eth_addrs { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_vlan { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_arp { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ipv4_addrs { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ipv6_addrs { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ip { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ports { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ports_range { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_tcp { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ipsec { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_icmp { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_mpls { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_enc_keyid { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_enc_opts { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_ct { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_pppoe { pub key:*mut c_void,pub mask:*mut c_void } #[repr(C)] pub struct flow_match_l2tpv3 { pub key:*mut c_void,pub mask:*mut c_void }
match_fn!(flow_rule_match_meta, flow_match_meta, FLOW_DISSECTOR_KEY_META); match_fn!(flow_rule_match_basic, flow_match_basic, FLOW_DISSECTOR_KEY_BASIC); match_fn!(flow_rule_match_control, flow_match_control, FLOW_DISSECTOR_KEY_CONTROL); match_fn!(flow_rule_match_eth_addrs, flow_match_eth_addrs, FLOW_DISSECTOR_KEY_ETH_ADDRS); match_fn!(flow_rule_match_vlan, flow_match_vlan, FLOW_DISSECTOR_KEY_VLAN); match_fn!(flow_rule_match_cvlan, flow_match_vlan, FLOW_DISSECTOR_KEY_CVLAN); match_fn!(flow_rule_match_arp, flow_match_arp, FLOW_DISSECTOR_KEY_ARP); match_fn!(flow_rule_match_ipv4_addrs, flow_match_ipv4_addrs, FLOW_DISSECTOR_KEY_IPV4_ADDRS); match_fn!(flow_rule_match_ipv6_addrs, flow_match_ipv6_addrs, FLOW_DISSECTOR_KEY_IPV6_ADDRS); match_fn!(flow_rule_match_ip, flow_match_ip, FLOW_DISSECTOR_KEY_IP); match_fn!(flow_rule_match_ports, flow_match_ports, FLOW_DISSECTOR_KEY_PORTS); match_fn!(flow_rule_match_ports_range, flow_match_ports_range, FLOW_DISSECTOR_KEY_PORTS_RANGE); match_fn!(flow_rule_match_tcp, flow_match_tcp, FLOW_DISSECTOR_KEY_TCP); match_fn!(flow_rule_match_ipsec, flow_match_ipsec, FLOW_DISSECTOR_KEY_IPSEC); match_fn!(flow_rule_match_icmp, flow_match_icmp, FLOW_DISSECTOR_KEY_ICMP); match_fn!(flow_rule_match_mpls, flow_match_mpls, FLOW_DISSECTOR_KEY_MPLS); match_fn!(flow_rule_match_enc_control, flow_match_control, FLOW_DISSECTOR_KEY_ENC_CONTROL); match_fn!(flow_rule_match_enc_ipv4_addrs, flow_match_ipv4_addrs, FLOW_DISSECTOR_KEY_ENC_IPV4_ADDRS); match_fn!(flow_rule_match_enc_ipv6_addrs, flow_match_ipv6_addrs, FLOW_DISSECTOR_KEY_ENC_IPV6_ADDRS); match_fn!(flow_rule_match_enc_ip, flow_match_ip, FLOW_DISSECTOR_KEY_ENC_IP); match_fn!(flow_rule_match_enc_ports, flow_match_ports, FLOW_DISSECTOR_KEY_ENC_PORTS); match_fn!(flow_rule_match_enc_keyid, flow_match_enc_keyid, FLOW_DISSECTOR_KEY_ENC_KEYID); match_fn!(flow_rule_match_enc_opts, flow_match_enc_opts, FLOW_DISSECTOR_KEY_ENC_OPTS); match_fn!(flow_rule_match_ct, flow_match_ct, FLOW_DISSECTOR_KEY_CT); match_fn!(flow_rule_match_pppoe, flow_match_pppoe, FLOW_DISSECTOR_KEY_PPPOE); match_fn!(flow_rule_match_l2tpv3, flow_match_l2tpv3, FLOW_DISSECTOR_KEY_L2TPV3);

pub unsafe fn flow_action_cookie_create(data:*mut c_void,len:u32,gfp:u32)->*mut flow_action_cookie { let c=kmalloc(core::mem::size_of::<flow_action_cookie>()+len as usize,gfp) as *mut flow_action_cookie; if c.is_null(){return core::ptr::null_mut()} (*c).cookie_len=len; memcpy((*c).cookie.as_mut_ptr() as *mut c_void,data,len as usize); c }
pub unsafe fn flow_action_cookie_destroy(c:*mut flow_action_cookie){ kfree(c as *mut c_void); }
pub unsafe fn flow_block_cb_priv(c:*mut flow_block_cb)->*mut c_void{(*c).cb_priv}
pub unsafe fn flow_block_cb_incref(c:*mut flow_block_cb){(*c).refcnt+=1}
pub unsafe fn flow_block_cb_decref(c:*mut flow_block_cb)->u32{(*c).refcnt-=1;(*c).refcnt}
pub unsafe fn flow_block_cb_alloc(cb:flow_setup_cb_t,id:*mut c_void,priv_:*mut c_void,release:Option<unsafe extern "C" fn(*mut c_void)>)->*mut flow_block_cb { let p=kzalloc_obj::<flow_block_cb>(); if p.is_null(){return (-ENOMEM) as isize as *mut flow_block_cb} (*p).cb=cb;(*p).cb_ident=id;(*p).cb_priv=priv_;(*p).release=release;p }
pub unsafe fn flow_block_cb_free(p:*mut flow_block_cb){if let Some(f)=(*p).release{f((*p).cb_priv)};kfree(p as *mut c_void)}
pub unsafe fn flow_block_cb_lookup(block:*mut flow_block,cb:flow_setup_cb_t,id:*mut c_void)->*mut flow_block_cb { let mut p=(*block).cb_list.next as *mut flow_block_cb; while p as *mut list_head != &mut (*block).cb_list {if (*p).cb==cb&&(*p).cb_ident==id{return p} p=(*p).list.next as *mut flow_block_cb;} core::ptr::null_mut() }
pub unsafe fn flow_block_cb_is_busy(cb:flow_setup_cb_t,id:*mut c_void,list:*mut list_head)->bool { let mut p=(*list).next as *mut flow_block_cb; while p as *mut list_head!=list {if (*p).cb==cb&&(*p).cb_ident==id{return true}p=(*p).driver_list.next as *mut flow_block_cb;}false }
pub unsafe fn flow_block_cb_setup_simple(f:*mut flow_block_offload,list:*mut list_head,cb:flow_setup_cb_t,id:*mut c_void,priv_:*mut c_void,ingress:bool)->i32 { if ingress&&(*f).binder_type!=FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS{return -EOPNOTSUPP} (*f).driver_block_list=list; match (*f).command { FLOW_BLOCK_BIND=>{if flow_block_cb_is_busy(cb,id,list){return -EBUSY}let p=flow_block_cb_alloc(cb,id,priv_,None);if (p as isize)<0{return p as isize as i32}flow_block_cb_add(p,f);(*p).driver_list.next=list;0}, FLOW_BLOCK_UNBIND=>{let p=flow_block_cb_lookup((*f).block,cb,id);if p.is_null(){return -ENOENT}flow_block_cb_remove(p,f);0}, _=>-EOPNOTSUPP} }
pub unsafe fn flow_indr_block_cb_alloc(cb:flow_setup_cb_t,id:*mut c_void,priv_:*mut c_void,release:Option<unsafe extern "C" fn(*mut c_void)>,bo:*mut flow_block_offload,dev:*mut net_device,sch:*mut Qdisc,data:*mut c_void,indr_priv:*mut c_void,cleanup:Option<unsafe extern "C" fn(*mut flow_block_cb)>)->*mut flow_block_cb {let p=flow_block_cb_alloc(cb,id,priv_,release);if (p as isize)<0{return p}(*p).indr.binder_type=(*bo).binder_type;(*p).indr.data=data;(*p).indr.cb_priv=indr_priv;(*p).indr.dev=dev;(*p).indr.sch=sch;(*p).indr.cleanup=cleanup;p}
pub unsafe fn flow_indr_dev_exists()->bool{false}
pub unsafe fn flow_indr_dev_register(_cb:flow_indr_block_bind_cb_t,_priv:*mut c_void)->i32{-ENOMEM}
pub unsafe fn flow_indr_dev_unregister(_cb:flow_indr_block_bind_cb_t,_priv:*mut c_void,_release:Option<unsafe extern "C" fn(*mut c_void)>){ }
pub unsafe fn flow_indr_dev_setup_offload(_dev:*mut net_device,_sch:*mut Qdisc,_type:u32,_data:*mut c_void,_bo:*mut flow_block_offload,_cleanup:Option<unsafe extern "C" fn(*mut flow_block_cb)>)->i32{-EOPNOTSUPP}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
