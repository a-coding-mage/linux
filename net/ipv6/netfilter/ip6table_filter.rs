// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is the 1999 rewrite of IP Firewalling, aiming for kernel 2.3.x.
 *
 * Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
 * Copyright (C) 2000-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// C dependencies: linux/module.h, linux/moduleparam.h,
// linux/netfilter_ipv6/ip6_tables.h, linux/slab.h

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
// MODULE_DESCRIPTION("ip6tables filter table");

const FILTER_VALID_HOOKS: u32 = (1 << NF_INET_LOCAL_IN)
    | (1 << NF_INET_FORWARD)
    | (1 << NF_INET_LOCAL_OUT);

static PACKET_FILTER: xt_table = xt_table {
    name: *b"filter\0",
    valid_hooks: FILTER_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV6,
    priority: NF_IP6_PRI_FILTER,
};

static mut FILTER_OPS: *mut nf_hook_ops = core::ptr::null_mut();

/* Default to forward because I got too much mail already. */
static mut FORWARD: bool = true;
// module_param(forward, bool, 0000);

unsafe fn ip6table_filter_table_init(net: *mut net) -> i32 {
    let mut repl: *mut ip6t_replace;
    let err: i32;

    repl = ip6t_alloc_initial_table(&PACKET_FILTER);
    if repl.is_null() {
        return -ENOMEM;
    }
    /* Entry 1 is the FORWARD hook */
    let entries = (*repl).entries as *mut ip6t_standard;
    (*entries.add(1)).target.verdict = if FORWARD {
        -NF_ACCEPT - 1
    } else {
        NF_DROP - 1
    };

    err = ip6t_register_table(net, &PACKET_FILTER, repl, FILTER_OPS);
    kfree(repl as *mut core::ffi::c_void);
    err
}

unsafe fn ip6table_filter_net_init(net: *mut net) -> i32 {
    if !FORWARD {
        return ip6table_filter_table_init(net);
    }

    0
}

unsafe fn ip6table_filter_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, b"filter\0".as_ptr() as *const i8);
}

unsafe fn ip6table_filter_net_exit(net: *mut net) {
    ip6t_unregister_table_exit(net, b"filter\0".as_ptr() as *const i8);
}

static mut IP6TABLE_FILTER_NET_OPS: pernet_operations = pernet_operations {
    init: Some(ip6table_filter_net_init),
    pre_exit: Some(ip6table_filter_net_pre_exit),
    exit: Some(ip6table_filter_net_exit),
};

unsafe fn ip6table_filter_init() -> i32 {
    let ret: i32;

    FILTER_OPS = xt_hook_ops_alloc(&PACKET_FILTER, Some(ip6t_do_table));
    if IS_ERR(FILTER_OPS as *const core::ffi::c_void) {
        return PTR_ERR(FILTER_OPS as *const core::ffi::c_void);
    }

    ret = register_pernet_subsys(&mut IP6TABLE_FILTER_NET_OPS);
    if ret < 0 {
        return ip6table_filter_init_err_free(ret);
    }

    let ret = xt_register_template(&PACKET_FILTER, Some(ip6table_filter_table_init));
    if ret < 0 {
        unregister_pernet_subsys(&mut IP6TABLE_FILTER_NET_OPS);
        return ip6table_filter_init_err_free(ret);
    }

    0
}

unsafe fn ip6table_filter_init_err_free(ret: i32) -> i32 {
    kfree(FILTER_OPS as *mut core::ffi::c_void);
    ret
}

unsafe fn ip6table_filter_fini() {
    xt_unregister_template(&PACKET_FILTER);
    unregister_pernet_subsys(&mut IP6TABLE_FILTER_NET_OPS);
    kfree(FILTER_OPS as *mut core::ffi::c_void);
}

// module_init(ip6table_filter_init);
// module_exit(ip6table_filter_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
