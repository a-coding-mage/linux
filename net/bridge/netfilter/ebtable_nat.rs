// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebtable_nat
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2002
 *
 */

// C dependencies supplied by the surrounding kernel translation.

const NAT_VALID_HOOKS: u32 = (1u32 << NF_BR_PRE_ROUTING)
    | (1u32 << NF_BR_LOCAL_OUT)
    | (1u32 << NF_BR_POST_ROUTING);

static mut initial_chains: [ebt_entries; 3] = [
    ebt_entries {
        name: *b"PREROUTING\0",
        policy: EBT_ACCEPT,
        ..unsafe { core::mem::zeroed() }
    },
    ebt_entries {
        name: *b"OUTPUT\0",
        policy: EBT_ACCEPT,
        ..unsafe { core::mem::zeroed() }
    },
    ebt_entries {
        name: *b"POSTROUTING\0",
        policy: EBT_ACCEPT,
        ..unsafe { core::mem::zeroed() }
    },
];

static mut initial_table: ebt_replace_kernel = ebt_replace_kernel {
    name: *b"nat\0",
    valid_hooks: NAT_VALID_HOOKS,
    entries_size: 3 * core::mem::size_of::<ebt_entries>(),
    hook_entry: {
        let mut entries = [core::ptr::null_mut(); NF_MAX_HOOKS as usize];
        entries[NF_BR_PRE_ROUTING as usize] = unsafe { &raw mut initial_chains[0] };
        entries[NF_BR_LOCAL_OUT as usize] = unsafe { &raw mut initial_chains[1] };
        entries[NF_BR_POST_ROUTING as usize] = unsafe { &raw mut initial_chains[2] };
        entries
    },
    entries: unsafe { initial_chains.as_mut_ptr() as *mut core::ffi::c_char },
    ..unsafe { core::mem::zeroed() }
};

static frame_nat: ebt_table = ebt_table {
    name: *b"nat\0",
    table: unsafe { &raw mut initial_table },
    valid_hooks: NAT_VALID_HOOKS,
    me: THIS_MODULE,
    ..unsafe { core::mem::zeroed() }
};

static ebt_ops_nat: [nf_hook_ops; 3] = [
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_LOCAL_OUT,
        priority: NF_BR_PRI_NAT_DST_OTHER,
        ..unsafe { core::mem::zeroed() }
    },
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_POST_ROUTING,
        priority: NF_BR_PRI_NAT_SRC,
        ..unsafe { core::mem::zeroed() }
    },
    nf_hook_ops {
        hook: Some(ebt_do_table),
        pf: NFPROTO_BRIDGE,
        hooknum: NF_BR_PRE_ROUTING,
        priority: NF_BR_PRI_NAT_DST_BRIDGED,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn frame_nat_table_init(net: *mut net) -> i32 {
    ebt_register_table(net, &raw const frame_nat, ebt_ops_nat.as_ptr())
}

unsafe fn frame_nat_net_pre_exit(net: *mut net) {
    ebt_unregister_table_pre_exit(net, b"nat\0".as_ptr() as *const core::ffi::c_char);
}

unsafe fn frame_nat_net_exit(net: *mut net) {
    ebt_unregister_table(net, b"nat\0".as_ptr() as *const core::ffi::c_char);
}

static mut frame_nat_net_ops: pernet_operations = pernet_operations {
    exit: Some(frame_nat_net_exit),
    pre_exit: Some(frame_nat_net_pre_exit),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn ebtable_nat_init() -> i32 {
    let mut ret = register_pernet_subsys(&raw mut frame_nat_net_ops);

    if ret != 0 {
        return ret;
    }

    ret = ebt_register_template(&raw const frame_nat, Some(frame_nat_table_init));
    if ret != 0 {
        unregister_pernet_subsys(&raw mut frame_nat_net_ops);
    }

    ret
}

unsafe fn ebtable_nat_fini() {
    ebt_unregister_template(&raw const frame_nat);
    unregister_pernet_subsys(&raw mut frame_nat_net_ops);
}

module_init!(ebtable_nat_init);
module_exit!(ebtable_nat_fini);
module_license!("GPL");
module_description!("ebtables legacy stateless nat table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
