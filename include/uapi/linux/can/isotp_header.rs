/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * linux/can/isotp.h
 *
 * Definitions for ISO 15765-2 CAN transport protocol sockets
 *
 * Copyright (c) 2020 Volkswagen Group Electronic Research
 * All rights reserved.
 *
 * This file is available under the licensing terms of the original C header.
 */

// Dependencies supplied by the corresponding Linux headers:
// SOL_CAN_BASE, CAN_ISOTP, CAN_MTU, and CAN_MAX_DLEN.

pub const SOL_CAN_ISOTP: u32 = SOL_CAN_BASE + CAN_ISOTP;

/* for socket options affecting the socket (not the global system) */
pub const CAN_ISOTP_OPTS: u32 = 1; /* pass struct can_isotp_options */
pub const CAN_ISOTP_RECV_FC: u32 = 2; /* pass struct can_isotp_fc_options */

/* sockopts to force stmin timer values for protocol regression tests */
pub const CAN_ISOTP_TX_STMIN: u32 = 3; /* pass __u32 value in nano secs */
pub const CAN_ISOTP_RX_STMIN: u32 = 4; /* pass __u32 value in nano secs */
pub const CAN_ISOTP_LL_OPTS: u32 = 5; /* pass struct can_isotp_ll_options */

#[repr(C)]
#[derive Copy, Clone]
pub struct can_isotp_options {
    pub flags: u32,
    pub frame_txtime: u32,
    pub ext_address: u8,
    pub txpad_content: u8,
    pub rxpad_content: u8,
    pub rx_ext_address: u8,
}

#[repr(C)]
#[derive Copy, Clone]
pub struct can_isotp_fc_options {
    pub bs: u8,
    pub stmin: u8,
    pub wftmax: u8,
}

#[repr(C)]
#[derive Copy, Clone]
pub struct can_isotp_ll_options {
    pub mtu: u8,
    pub tx_dl: u8,
    pub tx_flags: u8,
}

/* flags for isotp behaviour */
pub const CAN_ISOTP_LISTEN_MODE: u32 = 0x0001;
pub const CAN_ISOTP_EXTEND_ADDR: u32 = 0x0002;
pub const CAN_ISOTP_TX_PADDING: u32 = 0x0004;
pub const CAN_ISOTP_RX_PADDING: u32 = 0x0008;
pub const CAN_ISOTP_CHK_PAD_LEN: u32 = 0x0010;
pub const CAN_ISOTP_CHK_PAD_DATA: u32 = 0x0020;
pub const CAN_ISOTP_HALF_DUPLEX: u32 = 0x0040;
pub const CAN_ISOTP_FORCE_TXSTMIN: u32 = 0x0080;
pub const CAN_ISOTP_FORCE_RXSTMIN: u32 = 0x0100;
pub const CAN_ISOTP_RX_EXT_ADDR: u32 = 0x0200;
pub const CAN_ISOTP_WAIT_TX_DONE: u32 = 0x0400;
pub const CAN_ISOTP_SF_BROADCAST: u32 = 0x0800;
pub const CAN_ISOTP_CF_BROADCAST: u32 = 0x1000;
pub const CAN_ISOTP_DYN_FC_PARMS: u32 = 0x2000;

/* protocol machine default values */
pub const CAN_ISOTP_DEFAULT_FLAGS: u32 = 0;
pub const CAN_ISOTP_DEFAULT_EXT_ADDRESS: u32 = 0x00;
pub const CAN_ISOTP_DEFAULT_PAD_CONTENT: u32 = 0xCC;
pub const CAN_ISOTP_DEFAULT_FRAME_TXTIME: u32 = 50000;
pub const CAN_ISOTP_DEFAULT_RECV_BS: u32 = 0;
pub const CAN_ISOTP_DEFAULT_RECV_STMIN: u32 = 0x00;
pub const CAN_ISOTP_DEFAULT_RECV_WFTMAX: u32 = 0;

/* link layer default values => make use of Classical CAN frames */
pub const CAN_ISOTP_DEFAULT_LL_MTU: u32 = CAN_MTU;
pub const CAN_ISOTP_DEFAULT_LL_TX_DL: u32 = CAN_MAX_DLEN;
pub const CAN_ISOTP_DEFAULT_LL_TX_FLAGS: u32 = 0;

pub const CAN_ISOTP_FRAME_TXTIME_ZERO: u32 = 0xFFFFFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
