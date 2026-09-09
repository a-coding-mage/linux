// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is a module which is used for rejecting packets.
 */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux and Netfilter headers are supplied by external dependencies.

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Netfilter Core Team <coreteam@netfilter.org>");
MODULE_DESCRIPTION!("Xtables: packet \"rejection\" target for IPv4");

unsafe fn reject_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let reject = (*par).targinfo as *const ipt_reject_info;
    let hook: c_int = xt_hooknum(par);

    match (*reject).with {
        IPT_ICMP_NET_UNREACHABLE => {
            nf_send_unreach(skb, ICMP_NET_UNREACH, hook);
        }
        IPT_ICMP_HOST_UNREACHABLE => {
            nf_send_unreach(skb, ICMP_HOST_UNREACH, hook);
        }
        IPT_ICMP_PROT_UNREACHABLE => {
            nf_send_unreach(skb, ICMP_PROT_UNREACH, hook);
        }
        IPT_ICMP_PORT_UNREACHABLE => {
            nf_send_unreach(skb, ICMP_PORT_UNREACH, hook);
        }
        IPT_ICMP_NET_PROHIBITED => {
            nf_send_unreach(skb, ICMP_NET_ANO, hook);
        }
        IPT_ICMP_HOST_PROHIBITED => {
            nf_send_unreach(skb, ICMP_HOST_ANO, hook);
        }
        IPT_ICMP_ADMIN_PROHIBITED => {
            nf_send_unreach(skb, ICMP_PKT_FILTERED, hook);
        }
        IPT_TCP_RESET => {
            nf_send_reset(xt_net(par), (*par).state.sk, skb, hook);
        }
        IPT_ICMP_ECHOREPLY => {
            /* Doesn't happen. */
        }
        _ => {}
    }

    NF_DROP
}

unsafe fn reject_tg_check(par: *const xt_tgchk_param) -> c_int {
    let rejinfo = (*par).targinfo as *const ipt_reject_info;
    let e = (*par).entryinfo as *const ipt_entry;

    if (*rejinfo).with == IPT_ICMP_ECHOREPLY {
        pr_info_ratelimited!("ECHOREPLY no longer supported.\n");
        return -EINVAL;
    } else if (*rejinfo).with == IPT_TCP_RESET {
        /* Must specify that it's a TCP packet */
        if (*e).ip.proto != IPPROTO_TCP
            || ((*e).ip.invflags & XT_INV_PROTO) != 0
        {
            pr_info_ratelimited!("TCP_RESET invalid for non-tcp\n");
            return -EINVAL;
        }
    }
    0
}

static mut reject_tg_reg: xt_target = xt_target {
    name: b"REJECT\0".as_ptr() as *const c_char,
    family: NFPROTO_IPV4,
    target: Some(reject_tg),
    targetsize: core::mem::size_of::<ipt_reject_info>(),
    table: b"filter\0".as_ptr() as *const c_char,
    hooks: (1 << NF_INET_LOCAL_IN)
        | (1 << NF_INET_FORWARD)
        | (1 << NF_INET_LOCAL_OUT),
    checkentry: Some(reject_tg_check),
    me: THIS_MODULE,
};

unsafe extern "C" fn reject_tg_init() -> c_int {
    xt_register_target(&mut reject_tg_reg)
}

unsafe extern "C" fn reject_tg_exit() {
    xt_unregister_target(&mut reject_tg_reg);
}

module_init!(reject_tg_init);
module_exit!(reject_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
