/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *   DNS Resolver upcall management for CIFS DFS
 *   Handles host name to IP address resolution
 *
 *   Copyright (c) International Business Machines  Corp., 2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */

use core::ffi::{c_char, c_int};

// Supplied by the surrounding kernel/CIFS translation units.
#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dns_resolve_name(
        dom: *const c_char,
        name: *const c_char,
        namelen: usize,
        ip_addr: *mut sockaddr,
    ) -> c_int;

    fn strlen(s: *const c_char) -> usize;

    fn extract_unc_hostname(
        unc: *const c_char,
        name: *mut *const c_char,
        namelen: *mut usize,
    );
}

#[inline]
pub unsafe fn dns_resolve_unc(
    dom: *const c_char,
    unc: *const c_char,
    ip_addr: *mut sockaddr,
) -> c_int {
    let mut name: *const c_char;
    let mut namelen: usize;

    if unc.is_null() || strlen(unc) < 3 {
        return -22; // -EINVAL
    }

    extract_unc_hostname(unc, &mut name, &mut namelen);
    if namelen == 0 {
        return -22; // -EINVAL
    }

    dns_resolve_name(dom, name, namelen, ip_addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
