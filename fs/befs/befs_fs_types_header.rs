/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs/befs/befs_fs_types.h
 *
 * Copyright (C) 2001 Will Dyson (will@cs.earlham.edu)
 *
 * from linux/include/linux/befs_fs.h
 * Copyright (C) 1999 Makoto Kato (m_kato@ga2.so-net.ne.jp)
 */

// C header guard and kernel-only includes omitted; required kernel types are
// represented by their corresponding Rust integer types below.

pub const BEFS_NAME_LEN: usize = 255;
pub const BEFS_SYMLINK_LEN: usize = 144;
pub const BEFS_NUM_DIRECT_BLOCKS: usize = 12;
pub const B_OS_NAME_LENGTH: usize = 32;
pub const BEFS_DBLINDIR_BRUN_LEN: usize = 4;

#[repr(u32)]
pub enum super_flags {
    BEFS_BYTESEX_BE = 0,
    BEFS_BYTESEX_LE = 1,
    BEFS_CLEAN = 0x434c454e,
    BEFS_DIRTY = 0x44495254,
    BEFS_SUPER_MAGIC1 = 0x42465331,
    BEFS_SUPER_MAGIC2 = 0xdd121031,
    BEFS_SUPER_MAGIC3 = 0x15b6830e,
}

pub const BEFS_BYTEORDER_NATIVE: u32 = 0x42494745;
pub const BEFS_BYTEORDER_NATIVE_LE: fs32 = BEFS_BYTEORDER_NATIVE.to_le();
pub const BEFS_BYTEORDER_NATIVE_BE: fs32 = BEFS_BYTEORDER_NATIVE.to_be();
pub const BEFS_SUPER_MAGIC: u32 = 0x42465331;
pub const BEFS_SUPER_MAGIC1_LE: fs32 = BEFS_SUPER_MAGIC.to_le();
pub const BEFS_SUPER_MAGIC1_BE: fs32 = BEFS_SUPER_MAGIC.to_be();

pub const BEFS_INODE_MAGIC1: u32 = 0x3bbe0ad9;

#[repr(u32)]
pub enum inode_flags {
    BEFS_INODE_IN_USE = 0x00000001,
    BEFS_ATTR_INODE = 0x00000004,
    BEFS_INODE_LOGGED = 0x00000008,
    BEFS_INODE_DELETED = 0x00000010,
    BEFS_LONG_SYMLINK = 0x00000040,
    BEFS_PERMANENT_FLAG = 0x0000ffff,
    BEFS_INODE_NO_CREATE = 0x00010000,
    BEFS_INODE_WAS_WRITTEN = 0x00020000,
    BEFS_NO_TRANSACTION = 0x00040000,
}

pub type fs64 = u64;
pub type fs32 = u32;
pub type fs16 = u16;
pub type befs_off_t = u64;
pub type befs_time_t = fs64;

#[repr(C, packed)]
pub struct befs_disk_block_run { pub allocation_group: fs32, pub start: fs16, pub len: fs16 }
#[repr(C, packed)]
pub struct befs_block_run { pub allocation_group: u32, pub start: u16, pub len: u16 }
pub type befs_disk_inode_addr = befs_disk_block_run;
pub type befs_inode_addr = befs_block_run;

#[repr(C, packed)]
pub struct befs_super_block {
    pub name: [i8; B_OS_NAME_LENGTH], pub magic1: fs32, pub fs_byte_order: fs32,
    pub block_size: fs32, pub block_shift: fs32, pub num_blocks: fs64, pub used_blocks: fs64,
    pub inode_size: fs32, pub magic2: fs32, pub blocks_per_ag: fs32, pub ag_shift: fs32,
    pub num_ags: fs32, pub flags: fs32, pub log_blocks: befs_disk_block_run,
    pub log_start: fs64, pub log_end: fs64, pub magic3: fs32,
    pub root_dir: befs_disk_inode_addr, pub indices: befs_disk_inode_addr,
}

#[repr(C, packed)]
pub struct befs_disk_data_stream {
    pub direct: [befs_disk_block_run; BEFS_NUM_DIRECT_BLOCKS], pub max_direct_range: fs64,
    pub indirect: befs_disk_block_run, pub max_indirect_range: fs64,
    pub double_indirect: befs_disk_block_run, pub max_double_indirect_range: fs64, pub size: fs64,
}
#[repr(C, packed)]
pub struct befs_data_stream {
    pub direct: [befs_block_run; BEFS_NUM_DIRECT_BLOCKS], pub max_direct_range: befs_off_t,
    pub indirect: befs_block_run, pub max_indirect_range: befs_off_t,
    pub double_indirect: befs_block_run, pub max_double_indirect_range: befs_off_t, pub size: befs_off_t,
}

#[repr(C, packed)]
pub struct befs_small_data { pub type_: fs32, pub name_size: fs16, pub data_size: fs16, pub name: [i8; 1] }

#[repr(C)]
pub union befs_inode_data { pub datastream: befs_disk_data_stream, pub symlink: [i8; BEFS_SYMLINK_LEN] }
#[repr(C, packed)]
pub struct befs_inode {
    pub magic1: fs32, pub inode_num: befs_disk_inode_addr, pub uid: fs32, pub gid: fs32,
    pub mode: fs32, pub flags: fs32, pub create_time: befs_time_t, pub last_modified_time: befs_time_t,
    pub parent: befs_disk_inode_addr, pub attributes: befs_disk_inode_addr, pub type_: fs32,
    pub inode_size: fs32, pub etc: fs32, pub data: befs_inode_data, pub pad: [fs32; 4],
    pub small_data: [befs_small_data; 1],
}

pub const BEFS_BTREE_MAGIC: u32 = 0x69f6c2e8;
#[repr(u32)]
pub enum btree_types {
    BTREE_STRING_TYPE = 0, BTREE_INT32_TYPE = 1, BTREE_UINT32_TYPE = 2,
    BTREE_INT64_TYPE = 3, BTREE_UINT64_TYPE = 4, BTREE_FLOAT_TYPE = 5, BTREE_DOUBLE_TYPE = 6,
}

#[repr(C, packed)]
pub struct befs_disk_btree_super {
    pub magic: fs32, pub node_size: fs32, pub max_depth: fs32, pub data_type: fs32,
    pub root_node_ptr: fs64, pub free_node_ptr: fs64, pub max_size: fs64,
}
#[repr(C, packed)]
pub struct befs_btree_super {
    pub magic: u32, pub node_size: u32, pub max_depth: u32, pub data_type: u32,
    pub root_node_ptr: befs_off_t, pub free_node_ptr: befs_off_t, pub max_size: befs_off_t,
}
#[repr(C, packed)]
pub struct befs_btree_nodehead {
    pub left: fs64, pub right: fs64, pub overflow: fs64, pub all_key_count: fs16, pub all_key_length: fs16,
}
#[repr(C, packed)]
pub struct befs_host_btree_nodehead {
    pub left: befs_off_t, pub right: befs_off_t, pub overflow: befs_off_t,
    pub all_key_count: u16, pub all_key_length: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
