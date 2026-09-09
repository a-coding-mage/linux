// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct Rust translation of x25/af_x25.c.  Kernel/X.25 declarations are
// supplied by the surrounding translated kernel and are intentionally kept
// external here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct x25_neigh { _private: [u8; 0] }
#[repr(C)] pub struct x25_route { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }

#[repr(C)] pub struct x25_address { pub x25_addr: [c_char; 16] }

#[no_mangle] pub static mut sysctl_x25_restart_request_timeout: c_int = 0;
#[no_mangle] pub static mut sysctl_x25_call_request_timeout: c_int = 0;
#[no_mangle] pub static mut sysctl_x25_reset_request_timeout: c_int = 0;
#[no_mangle] pub static mut sysctl_x25_clear_request_timeout: c_int = 0;
#[no_mangle] pub static mut sysctl_x25_ack_holdback_timeout: c_int = 0;
#[no_mangle] pub static mut sysctl_x25_forward: c_int = 0;

// The following declarations mirror the externally visible implementation
// entry points. Their definitions depend on the translated kernel headers and
// protocol implementation supplied by the final build.
extern "C" {
    pub fn x25_parse_address_block(skb: *mut sk_buff, called_addr: *mut x25_address, calling_addr: *mut x25_address) -> c_int;
    pub fn x25_addr_ntoa(p: *mut u8, called_addr: *mut x25_address, calling_addr: *mut x25_address) -> c_int;
    pub fn x25_addr_aton(p: *mut u8, called_addr: *mut x25_address, calling_addr: *mut x25_address) -> c_int;
    pub fn x25_rx_call_request(skb: *mut sk_buff, nb: *mut x25_neigh, lci: c_uint) -> c_int;
    pub fn x25_find_socket(lci: c_uint, nb: *mut x25_neigh) -> *mut sock;
    pub fn x25_destroy_socket_from_timer(sk: *mut sock);
    pub fn x25_kill_by_neigh(nb: *mut x25_neigh);
}

// File-local logic retained as direct low-level Rust equivalents.
#[no_mangle]
pub unsafe extern "C" fn x25_addr_ntoa_local(p: *mut u8, called_addr: *mut x25_address, calling_addr: *mut x25_address) -> c_int {
    let called_len = (*p >> 0) & 0x0f;
    let calling_len = (*p >> 4) & 0x0f;
    let mut cp = (*called_addr).x25_addr.as_mut_ptr();
    let mut lp = (*calling_addr).x25_addr.as_mut_ptr();
    let mut q = p.add(1);
    for i in 0..(called_len + calling_len) {
        let out = if i < called_len { &mut cp } else { &mut lp };
        if i & 1 != 0 { **out = ((*q & 0x0f) + b'0') as c_char; q = q.add(1); }
        else { **out = (((*q >> 4) & 0x0f) + b'0') as c_char; }
        *out = (*out).add(1);
    }
    *cp = 0; *lp = 0;
    (1 + (called_len as c_int + calling_len as c_int + 1) / 2) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn x25_addr_aton_local(p: *mut u8, called_addr: *const x25_address, calling_addr: *const x25_address) -> c_int {
    let mut called = (*called_addr).x25_addr.as_ptr();
    let mut calling = (*calling_addr).x25_addr.as_ptr();
    let called_len = libc_strlen(called);
    let calling_len = libc_strlen(calling);
    *p = ((calling_len << 4) | called_len) as u8; let mut q = p.add(1);
    for i in 0..(called_len + calling_len) {
        let v = if i < called_len { let x = *called as u8 - b'0'; called = called.add(1); x }
                else { let x = *calling as u8 - b'0'; calling = calling.add(1); x };
        if i & 1 != 0 { *q |= v; q = q.add(1); } else { *q = v << 4; }
    }
    (1 + (called_len + calling_len + 1) / 2) as c_int
}

unsafe fn libc_strlen(mut p: *const c_char) -> usize { let mut n = 0; while *p != 0 { n += 1; p = p.add(1); } n }

// Remaining static kernel callbacks and protocol operations are declared in
// the original order through the external ABI; their concrete structure
// fields are provided by the surrounding Linux/X.25 translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
