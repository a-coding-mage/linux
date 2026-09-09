/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * 	connector.h
 * 
 * 2004-2005 Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
 * All rights reserved.
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// C dependency: #include <linux/types.h>

/*
 * Process Events connector unique ids -- used for message routing
 */
pub const CN_IDX_PROC: u32 = 0x1;
pub const CN_VAL_PROC: u32 = 0x1;
pub const CN_IDX_CIFS: u32 = 0x2;
pub const CN_VAL_CIFS: u32 = 0x1;
pub const CN_W1_IDX: u32 = 0x3; // w1 communication
pub const CN_W1_VAL: u32 = 0x1;
pub const CN_IDX_V86D: u32 = 0x4;
pub const CN_VAL_V86D_UVESAFB: u32 = 0x1;
pub const CN_IDX_BB: u32 = 0x5; // BlackBoard, from the TSP GPL sampling framework
pub const CN_DST_IDX: u32 = 0x6;
pub const CN_DST_VAL: u32 = 0x1;
pub const CN_IDX_DM: u32 = 0x7; // Device Mapper
pub const CN_VAL_DM_USERSPACE_LOG: u32 = 0x1;
pub const CN_IDX_DRBD: u32 = 0x8;
pub const CN_VAL_DRBD: u32 = 0x1;
pub const CN_KVP_IDX: u32 = 0x9; // HyperV KVP
pub const CN_KVP_VAL: u32 = 0x1; // queries from the kernel
pub const CN_VSS_IDX: u32 = 0xA; // HyperV VSS
pub const CN_VSS_VAL: u32 = 0x1; // queries from the kernel

pub const CN_NETLINK_USERS: u32 = 11; // Highest index + 1

/*
 * Maximum connector's message size.
 */
pub const CONNECTOR_MAX_MSG_SIZE: u32 = 16384;

/*
 * idx and val are unique identifiers which 
 * are used for message routing and 
 * must be registered in connector.h for in-kernel usage.
 */

#[repr(C)]
pub struct cb_id {
    pub idx: u32,
    pub val: u32,
}

#[repr(C)]
pub struct cn_msg {
    pub id: cb_id,

    pub seq: u32,
    pub ack: u32,

    pub len: u16, // Length of the following data
    pub flags: u16,
    pub data: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
