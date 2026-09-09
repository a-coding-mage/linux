/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux type and seg6 headers.

pub const SEG6_HMAC_SECRET_LEN: usize = 64;
pub const SEG6_HMAC_FIELD_LEN: usize = 32;

#[repr(C)]
pub struct sr6_tlv_hmac {
    pub tlvhdr: sr6_tlv,
    pub reserved: __u16,
    pub hmackeyid: __be32,
    pub hmac: [__u8; SEG6_HMAC_FIELD_LEN],
}

pub const SEG6_HMAC_ALGO_SHA1: i32 = 1;
pub const SEG6_HMAC_ALGO_SHA256: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
