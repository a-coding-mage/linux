/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions of structures for vfsv0 quota format
 */

/* C dependencies: linux/types.h and linux/quota.h */

/*
 * Definitions of magics and versions of current quota files
 */
pub const V2_INITQMAGICS: [u32; 3] = [
    0xd9c01f11, /* USRQUOTA */
    0xd9c01927, /* GRPQUOTA */
    0xd9c03f14, /* PRJQUOTA */
];

pub const V2_INITQVERSIONS: [u32; 3] = [
    1, /* USRQUOTA */
    1, /* GRPQUOTA */
    1, /* PRJQUOTA */
];

/* First generic header */
#[repr(C)]
pub struct v2_disk_dqheader {
    pub dqh_magic: u32,   /* Magic number identifying file */
    pub dqh_version: u32, /* File version */
}

/*
 * The following structure defines the format of the disk quota file
 * (as it appears on disk) - the file is a radix tree whose leaves point
 * to blocks of these structures.
 */
#[repr(C)]
pub struct v2r0_disk_dqblk {
    pub dqb_id: u32,          /* id this quota applies to */
    pub dqb_ihardlimit: u32,  /* absolute limit on allocated inodes */
    pub dqb_isoftlimit: u32,  /* preferred inode limit */
    pub dqb_curinodes: u32,   /* current # allocated inodes */
    pub dqb_bhardlimit: u32,  /* absolute limit on disk space (in QUOTABLOCK_SIZE) */
    pub dqb_bsoftlimit: u32,  /* preferred limit on disk space (in QUOTABLOCK_SIZE) */
    pub dqb_curspace: u64,    /* current space occupied (in bytes) */
    pub dqb_btime: u64,       /* time limit for excessive disk use */
    pub dqb_itime: u64,       /* time limit for excessive inode use */
}

#[repr(C)]
pub struct v2r1_disk_dqblk {
    pub dqb_id: u32,          /* id this quota applies to */
    pub dqb_pad: u32,
    pub dqb_ihardlimit: u64,  /* absolute limit on allocated inodes */
    pub dqb_isoftlimit: u64,  /* preferred inode limit */
    pub dqb_curinodes: u64,   /* current # allocated inodes */
    pub dqb_bhardlimit: u64,  /* absolute limit on disk space (in QUOTABLOCK_SIZE) */
    pub dqb_bsoftlimit: u64,  /* preferred limit on disk space (in QUOTABLOCK_SIZE) */
    pub dqb_curspace: u64,    /* current space occupied (in bytes) */
    pub dqb_btime: u64,       /* time limit for excessive disk use */
    pub dqb_itime: u64,       /* time limit for excessive inode use */
}

/* Header with type and version specific information */
#[repr(C)]
pub struct v2_disk_dqinfo {
    pub dqi_bgrace: u32,     /* Time before block soft limit becomes hard limit */
    pub dqi_igrace: u32,     /* Time before inode soft limit becomes hard limit */
    pub dqi_flags: u32,      /* Flags for quotafile (DQF_*) */
    pub dqi_blocks: u32,     /* Number of blocks in file */
    pub dqi_free_blk: u32,   /* Number of first free block in the list */
    pub dqi_free_entry: u32, /* Number of block with at least one free entry */
}

pub const V2_DQINFOOFF: usize = core::mem::size_of::<v2_disk_dqheader>();
pub const V2_DQBLKSIZE_BITS: u32 = 10; /* Size of leaf block in tree */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
