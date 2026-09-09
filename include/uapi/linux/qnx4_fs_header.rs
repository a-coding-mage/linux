/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  Name                         : qnx4_fs.h
 *  Author                       : Richard Frowijn
 *  Function                     : qnx4 global filesystem definitions
 *  History                      : 23-03-1998 created
 */

// Dependencies supplied by the corresponding Linux type, qnxtypes, and magic headers.

pub const QNX4_ROOT_INO: i32 = 1;

pub const QNX4_MAX_XTNTS_PER_XBLK: i32 = 60;
/* for di_status */
pub const QNX4_FILE_USED: i32 = 0x01;
pub const QNX4_FILE_MODIFIED: i32 = 0x02;
pub const QNX4_FILE_BUSY: i32 = 0x04;
pub const QNX4_FILE_LINK: i32 = 0x08;
pub const QNX4_FILE_INODE: i32 = 0x10;
pub const QNX4_FILE_FSYSCLEAN: i32 = 0x20;

pub const QNX4_I_MAP_SLOTS: i32 = 8;
pub const QNX4_Z_MAP_SLOTS: i32 = 64;
pub const QNX4_VALID_FS: i32 = 0x0001; /* Clean fs. */
pub const QNX4_ERROR_FS: i32 = 0x0002; /* fs has errors. */
pub const QNX4_BLOCK_SIZE: i32 = 0x200; /* blocksize of 512 bytes */
pub const QNX4_BLOCK_SIZE_BITS: i32 = 9; /* blocksize shift */
pub const QNX4_DIR_ENTRY_SIZE: i32 = 0x040; /* dir entry size of 64 bytes */
pub const QNX4_DIR_ENTRY_SIZE_BITS: i32 = 6; /* dir entry size shift */
pub const QNX4_XBLK_ENTRY_SIZE: i32 = 0x200; /* xblk entry size */
pub const QNX4_INODES_PER_BLOCK: i32 = 0x08; /* 512 / 64 */

/* for filenames */
pub const QNX4_SHORT_NAME_MAX: i32 = 16;
pub const QNX4_NAME_MAX: i32 = 48;

/*
 * This is the original qnx4 inode layout on disk.
 */
#[repr(C)]
pub struct qnx4_inode_entry {
    pub di_fname: [i8; QNX4_SHORT_NAME_MAX as usize],
    pub di_size: qnx4_off_t,
    pub di_first_xtnt: qnx4_xtnt_t,
    pub di_xblk: __le32,
    pub di_ftime: __le32,
    pub di_mtime: __le32,
    pub di_atime: __le32,
    pub di_ctime: __le32,
    pub di_num_xtnts: qnx4_nxtnt_t,
    pub di_mode: qnx4_mode_t,
    pub di_uid: qnx4_muid_t,
    pub di_gid: qnx4_mgid_t,
    pub di_nlink: qnx4_nlink_t,
    pub di_zero: [__u8; 4],
    pub di_type: qnx4_ftype_t,
    pub di_status: __u8,
}

#[repr(C)]
pub struct qnx4_link_info {
    pub dl_fname: [i8; QNX4_NAME_MAX as usize],
    pub dl_inode_blk: __le32,
    pub dl_inode_ndx: __u8,
    pub dl_spare: [__u8; 10],
    pub dl_status: __u8,
}

#[repr(C)]
pub struct qnx4_xblk {
    pub xblk_next_xblk: __le32,
    pub xblk_prev_xblk: __le32,
    pub xblk_num_xtnts: __u8,
    pub xblk_spare: [__u8; 3],
    pub xblk_num_blocks: __le32,
    pub xblk_xtnts: [qnx4_xtnt_t; QNX4_MAX_XTNTS_PER_XBLK as usize],
    pub xblk_signature: [i8; 8],
    pub xblk_first_xtnt: qnx4_xtnt_t,
}

#[repr(C)]
pub struct qnx4_super_block {
    pub RootDir: qnx4_inode_entry,
    pub Inode: qnx4_inode_entry,
    pub Boot: qnx4_inode_entry,
    pub AltBoot: qnx4_inode_entry,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
