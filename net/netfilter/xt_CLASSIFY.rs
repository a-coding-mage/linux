// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is a module which is used for setting the skb->priority field
 * of an skb for qdisc classification.
 */

/* (C) 2001-2002 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the kernel and netfilter integration.

module_author!("Patrick McHardy <kaber@trash.net>");
module_license!("GPL");
module_description!("Xtables: Qdisc classification");
module_alias!("ipt_CLASSIFY");
module_alias!("ip6t_CLASSIFY");
module_alias!("arpt_CLASSIFY");

unsafe fn classify_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let clinfo = (*par).targinfo as *const xt_classify_target_info;

    (*skb).priority = (*clinfo).priority;
    XT_CONTINUE
}

static mut classify_tg_reg: [xt_target; 3] = [
    xt_target {
        name: *b"CLASSIFY\0",
        revision: 0,
        family: NFPROTO_IPV4,
        hooks: (1 << NF_INET_LOCAL_OUT)
            | (1 << NF_INET_FORWARD)
            | (1 << NF_INET_POST_ROUTING),
        target: Some(classify_tg),
        targetsize: core::mem::size_of::<xt_classify_target_info>(),
        me: THIS_MODULE,
    },
    xt_target {
        name: *b"CLASSIFY\0",
        revision: 0,
        family: NFPROTO_ARP,
        hooks: (1 << NF_ARP_OUT) | (1 << NF_ARP_FORWARD),
        target: Some(classify_tg),
        targetsize: core::mem::size_of::<xt_classify_target_info>(),
        me: THIS_MODULE,
    },
    // Preserved from CONFIG_IP6_NF_IPTABLES conditional compilation.
    #[cfg(feature = "CONFIG_IP6_NF_IPTABLES")]
    xt_target {
        name: *b"CLASSIFY\0",
        revision: 0,
        family: NFPROTO_IPV6,
        hooks: (1 << NF_INET_LOCAL_OUT)
            | (1 << NF_INET_FORWARD)
            | (1 << NF_INET_POST_ROUTING),
        target: Some(classify_tg),
        targetsize: core::mem::size_of::<xt_classify_target_info>(),
        me: THIS_MODULE,
    },
];

unsafe fn classify_tg_init() -> i32 {
    xt_register_targets(
        classify_tg_reg.as_mut_ptr(),
        classify_tg_reg.len(),
    )
}

unsafe fn classify_tg_exit() {
    xt_unregister_targets(
        classify_tg_reg.as_mut_ptr(),
        classify_tg_reg.len(),
    );
}

module_init!(classify_tg_init);
module_exit!(classify_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
