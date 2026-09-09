/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
 */

// Dependency intent: the C header includes <linux/types.h>; fixed-width Rust
// integer types are used directly here, with little-endian fields represented
// by their corresponding integer widths.

pub const BOOT_SIGNATURE: u16 = 0xAA55;
pub const EXBOOT_SIGNATURE: u32 = 0xAA550000;
pub const STR_EXFAT: &str = "EXFAT   "; /* size should be 8 */

pub const EXFAT_MAX_FILE_LEN: usize = 255;

pub const VOLUME_DIRTY: u16 = 0x0002;
pub const MEDIA_FAILURE: u16 = 0x0004;

pub const EXFAT_EOF_CLUSTER: u32 = 0xFFFFFFFF;
pub const EXFAT_BAD_CLUSTER: u32 = 0xFFFFFFF7;
pub const EXFAT_FREE_CLUSTER: u32 = 0;
// Cluster 0, 1 are reserved, the first cluster is 2 in the cluster heap.
pub const EXFAT_RESERVED_CLUSTERS: u32 = 2;
pub const EXFAT_FIRST_CLUSTER: u32 = 2;
#[macro_export]
macro_rules! EXFAT_DATA_CLUSTER_COUNT { ($sbi:expr) => { ($sbi).num_clusters - $crate::EXFAT_RESERVED_CLUSTERS }; }
pub const EXFAT_MAX_NUM_CLUSTER: u32 = 0xFFFFFFF5;

// AllocationPossible and NoFatChain field in GeneralSecondaryFlags Field
pub const ALLOC_POSSIBLE: u8 = 0x01;
pub const ALLOC_FAT_CHAIN: u8 = 0x01;
pub const ALLOC_NO_FAT_CHAIN: u8 = 0x03;

pub const DENTRY_SIZE: usize = 32; /* directory entry size */
pub const DENTRY_SIZE_BITS: usize = 5;
/* exFAT allows 8388608(256MB) directory entries */
pub const MAX_EXFAT_DENTRIES: usize = 8388608;

/* dentry types */
pub const EXFAT_UNUSED: u8 = 0x00; /* end of directory */
pub const EXFAT_DELETE: u8 = !0x80u8;
pub const EXFAT_INVAL: u8 = 0x80; /* invalid value */
pub const EXFAT_BITMAP: u8 = 0x81; /* allocation bitmap */
pub const EXFAT_UPCASE: u8 = 0x82; /* upcase table */
pub const EXFAT_VOLUME: u8 = 0x83; /* volume label */
pub const EXFAT_FILE: u8 = 0x85; /* file or dir */
pub const EXFAT_GUID: u8 = 0xA0;
pub const EXFAT_PADDING: u8 = 0xA1;
pub const EXFAT_ACLTAB: u8 = 0xA2;
pub const EXFAT_STREAM: u8 = 0xC0; /* stream entry */
pub const EXFAT_NAME: u8 = 0xC1; /* file name entry */
pub const EXFAT_ACL: u8 = 0xC2; /* stream entry */
pub const EXFAT_VENDOR_EXT: u8 = 0xE0; /* vendor extension entry */
pub const EXFAT_VENDOR_ALLOC: u8 = 0xE1; /* vendor allocation entry */

#[inline]
pub const fn is_exfat_deleted(x: u8) -> bool { x < 0x80 }
#[inline]
pub const fn is_exfat_critical_pri(x: u8) -> bool { x < 0xA0 }
#[inline]
pub const fn is_exfat_benign_pri(x: u8) -> bool { x < 0xC0 }
#[inline]
pub const fn is_exfat_critical_sec(x: u8) -> bool { x < 0xE0 }

/* checksum types */
pub const CS_DIR_ENTRY: u32 = 0;
pub const CS_BOOT_SECTOR: u32 = 1;
pub const CS_DEFAULT: u32 = 2;

/* file attributes */
pub const EXFAT_ATTR_READONLY: u16 = 0x0001;
pub const EXFAT_ATTR_HIDDEN: u16 = 0x0002;
pub const EXFAT_ATTR_SYSTEM: u16 = 0x0004;
pub const EXFAT_ATTR_VOLUME: u16 = 0x0008;
pub const EXFAT_ATTR_SUBDIR: u16 = 0x0010;
pub const EXFAT_ATTR_ARCHIVE: u16 = 0x0020;
pub const EXFAT_ATTR_RWMASK: u16 = EXFAT_ATTR_HIDDEN | EXFAT_ATTR_SYSTEM | EXFAT_ATTR_VOLUME | EXFAT_ATTR_SUBDIR | EXFAT_ATTR_ARCHIVE;

pub const BOOTSEC_JUMP_BOOT_LEN: usize = 3;
pub const BOOTSEC_FS_NAME_LEN: usize = 8;
pub const BOOTSEC_OLDBPB_LEN: usize = 53;
pub const EXFAT_FILE_NAME_LEN: usize = 15;
pub const EXFAT_VOLUME_LABEL_LEN: usize = 11;
pub const EXFAT_MIN_SECT_SIZE_BITS: u8 = 9;
pub const EXFAT_MAX_SECT_SIZE_BITS: u8 = 12;
#[macro_export]
macro_rules! EXFAT_MAX_SECT_PER_CLUS_BITS { ($x:expr) => { 25 - ($x).sect_size_bits }; }

#[repr(C, packed)]
pub struct boot_sector {
    pub jmp_boot: [u8; BOOTSEC_JUMP_BOOT_LEN], pub fs_name: [u8; BOOTSEC_FS_NAME_LEN], pub must_be_zero: [u8; BOOTSEC_OLDBPB_LEN],
    pub partition_offset: u64, pub vol_length: u64, pub fat_offset: u32, pub fat_length: u32, pub clu_offset: u32, pub clu_count: u32,
    pub root_cluster: u32, pub vol_serial: u32, pub fs_revision: [u8; 2], pub vol_flags: u16, pub sect_size_bits: u8,
    pub sect_per_clus_bits: u8, pub num_fats: u8, pub drv_sel: u8, pub percent_in_use: u8, pub reserved: [u8; 7],
    pub boot_code: [u8; 390], pub signature: u16,
}

#[repr(C, packed)]
pub struct exfat_file { pub num_ext: u8, pub checksum: u16, pub attr: u16, pub reserved1: u16, pub create_time: u16, pub create_date: u16, pub modify_time: u16, pub modify_date: u16, pub access_time: u16, pub access_date: u16, pub create_time_cs: u8, pub modify_time_cs: u8, pub create_tz: u8, pub modify_tz: u8, pub access_tz: u8, pub reserved2: [u8; 7] }
#[repr(C, packed)]
pub struct exfat_stream { pub flags: u8, pub reserved1: u8, pub name_len: u8, pub name_hash: u16, pub reserved2: u16, pub valid_size: u64, pub reserved3: u32, pub start_clu: u32, pub size: u64 }
#[repr(C, packed)]
pub struct exfat_name { pub flags: u8, pub unicode_0_14: [u16; EXFAT_FILE_NAME_LEN] }
#[repr(C, packed)]
pub struct exfat_bitmap { pub flags: u8, pub reserved: [u8; 18], pub start_clu: u32, pub size: u64 }
#[repr(C, packed)]
pub struct exfat_upcase { pub reserved1: [u8; 3], pub checksum: u32, pub reserved2: [u8; 12], pub start_clu: u32, pub size: u64 }
#[repr(C, packed)]
pub struct exfat_volume_label { pub char_count: u8, pub volume_label: [u16; EXFAT_VOLUME_LABEL_LEN], pub reserved: [u8; 8] }
#[repr(C, packed)]
pub struct exfat_vendor_ext { pub flags: u8, pub vendor_guid: [u8; 16], pub vendor_defined: [u8; 14] }
#[repr(C, packed)]
pub struct exfat_vendor_alloc { pub flags: u8, pub vendor_guid: [u8; 16], pub vendor_defined: [u8; 2], pub start_clu: u32, pub size: u64 }
#[repr(C, packed)]
pub struct exfat_generic_secondary { pub flags: u8, pub custom_defined: [u8; 18], pub start_clu: u32, pub size: u64 }

#[repr(C)]
pub union exfat_dentry_union { pub file: exfat_file, pub stream: exfat_stream, pub name: exfat_name, pub bitmap: exfat_bitmap, pub upcase: exfat_upcase, pub volume_label: exfat_volume_label, pub vendor_ext: exfat_vendor_ext, pub vendor_alloc: exfat_vendor_alloc, pub generic_secondary: exfat_generic_secondary }
#[repr(C, packed)]
pub struct exfat_dentry { pub r#type: u8, pub dentry: exfat_dentry_union }

pub const EXFAT_TZ_VALID: u8 = 1 << 7;
/* Jan 1 GMT 00:00:00 1980 */
pub const EXFAT_MIN_TIMESTAMP_SECS: i64 = 315532800;
/* Dec 31 GMT 23:59:59 2107 */
pub const EXFAT_MAX_TIMESTAMP_SECS: i64 = 4354819199;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
