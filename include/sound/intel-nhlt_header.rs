/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  intel-nhlt.h - Intel HDA Platform NHLT header
 *
 *  Copyright (c) 2015-2019 Intel Corporation
 */

// Dependency supplied by the Linux ACPI headers.
#[repr(C)]
pub struct acpi_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum nhlt_link_type {
    NHLT_LINK_HDA = 0,
    NHLT_LINK_DSP = 1,
    NHLT_LINK_DMIC = 2,
    NHLT_LINK_SSP = 3,
    NHLT_LINK_INVALID,
}

#[repr(i32)]
pub enum nhlt_device_type {
    NHLT_DEVICE_BT = 0,
    NHLT_DEVICE_DMIC = 1,
    NHLT_DEVICE_I2S = 4,
    NHLT_DEVICE_INVALID,
}

#[repr(C, packed)]
pub struct wav_fmt {
    pub fmt_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

#[repr(C)]
pub union samples {
    pub valid_bits_per_sample: u16,
    pub samples_per_block: u16,
    pub reserved: u16,
}

#[repr(C, packed)]
pub struct wav_fmt_ext {
    pub fmt: wav_fmt,
    pub sample: samples,
    pub channel_mask: u32,
    pub sub_fmt: [u8; 16],
}

#[repr(C, packed)]
pub struct nhlt_specific_cfg {
    pub size: u32,
    pub caps: [u8; 0],
}

#[repr(C, packed)]
pub struct nhlt_fmt_cfg {
    pub fmt_ext: wav_fmt_ext,
    pub config: nhlt_specific_cfg,
}

#[repr(C, packed)]
pub struct nhlt_fmt {
    pub fmt_count: u8,
    pub fmt_config: [nhlt_fmt_cfg; 0],
}

#[repr(C, packed)]
pub struct nhlt_endpoint {
    pub length: u32,
    pub linktype: u8,
    pub instance_id: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u16,
    pub subsystem_id: u32,
    pub device_type: u8,
    pub direction: u8,
    pub virtual_bus_id: u8,
    pub config: nhlt_specific_cfg,
}

#[repr(C, packed)]
pub struct nhlt_acpi_table {
    pub header: acpi_table_header,
    pub endpoint_count: u8,
    pub desc: [nhlt_endpoint; 0],
}

#[repr(C, packed)]
pub struct nhlt_resource_desc {
    pub extra: u32,
    pub flags: u16,
    pub addr_spc_gra: u64,
    pub min_addr: u64,
    pub max_addr: u64,
    pub addr_trans_offset: u64,
    pub length: u64,
}

pub const MIC_ARRAY_2CH: i32 = 2;
pub const MIC_ARRAY_4CH: i32 = 4;

#[repr(C, packed)]
pub struct nhlt_device_specific_config {
    pub virtual_slot: u8,
    pub config_type: u8,
}

#[repr(C, packed)]
pub struct nhlt_dmic_array_config {
    pub device_config: nhlt_device_specific_config,
    pub array_type: u8,
}

#[repr(C, packed)]
pub struct nhlt_vendor_dmic_array_config {
    pub dmic_config: nhlt_dmic_array_config,
    pub nb_mics: u8,
    /* TODO add vendor mic config */
}

pub const NHLT_CONFIG_TYPE_GENERIC: i32 = 0;
pub const NHLT_CONFIG_TYPE_MIC_ARRAY: i32 = 1;

pub const NHLT_MIC_ARRAY_2CH_SMALL: i32 = 0xa;
pub const NHLT_MIC_ARRAY_2CH_BIG: i32 = 0xb;
pub const NHLT_MIC_ARRAY_4CH_1ST_GEOM: i32 = 0xc;
pub const NHLT_MIC_ARRAY_4CH_L_SHAPED: i32 = 0xd;
pub const NHLT_MIC_ARRAY_4CH_2ND_GEOM: i32 = 0xe;
pub const NHLT_MIC_ARRAY_VENDOR_DEFINED: i32 = 0xf;

// The following declarations correspond to the CONFIG_ACPI and
// CONFIG_SND_INTEL_NHLT build-time condition.
#[cfg(all(feature = "acpi", feature = "snd_intel_nhlt"))]
extern "C" {
    pub fn intel_nhlt_init(dev: *mut device) -> *mut nhlt_acpi_table;
    pub fn intel_nhlt_free(addr: *mut nhlt_acpi_table);
    pub fn intel_nhlt_get_dmic_geo(dev: *mut device, nhlt: *mut nhlt_acpi_table) -> i32;
    pub fn intel_nhlt_has_endpoint_type(nhlt: *mut nhlt_acpi_table, link_type: u8) -> bool;
    pub fn intel_nhlt_ssp_endpoint_mask(nhlt: *mut nhlt_acpi_table, device_type: u8) -> i32;
    pub fn intel_nhlt_ssp_mclk_mask(nhlt: *mut nhlt_acpi_table, ssp_num: i32) -> i32;
    pub fn intel_nhlt_get_endpoint_blob(
        dev: *mut device,
        nhlt: *mut nhlt_acpi_table,
        bus_id: u32,
        link_type: u8,
        vbps: u8,
        bps: u8,
        num_ch: u8,
        rate: u32,
        dir: u8,
        dev_type: u8,
    ) -> *mut nhlt_specific_cfg;
    pub fn intel_nhlt_ssp_device_type(
        dev: *mut device,
        nhlt: *mut nhlt_acpi_table,
        virtual_bus_id: u8,
    ) -> i32;
}

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_init(_dev: *mut device) -> *mut nhlt_acpi_table {
    core::ptr::null_mut()
}

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_free(_addr: *mut nhlt_acpi_table) {}

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_get_dmic_geo(_dev: *mut device, _nhlt: *mut nhlt_acpi_table) -> i32 { 0 }

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_has_endpoint_type(_nhlt: *mut nhlt_acpi_table, _link_type: u8) -> bool { false }

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_ssp_endpoint_mask(_nhlt: *mut nhlt_acpi_table, _device_type: u8) -> i32 { 0 }

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_ssp_mclk_mask(_nhlt: *mut nhlt_acpi_table, _ssp_num: i32) -> i32 { 0 }

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_get_endpoint_blob(
    _dev: *mut device,
    _nhlt: *mut nhlt_acpi_table,
    _bus_id: u32,
    _link_type: u8,
    _vbps: u8,
    _bps: u8,
    _num_ch: u8,
    _rate: u32,
    _dir: u8,
    _dev_type: u8,
) -> *mut nhlt_specific_cfg { core::ptr::null_mut() }

#[cfg(not(all(feature = "acpi", feature = "snd_intel_nhlt")))]
pub unsafe fn intel_nhlt_ssp_device_type(
    _dev: *mut device,
    _nhlt: *mut nhlt_acpi_table,
    _virtual_bus_id: u8,
) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
