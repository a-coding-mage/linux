/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * if_addrlabel.h - netlink interface for address labels
 *
 * Copyright (C)2007 USAGI/WIDE Project,  All Rights Reserved.
 *
 * Authors:
 *	YOSHIFUJI Hideaki @ USAGI/WIDE <yoshfuji@linux-ipv6.org>
 */

// Dependency corresponding to <linux/types.h> is supplied externally.

#[repr(C)]
pub struct ifaddrlblmsg {
	pub ifal_family: u8,      /* Address family */
	pub __ifal_reserved: u8,  /* Reserved */
	pub ifal_prefixlen: u8,   /* Prefix length */
	pub ifal_flags: u8,       /* Flags */
	pub ifal_index: u32,      /* Link index */
	pub ifal_seq: u32,        /* sequence number */
}

pub const IFAL_ADDRESS: i32 = 1;
pub const IFAL_LABEL: i32 = 2;
pub const __IFAL_MAX: i32 = 3;

pub const IFAL_MAX: i32 = __IFAL_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
