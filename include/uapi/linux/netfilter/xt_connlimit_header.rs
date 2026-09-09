/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// use linux_types::{__be32, __u32};
// use linux_netfilter::nf_inet_addr;

#[repr(C)]
pub struct xt_connlimit_data {
    _private: [u8; 0],
}

pub const XT_CONNLIMIT_INVERT: u32 = 1 << 0;
pub const XT_CONNLIMIT_DADDR: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xt_connlimit_userspace_mask {
    pub v4_mask: u32, // __be32
    pub v6_mask: [u32; 4], // __be32[4]
}

#[repr(C)]
pub union xt_connlimit_mask {
    pub mask: nf_inet_addr,
    // Present only outside the kernel in the C header.
    pub userspace: xt_connlimit_userspace_mask,
}

#[repr(C, align(8))]
pub struct xt_connlimit_info {
    pub mask: xt_connlimit_mask,
    pub limit: u32,
    // revision 1
    pub flags: u32,
    // Used internally by the kernel
    pub data: *mut nf_conncount_data,
}

// Dependency supplied by the corresponding Linux headers.
pub struct nf_conncount_data;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
