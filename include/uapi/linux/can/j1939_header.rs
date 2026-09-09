/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * j1939.h
 *
 * Copyright (c) 2010-2011 EIA Electronics
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependencies supplied by the surrounding Linux UAPI bindings:
// linux/types.h, linux/socket.h, and linux/can.h.

pub const J1939_MAX_UNICAST_ADDR: u32 = 0xfd;
pub const J1939_IDLE_ADDR: u32 = 0xfe;
pub const J1939_NO_ADDR: u32 = 0xff; // == broadcast or no addr
pub const J1939_NO_NAME: u64 = 0;
pub const J1939_PGN_REQUEST: u32 = 0x0ea00; // Request PG
pub const J1939_PGN_ADDRESS_CLAIMED: u32 = 0x0ee00; // Address Claimed
pub const J1939_PGN_ADDRESS_COMMANDED: u32 = 0x0fed8; // Commanded Address
pub const J1939_PGN_PDU1_MAX: u32 = 0x3ff00;
pub const J1939_PGN_MAX: u32 = 0x3ffff;
pub const J1939_NO_PGN: u32 = 0x40000;

/* J1939 Parameter Group Number
 *
 * bit 0-7  : PDU Specific (PS)
 * bit 8-15 : PDU Format (PF)
 * bit 16   : Data Page (DP)
 * bit 17   : Reserved (R)
 * bit 19-31: set to zero
 */
pub type PgnT = u32;

/* J1939 Priority
 *
 * bit 0-2: Priority (P)
 * bit 3-7: set to zero
 */
pub type PriorityT = u8;

/* J1939 NAME
 *
 * bit 0-20 : Identity Number
 * bit 21-31: Manufacturer Code
 * bit 32-34: ECU Instance
 * bit 35-39: Function Instance
 * bit 40-47: Function
 * bit 48   : Reserved
 * bit 49-55: Vehicle System
 * bit 56-59: Vehicle System Instance
 * bit 60-62: Industry Group
 * bit 63   : Arbitrary Address Capable
 */
pub type NameT = u64;

/* J1939 socket options */
// SOL_CAN_BASE and CAN_J1939 are supplied by the surrounding CAN bindings.
pub const SOL_CAN_J1939: i32 = SOL_CAN_BASE + CAN_J1939;

pub const SO_J1939_FILTER: i32 = 1; // set filters
pub const SO_J1939_PROMISC: i32 = 2; // set/clr promiscuous mode
pub const SO_J1939_SEND_PRIO: i32 = 3;
pub const SO_J1939_ERRQUEUE: i32 = 4;

pub const SCM_J1939_DEST_ADDR: i32 = 1;
pub const SCM_J1939_DEST_NAME: i32 = 2;
pub const SCM_J1939_PRIO: i32 = 3;
pub const SCM_J1939_ERRQUEUE: i32 = 4;

pub const J1939_NLA_PAD: i32 = 0;
pub const J1939_NLA_BYTES_ACKED: i32 = 1;
pub const J1939_NLA_TOTAL_SIZE: i32 = 2;
pub const J1939_NLA_PGN: i32 = 3;
pub const J1939_NLA_SRC_NAME: i32 = 4;
pub const J1939_NLA_DEST_NAME: i32 = 5;
pub const J1939_NLA_SRC_ADDR: i32 = 6;
pub const J1939_NLA_DEST_ADDR: i32 = 7;

pub const J1939_EE_INFO_NONE: i32 = 0;
pub const J1939_EE_INFO_TX_ABORT: i32 = 1;
pub const J1939_EE_INFO_RX_RTS: i32 = 2;
pub const J1939_EE_INFO_RX_DPO: i32 = 3;
pub const J1939_EE_INFO_RX_ABORT: i32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct J1939Filter {
    pub name: NameT,
    pub name_mask: NameT,
    pub pgn: PgnT,
    pub pgn_mask: PgnT,
    pub addr: u8,
    pub addr_mask: u8,
}

pub const J1939_FILTER_MAX: usize = 512; // maximum number of j1939_filter set via setsockopt()

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
