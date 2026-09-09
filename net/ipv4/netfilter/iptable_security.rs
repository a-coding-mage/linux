// SPDX-License-Identifier: GPL-2.0-only
/*
 * "security" table
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

// Linux kernel dependencies supplied by other translation units.

const SECURITY_VALID_HOOKS: u32 = (1u32 << NF_INET_LOCAL_IN)
    | (1u32 << NF_INET_FORWARD)
    | (1u32 << NF_INET_LOCAL_OUT);

static SECURITY_TABLE: xt_table = xt_table {
    name: *b"security\0",
    valid_hooks: SECURITY_VALID_HOOKS,
    me: &THIS_MODULE,
    af: NFPROTO_IPV4,
    priority: NF_IP_PRI_SECURITY,
};

static mut SECTBL_OPS: *mut nf_hook_ops = core::ptr::null_mut();

unsafe fn iptable_security_table_init(net: *mut net) -> i32 {
    let repl: *mut ipt_replace;
    let ret: i32;

    repl = ipt_alloc_initial_table(&SECURITY_TABLE);
    if repl.is_null() {
        return -ENOMEM;
    }
    ret = ipt_register_table(net, &SECURITY_TABLE, repl, SECTBL_OPS);
    kfree(repl as *mut core::ffi::c_void);
    ret
}

unsafe fn iptable_security_net_pre_exit(net: *mut net) {
    xt_unregister_table_pre_exit(net, NFPROTO_IPV4, b"security\0".as_ptr() as *const i8);
}

unsafe fn iptable_security_net_exit(net: *mut net) {
    ipt_unregister_table_exit(net, b"security\0".as_ptr() as *const i8);
}

static mut IPTABLE_SECURITY_NET_OPS: pernet_operations = pernet_operations {
    pre_exit: Some(iptable_security_net_pre_exit),
    exit: Some(iptable_security_net_exit),
};

unsafe fn iptable_security_init() -> i32 {
    let ret: i32;

    SECTBL_OPS = xt_hook_ops_alloc(&SECURITY_TABLE, ipt_do_table);
    if is_err(SECTBL_OPS as *const core::ffi::c_void) {
        return ptr_err(SECTBL_OPS as *const core::ffi::c_void);
    }

    ret = register_pernet_subsys(&mut IPTABLE_SECURITY_NET_OPS);
    if ret < 0 {
        goto_err_free();
        return ret;
    }

    ret = xt_register_template(&SECURITY_TABLE, iptable_security_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&mut IPTABLE_SECURITY_NET_OPS);
        goto_err_free();
    }

    if ret >= 0 {
        return 0;
    }
    ret
}

unsafe fn goto_err_free() {
    kfree(SECTBL_OPS as *mut core::ffi::c_void);
}

unsafe fn iptable_security_fini() {
    xt_unregister_template(&SECURITY_TABLE);
    unregister_pernet_subsys(&mut IPTABLE_SECURITY_NET_OPS);
    kfree(SECTBL_OPS as *mut core::ffi::c_void);
}

extern "C" {
    static THIS_MODULE: module;
    static NF_INET_LOCAL_IN: u32;
    static NF_INET_FORWARD: u32;
    static NF_INET_LOCAL_OUT: u32;
    static NFPROTO_IPV4: u8;
    static NF_IP_PRI_SECURITY: i32;
    static ENOMEM: i32;

    fn ipt_alloc_initial_table(table: *const xt_table) -> *mut ipt_replace;
    fn ipt_register_table(net: *mut net, table: *const xt_table, repl: *mut ipt_replace,
                          ops: *mut nf_hook_ops) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn xt_unregister_table_pre_exit(net: *mut net, af: u8, name: *const i8);
    fn ipt_unregister_table_exit(net: *mut net, name: *const i8);
    fn xt_hook_ops_alloc(table: *const xt_table, do_table: unsafe extern "C" fn()) -> *mut nf_hook_ops;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> i32;
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
    fn xt_register_template(table: *const xt_table, init: unsafe fn(*mut net) -> i32) -> i32;
    fn xt_unregister_template(table: *const xt_table);
    fn ipt_do_table();
}

#[repr(C)] struct module;
#[repr(C)] struct net;
#[repr(C)] struct ipt_replace;
#[repr(C)] struct nf_hook_ops;
#[repr(C)] struct xt_table {
    name: [u8; 9], valid_hooks: u32, me: *const module, af: u8, priority: i32,
}
#[repr(C)] struct pernet_operations {
    pre_exit: Option<unsafe fn(*mut net)>, exit: Option<unsafe fn(*mut net)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
