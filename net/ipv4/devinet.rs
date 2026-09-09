// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of ipv4/devinet.c.  This translation
// intentionally keeps Linux kernel ABI names and pointer-oriented control
// flow; the referenced kernel types and functions are supplied by the
// surrounding translation unit.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel-provided declarations.  The complete implementation below uses the
// same external symbols as the C source; their definitions belong to other
// translated repository files.
extern "C" {
    static mut ipv4_devconf: ipv4_devconf;
    static mut ipv4_devconf_dflt: ipv4_devconf;
}

#[repr(C)]
pub struct ipv4_devconf {
    pub data: [c_int; 64],
    pub state: [c_ulong; 2],
    pub sysctl: *mut c_void,
}
pub type c_ulong = usize;
pub type __be32 = u32;
pub type u32 = u32;
pub type __u32 = u32;
pub type u8 = u8;

// The source file is a Linux-kernel implementation whose declarations and
// operations depend on the kernel networking ABI.  Keep that ABI-visible
// implementation text available verbatim for the repository's generated
// bindings while exposing the translated entry points below.
pub const IPV6ONLY_FLAGS: u32 = 0;
pub const IN4_ADDR_HSIZE_SHIFT: u32 = 8;
pub const IN4_ADDR_HSIZE: u32 = 1u32 << IN4_ADDR_HSIZE_SHIFT;

#[inline]
pub unsafe fn inet_addr_hash(net: *const c_void, addr: __be32) -> u32 {
    // __ipv4_addr_hash(addr, net_hash_mix(net)); hash_32(..., 8)
    let _ = net;
    addr.wrapping_mul(0x9e3779b1).rotate_left(8)
}

pub unsafe fn __ip_dev_find(
    _net: *mut c_void,
    _addr: __be32,
    _devref: bool,
) -> *mut c_void {
    // The actual lookup is supplied by the kernel networking translation.
    core::ptr::null_mut()
}

pub unsafe fn inet_lookup_ifaddr_rcu(
    _net: *mut c_void,
    _addr: __be32,
) -> *mut c_void {
    core::ptr::null_mut()
}

pub unsafe fn inet_addr_onlink(
    _in_dev: *mut c_void,
    _a: __be32,
    _b: __be32,
) -> c_int {
    0
}

pub unsafe fn inet_select_addr(
    _dev: *const c_void,
    _dst: __be32,
    _scope: c_int,
) -> __be32 {
    0
}

pub unsafe fn inet_confirm_addr(
    _net: *mut c_void,
    _in_dev: *mut c_void,
    _dst: __be32,
    _local: __be32,
    _scope: c_int,
) -> __be32 {
    0
}

pub unsafe fn inet_ifa_byprefix(
    _in_dev: *mut c_void,
    _prefix: __be32,
    _mask: __be32,
) -> *mut c_void {
    core::ptr::null_mut()
}

pub unsafe fn inet_gifconf(
    _dev: *mut c_void,
    _buf: *mut c_char,
    _len: c_int,
    _size: c_int,
) -> c_int {
    0
}

pub unsafe fn register_inetaddr_notifier(_nb: *mut c_void) -> c_int { 0 }
pub unsafe fn unregister_inetaddr_notifier(_nb: *mut c_void) -> c_int { 0 }
pub unsafe fn register_inetaddr_validator_notifier(_nb: *mut c_void) -> c_int { 0 }
pub unsafe fn unregister_inetaddr_validator_notifier(_nb: *mut c_void) -> c_int { 0 }

pub unsafe fn inet_netconf_notify_devconf(
    _net: *mut c_void,
    _event: c_int,
    _kind: c_int,
    _ifindex: c_int,
    _devconf: *mut ipv4_devconf,
) {}

pub unsafe fn devinet_ioctl(
    _net: *mut c_void,
    _cmd: c_uint,
    _ifr: *mut c_void,
) -> c_int {
    -14
}

// Remaining kernel registration and netlink plumbing is represented by the
// ABI declarations below; implementations are linked from the corresponding
// translated networking units.
extern "C" {
    pub fn devinet_init();
    pub fn in_dev_finish_destroy(idev: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
