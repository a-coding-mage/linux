/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *	Linux INET6 implementation
 *
 *	Authors:
 *	Pedro Roque		<roque@di.fc.ul.pt>
 *
 *	This program is free software; you can redistribute it and/or
 *      modify it under the terms of the GNU General Public License
 *      as published by the Free Software Foundation; either version
 *      2 of the License, or (at your option) any later version.
 */

// Dependency equivalent of <linux/types.h>.
// Dependency equivalent of <linux/in6.h> (for struct in6_addr).

pub const RTF_DEFAULT: u32 = 0x0001_0000; // default - learned via ND
pub const RTF_ALLONLINK: u32 = 0x0002_0000; // deprecated and will be removed; fallback, no routers on link
pub const RTF_ADDRCONF: u32 = 0x0004_0000; // addrconf route - RA
pub const RTF_PREFIX_RT: u32 = 0x0008_0000; // A prefix only route - RA
pub const RTF_ANYCAST: u32 = 0x0010_0000; // Anycast

pub const RTF_NONEXTHOP: u32 = 0x0020_0000; // route with no nexthop
pub const RTF_EXPIRES: u32 = 0x0040_0000;

pub const RTF_ROUTEINFO: u32 = 0x0080_0000; // route information - RA

pub const RTF_CACHE: u32 = 0x0100_0000; // read-only: can not be set by user
pub const RTF_FLOW: u32 = 0x0200_0000; // flow significant route
pub const RTF_POLICY: u32 = 0x0400_0000; // policy route

#[macro_export]
macro_rules! RTF_PREF {
    ($pref:expr) => {
        ($pref) << 27
    };
}

pub const RTF_PREF_MASK: u32 = 0x1800_0000;

pub const RTF_PCPU: u32 = 0x4000_0000; // read-only: can not be set by user
pub const RTF_LOCAL: u32 = 0x8000_0000;

#[repr(C)]
pub struct in6_rtmsg {
    pub rtmsg_dst: in6_addr,
    pub rtmsg_src: in6_addr,
    pub rtmsg_gateway: in6_addr,
    pub rtmsg_type: __u32,
    pub rtmsg_dst_len: __u16,
    pub rtmsg_src_len: __u16,
    pub rtmsg_metric: __u32,
    pub rtmsg_info: ::core::ffi::c_ulong,
    pub rtmsg_flags: __u32,
    pub rtmsg_ifindex: ::core::ffi::c_int,
}

pub const RTMSG_NEWDEVICE: u32 = 0x11;
pub const RTMSG_DELDEVICE: u32 = 0x12;
pub const RTMSG_NEWROUTE: u32 = 0x21;
pub const RTMSG_DELROUTE: u32 = 0x22;

pub const IP6_RT_PRIO_USER: u32 = 1024;
pub const IP6_RT_PRIO_ADDRCONF: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
