// SPDX-License-Identifier: GPL-2.0-only
// C dependencies supplied by the surrounding kernel build are intentionally
// referenced here rather than reimplemented.

#[inline]
unsafe fn port_match(min: u16, max: u16, port: u16, invert: bool) -> bool {
    ((port >= min && port <= max) ^ invert)
}

unsafe fn tcp_find_option(
    option: u8,
    skb: *const sk_buff,
    protoff: u32,
    optlen: u32,
    invert: bool,
    hotdrop: *mut bool,
) -> bool {
    let mut opt = [0u8; 40];
    let mut i: u32;

    if optlen == 0 {
        return invert;
    }
    let op = skb_header_pointer(
        skb,
        protoff + core::mem::size_of::<tcphdr>() as u32,
        optlen,
        opt.as_mut_ptr() as *mut core::ffi::c_void,
    ) as *const u8;
    if op.is_null() {
        *hotdrop = true;
        return false;
    }

    i = 0;
    while i < optlen {
        if *op.add(i as usize) == option {
            return !invert;
        }
        if *op.add(i as usize) < 2 || i == optlen - 1 {
            i += 1;
        } else {
            let n = *op.add((i + 1) as usize);
            i += if n != 0 { n as u32 } else { 1 };
        }
    }
    invert
}

unsafe fn tcp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    if (*par).fragoff != 0 {
        if (*par).fragoff == 1 { (*par).hotdrop = true; }
        return false;
    }
    let mut tcph = core::mem::zeroed::<tcphdr>();
    let th = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<tcphdr>() as u32,
                                &mut tcph as *mut _ as *mut core::ffi::c_void) as *const tcphdr;
    if th.is_null() { (*par).hotdrop = true; return false; }
    let tcpinfo = (*par).matchinfo as *const xt_tcp;
    if !port_match((*tcpinfo).spts[0], (*tcpinfo).spts[1], u16::from_be((*th).source),
                   ((*tcpinfo).invflags & XT_TCP_INV_SRCPT) != 0) { return false; }
    if !port_match((*tcpinfo).dpts[0], (*tcpinfo).dpts[1], u16::from_be((*th).dest),
                   ((*tcpinfo).invflags & XT_TCP_INV_DSTPT) != 0) { return false; }
    let flags = *((th as *const u8).add(13));
    let matched = (flags & (*tcpinfo).flg_mask) == (*tcpinfo).flg_cmp;
    if !(NF_INVF(tcpinfo, XT_TCP_INV_FLAGS, matched)) { return false; }
    if (*tcpinfo).option != 0 {
        if ((*th).doff as usize * 4) < core::mem::size_of::<tcphdr>() { (*par).hotdrop = true; return false; }
        if !tcp_find_option((*tcpinfo).option, skb, (*par).thoff,
                            (*th).doff as u32 * 4 - core::mem::size_of::<tcphdr>() as u32,
                            ((*tcpinfo).invflags & XT_TCP_INV_OPTION) != 0,
                            &mut (*par).hotdrop) { return false; }
    }
    true
}

unsafe fn tcp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_tcp;
    if (*info).invflags & !XT_TCP_INV_MASK != 0 { -EINVAL } else { 0 }
}

unsafe fn udp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    if (*par).fragoff != 0 { return false; }
    let mut udph = core::mem::zeroed::<udphdr>();
    let uh = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<udphdr>() as u32,
                                &mut udph as *mut _ as *mut core::ffi::c_void) as *const udphdr;
    if uh.is_null() { (*par).hotdrop = true; return false; }
    let info = (*par).matchinfo as *const xt_udp;
    port_match((*info).spts[0], (*info).spts[1], u16::from_be((*uh).source),
               ((*info).invflags & XT_UDP_INV_SRCPT) != 0) &&
    port_match((*info).dpts[0], (*info).dpts[1], u16::from_be((*uh).dest),
               ((*info).invflags & XT_UDP_INV_DSTPT) != 0)
}

unsafe fn udp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_udp;
    if (*info).invflags & !XT_UDP_INV_MASK != 0 { -EINVAL } else { 0 }
}

unsafe fn type_code_in_range(test_type: u8, min_code: u8, max_code: u8, typ: u8, code: u8) -> bool {
    typ == test_type && code >= min_code && code <= max_code
}

unsafe fn icmp_type_code_match(test_type: u8, min_code: u8, max_code: u8, typ: u8, code: u8, invert: bool) -> bool {
    ((test_type == 0xff || type_code_in_range(test_type, min_code, max_code, typ, code)) ^ invert)
}

unsafe fn icmp6_type_code_match(test_type: u8, min_code: u8, max_code: u8, typ: u8, code: u8, invert: bool) -> bool {
    type_code_in_range(test_type, min_code, max_code, typ, code) ^ invert
}

unsafe fn icmp_match(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    if (*par).fragoff != 0 { return false; }
    let mut hdr = core::mem::zeroed::<icmphdr>();
    let ic = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<icmphdr>() as u32,
                                &mut hdr as *mut _ as *mut core::ffi::c_void) as *const icmphdr;
    if ic.is_null() { (*par).hotdrop = true; return false; }
    let info = (*par).matchinfo as *const ipt_icmp;
    icmp_type_code_match((*info).type_, (*info).code[0], (*info).code[1], (*ic).type_, (*ic).code,
                         ((*info).invflags & IPT_ICMP_INV) != 0)
}

unsafe fn icmp6_match(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    if (*par).fragoff != 0 { return false; }
    let mut hdr = core::mem::zeroed::<icmp6hdr>();
    let ic = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<icmp6hdr>() as u32,
                                &mut hdr as *mut _ as *mut core::ffi::c_void) as *const icmp6hdr;
    if ic.is_null() { (*par).hotdrop = true; return false; }
    let info = (*par).matchinfo as *const ip6t_icmp;
    icmp6_type_code_match((*info).type_, (*info).code[0], (*info).code[1], (*ic).icmp6_type,
                          (*ic).icmp6_code, ((*info).invflags & IP6T_ICMP_INV) != 0)
}

unsafe fn icmp_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ipt_icmp;
    if (*info).invflags & !IPT_ICMP_INV != 0 { -EINVAL } else { 0 }
}

unsafe fn icmp6_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ip6t_icmp;
    if (*info).invflags & !IP6T_ICMP_INV != 0 { -EINVAL } else { 0 }
}

// Kernel-provided types and constants referenced below are intentionally not
// redefined in this translation unit.
static mut tcpudp_mt_reg: [xt_match; 8] = [
    xt_match { name: *b"tcp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV4, checkentry: Some(tcp_mt_check), match_: Some(tcp_mt), matchsize: core::mem::size_of::<xt_tcp>(), proto: IPPROTO_TCP, me: THIS_MODULE },
    xt_match { name: *b"tcp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV6, checkentry: Some(tcp_mt_check), match_: Some(tcp_mt), matchsize: core::mem::size_of::<xt_tcp>(), proto: IPPROTO_TCP, me: THIS_MODULE },
    xt_match { name: *b"udp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV4, checkentry: Some(udp_mt_check), match_: Some(udp_mt), matchsize: core::mem::size_of::<xt_udp>(), proto: IPPROTO_UDP, me: THIS_MODULE },
    xt_match { name: *b"udp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV6, checkentry: Some(udp_mt_check), match_: Some(udp_mt), matchsize: core::mem::size_of::<xt_udp>(), proto: IPPROTO_UDP, me: THIS_MODULE },
    xt_match { name: *b"udplite\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV4, checkentry: Some(udp_mt_check), match_: Some(udp_mt), matchsize: core::mem::size_of::<xt_udp>(), proto: IPPROTO_UDPLITE, me: THIS_MODULE },
    xt_match { name: *b"udplite\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV6, checkentry: Some(udp_mt_check), match_: Some(udp_mt), matchsize: core::mem::size_of::<xt_udp>(), proto: IPPROTO_UDPLITE, me: THIS_MODULE },
    xt_match { name: *b"icmp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV4, checkentry: Some(icmp_checkentry), match_: Some(icmp_match), matchsize: core::mem::size_of::<ipt_icmp>(), proto: IPPROTO_ICMP, me: THIS_MODULE },
    xt_match { name: *b"icmp6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", family: NFPROTO_IPV6, checkentry: Some(icmp6_checkentry), match_: Some(icmp6_match), matchsize: core::mem::size_of::<ip6t_icmp>(), proto: IPPROTO_ICMPV6, me: THIS_MODULE },
];

extern "C" {
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
}

unsafe fn tcpudp_mt_init() -> i32 {
    xt_register_matches(tcpudp_mt_reg.as_mut_ptr(), tcpudp_mt_reg.len())
}

unsafe fn tcpudp_mt_exit() {
    xt_unregister_matches(tcpudp_mt_reg.as_mut_ptr(), tcpudp_mt_reg.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
