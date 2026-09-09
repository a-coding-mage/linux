/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wmfw.h - Wolfson firmware format information
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Dependency intent: the Linux integer types used by this header are supplied
// by the surrounding translation unit.

pub const WMFW_MAX_ALG_NAME: usize = 256;
pub const WMFW_MAX_ALG_DESCR_NAME: usize = 256;

pub const WMFW_MAX_COEFF_NAME: usize = 256;
pub const WMFW_MAX_COEFF_DESCR_NAME: usize = 256;

pub const WMFW_CTL_FLAG_SYS: u32 = 0x8000;
pub const WMFW_CTL_FLAG_VOLATILE: u32 = 0x0004;
pub const WMFW_CTL_FLAG_WRITEABLE: u32 = 0x0002;
pub const WMFW_CTL_FLAG_READABLE: u32 = 0x0001;

pub const WMFW_CTL_TYPE_BYTES: u32 = 0x0004; // byte control

// Non-ALSA coefficient types start at 0x1000
pub const WMFW_CTL_TYPE_ACKED: u32 = 0x1000; // acked control
pub const WMFW_CTL_TYPE_HOSTEVENT: u32 = 0x1001; // event control
pub const WMFW_CTL_TYPE_HOST_BUFFER: u32 = 0x1002; // host buffer pointer
pub const WMFW_CTL_TYPE_FWEVENT: u32 = 0x1004; // firmware event control

#[repr(C, packed)]
pub struct wmfw_header {
    pub magic: [core::ffi::c_char; 4],
    pub len: __le32,
    pub rev: __le16,
    pub core: u8,
    pub ver: u8,
}

#[repr(C, packed)]
pub struct wmfw_footer {
    pub timestamp: __le64,
    pub checksum: __le32,
}

#[repr(C, packed)]
pub struct wmfw_adsp1_sizes {
    pub dm: __le32,
    pub pm: __le32,
    pub zm: __le32,
}

#[repr(C, packed)]
pub struct wmfw_adsp2_sizes {
    pub xm: __le32,
    pub ym: __le32,
    pub pm: __le32,
    pub zm: __le32,
}

#[repr(C)]
pub union wmfw_region_type_offset {
    pub r#type: __be32,
    pub offset: __le32,
}

#[repr(C, packed)]
pub struct wmfw_region {
    pub type_offset: wmfw_region_type_offset,
    pub len: __le32,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct wmfw_id_hdr {
    pub core_id: __be32,
    pub core_rev: __be32,
    pub id: __be32,
    pub ver: __be32,
}

#[repr(C, packed)]
pub struct wmfw_v3_id_hdr {
    pub core_id: __be32,
    pub block_rev: __be32,
    pub vendor_id: __be32,
    pub id: __be32,
    pub ver: __be32,
}

#[repr(C, packed)]
pub struct wmfw_adsp1_id_hdr {
    pub fw: wmfw_id_hdr,
    pub zm: __be32,
    pub dm: __be32,
    pub n_algs: __be32,
}

#[repr(C, packed)]
pub struct wmfw_adsp2_id_hdr {
    pub fw: wmfw_id_hdr,
    pub zm: __be32,
    pub xm: __be32,
    pub ym: __be32,
    pub n_algs: __be32,
}

#[repr(C, packed)]
pub struct wmfw_halo_id_hdr {
    pub fw: wmfw_v3_id_hdr,
    pub xm_base: __be32,
    pub xm_size: __be32,
    pub ym_base: __be32,
    pub ym_size: __be32,
    pub n_algs: __be32,
}

#[repr(C, packed)]
pub struct wmfw_alg_hdr {
    pub id: __be32,
    pub ver: __be32,
}

#[repr(C, packed)]
pub struct wmfw_adsp1_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub zm: __be32,
    pub dm: __be32,
}

#[repr(C, packed)]
pub struct wmfw_adsp2_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub zm: __be32,
    pub xm: __be32,
    pub ym: __be32,
}

#[repr(C, packed)]
pub struct wmfw_halo_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub xm_base: __be32,
    pub xm_size: __be32,
    pub ym_base: __be32,
    pub ym_size: __be32,
}

#[repr(C, packed)]
pub struct wmfw_adsp_alg_data {
    pub id: __le32,
    pub name: [u8; WMFW_MAX_ALG_NAME],
    pub descr: [u8; WMFW_MAX_ALG_DESCR_NAME],
    pub ncoeff: __le32,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct wmfw_adsp_coeff_data_hdr {
    pub offset: __le16,
    pub r#type: __le16,
    pub size: __le32,
}

#[repr(C, packed)]
pub struct wmfw_adsp_coeff_data {
    pub hdr: wmfw_adsp_coeff_data_hdr,
    pub name: [u8; WMFW_MAX_COEFF_NAME],
    pub descr: [u8; WMFW_MAX_COEFF_DESCR_NAME],
    pub ctl_type: __le16,
    pub flags: __le16,
    pub len: __le32,
    pub data: [u8; 0],
}

#[repr(C)]
pub union wmfw_coeff_hdr_rev_ver {
    pub rev: __be32,
    pub ver: __le32,
}

#[repr(C)]
pub union wmfw_coeff_hdr_core_core_ver {
    pub core: __be32,
    pub core_ver: __le32,
}

#[repr(C, packed)]
pub struct wmfw_coeff_hdr {
    pub magic: [u8; 4],
    pub len: __le32,
    pub rev_ver: wmfw_coeff_hdr_rev_ver,
    pub core_core_ver: wmfw_coeff_hdr_core_core_ver,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct wmfw_coeff_item {
    pub offset: __le16,
    pub r#type: __le16,
    pub id: __le32,
    pub ver: __le32,
    pub offset32: __le32,
    pub len: __le32,
    pub data: [u8; 0],
}

pub const WMFW_ADSP1: u32 = 1;
pub const WMFW_ADSP2: u32 = 2;
pub const WMFW_HALO: u32 = 4;

pub const WMFW_ABSOLUTE: u32 = 0xf0;
pub const WMFW_ALGORITHM_DATA: u32 = 0xf2;
pub const WMFW_METADATA: u32 = 0xfc;
pub const WMFW_NAME_TEXT: u32 = 0xfe;
pub const WMFW_INFO_TEXT: u32 = 0xff;

pub const WMFW_ADSP1_PM: u32 = 2;
pub const WMFW_ADSP1_DM: u32 = 3;
pub const WMFW_ADSP1_ZM: u32 = 4;

pub const WMFW_ADSP2_PM: u32 = 2;
pub const WMFW_ADSP2_ZM: u32 = 4;
pub const WMFW_ADSP2_XM: u32 = 5;
pub const WMFW_ADSP2_YM: u32 = 6;

pub const WMFW_HALO_PM_PACKED: u32 = 0x10;
pub const WMFW_HALO_XM_PACKED: u32 = 0x11;
pub const WMFW_HALO_YM_PACKED: u32 = 0x12;

pub const WMFW_ADSP2_XM_LONG: u32 = 0xf405;
pub const WMFW_ADSP2_YM_LONG: u32 = 0xf406;
pub const WMFW_HALO_XM_PACKED_LONG: u32 = 0xf411;
pub const WMFW_HALO_YM_PACKED_LONG: u32 = 0xf412;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
