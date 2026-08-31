/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016 Facebook
 */

/* Translated from a C header that included <linux/types.h>. */

pub const MAX_IPTNL_ENTRIES: u32 = 256U;

#[repr(C)]
pub union vip_daddr {
    pub v6: [__u32; 4],
    pub v4: __u32,
}

#[repr(C)]
pub struct vip {
    pub daddr: vip_daddr,
    pub dport: __u16,
    pub family: __u16,
    pub protocol: __u8,
}

#[repr(C)]
pub union iptnl_info_saddr {
    pub v6: [__u32; 4],
    pub v4: __u32,
}

#[repr(C)]
pub union iptnl_info_daddr {
    pub v6: [__u32; 4],
    pub v4: __u32,
}

#[repr(C)]
pub struct iptnl_info {
    pub saddr: iptnl_info_saddr,
    pub daddr: iptnl_info_daddr,
    pub family: __u16,
    pub dmac: [__u8; 6],
}
