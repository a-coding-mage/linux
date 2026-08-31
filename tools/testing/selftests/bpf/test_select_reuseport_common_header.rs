/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018 Facebook */

// Translated from C header: <linux/types.h> supplies __u8, __u16, and __u32.

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum result {
    DROP_ERR_INNER_MAP,
    DROP_ERR_SKB_DATA,
    DROP_ERR_SK_SELECT_REUSEPORT,
    DROP_MISC,
    PASS,
    PASS_ERR_SK_SELECT_REUSEPORT,
    NR_RESULTS,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmd {
    pub reuseport_index: __u32,
    pub pass_on_failure: __u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct data_check {
    pub ip_protocol: __u32,
    pub skb_addrs: [__u32; 8],
    pub skb_ports: [__u16; 2],
    pub eth_protocol: __u16,
    pub bind_inany: __u8,
    pub equal_check_end: [__u8; 0],

    pub len: __u32,
    pub hash: __u32,
}
