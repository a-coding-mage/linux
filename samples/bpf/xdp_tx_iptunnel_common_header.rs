/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016 Facebook
 */

// Translated from the C header. The original dependency was <linux/types.h>.

pub const MAX_IPTNL_ENTRIES: u32 = 256u32;

#[repr(C)]
pub union VipDaddr {
    pub v6: [u32; 4],
    pub v4: u32,
}

#[repr(C)]
pub struct vip {
    pub daddr: VipDaddr,
    pub dport: u16,
    pub family: u16,
    pub protocol: u8,
}

#[repr(C)]
pub union IptnlInfoSaddr {
    pub v6: [u32; 4],
    pub v4: u32,
}

#[repr(C)]
pub union IptnlInfoDaddr {
    pub v6: [u32; 4],
    pub v4: u32,
}

#[repr(C)]
pub struct iptnl_info {
    pub saddr: IptnlInfoSaddr,
    pub daddr: IptnlInfoDaddr,
    pub family: u16,
    pub dmac: [u8; 6],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
