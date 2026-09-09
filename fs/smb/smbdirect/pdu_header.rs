/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (c) 2017 Stefan Metzmacher
 */

pub const SMBDIRECT_V1: u32 = 0x0100;

/* SMBD minimum receive size and fragmented sized defined in [MS-SMBD] */
pub const SMBDIRECT_MIN_RECEIVE_SIZE: u32 = 128;
pub const SMBDIRECT_MIN_FRAGMENTED_SIZE: u32 = 131072;

/* SMBD negotiation request packet [MS-SMBD] 2.2.1 */
#[repr(C, packed)]
pub struct smbdirect_negotiate_req {
    pub min_version: u16,
    pub max_version: u16,
    pub reserved: u16,
    pub credits_requested: u16,
    pub preferred_send_size: u32,
    pub max_receive_size: u32,
    pub max_fragmented_size: u32,
}

/* SMBD negotiation response packet [MS-SMBD] 2.2.2 */
#[repr(C, packed)]
pub struct smbdirect_negotiate_resp {
    pub min_version: u16,
    pub max_version: u16,
    pub negotiated_version: u16,
    pub reserved: u16,
    pub credits_requested: u16,
    pub credits_granted: u16,
    pub status: u32,
    pub max_readwrite_size: u32,
    pub preferred_send_size: u32,
    pub max_receive_size: u32,
    pub max_fragmented_size: u32,
}

pub const SMBDIRECT_DATA_MIN_HDR_SIZE: u32 = 0x14;
pub const SMBDIRECT_DATA_OFFSET: u32 = 0x18;

pub const SMBDIRECT_FLAG_RESPONSE_REQUESTED: u32 = 0x0001;

/* SMBD data transfer packet with payload [MS-SMBD] 2.2.3 */
#[repr(C, packed)]
pub struct smbdirect_data_transfer {
    pub credits_requested: u16,
    pub credits_granted: u16,
    pub flags: u16,
    pub reserved: u16,
    pub remaining_data_length: u32,
    pub data_offset: u32,
    pub data_length: u32,
    pub padding: u32,
    pub buffer: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
