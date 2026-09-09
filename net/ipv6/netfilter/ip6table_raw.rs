// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPv6 raw table, a port of the IPv4 raw table to IPv6
 *
 * Copyright (C) 2003 Jozsef Kadlecsik <kadlec@netfilter.org>
 */

// Dependency-provided kernel and netfilter declarations are referenced here.

const RAW_VALID_HOOKS: u32 = (1u32 << NF_INET_PRE_ROUTING) | (1u32 << NF_INET_LOCAL_OUT);

static mut raw_before_defrag: bool = false;

static packet_raw: xt_table = xt_table {
    name: *b"raw\0",
    valid_hooks: RAW_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV6,
    priority: NF_IP6_PRI_RAW,
};

static packet_raw_before_defrag: xt_table = xt_table {
    name: *b"raw\0",
    valid_hooks: RAW_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV6,
    priority: NF_IP6_PRI_RAW_BEFORE_DEFRAG,
};

static mut rawtable_ops: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn ip6table_raw_table_init(net: *mut net) -> i32 {
    let mut repl: *mut ip6t_replace;
    let mut table: *const xt_table = &packet_raw;
    let mut ret: i32;

    if raw_before_defrag {
        table = &packet_raw_before_defrag;
    }

    repl = ip6t_alloc_initial_table(table);
    if repl.is_null() {
        return -ENOMEM;
    }
    ret = ip6t_register_table(net, table, repl, rawtable_ops);
    kfree(repl as *mut core::ffi::c_void);
    ret
}

unsafe extern "C" fn ip6table_raw_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, *b"raw\0");
}

unsafe extern "C" fn ip6table_raw_net_exit(net: *mut net) {
    ip6t_unregister_table_exit(net, *b"raw\0");
}

static mut ip6table_raw_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(ip6table_raw_net_pre_exit),
    exit: Some(ip6table_raw_net_exit),
};

unsafe extern "C" fn ip6table_raw_init() -> i32 {
    let mut table: *const xt_table = &packet_raw;
    let mut ret: i32;

    if raw_before_defrag {
        table = &packet_raw_before_defrag;
        pr_info!("Enabling raw table before defrag\\n");
    }

    /* Register hooks */
    rawtable_ops = xt_hook_ops_alloc(table, Some(ip6t_do_table));
    if IS_ERR(rawtable_ops) {
        return PTR_ERR(rawtable_ops);
    }

    ret = register_pernet_subsys(&mut ip6table_raw_net_ops);
    if ret < 0 {
        kfree(rawtable_ops as *mut core::ffi::c_void);
        return ret;
    }

    ret = xt_register_template(table, Some(ip6table_raw_table_init));
    if ret < 0 {
        unregister_pernet_subsys(&mut ip6table_raw_net_ops);
        kfree(rawtable_ops as *mut core::ffi::c_void);
        return ret;
    }

    0
}

unsafe extern "C" fn ip6table_raw_fini() {
    xt_unregister_template(&packet_raw);
    unregister_pernet_subsys(&mut ip6table_raw_net_ops);
    kfree(rawtable_ops as *mut core::ffi::c_void);
}

// module_init(ip6table_raw_init);
// module_exit(ip6table_raw_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Ip6tables legacy raw table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
