/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Original dependency: <linux/types.h>

/**
 * struct sof_abi_hdr - Header for all non IPC ABI data.
 * @magic: Magic number for validation
 *       for IPC3 data: 0x00464F53 ('S', 'O', 'F', '\0')
 *       for IPC4 data: 0x34464F53 ('S', 'O', 'F', '4')
 * @type: module specific parameter
 *      for IPC3: Component specific type
 *      for IPC4: parameter ID (param_id) of the data
 * @size: The size in bytes of the data, excluding this struct
 * @abi: SOF ABI version. The version is valid in scope of the 'magic', IPC3 and
 *      IPC4 ABI version numbers have no relationship.
 * @reserved: Reserved for future use
 * @data: Component data - opaque to core
 *
 * Identifies data type, size and ABI.
 * Used by any bespoke component data structures or binary blobs.
 */
#[repr(C, packed)]
pub struct sof_abi_hdr {
    pub magic: u32,
    pub type_: u32,
    pub size: u32,
    pub abi: u32,
    pub reserved: [u32; 4],
    pub data: [u32; 0],
}

pub const SOF_MANIFEST_DATA_TYPE_NHLT: u32 = 1;

/**
 * struct sof_manifest_tlv - SOF manifest TLV data
 * @type: type of data
 * @size: data size (not including the size of this struct)
 * @data: payload data
 */
#[repr(C)]
pub struct sof_manifest_tlv {
    pub type_: __le32,
    pub size: __le32,
    pub data: [u8; 0],
}

/**
 * struct sof_manifest - SOF topology manifest
 * @abi_major: Major ABI version
 * @abi_minor: Minor ABI version
 * @abi_patch: ABI patch
 * @count: count of tlv items
 * @items: consecutive variable size tlv items
 */
#[repr(C)]
pub struct sof_manifest {
    pub abi_major: __le16,
    pub abi_minor: __le16,
    pub abi_patch: __le16,
    pub count: __le16,
    pub items: [sof_manifest_tlv; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
