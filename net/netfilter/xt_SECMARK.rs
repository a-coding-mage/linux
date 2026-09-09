// SPDX-License-Identifier: GPL-2.0-only
/*
 * Module for modifying the secmark field of the skb, for use by
 * security subsystems.
 *
 * Based on the nfmark match by:
 * (C) 1999-2001 Marc Boucher <marc@mbsi.ca>
 *
 * (C) 2006,2008 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies are supplied by the surrounding kernel translation.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("James Morris <jmorris@redhat.com>");
// MODULE_DESCRIPTION("Xtables: packet security mark modification");
// MODULE_ALIAS("ipt_SECMARK");
// MODULE_ALIAS("ip6t_SECMARK");

static mut mode: u8 = 0;

unsafe fn secmark_tg(
    skb: *mut sk_buff,
    info: *const xt_secmark_target_info_v1,
) -> u32 {
    let mut secmark: u32 = 0;

    match mode {
        SECMARK_MODE_SEL => {
            secmark = (*info).secid;
        }
        _ => {
            BUG!();
        }
    }

    (*skb).secmark = secmark;
    XT_CONTINUE
}

unsafe fn checkentry_lsm(info: *mut xt_secmark_target_info_v1) -> i32 {
    let mut err: i32;

    (*info).secctx[SECMARK_SECCTX_MAX - 1] = b'\0';
    (*info).secid = 0;

    err = security_secctx_to_secid(
        (*info).secctx.as_ptr(),
        strlen((*info).secctx.as_ptr()),
        &mut (*info).secid,
    );
    if err != 0 {
        if err == -EINVAL {
            pr_info_ratelimited!("invalid security context \'%s\'\n", (*info).secctx);
        }
        return err;
    }

    if (*info).secid == 0 {
        pr_info_ratelimited!("unable to map security context \'%s\'\n", (*info).secctx);
        return -ENOENT;
    }

    err = security_secmark_relabel_packet((*info).secid);
    if err != 0 {
        pr_info_ratelimited!("unable to obtain relabeling permission\n");
        return err;
    }

    security_secmark_refcount_inc();
    0
}

unsafe fn secmark_tg_check(
    table: *const c_char,
    info: *mut xt_secmark_target_info_v1,
) -> i32 {
    let mut err: i32;

    if strcmp(table, c"mangle".as_ptr()) != 0 && strcmp(table, c"security".as_ptr()) != 0 {
        pr_info_ratelimited!(
            "only valid in \'mangle\' or \'security\' table, not \'%s\'\n",
            table,
        );
        return -EINVAL;
    }

    if mode != 0 && mode != (*info).mode {
        pr_info_ratelimited!(
            "mode already set to %hu cannot mix with rules for mode %hu\n",
            mode,
            (*info).mode,
        );
        return -EINVAL;
    }

    match (*info).mode {
        SECMARK_MODE_SEL => {}
        _ => {
            pr_info_ratelimited!("invalid mode: %hu\n", (*info).mode);
            return -EINVAL;
        }
    }

    err = checkentry_lsm(info);
    if err != 0 {
        return err;
    }

    if mode == 0 {
        mode = (*info).mode;
    }
    0
}

unsafe fn secmark_tg_destroy(_par: *const xt_tgdtor_param) {
    match mode {
        SECMARK_MODE_SEL => {
            security_secmark_refcount_dec();
        }
        _ => {}
    }
}

unsafe fn secmark_tg_check_v0(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *mut xt_secmark_target_info;
    let mut newinfo: xt_secmark_target_info_v1 = core::mem::zeroed();
    newinfo.mode = (*info).mode;
    let ret: i32;

    memcpy(
        newinfo.secctx.as_mut_ptr() as *mut c_void,
        (*info).secctx.as_ptr() as *const c_void,
        SECMARK_SECCTX_MAX,
    );

    ret = secmark_tg_check((*par).table, &mut newinfo);
    (*info).secid = newinfo.secid;

    ret
}

unsafe fn secmark_tg_v0(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_secmark_target_info;
    let mut newinfo: xt_secmark_target_info_v1 = core::mem::zeroed();
    newinfo.secid = (*info).secid;

    secmark_tg(skb, &newinfo)
}

unsafe fn secmark_tg_check_v1(par: *const xt_tgchk_param) -> i32 {
    secmark_tg_check((*par).table, (*par).targinfo as *mut xt_secmark_target_info_v1)
}

unsafe fn secmark_tg_v1(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    secmark_tg(skb, (*par).targinfo as *const xt_secmark_target_info_v1)
}

// `CONFIG_IP6_NF_IPTABLES` conditionally includes the IPv6 entries below.
static mut secmark_tg_reg: [xt_target; 4] = [
    xt_target {
        name: *b"SECMARK\0",
        revision: 0,
        family: NFPROTO_IPV4,
        checkentry: Some(secmark_tg_check_v0),
        destroy: Some(secmark_tg_destroy),
        target: Some(secmark_tg_v0),
        targetsize: core::mem::size_of::<xt_secmark_target_info>(),
        usersize: 0,
        me: THIS_MODULE,
    },
    xt_target {
        name: *b"SECMARK\0",
        revision: 1,
        family: NFPROTO_IPV4,
        checkentry: Some(secmark_tg_check_v1),
        destroy: Some(secmark_tg_destroy),
        target: Some(secmark_tg_v1),
        targetsize: core::mem::size_of::<xt_secmark_target_info_v1>(),
        usersize: core::mem::offset_of!(xt_secmark_target_info_v1, secid),
        me: THIS_MODULE,
    },
    // IPv6 entries are present when CONFIG_IP6_NF_IPTABLES is enabled.
    xt_target {
        name: *b"SECMARK\0",
        revision: 0,
        family: NFPROTO_IPV6,
        checkentry: Some(secmark_tg_check_v0),
        destroy: Some(secmark_tg_destroy),
        target: Some(secmark_tg_v0),
        targetsize: core::mem::size_of::<xt_secmark_target_info>(),
        usersize: 0,
        me: THIS_MODULE,
    },
    xt_target {
        name: *b"SECMARK\0",
        revision: 1,
        family: NFPROTO_IPV6,
        checkentry: Some(secmark_tg_check_v1),
        destroy: Some(secmark_tg_destroy),
        target: Some(secmark_tg_v1),
        targetsize: core::mem::size_of::<xt_secmark_target_info_v1>(),
        usersize: core::mem::offset_of!(xt_secmark_target_info_v1, secid),
        me: THIS_MODULE,
    },
];

unsafe fn secmark_tg_init() -> i32 {
    xt_register_targets(secmark_tg_reg.as_mut_ptr(), secmark_tg_reg.len())
}

unsafe fn secmark_tg_exit() {
    xt_unregister_targets(secmark_tg_reg.as_mut_ptr(), secmark_tg_reg.len());
}

// module_init(secmark_tg_init);
// module_exit(secmark_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
