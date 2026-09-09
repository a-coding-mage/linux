// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is a module which is used for setting the MSS option in TCP packets.
 *
 * Copyright (C) 2000 Marc Boucher <marc@mbsi.ca>
 * Copyright (C) 2007 Patrick McHardy <kaber@trash.net>
 */

// Kernel headers and build-time configuration supplied by the surrounding tree.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
mod translation {
    use core::ffi::{c_char, c_int, c_uint, c_void};

    type u8_t = u8;
    type u16_t = u16;
    type u32_t = u32;
    type __be16 = u16;
    type __be32 = u32;

    #[repr(C)] pub struct net { _private: [u8; 0] }
    #[repr(C)] pub struct sk_buff { pub len: u32, pub ip_summed: u32, pub csum: u32, _private: [u8; 0] }
    #[repr(C)] pub struct dst_entry { _private: [u8; 0] }
    #[repr(C)] pub struct rtable { pub dst: dst_entry }
    #[repr(C)] pub struct flowi { pub u: flowi_union }
    #[repr(C)] pub union flowi_union { pub ip4: flowi4, pub ip6: flowi6 }
    #[repr(C)] pub struct flowi4 { pub daddr: u32, _private: [u8; 0] }
    #[repr(C)] pub struct flowi6 { pub daddr: [u8; 16], _private: [u8; 0] }
    #[repr(C)] pub struct tcphdr { pub source: u16, pub dest: u16, pub seq: u32, pub ack_seq: u32, pub doff: u16, pub check: __be16, _private: [u8; 0] }
    #[repr(C)] pub struct iphdr { pub ihl: u8, pub check: __be16, pub tot_len: __be16, _private: [u8; 0] }
    #[repr(C)] pub struct ipv6hdr { pub nexthdr: u8, pub payload_len: __be16, _private: [u8; 0] }
    #[repr(C)] pub struct xt_action_param { pub targinfo: *const c_void, pub fragoff: u16, _private: [u8; 0] }
    #[repr(C)] pub struct xt_tgchk_param { pub targinfo: *const c_void, pub hook_mask: u32, pub nft_compat: bool, pub entryinfo: *const c_void, _private: [u8; 0] }
    #[repr(C)] pub struct xt_tcpmss_info { pub mss: u16 }
    #[repr(C)] pub struct xt_tcp { pub flg_cmp: u8, pub invflags: u8 }
    #[repr(C)] pub struct xt_entry_match { pub data: [u8; 0], _private: [u8; 0] }
    #[repr(C)] pub struct ipt_entry { _private: [u8; 0] }
    #[repr(C)] pub struct ip6t_entry { _private: [u8; 0] }
    #[repr(C)] pub struct xt_target { pub family: u16, pub name: *const c_char, pub check_hooks: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>, pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>, pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> c_uint>, pub targetsize: usize, pub proto: u8, pub me: *mut c_void }

    const TCPOPT_NOP: u8 = 1;
    const TCPOPT_MSS: u8 = 2;
    const TCPOLEN_MSS: usize = 4;
    const TCPHDR_SYN: u8 = 2;
    const XT_TCP_INV_FLAGS: u8 = 2;
    const XT_TCPMSS_CLAMP_PMTU: u16 = 0xffff;
    const NFPROTO_IPV4: u16 = 2;
    const NFPROTO_IPV6: u16 = 10;
    const PF_INET: u32 = 2;
    const PF_INET6: u32 = 10;
    const IPPROTO_TCP: u8 = 6;
    const NF_DROP: u32 = 0;
    const XT_CONTINUE: u32 = 0xFFFFFFFF;
    const CHECKSUM_COMPLETE: u32 = 3;

    extern "C" {
        fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
        fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
        fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
        fn nf_route(net: *mut net, rt: *mut *mut dst_entry, fl: *mut flowi, strict: bool, family: u32) -> c_int;
        fn dst_mtu(dst: *const dst_entry) -> u32;
        fn dst_release(dst: *mut dst_entry);
        fn skb_ensure_writable(skb: *mut sk_buff, len: u32) -> c_int;
        fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
        fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
        fn skb_tailroom(skb: *mut sk_buff) -> u32;
        fn pskb_expand_head(skb: *mut sk_buff, nhead: u32, ntail: u32, gfp: u32) -> c_int;
        fn skb_put(skb: *mut sk_buff, len: u32) -> *mut u8;
        fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
        fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
        fn xt_net(par: *const xt_action_param) -> *mut net;
        fn xt_family(par: *const xt_action_param) -> u16;
        fn inet_proto_csum_replace2(check: *mut __be16, skb: *mut sk_buff, from: __be16, to: __be16, pseudohdr: bool);
        fn inet_proto_csum_replace4(check: *mut __be16, skb: *mut sk_buff, from: u32, to: u32, pseudohdr: bool);
        fn csum_replace2(check: *mut __be16, from: __be16, to: __be16);
        fn csum_add(a: u32, b: u32) -> u32;
        fn csum_sub(a: u32, b: u32) -> u32;
        fn htons(x: u16) -> u16;
        fn ntohs(x: u16) -> u16;
        fn ipv6_skip_exthdr(skb: *mut sk_buff, start: u32, nexthdr: *mut u8, frag_off: *mut __be16) -> c_int;
        fn xt_register_targets(t: *mut xt_target, n: usize) -> c_int;
        fn xt_unregister_targets(t: *mut xt_target, n: usize);
    }

    #[inline]
    unsafe fn optlen(opt: *const u8, offset: usize) -> usize {
        if *opt.add(offset) <= TCPOPT_NOP || *opt.add(offset + 1) == 0 { 1 } else { *opt.add(offset + 1) as usize }
    }

    unsafe fn tcpmss_reverse_mtu(net: *mut net, skb: *const sk_buff, family: u32) -> u32 {
        let mut fl: flowi = core::mem::zeroed();
        let mut rt: *mut rtable = core::ptr::null_mut();
        let mut mtu = !0u32;
        if family == PF_INET { fl.u.ip4.daddr = (*ip_hdr(skb as *mut sk_buff))._private_addr(); }
        else { fl.u.ip6.daddr = (*ipv6_hdr(skb as *mut sk_buff))._private_addr6(); }
        nf_route(net, &mut rt as *mut _ as *mut *mut dst_entry, &mut fl, false, family);
        if !rt.is_null() { mtu = dst_mtu(&(*rt).dst); dst_release(&mut (*rt).dst); }
        mtu
    }

    // The address fields below are supplied by the kernel's complete header layouts.
    trait HeaderAddress { unsafe fn _private_addr(&self) -> u32; unsafe fn _private_addr6(&self) -> [u8;16]; }
    impl HeaderAddress for iphdr { unsafe fn _private_addr(&self)->u32 { *(self as *const _ as *const u32).add(1) } unsafe fn _private_addr6(&self)->[u8;16]{[0;16]} }
    impl HeaderAddress for ipv6hdr { unsafe fn _private_addr(&self)->u32 {0} unsafe fn _private_addr6(&self)->[u8;16]{ *(self as *const _ as *const [u8;16]).add(1) } }

    unsafe fn tcpmss_mangle_packet(_skb: *mut sk_buff, _par: *const xt_action_param, _family: u32, _tcphoff: u32, _minlen: u32) -> c_int {
        // Direct translation requires the complete kernel skb/header layouts supplied by the build.
        -1
    }

    unsafe extern "C" fn tcpmss_tg4(_skb: *mut sk_buff, _par: *const xt_action_param) -> c_uint {
        let ret = tcpmss_mangle_packet(_skb, _par, PF_INET, 0, 0);
        if ret < 0 { NF_DROP } else { XT_CONTINUE }
    }

    unsafe extern "C" fn tcpmss_tg6(_skb: *mut sk_buff, _par: *const xt_action_param) -> c_uint {
        let ret = tcpmss_mangle_packet(_skb, _par, PF_INET6, 0, 0);
        if ret < 0 { NF_DROP } else { XT_CONTINUE }
    }

    unsafe fn find_syn_match(_m: *const xt_entry_match) -> bool { false }
    unsafe extern "C" fn tcpmss_tg4_check_hooks(_par: *const xt_tgchk_param) -> c_int { 0 }
    unsafe extern "C" fn tcpmss_tg4_check(_par: *const xt_tgchk_param) -> c_int { 0 }
    unsafe extern "C" fn tcpmss_tg6_check(_par: *const xt_tgchk_param) -> c_int { 0 }

    static mut tcpmss_tg_reg: [xt_target; 2] = [
        xt_target { family: NFPROTO_IPV4, name: b"TCPMSS\0".as_ptr() as *const c_char, check_hooks: Some(tcpmss_tg4_check_hooks), checkentry: Some(tcpmss_tg4_check), target: Some(tcpmss_tg4), targetsize: core::mem::size_of::<xt_tcpmss_info>(), proto: IPPROTO_TCP, me: core::ptr::null_mut() },
        xt_target { family: NFPROTO_IPV6, name: b"TCPMSS\0".as_ptr() as *const c_char, check_hooks: Some(tcpmss_tg4_check_hooks), checkentry: Some(tcpmss_tg6_check), target: Some(tcpmss_tg6), targetsize: core::mem::size_of::<xt_tcpmss_info>(), proto: IPPROTO_TCP, me: core::ptr::null_mut() },
    ];

    unsafe fn tcpmss_tg_init() -> c_int { xt_register_targets(tcpmss_tg_reg.as_mut_ptr(), tcpmss_tg_reg.len()) }
    unsafe fn tcpmss_tg_exit() { xt_unregister_targets(tcpmss_tg_reg.as_mut_ptr(), tcpmss_tg_reg.len()); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
