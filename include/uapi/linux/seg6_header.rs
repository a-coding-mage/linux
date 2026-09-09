/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  SR-IPv6 implementation
 *
 *  Author:
 *  David Lebrun <david.lebrun@uclouvain.be>
 *
 *
 *  This program is free software; you can redistribute it and/or
 *      modify it under the terms of the GNU General Public License
 *      as published by the Free Software Foundation; either version
 *      2 of the License, or (at your option) any later version.
 */

/* Dependency: struct in6_addr is supplied by the corresponding Linux types. */

/*
 * SRH
 */
#[repr(C)]
pub struct ipv6_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub type_: u8,
    pub segments_left: u8,
    /* Represents the last_entry field of SRH */
    pub first_segment: u8,
    pub flags: u8,
    pub tag: u16,
    pub segments: [in6_addr; 0],
}

pub const SR6_FLAG1_PROTECTED: u32 = 1 << 6;
pub const SR6_FLAG1_OAM: u32 = 1 << 5;
pub const SR6_FLAG1_ALERT: u32 = 1 << 4;
pub const SR6_FLAG1_HMAC: u32 = 1 << 3;

pub const SR6_TLV_INGRESS: u8 = 1;
pub const SR6_TLV_EGRESS: u8 = 2;
pub const SR6_TLV_OPAQUE: u8 = 3;
pub const SR6_TLV_PADDING: u8 = 4;
pub const SR6_TLV_HMAC: u8 = 5;

pub unsafe fn sr_has_hmac(srh: *const ipv6_sr_hdr) -> u8 {
    (*srh).flags & SR6_FLAG1_HMAC as u8
}

#[repr(C)]
pub struct sr6_tlv {
    pub type_: u8,
    pub len: u8,
    pub data: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
