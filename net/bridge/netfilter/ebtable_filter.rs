// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebtable_filter
 *
 *  Authors:
 *  Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2002
 */

// Dependencies supplied by the corresponding Linux kernel headers/modules.

const FILTER_VALID_HOOKS: u32 =
    (1u32 << NF_BR_LOCAL_IN) | (1u32 << NF_BR_FORWARD) | (1u32 << NF_BR_LOCAL_OUT);

static mut INITIAL_CHAINS: [ebt_entries; 3] = [
    ebt_entries {
        name: *b"INPUT\0",
        policy: EBT_ACCEPT,
    },
    ebt_entries {
        name: *b"FORWARD\0",
        policy: EBT_ACCEPT,
    },
    ebt_entries {
        name: *b"OUTPUT\0",
        policy: EBT_ACCEPT,
    },
];

static mut INITIAL_TABLE: ebt_replace_kernel = ebt_replace_kernel {
    name: *b"filter\0",
    valid_hooks: FILTER_VALID_HOOKS,
    entries_size: 3 * core::mem::size_of::<ebt_entries>(),
    hook_entry: [
        0 as *mut ebt_entries,
        0 as *mut ebt_entries,
        0 as *mut ebt_entries,
        0 as *mut ebt_entries,
        0 as *mut ebt_entries,
        0 as *mut ebt_entries,
    ],
    entries: core::ptr::null_mut(),
};

static FRAME_FILTER: ebt_table = ebt_table {
    name: *b"filter\0",
    table: unsafe { &raw mut INITIAL_TABLE },
    valid_hooks: FILTER_VALID_HOOKS,
    me: THIS_MODULE,
};

static EBT_OPS_FILTER: [nf_hook_ops; 3] = [
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_LOCAL_IN,
        priority: NF_BR_PRI_FILTER_BRIDGED,
    },
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_FORWARD,
        priority: NF_BR_PRI_FILTER_BRIDGED,
    },
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_LOCAL_OUT,
        priority: NF_BR_PRI_FILTER_OTHER,
    },
];

unsafe fn frame_filter_table_init(net: *mut net) -> i32 {
    ebt_register_table(net, &raw const FRAME_FILTER, &raw const EBT_OPS_FILTER)
}

unsafe fn frame_filter_net_pre_exit(net: *mut net) {
    ebt_unregister_table_pre_exit(net, b"filter\0".as_ptr() as *const i8);
}

unsafe fn frame_filter_net_exit(net: *mut net) {
    ebt_unregister_table(net, b"filter\0".as_ptr() as *const i8);
}

static mut FRAME_FILTER_NET_OPS: pernet_operations = pernet_operations {
    exit: Some(frame_filter_net_exit),
    pre_exit: Some(frame_filter_net_pre_exit),
};

unsafe fn ebtable_filter_init() -> i32 {
    let mut ret = register_pernet_subsys(&raw mut FRAME_FILTER_NET_OPS);

    if ret != 0 {
        return ret;
    }

    ret = ebt_register_template(&raw const FRAME_FILTER, Some(frame_filter_table_init));
    if ret != 0 {
        unregister_pernet_subsys(&raw mut FRAME_FILTER_NET_OPS);
    }

    ret
}

unsafe fn ebtable_filter_fini() {
    ebt_unregister_template(&raw const FRAME_FILTER);
    unregister_pernet_subsys(&raw mut FRAME_FILTER_NET_OPS);
}

// module_init(ebtable_filter_init);
// module_exit(ebtable_filter_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ebtables legacy filter table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
