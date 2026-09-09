/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: __u8 and __u32 are supplied by linux/types.h.
// Dependency intent: filesystem magic definitions are supplied by linux/magic.h.

pub const CRAMFS_SIGNATURE: &[u8] = b"Compressed ROMFS\0";

/*
 * Width of various bitfields in struct cramfs_inode.
 * Primarily used to generate warnings in mkcramfs.
 */
pub const CRAMFS_MODE_WIDTH: u32 = 16;
pub const CRAMFS_UID_WIDTH: u32 = 16;
pub const CRAMFS_SIZE_WIDTH: u32 = 24;
pub const CRAMFS_GID_WIDTH: u32 = 8;
pub const CRAMFS_NAMELEN_WIDTH: u32 = 6;
pub const CRAMFS_OFFSET_WIDTH: u32 = 26;

/*
 * Since inode.namelen is a unsigned 6-bit number, the maximum cramfs
 * path length is 63 << 2 = 252.
 */
pub const CRAMFS_MAXPATHLEN: u32 = ((1u32 << CRAMFS_NAMELEN_WIDTH) - 1) << 2;

/*
 * Reasonably terse representation of the inode data.
 *
 * C bitfields are represented by their containing words.  The bit layout is
 * mode:0..16, uid:16..32; size:0..24, gid:24..32; and
 * namelen:0..6, offset:6..32, respectively.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_inode {
    pub mode_uid: __u32,
    /* SIZE for device files is i_rdev */
    pub size_gid: __u32,
    /* NAMELEN is the length of the file name, divided by 4 and
       rounded up.  (cramfs doesn't support hard links.) */
    /* OFFSET: For symlinks and non-empty regular files, this
       contains the offset (divided by 4) of the file data in
       compressed form (starting with an array of block pointers;
       see README).  For non-empty directories it is the offset
       (divided by 4) of the inode of the first file in that
       directory.  For anything else, offset is zero. */
    pub namelen_offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_info {
    pub crc: __u32,
    pub edition: __u32,
    pub blocks: __u32,
    pub files: __u32,
}

/*
 * Superblock information at the beginning of the FS.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_super {
    pub magic: __u32,          /* 0x28cd3d45 - random number */
    pub size: __u32,           /* length in bytes */
    pub flags: __u32,          /* feature flags */
    pub future: __u32,         /* reserved for future use */
    pub signature: [__u8; 16], /* "Compressed ROMFS" */
    pub fsid: cramfs_info,     /* unique filesystem info */
    pub name: [__u8; 16],      /* user-defined name */
    pub root: cramfs_inode,    /* root inode data */
}

/*
 * Feature flags
 *
 * 0x00000000 - 0x000000ff: features that work for all past kernels
 * 0x00000100 - 0xffffffff: features that don't work for past kernels
 */
pub const CRAMFS_FLAG_FSID_VERSION_2: __u32 = 0x00000001; /* fsid version #2 */
pub const CRAMFS_FLAG_SORTED_DIRS: __u32 = 0x00000002; /* sorted dirs */
pub const CRAMFS_FLAG_HOLES: __u32 = 0x00000100; /* support for holes */
pub const CRAMFS_FLAG_WRONG_SIGNATURE: __u32 = 0x00000200; /* reserved */
pub const CRAMFS_FLAG_SHIFTED_ROOT_OFFSET: __u32 = 0x00000400; /* shifted root fs */
pub const CRAMFS_FLAG_EXT_BLOCK_POINTERS: __u32 = 0x00000800; /* block pointer extensions */

/*
 * Valid values in super.flags.  Currently we refuse to mount
 * if (flags & ~CRAMFS_SUPPORTED_FLAGS).  Maybe that should be
 * changed to test super.future instead.
 */
pub const CRAMFS_SUPPORTED_FLAGS: __u32 = 0x000000ff
    | CRAMFS_FLAG_HOLES
    | CRAMFS_FLAG_WRONG_SIGNATURE
    | CRAMFS_FLAG_SHIFTED_ROOT_OFFSET
    | CRAMFS_FLAG_EXT_BLOCK_POINTERS;

/*
 * Block pointer flags
 *
 * The maximum block offset that needs to be represented is roughly:
 *
 *   (1 << CRAMFS_OFFSET_WIDTH) * 4 +
 *   (1 << CRAMFS_SIZE_WIDTH) / PAGE_SIZE * (4 + PAGE_SIZE)
 *   = 0x11004000
 *
 * That leaves room for 3 flag bits in the block pointer table.
 */
pub const CRAMFS_BLK_FLAG_UNCOMPRESSED: __u32 = 1 << 31;
pub const CRAMFS_BLK_FLAG_DIRECT_PTR: __u32 = 1 << 30;

pub const CRAMFS_BLK_FLAGS: __u32 =
    CRAMFS_BLK_FLAG_UNCOMPRESSED | CRAMFS_BLK_FLAG_DIRECT_PTR;

/*
 * Direct blocks are at least 4-byte aligned.
 * Pointers to direct blocks are shifted down by 2 bits.
 */
pub const CRAMFS_BLK_DIRECT_PTR_SHIFT: __u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
