/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/icmpv6.h.  The included kernel types and symbols are
// supplied by the surrounding translation unit.

#[inline]
pub unsafe fn icmp6_hdr(skb: *const sk_buff) -> *mut icmp6hdr {
    skb_transport_header(skb) as *mut icmp6hdr
}

// CONFIG_IPV6-enabled declarations.
extern "C" {
    pub fn icmp6_send(
        skb: *mut sk_buff,
        type_: u8,
        code: u8,
        info: u32,
        force_saddr: *const in6_addr,
        parm: *const inet6_skb_parm,
    );

    pub fn ip6_err_gen_icmpv6_unreach(
        skb: *mut sk_buff,
        nhs: i32,
        type_: i32,
        data_len: u32,
    ) -> i32;

    pub fn icmpv6_init() -> i32;
    pub fn icmpv6_err_convert(type_: u8, code: u8, err: *mut i32) -> i32;
    pub fn icmpv6_cleanup();
    pub fn icmpv6_param_prob_reason(
        skb: *mut sk_buff,
        code: u8,
        pos: i32,
        reason: skb_drop_reason,
    );

    pub fn icmpv6_flow_init(
        sk: *const sock,
        fl6: *mut flowi6,
        type_: u8,
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        oif: i32,
    );
}

#[inline]
pub unsafe fn icmpv6_send(
    skb: *mut sk_buff,
    type_: u8,
    code: u8,
    info: u32,
) {
    icmp6_send(skb, type_, code, info, core::ptr::null(), IP6CB(skb));
}

// CONFIG_NF_NAT-enabled declaration; otherwise the local implementation is used.
#[inline]
pub unsafe fn icmpv6_ndo_send(
    skb_in: *mut sk_buff,
    type_: u8,
    code: u8,
    info: u32,
) {
    let parm: inet6_skb_parm = core::mem::zeroed();
    icmp6_send(skb_in, type_, code, info, core::ptr::null(), &parm);
}

// CONFIG_IPV6-disabled stubs preserve the header's no-op behavior.
#[inline]
pub unsafe fn icmpv6_send_disabled(
    _skb: *mut sk_buff,
    _type_: u8,
    _code: u8,
    _info: u32,
) {
}

#[inline]
pub unsafe fn icmpv6_ndo_send_disabled(
    _skb: *mut sk_buff,
    _type_: u8,
    _code: u8,
    _info: u32,
) {
}

#[inline]
pub unsafe fn icmpv6_param_prob(skb: *mut sk_buff, code: u8, pos: i32) {
    icmpv6_param_prob_reason(skb, code, pos, SKB_DROP_REASON_NOT_SPECIFIED);
}

#[inline]
pub fn icmpv6_is_err(type_: i32) -> bool {
    match type_ {
        ICMPV6_DEST_UNREACH | ICMPV6_PKT_TOOBIG | ICMPV6_TIME_EXCEED | ICMPV6_PARAMPROB => true,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
