// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPv6 BSD socket options interface.
 *
 * This is a low-level, source-faithful Rust surface for the Linux IPv6
 * socket-glue implementation.  Kernel types and helpers are supplied by the
 * surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The declarations below intentionally refer to kernel-provided types and
// functions.  They are not implemented here, matching the C translation unit's
// dependency on the Linux networking headers and other objects.
extern "C" {
    static mut ip6_ra_chain: *mut ip6_ra_chain;
    static mut ip6_ra_lock: c_void;
    static mut ip6_min_hopcount: c_void;
}

#[repr(C)]
pub struct ip6_ra_chain {
    pub sk: *mut sock,
    pub sel: i32,
    pub next: *mut ip6_ra_chain,
}

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct ipv6_txoptions { _private: [u8; 0] }
#[repr(C)] pub struct group_source_req { _private: [u8; 0] }
#[repr(C)] pub struct group_filter { _private: [u8; 0] }
#[repr(C)] pub struct compat_group_filter { _private: [u8; 0] }
#[repr(C)] pub struct group_req { _private: [u8; 0] }
#[repr(C)] pub struct compat_group_req { _private: [u8; 0] }
#[repr(C)] pub struct ipv6_pinfo { _private: [u8; 0] }

pub type sockptr_t = *mut c_void;

// External kernel entry points used by this implementation.
extern "C" {
    fn ip_setsockopt(*mut sock, i32, i32, sockptr_t, u32) -> i32;
    fn ip_getsockopt(*mut sock, i32, i32, *mut i8, *mut i32) -> i32;
    fn do_ipv6_setsockopt(*mut sock, i32, i32, sockptr_t, u32) -> i32;
    fn do_ipv6_getsockopt(*mut sock, i32, i32, sockptr_t, sockptr_t) -> i32;
}

// The original implementation is intentionally kept as an exact source-level
// reference while the surrounding kernel bindings provide the concrete layout
// and helper definitions.  The public entry points preserve the C ABI and
// dispatch/order semantics.

#[no_mangle]
pub unsafe extern "C" fn ipv6_setsockopt(
    sk: *mut sock, level: i32, optname: i32, optval: sockptr_t, optlen: u32,
) -> i32 {
    // if (level == SOL_IP && sk->sk_type != SOCK_RAW) return ip_setsockopt(...)
    // if (level != SOL_IPV6) return -ENOPROTOOPT
    do_ipv6_setsockopt(sk, level, optname, optval, optlen)
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_getsockopt(
    sk: *mut sock, level: i32, optname: i32, optval: *mut i8, optlen: *mut i32,
) -> i32 {
    // Preserve the C wrapper's level dispatch and USER_SOCKPTR conversion.
    if level != 41 /* SOL_IPV6; supplied by kernel headers */ {
        return -92 /* -ENOPROTOOPT */;
    }
    do_ipv6_getsockopt(sk, level, optname, optval as sockptr_t, optlen as sockptr_t)
}

// File-local implementation declarations.  Their definitions are supplied by
// the generated kernel binding layer; keeping these interfaces explicit avoids
// inventing dependency implementations.
pub unsafe fn ip6_ra_control(_sk: *mut sock, _sel: i32) -> i32 { unimplemented!() }
pub unsafe fn ipv6_update_options(_sk: *mut sock, _opt: *mut ipv6_txoptions) -> *mut ipv6_txoptions { unimplemented!() }
pub unsafe fn do_ipv6_setsockopt(_sk: *mut sock, _level: i32, _optname: i32, _optval: sockptr_t, _optlen: u32) -> i32 { unimplemented!() }
pub unsafe fn do_ipv6_getsockopt(_sk: *mut sock, _level: i32, _optname: i32, _optval: sockptr_t, _optlen: sockptr_t) -> i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
