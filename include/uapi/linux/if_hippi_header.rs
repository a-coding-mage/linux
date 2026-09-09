/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET An implementation of the TCP/IP protocol suite for Linux.
 * Global definitions for the HIPPI interface.
 */

/* <linux/types.h> and <asm/byteorder.h> provide the integer and byte-order
 * types used by the original header. */

pub const HIPPI_ALEN: usize = 6; /* Bytes in one HIPPI hw-addr */
pub const HIPPI_HLEN: usize = core::mem::size_of::<hippi_hdr>();
pub const HIPPI_ZLEN: usize = 0; /* Min. bytes in frame without FCS */
pub const HIPPI_DATA_LEN: usize = 65280; /* Max. bytes in payload */
pub const HIPPI_FRAME_LEN: usize = HIPPI_DATA_LEN + HIPPI_HLEN;

pub const HIPPI_EXTENDED_SAP: u8 = 0xAA;
pub const HIPPI_UI_CMD: u8 = 0x03;

#[repr(C)]
pub struct hipnet_statistics {
    pub rx_packets: i32,
    pub tx_packets: i32,
    pub rx_errors: i32,
    pub tx_errors: i32,
    pub rx_dropped: i32,
    pub tx_dropped: i32,
    pub rx_length_errors: i32,
    pub rx_over_errors: i32,
    pub rx_crc_errors: i32,
    pub rx_frame_errors: i32,
    pub rx_fifo_errors: i32,
    pub rx_missed_errors: i32,
    pub tx_aborted_errors: i32,
    pub tx_carrier_errors: i32,
    pub tx_fifo_errors: i32,
    pub tx_heartbeat_errors: i32,
    pub tx_window_errors: i32,
}

#[repr(C, packed)]
pub struct hippi_fp_hdr {
    /* The disabled ULP/bit-field layout is selected out in the C header. */
    pub fixed: u32,
    pub d2_size: u32,
}

#[repr(C, packed)]
pub struct hippi_le_hdr {
    /* fc:3, double_wide:1, message_type:4; bit order follows target endian. */
    pub fc_double_wide_message_type: u8,
    pub dest_switch_addr: [u8; 3],
    /* dest_addr_type:4, src_addr_type:4; bit order follows target endian. */
    pub dest_src_addr_type: u8,
    pub src_switch_addr: [u8; 3],
    pub reserved: u16,
    pub daddr: [u8; HIPPI_ALEN],
    pub locally_administered: u16,
    pub saddr: [u8; HIPPI_ALEN],
}

pub const HIPPI_OUI_LEN: usize = 3;

/* The dsap and ssap fields appear swapped by mistake in RFC 2067. */
#[repr(C, packed)]
pub struct hippi_snap_hdr {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub oui: [u8; HIPPI_OUI_LEN],
    pub ethertype: u16,
}

#[repr(C, packed)]
pub struct hippi_hdr {
    pub fp: hippi_fp_hdr,
    pub le: hippi_le_hdr,
    pub snap: hippi_snap_hdr,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
