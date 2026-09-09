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

// Dependency: `ipv6_sr_hdr` is supplied by linux/seg6.h.

#[repr(i32)]
pub enum Seg6IptunnelAttribute {
    SEG6_IPTUNNEL_UNSPEC,
    SEG6_IPTUNNEL_SRH,
    SEG6_IPTUNNEL_SRC,
    SEG6_IPTUNNEL_TABLE,
    __SEG6_IPTUNNEL_MAX,
}

pub const SEG6_IPTUNNEL_MAX: i32 =
    Seg6IptunnelAttribute::__SEG6_IPTUNNEL_MAX as i32 - 1;

#[repr(C)]
pub struct seg6_iptunnel_encap {
    pub mode: ::core::ffi::c_int,
    pub srh: [ipv6_sr_hdr; 0],
}

#[inline]
pub unsafe fn SEG6_IPTUN_ENCAP_SIZE(x: *const seg6_iptunnel_encap) -> usize {
    ::core::mem::size_of::<seg6_iptunnel_encap>()
        + ((((*x).srh.as_ptr() as *const ipv6_sr_hdr).as_ref().unwrap().hdrlen as usize + 1) << 3)
}

#[repr(i32)]
pub enum Seg6IptunMode {
    SEG6_IPTUN_MODE_INLINE,
    SEG6_IPTUN_MODE_ENCAP,
    SEG6_IPTUN_MODE_L2ENCAP,
    SEG6_IPTUN_MODE_ENCAP_RED,
    SEG6_IPTUN_MODE_L2ENCAP_RED,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
