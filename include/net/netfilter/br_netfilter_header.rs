/* SPDX-License-Identifier: GPL-2.0 */

// Translated from br_netfilter.h.  The included kernel declarations are
// supplied by other translated units.

use core::ffi::c_void;

pub const NF_ACCEPT: u32 = 1;
pub const SKB_EXT_BRIDGE_NF: u32 = 0;

#[repr(C)]
pub struct nf_bridge_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub network_header: u16,
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_bridge {
    pub fake_rtable: rtable,
}

#[repr(C)]
pub struct net_bridge_port {
    pub br: *mut net_bridge,
}

#[repr(C)]
pub struct nf_hook_state {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn skb_ext_add(skb: *mut sk_buff, id: u32) -> *mut nf_bridge_info;
    pub fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    pub fn skb_push(skb: *mut sk_buff, len: u32) -> *mut c_void;
    pub fn br_port_get_rcu(dev: *const net_device) -> *mut net_bridge_port;

    pub fn nf_bridge_update_protocol(skb: *mut sk_buff);
    pub fn br_nf_hook_thresh(
        hook: u32,
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        indev: *mut net_device,
        outdev: *mut net_device,
        okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>,
    ) -> i32;
    pub fn nf_bridge_encap_header_len(skb: *const sk_buff) -> u32;
    pub fn br_nf_pre_routing_finish_bridge(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
    ) -> i32;
    pub fn setup_pre_routing(skb: *mut sk_buff, net: *const net) -> *mut net_device;
}

#[inline]
pub unsafe fn nf_bridge_alloc(skb: *mut sk_buff) -> *mut nf_bridge_info {
    // CONFIG_BRIDGE_NETFILTER is a build-time condition from the C header.
    let b = unsafe { skb_ext_add(skb, SKB_EXT_BRIDGE_NF) };
    if !b.is_null() {
        unsafe { memset(b.cast(), 0, core::mem::size_of::<nf_bridge_info>()); }
    }
    b
}

#[inline]
pub unsafe fn nf_bridge_push_encap_header(skb: *mut sk_buff) {
    let len = unsafe { nf_bridge_encap_header_len(skb) };
    unsafe { skb_push(skb, len); }
    (*skb).network_header = (*skb).network_header.wrapping_sub(len as u16);
}

#[inline]
pub unsafe fn bridge_parent_rtable(dev: *const net_device) -> *mut rtable {
    // CONFIG_BRIDGE_NETFILTER is a build-time condition from the C header.
    let port = unsafe { br_port_get_rcu(dev) };
    if port.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { &mut (*(*port).br).fake_rtable }
    }
}

#[cfg(feature = "CONFIG_IPV6")]
unsafe extern "C" {
    pub fn br_validate_ipv6(net: *mut net, skb: *mut sk_buff) -> i32;
    pub fn br_nf_pre_routing_ipv6(
        priv_: *mut c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> u32;
}

#[cfg(not(feature = "CONFIG_IPV6"))]
#[inline]
pub unsafe fn br_validate_ipv6(_net: *mut net, _skb: *mut sk_buff) -> i32 {
    -1
}

#[cfg(not(feature = "CONFIG_IPV6"))]
#[inline]
pub unsafe fn br_nf_pre_routing_ipv6(
    _priv: *mut c_void,
    _skb: *mut sk_buff,
    _state: *const nf_hook_state,
) -> u32 {
    NF_ACCEPT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
