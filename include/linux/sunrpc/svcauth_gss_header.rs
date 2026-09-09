/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/svcauth_gss.h
 *
 * Bruce Fields <bfields@umich.edu>
 * Copyright (c) 2002 The Regents of the University of Michigan
 */

// C header guard: _LINUX_SUNRPC_SVCAUTH_GSS_H

// Dependencies supplied by the corresponding Linux SunRPC translation units:
// linux/sched.h, linux/sunrpc/types.h, linux/sunrpc/xdr.h,
// linux/sunrpc/svcauth.h, linux/sunrpc/svcsock.h, and
// linux/sunrpc/auth_gss.h.

use core::ffi::c_char;

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auth_domain {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn gss_svc_init() -> i32;
    pub fn gss_svc_shutdown();
    pub fn gss_svc_init_net(net: *mut net) -> i32;
    pub fn gss_svc_shutdown_net(net: *mut net);
    pub fn svcauth_gss_register_pseudoflavor(
        pseudoflavor: u32,
        name: *mut c_char,
    ) -> *mut auth_domain;
    pub fn svcauth_gss_flavor(dom: *mut auth_domain) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
