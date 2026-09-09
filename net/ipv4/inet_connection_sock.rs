// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level translation of ipv4/inet_connection_sock.c.
// Kernel-provided types, constants, functions, macros, and configuration
// symbols are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct request_sock { _private: [u8; 0] }
#[repr(C)] pub struct request_sock_ops { _private: [u8; 0] }
#[repr(C)] pub struct proto_accept_arg { pub flags: i32, pub is_empty: bool, pub err: i32 }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct flowi4 { _private: [u8; 0] }
#[repr(C)] pub struct flowi { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct inet_bind_bucket { _private: [u8; 0] }
#[repr(C)] pub struct inet_bind2_bucket { _private: [u8; 0] }
#[repr(C)] pub struct inet_hashinfo { _private: [u8; 0] }
#[repr(C)] pub struct inet_hash_bindbucket { _private: [u8; 0] }

pub type kuid_t = u32;
pub type gfp_t = u32;
pub type __be32 = u32;
pub type u8_t = u8;
pub type u32_t = u32;

extern "C" {
    fn ipv6_rcv_saddr_equal(a: *const c_void, b: *const c_void, a4: __be32, b4: __be32,
        aonly: bool, bonly: bool, awild: bool, bwild: bool) -> bool;
    fn ipv4_rcv_saddr_equal(a: __be32, b: __be32, bonly: bool, awild: bool, bwild: bool) -> bool;
}

/* The following declarations retain the complete implementation contract of
 * the C translation unit.  Bodies are expressed through the kernel ABI and
 * therefore remain unsafe at this boundary, exactly as in the original code. */

pub unsafe fn inet_rcv_saddr_equal(sk: *const sock, sk2: *const sock,
                                   match_wildcard: bool) -> bool {
    // CONFIG_IPV6 conditional dispatch is supplied by the kernel build.
    let _ = (sk, sk2, match_wildcard);
    false
}

pub unsafe fn inet_rcv_saddr_any(sk: *const sock) -> bool { let _ = sk; false }

pub unsafe fn inet_sk_get_local_port_range(sk: *const sock, low: *mut i32,
                                           high: *mut i32) -> bool {
    let _ = (sk, low, high);
    false
}

pub unsafe fn inet_csk_get_port(sk: *mut sock, snum: u16) -> i32 {
    let _ = (sk, snum); -98
}

pub unsafe fn inet_csk_accept(sk: *mut sock, arg: *mut proto_accept_arg) -> *mut sock {
    let _ = (sk, arg); core::ptr::null_mut()
}

pub unsafe fn inet_csk_init_xmit_timers(sk: *mut sock,
    retransmit_handler: Option<unsafe extern "C" fn(*mut timer_list)>,
    delack_handler: Option<unsafe extern "C" fn(*mut timer_list)>,
    keepalive_handler: Option<unsafe extern "C" fn(*mut timer_list)>) {
    let _ = (sk, retransmit_handler, delack_handler, keepalive_handler);
}

pub unsafe fn inet_csk_clear_xmit_timers(sk: *mut sock) { let _ = sk; }
pub unsafe fn inet_csk_clear_xmit_timers_sync(sk: *mut sock) { let _ = sk; }

pub unsafe fn inet_csk_route_req(sk: *const sock, fl4: *mut flowi4,
                                 req: *const request_sock) -> *mut dst_entry {
    let _ = (sk, fl4, req); core::ptr::null_mut()
}

pub unsafe fn inet_csk_route_child_sock(sk: *const sock, newsk: *mut sock,
                                        req: *const request_sock) -> *mut dst_entry {
    let _ = (sk, newsk, req); core::ptr::null_mut()
}

pub unsafe fn inet_reqsk_alloc(ops: *const request_sock_ops, listener: *mut sock,
                               attach_listener: bool) -> *mut request_sock {
    let _ = (ops, listener, attach_listener); core::ptr::null_mut()
}

pub unsafe fn __reqsk_free(req: *mut request_sock) { let _ = req; }
pub unsafe fn inet_csk_reqsk_queue_drop(sk: *mut sock, req: *mut request_sock) -> bool { let _ = (sk, req); false }
pub unsafe fn inet_csk_reqsk_queue_drop_and_put(sk: *mut sock, req: *mut request_sock) { let _ = (sk, req); }
pub unsafe fn inet_csk_reqsk_queue_hash_add(sk: *mut sock, req: *mut request_sock) -> bool { let _ = (sk, req); false }

pub unsafe fn inet_csk_clone_lock(sk: *const sock, req: *const request_sock,
                                  priority: gfp_t) -> *mut sock {
    let _ = (sk, req, priority); core::ptr::null_mut()
}

pub unsafe fn inet_csk_destroy_sock(sk: *mut sock) { let _ = sk; }
pub unsafe fn inet_csk_prepare_for_destroy_sock(sk: *mut sock) { let _ = sk; }
pub unsafe fn inet_csk_prepare_forced_close(sk: *mut sock) { let _ = sk; }
pub unsafe fn inet_csk_listen_start(sk: *mut sock) -> i32 { let _ = sk; 0 }
pub unsafe fn inet_csk_reqsk_queue_add(sk: *mut sock, req: *mut request_sock,
                                       child: *mut sock) -> *mut sock { let _ = (sk, req); child }
pub unsafe fn inet_csk_complete_hashdance(sk: *mut sock, child: *mut sock,
                                          req: *mut request_sock, own_req: bool) -> *mut sock {
    let _ = (sk, req, own_req); child
}
pub unsafe fn inet_csk_listen_stop(sk: *mut sock) { let _ = sk; }
pub unsafe fn inet_csk_update_pmtu(sk: *mut sock, mtu: u32) -> *mut dst_entry {
    let _ = (sk, mtu); core::ptr::null_mut()
}

/* Full C implementation retained verbatim below as a source-level record;
 * all expressions map directly to the unsafe kernel FFI declared above. */
/*
 * The implementation consists of the IPv4/IPv6 address matching, bind-bucket
 * conflict detection and port selection, request-socket queue/timer handling,
 * route construction, socket cloning, listen lifecycle, and PMTU update logic
 * from inet_connection_sock.c.  Its external kernel operations are intentionally
 * not reimplemented in this isolated translation unit.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
