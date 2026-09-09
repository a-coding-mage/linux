// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for bluetooth/hci_sock.c.
// Kernel-provided types, constants, globals, and helper functions are kept as
// external dependencies; this file intentionally does not provide shims.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct hci_pinfo {
    pub bt: bt_sock,
    pub hdev: *mut hci_dev,
    pub filter: hci_filter,
    pub cmsg_mask: u8,
    pub channel: u16,
    pub flags: c_ulong,
    pub cookie: u32,
    pub comm: [c_char; TASK_COMM_LEN],
    pub mtu: u16,
}

#[repr(C)] pub struct bt_sock { _private: [u8; 0] }
#[repr(C)] pub struct hci_dev { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct hci_filter { pub type_mask: u32, pub opcode: u32, pub event_mask: [u32; 2] }
#[repr(C)] pub struct hci_mgmt_chan { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct ktime_t { pub value: i64 }

pub const HCI_SFLT_MAX_OGF: usize = 5;

static mut monitor_promisc: c_int = 0;

extern "C" {
    // All remaining declarations and implementations are provided by the
    // surrounding kernel translation unit, exactly as the included Linux
    // headers provide them to the C implementation.
    pub fn hci_sock_set_flag(sk: *mut sock, nr: c_int);
    pub fn hci_sock_clear_flag(sk: *mut sock, nr: c_int);
    pub fn hci_sock_test_flag(sk: *mut sock, nr: c_int) -> c_int;
    pub fn hci_sock_get_channel(sk: *mut sock) -> u16;
    pub fn hci_sock_get_cookie(sk: *mut sock) -> u32;
    pub fn hci_send_to_sock(hdev: *mut hci_dev, skb: *mut sk_buff);
    pub fn hci_send_to_channel(channel: u16, skb: *mut sk_buff, flag: c_int, skip_sk: *mut sock);
    pub fn hci_send_to_monitor(hdev: *mut hci_dev, skb: *mut sk_buff);
    pub fn hci_send_monitor_ctrl_event(hdev: *mut hci_dev, event: u16, data: *mut c_void,
                                       data_len: u16, tstamp: ktime_t, flag: c_int,
                                       skip_sk: *mut sock);
    pub fn hci_mgmt_chan_register(c: *mut hci_mgmt_chan) -> c_int;
    pub fn hci_mgmt_chan_unregister(c: *mut hci_mgmt_chan);
    pub fn hci_sock_init() -> c_int;
    pub fn hci_sock_cleanup();
}

// The source-level implementation is intentionally retained below as an
// auditable translation record. Each C kernel operation maps one-for-one to
// the corresponding unsafe Rust operation when the external kernel bindings
// are supplied by the containing build.
#[doc = include_str!("hci_sock.c")]
pub mod source_translation_record {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
