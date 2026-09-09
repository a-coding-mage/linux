/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (c) 2013-2019, 2021 The Linux Foundation. All rights reserved.
 */

use core::primitive::{u16, u8};

#[repr(C, packed)]
pub struct rmnet_map_header {
    pub flags: u8, /* MAP_CMD_FLAG, MAP_PAD_LEN_MASK */
    pub mux_id: u8,
    pub pkt_len: u16, /* Length of packet, including pad */
}

/* rmnet_map_header flags field:
 *  PAD_LEN:    number of pad bytes following packet data
 *  CMD:        1 = packet contains a MAP command; 0 = packet contains data
 *  NEXT_HEADER: 1 = packet contains V5 CSUM header 0 = no V5 CSUM header
 */
pub const MAP_PAD_LEN_MASK: u8 = (1u8 << 6) - 1;
pub const MAP_NEXT_HEADER_FLAG: u8 = 1u8 << 6;
pub const MAP_CMD_FLAG: u8 = 1u8 << 7;

#[repr(C, packed)]
pub struct rmnet_map_dl_csum_trailer {
    pub reserved1: u8,
    pub flags: u8, /* MAP_CSUM_DL_VALID_FLAG */
    pub csum_start_offset: u16,
    pub csum_length: u16,
    pub csum_value: u16,
}

/* rmnet_map_dl_csum_trailer flags field:
 *  VALID:  1 = checksum and length valid; 0 = ignore them
 */
pub const MAP_CSUM_DL_VALID_FLAG: u8 = 1u8 << 0;

#[repr(C, packed)]
pub struct rmnet_map_ul_csum_header {
    pub csum_start_offset: u16,
    pub csum_info: u16, /* MAP_CSUM_UL_* */
}

/* csum_info field:
 *  OFFSET:   where (offset in bytes) to insert computed checksum
 *  UDP:      1 = UDP checksum (zero checksum means no checksum)
 *  ENABLED:  1 = checksum computation requested
 */
pub const MAP_CSUM_UL_OFFSET_MASK: u16 = (1u16 << 14) - 1;
pub const MAP_CSUM_UL_UDP_FLAG: u16 = 1u16 << 14;
pub const MAP_CSUM_UL_ENABLED_FLAG: u16 = 1u16 << 15;

/* MAP CSUM headers */
#[repr(C, packed)]
pub struct rmnet_map_v5_csum_header {
    pub header_info: u8,
    pub csum_info: u8,
    pub reserved: u16,
}

/* v5 header_info field
 * NEXT_HEADER: represents whether there is any next header
 * HEADER_TYPE: represents the type of this header
 *
 * csum_info field
 * CSUM_VALID_OR_REQ:
 * 1 = for UL, checksum computation is requested.
 * 1 = for DL, validated the checksum and has found it valid
 */

pub const MAPV5_HDRINFO_NXT_HDR_FLAG: u8 = 1u8 << 0;
pub const MAPV5_HDRINFO_HDR_TYPE_FMASK: u8 = ((1u16 << 8) - 1) as u8 & !((1u8 << 1) - 1);
pub const MAPV5_CSUMINFO_VALID_FLAG: u8 = 1u8 << 7;

pub const RMNET_MAP_HEADER_TYPE_CSUM_OFFLOAD: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
