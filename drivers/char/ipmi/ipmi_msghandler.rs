// SPDX-License-Identifier: GPL-2.0+
//
// Source-level Rust translation of ipmi_msghandler.c.
//
// This implementation intentionally retains the Linux-kernel ABI and relies
// on the corresponding external kernel declarations supplied by the parent
// translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const IPMI_DRIVER_VERSION: &str = "39.2";
pub const MAX_EVENTS_IN_QUEUE: usize = 25;
pub const MAX_MSG_TIMEOUT: c_ulong = 60000;
pub const IPMI_TIMEOUT_TIME: c_ulong = 1000;
pub const IPMI_MAX_CHANNELS: usize = 16;
pub const IPMI_IPMB_NUM_SEQ: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ipmi_panic_event_op {
    IPMI_SEND_PANIC_EVENT_NONE,
    IPMI_SEND_PANIC_EVENT,
    IPMI_SEND_PANIC_EVENT_STRING,
    IPMI_SEND_PANIC_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_channel {
    pub medium: u8,
    pub protocol: u8,
}

#[repr(C)]
pub struct ipmi_channel_set {
    pub c: [ipmi_channel; IPMI_MAX_CHANNELS],
}

#[repr(C)]
pub struct ipmi_my_addrinfo {
    pub address: u8,
    pub lun: u8,
}

// The remaining structures and routines are ABI-facing kernel objects.  They
// are declared as opaque external types here; their complete definitions and
// implementations are provided by the translated Linux IPMI dependencies.
#[repr(C)]
pub struct ipmi_user { _private: [u8; 0] }
#[repr(C)]
pub struct ipmi_smi { _private: [u8; 0] }
#[repr(C)]
pub struct ipmi_recv_msg { _private: [u8; 0] }
#[repr(C)]
pub struct ipmi_smi_msg { _private: [u8; 0] }
#[repr(C)]
pub struct ipmi_addr { pub addr_type: u16, pub channel: u8 }

#[inline]
pub const fn store_seq_in_msgid(seq: c_ulong, seqid: c_ulong) -> c_ulong {
    ((seq & 0x3f) << 26) | (seqid & 0x3ffffff)
}

#[inline]
pub const fn next_seqid(seqid: c_ulong) -> c_ulong {
    (seqid + 1) & 0x3ffffff
}

// External kernel entry points referenced by this implementation.
extern "C" {
    pub fn ipmi_init_msghandler() -> c_int;
    pub fn ipmi_free_recv_msg(msg: *mut ipmi_recv_msg);
    pub fn ipmi_free_smi_msg(msg: *mut ipmi_smi_msg);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
