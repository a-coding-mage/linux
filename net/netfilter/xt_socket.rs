// SPDX-License-Identifier: GPL-2.0-only
/*
 * Transparent proxy support for Linux/iptables
 *
 * Copyright (C) 2007-2008 BalaBit IT Ltd.
 * Author: Krisztian Kovacs
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel dependencies are supplied by the surrounding build.

/* "socket" match based redirection (no specific rule)
 * ===================================================
 *
 * There are connections with dynamic endpoints (e.g. FTP data
 * connection) that the user is unable to add explicit rules
 * for. These are taken care of by a generic "socket" rule. It is
 * assumed that the proxy application is trusted to open such
 * connections without explicit iptables rule (except of course the
 * generic 'socket' rule). In this case the following sockets are
 * matched in preference order:
 *
 *   - match: if there's a fully established connection matching the
 *     _packet_ tuple
 *
 *   - match: if there's a non-zero bound listener (possibly with a
 *     non-local address) We don't accept zero-bound listeners, since
 *     then local services could intercept traffic going through the
 *     box.
 */
unsafe fn socket_match(
    skb: *const sk_buff,
    par: *mut xt_action_param,
    info: *const xt_socket_mtinfo1,
) -> bool {
    let pskb = skb as *mut sk_buff;
    let mut sk = (*skb).sk;

    if !sk.is_null() && !net_eq(xt_net(par), sock_net(sk)) {
        sk = core::ptr::null_mut();
    }

    if sk.is_null() {
        sk = nf_sk_lookup_slow_v4(xt_net(par), skb, xt_in(par));
    }

    if !sk.is_null() {
        let wildcard = (!( (*info).flags & XT_SOCKET_NOWILDCARD) != 0
            && sk_fullsock(sk)
            && (*inet_sk(sk)).inet_rcv_saddr == 0);
        let mut transparent = true;

        // Ignore sockets listening on INADDR_ANY, unless XT_SOCKET_NOWILDCARD is set.
        // Ignore non-transparent sockets if XT_SOCKET_TRANSPARENT is used.
        if ((*info).flags & XT_SOCKET_TRANSPARENT) != 0 {
            transparent = inet_sk_transparent(sk);
        }

        if ((*info).flags & XT_SOCKET_RESTORESKMARK) != 0
            && !wildcard && transparent && sk_fullsock(sk)
        {
            (*pskb).mark = core::ptr::read_volatile(&(*sk).sk_mark);
        }

        if sk != (*skb).sk {
            sock_gen_put(sk);
        }

        if wildcard || !transparent {
            sk = core::ptr::null_mut();
        }
    }

    !sk.is_null()
}

unsafe fn socket_mt4_v0(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static mut XT_INFO_V0: xt_socket_mtinfo1 = xt_socket_mtinfo1 { flags: 0 };
    socket_match(skb, par, &raw const XT_INFO_V0)
}

unsafe fn socket_mt4_v1_v2_v3(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    socket_match(skb, par, (*par).matchinfo as *const xt_socket_mtinfo1)
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe fn socket_mt6_v1_v2_v3(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_socket_mtinfo1;
    let pskb = skb as *mut sk_buff;
    let mut sk = (*skb).sk;

    if !sk.is_null() && !net_eq(xt_net(par), sock_net(sk)) {
        sk = core::ptr::null_mut();
    }
    if sk.is_null() {
        sk = nf_sk_lookup_slow_v6(xt_net(par), skb, xt_in(par));
    }

    if !sk.is_null() {
        let wildcard = (!( (*info).flags & XT_SOCKET_NOWILDCARD) != 0
            && sk_fullsock(sk)
            && ipv6_addr_any(&(*sk).sk_v6_rcv_saddr));
        let mut transparent = true;

        if ((*info).flags & XT_SOCKET_TRANSPARENT) != 0 {
            transparent = inet_sk_transparent(sk);
        }
        if ((*info).flags & XT_SOCKET_RESTORESKMARK) != 0
            && !wildcard && transparent && sk_fullsock(sk)
        {
            (*pskb).mark = core::ptr::read_volatile(&(*sk).sk_mark);
        }
        if sk != (*skb).sk {
            sock_gen_put(sk);
        }
        if wildcard || !transparent {
            sk = core::ptr::null_mut();
        }
    }
    !sk.is_null()
}

unsafe fn socket_mt_enable_defrag(net: *mut net, family: i32) -> i32 {
    match family {
        NFPROTO_IPV4 => nf_defrag_ipv4_enable(net),
        #[cfg(CONFIG_IP6_NF_IPTABLES)]
        NFPROTO_IPV6 => nf_defrag_ipv6_enable(net),
        _ => {
            WARN_ONCE!(true, "Unknown family %d\n", family);
            0
        }
    }
}

unsafe fn socket_mt_v1_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_socket_mtinfo1;
    if ((*info).flags & !XT_SOCKET_FLAGS_V1) != 0 {
        pr_info_ratelimited!("unknown flags 0x%x\n", (*info).flags & !XT_SOCKET_FLAGS_V1);
        return -EINVAL;
    }
    socket_mt_enable_defrag((*par).net, (*par).family)
}

unsafe fn socket_mt_v2_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_socket_mtinfo2;
    if ((*info).flags & !XT_SOCKET_FLAGS_V2) != 0 {
        pr_info_ratelimited!("unknown flags 0x%x\n", (*info).flags & !XT_SOCKET_FLAGS_V2);
        return -EINVAL;
    }
    socket_mt_enable_defrag((*par).net, (*par).family)
}

unsafe fn socket_mt_v3_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_socket_mtinfo3;
    if ((*info).flags & !XT_SOCKET_FLAGS_V3) != 0 {
        pr_info_ratelimited!("unknown flags 0x%x\n", (*info).flags & !XT_SOCKET_FLAGS_V3);
        return -EINVAL;
    }
    socket_mt_enable_defrag((*par).net, (*par).family)
}

unsafe fn socket_mt_destroy(par: *const xt_mtdtor_param) {
    if (*par).family == NFPROTO_IPV4 {
        nf_defrag_ipv4_disable((*par).net);
    }
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    else if (*par).family == NFPROTO_IPV6 {
        nf_defrag_ipv6_disable((*par).net);
    }
}

// Registration table translated from socket_mt_reg. Field types and constants
// are supplied by the kernel x_tables bindings.
static mut SOCKET_MT_REG: [xt_match; 7] = [
    xt_match { name: "socket", revision: 0, family: NFPROTO_IPV4, match_: Some(socket_mt4_v0), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    xt_match { name: "socket", revision: 1, family: NFPROTO_IPV4, match_: Some(socket_mt4_v1_v2_v3), destroy: Some(socket_mt_destroy), checkentry: Some(socket_mt_v1_check), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    xt_match { name: "socket", revision: 1, family: NFPROTO_IPV6, match_: Some(socket_mt6_v1_v2_v3), checkentry: Some(socket_mt_v1_check), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), destroy: Some(socket_mt_destroy), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    xt_match { name: "socket", revision: 2, family: NFPROTO_IPV4, match_: Some(socket_mt4_v1_v2_v3), checkentry: Some(socket_mt_v2_check), destroy: Some(socket_mt_destroy), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    xt_match { name: "socket", revision: 2, family: NFPROTO_IPV6, match_: Some(socket_mt6_v1_v2_v3), checkentry: Some(socket_mt_v2_check), destroy: Some(socket_mt_destroy), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    xt_match { name: "socket", revision: 3, family: NFPROTO_IPV4, match_: Some(socket_mt4_v1_v2_v3), checkentry: Some(socket_mt_v3_check), destroy: Some(socket_mt_destroy), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    xt_match { name: "socket", revision: 3, family: NFPROTO_IPV6, match_: Some(socket_mt6_v1_v2_v3), checkentry: Some(socket_mt_v3_check), destroy: Some(socket_mt_destroy), matchsize: core::mem::size_of::<xt_socket_mtinfo1>(), hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE, ..xt_match::default() },
];

unsafe fn socket_mt_init() -> i32 {
    xt_register_matches(SOCKET_MT_REG.as_mut_ptr(), SOCKET_MT_REG.len())
}

unsafe fn socket_mt_exit() {
    xt_unregister_matches(SOCKET_MT_REG.as_mut_ptr(), SOCKET_MT_REG.len());
}

module_init!(socket_mt_init);
module_exit!(socket_mt_exit);

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Krisztian Kovacs, Balazs Scheidler");
MODULE_DESCRIPTION!("x_tables socket match module");
MODULE_ALIAS!("ipt_socket");
MODULE_ALIAS!("ip6t_socket");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
