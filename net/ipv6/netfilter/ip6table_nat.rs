// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
 *
 * Based on Rusty Russell's IPv4 NAT code. Development of IPv6 NAT
 * funded by Astaro.
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

#[repr(C)]
pub struct ip6table_nat_pernet {
    pub nf_nat_ops: *mut nf_hook_ops,
}

static mut ip6table_nat_net_id: c_uint = 0;

#[repr(C)]
struct xt_table {
    name: *const c_char,
    valid_hooks: c_uint,
    me: *mut c_void,
    af: c_uint,
}

#[repr(C)]
struct nf_hook_ops {
    hook: Option<unsafe extern "C" fn() -> c_uint>,
    pf: c_uint,
    hooknum: c_uint,
    priority: c_int,
    priv_: *mut c_void,
}

#[repr(C)]
struct net;
#[repr(C)]
struct ip6t_replace;
#[repr(C)]
struct pernet_operations {
    pre_exit: Option<unsafe extern "C" fn(*mut net)>,
    exit: Option<unsafe extern "C" fn(*mut net)>,
    id: *mut c_uint,
    size: usize,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    fn xt_find_table(net: *mut net, pf: c_uint, name: *const c_char) -> *mut xt_table;
    fn net_generic(net: *mut net, id: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree_rcu(ptr: *mut c_void, rcu: *mut c_void);
    fn nf_nat_ipv6_register_fn(net: *mut net, ops: *mut nf_hook_ops) -> c_int;
    fn nf_nat_ipv6_unregister_fn(net: *mut net, ops: *mut nf_hook_ops);
    fn ip6t_do_table();
    fn ip6t_alloc_initial_table(table: *const xt_table) -> *mut ip6t_replace;
    fn ip6t_register_table(net: *mut net, table: *const xt_table, repl: *mut ip6t_replace, arg: *mut c_void) -> c_int;
    fn xt_unregister_table_pre_exit(net: *mut net, pf: c_uint, name: *const c_char);
    fn synchronize_rcu();
    fn ip6t_unregister_table_exit(net: *mut net, name: *const c_char);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
    fn xt_register_template(table: *const xt_table, init: unsafe extern "C" fn(*mut net) -> c_int) -> c_int;
    fn xt_unregister_template(table: *const xt_table);
    fn kfree(ptr: *mut c_void);
}

static mut nf_nat_ipv6_table: xt_table = xt_table {
    name: b"nat\0".as_ptr() as *const c_char,
    valid_hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING) |
        (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_LOCAL_IN),
    me: unsafe { THIS_MODULE },
    af: NFPROTO_IPV6,
};

static mut nf_nat_ipv6_ops: [nf_hook_ops; 4] = [
    nf_hook_ops { hook: Some(ip6t_do_table), pf: NFPROTO_IPV6, hooknum: NF_INET_PRE_ROUTING, priority: NF_IP6_PRI_NAT_DST, priv_: core::ptr::null_mut() },
    nf_hook_ops { hook: Some(ip6t_do_table), pf: NFPROTO_IPV6, hooknum: NF_INET_POST_ROUTING, priority: NF_IP6_PRI_NAT_SRC, priv_: core::ptr::null_mut() },
    nf_hook_ops { hook: Some(ip6t_do_table), pf: NFPROTO_IPV6, hooknum: NF_INET_LOCAL_OUT, priority: NF_IP6_PRI_NAT_DST, priv_: core::ptr::null_mut() },
    nf_hook_ops { hook: Some(ip6t_do_table), pf: NFPROTO_IPV6, hooknum: NF_INET_LOCAL_IN, priority: NF_IP6_PRI_NAT_SRC, priv_: core::ptr::null_mut() },
];

unsafe fn ip6t_nat_register_lookups(net: *mut net) -> c_int {
    let table = xt_find_table(net, NFPROTO_IPV6, b"nat\0".as_ptr() as *const c_char);
    if table.is_null() { return -ENOENT; }
    let xt_nat_net = net_generic(net, ip6table_nat_net_id) as *mut ip6table_nat_pernet;
    let ops = kmemdup(nf_nat_ipv6_ops.as_ptr() as *const c_void, core::mem::size_of_val(&nf_nat_ipv6_ops), GFP_KERNEL) as *mut nf_hook_ops;
    if ops.is_null() { return -ENOMEM; }
    for i in 0..nf_nat_ipv6_ops.len() {
        (*ops.add(i)).priv_ = table as *mut c_void;
        let ret = nf_nat_ipv6_register_fn(net, ops.add(i));
        if ret != 0 {
            for j in (0..i).rev() { nf_nat_ipv6_unregister_fn(net, ops.add(j)); }
            kfree_rcu(ops as *mut c_void, core::ptr::null_mut());
            return ret;
        }
    }
    (*xt_nat_net).nf_nat_ops = ops;
    0
}

unsafe fn ip6t_nat_unregister_lookups(net: *mut net) {
    let xt_nat_net = net_generic(net, ip6table_nat_net_id) as *mut ip6table_nat_pernet;
    let ops = (*xt_nat_net).nf_nat_ops;
    if ops.is_null() { return; }
    for i in 0..nf_nat_ipv6_ops.len() { nf_nat_ipv6_unregister_fn(net, ops.add(i)); }
    kfree_rcu(ops as *mut c_void, core::ptr::null_mut());
}

unsafe extern "C" fn ip6table_nat_table_init(net: *mut net) -> c_int {
    let repl = ip6t_alloc_initial_table(&nf_nat_ipv6_table);
    if repl.is_null() { return -ENOMEM; }
    let mut ret = ip6t_register_table(net, &nf_nat_ipv6_table, repl, core::ptr::null_mut());
    if ret < 0 { kfree(repl as *mut c_void); return ret; }
    ret = ip6t_nat_register_lookups(net);
    if ret < 0 {
        xt_unregister_table_pre_exit(net, NFPROTO_IPV6, b"nat\0".as_ptr() as *const c_char);
        synchronize_rcu();
        ip6t_unregister_table_exit(net, b"nat\0".as_ptr() as *const c_char);
    }
    kfree(repl as *mut c_void);
    ret
}

unsafe extern "C" fn ip6table_nat_net_pre_exit(net: *mut net) {
    ip6t_nat_unregister_lookups(net);
    xt_unregister_table_pre_exit(net, NFPROTO_IPV6, b"nat\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn ip6table_nat_net_exit(net: *mut net) {
    ip6t_unregister_table_exit(net, b"nat\0".as_ptr() as *const c_char);
}

static mut ip6table_nat_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(ip6table_nat_net_pre_exit), exit: Some(ip6table_nat_net_exit),
    id: unsafe { &mut ip6table_nat_net_id }, size: core::mem::size_of::<ip6table_nat_pernet>(),
};

unsafe extern "C" fn ip6table_nat_init() -> c_int {
    let mut ret = register_pernet_subsys(&mut ip6table_nat_net_ops);
    if ret < 0 { return ret; }
    ret = xt_register_template(&nf_nat_ipv6_table, ip6table_nat_table_init);
    if ret != 0 { unregister_pernet_subsys(&mut ip6table_nat_net_ops); }
    ret
}

unsafe extern "C" fn ip6table_nat_exit() {
    xt_unregister_template(&nf_nat_ipv6_table);
    unregister_pernet_subsys(&mut ip6table_nat_net_ops);
}

// module_init(ip6table_nat_init); module_exit(ip6table_nat_exit);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Ip6tables legacy nat table");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
