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

/* C dependencies: <linux/types.h>, <linux/if_ether.h>, <linux/filter.h> */

/* Read queue size */
pub const TUN_READQ_SIZE: u32 = 500;
/* TUN device type flags: deprecated. Use IFF_TUN/IFF_TAP instead. */
pub const TUN_TUN_DEV: u32 = IFF_TUN;
pub const TUN_TAP_DEV: u32 = IFF_TAP;
pub const TUN_TYPE_MASK: u32 = 0x000f;

/* Ioctl defines */
pub const TUNSETNOCSUM: u32 = _IOW(b'T' as u32, 200, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETDEBUG: u32 = _IOW(b'T' as u32, 201, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETIFF: u32 = _IOW(b'T' as u32, 202, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETPERSIST: u32 = _IOW(b'T' as u32, 203, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETOWNER: u32 = _IOW(b'T' as u32, 204, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETLINK: u32 = _IOW(b'T' as u32, 205, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETGROUP: u32 = _IOW(b'T' as u32, 206, core::mem::size_of::<core::ffi::c_int>());
pub const TUNGETFEATURES: u32 = _IOR(b'T' as u32, 207, core::mem::size_of::<core::ffi::c_uint>());
pub const TUNSETOFFLOAD: u32 = _IOW(b'T' as u32, 208, core::mem::size_of::<core::ffi::c_uint>());
pub const TUNSETTXFILTER: u32 = _IOW(b'T' as u32, 209, core::mem::size_of::<core::ffi::c_uint>());
pub const TUNGETIFF: u32 = _IOR(b'T' as u32, 210, core::mem::size_of::<core::ffi::c_uint>());
pub const TUNGETSNDBUF: u32 = _IOR(b'T' as u32, 211, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETSNDBUF: u32 = _IOW(b'T' as u32, 212, core::mem::size_of::<core::ffi::c_int>());
pub const TUNATTACHFILTER: u32 = _IOW(b'T' as u32, 213, core::mem::size_of::<sock_fprog>());
pub const TUNDETACHFILTER: u32 = _IOW(b'T' as u32, 214, core::mem::size_of::<sock_fprog>());
pub const TUNGETVNETHDRSZ: u32 = _IOR(b'T' as u32, 215, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETVNETHDRSZ: u32 = _IOW(b'T' as u32, 216, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETQUEUE: u32 = _IOW(b'T' as u32, 217, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETIFINDEX: u32 = _IOW(b'T' as u32, 218, core::mem::size_of::<core::ffi::c_uint>());
pub const TUNGETFILTER: u32 = _IOR(b'T' as u32, 219, core::mem::size_of::<sock_fprog>());
pub const TUNSETVNETLE: u32 = _IOW(b'T' as u32, 220, core::mem::size_of::<core::ffi::c_int>());
pub const TUNGETVNETLE: u32 = _IOR(b'T' as u32, 221, core::mem::size_of::<core::ffi::c_int>());
/* The TUNSETVNETBE and TUNGETVNETBE ioctls are for cross-endian support on
 * little-endian hosts. Not all kernel configurations support them, but all
 * configurations that support SET also support GET.
 */
pub const TUNSETVNETBE: u32 = _IOW(b'T' as u32, 222, core::mem::size_of::<core::ffi::c_int>());
pub const TUNGETVNETBE: u32 = _IOR(b'T' as u32, 223, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETSTEERINGEBPF: u32 = _IOR(b'T' as u32, 224, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETFILTEREBPF: u32 = _IOR(b'T' as u32, 225, core::mem::size_of::<core::ffi::c_int>());
pub const TUNSETCARRIER: u32 = _IOW(b'T' as u32, 226, core::mem::size_of::<core::ffi::c_int>());
pub const TUNGETDEVNETNS: u32 = _IO(b'T' as u32, 227);

/* TUNSETIFF ifr flags */
pub const IFF_TUN: u32 = 0x0001;
pub const IFF_TAP: u32 = 0x0002;
pub const IFF_NAPI: u32 = 0x0010;
pub const IFF_NAPI_FRAGS: u32 = 0x0020;
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

/* Protocol info prepended to the packets (when IFF_NO_PI is not set) */
pub const TUN_PKT_STRIP: u32 = 0x0001;
#[repr(C)]
pub struct tun_pi {
    pub flags: __u16,
    pub proto: __be16,
}

/*
 * Filter spec (used for SETXXFILTER ioctls)
 * This stuff is applicable only to the TAP (Ethernet) devices.
 * If the count is zero the filter is disabled and the driver accepts
 * all packets (promisc mode).
 * If the filter is enabled in order to accept broadcast packets
 * broadcast addr must be explicitly included in the addr list.
 */
pub const TUN_FLT_ALLMULTI: u32 = 0x0001; /* Accept all multicast packets */
#[repr(C)]
pub struct tun_filter {
    pub flags: __u16, /* TUN_FLT_ flags see above */
    pub count: __u16, /* Number of addresses */
    pub addr: [[__u8; ETH_ALEN]; 0],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
