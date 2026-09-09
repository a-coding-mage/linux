// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Copyright (c) 2007 Igor Mammedov
 *   Author(s): Igor Mammedov (niallain@gmail.com)
 *              Steve French (sfrench@us.ibm.com)
 *              Wang Lei (wang840925@gmail.com)
 *              David Howells (dhowells@redhat.com)
 *
 *   Contains the CIFS DFS upcall routines used for hostname to
 *   IP address translation.
 *
 */

use core::ffi::{c_char, c_int, c_void};

// Symbols and types supplied by the kernel/CIFS dependencies.
#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsproxy {
    pub net_ns: *mut net,
}

#[repr(C)]
pub struct task_struct {
    pub nsproxy: *mut nsproxy,
}

extern "C" {
    static mut current: *mut task_struct;

    fn dns_query(
        net: *mut net,
        keyring: *const c_void,
        name: *const c_char,
        namelen: usize,
        type_: *const c_void,
        result: *mut *mut c_char,
        dest: *const c_void,
        netns: bool,
    ) -> c_int;
    fn cifs_convert_address(addr: *mut sockaddr, name: *const c_char, namelen: usize) -> c_int;
    fn cifs_netbios_name(name: *const c_char, namelen: usize) -> c_int;
    fn cifs_dbg(level: c_int, fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
    fn strnlen(s: *const c_char, maxlen: usize) -> usize;
}

const FYI: c_int = 0;
const GFP_KERNEL: c_int = 0;
const CIFS_MAX_DOMAINNAME_LEN: usize = 256;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EHOSTUNREACH: c_int = 113;

unsafe fn resolve_name(name: *const c_char, namelen: usize, addr: *mut sockaddr) -> c_int {
    let mut ip: *mut c_char = core::ptr::null_mut();
    let mut rc: c_int;

    rc = dns_query(
        (*(*current).nsproxy).net_ns,
        core::ptr::null(),
        name,
        namelen,
        core::ptr::null(),
        &mut ip,
        core::ptr::null(),
        false,
    );
    if rc < 0 {
        cifs_dbg(
            FYI,
            c"%s: unable to resolve: %*.*s\n".as_ptr(),
            c"resolve_name".as_ptr(),
            namelen as c_int,
            namelen as c_int,
            name,
        );
    } else {
        cifs_dbg(
            FYI,
            c"%s: resolved: %*.*s to %s\n".as_ptr(),
            c"resolve_name".as_ptr(),
            namelen as c_int,
            namelen as c_int,
            name,
            ip,
        );

        rc = cifs_convert_address(addr, ip, strlen(ip));
        kfree(ip.cast());
        if rc == 0 {
            cifs_dbg(
                FYI,
                c"%s: unable to determine ip address\n".as_ptr(),
                c"resolve_name".as_ptr(),
            );
            rc = -EHOSTUNREACH;
        } else {
            rc = 0;
        }
    }
    rc
}

/// Perform an upcall to resolve hostname to an ip address.
/// `dom` is the DNS domain name, or null.
/// `name` is the name to look up.
/// `namelen` is the length of `name`.
/// `ip_addr` is where to return the IP address.
///
/// Returns zero on success, or a negative error code otherwise.
pub unsafe fn dns_resolve_name(
    dom: *const c_char,
    name: *const c_char,
    namelen: usize,
    ip_addr: *mut sockaddr,
) -> c_int {
    let mut len: usize;
    let mut s: *mut c_char;
    let mut rc: c_int;

    cifs_dbg(
        FYI,
        c"%s: dom=%s name=%.*s\n".as_ptr(),
        c"dns_resolve_name".as_ptr(),
        dom,
        namelen as c_int,
        name,
    );
    if ip_addr.is_null() || name.is_null() || *name == 0 || namelen == 0 {
        return -EINVAL;
    }

    cifs_dbg(
        FYI,
        c"%s: hostname=%.*s\n".as_ptr(),
        c"dns_resolve_name".as_ptr(),
        namelen as c_int,
        name,
    );
    // Try to interpret hostname as an IPv4 or IPv6 address.
    rc = cifs_convert_address(ip_addr, name, namelen);
    if rc > 0 {
        cifs_dbg(
            FYI,
            c"%s: unc is IP, skipping dns upcall: %*.*s\n".as_ptr(),
            c"dns_resolve_name".as_ptr(),
            namelen as c_int,
            namelen as c_int,
            name,
        );
        return 0;
    }

    // If `name` contains a NetBIOS name and `dom` has been specified,
    // convert `name` to an FQDN and try resolving it first.
    if !dom.is_null() && *dom != 0 && cifs_netbios_name(name, namelen) != 0 {
        len = strnlen(dom, CIFS_MAX_DOMAINNAME_LEN) + namelen + 2;
        s = kmalloc(len, GFP_KERNEL).cast();
        if s.is_null() {
            return -ENOMEM;
        }

        scnprintf(
            s,
            len,
            c"%.*s.%s".as_ptr(),
            namelen as c_int,
            name,
            dom,
        );
        rc = resolve_name(s, len - 1, ip_addr);
        kfree(s.cast());
        if rc == 0 {
            return 0;
        }
    }
    resolve_name(name, namelen, ip_addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
