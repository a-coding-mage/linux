/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/lwtunnel.h. External kernel types and functions are
// intentionally referenced but not defined here.

pub const LWTUNNEL_HASH_BITS: u32 = 7;
pub const LWTUNNEL_HASH_SIZE: u32 = 1 << LWTUNNEL_HASH_BITS;

pub const LWTUNNEL_STATE_OUTPUT_REDIRECT: u16 = 1 << 0;
pub const LWTUNNEL_STATE_INPUT_REDIRECT: u16 = 1 << 1;
pub const LWTUNNEL_STATE_XMIT_REDIRECT: u16 = 1 << 2;

pub const LWTUNNEL_XMIT_DONE: i32 = 0;
pub const LWTUNNEL_XMIT_CONTINUE: i32 = 0x100;

#[repr(C)]
pub struct lwtunnel_state {
    pub type_: u16,
    pub flags: u16,
    pub headroom: u16,
    pub refcnt: atomic_t,
    pub orig_output: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>,
    pub orig_input: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub rcu: rcu_head,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct lwtunnel_encap_ops {
    pub build_state: Option<unsafe extern "C" fn(*mut net, *mut nlattr, u32, *const core::ffi::c_void, *mut *mut lwtunnel_state, *mut netlink_ext_ack) -> i32>,
    pub destroy_state: Option<unsafe extern "C" fn(*mut lwtunnel_state)>,
    pub output: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>,
    pub input: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub fill_encap: Option<unsafe extern "C" fn(*mut sk_buff, *mut lwtunnel_state) -> i32>,
    pub get_encap_size: Option<unsafe extern "C" fn(*mut lwtunnel_state) -> i32>,
    pub cmp_encap: Option<unsafe extern "C" fn(*mut lwtunnel_state, *mut lwtunnel_state) -> i32>,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub owner: *mut module,
}

#[cfg(feature = "CONFIG_LWTUNNEL")]
extern "C" {
    pub static nf_hooks_lwtunnel_enabled: static_key_false;
    pub fn lwtstate_free(lws: *mut lwtunnel_state);
    pub fn lwtunnel_encap_add_ops(op: *const lwtunnel_encap_ops, num: u32) -> i32;
    pub fn lwtunnel_encap_del_ops(op: *const lwtunnel_encap_ops, num: u32) -> i32;
    pub fn lwtunnel_valid_encap_type(encap_type: u16, extack: *mut netlink_ext_ack) -> i32;
    pub fn lwtunnel_valid_encap_type_attr(attr: *mut nlattr, len: i32, extack: *mut netlink_ext_ack) -> i32;
    pub fn lwtunnel_build_state(net: *mut net, encap_type: u16, encap: *mut nlattr, family: u32, cfg: *const core::ffi::c_void, lws: *mut *mut lwtunnel_state, extack: *mut netlink_ext_ack) -> i32;
    pub fn lwtunnel_fill_encap(skb: *mut sk_buff, lwtstate: *mut lwtunnel_state, encap_attr: i32, encap_type_attr: i32) -> i32;
    pub fn lwtunnel_get_encap_size(lwtstate: *mut lwtunnel_state) -> i32;
    pub fn lwtunnel_state_alloc(hdr_len: i32) -> *mut lwtunnel_state;
    pub fn lwtunnel_cmp_encap(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> i32;
    pub fn lwtunnel_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn lwtunnel_input(skb: *mut sk_buff) -> i32;
    pub fn lwtunnel_xmit(skb: *mut sk_buff) -> i32;
    pub fn bpf_lwt_push_ip_encap(skb: *mut sk_buff, hdr: *mut core::ffi::c_void, len: u32, ingress: bool) -> i32;
    pub fn lwtunnel_set_redirect(dst: *mut dst_entry);
}

#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtstate_get(lws: *mut lwtunnel_state) -> *mut lwtunnel_state {
    if !lws.is_null() { atomic_inc(&mut (*lws).refcnt); }
    lws
}

#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtstate_put(lws: *mut lwtunnel_state) {
    if !lws.is_null() && atomic_dec_and_test(&mut (*lws).refcnt) { lwtstate_free(lws); }
}

#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtunnel_output_redirect(s: *mut lwtunnel_state) -> bool {
    !s.is_null() && ((*s).flags & LWTUNNEL_STATE_OUTPUT_REDIRECT) != 0
}
#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtunnel_input_redirect(s: *mut lwtunnel_state) -> bool {
    !s.is_null() && ((*s).flags & LWTUNNEL_STATE_INPUT_REDIRECT) != 0
}
#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtunnel_xmit_redirect(s: *mut lwtunnel_state) -> bool {
    !s.is_null() && ((*s).flags & LWTUNNEL_STATE_XMIT_REDIRECT) != 0
}
#[cfg(feature = "CONFIG_LWTUNNEL")]
#[inline]
pub unsafe fn lwtunnel_headroom(s: *mut lwtunnel_state, mtu: u32) -> u32 {
    if (lwtunnel_xmit_redirect(s) || lwtunnel_output_redirect(s)) && (*s).headroom as u32 < mtu { (*s).headroom as u32 } else { 0 }
}

#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtstate_free(_: *mut lwtunnel_state) {}
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtstate_get(s: *mut lwtunnel_state) -> *mut lwtunnel_state { s }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtstate_put(_: *mut lwtunnel_state) {}
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_output_redirect(_: *mut lwtunnel_state) -> bool { false }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_input_redirect(_: *mut lwtunnel_state) -> bool { false }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_xmit_redirect(_: *mut lwtunnel_state) -> bool { false }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_headroom(_: *mut lwtunnel_state, _: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_set_redirect(_: *mut dst_entry) {}

#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_encap_add_ops(_: *const lwtunnel_encap_ops, _: u32) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_encap_del_ops(_: *const lwtunnel_encap_ops, _: u32) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_valid_encap_type(_: u16, _: *mut netlink_ext_ack) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_valid_encap_type_attr(_: *mut nlattr, _: i32, _: *mut netlink_ext_ack) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_build_state(_: *mut net, _: u16, _: *mut nlattr, _: u32, _: *const core::ffi::c_void, _: *mut *mut lwtunnel_state, _: *mut netlink_ext_ack) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_fill_encap(_: *mut sk_buff, _: *mut lwtunnel_state, _: i32, _: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_get_encap_size(_: *mut lwtunnel_state) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_state_alloc(_: i32) -> *mut lwtunnel_state { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_cmp_encap(_: *mut lwtunnel_state, _: *mut lwtunnel_state) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_output(_: *mut net, _: *mut sock, _: *mut sk_buff) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_input(_: *mut sk_buff) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_LWTUNNEL"))]
#[inline] pub unsafe fn lwtunnel_xmit(_: *mut sk_buff) -> i32 { -EOPNOTSUPP }

// External dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub fn atomic_inc(v: *mut atomic_t);
    pub fn atomic_dec_and_test(v: *mut atomic_t) -> bool;
}

// C identifiers/types supplied by included headers.
pub enum atomic_t {}
pub enum rcu_head {}
pub enum net {}
pub enum sock {}
pub enum sk_buff {}
pub enum nlattr {}
pub enum netlink_ext_ack {}
pub enum module {}
pub enum static_key_false {}
pub enum dst_entry {}
pub const EOPNOTSUPP: i32 = 95;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
