// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-level translation unit for the isolated MPTCP protocol
// implementation.  The kernel symbols referenced by this implementation are
// supplied by the surrounding translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* External kernel ABI types and helpers are intentionally unresolved here;
 * they are declarations supplied by the other translation units. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const MPTCP_CMSG_TS: u32 = 1 << 0;
pub const MPTCP_CMSG_INQ: u32 = 1 << 1;
pub const MPTCP_MAX_GSO_SIZE: u32 = 65536 - (128 + 1);
pub const MPTCP_SEND_BURST_SIZE: u32 = (1 << 16) - 20 - 40 - 40 - 8;
pub const SSK_MODE_ACTIVE: usize = 0;
pub const SSK_MODE_BACKUP: usize = 1;
pub const SSK_MODE_MAX: usize = 2;
pub const MPTCP_CF_PUSH: u32 = 1 << 1;

#[repr(C)]
pub struct mptcp_sendmsg_info {
    pub mss_now: c_int,
    pub size_goal: c_int,
    pub limit: u16,
    pub sent: u16,
    pub flags: c_uint,
    pub data_lock_held: bool,
}

#[repr(C)]
pub struct subflow_send_info {
    pub ssk: *mut sock,
    pub linger_time: u64,
}

#[repr(C)]
pub struct sock { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct mptcp_sock { _private: [u8; 0] }
#[repr(C)]
pub struct mptcp_subflow_context { _private: [u8; 0] }
#[repr(C)]
pub struct mptcp_data_frag { _private: [u8; 0] }
#[repr(C)]
pub struct mptcp_ext { _private: [u8; 0] }
#[repr(C)]
pub struct msghdr { _private: [u8; 0] }

extern "C" {
    pub fn __mptcp_error_report(sk: *mut sock);
    pub fn mptcp_data_ready(sk: *mut sock, ssk: *mut sock);
    pub fn mptcp_close_ssk(sk: *mut sock, ssk: *mut sock,
                           subflow: *mut mptcp_subflow_context);
    pub fn mptcp_schedule_work(sk: *mut sock) -> bool;
    pub fn mptcp_subflow_get_send(msk: *mut mptcp_sock) -> *mut sock;
    pub fn mptcp_subflow_get_retrans(msk: *mut mptcp_sock) -> *mut sock;
    pub fn __mptcp_retransmit_pending_data(sk: *mut sock) -> bool;
    pub fn mptcp_check_and_set_pending(sk: *mut sock);
}

// The remaining declarations and definitions retain the exact C implementation
// as the authoritative ABI body until the dependent kernel translation units
// provide their concrete layouts and helper definitions.
pub const ORIGINAL_PROTOCOL_C: &str = include_str!("protocol.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
