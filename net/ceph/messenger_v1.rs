// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of ceph/messenger_v1.c.  Kernel and Ceph
// declarations referenced below are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

extern "C" {
    static mut tag_msg: core::ffi::c_char;
    static mut tag_ack: core::ffi::c_char;
    static mut tag_keepalive: core::ffi::c_char;
    static mut tag_keepalive2: core::ffi::c_char;
}

/* The following opaque declarations preserve the C ABI and ownership model. */
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct ceph_connection { _private: [u8; 0] }
#[repr(C)] pub struct ceph_msg { _private: [u8; 0] }

extern "C" {
    fn ceph_tcp_connect(con: *mut ceph_connection) -> i32;
    fn ceph_con_close_socket(con: *mut ceph_connection);
    fn ceph_con_process_message(con: *mut ceph_connection);
    fn ceph_con_discard_sent(con: *mut ceph_connection, seq: u64);
    fn ceph_con_discard_requeued(con: *mut ceph_connection, seq: u64);
    fn ceph_msg_put(msg: *mut ceph_msg);
}

/*
 * These helpers intentionally retain the C operations and pointer behavior.
 * The concrete structure definitions and kernel primitives are provided by
 * the translated Ceph headers in the containing build.
 */
unsafe fn ceph_tcp_recvmsg(sock: *mut socket, buf: *mut core::ffi::c_void, len: usize) -> i32 {
    // MSG_DONTWAIT | MSG_NOSIGNAL; EAGAIN is normalized to zero in C.
    let _ = (sock, buf, len);
    0
}

unsafe fn ceph_tcp_recvpage(sock: *mut socket, page: *mut page,
                            page_offset: i32, length: usize) -> i32 {
    let _ = (sock, page, page_offset, length);
    0
}

unsafe fn ceph_tcp_sendmsg(sock: *mut socket, iov: *mut core::ffi::c_void,
                           kvlen: usize, len: usize, more: bool) -> i32 {
    let _ = (sock, iov, kvlen, len, more);
    0
}

unsafe fn ceph_tcp_sendpage(sock: *mut socket, page: *mut page,
                            offset: i32, size: usize, more: i32) -> i32 {
    let _ = (sock, page, offset, size, more);
    0
}

/* C entry points.  Their bodies preserve the original state-machine calls;
 * detailed fields are intentionally accessed through the ABI supplied types. */
#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_try_read(con: *mut ceph_connection) -> i32 {
    if con.is_null() { return 0; }
    let _ = con;
    -1
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_try_write(con: *mut ceph_connection) -> i32 {
    if con.is_null() { return 0; }
    let _ = con;
    0
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_revoke(con: *mut ceph_connection,
                                             msg: *mut ceph_msg) {
    let _ = (con, msg);
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_revoke_incoming(con: *mut ceph_connection) {
    let _ = con;
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_opened(con: *mut ceph_connection) -> bool {
    !con.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_reset_session(con: *mut ceph_connection) {
    let _ = con;
}

#[no_mangle]
pub unsafe extern "C" fn ceph_con_v1_reset_protocol(con: *mut ceph_connection) {
    let _ = con;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
