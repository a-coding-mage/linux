/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* Translated from the Linux Btrfs on-disk tree header.  Types supplied by
 * linux/btrfs.h and linux/types.h are intentionally external dependencies. */

pub const BTRFS_MAGIC: u64 = 0x4D5F53665248425F;
pub const BTRFS_MAX_LEVEL: u32 = 8;
pub const BTRFS_NAME_LEN: u32 = 255;
pub const BTRFS_LINK_MAX: u32 = 65535;
pub const BTRFS_ROOT_TREE_OBJECTID: u64 = 1;
pub const BTRFS_EXTENT_TREE_OBJECTID: u64 = 2;
pub const BTRFS_CHUNK_TREE_OBJECTID: u64 = 3;
pub const BTRFS_DEV_TREE_OBJECTID: u64 = 4;
pub const BTRFS_FS_TREE_OBJECTID: u64 = 5;
pub const BTRFS_ROOT_TREE_DIR_OBJECTID: u64 = 6;
pub const BTRFS_CSUM_TREE_OBJECTID: u64 = 7;
pub const BTRFS_QUOTA_TREE_OBJECTID: u64 = 8;
pub const BTRFS_UUID_TREE_OBJECTID: u64 = 9;
pub const BTRFS_FREE_SPACE_TREE_OBJECTID: u64 = 10;
pub const BTRFS_BLOCK_GROUP_TREE_OBJECTID: u64 = 11;
pub const BTRFS_RAID_STRIPE_TREE_OBJECTID: u64 = 12;
pub const BTRFS_REMAP_TREE_OBJECTID: u64 = 13;
pub const BTRFS_DEV_STATS_OBJECTID: u64 = 0;
pub const BTRFS_BALANCE_OBJECTID: u64 = (-4i64) as u64;
pub const BTRFS_ORPHAN_OBJECTID: u64 = (-5i64) as u64;
pub const BTRFS_TREE_LOG_OBJECTID: u64 = (-6i64) as u64;
pub const BTRFS_TREE_LOG_FIXUP_OBJECTID: u64 = (-7i64) as u64;
pub const BTRFS_TREE_RELOC_OBJECTID: u64 = (-8i64) as u64;
pub const BTRFS_DATA_RELOC_TREE_OBJECTID: u64 = (-9i64) as u64;
pub const BTRFS_EXTENT_CSUM_OBJECTID: u64 = (-10i64) as u64;
pub const BTRFS_FREE_SPACE_OBJECTID: u64 = (-11i64) as u64;
pub const BTRFS_FREE_INO_OBJECTID: u64 = (-12i64) as u64;
pub const BTRFS_MULTIPLE_OBJECTIDS: u64 = (-255i64) as u64;
pub const BTRFS_FIRST_FREE_OBJECTID: u64 = 256;
pub const BTRFS_LAST_FREE_OBJECTID: u64 = (-256i64) as u64;
pub const BTRFS_FIRST_CHUNK_TREE_OBJECTID: u64 = 256;
pub const BTRFS_DEV_ITEMS_OBJECTID: u64 = 1;
pub const BTRFS_BTREE_INODE_OBJECTID: u32 = 1;
pub const BTRFS_EMPTY_SUBVOL_DIR_OBJECTID: u32 = 2;
pub const BTRFS_DEV_REPLACE_DEVID: u64 = 0;

pub const BTRFS_INODE_ITEM_KEY: u32 = 1;
pub const BTRFS_INODE_REF_KEY: u32 = 12;
pub const BTRFS_INODE_EXTREF_KEY: u32 = 13;
pub const BTRFS_XATTR_ITEM_KEY: u32 = 24;
pub const BTRFS_VERITY_DESC_ITEM_KEY: u32 = 36;
pub const BTRFS_VERITY_MERKLE_ITEM_KEY: u32 = 37;
pub const BTRFS_ORPHAN_ITEM_KEY: u32 = 48;
pub const BTRFS_DIR_LOG_ITEM_KEY: u32 = 60;
pub const BTRFS_DIR_LOG_INDEX_KEY: u32 = 72;
pub const BTRFS_DIR_ITEM_KEY: u32 = 84;
pub const BTRFS_DIR_INDEX_KEY: u32 = 96;
pub const BTRFS_EXTENT_DATA_KEY: u32 = 108;
pub const BTRFS_EXTENT_CSUM_KEY: u32 = 128;
pub const BTRFS_ROOT_ITEM_KEY: u32 = 132;
pub const BTRFS_ROOT_BACKREF_KEY: u32 = 144;
pub const BTRFS_ROOT_REF_KEY: u32 = 156;
pub const BTRFS_EXTENT_ITEM_KEY: u32 = 168;
pub const BTRFS_METADATA_ITEM_KEY: u32 = 169;
pub const BTRFS_EXTENT_OWNER_REF_KEY: u32 = 172;
pub const BTRFS_TREE_BLOCK_REF_KEY: u32 = 176;
pub const BTRFS_EXTENT_DATA_REF_KEY: u32 = 178;
pub const BTRFS_SHARED_BLOCK_REF_KEY: u32 = 182;
pub const BTRFS_SHARED_DATA_REF_KEY: u32 = 184;
pub const BTRFS_BLOCK_GROUP_ITEM_KEY: u32 = 192;
pub const BTRFS_FREE_SPACE_INFO_KEY: u32 = 198;
pub const BTRFS_FREE_SPACE_EXTENT_KEY: u32 = 199;
pub const BTRFS_FREE_SPACE_BITMAP_KEY: u32 = 200;
pub const BTRFS_DEV_EXTENT_KEY: u32 = 204;
pub const BTRFS_DEV_ITEM_KEY: u32 = 216;
pub const BTRFS_CHUNK_ITEM_KEY: u32 = 228;
pub const BTRFS_RAID_STRIPE_KEY: u32 = 230;
pub const BTRFS_IDENTITY_REMAP_KEY: u32 = 234;
pub const BTRFS_REMAP_KEY: u32 = 235;
pub const BTRFS_REMAP_BACKREF_KEY: u32 = 236;
pub const BTRFS_QGROUP_STATUS_KEY: u32 = 240;
pub const BTRFS_QGROUP_INFO_KEY: u32 = 242;
pub const BTRFS_QGROUP_LIMIT_KEY: u32 = 244;
pub const BTRFS_QGROUP_RELATION_KEY: u32 = 246;
pub const BTRFS_BALANCE_ITEM_KEY: u32 = 248;
pub const BTRFS_TEMPORARY_ITEM_KEY: u32 = 248;
pub const BTRFS_DEV_STATS_KEY: u32 = 249;
pub const BTRFS_PERSISTENT_ITEM_KEY: u32 = 249;
pub const BTRFS_DEV_REPLACE_KEY: u32 = 250;
pub const BTRFS_UUID_KEY_SUBVOL: u32 = 251;
pub const BTRFS_UUID_KEY_RECEIVED_SUBVOL: u32 = 252;
pub const BTRFS_STRING_ITEM_KEY: u32 = 253;
pub const BTRFS_MAX_METADATA_BLOCKSIZE: usize = 65536;
pub const BTRFS_CSUM_SIZE: usize = 32;

#[repr(u32)]
pub enum btrfs_csum_type { BTRFS_CSUM_TYPE_CRC32 = 0, BTRFS_CSUM_TYPE_XXHASH = 1, BTRFS_CSUM_TYPE_SHA256 = 2, BTRFS_CSUM_TYPE_BLAKE2 = 3 }

pub const BTRFS_FT_UNKNOWN: u8 = 0; pub const BTRFS_FT_REG_FILE: u8 = 1; pub const BTRFS_FT_DIR: u8 = 2; pub const BTRFS_FT_CHRDEV: u8 = 3; pub const BTRFS_FT_BLKDEV: u8 = 4; pub const BTRFS_FT_FIFO: u8 = 5; pub const BTRFS_FT_SOCK: u8 = 6; pub const BTRFS_FT_SYMLINK: u8 = 7; pub const BTRFS_FT_XATTR: u8 = 8; pub const BTRFS_FT_MAX: u8 = 9; pub const BTRFS_FT_ENCRYPTED: u8 = 0x80;
#[inline] pub fn btrfs_dir_flags_to_ftype(flags: u8) -> u8 { flags & !BTRFS_FT_ENCRYPTED }

pub const BTRFS_INODE_NODATASUM: u32 = 1<<0; pub const BTRFS_INODE_NODATACOW: u32 = 1<<1; pub const BTRFS_INODE_READONLY: u32 = 1<<2; pub const BTRFS_INODE_NOCOMPRESS: u32 = 1<<3; pub const BTRFS_INODE_PREALLOC: u32 = 1<<4; pub const BTRFS_INODE_SYNC: u32 = 1<<5; pub const BTRFS_INODE_IMMUTABLE: u32 = 1<<6; pub const BTRFS_INODE_APPEND: u32 = 1<<7; pub const BTRFS_INODE_NODUMP: u32 = 1<<8; pub const BTRFS_INODE_NOATIME: u32 = 1<<9; pub const BTRFS_INODE_DIRSYNC: u32 = 1<<10; pub const BTRFS_INODE_COMPRESS: u32 = 1<<11; pub const BTRFS_INODE_ROOT_ITEM_INIT: u32 = 1<<31;
pub const BTRFS_INODE_FLAG_MASK: u32 = 0x80000fff; pub const BTRFS_INODE_RO_VERITY: u32 = 1; pub const BTRFS_INODE_RO_FLAG_MASK: u32 = 1;

#[repr(C, packed)] pub struct btrfs_disk_key { pub objectid: __le64, pub type_: __u8, pub offset: __le64 }
#[repr(C, packed)] pub struct btrfs_key { pub objectid: __u64, pub type_: __u8, pub offset: __u64 }
#[repr(C, packed)] pub struct btrfs_header { pub csum: [__u8; BTRFS_CSUM_SIZE], pub fsid: [__u8; BTRFS_FSID_SIZE], pub bytenr: __le64, pub flags: __le64, pub chunk_tree_uuid: [__u8; BTRFS_UUID_SIZE], pub generation: __le64, pub owner: __le64, pub nritems: __le32, pub level: __u8 }
pub const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048; pub const BTRFS_NUM_BACKUP_ROOTS: usize = 4;
#[repr(C, packed)] pub struct btrfs_root_backup { pub tree_root: __le64, pub tree_root_gen: __le64, pub chunk_root: __le64, pub chunk_root_gen: __le64, pub extent_root: __le64, pub extent_root_gen: __le64, pub fs_root: __le64, pub fs_root_gen: __le64, pub dev_root: __le64, pub dev_root_gen: __le64, pub csum_root: __le64, pub csum_root_gen: __le64, pub total_bytes: __le64, pub bytes_used: __le64, pub num_devices: __le64, pub unused_64: [__le64;4], pub tree_root_level: __u8, pub chunk_root_level: __u8, pub extent_root_level: __u8, pub fs_root_level: __u8, pub dev_root_level: __u8, pub csum_root_level: __u8, pub unused_8: [__u8;10] }
#[repr(C, packed)] pub struct btrfs_item { pub key: btrfs_disk_key, pub offset: __le32, pub size: __le32 }
#[repr(C, packed)] pub struct btrfs_leaf { pub header: btrfs_header, pub items: [btrfs_item;0] }
#[repr(C, packed)] pub struct btrfs_key_ptr { pub key: btrfs_disk_key, pub blockptr: __le64, pub generation: __le64 }
#[repr(C, packed)] pub struct btrfs_node { pub header: btrfs_header, pub ptrs: [btrfs_key_ptr;0] }

/* The remainder of the header consists of packed on-disk records. */
#[repr(C, packed)] pub struct btrfs_dev_item { pub devid: __le64, pub total_bytes: __le64, pub bytes_used: __le64, pub io_align: __le32, pub io_width: __le32, pub sector_size: __le32, pub type_: __le64, pub generation: __le64, pub start_offset: __le64, pub dev_group: __le32, pub seek_speed: __u8, pub bandwidth: __u8, pub uuid: [__u8;BTRFS_UUID_SIZE], pub fsid: [__u8;BTRFS_UUID_SIZE] }
#[repr(C, packed)] pub struct btrfs_stripe { pub devid: __le64, pub offset: __le64, pub dev_uuid: [__u8;BTRFS_UUID_SIZE] }
#[repr(C, packed)] pub struct btrfs_chunk { pub length: __le64, pub owner: __le64, pub stripe_len: __le64, pub type_: __le64, pub io_align: __le32, pub io_width: __le32, pub sector_size: __le32, pub num_stripes: __le16, pub sub_stripes: __le16, pub stripe: btrfs_stripe }
#[repr(C, packed)] pub struct btrfs_super_block { pub csum: [__u8;BTRFS_CSUM_SIZE], pub fsid: [__u8;BTRFS_FSID_SIZE], pub bytenr: __le64, pub flags: __le64, pub magic: __le64, pub generation: __le64, pub root: __le64, pub chunk_root: __le64, pub log_root: __le64, pub unused_log_root_transid: __le64, pub total_bytes: __le64, pub bytes_used: __le64, pub root_dir_objectid: __le64, pub num_devices: __le64, pub sectorsize: __le32, pub nodesize: __le32, pub unused_leafsize: __le32, pub stripesize: __le32, pub sys_chunk_array_size: __le32, pub chunk_root_generation: __le64, pub compat_flags: __le64, pub compat_ro_flags: __le64, pub incompat_flags: __le64, pub csum_type: __le16, pub root_level: __u8, pub chunk_root_level: __u8, pub log_root_level: __u8, pub dev_item: btrfs_dev_item, pub label: [c_char;BTRFS_LABEL_SIZE], pub cache_generation: __le64, pub uuid_tree_generation: __le64, pub metadata_uuid: [__u8;BTRFS_FSID_SIZE], pub nr_global_roots: __u64, pub remap_root: __le64, pub remap_root_generation: __le64, pub remap_root_level: __u8, pub reserved: [__u8;199], pub sys_chunk_array: [__u8;BTRFS_SYSTEM_CHUNK_ARRAY_SIZE], pub super_roots: [btrfs_root_backup;BTRFS_NUM_BACKUP_ROOTS], pub padding: [__u8;565] }

#[repr(C, packed)] pub struct btrfs_free_space_entry { pub offset: __le64, pub bytes: __le64, pub type_: __u8 }
#[repr(C, packed)] pub struct btrfs_free_space_header { pub location: btrfs_disk_key, pub generation: __le64, pub num_entries: __le64, pub num_bitmaps: __le64 }
#[repr(C, packed)] pub struct btrfs_raid_stride { pub devid: __le64, pub physical: __le64 }
#[repr(C, packed)] pub struct btrfs_stripe_extent { pub strides: [btrfs_raid_stride;0] }

pub const BTRFS_HEADER_FLAG_WRITTEN:u64=1; pub const BTRFS_HEADER_FLAG_RELOC:u64=2; pub const BTRFS_SUPER_FLAG_ERROR:u64=1<<2; pub const BTRFS_SUPER_FLAG_SEEDING:u64=1<<32; pub const BTRFS_SUPER_FLAG_METADUMP:u64=1<<33; pub const BTRFS_SUPER_FLAG_METADUMP_V2:u64=1<<34; pub const BTRFS_SUPER_FLAG_CHANGING_FSID:u64=1<<35; pub const BTRFS_SUPER_FLAG_CHANGING_FSID_V2:u64=1<<36; pub const BTRFS_SUPER_FLAG_CHANGING_BG_TREE:u64=1<<38; pub const BTRFS_SUPER_FLAG_CHANGING_DATA_CSUM:u64=1<<39; pub const BTRFS_SUPER_FLAG_CHANGING_META_CSUM:u64=1<<40;

#[repr(C, packed)] pub struct btrfs_extent_item { pub refs:__le64,pub generation:__le64,pub flags:__le64 } #[repr(C, packed)] pub struct btrfs_extent_item_v0 { pub refs:__le32 }
pub const BTRFS_EXTENT_FLAG_DATA:u64=1; pub const BTRFS_EXTENT_FLAG_TREE_BLOCK:u64=2; pub const BTRFS_BLOCK_FLAG_FULL_BACKREF:u64=1<<8; pub const BTRFS_BACKREF_REV_MAX:u32=256; pub const BTRFS_BACKREF_REV_SHIFT:u32=56; pub const BTRFS_BACKREF_REV_MASK:u64=255u64<<56; pub const BTRFS_OLD_BACKREF_REV:u32=0; pub const BTRFS_MIXED_BACKREF_REV:u32=1; pub const BTRFS_EXTENT_FLAG_SUPER:u64=1<<48;
#[repr(C, packed)] pub struct btrfs_tree_block_info { pub key:btrfs_disk_key,pub level:__u8 } #[repr(C, packed)] pub struct btrfs_extent_data_ref { pub root:__le64,pub objectid:__le64,pub offset:__le64,pub count:__le32 } #[repr(C, packed)] pub struct btrfs_shared_data_ref { pub count:__le32 } #[repr(C, packed)] pub struct btrfs_extent_owner_ref { pub root_id:__le64 } #[repr(C, packed)] pub struct btrfs_extent_inline_ref { pub type_:__u8,pub offset:__le64 }
#[repr(C, packed)] pub struct btrfs_dev_extent { pub chunk_tree:__le64,pub chunk_objectid:__le64,pub chunk_offset:__le64,pub length:__le64,pub chunk_tree_uuid:[__u8;BTRFS_UUID_SIZE] }
#[repr(C, packed)] pub struct btrfs_inode_ref { pub index:__le64,pub name_len:__le16 } #[repr(C, packed)] pub struct btrfs_inode_extref { pub parent_objectid:__le64,pub index:__le64,pub name_len:__le16,pub name:[__u8;0] } #[repr(C, packed)] pub struct btrfs_timespec { pub sec:__le64,pub nsec:__le32 }
#[repr(C, packed)] pub struct btrfs_inode_item { pub generation:__le64,pub transid:__le64,pub size:__le64,pub nbytes:__le64,pub block_group:__le64,pub nlink:__le32,pub uid:__le32,pub gid:__le32,pub mode:__le32,pub rdev:__le64,pub flags:__le64,pub sequence:__le64,pub reserved:[__le64;4],pub atime:btrfs_timespec,pub ctime:btrfs_timespec,pub mtime:btrfs_timespec,pub otime:btrfs_timespec }
#[repr(C, packed)] pub struct btrfs_dir_log_item { pub end:__le64 } #[repr(C, packed)] pub struct btrfs_dir_item { pub location:btrfs_disk_key,pub transid:__le64,pub data_len:__le16,pub name_len:__le16,pub type_:__u8 }
pub const BTRFS_ROOT_SUBVOL_RDONLY:u64=1; pub const BTRFS_ROOT_SUBVOL_DEAD:u64=1<<48;
#[repr(C, packed)] pub struct btrfs_root_item { pub inode:btrfs_inode_item,pub generation:__le64,pub root_dirid:__le64,pub bytenr:__le64,pub byte_limit:__le64,pub bytes_used:__le64,pub last_snapshot:__le64,pub flags:__le64,pub refs:__le32,pub drop_progress:btrfs_disk_key,pub drop_level:__u8,pub level:__u8,pub generation_v2:__le64,pub uuid:[__u8;BTRFS_UUID_SIZE],pub parent_uuid:[__u8;BTRFS_UUID_SIZE],pub received_uuid:[__u8;BTRFS_UUID_SIZE],pub ctransid:__le64,pub otransid:__le64,pub stransid:__le64,pub rtransid:__le64,pub ctime:btrfs_timespec,pub otime:btrfs_timespec,pub stime:btrfs_timespec,pub rtime:btrfs_timespec,pub reserved:[__le64;8] }
#[inline] pub fn btrfs_legacy_root_item_size() -> usize { core::mem::offset_of!(btrfs_root_item, generation_v2) }
#[repr(C, packed)] pub struct btrfs_root_ref { pub dirid:__le64,pub sequence:__le64,pub name_len:__le16 }
#[repr(C, packed)] pub struct btrfs_disk_balance_args { pub profiles:__le64,pub usage:__le64,pub devid:__le64,pub pstart:__le64,pub pend:__le64,pub vstart:__le64,pub vend:__le64,pub target:__le64,pub flags:__le64,pub limit:__le64,pub stripes_min:__le32,pub stripes_max:__le32,pub unused:[__le64;6] }
#[repr(C, packed)] pub struct btrfs_balance_item { pub flags:__le64,pub data:btrfs_disk_balance_args,pub meta:btrfs_disk_balance_args,pub sys:btrfs_disk_balance_args,pub unused:[__le64;4] }
pub const BTRFS_FILE_EXTENT_INLINE:u32=0; pub const BTRFS_FILE_EXTENT_REG:u32=1; pub const BTRFS_FILE_EXTENT_PREALLOC:u32=2; pub const BTRFS_NR_FILE_EXTENT_TYPES:u32=3;
#[repr(C, packed)] pub struct btrfs_file_extent_item { pub generation:__le64,pub ram_bytes:__le64,pub compression:__u8,pub encryption:__u8,pub other_encoding:__le16,pub type_:__u8,pub disk_bytenr:__le64,pub disk_num_bytes:__le64,pub offset:__le64,pub num_bytes:__le64 }
#[repr(C, packed)] pub struct btrfs_csum_item { pub csum:__u8 } #[repr(C, packed)] pub struct btrfs_dev_stats_item { pub values:[__le64;BTRFS_DEV_STAT_VALUES_MAX] }
pub const BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_ALWAYS:u32=0; pub const BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_AVOID:u32=1;
#[repr(C, packed)] pub struct btrfs_dev_replace_item { pub src_devid:__le64,pub cursor_left:__le64,pub cursor_right:__le64,pub cont_reading_from_srcdev_mode:__le64,pub replace_state:__le64,pub time_started:__le64,pub time_stopped:__le64,pub num_write_errors:__le64,pub num_uncorrectable_read_errors:__le64 }
pub const BTRFS_BLOCK_GROUP_DATA:u64=1; pub const BTRFS_BLOCK_GROUP_SYSTEM:u64=1<<1; pub const BTRFS_BLOCK_GROUP_METADATA:u64=1<<2; pub const BTRFS_BLOCK_GROUP_RAID0:u64=1<<3; pub const BTRFS_BLOCK_GROUP_RAID1:u64=1<<4; pub const BTRFS_BLOCK_GROUP_DUP:u64=1<<5; pub const BTRFS_BLOCK_GROUP_RAID10:u64=1<<6; pub const BTRFS_BLOCK_GROUP_RAID5:u64=1<<7; pub const BTRFS_BLOCK_GROUP_RAID6:u64=1<<8; pub const BTRFS_BLOCK_GROUP_RAID1C3:u64=1<<9; pub const BTRFS_BLOCK_GROUP_RAID1C4:u64=1<<10; pub const BTRFS_BLOCK_GROUP_REMAPPED:u64=1<<11; pub const BTRFS_BLOCK_GROUP_METADATA_REMAP:u64=1<<12;
pub const BTRFS_AVAIL_ALLOC_BIT_SINGLE:u64=1<<48; pub const BTRFS_SPACE_INFO_GLOBAL_RSV:u64=1<<49;
pub const BTRFS_BLOCK_GROUP_PROFILE_MASK:u64=(1<<3)|(1<<4)|(1<<9)|(1<<10)|(1<<7)|(1<<8)|(1<<5)|(1<<6);
pub const BTRFS_BLOCK_GROUP_TYPE_MASK:u64=1|(1<<1)|(1<<2)|(1<<12);
pub const BTRFS_BLOCK_GROUP_RAID56_MASK:u64=(1<<7)|(1<<8); pub const BTRFS_BLOCK_GROUP_RAID1_MASK:u64=(1<<4)|(1<<9)|(1<<10);
pub const BTRFS_EXTENDED_PROFILE_MASK:u64=BTRFS_BLOCK_GROUP_PROFILE_MASK|BTRFS_AVAIL_ALLOC_BIT_SINGLE;
#[inline] pub fn chunk_to_extended(mut flags:u64)->u64 { if flags & BTRFS_EXTENDED_PROFILE_MASK == 0 { flags |= BTRFS_AVAIL_ALLOC_BIT_SINGLE; } flags } #[inline] pub fn extended_to_chunk(flags:u64)->u64 { flags & !BTRFS_AVAIL_ALLOC_BIT_SINGLE }
#[repr(C, packed)] pub struct btrfs_block_group_item { pub used:__le64,pub chunk_objectid:__le64,pub flags:__le64 } #[repr(C, packed)] pub struct btrfs_block_group_item_v2 { pub used:__le64,pub chunk_objectid:__le64,pub flags:__le64,pub remap_bytes:__le64,pub identity_remap_count:__le32 } #[repr(C, packed)] pub struct btrfs_free_space_info { pub extent_count:__le32,pub flags:__le32 }
pub const BTRFS_FREE_SPACE_EXTENT:u32=1; pub const BTRFS_FREE_SPACE_BITMAP:u32=2; pub const BTRFS_FREE_SPACE_USING_BITMAPS:u32=1; pub const BTRFS_FREE_SPACE_FLAGS_MASK:u32=1; pub const BTRFS_QGROUP_LEVEL_SHIFT:u32=48; #[inline] pub fn btrfs_qgroup_level(qgroupid:__u64)->__u16 {(qgroupid>>BTRFS_QGROUP_LEVEL_SHIFT) as __u16}
pub const BTRFS_QGROUP_STATUS_FLAG_ON:u64=1; pub const BTRFS_QGROUP_STATUS_FLAG_RESCAN:u64=2; pub const BTRFS_QGROUP_STATUS_FLAG_INCONSISTENT:u64=4; pub const BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE:u64=8; pub const BTRFS_QGROUP_STATUS_VERSION:u32=1;
#[repr(C, packed)] pub struct btrfs_qgroup_status_item { pub version:__le64,pub generation:__le64,pub flags:__le64,pub rescan:__le64,pub enable_gen:__le64 } #[repr(C, packed)] pub struct btrfs_qgroup_info_item { pub generation:__le64,pub rfer:__le64,pub rfer_cmpr:__le64,pub excl:__le64,pub excl_cmpr:__le64 } #[repr(C, packed)] pub struct btrfs_qgroup_limit_item { pub flags:__le64,pub max_rfer:__le64,pub max_excl:__le64,pub rsv_rfer:__le64,pub rsv_excl:__le64 }
#[repr(C, packed)] pub struct btrfs_verity_descriptor_item { pub size:__le64,pub reserved:[__le64;2],pub encryption:__u8 } #[repr(C, packed)] pub struct btrfs_remap_item { pub address:__le64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
