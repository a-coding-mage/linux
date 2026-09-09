/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * linux/can/gw.h
 *
 * Definitions for CAN frame Gateway/Router/Bridge
 *
 * Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 * Copyright (c) 2011 Volkswagen Group Electronic Research
 * All rights reserved.
 */

// C dependencies: linux/types.h and linux/can.h provide the integer aliases,
// `can_frame`, and `canfd_frame` referenced below.

#[repr(C)]
pub struct rtcanmsg {
    pub can_family: __u8,
    pub gwtype: __u8,
    pub flags: __u16,
}

/* CAN gateway types */
pub const CGW_TYPE_UNSPEC: i32 = 0;
pub const CGW_TYPE_CAN_CAN: i32 = 1; /* CAN->CAN routing */
pub const __CGW_TYPE_MAX: i32 = 2;
pub const CGW_TYPE_MAX: i32 = __CGW_TYPE_MAX - 1;

/* CAN rtnetlink attribute definitions */
pub const CGW_UNSPEC: i32 = 0;
pub const CGW_MOD_AND: i32 = 1; /* CAN frame modification binary AND */
pub const CGW_MOD_OR: i32 = 2; /* CAN frame modification binary OR */
pub const CGW_MOD_XOR: i32 = 3; /* CAN frame modification binary XOR */
pub const CGW_MOD_SET: i32 = 4; /* CAN frame modification set alternate values */
pub const CGW_CS_XOR: i32 = 5; /* set data[] XOR checksum into data[index] */
pub const CGW_CS_CRC8: i32 = 6; /* set data[] CRC8 checksum into data[index] */
pub const CGW_HANDLED: i32 = 7; /* number of handled CAN frames */
pub const CGW_DROPPED: i32 = 8; /* number of dropped CAN frames */
pub const CGW_SRC_IF: i32 = 9; /* ifindex of source network interface */
pub const CGW_DST_IF: i32 = 10; /* ifindex of destination network interface */
pub const CGW_FILTER: i32 = 11; /* specify struct can_filter on source CAN device */
pub const CGW_DELETED: i32 = 12; /* number of deleted CAN frames (see max_hops param) */
pub const CGW_LIM_HOPS: i32 = 13; /* limit the number of hops of this specific rule */
pub const CGW_MOD_UID: i32 = 14; /* user defined identifier for modification updates */
pub const CGW_FDMOD_AND: i32 = 15; /* CAN FD frame modification binary AND */
pub const CGW_FDMOD_OR: i32 = 16; /* CAN FD frame modification binary OR */
pub const CGW_FDMOD_XOR: i32 = 17; /* CAN FD frame modification binary XOR */
pub const CGW_FDMOD_SET: i32 = 18; /* CAN FD frame modification set alternate values */
pub const __CGW_MAX: i32 = 19;
pub const CGW_MAX: i32 = __CGW_MAX - 1;

pub const CGW_FLAGS_CAN_ECHO: u32 = 0x01;
pub const CGW_FLAGS_CAN_SRC_TSTAMP: u32 = 0x02;
pub const CGW_FLAGS_CAN_IIF_TX_OK: u32 = 0x04;
pub const CGW_FLAGS_CAN_FD: u32 = 0x08;

pub const CGW_MOD_FUNCS: usize = 4; /* AND OR XOR SET */
pub const CGW_MOD_ID: u32 = 0x01;
pub const CGW_MOD_DLC: u32 = 0x02; /* Classical CAN data length code */
pub const CGW_MOD_LEN: u32 = CGW_MOD_DLC; /* CAN FD (plain) data length */
pub const CGW_MOD_DATA: u32 = 0x04;
pub const CGW_MOD_FLAGS: u32 = 0x08; /* CAN FD flags */
pub const CGW_FRAME_MODS: usize = 4; /* ID DLC/LEN DATA FLAGS */
pub const MAX_MODFUNCTIONS: usize = CGW_MOD_FUNCS * CGW_FRAME_MODS;

#[repr(C, packed)]
pub struct cgw_frame_mod {
    pub cf: can_frame,
    pub modtype: __u8,
}

#[repr(C, packed)]
pub struct cgw_fdframe_mod {
    pub cf: canfd_frame,
    pub modtype: __u8,
}

pub const CGW_MODATTR_LEN: usize = core::mem::size_of::<cgw_frame_mod>();
pub const CGW_FDMODATTR_LEN: usize = core::mem::size_of::<cgw_fdframe_mod>();

#[repr(C, packed)]
pub struct cgw_csum_xor {
    pub from_idx: __s8,
    pub to_idx: __s8,
    pub result_idx: __s8,
    pub init_xor_val: __u8,
}

#[repr(C, packed)]
pub struct cgw_csum_crc8 {
    pub from_idx: __s8,
    pub to_idx: __s8,
    pub result_idx: __s8,
    pub init_crc_val: __u8,
    pub final_xor_val: __u8,
    pub crctab: [__u8; 256],
    pub profile: __u8,
    pub profile_data: [__u8; 20],
}

pub const CGW_CS_XOR_LEN: usize = core::mem::size_of::<cgw_csum_xor>();
pub const CGW_CS_CRC8_LEN: usize = core::mem::size_of::<cgw_csum_crc8>();

/* CRC8 profiles (compute CRC for additional data elements - see below) */
pub const CGW_CRC8PRF_UNSPEC: i32 = 0;
pub const CGW_CRC8PRF_1U8: i32 = 1; /* compute one additional u8 value */
pub const CGW_CRC8PRF_16U8: i32 = 2; /* u8 value table indexed by data[1] & 0xF */
pub const CGW_CRC8PRF_SFFID_XOR: i32 = 3; /* (can_id & 0xFF) ^ (can_id >> 8 & 0xFF) */
pub const __CGW_CRC8PRF_MAX: i32 = 4;
pub const CGW_CRC8PRF_MAX: i32 = __CGW_CRC8PRF_MAX - 1;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
