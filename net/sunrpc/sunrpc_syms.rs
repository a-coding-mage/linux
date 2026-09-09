// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/sunrpc_syms.c
 *
 * Symbols exported by the sunrpc module.
 *
 * Copyright (C) 1997 Olaf Kirch <okir@monad.swb.de>
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::mem::size_of;

extern "C" {
    static mut sunrpc_nl_family: genl_family;
    static mut sunrpc_net_id: u32;

    fn rpc_proc_init(net: *mut net) -> i32;
    fn ip_map_cache_create(net: *mut net) -> i32;
    fn unix_gid_cache_create(net: *mut net) -> i32;
    fn rpc_pipefs_init_net(net: *mut net) -> i32;
    fn unix_gid_cache_destroy(net: *mut net);
    fn ip_map_cache_destroy(net: *mut net);
    fn rpc_proc_exit(net: *mut net);
    fn rpc_pipefs_exit_net(net: *mut net);
    fn rpc_init_mempool() -> i32;
    fn rpcauth_init_module() -> i32;
    fn cache_initialize();
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
    fn register_rpc_pipefs() -> i32;
    fn rpc_sysfs_init() -> i32;
    fn genl_register_family(family: *mut genl_family) -> i32;
    fn sunrpc_debugfs_init();
    fn rpc_register_sysctl();
    fn svc_init_xprt_sock();
    fn init_socket_xprt();
    fn rpc_sysfs_exit();
    fn unregister_rpc_pipefs();
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
    fn rpcauth_remove_module();
    fn rpc_destroy_mempool();
    fn genl_unregister_family(family: *mut genl_family);
    fn rpc_cleanup_clids();
    fn xprt_cleanup_ids();
    fn xprt_multipath_cleanup_ids();
    fn cleanup_socket_xprt();
    fn svc_cleanup_xprt_sock();
    fn sunrpc_debugfs_exit();
    fn auth_domain_cleanup();
    fn rpc_unregister_sysctl();
    fn rcu_barrier();
    fn net_generic(net: *mut net, id: u32) -> *mut sunrpc_net;
    fn init_list_head(head: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn list_empty(head: *const list_head) -> bool;
    fn warn_on_once(condition: bool);
}

#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct genl_family { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct sunrpc_net {
    pub all_clients: list_head,
    pub rpc_client_lock: spinlock_t,
    pub rpcb_clnt_lock: spinlock_t,
    pub gssp_lock: mutex,
}

#[repr(C)]
pub struct pernet_operations {
    pub init: Option<unsafe extern "C" fn(*mut net) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut net)>,
    pub id: *mut u32,
    pub size: usize,
}

#[no_mangle]
pub static mut SUNRPC_NET_ID: u32 = 0;

// EXPORT_SYMBOL_GPL(sunrpc_net_id);

unsafe extern "C" fn sunrpc_init_net(net: *mut net) -> i32 {
    let mut err: i32;
    let sn = net_generic(net, SUNRPC_NET_ID);

    err = rpc_proc_init(net);
    if err != 0 { return err; }

    err = ip_map_cache_create(net);
    if err != 0 { rpc_proc_exit(net); return err; }

    err = unix_gid_cache_create(net);
    if err != 0 { ip_map_cache_destroy(net); rpc_proc_exit(net); return err; }

    err = rpc_pipefs_init_net(net);
    if err != 0 { unix_gid_cache_destroy(net); ip_map_cache_destroy(net); rpc_proc_exit(net); return err; }

    init_list_head(&mut (*sn).all_clients);
    spin_lock_init(&mut (*sn).rpc_client_lock);
    spin_lock_init(&mut (*sn).rpcb_clnt_lock);
    mutex_init(&mut (*sn).gssp_lock);
    0
}

unsafe extern "C" fn sunrpc_exit_net(net: *mut net) {
    let sn = net_generic(net, SUNRPC_NET_ID);
    rpc_pipefs_exit_net(net);
    unix_gid_cache_destroy(net);
    ip_map_cache_destroy(net);
    rpc_proc_exit(net);
    warn_on_once(!list_empty(&(*sn).all_clients));
}

static mut SUNRPC_NET_OPS: pernet_operations = pernet_operations {
    init: Some(sunrpc_init_net),
    exit: Some(sunrpc_exit_net),
    id: unsafe { &raw mut SUNRPC_NET_ID },
    size: size_of::<sunrpc_net>(),
};

unsafe extern "C" fn init_sunrpc() -> i32 {
    let mut err = rpc_init_mempool();
    if err != 0 { return err; }
    err = rpcauth_init_module();
    if err != 0 { rpc_destroy_mempool(); return err; }

    cache_initialize();
    err = register_pernet_subsys(&raw mut SUNRPC_NET_OPS);
    if err != 0 { rpcauth_remove_module(); rpc_destroy_mempool(); return err; }
    err = register_rpc_pipefs();
    if err != 0 { unregister_pernet_subsys(&raw mut SUNRPC_NET_OPS); rpcauth_remove_module(); rpc_destroy_mempool(); return err; }
    err = rpc_sysfs_init();
    if err != 0 { unregister_rpc_pipefs(); unregister_pernet_subsys(&raw mut SUNRPC_NET_OPS); rpcauth_remove_module(); rpc_destroy_mempool(); return err; }
    err = genl_register_family(&raw mut sunrpc_nl_family);
    if err != 0 { rpc_sysfs_exit(); unregister_rpc_pipefs(); unregister_pernet_subsys(&raw mut SUNRPC_NET_OPS); rpcauth_remove_module(); rpc_destroy_mempool(); return err; }
    sunrpc_debugfs_init();
    // #if IS_ENABLED(CONFIG_SUNRPC_DEBUG)
    rpc_register_sysctl();
    // #endif
    svc_init_xprt_sock();
    init_socket_xprt();
    0
}

unsafe extern "C" fn cleanup_sunrpc() {
    genl_unregister_family(&raw mut sunrpc_nl_family);
    rpc_sysfs_exit();
    rpc_cleanup_clids();
    xprt_cleanup_ids();
    xprt_multipath_cleanup_ids();
    rpcauth_remove_module();
    cleanup_socket_xprt();
    svc_cleanup_xprt_sock();
    sunrpc_debugfs_exit();
    unregister_rpc_pipefs();
    rpc_destroy_mempool();
    unregister_pernet_subsys(&raw mut SUNRPC_NET_OPS);
    auth_domain_cleanup();
    // #if IS_ENABLED(CONFIG_SUNRPC_DEBUG)
    rpc_unregister_sysctl();
    // #endif
    rcu_barrier();
}

// MODULE_DESCRIPTION("Sun RPC core");
// MODULE_LICENSE("GPL");
// fs_initcall(init_sunrpc); /* Ensure we're initialised before nfs */
// module_exit(cleanup_sunrpc);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
