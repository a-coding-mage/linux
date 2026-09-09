// SPDX-License-Identifier: GPL-2.0-only
/*
 * Filtering ARP tables module.
 *
 * Copyright (C) 2002 David S. Miller (davem@redhat.com)
 *
 */

// C dependencies supplied by the surrounding kernel translation.

const FILTER_VALID_HOOKS: u32 =
    (1u32 << NF_ARP_IN) | (1u32 << NF_ARP_OUT) | (1u32 << NF_ARP_FORWARD);

static PACKET_FILTER: xt_table = xt_table {
    name: *b"filter\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    valid_hooks: FILTER_VALID_HOOKS,
    me: THIS_MODULE,
    af: NFPROTO_ARP,
    priority: NF_IP_PRI_FILTER,
};

static mut arpfilter_ops: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn arptable_filter_table_init(net: *mut net) -> i32 {
    let repl: *mut arpt_replace;
    let err: i32;

    repl = arpt_alloc_initial_table(&PACKET_FILTER);
    if repl.is_null() {
        return -ENOMEM;
    }
    err = arpt_register_table(net, &PACKET_FILTER, repl, arpfilter_ops);
    kfree(repl.cast());
    err
}

unsafe fn arptable_filter_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_ARP, b"filter\0".as_ptr().cast());
}

unsafe fn arptable_filter_net_exit(net: *mut net) {
    arpt_unregister_table(net, b"filter\0".as_ptr().cast());
}

static mut arptable_filter_net_ops: pernet_operations = pernet_operations {
    exit: Some(arptable_filter_net_exit),
    pre_exit: Some(arptable_filter_net_pre_exit),
};

unsafe fn arptable_filter_init() -> i32 {
    let ret: i32;

    arpfilter_ops = xt_hook_ops_alloc(&PACKET_FILTER, arpt_do_table);
    if IS_ERR(arpfilter_ops.cast()) {
        return PTR_ERR(arpfilter_ops.cast());
    }

    ret = register_pernet_subsys(&mut arptable_filter_net_ops);
    if ret < 0 {
        goto_err_free();
        return ret;
    }

    ret = xt_register_template(&PACKET_FILTER, arptable_filter_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&mut arptable_filter_net_ops);
        goto_err_free();
        return ret;
    }

    0
}

unsafe fn goto_err_free() {
    kfree(arpfilter_ops.cast());
}

unsafe fn arptable_filter_fini() {
    xt_unregister_template(&PACKET_FILTER);
    unregister_pernet_subsys(&mut arptable_filter_net_ops);
    kfree(arpfilter_ops.cast());
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("David S. Miller <davem@redhat.com>");
// MODULE_DESCRIPTION("arptables filter table");
// module_init(arptable_filter_init);
// module_exit(arptable_filter_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
