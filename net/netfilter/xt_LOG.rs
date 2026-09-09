// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is a module which is used for logging packets.
 */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn log_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let loginfo = (*par).targinfo as *const xt_log_info;
    let net = xt_net(par);
    let mut li: nf_loginfo = core::mem::zeroed();

    li.type_ = NF_LOG_TYPE_LOG;
    (*li.u.log_mut()).level = (*loginfo).level;
    (*li.u.log_mut()).logflags = (*loginfo).logflags;

    nf_log_packet(
        net,
        xt_family(par),
        xt_hooknum(par),
        skb,
        xt_in(par),
        xt_out(par),
        &li,
        c"%s".as_ptr(),
        (*loginfo).prefix.as_ptr(),
    );
    XT_CONTINUE
}

unsafe fn log_tg_check(par: *const xt_tgchk_param) -> c_int {
    let loginfo = (*par).targinfo as *const xt_log_info;
    let mut ret: c_int;

    if (*par).family != NFPROTO_IPV4 && (*par).family != NFPROTO_IPV6 {
        return -EINVAL;
    }

    if (*loginfo).level >= 8 {
        pr_info_ratelimited!(c"level %u >= 8\n", (*loginfo).level);
        return -EINVAL;
    }

    if (*loginfo).prefix[core::mem::size_of_val(&(*loginfo).prefix) - 1] != 0 {
        pr_info_ratelimited!(c"prefix is not null-terminated\n");
        return -EINVAL;
    }

    ret = nf_logger_find_get((*par).family, NF_LOG_TYPE_LOG);
    if ret != 0 && !(*par).nft_compat {
        request_module!(c"%s", c"nf_log_syslog");

        ret = nf_logger_find_get((*par).family, NF_LOG_TYPE_LOG);
    }

    ret
}

unsafe fn log_tg_destroy(par: *const xt_tgdtor_param) {
    nf_logger_put((*par).family, NF_LOG_TYPE_LOG);
}

static mut log_tg_regs: [xt_target; 2] = [
    xt_target {
        name: *b"LOG\0",
        family: NFPROTO_IPV4,
        target: Some(log_tg),
        targetsize: core::mem::size_of::<xt_log_info>(),
        checkentry: Some(log_tg_check),
        destroy: Some(log_tg_destroy),
        me: THIS_MODULE,
    },
    // Preserved from CONFIG_IP6_NF_IPTABLES conditional compilation.
    xt_target {
        name: *b"LOG\0",
        family: NFPROTO_IPV6,
        target: Some(log_tg),
        targetsize: core::mem::size_of::<xt_log_info>(),
        checkentry: Some(log_tg_check),
        destroy: Some(log_tg_destroy),
        me: THIS_MODULE,
    },
];

unsafe fn log_tg_init() -> c_int {
    xt_register_targets(log_tg_regs.as_mut_ptr(), log_tg_regs.len())
}

unsafe fn log_tg_exit() {
    xt_unregister_targets(log_tg_regs.as_mut_ptr(), log_tg_regs.len());
}

module_init!(log_tg_init);
module_exit!(log_tg_exit);

module_license!(c"GPL");
module_author!(c"Netfilter Core Team <coreteam@netfilter.org>");
module_author!(c"Jan Rekorajski <baggins@pld.org.pl>");
module_description!(c"Xtables: IPv4/IPv6 packet logging");
module_alias!(c"ipt_LOG");
module_alias!(c"ip6t_LOG");
module_softdep!(c"pre: nf_log_syslog");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
