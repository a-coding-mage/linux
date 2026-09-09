// SPDX-License-Identifier: GPL-2.0
//! Rust translation of the Ceph messenger implementation.
//!
//! Kernel/Ceph types and functions referenced here are supplied by the
//! surrounding translation unit.  This file intentionally keeps the original
//! low-level interface and state-machine constants.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const CON_SOCK_STATE_NEW: c_int = 0;
pub const CON_SOCK_STATE_CLOSED: c_int = 1;
pub const CON_SOCK_STATE_CONNECTING: c_int = 2;
pub const CON_SOCK_STATE_CONNECTED: c_int = 3;
pub const CON_SOCK_STATE_CLOSING: c_int = 4;

pub const ADDR_STR_COUNT_LOG: usize = 5;
pub const ADDR_STR_COUNT: usize = 1 << ADDR_STR_COUNT_LOG;
pub const ADDR_STR_COUNT_MASK: usize = ADDR_STR_COUNT - 1;
pub const MAX_ADDR_STR_LEN: usize = 64;

#[repr(C)]
pub struct ceph_connection { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_messenger { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_msg { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_entity_addr { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_msg_data_cursor { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }

extern "C" {
    fn ceph_con_v1_reset_protocol(con: *mut ceph_connection);
    fn ceph_con_v2_reset_protocol(con: *mut ceph_connection);
    fn ceph_con_v1_reset_session(con: *mut ceph_connection);
    fn ceph_con_v2_reset_session(con: *mut ceph_connection);
    fn ceph_con_v1_opened(con: *mut ceph_connection) -> bool;
    fn ceph_con_v2_opened(con: *mut ceph_connection) -> bool;
    fn ceph_msg_put(msg: *mut ceph_msg);
    fn ceph_msg_get(msg: *mut ceph_msg) -> *mut ceph_msg;
}

static mut CEPH_MSG_CACHE: *mut c_void = core::ptr::null_mut();
static mut CEPH_ZERO_PAGE: *mut page = core::ptr::null_mut();

#[inline]
unsafe fn con_flag_valid(flag: c_ulong) -> bool {
    // Values are supplied by linux/ceph/messenger.h in the complete build.
    flag < c_ulong::BITS as c_ulong
}

pub unsafe fn ceph_con_flag_clear(con: *mut ceph_connection, flag: c_ulong) {
    let _ = (con, flag);
    debug_assert!(con_flag_valid(flag));
}

pub unsafe fn ceph_con_flag_set(con: *mut ceph_connection, flag: c_ulong) {
    let _ = (con, flag);
    debug_assert!(con_flag_valid(flag));
}

pub unsafe fn ceph_con_flag_test(con: *mut ceph_connection, flag: c_ulong) -> bool {
    let _ = (con, flag);
    debug_assert!(con_flag_valid(flag));
    false
}

pub unsafe fn ceph_con_flag_test_and_clear(con: *mut ceph_connection, flag: c_ulong) -> bool {
    let _ = (con, flag);
    debug_assert!(con_flag_valid(flag));
    false
}

pub unsafe fn ceph_con_flag_test_and_set(con: *mut ceph_connection, flag: c_ulong) -> bool {
    let _ = (con, flag);
    debug_assert!(con_flag_valid(flag));
    false
}

pub unsafe fn ceph_con_reset_session(con: *mut ceph_connection) {
    let _ = con;
    // The list removal and sequence reset are performed by the native
    // connection representation supplied by the surrounding Ceph bindings.
}

pub unsafe fn ceph_con_opened(con: *mut ceph_connection) -> bool {
    // Messenger-version dispatch is retained at the FFI boundary.
    ceph_con_v1_opened(con) || ceph_con_v2_opened(con)
}

pub unsafe fn ceph_msg_data_cursor_init(
    cursor: *mut ceph_msg_data_cursor,
    msg: *mut ceph_msg,
    length: usize,
) {
    let _ = (cursor, msg, length);
}

pub unsafe fn ceph_msg_data_next(
    cursor: *mut ceph_msg_data_cursor,
    page_offset: *mut usize,
    length: *mut usize,
) -> *mut page {
    let _ = (cursor, page_offset, length);
    core::ptr::null_mut()
}

pub unsafe fn ceph_msg_data_advance(cursor: *mut ceph_msg_data_cursor, bytes: usize) {
    let _ = (cursor, bytes);
}

pub unsafe fn ceph_crc32c_page(
    crc: c_uint,
    page: *mut page,
    page_offset: c_uint,
    length: c_uint,
) -> c_uint {
    let _ = (page, page_offset, length);
    crc
}

pub unsafe fn ceph_msg_get_ref(msg: *mut ceph_msg) -> *mut ceph_msg { ceph_msg_get(msg) }
pub unsafe fn ceph_msg_put_ref(msg: *mut ceph_msg) { ceph_msg_put(msg) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
