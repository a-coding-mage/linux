// SPDX-License-Identifier: GPL-2.0-only
/*****************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


Public declarations for DSP Proramming Interface to TI C6701

Shared between hpi6000.c and DSP code

(C) Copyright AudioScience Inc. 1998-2003
******************************************************************************/

pub const HPI_NMIXER_CONTROLS: u32 = 200;

/*
 * Control caching is always supported in the HPI code.
 * The DSP should make sure that dwControlCacheSizeInBytes is initialized to 0
 * during boot to make it in-active.
 */
#[repr(C)]
pub struct hpi_hif_6000 {
    pub host_cmd: u32,
    pub dsp_ack: u32,
    pub address: u32,
    pub length: u32,
    pub message_buffer_address: u32,
    pub response_buffer_address: u32,
    pub dsp_number: u32,
    pub adapter_info: u32,
    pub control_cache_is_dirty: u32,
    pub control_cache_address: u32,
    pub control_cache_size_in_bytes: u32,
    pub control_cache_count: u32,
}

pub const fn HPI_HIF_PACK_ADAPTER_INFO(
    adapter: u32,
    version_major: u32,
    version_minor: u32,
) -> u32 {
    (adapter << 16) | (version_major << 8) | version_minor
}

pub const fn HPI_HIF_ADAPTER_INFO_EXTRACT_ADAPTER(adapterinfo: u32) -> u32 {
    (adapterinfo >> 16) & 0xffff
}

pub const fn HPI_HIF_ADAPTER_INFO_EXTRACT_HWVERSION_MAJOR(adapterinfo: u32) -> u32 {
    (adapterinfo >> 8) & 0xff
}

pub const fn HPI_HIF_ADAPTER_INFO_EXTRACT_HWVERSION_MINOR(adapterinfo: u32) -> u32 {
    adapterinfo & 0xff
}

/* Command/status exchanged between host and DSP */
pub const HPI_HIF_IDLE: u32 = 0;
pub const HPI_HIF_SEND_MSG: u32 = 1;
pub const HPI_HIF_GET_RESP: u32 = 2;
pub const HPI_HIF_DATA_MASK: u32 = 0x10;
pub const HPI_HIF_SEND_DATA: u32 = 0x13;
pub const HPI_HIF_GET_DATA: u32 = 0x14;
pub const HPI_HIF_SEND_DONE: u32 = 5;
pub const HPI_HIF_RESET: u32 = 9;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
