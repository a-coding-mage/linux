/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: linux/types.h and linux/magic.h provide the integer
// aliases and BLOCK_SIZE used by this header.

/*
 * The minix filesystem constants/structures
 */

/*
 * Thanks to Kees J Bot for sending me the definitions of the new
 * minix filesystem (aka V2) with bigger inodes and 32-bit block
 * pointers.
 */

pub const MINIX_ROOT_INO: u32 = 1;

/* Not the same as the bogus LINK_MAX in <linux/limits.h>. Oh well. */
pub const MINIX_LINK_MAX: u32 = 250;
pub const MINIX2_LINK_MAX: u32 = 65530;

pub const MINIX_I_MAP_SLOTS: u32 = 8;
pub const MINIX_Z_MAP_SLOTS: u32 = 64;
pub const MINIX_VALID_FS: u32 = 0x0001; // Clean fs.
pub const MINIX_ERROR_FS: u32 = 0x0002; // fs has errors.

pub const MINIX_INODES_PER_BLOCK: usize = BLOCK_SIZE / core::mem::size_of::<minix_inode>();

/*
 * This is the original minix inode layout on disk.
 * Note the 8-bit gid and atime and ctime.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_inode {
    pub i_mode: u16,
    pub i_uid: u16,
    pub i_size: u32,
    pub i_time: u32,
    pub i_gid: u8,
    pub i_nlinks: u8,
    pub i_zone: [u16; 9],
}

/*
 * The new minix inode has all the time entries, as well as
 * long block numbers and a third indirect block (7+1+1+1
 * instead of 7+1+1). Also, some previously 8-bit values are
 * now 16-bit. The inode is now 64 bytes instead of 32.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix2_inode {
    pub i_mode: u16,
    pub i_nlinks: u16,
    pub i_uid: u16,
    pub i_gid: u16,
    pub i_size: u32,
    pub i_atime: u32,
    pub i_mtime: u32,
    pub i_ctime: u32,
    pub i_zone: [u32; 10],
}

/*
 * minix super-block data on disk
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_super_block {
    pub s_ninodes: u16,
    pub s_nzones: u16,
    pub s_imap_blocks: u16,
    pub s_zmap_blocks: u16,
    pub s_firstdatazone: u16,
    pub s_log_zone_size: u16,
    pub s_max_size: u32,
    pub s_magic: u16,
    pub s_state: u16,
    pub s_zones: u32,
}

/*
 * V3 minix super-block data on disk
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix3_super_block {
    pub s_ninodes: u32,
    pub s_pad0: u16,
    pub s_imap_blocks: u16,
    pub s_zmap_blocks: u16,
    pub s_firstdatazone: u16,
    pub s_log_zone_size: u16,
    pub s_pad1: u16,
    pub s_max_size: u32,
    pub s_zones: u32,
    pub s_magic: u16,
    pub s_pad2: u16,
    pub s_blocksize: u16,
    pub s_disk_version: u8,
}

#[repr(C)]
pub struct minix_dir_entry {
    pub inode: u16,
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct minix3_dir_entry {
    pub inode: u32,
    pub name: [core::ffi::c_char; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
