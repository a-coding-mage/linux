/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IEEE 802.15.4 interface for userspace
 *
 * Copyright 2007, 2008 Siemens AG
 *
 * Written by:
 * Sergey Lapin <slapin@ossfans.org>
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 */

// Dependency supplied by the Linux socket definitions: sa_family_t.

pub const IEEE802154_ADDR_NONE: i32 = 0x0;
/* RESERVED = 0x01, */
pub const IEEE802154_ADDR_SHORT: i32 = 0x2; /* 16-bit address + PANid */
pub const IEEE802154_ADDR_LONG: i32 = 0x3; /* 64-bit address + PANid */

/* address length, octets */
pub const IEEE802154_ADDR_LEN: usize = 8;

#[repr(C)]
pub union ieee802154_addr_sa__bindgen_ty_1 {
    pub hwaddr: [u8; IEEE802154_ADDR_LEN],
    pub short_addr: u16,
}

#[repr(C)]
pub struct ieee802154_addr_sa {
    pub addr_type: i32,
    pub pan_id: u16,
    pub __bindgen_anon_1: ieee802154_addr_sa__bindgen_ty_1,
}

pub const IEEE802154_PANID_BROADCAST: u16 = 0xffff;
pub const IEEE802154_ADDR_BROADCAST: u16 = 0xffff;
pub const IEEE802154_ADDR_UNDEF: u16 = 0xfffe;

#[repr(C)]
pub struct sockaddr_ieee802154 {
    pub family: sa_family_t, /* AF_IEEE802154 */
    pub addr: ieee802154_addr_sa,
}

/* get/setsockopt */
pub const SOL_IEEE802154: i32 = 0;

pub const WPAN_WANTACK: i32 = 0;
pub const WPAN_SECURITY: i32 = 1;
pub const WPAN_SECURITY_LEVEL: i32 = 2;
pub const WPAN_WANTLQI: i32 = 3;

pub const WPAN_SECURITY_DEFAULT: i32 = 0;
pub const WPAN_SECURITY_OFF: i32 = 1;
pub const WPAN_SECURITY_ON: i32 = 2;

pub const WPAN_SECURITY_LEVEL_DEFAULT: i32 = -1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
