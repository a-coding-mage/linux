// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2011 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct iptable_nat_pernet {
    pub nf_nat_ops: *mut nf_hook_ops,
}

static mut iptable_nat_net_id: c_uint = 0;

static nf_nat_ipv4_table: xt_table = xt_table {
    name: *b"nat\0" as *const u8 as *const c_char,
    valid_hooks: (1u32 << NF_INET_PRE_ROUTING)
        | (1u32 << NF_INET_POST_ROUTING)
        | (1u32 << NF_INET_LOCAL_OUT)
        | (1u32 << NF_INET_LOCAL_IN),
    me: THIS_MODULE,
    af: NFPROTO_IPV4,
};

static nf_nat_ipv4_ops: [nf_hook_ops; 4] = [
    nf_hook_ops {
        hook: Some(ipt_do_table),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_PRE_ROUTING,
        priority: NF_IP_PRI_NAT_DST,
        priv: core::ptr::null_mut(),
    },
    nf_hook_ops {
        hook: Some(ipt_do_table),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_POST_ROUTING,
        priority: NF_IP_PRI_NAT_SRC,
        priv: core::ptr::null_mut(),
    },
    nf_hook_ops {
        hook: Some(ipt_do_table),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_LOCAL_OUT,
        priority: NF_IP_PRI_NAT_DST,
        priv: core::ptr::null_mut(),
    },
    nf_hook_ops {
        hook: Some(ipt_do_table),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_LOCAL_IN,
        priority: NF_IP_PRI_NAT_SRC,
        priv: core::ptr::null_mut(),
    },
];

unsafe fn ipt_nat_register_lookups(net: *mut net) -> c_int {
    let xt_nat_net: *mut iptable_nat_pernet;
    let mut ops: *mut nf_hook_ops;
    let table: *mut xt_table;
    let mut i: usize;
    let mut ret: c_int;

    xt_nat_net = net_generic(net, iptable_nat_net_id);
    table = xt_find_table(net, NFPROTO_IPV4, b"nat\0".as_ptr() as *const c_char);
    if table.is_null() {
        return -ENOENT;
    }

    ops = kmemdup(
        nf_nat_ipv4_ops.as_ptr() as *const c_void,
        core::mem::size_of_val(&nf_nat_ipv4_ops),
        GFP_KERNEL,
    ) as *mut nf_hook_ops;
    if ops.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < nf_nat_ipv4_ops.len() {
        (*ops.add(i)).priv_ = table as *mut c_void;
        ret = nf_nat_ipv4_register_fn(net, ops.add(i));
        if ret != 0 {
            while i != 0 {
                i -= 1;
                nf_nat_ipv4_unregister_fn(net, ops.add(i));
            }
            kfree_rcu(ops as *mut c_void, rcu);
            return ret;
        }
        i += 1;
    }

    (*xt_nat_net).nf_nat_ops = ops;
    0
}

unsafe fn ipt_nat_unregister_lookups(net: *mut net) {
    let xt_nat_net: *mut iptable_nat_pernet = net_generic(net, iptable_nat_net_id);
    let ops: *mut nf_hook_ops = (*xt_nat_net).nf_nat_ops;
    let mut i: usize;

    if ops.is_null() {
        return;
    }

    i = 0;
    while i < nf_nat_ipv4_ops.len() {
        nf_nat_ipv4_unregister_fn(net, ops.add(i));
        i += 1;
    }
    kfree_rcu(ops as *mut c_void, rcu);
}

unsafe fn iptable_nat_table_init(net: *mut net) -> c_int {
    let repl: *mut ipt_replace;
    let mut ret: c_int;

    repl = ipt_alloc_initial_table(&nf_nat_ipv4_table);
    if repl.is_null() {
        return -ENOMEM;
    }

    ret = ipt_register_table(net, &nf_nat_ipv4_table, repl, core::ptr::null_mut());
    if ret < 0 {
        kfree(repl as *mut c_void);
        return ret;
    }

    ret = ipt_nat_register_lookups(net);
    if ret < 0 {
        xt_unregister_table_pre_exit(net, NFPROTO_IPV4, b"nat\0".as_ptr() as *const c_char);
        synchronize_rcu();
        ipt_unregister_table_exit(net, b"nat\0".as_ptr() as *const c_char);
    }

    kfree(repl as *mut c_void);
    ret
}

unsafe fn iptable_nat_net_pre_exit(net: *mut net) {
    ipt_nat_unregister_lookups(net);
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, b"nat\0".as_ptr() as *const c_char);
}

unsafe fn iptable_nat_net_exit(net: *mut net) {
    ipt_unregister_table_exit(net, b"nat\0".as_ptr() as *const c_char);
}

static mut iptable_nat_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(iptable_nat_net_pre_exit),
    exit: Some(iptable_nat_net_exit),
    id: &mut iptable_nat_net_id,
    size: core::mem::size_of::<iptable_nat_pernet>(),
};

unsafe fn iptable_nat_init() -> c_int {
    let mut ret: c_int;

    ret = register_pernet_subsys(&mut iptable_nat_net_ops);
    if ret < 0 {
        return ret;
    }

    ret = xt_register_template(&nf_nat_ipv4_table, iptable_nat_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&mut iptable_nat_net_ops);
    }

    ret
}

unsafe fn iptable_nat_exit() {
    xt_unregister_template(&nf_nat_ipv4_table);
    unregister_pernet_subsys(&mut iptable_nat_net_ops);
}

// module_init(iptable_nat_init);
// module_exit(iptable_nat_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("iptables legacy nat table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
