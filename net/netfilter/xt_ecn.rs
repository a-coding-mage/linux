// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xtables module for matching the value of the IPv4/IPv6 and TCP ECN bits
 *
 * (C) 2002 by Harald Welte <laforge@gnumonks.org>
 * (C) 2011 Patrick McHardy <kaber@trash.net>
 */
// Kernel headers and build-time module declarations are supplied by external dependencies.

unsafe fn match_tcp(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let einfo = (*par).matchinfo as *const xt_ecn_info;
    let mut _tcph: tcphdr = core::mem::zeroed();
    let th: *const tcphdr;

    /* this is fine for IPv6 as ecn_mt_check6() enforces -p tcp */
    if (*par).fragoff != 0 {
        return false;
    }

    /* In practice, TCP match does this, so can't fail.  But let's
     * be good citizens.
     */
    th = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<tcphdr>(),
        &mut _tcph as *mut tcphdr as *mut core::ffi::c_void,
    );
    if th.is_null() {
        return false;
    }

    if (*einfo).operation & XT_ECN_OP_MATCH_ECE != 0 {
        if (*einfo).invert & XT_ECN_OP_MATCH_ECE != 0 {
            if (*th).ece == 1 {
                return false;
            }
        } else if (*th).ece == 0 {
            return false;
        }
    }

    if (*einfo).operation & XT_ECN_OP_MATCH_CWR != 0 {
        if (*einfo).invert & XT_ECN_OP_MATCH_CWR != 0 {
            if (*th).cwr == 1 {
                return false;
            }
        } else if (*th).cwr == 0 {
            return false;
        }
    }

    true
}

unsafe fn match_ip(skb: *const sk_buff, einfo: *const xt_ecn_info) -> bool {
    ((ip_hdr(skb).as_ref().unwrap().tos & XT_ECN_IP_MASK) == (*einfo).ip_ect)
        ^ ((*einfo).invert & XT_ECN_OP_MATCH_IP != 0)
}

unsafe extern "C" fn ecn_mt4(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_ecn_info;

    if (*info).operation & XT_ECN_OP_MATCH_IP != 0 && !match_ip(skb, info) {
        return false;
    }
    if (*info).operation & (XT_ECN_OP_MATCH_ECE | XT_ECN_OP_MATCH_CWR) != 0
        && !match_tcp(skb, par)
    {
        return false;
    }
    true
}

unsafe extern "C" fn ecn_mt_check4(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_ecn_info;
    let ip = (*par).entryinfo as *const ipt_ip;

    if (*info).operation & XT_ECN_OP_MATCH_MASK != 0 {
        return -EINVAL;
    }
    if (*info).invert & XT_ECN_OP_MATCH_MASK != 0 {
        return -EINVAL;
    }
    if (*info).operation & (XT_ECN_OP_MATCH_ECE | XT_ECN_OP_MATCH_CWR) != 0
        && ((*ip).proto != IPPROTO_TCP || (*ip).invflags & IPT_INV_PROTO != 0)
    {
        pr_info_ratelimited!("cannot match TCP bits for non-tcp packets\n");
        return -EINVAL;
    }
    0
}

unsafe fn match_ipv6(skb: *const sk_buff, einfo: *const xt_ecn_info) -> bool {
    ((((ipv6_hdr(skb).as_ref().unwrap().flow_lbl[0] >> 4) & XT_ECN_IP_MASK)
        == (*einfo).ip_ect))
        ^ ((*einfo).invert & XT_ECN_OP_MATCH_IP != 0)
}

unsafe extern "C" fn ecn_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_ecn_info;

    if (*info).operation & XT_ECN_OP_MATCH_IP != 0 && !match_ipv6(skb, info) {
        return false;
    }
    if (*info).operation & (XT_ECN_OP_MATCH_ECE | XT_ECN_OP_MATCH_CWR) != 0
        && !match_tcp(skb, par)
    {
        return false;
    }
    true
}

unsafe extern "C" fn ecn_mt_check6(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_ecn_info;
    let ip = (*par).entryinfo as *const ip6t_ip6;

    if (*info).operation & XT_ECN_OP_MATCH_MASK != 0 {
        return -EINVAL;
    }
    if (*info).invert & XT_ECN_OP_MATCH_MASK != 0 {
        return -EINVAL;
    }
    if (*info).operation & (XT_ECN_OP_MATCH_ECE | XT_ECN_OP_MATCH_CWR) != 0
        && ((*ip).proto != IPPROTO_TCP || (*ip).invflags & IP6T_INV_PROTO != 0)
    {
        pr_info_ratelimited!("cannot match TCP bits for non-tcp packets\n");
        return -EINVAL;
    }
    0
}

#[repr(C)]
static mut ecn_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"ecn\0",
        family: NFPROTO_IPV4,
        match_: Some(ecn_mt4),
        matchsize: core::mem::size_of::<xt_ecn_info>(),
        checkentry: Some(ecn_mt_check4),
        me: THIS_MODULE,
    },
    xt_match {
        name: *b"ecn\0",
        family: NFPROTO_IPV6,
        match_: Some(ecn_mt6),
        matchsize: core::mem::size_of::<xt_ecn_info>(),
        checkentry: Some(ecn_mt_check6),
        me: THIS_MODULE,
    },
];

unsafe extern "C" fn ecn_mt_init() -> i32 {
    xt_register_matches(ecn_mt_reg.as_mut_ptr(), ecn_mt_reg.len())
}

unsafe extern "C" fn ecn_mt_exit() {
    xt_unregister_matches(ecn_mt_reg.as_mut_ptr(), ecn_mt_reg.len());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
