/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2004 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005 Voltaire, Inc. All rights reserved.
 *
 * This software is available under a choice of one of two licenses.  See the
 * original header for the complete license text.
 */

// Dependency intent: __u8/__u16/__u32/__be16/__be32/__aligned_u64 are supplied
// by the Linux/RDMA userspace type definitions.

pub const IB_USER_MAD_ABI_VERSION: u32 = 5;

/// Old version of MAD packet header without pkey_index.
#[repr(C)]
pub struct ib_user_mad_hdr_old {
    pub id: __u32,
    pub status: __u32,
    pub timeout_ms: __u32,
    pub retries: __u32,
    pub length: __u32,
    pub qpn: __be32,
    pub qkey: __be32,
    pub lid: __be16,
    pub sl: __u8,
    pub path_bits: __u8,
    pub grh_present: __u8,
    pub gid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub gid: [__u8; 16],
    pub flow_label: __be32,
}

/// MAD packet header.
#[repr(C)]
pub struct ib_user_mad_hdr {
    pub id: __u32,
    pub status: __u32,
    pub timeout_ms: __u32,
    pub retries: __u32,
    pub length: __u32,
    pub qpn: __be32,
    pub qkey: __be32,
    pub lid: __be16,
    pub sl: __u8,
    pub path_bits: __u8,
    pub grh_present: __u8,
    pub gid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub gid: [__u8; 16],
    pub flow_label: __be32,
    pub pkey_index: __u16,
    pub reserved: [__u8; 6],
}

/// MAD packet.
#[repr(C)]
pub struct ib_user_mad {
    pub hdr: ib_user_mad_hdr,
    pub data: [__aligned_u64; 0],
}

// The C declaration uses unsigned long with an explicit 4-byte alignment.
// The alias preserves the unsigned-long integer intent; ABI alignment is a
// property of the containing C-compatible layout.
pub type packed_ulong = core::ffi::c_ulong;

pub const IB_USER_MAD_LONGS_PER_METHOD_MASK: usize =
    128 / (8 * core::mem::size_of::<core::ffi::c_ulong>());

/// MAD registration request.
#[repr(C)]
pub struct ib_user_mad_reg_req {
    pub id: __u32,
    pub method_mask: [packed_ulong; IB_USER_MAD_LONGS_PER_METHOD_MASK],
    pub qpn: __u8,
    pub mgmt_class: __u8,
    pub mgmt_class_version: __u8,
    pub oui: [__u8; 3],
    pub rmpp_version: __u8,
}

/// MAD registration request, version 2.
#[repr(C)]
pub struct ib_user_mad_reg_req2 {
    pub id: __u32,
    pub qpn: __u32,
    pub mgmt_class: __u8,
    pub mgmt_class_version: __u8,
    pub res: __u16,
    pub flags: __u32,
    pub method_mask: [__aligned_u64; 2],
    pub oui: __u32,
    pub rmpp_version: __u8,
    pub reserved: [__u8; 3],
}

pub const IB_USER_MAD_USER_RMPP: u32 = 1 << 0;
pub const IB_USER_MAD_REG_FLAGS_CAP: u32 = IB_USER_MAD_USER_RMPP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
