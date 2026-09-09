// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match TCP MSS values. */

/* Copyright (C) 2000 Marc Boucher <marc@mbsi.ca>
 * Portions (C) 2005 by Harald Welte <laforge@netfilter.org>
 */

// C dependencies: linux/module.h, linux/skbuff.h, net/tcp.h,
// linux/netfilter/xt_tcpmss.h, linux/netfilter/x_tables.h,
// linux/netfilter_ipv4/ip_tables.h, linux/netfilter_ipv6/ip6_tables.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Marc Boucher <marc@mbsi.ca>");
// MODULE_DESCRIPTION("Xtables: TCP MSS match");
// MODULE_ALIAS("ipt_tcpmss");
// MODULE_ALIAS("ip6t_tcpmss");

unsafe fn tcpmss_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const xt_tcpmss_match_info = (*par).matchinfo as *const xt_tcpmss_match_info;
    let th: *const tcphdr;
    let mut _tcph: tcphdr = core::mem::zeroed();
    /* tcp.doff is only 4 bits, ie. max 15 * 4 bytes */
    let op: *const u8;
    let mut _opt: [u8; 15 * 4 - core::mem::size_of::<tcphdr>()] = [0; 15 * 4 - core::mem::size_of::<tcphdr>()];
    let mut i: usize;
    let optlen: usize;

    /* this is fine for IPv6 as xt_tcpmss enforces -p tcp */
    if (*par).fragoff != 0 {
        return false;
    }

    /* If we don't have the whole header, drop packet. */
    th = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<tcphdr>(), &mut _tcph as *mut _ as *mut core::ffi::c_void);
    if th.is_null() {
        goto_dropit(par);
        return false;
    }

    /* Malformed. */
    if ((*th).doff as usize) * 4 < core::mem::size_of::<tcphdr>() {
        goto_dropit(par);
        return false;
    }

    optlen = ((*th).doff as usize) * 4 - core::mem::size_of::<tcphdr>();
    if optlen == 0 {
        return (*info).invert != 0;
    }

    /* Truncated options. */
    op = skb_header_pointer(skb, (*par).thoff + core::mem::size_of::<tcphdr>(), optlen, _opt.as_mut_ptr() as *mut core::ffi::c_void);
    if op.is_null() {
        goto_dropit(par);
        return false;
    }

    i = 0;
    while i < optlen {
        if *op.add(i) == TCPOPT_MSS
            && (optlen - i) >= TCPOLEN_MSS
            && *op.add(i + 1) == TCPOLEN_MSS
        {
            let mssval: u16 = ((*op.add(i + 2) as u16) << 8) | (*op.add(i + 3) as u16);
            return ((mssval >= (*info).mss_min && mssval <= (*info).mss_max)
                ^ ((*info).invert != 0));
        }
        if *op.add(i) < 2 || i == optlen - 1 {
            i += 1;
        } else {
            i += if *op.add(i + 1) != 0 { *op.add(i + 1) as usize } else { 1 };
        }
    }
    return (*info).invert != 0;
}

unsafe fn goto_dropit(par: *mut xt_action_param) {
    (*par).hotdrop = true;
}

unsafe fn tcpmss_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info: *const xt_tcpmss_match_info = (*par).matchinfo as *const xt_tcpmss_match_info;

    if (*info).mss_min > (*info).mss_max {
        return -EINVAL;
    }
    if (*info).invert > 1 {
        return -EINVAL;
    }

    0
}

static mut tcpmss_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"tcpmss\0",
        family: NFPROTO_IPV4,
        checkentry: Some(tcpmss_mt_check),
        match_: Some(tcpmss_mt),
        matchsize: core::mem::size_of::<xt_tcpmss_match_info>(),
        proto: IPPROTO_TCP,
        me: THIS_MODULE,
    },
    xt_match {
        name: *b"tcpmss\0",
        family: NFPROTO_IPV6,
        checkentry: Some(tcpmss_mt_check),
        match_: Some(tcpmss_mt),
        matchsize: core::mem::size_of::<xt_tcpmss_match_info>(),
        proto: IPPROTO_TCP,
        me: THIS_MODULE,
    },
];

unsafe fn tcpmss_mt_init() -> i32 {
    xt_register_matches(tcpmss_mt_reg.as_mut_ptr(), tcpmss_mt_reg.len())
}

unsafe fn tcpmss_mt_exit() {
    xt_unregister_matches(tcpmss_mt_reg.as_mut_ptr(), tcpmss_mt_reg.len());
}

// module_init(tcpmss_mt_init);
// module_exit(tcpmss_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
