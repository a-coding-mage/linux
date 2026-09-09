// SPDX-License-Identifier: GPL-2.0-only
/*
 * 'raw' table, which is the very first hooked in at PRE_ROUTING and LOCAL_OUT .
 *
 * Copyright (C) 2003 Jozsef Kadlecsik <kadlec@netfilter.org>
 */
// Dependency headers from the C translation unit:
// linux/module.h, linux/netfilter_ipv4/ip_tables.h, linux/slab.h, net/ip.h

const RAW_VALID_HOOKS: u32 = (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT);

static mut raw_before_defrag: bool = false;
// MODULE_PARM_DESC(raw_before_defrag, "Enable raw table before defrag");
// module_param(raw_before_defrag, bool, 0000);

#[repr(C)]
static packet_raw: xt_table = xt_table {
    name: *b"raw\0" as *const u8 as *const i8,
    valid_hooks: RAW_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV4,
    priority: NF_IP_PRI_RAW,
};

#[repr(C)]
static packet_raw_before_defrag: xt_table = xt_table {
    name: *b"raw\0" as *const u8 as *const i8,
    valid_hooks: RAW_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_IPV4,
    priority: NF_IP_PRI_RAW_BEFORE_DEFRAG,
};

static mut rawtable_ops: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn iptable_raw_table_init(net: *mut net) -> i32 {
    let mut repl: *mut ipt_replace;
    let mut table: *const xt_table = &packet_raw;
    let ret: i32;

    if raw_before_defrag {
        table = &packet_raw_before_defrag;
    }

    repl = ipt_alloc_initial_table(table);
    if repl.is_null() {
        return -ENOMEM;
    }
    ret = ipt_register_table(net, table, repl, rawtable_ops);
    kfree(repl as *mut core::ffi::c_void);
    ret
}

unsafe fn iptable_raw_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, b"raw\0".as_ptr() as *const i8);
}

unsafe fn iptable_raw_net_exit(net: *mut net) {
    ipt_unregister_table_exit(net, b"raw\0".as_ptr() as *const i8);
}

#[repr(C)]
static mut iptable_raw_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(iptable_raw_net_pre_exit),
    exit: Some(iptable_raw_net_exit),
};

unsafe fn iptable_raw_init() -> i32 {
    let ret: i32;
    let mut table: *const xt_table = &packet_raw;

    if raw_before_defrag {
        table = &packet_raw_before_defrag;

        pr_info!("Enabling raw table before defrag\n");
    }

    rawtable_ops = xt_hook_ops_alloc(table, Some(ipt_do_table));
    if IS_ERR(rawtable_ops as *const core::ffi::c_void) {
        return PTR_ERR(rawtable_ops as *const core::ffi::c_void);
    }

    ret = register_pernet_subsys(&iptable_raw_net_ops);
    if ret < 0 {
        goto_err_free();
        return ret;
    }

    ret = xt_register_template(table, Some(iptable_raw_table_init));
    if ret < 0 {
        unregister_pernet_subsys(&iptable_raw_net_ops);
        goto_err_free();
        return ret;
    }

    0
}

unsafe fn goto_err_free() {
    kfree(rawtable_ops as *mut core::ffi::c_void);
}

unsafe fn iptable_raw_fini() {
    xt_unregister_template(&packet_raw);
    unregister_pernet_subsys(&iptable_raw_net_ops);
    kfree(rawtable_ops as *mut core::ffi::c_void);
}

// module_init(iptable_raw_init);
// module_exit(iptable_raw_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("iptables legacy raw table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
