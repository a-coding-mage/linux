// SPDX-License-Identifier: GPL-2.0-only
/*
 * A module for stripping a specific TCP option from TCP packets.
 *
 * Copyright (C) 2007 Sven Schnelle <svens@bitebene.org>
 * Copyright © CC Computer Consultants GmbH, 2007
 */

// Linux kernel headers and symbols are supplied by the surrounding repository.

#[inline]
unsafe fn optlen(opt: *const u8, offset: usize) -> u32 {
    /* Beware zero-length options: make finite progress */
    if *opt.add(offset) <= TCPOPT_NOP || *opt.add(offset + 1) == 0 {
        1
    } else {
        *opt.add(offset + 1) as u32
    }
}

unsafe fn tcpoptstrip_mangle_packet(
    skb: *mut sk_buff,
    par: *const xt_action_param,
    tcphoff: u32,
) -> u32 {
    let info = (*par).targinfo as *const xt_tcpoptstrip_target_info;
    let mut _th: tcphdr = core::mem::zeroed();
    let mut optl: u32;
    let mut i: u32;
    let mut j: u32;
    let mut n: u16;
    let mut o: u16;
    let mut opt: *mut u8;
    let tcp_hdrlen: i32;

    /* This is a fragment, no TCP header is available */
    if (*par).fragoff != 0 {
        return XT_CONTINUE;
    }

    let mut tcph = skb_header_pointer(
        skb,
        tcphoff,
        core::mem::size_of::<tcphdr>(),
        &mut _th as *mut tcphdr as *mut core::ffi::c_void,
    ) as *mut tcphdr;
    if tcph.is_null() {
        return NF_DROP;
    }

    tcp_hdrlen = ((*tcph).doff as i32) * 4;
    if tcp_hdrlen < core::mem::size_of::<tcphdr>() as i32 {
        return NF_DROP;
    }

    if skb_ensure_writable(skb, tcphoff + tcp_hdrlen as u32) != 0 {
        return NF_DROP;
    }

    /* must reload tcph, might have been moved */
    tcph = (skb_network_header(skb).add(tcphoff as usize)) as *mut tcphdr;
    opt = tcph as *mut u8;

    /*
     * Walk through all TCP options - if we find some option to remove,
     * set all octets to %TCPOPT_NOP and adjust checksum.
     */
    i = core::mem::size_of::<tcphdr>() as u32;
    while i < tcp_hdrlen as u32 - 1 {
        optl = optlen(opt, i as usize);

        if i + optl > tcp_hdrlen as u32 {
            break;
        }

        if !tcpoptstrip_test_bit((*info).strip_bmap, *opt.add(i as usize)) {
            i += optl;
            continue;
        }

        j = 0;
        while j < optl {
            o = *opt.add((i + j) as usize) as u16;
            n = TCPOPT_NOP as u16;
            if (i + j) % 2 == 0 {
                o <<= 8;
                n <<= 8;
            }
            inet_proto_csum_replace2(
                &mut (*tcph).check,
                skb,
                htons(o),
                htons(n),
                false,
            );
            j += 1;
        }
        memset(opt.add(i as usize), TCPOPT_NOP as i32, optl as usize);
        i += optl;
    }

    XT_CONTINUE
}

unsafe fn tcpoptstrip_tg4(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    tcpoptstrip_mangle_packet(skb, par, ip_hdrlen(skb))
}

// Preserved conditional intent: this function and IPv6 registration exist
// when CONFIG_IP6_NF_IPTABLES is enabled.
#[cfg(feature = "CONFIG_IP6_NF_IPTABLES")]
unsafe fn tcpoptstrip_tg6(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let ipv6h = ipv6_hdr(skb);
    let mut nexthdr = (*ipv6h).nexthdr;
    let mut frag_off: __be16 = 0;
    let tcphoff = ipv6_skip_exthdr(
        skb,
        core::mem::size_of::<ipv6hdr>() as i32,
        &mut nexthdr,
        &mut frag_off,
    );
    if tcphoff < 0 {
        return NF_DROP;
    }
    tcpoptstrip_mangle_packet(skb, par, tcphoff as u32)
}

static mut tcpoptstrip_tg_reg: [xt_target; 2] = [
    xt_target {
        name: *b"TCPOPTSTRIP\0",
        family: NFPROTO_IPV4,
        table: *b"mangle\0\0",
        proto: IPPROTO_TCP,
        target: Some(tcpoptstrip_tg4),
        targetsize: core::mem::size_of::<xt_tcpoptstrip_target_info>(),
        me: THIS_MODULE,
    },
    // IPv6 registration is conditionally supplied by the kernel build.
    xt_target {
        name: *b"TCPOPTSTRIP\0",
        family: NFPROTO_IPV6,
        table: *b"mangle\0\0",
        proto: IPPROTO_TCP,
        target: Some(tcpoptstrip_tg6),
        targetsize: core::mem::size_of::<xt_tcpoptstrip_target_info>(),
        me: THIS_MODULE,
    },
];

unsafe fn tcpoptstrip_tg_init() -> i32 {
    xt_register_targets(
        tcpoptstrip_tg_reg.as_mut_ptr(),
        tcpoptstrip_tg_reg.len(),
    )
}

unsafe fn tcpoptstrip_tg_exit() {
    xt_unregister_targets(
        tcpoptstrip_tg_reg.as_mut_ptr(),
        tcpoptstrip_tg_reg.len(),
    );
}

module_init!(tcpoptstrip_tg_init);
module_exit!(tcpoptstrip_tg_exit);
module_author!("Sven Schnelle <svens@bitebene.org>, Jan Engelhardt <jengelh@medozas.de>");
module_description!("Xtables: TCP option stripping");
module_license!("GPL");
module_alias!("ipt_TCPOPTSTRIP");
module_alias!("ip6t_TCPOPTSTRIP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
