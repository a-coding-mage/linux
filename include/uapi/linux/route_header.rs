/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for the IP router interface.
 *
 * Version:	@(#)route.h	1.0.3	05/27/93
 *
 * Authors:	Original taken from Berkeley UNIX 4.3, (c) UCB 1986-1988
 *		for the purposes of compatibility only.
 *
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 * Changes:
 *              Mike McLagan    :       Routing by source
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		of the License, or (at your option) any later version.
 */

// Dependencies supplied by the surrounding Linux UAPI translation.

/* This structure gets passed by the SIOCADDRT and SIOCDELRT calls. */
#[repr(C)]
pub struct rtentry {
    pub rt_pad1: ::core::ffi::c_ulong,
    pub rt_dst: sockaddr,
    pub rt_gateway: sockaddr,
    pub rt_genmask: sockaddr,
    pub rt_flags: ::core::ffi::c_ushort,
    pub rt_pad2: ::core::ffi::c_short,
    pub rt_pad3: ::core::ffi::c_ulong,
    pub rt_pad4: *mut ::core::ffi::c_void,
    pub rt_metric: ::core::ffi::c_short, /* +1 for binary compatibility! */
    pub rt_dev: *mut ::core::ffi::c_char, /* forcing the device at add */
    pub rt_mtu: ::core::ffi::c_ulong, /* per route MTU/Window */
    // In non-kernel builds, C provides: #define rt_mss rt_mtu
    pub rt_window: ::core::ffi::c_ulong, /* Window clamping */
    pub rt_irtt: ::core::ffi::c_ushort, /* Initial RTT */
}

pub const RTF_UP: ::core::ffi::c_uint = 0x0001; /* route usable */
pub const RTF_GATEWAY: ::core::ffi::c_uint = 0x0002; /* destination is a gateway */
pub const RTF_HOST: ::core::ffi::c_uint = 0x0004; /* host entry (net otherwise) */
pub const RTF_REINSTATE: ::core::ffi::c_uint = 0x0008; /* reinstate route after tmout */
pub const RTF_DYNAMIC: ::core::ffi::c_uint = 0x0010; /* created dyn. (by redirect) */
pub const RTF_MODIFIED: ::core::ffi::c_uint = 0x0020; /* modified dyn. (by redirect) */
pub const RTF_MTU: ::core::ffi::c_uint = 0x0040; /* specific MTU for this route */
pub const RTF_MSS: ::core::ffi::c_uint = RTF_MTU; /* Compatibility :-( */
pub const RTF_WINDOW: ::core::ffi::c_uint = 0x0080; /* per route window clamping */
pub const RTF_IRTT: ::core::ffi::c_uint = 0x0100; /* Initial round trip time */
pub const RTF_REJECT: ::core::ffi::c_uint = 0x0200; /* Reject route */

/*
 * <linux/ipv6_route.h> uses RTF values >= 64k
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
