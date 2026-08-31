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

/* Depends on Linux UAPI definitions from <linux/types.h> and <linux/in6.h>. */

/*
 * SRH
 */
#[repr(C)]
pub struct ipv6_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub type_: u8,
    pub segments_left: u8,
    pub first_segment: u8, /* Represents the last_entry field of SRH */
    pub flags: u8,
    pub tag: u16,

    pub segments: [in6_addr; 0],
}

pub const SR6_FLAG1_PROTECTED: i32 = 1 << 6;
pub const SR6_FLAG1_OAM: i32 = 1 << 5;
pub const SR6_FLAG1_ALERT: i32 = 1 << 4;
pub const SR6_FLAG1_HMAC: i32 = 1 << 3;

pub const SR6_TLV_INGRESS: i32 = 1;
pub const SR6_TLV_EGRESS: i32 = 2;
pub const SR6_TLV_OPAQUE: i32 = 3;
pub const SR6_TLV_PADDING: i32 = 4;
pub const SR6_TLV_HMAC: i32 = 5;

#[inline]
pub unsafe fn sr_has_hmac(srh: *const ipv6_sr_hdr) -> i32 {
    unsafe { ((*srh).flags as i32) & SR6_FLAG1_HMAC }
}

#[repr(C)]
pub struct sr6_tlv {
    pub type_: u8,
    pub len: u8,
    pub data: [u8; 0],
}
