// SPDX-License-Identifier: GPL-2.0-only
/*
 * "security" table for IPv6
 *
 * This is for use by Mandatory Access Control (MAC) security models,
 * which need to be able to manage security policy in separate context
 * to DAC.
 *
 * Based on iptable_mangle.c
 *
 * Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
 * Copyright (C) 2000-2004 Netfilter Core Team <coreteam <at> netfilter.org>
 * Copyright (C) 2008 Red Hat, Inc., James Morris <jmorris <at> redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const SECURITY_VALID_HOOKS: u32 = (1u32 << NF_INET_LOCAL_IN)
    | (1u32 << NF_INET_FORWARD)
    | (1u32 << NF_INET_LOCAL_OUT);

static mut security_table: xt_table = xt_table {
    name: *b"security\0",
    valid_hooks: SECURITY_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV6,
    priority: NF_IP6_PRI_SECURITY,
};

static mut sectbl_ops: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn ip6table_security_table_init(net: *mut net) -> c_int {
    let repl: *mut ip6t_replace = ip6t_alloc_initial_table(&security_table);
    if repl.is_null() {
        return -ENOMEM;
    }
    let ret: c_int = ip6t_register_table(net, &security_table, repl, sectbl_ops);
    kfree(repl.cast());
    ret
}

unsafe fn ip6table_security_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, b"security\0".as_ptr().cast());
}

unsafe fn ip6table_security_net_exit(net: *mut net) {
    ip6t_unregister_table_exit(net, b"security\0".as_ptr().cast());
}

static mut ip6table_security_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(ip6table_security_net_pre_exit),
    exit: Some(ip6table_security_net_exit),
};

unsafe fn ip6table_security_init() -> c_int {
    let mut ret: c_int;

    sectbl_ops = xt_hook_ops_alloc(&security_table, ip6t_do_table);
    if IS_ERR(sectbl_ops.cast()) {
        return PTR_ERR(sectbl_ops.cast());
    }

    ret = register_pernet_subsys(&ip6table_security_net_ops);
    if ret < 0 {
        kfree(sectbl_ops.cast());
        return ret;
    }

    ret = xt_register_template(&security_table, ip6table_security_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&ip6table_security_net_ops);
        kfree(sectbl_ops.cast());
        return ret;
    }

    0
}

unsafe fn ip6table_security_fini() {
    xt_unregister_template(&security_table);
    unregister_pernet_subsys(&ip6table_security_net_ops);
    kfree(sectbl_ops.cast());
}

// module_init(ip6table_security_init);
// module_exit(ip6table_security_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
