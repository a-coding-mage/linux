/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  fs/partitions/mac.h
 */

pub const MAC_PARTITION_MAGIC: u16 = 0x504d;

/* type field value for A/UX or other Unix partitions */
pub const APPLE_AUX_TYPE: &str = "Apple_UNIX_SVR2";

#[repr(C)]
pub struct mac_partition {
    pub signature: __be16,   /* expected to be MAC_PARTITION_MAGIC */
    pub res1: __be16,
    pub map_count: __be32,   /* # blocks in partition map */
    pub start_block: __be32, /* absolute starting block # of partition */
    pub block_count: __be32, /* number of blocks in partition */
    pub name: [core::ffi::c_char; 32], /* partition name */
    pub r#type: [core::ffi::c_char; 32], /* string type description */
    pub data_start: __be32, /* rel block # of first data block */
    pub data_count: __be32, /* number of data blocks */
    pub status: __be32,     /* partition status bits */
    pub boot_start: __be32,
    pub boot_size: __be32,
    pub boot_load: __be32,
    pub boot_load2: __be32,
    pub boot_entry: __be32,
    pub boot_entry2: __be32,
    pub boot_cksum: __be32,
    pub processor: [core::ffi::c_char; 16], /* identifies ISA of boot */
    /* there is more stuff after this that we don't need */
}

pub const MAC_STATUS_BOOTABLE: u32 = 8; /* partition is bootable */

pub const MAC_DRIVER_MAGIC: u16 = 0x4552;

/* Driver descriptor structure, in block 0 */
#[repr(C)]
pub struct mac_driver_desc {
    pub signature: __be16, /* expected to be MAC_DRIVER_MAGIC */
    pub block_size: __be16,
    pub block_count: __be32,
    /* ... more stuff */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
