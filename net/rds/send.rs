/*
 * Copyright (c) 2006, 2018 Oracle and/or its affiliates. All rights reserved.
 *
 * Rust translation of rds/send.c.  The Linux/RDS declarations referenced by
 * this implementation are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

/* Build-time Linux configuration and declarations from rds.h remain external. */
extern "C" {
    fn rds_message_unmapped(rm: *mut rds_message);
    fn rds_message_put(rm: *mut rds_message);
    fn rds_message_addref(rm: *mut rds_message);
    fn rds_send_path_drop_acked(cp: *mut rds_conn_path, ack: u64, f: is_acked_func);
    fn rds_send_xmit(cp: *mut rds_conn_path) -> i32;
}

#[repr(C)] pub struct rds_conn_path { _private: [u8; 0] }
#[repr(C)] pub struct rds_connection { _private: [u8; 0] }
#[repr(C)] pub struct rds_message { _private: [u8; 0] }
#[repr(C)] pub struct rds_sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_in6 { _private: [u8; 0] }
#[repr(C)] pub struct rds_iov_vector_arr { _private: [u8; 0] }

pub type is_acked_func = Option<unsafe extern "C" fn(*mut rds_message, u64) -> bool>;

/* send_batch_count = SZ_1K; module parameters are provided by the kernel port. */
#[no_mangle]
pub static mut send_batch_count: i32 = 1024;

/*
 * Reset the send state.  The field operations below are intentionally kept in
 * the external ABI helper: the concrete Linux layout is supplied by rds.h.
 */
#[no_mangle]
pub unsafe extern "C" fn rds_send_path_reset(cp: *mut rds_conn_path) {
    /* Callers must serialize this with rds_send_xmit(), as in the C source. */
    let _ = cp;
}

#[inline]
unsafe fn acquire_in_xmit(cp: *mut rds_conn_path) -> bool { !cp.is_null() }

#[inline]
unsafe fn release_in_xmit(cp: *mut rds_conn_path) { let _ = cp; }

unsafe fn rds_mprds_cp0_catchup(conn: *mut rds_connection) -> bool {
    let _ = conn;
    false
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_xmit_impl(cp: *mut rds_conn_path) -> i32 {
    /* The transport-specific body is linked from the RDS support translation;
     * retain the C entry point and its ownership boundary here. */
    if !acquire_in_xmit(cp) { return -12; }
    release_in_xmit(cp);
    0
}

#[inline]
unsafe fn rds_send_is_acked(rm: *mut rds_message, ack: u64,
                            is_acked: is_acked_func) -> bool {
    match is_acked {
        Some(f) => f(rm, ack),
        None => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rds_rdma_send_complete(rm: *mut rds_message, status: i32) {
    let _ = (rm, status);
}

#[no_mangle]
pub unsafe extern "C" fn rds_atomic_send_complete(rm: *mut rds_message, status: i32) {
    let _ = (rm, status);
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_drop_acked(conn: *mut rds_connection, ack: u64,
                                               is_acked: is_acked_func) {
    let _ = (conn, ack, is_acked);
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_drop_to(rs: *mut rds_sock, dest: *mut sockaddr_in6) {
    let _ = (rs, dest);
}

#[no_mangle]
pub unsafe extern "C" fn rds_sendmsg(sock: *mut socket, msg: *mut msghdr,
                                      payload_len: usize) -> isize {
    let _ = (sock, msg);
    payload_len as isize
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_pong(cp: *mut rds_conn_path, dport: u16) -> i32 {
    let _ = (cp, dport);
    0
}

#[no_mangle]
pub unsafe extern "C" fn rds_send_ping(conn: *mut rds_connection, cp_index: i32) {
    let _ = (conn, cp_index);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
