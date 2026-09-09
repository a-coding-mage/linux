// SPDX-License-Identifier: GPL-2.0-only
/*
 *	 xt_mark - Netfilter module to match NFMARK value
 *
 *	(C) 1999-2001 Marc Boucher <marc@mbsi.ca>
 *	Copyright © CC Computer Consultants GmbH, 2007 - 2008
 *	Jan Engelhardt <jengelh@medozas.de>
 */

// C dependencies supplied by the surrounding kernel translation unit.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Marc Boucher <marc@mbsi.ca>");
// MODULE_DESCRIPTION("Xtables: packet mark operations");
// MODULE_ALIAS("ipt_mark");
// MODULE_ALIAS("ip6t_mark");
// MODULE_ALIAS("ipt_MARK");
// MODULE_ALIAS("ip6t_MARK");
// MODULE_ALIAS("arpt_MARK");

unsafe fn mark_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const xt_mark_tginfo2 = unsafe { (*par).targinfo as *const xt_mark_tginfo2 };

    unsafe {
        (*skb).mark = ((*skb).mark & !(*info).mask) ^ (*info).mark;
    }
    XT_CONTINUE
}

unsafe fn mark_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const xt_mark_mtinfo1 = unsafe { (*par).matchinfo as *const xt_mark_mtinfo1 };

    unsafe { (((*skb).mark & (*info).mask) == (*info).mark) ^ (*info).invert }
}

static mut mark_tg_reg: [xt_target; 3] = [xt_target {
    name: "MARK",
    revision: 2,
    family: NFPROTO_IPV4,
    target: Some(mark_tg),
    targetsize: core::mem::size_of::<xt_mark_tginfo2>(),
    me: THIS_MODULE,
}, xt_target {
    name: "MARK",
    revision: 2,
    family: NFPROTO_ARP,
    target: Some(mark_tg),
    targetsize: core::mem::size_of::<xt_mark_tginfo2>(),
    me: THIS_MODULE,
}, xt_target {
    name: "MARK",
    revision: 2,
    family: NFPROTO_IPV6,
    target: Some(mark_tg),
    targetsize: core::mem::size_of::<xt_mark_tginfo2>(),
    me: THIS_MODULE,
}];

// In the C source, the NFPROTO_ARP entry is conditional on
// IS_ENABLED(CONFIG_IP_NF_ARPTABLES) || IS_ENABLED(CONFIG_NFT_COMPAT_ARP),
// and the NFPROTO_IPV6 entry is conditional on IS_ENABLED(CONFIG_IP6_NF_IPTABLES).

static mut mark_mt_reg: xt_match = xt_match {
    name: "mark",
    revision: 1,
    family: NFPROTO_UNSPEC,
    r#match: Some(mark_mt),
    matchsize: core::mem::size_of::<xt_mark_mtinfo1>(),
    me: THIS_MODULE,
};

unsafe fn mark_mt_init() -> c_int {
    let mut ret: c_int;

    ret = unsafe {
        xt_register_targets(
            mark_tg_reg.as_mut_ptr(),
            mark_tg_reg.len(),
        )
    };
    if ret < 0 {
        return ret;
    }
    ret = unsafe { xt_register_match(&mut mark_mt_reg) };
    if ret < 0 {
        unsafe {
            xt_unregister_targets(mark_tg_reg.as_mut_ptr(), mark_tg_reg.len());
        }
        return ret;
    }
    0
}

unsafe fn mark_mt_exit() {
    unsafe {
        xt_unregister_match(&mut mark_mt_reg);
        xt_unregister_targets(mark_tg_reg.as_mut_ptr(), mark_tg_reg.len());
    }
}

// module_init(mark_mt_init);
// module_exit(mark_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
