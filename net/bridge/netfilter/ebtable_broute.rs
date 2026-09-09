// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebtable_broute
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2002
 *
 *  This table lets you choose between routing and bridging for frames
 *  entering on a bridge enslaved nic. This table is traversed before any
 *  other ebtables table. See net/bridge/br_input.c.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut initial_chain: ebt_entries = ebt_entries {
    name: *b"BROUTING\0",
    policy: EBT_ACCEPT,
};

static mut initial_table: ebt_replace_kernel = ebt_replace_kernel {
    name: *b"broute\0",
    valid_hooks: 1 << NF_BR_BROUTING,
    entries_size: core::mem::size_of::<ebt_entries>(),
    hook_entry: {
        let mut value = [core::ptr::null_mut(); NF_BR_NUMHOOKS];
        value[NF_BR_BROUTING] = core::ptr::addr_of_mut!(initial_chain);
        value
    },
    entries: core::ptr::addr_of_mut!(initial_chain) as *mut core::ffi::c_char,
};

static broute_table: ebt_table = ebt_table {
    name: *b"broute\0",
    table: core::ptr::addr_of_mut!(initial_table),
    valid_hooks: 1 << NF_BR_BROUTING,
    me: THIS_MODULE,
};

unsafe fn ebt_broute(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    s: *const nf_hook_state,
) -> u32 {
    let p: *mut net_bridge_port = br_port_get_rcu((*skb).dev);
    let mut state: nf_hook_state = core::mem::zeroed();
    let dest: *mut u8;
    let mut ret: i32;

    if p.is_null() || (*p).state != BR_STATE_FORWARDING {
        return NF_ACCEPT;
    }

    nf_hook_state_init(
        &mut state,
        NF_BR_BROUTING,
        NFPROTO_BRIDGE,
        (*s).in_,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        (*s).net,
        core::ptr::null_mut(),
    );

    ret = ebt_do_table(priv_, skb, &mut state);
    if ret != NF_DROP {
        return ret as u32;
    }

    /* DROP in ebtables -t broute means that the
     * skb should be routed, not bridged.
     * This is awkward, but can't be changed for compatibility
     * reasons.
     *
     * We map DROP to ACCEPT and set the ->br_netfilter_broute flag.
     */
    (*BR_INPUT_SKB_CB(skb)).br_netfilter_broute = 1;

    /* undo PACKET_HOST mangling done in br_input in case the dst
     * address matches the logical bridge but not the port.
     */
    dest = (*eth_hdr(skb)).h_dest.as_mut_ptr();
    if (*skb).pkt_type == PACKET_HOST
        && !ether_addr_equal((*(*skb).dev).dev_addr.as_ptr(), dest)
        && ether_addr_equal((*(*p).br).dev.dev_addr.as_ptr(), dest)
    {
        (*skb).pkt_type = PACKET_OTHERHOST;
    }

    NF_ACCEPT
}

static ebt_ops_broute: nf_hook_ops = nf_hook_ops {
    hook: Some(ebt_broute),
    pf: NFPROTO_BRIDGE,
    hooknum: NF_BR_PRE_ROUTING,
    priority: NF_BR_PRI_FIRST,
};

unsafe fn broute_table_init(net: *mut net) -> i32 {
    ebt_register_table(net, &mut broute_table, &mut ebt_ops_broute)
}

unsafe fn broute_net_pre_exit(net: *mut net) {
    ebt_unregister_table_pre_exit(net, b"broute\0".as_ptr() as *const core::ffi::c_char);
}

unsafe fn broute_net_exit(net: *mut net) {
    ebt_unregister_table(net, b"broute\0".as_ptr() as *const core::ffi::c_char);
}

static mut broute_net_ops: pernet_operations = pernet_operations {
    exit: Some(broute_net_exit),
    pre_exit: Some(broute_net_pre_exit),
};

unsafe fn ebtable_broute_init() -> i32 {
    let mut ret: i32 = register_pernet_subsys(&mut broute_net_ops);

    if ret != 0 {
        return ret;
    }

    ret = ebt_register_template(&broute_table, Some(broute_table_init));
    if ret != 0 {
        unregister_pernet_subsys(&mut broute_net_ops);
    }

    ret
}

unsafe fn ebtable_broute_fini() {
    ebt_unregister_template(&broute_table);
    unregister_pernet_subsys(&mut broute_net_ops);
}

// module_init(ebtable_broute_init);
// module_exit(ebtable_broute_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Force packets to be routed instead of bridged");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
