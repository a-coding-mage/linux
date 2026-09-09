// SPDX-License-Identifier: GPL-2.0-only
/* iptables module for the packet checksum mangling
 *
 * (C) 2002 by Harald Welte <laforge@netfilter.org>
 * (C) 2010 Red Hat, Inc.
 *
 * Author: Michael S. Tsirkin <mst@redhat.com>
 */
// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.
// External Linux kernel declarations and constants are supplied by dependencies.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Michael S. Tsirkin <mst@redhat.com>");
// MODULE_DESCRIPTION("Xtables: checksum modification");
// MODULE_ALIAS("ipt_CHECKSUM");
// MODULE_ALIAS("ip6t_CHECKSUM");

unsafe fn checksum_tg(
    skb: *mut sk_buff,
    _par: *const xt_action_param,
) -> c_uint {
    unsafe {
        if (*skb).ip_summed == CHECKSUM_PARTIAL && !skb_is_gso(skb) {
            skb_checksum_help(skb);
        }
    }

    XT_CONTINUE
}

unsafe fn checksum_tg_check(par: *const xt_tgchk_param) -> c_int {
    unsafe {
        let einfo = (*par).targinfo as *const xt_CHECKSUM_info;
        let i6 = (*par).entryinfo as *const ip6t_ip6;
        let i4 = (*par).entryinfo as *const ipt_ip;

        if (*einfo).operation & !XT_CHECKSUM_OP_FILL != 0 {
            pr_info_ratelimited!(
                "unsupported CHECKSUM operation %x\n",
                (*einfo).operation
            );
            return -EINVAL;
        }
        if (*einfo).operation == 0 {
            return -EINVAL;
        }

        match (*par).family {
            NFPROTO_IPV4 => {
                if (*i4).proto == IPPROTO_UDP
                    && ((*i4).invflags & XT_INV_PROTO) == 0
                {
                    return 0;
                }
            }
            NFPROTO_IPV6 => {
                if ((*i6).flags & IP6T_F_PROTO) != 0
                    && (*i6).proto == IPPROTO_UDP
                    && ((*i6).invflags & XT_INV_PROTO) == 0
                {
                    return 0;
                }
            }
            _ => {}
        }

        pr_warn_once!(
            "CHECKSUM should be avoided.  If really needed, restrict with \"-p udp\" and only use in OUTPUT\n"
        );
        0
    }
}

static mut checksum_tg_reg: [xt_target; 2] = [
    xt_target {
        name: "CHECKSUM",
        family: NFPROTO_IPV4,
        target: Some(checksum_tg),
        targetsize: core::mem::size_of::<xt_CHECKSUM_info>(),
        table: "mangle",
        checkentry: Some(checksum_tg_check),
        me: THIS_MODULE,
    },
    // Preserved from #if IS_ENABLED(CONFIG_IP6_NF_IPTABLES):
    // this target entry is present when IPv6 iptables support is enabled.
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    xt_target {
        name: "CHECKSUM",
        family: NFPROTO_IPV6,
        target: Some(checksum_tg),
        targetsize: core::mem::size_of::<xt_CHECKSUM_info>(),
        table: "mangle",
        checkentry: Some(checksum_tg_check),
        me: THIS_MODULE,
    },
];

unsafe fn checksum_tg_init() -> c_int {
    xt_register_targets(
        checksum_tg_reg.as_mut_ptr(),
        checksum_tg_reg.len(),
    )
}

unsafe fn checksum_tg_exit() {
    xt_unregister_targets(
        checksum_tg_reg.as_mut_ptr(),
        checksum_tg_reg.len(),
    );
}

// module_init(checksum_tg_init);
// module_exit(checksum_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
