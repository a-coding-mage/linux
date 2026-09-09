/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the Linux networking environment are intentionally
// left external to this translation unit.

#[repr(C)]
pub struct synproxy_stats {
    pub syn_received: ::core::ffi::c_uint,
    pub cookie_invalid: ::core::ffi::c_uint,
    pub cookie_valid: ::core::ffi::c_uint,
    pub cookie_retrans: ::core::ffi::c_uint,
    pub conn_reopened: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct synproxy_net {
    pub tmpl: *mut nf_conn,
    pub stats: *mut synproxy_stats,
    pub hook_ref4: ::core::ffi::c_uint,
    pub hook_ref6: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub static mut synproxy_net_id: ::core::ffi::c_uint;
}

#[inline]
pub unsafe fn synproxy_pernet(net: *mut net) -> *mut synproxy_net {
    net_generic(net, synproxy_net_id)
}

#[repr(C)]
pub struct synproxy_options {
    pub options: u8,
    pub wscale: u8,
    pub mss_option: u16,
    pub mss_encode: u16,
    pub tsval: u32,
    pub tsecr: u32,
}

pub struct nf_synproxy_info;
pub struct nf_hook_state;

unsafe extern "C" {
    pub fn synproxy_parse_options(
        skb: *const sk_buff,
        doff: ::core::ffi::c_uint,
        th: *const tcphdr,
        opts: *mut synproxy_options,
    ) -> bool;

    pub fn synproxy_init_timestamp_cookie(
        info: *const nf_synproxy_info,
        opts: *mut synproxy_options,
    );

    pub fn synproxy_send_client_synack(
        net: *mut net,
        skb: *const sk_buff,
        th: *const tcphdr,
        opts: *const synproxy_options,
    );

    pub fn synproxy_recv_client_ack(
        net: *mut net,
        skb: *const sk_buff,
        th: *const tcphdr,
        opts: *mut synproxy_options,
        recv_seq: u32,
    ) -> bool;

    pub fn ipv4_synproxy_hook(
        priv_: *mut ::core::ffi::c_void,
        skb: *mut sk_buff,
        nhs: *const nf_hook_state,
    ) -> ::core::ffi::c_uint;
    pub fn nf_synproxy_ipv4_init(snet: *mut synproxy_net, net: *mut net) -> ::core::ffi::c_int;
    pub fn nf_synproxy_ipv4_fini(snet: *mut synproxy_net, net: *mut net);
}

// Equivalent to: #if IS_ENABLED(CONFIG_IPV6)
#[cfg(CONFIG_IPV6)]
unsafe extern "C" {
    pub fn synproxy_send_client_synack_ipv6(
        net: *mut net,
        skb: *const sk_buff,
        th: *const tcphdr,
        opts: *const synproxy_options,
    );

    pub fn synproxy_recv_client_ack_ipv6(
        net: *mut net,
        skb: *const sk_buff,
        th: *const tcphdr,
        opts: *mut synproxy_options,
        recv_seq: u32,
    ) -> bool;

    pub fn ipv6_synproxy_hook(
        priv_: *mut ::core::ffi::c_void,
        skb: *mut sk_buff,
        nhs: *const nf_hook_state,
    ) -> ::core::ffi::c_uint;
    pub fn nf_synproxy_ipv6_init(snet: *mut synproxy_net, net: *mut net) -> ::core::ffi::c_int;
    pub fn nf_synproxy_ipv6_fini(snet: *mut synproxy_net, net: *mut net);
}

#[cfg(not(CONFIG_IPV6))]
#[inline]
pub unsafe fn nf_synproxy_ipv6_init(_snet: *mut synproxy_net, _net: *mut net) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_IPV6))]
#[inline]
pub unsafe fn nf_synproxy_ipv6_fini(_snet: *mut synproxy_net, _net: *mut net) {}

// External types and net_generic are provided by the translated dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
