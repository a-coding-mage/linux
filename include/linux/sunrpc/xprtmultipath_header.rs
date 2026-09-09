/* SPDX-License-Identifier: GPL-2.0 */
/*
 * RPC client multipathing definitions
 *
 * Copyright (c) 2015, 2016, Primary Data, Inc. All rights reserved.
 *
 * Trond Myklebust <trond.myklebust@primarydata.com>
 */

// Forward declarations supplied by other translation units.
pub struct rpc_xprt_iter_ops;
pub struct rpc_sysfs_xprt_switch;

#[repr(C)]
pub struct rpc_xprt_switch {
    pub xps_lock: spinlock_t,
    pub xps_kref: kref,

    pub xps_id: ::core::ffi::c_uint,
    pub xps_nxprts: ::core::ffi::c_uint,
    pub xps_nactive: ::core::ffi::c_uint,
    pub xps_nunique_destaddr_xprts: ::core::ffi::c_uint,
    pub xps_queuelen: atomic_long_t,
    pub xps_xprt_list: list_head,

    pub xps_net: *mut net,

    pub xps_iter_ops: *const rpc_xprt_iter_ops,

    pub xps_sysfs: *mut rpc_sysfs_xprt_switch,
    pub xps_rcu: rcu_head,
}

#[repr(C)]
pub struct rpc_xprt_iter {
    // __rcu annotation preserved as a comment; synchronization is supplied externally.
    pub xpi_xpswitch: *mut rpc_xprt_switch,
    pub xpi_cursor: *mut rpc_xprt,

    pub xpi_ops: *const rpc_xprt_iter_ops,
}

#[repr(C)]
pub struct rpc_xprt_iter_ops {
    pub xpi_rewind: Option<unsafe extern "C" fn(*mut rpc_xprt_iter)>,
    pub xpi_xprt: Option<unsafe extern "C" fn(*mut rpc_xprt_iter) -> *mut rpc_xprt>,
    pub xpi_next: Option<unsafe extern "C" fn(*mut rpc_xprt_iter) -> *mut rpc_xprt>,
}

extern "C" {
    pub fn xprt_switch_alloc(
        xprt: *mut rpc_xprt,
        gfp_flags: gfp_t,
    ) -> *mut rpc_xprt_switch;

    pub fn xprt_switch_get(xps: *mut rpc_xprt_switch) -> *mut rpc_xprt_switch;
    pub fn xprt_switch_put(xps: *mut rpc_xprt_switch);

    pub fn rpc_xprt_switch_set_roundrobin(xps: *mut rpc_xprt_switch);

    pub fn rpc_xprt_switch_add_xprt(xps: *mut rpc_xprt_switch, xprt: *mut rpc_xprt);
    pub fn rpc_xprt_switch_remove_xprt(
        xps: *mut rpc_xprt_switch,
        xprt: *mut rpc_xprt,
        offline: bool,
    );
    pub fn rpc_xprt_switch_get_main_xprt(xps: *mut rpc_xprt_switch) -> *mut rpc_xprt;

    pub fn xprt_iter_init(xpi: *mut rpc_xprt_iter, xps: *mut rpc_xprt_switch);
    pub fn xprt_iter_init_listall(xpi: *mut rpc_xprt_iter, xps: *mut rpc_xprt_switch);
    pub fn xprt_iter_init_listoffline(xpi: *mut rpc_xprt_iter, xps: *mut rpc_xprt_switch);
    pub fn xprt_iter_destroy(xpi: *mut rpc_xprt_iter);
    pub fn xprt_iter_rewind(xpi: *mut rpc_xprt_iter);

    pub fn xprt_iter_xchg_switch(
        xpi: *mut rpc_xprt_iter,
        newswitch: *mut rpc_xprt_switch,
    ) -> *mut rpc_xprt_switch;

    pub fn xprt_iter_xprt(xpi: *mut rpc_xprt_iter) -> *mut rpc_xprt;
    pub fn xprt_iter_get_next(xpi: *mut rpc_xprt_iter) -> *mut rpc_xprt;

    pub fn rpc_xprt_switch_has_addr(
        xps: *mut rpc_xprt_switch,
        sap: *const sockaddr,
    ) -> bool;

    pub fn xprt_multipath_cleanup_ids();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
