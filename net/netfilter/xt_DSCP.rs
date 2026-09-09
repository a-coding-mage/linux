// SPDX-License-Identifier: GPL-2.0-only
/* x_tables module for setting the IPv4/IPv6 DSCP field, Version 1.8
 *
 * (C) 2002 by Harald Welte <laforge@netfilter.org>
 * based on ipt_FTOS.c (C) 2000 by Matthew G. Marsh <mgm@paktronix.com>
 *
 * See RFC2474 for a description of the DSCP field within the IP Header.
 */

// Kernel and x_tables declarations are supplied by the surrounding crate.

const XT_DSCP_ECN_MASK: u8 = 3u8;

unsafe fn dscp_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let dinfo = (*par).targinfo as *const xt_DSCP_info;
    let dscp: u8 = ipv4_get_dsfield(ip_hdr(skb)) >> XT_DSCP_SHIFT;

    if dscp != (*dinfo).dscp {
        if skb_ensure_writable(skb, core::mem::size_of::<iphdr>()) != 0 {
            return NF_DROP;
        }

        ipv4_change_dsfield(
            ip_hdr(skb),
            XT_DSCP_ECN_MASK,
            (*dinfo).dscp << XT_DSCP_SHIFT,
        );
    }
    XT_CONTINUE
}

unsafe fn dscp_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let dinfo = (*par).targinfo as *const xt_DSCP_info;
    let dscp: u8 = ipv6_get_dsfield(ipv6_hdr(skb)) >> XT_DSCP_SHIFT;

    if dscp != (*dinfo).dscp {
        if skb_ensure_writable(skb, core::mem::size_of::<ipv6hdr>()) != 0 {
            return NF_DROP;
        }

        ipv6_change_dsfield(
            ipv6_hdr(skb),
            XT_DSCP_ECN_MASK,
            (*dinfo).dscp << XT_DSCP_SHIFT,
        );
    }
    XT_CONTINUE
}

unsafe fn dscp_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const xt_DSCP_info;

    if (*info).dscp > XT_DSCP_MAX {
        return -EDOM;
    }
    0
}

unsafe fn tos_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_tos_target_info;
    let mut iph = ip_hdr(skb);
    let orig: u8 = ipv4_get_dsfield(iph);
    let nv: u8 = (orig & !(*info).tos_mask) ^ (*info).tos_value;

    if orig != nv {
        if skb_ensure_writable(skb, core::mem::size_of::<iphdr>()) != 0 {
            return NF_DROP;
        }
        iph = ip_hdr(skb);
        ipv4_change_dsfield(iph, 0, nv);
    }

    XT_CONTINUE
}

unsafe fn tos_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_tos_target_info;
    let mut iph = ipv6_hdr(skb);
    let orig: u8 = ipv6_get_dsfield(iph);
    let nv: u8 = (orig & !(*info).tos_mask) ^ (*info).tos_value;

    if orig != nv {
        if skb_ensure_writable(skb, core::mem::size_of::<iphdr>()) != 0 {
            return NF_DROP;
        }
        iph = ipv6_hdr(skb);
        ipv6_change_dsfield(iph, 0, nv);
    }

    XT_CONTINUE
}

static mut dscp_tg_reg: [xt_target; 4] = [
    xt_target {
        name: *b"DSCP\0\0\0\0",
        family: NFPROTO_IPV4,
        checkentry: Some(dscp_tg_check),
        target: Some(dscp_tg),
        targetsize: core::mem::size_of::<xt_DSCP_info>(),
        table: *b"mangle\0\0\0\0\0\0",
        me: THIS_MODULE,
        ..xt_target::ZERO
    },
    xt_target {
        name: *b"DSCP\0\0\0\0",
        family: NFPROTO_IPV6,
        checkentry: Some(dscp_tg_check),
        target: Some(dscp_tg6),
        targetsize: core::mem::size_of::<xt_DSCP_info>(),
        table: *b"mangle\0\0\0\0\0\0",
        me: THIS_MODULE,
        ..xt_target::ZERO
    },
    xt_target {
        name: *b"TOS\0\0\0\0\0",
        revision: 1,
        family: NFPROTO_IPV4,
        table: *b"mangle\0\0\0\0\0\0",
        target: Some(tos_tg),
        targetsize: core::mem::size_of::<xt_tos_target_info>(),
        me: THIS_MODULE,
        ..xt_target::ZERO
    },
    xt_target {
        name: *b"TOS\0\0\0\0\0",
        revision: 1,
        family: NFPROTO_IPV6,
        table: *b"mangle\0\0\0\0\0\0",
        target: Some(tos_tg6),
        targetsize: core::mem::size_of::<xt_tos_target_info>(),
        me: THIS_MODULE,
        ..xt_target::ZERO
    },
];

unsafe fn dscp_tg_init() -> c_int {
    xt_register_targets(dscp_tg_reg.as_mut_ptr(), dscp_tg_reg.len())
}

unsafe fn dscp_tg_exit() {
    xt_unregister_targets(dscp_tg_reg.as_mut_ptr(), dscp_tg_reg.len());
}

// module_init(dscp_tg_init);
// module_exit(dscp_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
