/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2010, 2011 Mark Nelson and Tseng-Hui (Frank) Lin, IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h and linux/notifier.h

pub const PSERIES_IOEI_RPC_MAX_LEN: usize = 216;

pub const PSERIES_IOEI_TYPE_ERR_DETECTED: u8 = 0x01;
pub const PSERIES_IOEI_TYPE_ERR_RECOVERED: u8 = 0x02;
pub const PSERIES_IOEI_TYPE_EVENT: u8 = 0x03;
pub const PSERIES_IOEI_TYPE_RPC_PASS_THRU: u8 = 0x04;

pub const PSERIES_IOEI_SUBTYPE_NOT_APP: u8 = 0x00;
pub const PSERIES_IOEI_SUBTYPE_REBALANCE_REQ: u8 = 0x01;
pub const PSERIES_IOEI_SUBTYPE_NODE_ONLINE: u8 = 0x03;
pub const PSERIES_IOEI_SUBTYPE_NODE_OFFLINE: u8 = 0x04;
pub const PSERIES_IOEI_SUBTYPE_DUMP_SIZE_CHANGE: u8 = 0x05;
pub const PSERIES_IOEI_SUBTYPE_TORRENT_IRV_UPDATE: u8 = 0x06;
pub const PSERIES_IOEI_SUBTYPE_TORRENT_HFI_CFGED: u8 = 0x07;

pub const PSERIES_IOEI_SCOPE_NOT_APP: u8 = 0x00;
pub const PSERIES_IOEI_SCOPE_RIO_HUB: u8 = 0x36;
pub const PSERIES_IOEI_SCOPE_RIO_BRIDGE: u8 = 0x37;
pub const PSERIES_IOEI_SCOPE_PHB: u8 = 0x38;
pub const PSERIES_IOEI_SCOPE_EADS_GLOBAL: u8 = 0x39;
pub const PSERIES_IOEI_SCOPE_EADS_SLOT: u8 = 0x3A;
pub const PSERIES_IOEI_SCOPE_TORRENT_HUB: u8 = 0x3B;
pub const PSERIES_IOEI_SCOPE_SERVICE_PROC: u8 = 0x51;

/* Platform Event Log Format, Version 6, data portition of IO event section */
#[repr(C)]
pub struct pseries_io_event {
    pub event_type: u8, // 0x00 IO-Event Type
    pub rpc_data_len: u8, // 0x01 RPC data length
    pub scope: u8, // 0x02 Error/Event Scope
    pub event_subtype: u8, // 0x03 I/O-Event Sub-Type
    pub drc_index: u32, // 0x04 DRC Index
    pub rpc_data: [u8; PSERIES_IOEI_RPC_MAX_LEN],
    // 0x08 RPC Data (0-216 bytes, padded to 4 bytes alignment)
}

// Declaration supplied by linux/notifier.h; retained as an external symbol.
unsafe extern "C" {
    pub static mut pseries_ioei_notifier_list: atomic_notifier_head;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
