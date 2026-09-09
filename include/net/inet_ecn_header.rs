/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux networking headers.

pub const INET_ECN_NOT_ECT: u8 = 0;
pub const INET_ECN_ECT_1: u8 = 1;
pub const INET_ECN_ECT_0: u8 = 2;
pub const INET_ECN_CE: u8 = 3;
pub const INET_ECN_MASK: u8 = 3;

extern "C" {
    pub static mut sysctl_tunnel_ecn_log: ::core::ffi::c_int;
}

#[inline]
pub fn INET_ECN_is_ce(dsfield: u8) -> ::core::ffi::c_int {
    ((dsfield & INET_ECN_MASK) == INET_ECN_CE) as ::core::ffi::c_int
}

#[inline]
pub fn INET_ECN_is_not_ect(dsfield: u8) -> ::core::ffi::c_int {
    ((dsfield & INET_ECN_MASK) == INET_ECN_NOT_ECT) as ::core::ffi::c_int
}

#[inline]
pub fn INET_ECN_is_capable(dsfield: u8) -> u8 {
    dsfield & INET_ECN_ECT_0
}

#[inline]
pub fn INET_ECN_encapsulate(mut outer: u8, inner: u8) -> u8 {
    outer &= !INET_ECN_MASK;
    outer |= if INET_ECN_is_ce(inner) == 0 {
        inner & INET_ECN_MASK
    } else {
        INET_ECN_ECT_0
    };
    outer
}

#[inline]
pub unsafe fn __INET_ECN_xmit(sk: *mut sock, use_ect_1: bool) {
    let ect: u8 = if use_ect_1 { INET_ECN_ECT_1 } else { INET_ECN_ECT_0 };
    (*inet_sk(sk)).tos &= !INET_ECN_MASK;
    (*inet_sk(sk)).tos |= ect;
    if !inet6_sk(sk).is_null() {
        (*inet6_sk(sk)).tclass &= !INET_ECN_MASK;
        (*inet6_sk(sk)).tclass |= ect;
    }
}

#[inline]
pub unsafe fn INET_ECN_xmit(sk: *mut sock) {
    __INET_ECN_xmit(sk, false);
}

#[inline]
pub unsafe fn INET_ECN_dontxmit(sk: *mut sock) {
    (*inet_sk(sk)).tos &= !INET_ECN_MASK;
    if !inet6_sk(sk).is_null() {
        (*inet6_sk(sk)).tclass &= !INET_ECN_MASK;
    }
}

#[inline]
pub fn IP6_ECN_flow_init(label: &mut u32) {
    *label &= !(htonl((INET_ECN_MASK as u32) << 20));
}

#[inline]
pub unsafe fn IP6_ECN_flow_xmit(sk: *mut sock, label: &mut u32) {
    if INET_ECN_is_capable((*inet6_sk(sk)).tclass) != 0 {
        *label |= htonl((INET_ECN_ECT_0 as u32) << 20);
    }
}

#[inline]
pub unsafe fn IP_ECN_set_ce(iph: *mut iphdr) -> ::core::ffi::c_int {
    let ecn: u32 = ((*iph).tos as u32 + 1) & INET_ECN_MASK as u32;
    if (ecn & 2) == 0 {
        return (ecn == 0) as ::core::ffi::c_int;
    }
    let check_add: u16 = (htons(0xfffb) as u16).wrapping_add(htons(ecn as u16) as u16);
    (*iph).check = csum16_add((*iph).check, check_add);
    (*iph).tos |= INET_ECN_CE;
    1
}

#[inline]
pub unsafe fn IP_ECN_set_ect1(iph: *mut iphdr) -> ::core::ffi::c_int {
    if ((*iph).tos & INET_ECN_MASK) != INET_ECN_ECT_0 { return 0; }
    (*iph).check = csum16_add((*iph).check, htons(1));
    (*iph).tos ^= INET_ECN_MASK;
    1
}

#[inline]
pub unsafe fn IP_ECN_clear(iph: *mut iphdr) { (*iph).tos &= !INET_ECN_MASK; }

#[inline]
pub unsafe fn ipv4_copy_dscp(dscp: u32, inner: *mut iphdr) {
    ipv4_change_dsfield(inner, INET_ECN_MASK, dscp & !(INET_ECN_MASK as u32));
}

#[repr(C)]
pub struct ipv6hdr { _opaque: [u8; 0] }

#[inline]
pub unsafe fn IP6_ECN_set_ce(skb: *mut sk_buff, iph: *mut ipv6hdr) -> ::core::ffi::c_int {
    if ipv6_get_dsfield(iph) & INET_ECN_MASK == INET_ECN_NOT_ECT { return 0; }
    let from = *(iph as *const u32);
    let to = from | htonl((INET_ECN_CE as u32) << 20);
    *(iph as *mut u32) = to;
    if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).csum = csum_add(csum_sub((*skb).csum, from), to); }
    1
}

#[inline]
pub unsafe fn IP6_ECN_set_ect1(skb: *mut sk_buff, iph: *mut ipv6hdr) -> ::core::ffi::c_int {
    if ipv6_get_dsfield(iph) & INET_ECN_MASK != INET_ECN_ECT_0 { return 0; }
    let from = *(iph as *const u32);
    let to = from ^ htonl((INET_ECN_MASK as u32) << 20);
    *(iph as *mut u32) = to;
    if (*skb).ip_summed == CHECKSUM_COMPLETE { (*skb).csum = csum_add(csum_sub((*skb).csum, from), to); }
    1
}

#[inline]
pub unsafe fn ipv6_copy_dscp(dscp: u32, inner: *mut ipv6hdr) {
    ipv6_change_dsfield(inner, INET_ECN_MASK, dscp & !(INET_ECN_MASK as u32));
}

#[inline]
pub unsafe fn INET_ECN_set_ce(skb: *mut sk_buff) -> ::core::ffi::c_int {
    match skb_protocol(skb, true) {
        x if x == cpu_to_be16(ETH_P_IP) => if skb_network_header(skb).add(core::mem::size_of::<iphdr>()) <= skb_tail_pointer(skb) { IP_ECN_set_ce(ip_hdr(skb)) } else { 0 },
        x if x == cpu_to_be16(ETH_P_IPV6) => if skb_network_header(skb).add(core::mem::size_of::<ipv6hdr>()) <= skb_tail_pointer(skb) { IP6_ECN_set_ce(skb, ipv6_hdr(skb)) } else { 0 },
        _ => 0,
    }
}

#[inline]
pub unsafe fn skb_get_dsfield(skb: *mut sk_buff) -> ::core::ffi::c_int {
    match skb_protocol(skb, true) {
        x if x == cpu_to_be16(ETH_P_IP) => { if !pskb_network_may_pull(skb, core::mem::size_of::<iphdr>()) { return -1; } ipv4_get_dsfield(ip_hdr(skb)) as _ },
        x if x == cpu_to_be16(ETH_P_IPV6) => { if !pskb_network_may_pull(skb, core::mem::size_of::<ipv6hdr>()) { return -1; } ipv6_get_dsfield(ipv6_hdr(skb)) as _ },
        _ => -1,
    }
}

#[inline]
pub unsafe fn INET_ECN_set_ect1(skb: *mut sk_buff) -> ::core::ffi::c_int {
    match skb_protocol(skb, true) {
        x if x == cpu_to_be16(ETH_P_IP) => if skb_network_header(skb).add(core::mem::size_of::<iphdr>()) <= skb_tail_pointer(skb) { IP_ECN_set_ect1(ip_hdr(skb)) } else { 0 },
        x if x == cpu_to_be16(ETH_P_IPV6) => if skb_network_header(skb).add(core::mem::size_of::<ipv6hdr>()) <= skb_tail_pointer(skb) { IP6_ECN_set_ect1(skb, ipv6_hdr(skb)) } else { 0 },
        _ => 0,
    }
}

#[inline]
pub unsafe fn __INET_ECN_decapsulate(outer: u8, inner: u8, set_ce: *mut bool) -> ::core::ffi::c_int {
    if INET_ECN_is_not_ect(inner) != 0 {
        match outer & INET_ECN_MASK { 0 => return 0, 2 | 1 => return 1, 3 => return 2, _ => {} }
    }
    *set_ce = INET_ECN_is_ce(outer) != 0;
    0
}

#[inline]
pub unsafe fn INET_ECN_decapsulate(skb: *mut sk_buff, outer: u8, inner: u8) -> ::core::ffi::c_int {
    let mut set_ce = false;
    let rc = __INET_ECN_decapsulate(outer, inner, &mut set_ce);
    if rc == 0 { if set_ce { INET_ECN_set_ce(skb); } else if outer & INET_ECN_MASK == INET_ECN_ECT_1 { INET_ECN_set_ect1(skb); } }
    rc
}

#[inline]
pub unsafe fn IP_ECN_decapsulate(oiph: *const iphdr, skb: *mut sk_buff) -> ::core::ffi::c_int {
    let inner = match skb_protocol(skb, true) { x if x == htons(ETH_P_IP) => (*ip_hdr(skb)).tos, x if x == htons(ETH_P_IPV6) => ipv6_get_dsfield(ipv6_hdr(skb)), _ => return 0 };
    INET_ECN_decapsulate(skb, (*oiph).tos, inner)
}

#[inline]
pub unsafe fn IP6_ECN_decapsulate(oipv6h: *const ipv6hdr, skb: *mut sk_buff) -> ::core::ffi::c_int {
    let inner = match skb_protocol(skb, true) { x if x == htons(ETH_P_IP) => (*ip_hdr(skb)).tos, x if x == htons(ETH_P_IPV6) => ipv6_get_dsfield(ipv6_hdr(skb)), _ => return 0 };
    INET_ECN_decapsulate(skb, ipv6_get_dsfield(oipv6h as *mut ipv6hdr), inner)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
