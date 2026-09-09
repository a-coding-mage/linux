/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2022 Intel Corporation
 */

/*
 * Extended manifest is a place to store metadata about firmware, known during
 * compilation time - for example firmware version or used compiler.
 * Given information are read on host side before firmware startup.
 * This part of output binary is not signed.
 */

// Dependency supplied by the surrounding translation unit: guid_t.

/* In ASCII  $AE1 */
pub const SOF_EXT_MAN4_MAGIC_NUMBER: u32 = 0x31454124;

pub const MAX_MODULE_NAME_LEN: usize = 8;
pub const MAX_FW_BINARY_NAME: usize = 8;
pub const DEFAULT_HASH_SHA256_LEN: usize = 32;
pub const SOF_MAN4_FW_HDR_OFFSET: usize = 0x2000;
pub const SOF_MAN4_FW_HDR_OFFSET_CAVS_1_5: usize = 0x284;

/*********************************************************************
 *	extended manifest		(struct sof_ext_manifest4_hdr)
 *-------------------
 *	css_manifest hdr
 *-------------------
 *	offset reserved for future
 *-------------------
 *	fw_hdr				(struct sof_man4_fw_binary_header)
 *-------------------
 *	module_entry[0]			(struct sof_man4_module)
 *-------------------
 *	module_entry[1]
 *-------------------
 *	...
 *-------------------
 *	module_entry[n]
 *-------------------
 *	module_config[0]		(struct sof_man4_module_config)
 *-------------------
 *	module_config[1]
 *-------------------
 *	...
 *-------------------
 *	module_config[m]
 *-------------------
 *	FW content
 *-------------------
 *********************************************************************/

#[repr(C, packed)]
pub struct sof_ext_manifest4_hdr {
    pub id: u32,
    pub len: u32, /* length of extension manifest */
    pub version_major: u16, /* header version */
    pub version_minor: u16,
    pub num_module_entries: u32,
}

#[repr(C, packed)]
pub struct sof_man4_fw_binary_header {
    /* This part must be unchanged to be backward compatible with SPT-LP ROM */
    pub id: u32,
    pub len: u32, /* sizeof(sof_man4_fw_binary_header) in bytes */
    pub name: [u8; MAX_FW_BINARY_NAME],
    pub preload_page_count: u32, /* number of pages of preloaded image */
    pub fw_image_flags: u32,
    pub feature_mask: u32,
    pub major_version: u16, /* Firmware version */
    pub minor_version: u16,
    pub hotfix_version: u16,
    pub build_version: u16,
    pub num_module_entries: u32,

    /* This part may change to contain any additional data for BaseFw that is skipped by ROM */
    pub hw_buf_base_addr: u32,
    pub hw_buf_length: u32,
    pub load_offset: u32, /* This value is used by ROM */
}

#[repr(C, packed)]
pub struct sof_man4_segment_desc {
    pub flags: u32,
    pub v_base_addr: u32,
    pub file_offset: u32,
}

#[repr(C, packed)]
pub struct sof_man4_module {
    pub id: u32,
    pub name: [u8; MAX_MODULE_NAME_LEN],
    pub uuid: guid_t,
    pub r#type: u32,
    pub hash: [u8; DEFAULT_HASH_SHA256_LEN],
    pub entry_point: u32,
    pub cfg_offset: u16,
    pub cfg_count: u16,
    pub affinity_mask: u32,
    pub instance_max_count: u16,
    pub instance_stack_size: u16,
    pub segments: [sof_man4_segment_desc; 3],
}

#[repr(C, packed)]
pub struct sof_man4_module_config {
    pub par: [u32; 4], /* module parameters */
    pub is_bytes: u32, /* actual size of instance .bss (bytes) */
    pub cps: u32, /* cycles per second */
    pub ibs: u32, /* input buffer size (bytes) */
    pub obs: u32, /* output buffer size (bytes) */
    pub module_flags: u32, /* flags, reserved for future use */
    pub cpc: u32, /* cycles per single run */
    pub obls: u32, /* output block size, reserved for future use */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
