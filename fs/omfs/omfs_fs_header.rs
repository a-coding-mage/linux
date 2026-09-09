/* SPDX-License-Identifier: GPL-2.0 */

/* OMFS On-disk structures */

pub const OMFS_MAGIC: u32 = 0xC2993D87;
pub const OMFS_IMAGIC: u8 = 0xD2;

pub const OMFS_DIR: i8 = b'D' as i8;
pub const OMFS_FILE: i8 = b'F' as i8;
pub const OMFS_INODE_NORMAL: i8 = b'e' as i8;
pub const OMFS_INODE_CONTINUATION: i8 = b'c' as i8;
pub const OMFS_INODE_SYSTEM: i8 = b's' as i8;
pub const OMFS_NAMELEN: usize = 256;
pub const OMFS_DIR_START: usize = 0x1b8;
pub const OMFS_EXTENT_START: usize = 0x1d0;
pub const OMFS_EXTENT_CONT: usize = 0x40;
pub const OMFS_XOR_COUNT: usize = 19;
pub const OMFS_MAX_BLOCK_SIZE: usize = 8192;
pub const OMFS_MAX_CLUSTER_SIZE: usize = 8;
pub const OMFS_MAX_BLOCKS: u64 = 1u64 << 31;

#[repr(C)]
pub struct omfs_super_block {
    pub s_fill1: [i8; 256],
    pub s_root_block: u64, // block number of omfs_root_block
    pub s_num_blocks: u64, // total number of FS blocks
    pub s_magic: u32, // OMFS_MAGIC
    pub s_blocksize: u32, // size of a block
    pub s_mirrors: u32, // # of mirrors of system blocks
    pub s_sys_blocksize: u32, // size of non-data blocks
}

#[repr(C)]
pub struct omfs_header {
    pub h_self: u64, // FS block where this is located
    pub h_body_size: u32, // size of useful data after header
    pub h_crc: u16, // crc-ccitt of body_size bytes
    pub h_fill1: [i8; 2],
    pub h_version: u8, // version, always 1
    pub h_type: i8, // OMFS_INODE_X
    pub h_magic: u8, // OMFS_IMAGIC
    pub h_check_xor: u8, // XOR of header bytes before this
    pub h_fill2: u32,
}

#[repr(C)]
pub struct omfs_root_block {
    pub r_head: omfs_header, // header
    pub r_fill1: u64,
    pub r_num_blocks: u64, // total number of FS blocks
    pub r_root_dir: u64, // block # of root directory
    pub r_bitmap: u64, // block # of free space bitmap
    pub r_blocksize: u32, // size of a block
    pub r_clustersize: u32, // size allocated for data blocks
    pub r_mirrors: u64, // # of mirrors of system blocks
    pub r_name: [i8; OMFS_NAMELEN], // partition label
}

#[repr(C)]
pub struct omfs_inode {
    pub i_head: omfs_header, // header
    pub i_parent: u64, // parent containing this inode
    pub i_sibling: u64, // next inode in hash bucket
    pub i_ctime: u64, // ctime, in milliseconds
    pub i_fill1: [i8; 35],
    pub i_type: i8, // OMFS_[DIR,FILE]
    pub i_fill2: u32,
    pub i_fill3: [i8; 64],
    pub i_name: [i8; OMFS_NAMELEN], // filename
    pub i_size: u64, // size of file, in bytes
}

#[repr(C)]
pub struct omfs_extent_entry {
    pub e_cluster: u64, // start location of a set of blocks
    pub e_blocks: u64, // number of blocks after e_cluster
}

#[repr(C)]
pub struct omfs_extent {
    pub e_next: u64, // next extent table location
    pub e_extent_count: u32, // total # extents in this table
    pub e_fill: u32,
    pub e_entry: [omfs_extent_entry; 0], // start of extent entries
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
