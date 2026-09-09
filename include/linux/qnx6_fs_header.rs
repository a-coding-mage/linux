/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Name                 : qnx6_fs.h
 *  Author               : Kai Bankett
 *  Function             : qnx6 global filesystem definitions
 *  History              : 17-01-2012 created
 */

// Dependencies supplied by the surrounding Linux-compatible environment:
// linux/types.h and linux/magic.h.

pub const QNX6_ROOT_INO: u32 = 1;

/* for di_status */
pub const QNX6_FILE_DIRECTORY: u32 = 0x01;
pub const QNX6_FILE_DELETED: u32 = 0x02;
pub const QNX6_FILE_NORMAL: u32 = 0x03;

pub const QNX6_SUPERBLOCK_SIZE: u32 = 0x200; /* superblock always is 512 bytes */
pub const QNX6_SUPERBLOCK_AREA: u32 = 0x1000; /* area reserved for superblock */
pub const QNX6_BOOTBLOCK_SIZE: u32 = 0x2000; /* heading bootblock area */
pub const QNX6_DIR_ENTRY_SIZE: u32 = 0x20; /* dir entry size of 32 bytes */
pub const QNX6_INODE_SIZE: u32 = 0x80; /* each inode is 128 bytes */
pub const QNX6_INODE_SIZE_BITS: u32 = 7; /* inode entry size shift */

pub const QNX6_NO_DIRECT_POINTERS: usize = 16; /* 16 blockptrs in sbl/inode */
pub const QNX6_PTR_MAX_LEVELS: u32 = 5; /* maximum indirect levels */

/* for filenames */
pub const QNX6_SHORT_NAME_MAX: usize = 27;
pub const QNX6_LONG_NAME_MAX: usize = 510;

/* list of mount options */
pub const QNX6_MOUNT_MMI_FS: u32 = 0x010000; /* mount as Audi MMI 3G fs */

/*
 * This is the original qnx6 inode layout on disk.
 * Each inode is 128 byte long.
 */
#[repr(C)]
pub struct qnx6_inode_entry {
    pub di_size: __fs64,
    pub di_uid: __fs32,
    pub di_gid: __fs32,
    pub di_ftime: __fs32,
    pub di_mtime: __fs32,
    pub di_atime: __fs32,
    pub di_ctime: __fs32,
    pub di_mode: __fs16,
    pub di_ext_mode: __fs16,
    pub di_block_ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub di_filelevels: __u8,
    pub di_status: __u8,
    pub di_unknown2: [__u8; 2],
    pub di_zero2: [__fs32; 6],
}

/*
 * Each directory entry is maximum 32 bytes long.
 * If more characters or special characters required it is stored
 * in the longfilenames structure.
 */
#[repr(C)]
pub struct qnx6_dir_entry {
    pub de_inode: __fs32,
    pub de_size: __u8,
    pub de_fname: [i8; QNX6_SHORT_NAME_MAX],
}

/* Longfilename direntries have a different structure */
#[repr(C)]
pub struct qnx6_long_dir_entry {
    pub de_inode: __fs32,
    pub de_size: __u8,
    pub de_unknown: [__u8; 3],
    pub de_long_inode: __fs32,
    pub de_checksum: __fs32,
}

#[repr(C)]
pub struct qnx6_long_filename {
    pub lf_size: __fs16,
    pub lf_fname: [__u8; QNX6_LONG_NAME_MAX],
}

#[repr(C)]
pub struct qnx6_root_node {
    pub size: __fs64,
    pub ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub levels: __u8,
    pub mode: __u8,
    pub spare: [__u8; 6],
}

#[repr(C)]
pub struct qnx6_super_block {
    pub sb_magic: __fs32,
    pub sb_checksum: __fs32,
    pub sb_serial: __fs64,
    pub sb_ctime: __fs32, /* time the fs was created */
    pub sb_atime: __fs32, /* last access time */
    pub sb_flags: __fs32,
    pub sb_version1: __fs16, /* filesystem version information */
    pub sb_version2: __fs16, /* filesystem version information */
    pub sb_volumeid: [__u8; 16],
    pub sb_blocksize: __fs32,
    pub sb_num_inodes: __fs32,
    pub sb_free_inodes: __fs32,
    pub sb_num_blocks: __fs32,
    pub sb_free_blocks: __fs32,
    pub sb_allocgroup: __fs32,
    pub Inode: qnx6_root_node,
    pub Bitmap: qnx6_root_node,
    pub Longfile: qnx6_root_node,
    pub Unknown: qnx6_root_node,
}

/* Audi MMI 3G superblock layout is different to plain qnx6 */
#[repr(C)]
pub struct qnx6_mmi_super_block {
    pub sb_magic: __fs32,
    pub sb_checksum: __fs32,
    pub sb_serial: __fs64,
    pub sb_spare0: [__u8; 12],
    pub sb_id: [__u8; 12],
    pub sb_blocksize: __fs32,
    pub sb_num_inodes: __fs32,
    pub sb_free_inodes: __fs32,
    pub sb_num_blocks: __fs32,
    pub sb_free_blocks: __fs32,
    pub sb_spare1: [__u8; 4],
    pub Inode: qnx6_root_node,
    pub Bitmap: qnx6_root_node,
    pub Longfile: qnx6_root_node,
    pub Unknown: qnx6_root_node,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
