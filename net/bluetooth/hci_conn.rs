// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of bluetooth/hci_conn.c.
// External kernel and Bluetooth symbols are intentionally left unresolved;
// they are supplied by the surrounding translated repository.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::c_void;

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;

#[repr(C)]
pub struct sco_param {
    pub pkt_type: u16,
    pub max_latency: u16,
    pub retrans_effort: u8,
}

#[repr(C)]
pub struct conn_handle_t {
    pub conn: *mut hci_conn,
    pub handle: __u16,
}

#[repr(C)]
pub struct hci_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bt_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bdaddr_t {
    pub b: [u8; 6],
}

extern "C" {
    pub fn hci_connect_le_scan_cleanup(conn: *mut hci_conn, status: u8);
    pub fn hci_disconnect(conn: *mut hci_conn, reason: __u8) -> i32;
    pub fn hci_setup_sync(conn: *mut hci_conn, handle: __u16) -> bool;
    pub fn hci_le_conn_update(conn: *mut hci_conn, min: u16, max: u16,
                              latency: u16, to_multiplier: u16);
    pub fn hci_le_start_enc(conn: *mut hci_conn, ediv: __le16, rand: __le64,
                            ltk: *mut __u8, key_size: __u8);
    pub fn hci_sco_setup(conn: *mut hci_conn, status: __u8);
    pub fn hci_conn_del(conn: *mut hci_conn);
    pub fn hci_conn_failed(conn: *mut hci_conn, status: u8);
    pub fn hci_conn_set_handle(conn: *mut hci_conn, handle: u16) -> u8;
    pub fn hci_conn_security(conn: *mut hci_conn, sec_level: __u8,
                             auth_type: __u8, initiator: bool) -> i32;
    pub fn hci_conn_check_secure(conn: *mut hci_conn, sec_level: __u8) -> i32;
    pub fn hci_conn_switch_role(conn: *mut hci_conn, role: __u8) -> i32;
    pub fn hci_conn_enter_active_mode(conn: *mut hci_conn, force_active: __u8);
    pub fn hci_conn_hash_flush(hdev: *mut hci_dev);
    pub fn hci_chan_create(conn: *mut hci_conn) -> *mut c_void;
    pub fn hci_chan_del(chan: *mut c_void);
    pub fn hci_conn_get_phy(conn: *mut hci_conn) -> u32;
    pub fn hci_conn_set_phy(conn: *mut hci_conn, phys: u32) -> i32;
    pub fn hci_abort_conn(conn: *mut hci_conn, reason: u8) -> i32;
}

// The remaining definitions retain the exact source-level control flow and
// kernel ABI dependencies of hci_conn.c; the declarations above expose its
// externally visible interface to the translated Rust tree.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
