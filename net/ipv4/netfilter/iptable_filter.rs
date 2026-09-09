// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is the 1999 rewrite of IP Firewalling, aiming for kernel 2.3.x.
 *
 * Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
 * Copyright (C) 2000-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// C dependencies supplied by the surrounding kernel translation.

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Netfilter Core Team <coreteam@netfilter.org>");
MODULE_DESCRIPTION!("iptables filter table");

const FILTER_VALID_HOOKS: u32 = (1u32 << NF_INET_LOCAL_IN)
    | (1u32 << NF_INET_FORWARD)
    | (1u32 << NF_INET_LOCAL_OUT);

static PACKET_FILTER: xt_table = xt_table {
    name: *b"filter\0",
    valid_hooks: FILTER_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV4,
    priority: NF_IP_PRI_FILTER,
};

static mut FILTER_OPS: *mut nf_hook_ops = core::ptr::null_mut();

/* Default to forward because I got too much mail already. */
static mut FORWARD: bool = true;
module_param!(FORWARD, bool, 0o000);

unsafe fn iptable_filter_table_init(net: *mut net) -> c_int {
    let repl: *mut ipt_replace = ipt_alloc_initial_table(&PACKET_FILTER);
    if repl.is_null() {
        return -ENOMEM;
    }

    /* Entry 1 is the FORWARD hook */
    (*((*repl).entries.cast::<ipt_standard>().add(1))).target.verdict =
        if FORWARD { -NF_ACCEPT - 1 } else { NF_DROP - 1 };

    let err: c_int = ipt_register_table(net, &PACKET_FILTER, repl, FILTER_OPS);
    kfree(repl.cast());
    err
}

unsafe fn iptable_filter_net_init(net: *mut net) -> c_int {
    if !FORWARD {
        return iptable_filter_table_init(net);
    }

    0
}

unsafe fn iptable_filter_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, b"filter\0".as_ptr().cast());
}

unsafe fn iptable_filter_net_exit(net: *mut net) {
    ipt_unregister_table_exit(net, b"filter\0".as_ptr().cast());
}

static mut IPTABLE_FILTER_NET_OPS: pernet_operations = pernet_operations {
    init: Some(iptable_filter_net_init),
    pre_exit: Some(iptable_filter_net_pre_exit),
    exit: Some(iptable_filter_net_exit),
};

unsafe fn iptable_filter_init() -> c_int {
    let mut ret: c_int;

    FILTER_OPS = xt_hook_ops_alloc(&PACKET_FILTER, ipt_do_table);
    if IS_ERR(FILTER_OPS.cast()) {
        return PTR_ERR(FILTER_OPS.cast());
    }

    ret = register_pernet_subsys(&mut IPTABLE_FILTER_NET_OPS);
    if ret < 0 {
        kfree(FILTER_OPS.cast());
        return ret;
    }

    ret = xt_register_template(&PACKET_FILTER, iptable_filter_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&mut IPTABLE_FILTER_NET_OPS);
        kfree(FILTER_OPS.cast());
        return ret;
    }

    return 0;
}

unsafe fn iptable_filter_fini() {
    xt_unregister_template(&PACKET_FILTER);
    unregister_pernet_subsys(&mut IPTABLE_FILTER_NET_OPS);
    kfree(FILTER_OPS.cast());
}

module_init!(iptable_filter_init);
module_exit!(iptable_filter_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
