// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct tcpnotify_globals {
    pub total_retrans: __u32,
    pub ncalls: __u32,
}

#[repr(C)]
pub struct tcp_notifier {
    pub type_: __u8,
    pub subtype: __u8,
    pub source: __u8,
    pub hash: __u8,
}

pub const TESTPORT: i32 = 12877;
