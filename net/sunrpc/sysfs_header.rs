// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 Anna Schumaker <Anna.Schumaker@Netapp.com>
 */

// Translated from the C header `sysfs.h`.

#[repr(C)]
pub struct rpc_sysfs_xprt_switch {
    pub kobject: kobject,
    pub net: *mut net,
    pub xprt_switch: *mut rpc_xprt_switch,
    pub xprt: *mut rpc_xprt,
}

#[repr(C)]
pub struct rpc_sysfs_xprt {
    pub kobject: kobject,
    pub xprt: *mut rpc_xprt,
    pub xprt_switch: *mut rpc_xprt_switch,
}

extern "C" {
    pub fn rpc_sysfs_init() -> ::std::os::raw::c_int;
    pub fn rpc_sysfs_exit();

    pub fn rpc_sysfs_client_setup(
        clnt: *mut rpc_clnt,
        xprt_switch: *mut rpc_xprt_switch,
        net: *mut net,
    );
    pub fn rpc_sysfs_client_destroy(clnt: *mut rpc_clnt);
    pub fn rpc_sysfs_xprt_switch_setup(
        xprt_switch: *mut rpc_xprt_switch,
        xprt: *mut rpc_xprt,
        gfp_flags: gfp_t,
    );
    pub fn rpc_sysfs_xprt_switch_destroy(xprt_switch: *mut rpc_xprt_switch);
    pub fn rpc_sysfs_xprt_setup(
        xprt_switch: *mut rpc_xprt_switch,
        xprt: *mut rpc_xprt,
        gfp_flags: gfp_t,
    );
    pub fn rpc_sysfs_xprt_destroy(xprt: *mut rpc_xprt);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
