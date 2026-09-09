// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPv6 packet mangling table, a port of the IPv4 mangle table to IPv6
 *
 * Copyright (C) 2000-2001 by Harald Welte <laforge@gnumonks.org>
 * Copyright (C) 2000-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Linux kernel headers and module metadata are supplied by the surrounding build.
// MODULE_LICENSE!("GPL");
// MODULE_AUTHOR!("Netfilter Core Team <coreteam@netfilter.org>");
// MODULE_DESCRIPTION!("ip6tables mangle table");

const MANGLE_VALID_HOOKS: u32 = (1 << NF_INET_PRE_ROUTING)
    | (1 << NF_INET_LOCAL_IN)
    | (1 << NF_INET_FORWARD)
    | (1 << NF_INET_LOCAL_OUT)
    | (1 << NF_INET_POST_ROUTING);

static PACKET_MANGLER: xt_table = xt_table {
    name: *b"mangle\0",
    valid_hooks: MANGLE_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV6,
    priority: NF_IP6_PRI_MANGLE,
};

unsafe fn ip6t_mangle_out(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    let mut saddr: in6_addr = core::mem::zeroed();
    let mut daddr: in6_addr = core::mem::zeroed();
    let mut ret: u32;
    let verdict: u32;
    let flowlabel: u32;
    let mark: u32;
    let hop_limit: u8;
    let err: i32;

    /* save source/dest address, mark, hoplimit, flowlabel, priority,  */
    core::ptr::copy_nonoverlapping(
        &(*ipv6_hdr(skb)).saddr,
        &mut saddr,
        1,
    );
    core::ptr::copy_nonoverlapping(
        &(*ipv6_hdr(skb)).daddr,
        &mut daddr,
        1,
    );
    mark = (*skb).mark;
    hop_limit = (*ipv6_hdr(skb)).hop_limit;

    /* flowlabel and prio (includes version, which shouldn't change either */
    flowlabel = *((ipv6_hdr(skb) as *const u8).cast::<u32>());

    ret = ip6t_do_table(priv_, skb, state);
    verdict = ret & NF_VERDICT_MASK;

    if verdict != NF_DROP
        && verdict != NF_STOLEN
        && (!ipv6_addr_equal(&(*ipv6_hdr(skb)).saddr, &saddr)
            || !ipv6_addr_equal(&(*ipv6_hdr(skb)).daddr, &daddr)
            || (*skb).mark != mark
            || (*ipv6_hdr(skb)).hop_limit != hop_limit
            || flowlabel != *((ipv6_hdr(skb) as *const u8).cast::<u32>()))
    {
        err = ip6_route_me_harder((*state).net, (*state).sk, skb);
        if err < 0 {
            ret = NF_DROP_ERR(err);
        }
    }

    ret
}

/* The work comes in here from netfilter.c. */
unsafe fn ip6table_mangle_hook(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    if (*state).hook == NF_INET_LOCAL_OUT {
        return ip6t_mangle_out(priv_, skb, state);
    }
    ip6t_do_table(priv_, skb, state)
}

static mut MANGLE_OPS: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn ip6table_mangle_table_init(net: *mut net) -> i32 {
    let repl: *mut ip6t_replace;
    let ret: i32;

    repl = ip6t_alloc_initial_table(&PACKET_MANGLER);
    if repl.is_null() {
        return -ENOMEM;
    }
    ret = ip6t_register_table(net, &PACKET_MANGLER, repl, MANGLE_OPS);
    kfree(repl.cast());
    ret
}

unsafe fn ip6table_mangle_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, b"mangle\0".as_ptr());
}

unsafe fn ip6table_mangle_net_exit(net: *mut net) {
    ip6t_unregister_table_exit(net, b"mangle\0".as_ptr());
}

static mut IP6TABLE_MANGLE_NET_OPS: pernet_operations = pernet_operations {
    pre_exit: Some(ip6table_mangle_net_pre_exit),
    exit: Some(ip6table_mangle_net_exit),
};

unsafe fn ip6table_mangle_init() -> i32 {
    let mut ret: i32;

    MANGLE_OPS = xt_hook_ops_alloc(&PACKET_MANGLER, Some(ip6table_mangle_hook));
    if is_err(MANGLE_OPS.cast()) {
        return ptr_err(MANGLE_OPS.cast());
    }

    ret = register_pernet_subsys(&raw mut IP6TABLE_MANGLE_NET_OPS);
    if ret < 0 {
        goto_err_free();
        return ret;
    }

    ret = xt_register_template(&PACKET_MANGLER, Some(ip6table_mangle_table_init));
    if ret < 0 {
        unregister_pernet_subsys(&raw mut IP6TABLE_MANGLE_NET_OPS);
        goto_err_free();
    }
    ret
}

unsafe fn goto_err_free() {
    kfree(MANGLE_OPS.cast());
}

unsafe fn ip6table_mangle_fini() {
    xt_unregister_template(&PACKET_MANGLER);
    unregister_pernet_subsys(&raw mut IP6TABLE_MANGLE_NET_OPS);
    kfree(MANGLE_OPS.cast());
}

// module_init!(ip6table_mangle_init);
// module_exit!(ip6table_mangle_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
