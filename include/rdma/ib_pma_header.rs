/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2006, 2007, 2008, 2009, 2010 QLogic Corporation.
 * All rights reserved.
 * Copyright (c) 2005, 2006 PathScale, Inc. All rights reserved.
 */

// Dependency supplied by rdma/ib_mad.h in the original header.

/*
 * PMA class portinfo capability mask bits
 */
pub const IB_PMA_CLASS_CAP_ALLPORTSELECT: u16 = (1u16 << 8).to_be();
pub const IB_PMA_CLASS_CAP_EXT_WIDTH: u16 = (1u16 << 9).to_be();
pub const IB_PMA_CLASS_CAP_EXT_WIDTH_NOIETF: u16 = (1u16 << 10).to_be();
pub const IB_PMA_CLASS_CAP_XMIT_WAIT: u16 = (1u16 << 12).to_be();

pub const IB_PMA_CLASS_PORT_INFO: u16 = 0x0001u16.to_be();
pub const IB_PMA_PORT_SAMPLES_CONTROL: u16 = 0x0010u16.to_be();
pub const IB_PMA_PORT_SAMPLES_RESULT: u16 = 0x0011u16.to_be();
pub const IB_PMA_PORT_COUNTERS: u16 = 0x0012u16.to_be();
pub const IB_PMA_PORT_COUNTERS_EXT: u16 = 0x001Du16.to_be();
pub const IB_PMA_PORT_SAMPLES_RESULT_EXT: u16 = 0x001Eu16.to_be();

#[repr(C, packed)]
pub struct ib_pma_mad {
    pub mad_hdr: ib_mad_hdr,
    pub reserved: [u8; 40],
    pub data: [u8; 192],
}

#[repr(C)]
pub struct ib_pma_portsamplescontrol {
    pub opcode: u8,
    pub port_select: u8,
    pub tick: u8,
    pub counter_width: u8, // resv: 7:3, counter width: 2:0
    pub counter_mask0_9: u32, // 2, 10 3-bit fields
    pub counter_mask10_14: u16, // 1, 5 3-bit fields
    pub sample_mechanisms: u8,
    pub sample_status: u8, // only lower 2 bits
    pub option_mask: u64,
    pub vendor_mask: u64,
    pub sample_start: u32,
    pub sample_interval: u32,
    pub tag: u16,
    pub counter_select: [u16; 15],
    pub reserved1: u32,
    pub samples_only_option_mask: u64,
    pub reserved2: [u32; 28],
}

#[repr(C)]
pub struct ib_pma_portsamplesresult {
    pub tag: u16,
    pub sample_status: u16, // only lower 2 bits
    pub counter: [u32; 15],
}

#[repr(C)]
pub struct ib_pma_portsamplesresult_ext {
    pub tag: u16,
    pub sample_status: u16, // only lower 2 bits
    pub extended_width: u32, // only upper 2 bits
    pub counter: [u64; 15],
}

#[repr(C, packed)]
pub struct ib_pma_portcounters {
    pub reserved: u8,
    pub port_select: u8,
    pub counter_select: u16,
    pub symbol_error_counter: u16,
    pub link_error_recovery_counter: u8,
    pub link_downed_counter: u8,
    pub port_rcv_errors: u16,
    pub port_rcv_remphys_errors: u16,
    pub port_rcv_switch_relay_errors: u16,
    pub port_xmit_discards: u16,
    pub port_xmit_constraint_errors: u8,
    pub port_rcv_constraint_errors: u8,
    pub reserved1: u8,
    pub link_overrun_errors: u8, // LocalLink: 7:4, BufferOverrun: 3:0
    pub reserved2: u16,
    pub vl15_dropped: u16,
    pub port_xmit_data: u32,
    pub port_rcv_data: u32,
    pub port_xmit_packets: u32,
    pub port_rcv_packets: u32,
    pub port_xmit_wait: u32,
}

pub const IB_PMA_SEL_SYMBOL_ERROR: u16 = 0x0001u16.to_be();
pub const IB_PMA_SEL_LINK_ERROR_RECOVERY: u16 = 0x0002u16.to_be();
pub const IB_PMA_SEL_LINK_DOWNED: u16 = 0x0004u16.to_be();
pub const IB_PMA_SEL_PORT_RCV_ERRORS: u16 = 0x0008u16.to_be();
pub const IB_PMA_SEL_PORT_RCV_REMPHYS_ERRORS: u16 = 0x0010u16.to_be();
pub const IB_PMA_SEL_PORT_XMIT_DISCARDS: u16 = 0x0040u16.to_be();
pub const IB_PMA_SEL_LOCAL_LINK_INTEGRITY_ERRORS: u16 = 0x0200u16.to_be();
pub const IB_PMA_SEL_EXCESSIVE_BUFFER_OVERRUNS: u16 = 0x0400u16.to_be();
pub const IB_PMA_SEL_PORT_VL15_DROPPED: u16 = 0x0800u16.to_be();
pub const IB_PMA_SEL_PORT_XMIT_DATA: u16 = 0x1000u16.to_be();
pub const IB_PMA_SEL_PORT_RCV_DATA: u16 = 0x2000u16.to_be();
pub const IB_PMA_SEL_PORT_XMIT_PACKETS: u16 = 0x4000u16.to_be();
pub const IB_PMA_SEL_PORT_RCV_PACKETS: u16 = 0x8000u16.to_be();

#[repr(C, packed)]
pub struct ib_pma_portcounters_ext {
    pub reserved: u8,
    pub port_select: u8,
    pub counter_select: u16,
    pub reserved1: u32,
    pub port_xmit_data: u64,
    pub port_rcv_data: u64,
    pub port_xmit_packets: u64,
    pub port_rcv_packets: u64,
    pub port_unicast_xmit_packets: u64,
    pub port_unicast_rcv_packets: u64,
    pub port_multicast_xmit_packets: u64,
    pub port_multicast_rcv_packets: u64,
}

pub const IB_PMA_SELX_PORT_XMIT_DATA: u16 = 0x0001u16.to_be();
pub const IB_PMA_SELX_PORT_RCV_DATA: u16 = 0x0002u16.to_be();
pub const IB_PMA_SELX_PORT_XMIT_PACKETS: u16 = 0x0004u16.to_be();
pub const IB_PMA_SELX_PORT_RCV_PACKETS: u16 = 0x0008u16.to_be();
pub const IB_PMA_SELX_PORT_UNI_XMIT_PACKETS: u16 = 0x0010u16.to_be();
pub const IB_PMA_SELX_PORT_UNI_RCV_PACKETS: u16 = 0x0020u16.to_be();
pub const IB_PMA_SELX_PORT_MULTI_XMIT_PACKETS: u16 = 0x0040u16.to_be();
pub const IB_PMA_SELX_PORT_MULTI_RCV_PACKETS: u16 = 0x0080u16.to_be();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
