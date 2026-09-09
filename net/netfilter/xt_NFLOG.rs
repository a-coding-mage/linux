// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2006 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, linux/init.h, linux/skbuff.h,
// linux/netfilter/x_tables.h, linux/netfilter/xt_NFLOG.h,
// net/netfilter/nf_log.h

// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("Xtables: packet logging to netlink using NFLOG");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_NFLOG");
// MODULE_ALIAS("ip6t_NFLOG");

unsafe fn nflog_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const xt_nflog_info = unsafe { (*par).targinfo as *const xt_nflog_info };
    let net: *mut net = unsafe { xt_net(par) };
    let mut li: nf_loginfo = unsafe { core::mem::zeroed() };

    unsafe {
        (*li.u.ulog_mut()).copy_len = (*info).len;
        (*li.u.ulog_mut()).group = (*info).group;
        (*li.u.ulog_mut()).qthreshold = (*info).threshold;
        (*li.u.ulog_mut()).flags = 0;
    }

    if unsafe { (*info).flags & XT_NFLOG_F_COPY_LEN } != 0 {
        unsafe { (*li.u.ulog_mut()).flags |= NF_LOG_F_COPY_LEN; }
    }

    unsafe {
        nf_log_packet(
            net,
            xt_family(par),
            xt_hooknum(par),
            skb,
            xt_in(par),
            xt_out(par),
            &li,
            c"%s".as_ptr(),
            (*info).prefix.as_ptr(),
        );
    }

    XT_CONTINUE
}

unsafe fn nflog_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info: *const xt_nflog_info = unsafe { (*par).targinfo as *const xt_nflog_info };
    let mut ret: c_int;

    if unsafe { (*info).flags & !XT_NFLOG_MASK } != 0 {
        return -EINVAL;
    }
    if unsafe { (*info).prefix[core::mem::size_of_val(&(*info).prefix) - 1] } != 0 {
        return -EINVAL;
    }

    ret = unsafe { nf_logger_find_get((*par).family, NF_LOG_TYPE_ULOG) };
    if ret != 0 && unsafe { !(*par).nft_compat } {
        unsafe {
            request_module(c"%s".as_ptr(), c"nfnetlink_log".as_ptr());
            ret = nf_logger_find_get((*par).family, NF_LOG_TYPE_ULOG);
        }
    }

    ret
}

unsafe fn nflog_tg_destroy(par: *const xt_tgdtor_param) {
    unsafe { nf_logger_put((*par).family, NF_LOG_TYPE_ULOG); }
}

static mut nflog_tg_reg: [xt_target; 2] = [xt_target {
    name: *b"NFLOG\0",
    revision: 0,
    family: NFPROTO_IPV4,
    checkentry: Some(nflog_tg_check),
    destroy: Some(nflog_tg_destroy),
    target: Some(nflog_tg),
    targetsize: core::mem::size_of::<xt_nflog_info>(),
    me: THIS_MODULE,
}, #[cfg(CONFIG_IP6_NF_IPTABLES)] xt_target {
    name: *b"NFLOG\0",
    revision: 0,
    family: NFPROTO_IPV6,
    checkentry: Some(nflog_tg_check),
    destroy: Some(nflog_tg_destroy),
    target: Some(nflog_tg),
    targetsize: core::mem::size_of::<xt_nflog_info>(),
    me: THIS_MODULE,
}];

unsafe fn nflog_tg_init() -> c_int {
    xt_register_targets(
        nflog_tg_reg.as_mut_ptr(),
        nflog_tg_reg.len(),
    )
}

unsafe fn nflog_tg_exit() {
    xt_unregister_targets(
        nflog_tg_reg.as_mut_ptr(),
        nflog_tg_reg.len(),
    );
}

// module_init(nflog_tg_init);
// module_exit(nflog_tg_exit);
// MODULE_SOFTDEP("pre: nfnetlink_log");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
