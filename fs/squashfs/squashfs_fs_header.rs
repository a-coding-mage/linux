/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from squashfs_fs.h. */

use core::mem::ManuallyDrop;

pub const SQUASHFS_CACHED_FRAGMENTS: usize = CONFIG_SQUASHFS_FRAGMENT_CACHE_SIZE;
pub const SQUASHFS_MAJOR: u32 = 4;
pub const SQUASHFS_MINOR: u32 = 0;
pub const SQUASHFS_START: u32 = 0;
pub const SQUASHFS_METADATA_SIZE: usize = 8192;
pub const SQUASHFS_BLOCK_OFFSET: usize = 2;
// CONFIG_SQUASHFS_4K_DEVBLK_SIZE selects the build-time value.
#[cfg(CONFIG_SQUASHFS_4K_DEVBLK_SIZE)]
pub const SQUASHFS_DEVBLK_SIZE: usize = 4096;
#[cfg(not(CONFIG_SQUASHFS_4K_DEVBLK_SIZE))]
pub const SQUASHFS_DEVBLK_SIZE: usize = 1024;
pub const SQUASHFS_FILE_MAX_SIZE: usize = 1048576;
pub const SQUASHFS_FILE_MAX_LOG: usize = 20;
pub const SQUASHFS_NAME_LEN: usize = 256;
pub const SQUASHFS_DIR_COUNT: usize = 256;
pub const SQUASHFS_INVALID_FRAG: u32 = 0xffffffff;
pub const SQUASHFS_INVALID_XATTR: u32 = 0xffffffff;
pub const SQUASHFS_INVALID_BLK: i64 = -1;

pub const SQUASHFS_NOI: u32 = 0;
pub const SQUASHFS_NOD: u32 = 1;
pub const SQUASHFS_NOF: u32 = 3;
pub const SQUASHFS_NO_FRAG: u32 = 4;
pub const SQUASHFS_ALWAYS_FRAG: u32 = 5;
pub const SQUASHFS_DUPLICATE: u32 = 6;
pub const SQUASHFS_EXPORT: u32 = 7;
pub const SQUASHFS_COMP_OPT: u32 = 10;

#[inline] pub const fn SQUASHFS_BIT(flag: u32, bit: u32) -> u32 { (flag >> bit) & 1 }
#[inline] pub const fn SQUASHFS_UNCOMPRESSED_INODES(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_NOI) }
#[inline] pub const fn SQUASHFS_UNCOMPRESSED_DATA(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_NOD) }
#[inline] pub const fn SQUASHFS_UNCOMPRESSED_FRAGMENTS(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_NOF) }
#[inline] pub const fn SQUASHFS_NO_FRAGMENTS(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_NO_FRAG) }
#[inline] pub const fn SQUASHFS_ALWAYS_FRAGMENTS(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_ALWAYS_FRAG) }
#[inline] pub const fn SQUASHFS_DUPLICATES(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_DUPLICATE) }
#[inline] pub const fn SQUASHFS_EXPORTABLE(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_EXPORT) }
#[inline] pub const fn SQUASHFS_COMP_OPTS(flags: u32) -> u32 { SQUASHFS_BIT(flags, SQUASHFS_COMP_OPT) }

pub const SQUASHFS_DIR_TYPE: u16 = 1; pub const SQUASHFS_REG_TYPE: u16 = 2;
pub const SQUASHFS_SYMLINK_TYPE: u16 = 3; pub const SQUASHFS_BLKDEV_TYPE: u16 = 4;
pub const SQUASHFS_CHRDEV_TYPE: u16 = 5; pub const SQUASHFS_FIFO_TYPE: u16 = 6;
pub const SQUASHFS_SOCKET_TYPE: u16 = 7; pub const SQUASHFS_LDIR_TYPE: u16 = 8;
pub const SQUASHFS_LREG_TYPE: u16 = 9; pub const SQUASHFS_LSYMLINK_TYPE: u16 = 10;
pub const SQUASHFS_LBLKDEV_TYPE: u16 = 11; pub const SQUASHFS_LCHRDEV_TYPE: u16 = 12;
pub const SQUASHFS_LFIFO_TYPE: u16 = 13; pub const SQUASHFS_LSOCKET_TYPE: u16 = 14;
pub const SQUASHFS_MAX_DIR_TYPE: u16 = 7;
pub const SQUASHFS_XATTR_USER: u16 = 0; pub const SQUASHFS_XATTR_TRUSTED: u16 = 1;
pub const SQUASHFS_XATTR_SECURITY: u16 = 2; pub const SQUASHFS_XATTR_VALUE_OOL: u16 = 256;
pub const SQUASHFS_XATTR_PREFIX_MASK: u16 = 0xff;
pub const SQUASHFS_COMPRESSED_BIT: u32 = 1 << 15;
#[inline] pub const fn SQUASHFS_COMPRESSED_SIZE(b: u32) -> u32 { if b & !SQUASHFS_COMPRESSED_BIT != 0 { b & !SQUASHFS_COMPRESSED_BIT } else { SQUASHFS_COMPRESSED_BIT } }
#[inline] pub const fn SQUASHFS_COMPRESSED(b: u32) -> bool { b & SQUASHFS_COMPRESSED_BIT == 0 }
pub const SQUASHFS_COMPRESSED_BIT_BLOCK: u32 = 1 << 24;
#[inline] pub const fn SQUASHFS_COMPRESSED_SIZE_BLOCK(b: u32) -> u32 { b & !SQUASHFS_COMPRESSED_BIT_BLOCK }
#[inline] pub const fn SQUASHFS_COMPRESSED_BLOCK(b: u32) -> bool { b & SQUASHFS_COMPRESSED_BIT_BLOCK == 0 }

pub type __le16 = u16; pub type __le32 = u32; pub type __le64 = u64;
extern "C" { pub fn le32_to_cpu(raw: __le32) -> u32; }
#[inline] pub unsafe fn squashfs_block_size(raw: __le32) -> i32 { let size = le32_to_cpu(raw); if size >> 25 != 0 { -EIO } else { size as i32 } }

#[inline] pub const fn SQUASHFS_INODE_BLK(a: u64) -> u32 { (a >> 16) as u32 }
#[inline] pub const fn SQUASHFS_INODE_OFFSET(a: u64) -> u32 { (a & 0xffff) as u32 }
#[inline] pub const fn SQUASHFS_MKINODE(a: u64, b: u64) -> i64 { (((a as i64) << 16) + b as i64) }

pub const SQUASHFS_CACHED_BLKS: usize = 8;
pub const SQUASHFS_META_ENTRIES: usize = 127; pub const SQUASHFS_META_SLOTS: usize = 8; pub const SQUASHFS_SCAN_INDEXES: usize = 1024;

#[repr(C)] pub struct meta_entry { pub data_block: u64, pub index_block: u32, pub offset: u16, pub pad: u16 }
#[repr(C)] pub struct meta_index { pub inode_number: u32, pub offset: u32, pub entries: u16, pub skip: u16, pub locked: u16, pub pad: u16, pub meta_entry: [meta_entry; 127] }

pub const ZLIB_COMPRESSION: u16 = 1; pub const LZMA_COMPRESSION: u16 = 2; pub const LZO_COMPRESSION: u16 = 3;
pub const XZ_COMPRESSION: u16 = 4; pub const LZ4_COMPRESSION: u16 = 5; pub const ZSTD_COMPRESSION: u16 = 6;

#[repr(C)] pub struct squashfs_super_block { pub s_magic:__le32,pub inodes:__le32,pub mkfs_time:__le32,pub block_size:__le32,pub fragments:__le32,pub compression:__le16,pub block_log:__le16,pub flags:__le16,pub no_ids:__le16,pub s_major:__le16,pub s_minor:__le16,pub root_inode:__le64,pub bytes_used:__le64,pub id_table_start:__le64,pub xattr_id_table_start:__le64,pub inode_table_start:__le64,pub directory_table_start:__le64,pub fragment_table_start:__le64,pub lookup_table_start:__le64 }
#[repr(C)] pub struct squashfs_dir_index { pub index:__le32,pub start_block:__le32,pub size:__le32,pub name:[u8;0] }
#[repr(C)] pub struct squashfs_base_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32 }
#[repr(C)] pub struct squashfs_ipc_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32 }
#[repr(C)] pub struct squashfs_lipc_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32,pub xattr:__le32 }
#[repr(C)] pub struct squashfs_dev_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32,pub rdev:__le32 }
#[repr(C)] pub struct squashfs_ldev_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32,pub rdev:__le32,pub xattr:__le32 }
#[repr(C)] pub struct squashfs_symlink_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32,pub symlink_size:__le32,pub symlink:[i8;0] }
#[repr(C)] pub struct squashfs_reg_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub start_block:__le32,pub fragment:__le32,pub offset:__le32,pub file_size:__le32,pub block_list:[__le16;0] }
#[repr(C)] pub struct squashfs_lreg_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub start_block:__le64,pub file_size:__le64,pub sparse:__le64,pub nlink:__le32,pub fragment:__le32,pub offset:__le32,pub xattr:__le32,pub block_list:[__le16;0] }
#[repr(C)] pub struct squashfs_dir_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub start_block:__le32,pub nlink:__le32,pub file_size:__le16,pub offset:__le16,pub parent_inode:__le32 }
#[repr(C)] pub struct squashfs_ldir_inode { pub inode_type:__le16,pub mode:__le16,pub uid:__le16,pub guid:__le16,pub mtime:__le32,pub inode_number:__le32,pub nlink:__le32,pub file_size:__le32,pub start_block:__le32,pub parent_inode:__le32,pub i_count:__le16,pub offset:__le16,pub xattr:__le32,pub index:[squashfs_dir_index;0] }
#[repr(C)] pub union squashfs_inode { pub base:ManuallyDrop<squashfs_base_inode>,pub dev:ManuallyDrop<squashfs_dev_inode>,pub ldev:ManuallyDrop<squashfs_ldev_inode>,pub symlink:ManuallyDrop<squashfs_symlink_inode>,pub reg:ManuallyDrop<squashfs_reg_inode>,pub lreg:ManuallyDrop<squashfs_lreg_inode>,pub dir:ManuallyDrop<squashfs_dir_inode>,pub ldir:ManuallyDrop<squashfs_ldir_inode>,pub ipc:ManuallyDrop<squashfs_ipc_inode>,pub lipc:ManuallyDrop<squashfs_lipc_inode> }
#[repr(C)] pub struct squashfs_dir_entry { pub offset:__le16,pub inode_number:__le16,pub type_:__le16,pub size:__le16,pub name:[i8;0] }
#[repr(C)] pub struct squashfs_dir_header { pub count:__le32,pub start_block:__le32,pub inode_number:__le32 }
#[repr(C)] pub struct squashfs_fragment_entry { pub start_block:__le64,pub size:__le32,pub unused:u32 }
#[repr(C)] pub struct squashfs_xattr_entry { pub type_:__le16,pub size:__le16,pub data:[i8;0] }
#[repr(C)] pub struct squashfs_xattr_val { pub vsize:__le32,pub value:[i8;0] }
#[repr(C)] pub struct squashfs_xattr_id { pub xattr:__le64,pub count:__le32,pub size:__le32 }
#[repr(C)] pub struct squashfs_xattr_id_table { pub xattr_table_start:__le64,pub xattr_ids:__le32,pub unused:__le32 }

#[inline] pub const fn SQUASHFS_FRAGMENT_BYTES(a: usize) -> usize { a * core::mem::size_of::<squashfs_fragment_entry>() }
#[inline] pub const fn SQUASHFS_FRAGMENT_INDEX(a: usize) -> usize { SQUASHFS_FRAGMENT_BYTES(a) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_FRAGMENT_INDEX_OFFSET(a: usize) -> usize { SQUASHFS_FRAGMENT_BYTES(a) % SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_FRAGMENT_INDEXES(a: usize) -> usize { (SQUASHFS_FRAGMENT_BYTES(a) + SQUASHFS_METADATA_SIZE - 1) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_FRAGMENT_INDEX_BYTES(a: usize) -> usize { SQUASHFS_FRAGMENT_INDEXES(a) * core::mem::size_of::<u64>() }
#[inline] pub const fn SQUASHFS_LOOKUP_BYTES(a: usize) -> usize { a * core::mem::size_of::<u64>() }
#[inline] pub const fn SQUASHFS_LOOKUP_BLOCK(a: usize) -> usize { SQUASHFS_LOOKUP_BYTES(a) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_LOOKUP_BLOCK_OFFSET(a: usize) -> usize { SQUASHFS_LOOKUP_BYTES(a) % SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_LOOKUP_BLOCKS(a: usize) -> usize { (SQUASHFS_LOOKUP_BYTES(a) + SQUASHFS_METADATA_SIZE - 1) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_LOOKUP_BLOCK_BYTES(a: usize) -> usize { SQUASHFS_LOOKUP_BLOCKS(a) * core::mem::size_of::<u64>() }
#[inline] pub const fn SQUASHFS_ID_BYTES(a: usize) -> usize { a * core::mem::size_of::<u32>() }
#[inline] pub const fn SQUASHFS_ID_BLOCK(a: usize) -> usize { SQUASHFS_ID_BYTES(a) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_ID_BLOCK_OFFSET(a: usize) -> usize { SQUASHFS_ID_BYTES(a) % SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_ID_BLOCKS(a: usize) -> usize { (SQUASHFS_ID_BYTES(a) + SQUASHFS_METADATA_SIZE - 1) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_ID_BLOCK_BYTES(a: usize) -> usize { SQUASHFS_ID_BLOCKS(a) * core::mem::size_of::<u64>() }
#[inline] pub const fn SQUASHFS_XATTR_BYTES(a: u64) -> usize { (a as usize) * core::mem::size_of::<squashfs_xattr_id>() }
#[inline] pub const fn SQUASHFS_XATTR_BLOCK(a: u64) -> usize { SQUASHFS_XATTR_BYTES(a) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_XATTR_BLOCK_OFFSET(a: u64) -> usize { SQUASHFS_XATTR_BYTES(a) % SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_XATTR_BLOCKS(a: u64) -> usize { (SQUASHFS_XATTR_BYTES(a) + SQUASHFS_METADATA_SIZE - 1) / SQUASHFS_METADATA_SIZE }
#[inline] pub const fn SQUASHFS_XATTR_BLOCK_BYTES(a: u64) -> usize { SQUASHFS_XATTR_BLOCKS(a) * core::mem::size_of::<u64>() }
#[inline] pub const fn SQUASHFS_XATTR_BLK(a: u64) -> u32 { (a >> 16) as u32 }
#[inline] pub const fn SQUASHFS_XATTR_OFFSET(a: u64) -> u32 { (a & 0xffff) as u32 }
pub const SQUASHFS_META_INDEXES: usize = SQUASHFS_METADATA_SIZE / core::mem::size_of::<u32>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
