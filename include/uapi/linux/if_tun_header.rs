/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Universal TUN/TAP device driver.
 *  Copyright (C) 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 */

// Dependencies supplied by the corresponding Linux UAPI headers are expected
// to provide IFF_TUN, IFF_TAP, ETH_ALEN, _IO, _IOW, and _IOR.

/* Read queue size */
pub const TUN_READQ_SIZE: u32 = 500;
/* TUN device type flags: deprecated. Use IFF_TUN/IFF_TAP instead. */
pub const TUN_TUN_DEV: u32 = IFF_TUN;
pub const TUN_TAP_DEV: u32 = IFF_TAP;
pub const TUN_TYPE_MASK: u32 = 0x000f;

/* Ioctl defines */
pub const TUNSETNOCSUM: _ = _IOW('T' as u8, 200, core::ffi::c_int);
pub const TUNSETDEBUG: _ = _IOW('T' as u8, 201, core::ffi::c_int);
pub const TUNSETIFF: _ = _IOW('T' as u8, 202, core::ffi::c_int);
pub const TUNSETPERSIST: _ = _IOW('T' as u8, 203, core::ffi::c_int);
pub const TUNSETOWNER: _ = _IOW('T' as u8, 204, core::ffi::c_int);
pub const TUNSETLINK: _ = _IOW('T' as u8, 205, core::ffi::c_int);
pub const TUNSETGROUP: _ = _IOW('T' as u8, 206, core::ffi::c_int);
pub const TUNGETFEATURES: _ = _IOR('T' as u8, 207, u32);
pub const TUNSETOFFLOAD: _ = _IOW('T' as u8, 208, u32);
pub const TUNSETTXFILTER: _ = _IOW('T' as u8, 209, u32);
pub const TUNGETIFF: _ = _IOR('T' as u8, 210, u32);
pub const TUNGETSNDBUF: _ = _IOR('T' as u8, 211, core::ffi::c_int);
pub const TUNSETSNDBUF: _ = _IOW('T' as u8, 212, core::ffi::c_int);
pub const TUNATTACHFILTER: _ = _IOW('T' as u8, 213, sock_fprog);
pub const TUNDETACHFILTER: _ = _IOW('T' as u8, 214, sock_fprog);
pub const TUNGETVNETHDRSZ: _ = _IOR('T' as u8, 215, core::ffi::c_int);
pub const TUNSETVNETHDRSZ: _ = _IOW('T' as u8, 216, core::ffi::c_int);
pub const TUNSETQUEUE: _ = _IOW('T' as u8, 217, core::ffi::c_int);
pub const TUNSETIFINDEX: _ = _IOW('T' as u8, 218, u32);
pub const TUNGETFILTER: _ = _IOR('T' as u8, 219, sock_fprog);
pub const TUNSETVNETLE: _ = _IOW('T' as u8, 220, core::ffi::c_int);
pub const TUNGETVNETLE: _ = _IOR('T' as u8, 221, core::ffi::c_int);
pub const TUNSETVNETBE: _ = _IOW('T' as u8, 222, core::ffi::c_int);
pub const TUNGETVNETBE: _ = _IOR('T' as u8, 223, core::ffi::c_int);
pub const TUNSETSTEERINGEBPF: _ = _IOR('T' as u8, 224, core::ffi::c_int);
pub const TUNSETFILTEREBPF: _ = _IOR('T' as u8, 225, core::ffi::c_int);
pub const TUNSETCARRIER: _ = _IOW('T' as u8, 226, core::ffi::c_int);
pub const TUNGETDEVNETNS: _ = _IO('T' as u8, 227);

/* TUNSETIFF ifr flags */
pub const IFF_TUN: u32 = 0x0001;
pub const IFF_TAP: u32 = 0x0002;
pub const IFF_NAPI: u32 = 0x0010;
pub const IFF_NAPI_FRAGS: u32 = 0x0020;
/* Used in TUNSETIFF to bring up tun/tap without carrier */
pub const IFF_NO_CARRIER: u32 = 0x0040;
/* Stop the queue instead of dropping when the internal ring is full, so an
 * attached qdisc applies backpressure instead of being bypassed.
 */
pub const IFF_BACKPRESSURE: u32 = 0x0080;
pub const IFF_NO_PI: u32 = 0x1000;
/* This flag has no real effect */
pub const IFF_ONE_QUEUE: u32 = 0x2000;
pub const IFF_VNET_HDR: u32 = 0x4000;
pub const IFF_TUN_EXCL: u32 = 0x8000;
pub const IFF_MULTI_QUEUE: u32 = 0x0100;
pub const IFF_ATTACH_QUEUE: u32 = 0x0200;
pub const IFF_DETACH_QUEUE: u32 = 0x0400;
/* read-only flag */
pub const IFF_PERSIST: u32 = 0x0800;
pub const IFF_NOFILTER: u32 = 0x1000;

/* Socket options */
pub const TUN_TX_TIMESTAMP: u32 = 1;

/* Features for GSO (TUNSETOFFLOAD). */
pub const TUN_F_CSUM: u32 = 0x01; /* You can hand me unchecksummed packets. */
pub const TUN_F_TSO4: u32 = 0x02; /* I can handle TSO for IPv4 packets */
pub const TUN_F_TSO6: u32 = 0x04; /* I can handle TSO for IPv6 packets */
pub const TUN_F_TSO_ECN: u32 = 0x08; /* I can handle TSO with ECN bits. */
pub const TUN_F_UFO: u32 = 0x10; /* I can handle UFO packets */
pub const TUN_F_USO4: u32 = 0x20; /* I can handle USO for IPv4 packets */
pub const TUN_F_USO6: u32 = 0x40; /* I can handle USO for IPv6 packets */
pub const TUN_F_UDP_TUNNEL_GSO: u32 = 0x080;
pub const TUN_F_UDP_TUNNEL_GSO_CSUM: u32 = 0x100;

/* Protocol info prepended to the packets (when IFF_NO_PI is not set) */
pub const TUN_PKT_STRIP: u16 = 0x0001;
#[repr(C)]
pub struct tun_pi {
    pub flags: u16,
    pub proto: u16,
}

/*
 * Filter spec (used for SETXXFILTER ioctls)
 * This stuff is applicable only to the TAP (Ethernet) devices.
 * If the count is zero the filter is disabled and the driver accepts
 * all packets (promisc mode).
 * If the filter is enabled in order to accept broadcast packets
 * broadcast addr must be explicitly included in the addr list.
 */
pub const TUN_FLT_ALLMULTI: u16 = 0x0001; /* Accept all multicast packets */
#[repr(C)]
pub struct tun_filter {
    pub flags: u16, /* TUN_FLT_ flags see above */
    pub count: u16, /* Number of addresses */
    pub addr: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
