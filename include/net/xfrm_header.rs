/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust declaration translation of net/xfrm.h.  Kernel-provided types
// and functions referenced here are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes)]
use core::ffi::{c_char, c_int, c_void};

pub const XFRM_PROTO_ESP: u8 = 50;
pub const XFRM_PROTO_AH: u8 = 51;
pub const XFRM_PROTO_COMP: u8 = 108;
pub const XFRM_PROTO_IPIP: u8 = 4;
pub const XFRM_PROTO_IPV6: u8 = 41;
pub const XFRM_MAX_DEPTH: usize = 6;
pub const XFRM_MAX_OFFLOAD_DEPTH: usize = 1;
pub const XFRM_MODE_FLAG_TUNNEL: u8 = 1;
pub const XFRM_TIME_DEFER: u32 = 1;
pub const XFRM_SOFT_EXPIRE: u32 = 2;
pub const XFRM_KM_TIMEOUT: u32 = 30;
pub const XFRM_AE_ETIME: u32 = 10;
pub const XFRM_AE_ETH_M: u32 = 10;
pub const XFRM_AE_SEQT_SIZE: u32 = 2;

pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32;
pub type __be16 = u16; pub type __be32 = u32; pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32;
pub type gfp_t = u32; pub type time64_t = i64; pub type netdev_features_t = u64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct possible_net_t { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub cb: [u8; 48] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct rtable { _private: [u8; 0] }
#[repr(C)] pub struct rt6_info { _private: [u8; 0] }
#[repr(C)] pub struct flowi { pub flowi_proto: u8, _private: [u8; 0] }
#[repr(C)] pub struct xfrm_id { pub daddr: xfrm_address_t, pub proto: u8, pub spi: __be32 }
#[repr(C)] pub struct xfrm_address_t { pub a4: __be32, pub in6: [u32; 4] }
#[repr(C)] pub struct xfrm_selector { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_mark { pub v: u32, pub m: u32 }
#[repr(C)] pub struct xfrm_lifetime_cfg { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_lifetime_cur { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_algo { pub alg_name: [c_char; 64], pub alg_key_len: u32, pub alg_key: [u8; 0] }
#[repr(C)] pub struct xfrm_algo_auth { pub alg_name: [c_char; 64], pub alg_key_len: u32, pub alg_key: [u8; 0] }
#[repr(C)] pub struct xfrm_algo_aead { pub alg_name: [c_char; 64], pub alg_key_len: u32, pub alg_icv_len: u32, pub alg_key: [u8; 0] }
#[repr(C)] pub struct xfrm_replay_state { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_replay_state_esn { pub bmp_len: u32, _private: [u8; 0] }
#[repr(C)] pub struct xfrm_stats { _private: [u8; 0] }
#[repr(C)] pub struct page_frag { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_encap_tmpl { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_sec_ctx { pub ctx_sid: u32, pub ctx_doi: u32, pub ctx_alg: u32 }
#[repr(C)] pub struct xfrm_type { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_type_offload { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_mode_cbs { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_user_offload { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_address_filter { _private: [u8; 0] }
#[repr(C)] pub struct gro_cells { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }

#[repr(u32)] pub enum xfrm_replay_mode { XFRM_REPLAY_MODE_LEGACY, XFRM_REPLAY_MODE_BMP, XFRM_REPLAY_MODE_ESN }
#[repr(C)] pub struct xfrm_state_walk { pub all: list_head, pub state: u8, pub dying: u8, pub proto: u8, pub seq: u32, pub filter: *mut xfrm_address_filter }
#[repr(C)] pub struct xfrm_mode { pub encap: u8, pub family: u8, pub flags: u8 }
#[repr(C)] pub struct xfrm_dev_offload { pub dev: *mut net_device, pub real_dev: *mut net_device, pub offload_handle: usize, pub ifindex: c_int, pub dir: u8, pub type_: u8, pub flags: u8 }
#[repr(C)] pub struct xfrm_state {
    pub xs_net: possible_net_t, pub gclist: hlist_node, pub dev_gclist: hlist_node,
    pub byspi: hlist_node, pub byseq: hlist_node, pub state_cache: hlist_node,
    pub state_cache_input: hlist_node, pub refcnt: refcount_t, pub lock: spinlock_t,
    pub pcpu_num: u32, pub id: xfrm_id, pub sel: xfrm_selector, pub mark: xfrm_mark,
    pub if_id: u32, pub tfcpad: u32, pub genid: u32, pub props: xfrm_state_props,
    pub lft: xfrm_lifetime_cfg, pub aalg: *mut xfrm_algo_auth, pub ealg: *mut xfrm_algo,
    pub calg: *mut xfrm_algo, pub aead: *mut xfrm_algo_aead, pub geniv: *const c_char,
    pub encap: *mut xfrm_encap_tmpl, pub coaddr: *mut xfrm_address_t, pub tunnel: *mut xfrm_state,
    pub tunnel_users: atomic_t, pub replay: xfrm_replay_state, pub replay_esn: *mut xfrm_replay_state_esn,
    pub preplay: xfrm_replay_state, pub preplay_esn: *mut xfrm_replay_state_esn,
    pub repl_mode: xfrm_replay_mode, pub xflags: u32, pub replay_maxage: u32, pub replay_maxdiff: u32,
    pub rtimer: timer_list, pub stats: xfrm_stats, pub curlft: xfrm_lifetime_cur, pub mtimer: hrtimer,
    pub xso: xfrm_dev_offload, pub saved_tmo: isize, pub lastused: time64_t, pub xfrag: page_frag,
    pub type_: *const xfrm_type, pub inner_mode: xfrm_mode, pub inner_mode_iaf: xfrm_mode, pub outer_mode: xfrm_mode,
    pub type_offload: *const xfrm_type_offload, pub security: *mut xfrm_sec_ctx, pub data: *mut c_void,
    pub dir: u8, pub mode_cbs: *const xfrm_mode_cbs, pub mode_data: *mut c_void,
}
#[repr(C)] pub struct xfrm_state_props { pub reqid: u32, pub mode: u8, pub replay_window: u8, pub a_algo: u8, pub e_algo: u8, pub c_algo: u8, pub flags: u8, pub family: u16, pub saddr: xfrm_address_t, pub header_len: c_int, pub enc_hdr_len: c_int, pub trailer_len: c_int, pub extra_flags: u32, pub smark: xfrm_mark }
#[repr(C)] pub struct xfrm_tmpl { pub id: xfrm_id, pub saddr: xfrm_address_t, pub encap_family: u16, pub reqid: u32, pub mode: u8, pub share: u8, pub optional: u8, pub allalgs: u8, pub aalgos: u32, pub ealgos: u32, pub calgos: u32 }
#[repr(C)] pub struct xfrm_policy { pub xp_net: possible_net_t, pub bydst: hlist_node, pub byidx: hlist_node, pub state_cache_list: hlist_head, pub lock: rwlock_t, pub refcnt: refcount_t, pub pos: u32, pub timer: timer_list, pub genid: atomic_t, pub priority: u32, pub index: u32, pub if_id: u32, pub mark: xfrm_mark, pub selector: xfrm_selector, pub lft: xfrm_lifetime_cfg, pub curlft: xfrm_lifetime_cur, pub xfrm_vec: [xfrm_tmpl; XFRM_MAX_DEPTH], pub xdo: xfrm_dev_offload }

pub const fn xfrm_align4(len: usize) -> usize { (len + 3) & !3 }
pub const fn xfrm_align8(len: usize) -> usize { (len + 7) & !7 }
pub unsafe fn xfrm_af2proto(family: u32) -> i32 { match family { 2 => 4, 10 => 41, _ => 0 } }
pub unsafe fn xfrm_ip2inner_mode<'a>(x: &'a xfrm_state, ipproto: i32) -> &'a xfrm_mode { if x.sel as *const _ as usize != 0 || (ipproto == 4 && x.props.family == 2) || (ipproto == 41 && x.props.family == 10) { &x.inner_mode } else { &x.inner_mode_iaf } }

extern "C" {
    pub fn xfrm_if_register_cb(ifcb: *const c_void);
    pub fn xfrm_if_unregister_cb();
    pub fn xfrm_policy_register_afinfo(afinfo: *const c_void, family: c_int) -> c_int;
    pub fn xfrm_policy_unregister_afinfo(afinfo: *const c_void);
    pub fn xfrm_state_register_afinfo(afinfo: *mut c_void) -> c_int;
    pub fn xfrm_state_unregister_afinfo(afinfo: *mut c_void) -> c_int;
    pub fn xfrm_state_alloc(net: *mut net) -> *mut xfrm_state;
    pub fn xfrm_state_free(x: *mut xfrm_state);
    pub fn xfrm_state_delete(x: *mut xfrm_state) -> c_int;
    pub fn xfrm_state_put(x: *mut xfrm_state);
    pub fn xfrm_state_hold(x: *mut xfrm_state);
    pub fn xfrm_policy_destroy(policy: *mut xfrm_policy);
    pub fn xfrm_policy_delete(pol: *mut xfrm_policy, dir: c_int) -> c_int;
    pub fn xfrm_input(skb: *mut sk_buff, nexthdr: c_int, spi: __be32, encap_type: c_int) -> c_int;
    pub fn xfrm_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn xfrm4_init(); pub fn xfrm_init(); pub fn xfrm4_rcv(skb: *mut sk_buff) -> c_int;
    pub fn xfrm6_rcv(skb: *mut sk_buff) -> c_int;
}

pub unsafe fn xfrm_state_kern(x: *const xfrm_state) -> c_int { (*x).tunnel_users.counter }
pub unsafe fn xfrm_policy_id2dir(index: u32) -> c_int { (index & 7) as c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
