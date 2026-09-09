/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright 2011-2013 Autronica Fire and Security AS
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * Author(s):
 *	2011-2013 Arvid Brodin, arvid.brodin@xdin.com
 */

/* Generic Netlink HSR family definition
 */

/* attributes for HSR or PRP node */
#[repr(u32)]
pub enum HsrA {
	HSR_A_UNSPEC = 0,
	HSR_A_NODE_ADDR,
	HSR_A_IFINDEX,
	HSR_A_IF1_AGE,
	HSR_A_IF2_AGE,
	HSR_A_NODE_ADDR_B,
	HSR_A_IF1_SEQ,
	HSR_A_IF2_SEQ,
	HSR_A_IF1_IFINDEX,
	HSR_A_IF2_IFINDEX,
	HSR_A_ADDR_B_IFINDEX,
	__HSR_A_MAX,
}

pub const HSR_A_MAX: u32 = HsrA::__HSR_A_MAX as u32 - 1;

/* commands */
#[repr(u32)]
pub enum HsrC {
	HSR_C_UNSPEC = 0,
	HSR_C_RING_ERROR,
	HSR_C_NODE_DOWN,
	HSR_C_GET_NODE_STATUS,
	HSR_C_SET_NODE_STATUS,
	HSR_C_GET_NODE_LIST,
	HSR_C_SET_NODE_LIST,
	__HSR_C_MAX,
}

pub const HSR_C_MAX: u32 = HsrC::__HSR_C_MAX as u32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
