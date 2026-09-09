/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) International Business Machines Corp., 2000-2003 */
/* Translated from jfs_filsys.h. */

/* file system option (superblock flag) */
pub const JFS_UNICODE: u32 = 0x00000001;
pub const JFS_ERR_REMOUNT_RO: u32 = 0x00000002;
pub const JFS_ERR_CONTINUE: u32 = 0x00000004;
pub const JFS_ERR_PANIC: u32 = 0x00000008;
pub const JFS_ERR_MASK: u32 = JFS_ERR_REMOUNT_RO | JFS_ERR_CONTINUE | JFS_ERR_PANIC;
pub const JFS_USRQUOTA: u32 = 0x00000010;
pub const JFS_GRPQUOTA: u32 = 0x00000020;
pub const JFS_NOINTEGRITY: u32 = 0x00000040;
pub const JFS_DISCARD: u32 = 0x00000080;
pub const JFS_COMMIT: u32 = 0x00000f00;
pub const JFS_GROUPCOMMIT: u32 = 0x00000100;
pub const JFS_LAZYCOMMIT: u32 = 0x00000200;
pub const JFS_TMPFS: u32 = 0x00000400;
pub const JFS_INLINELOG: u32 = 0x00000800;
pub const JFS_INLINEMOVE: u32 = 0x00001000;
pub const JFS_BAD_SAIT: u32 = 0x00010000;
pub const JFS_SPARSE: u32 = 0x00020000;
pub const JFS_DASD_ENABLED: u32 = 0x00040000;
pub const JFS_DASD_PRIME: u32 = 0x00080000;
pub const JFS_SWAP_BYTES: u32 = 0x00100000;
pub const JFS_DIR_INDEX: u32 = 0x00200000;
pub const JFS_LINUX: u32 = 0x10000000;
pub const JFS_DFS: u32 = 0x20000000;
pub const JFS_OS2: u32 = 0x40000000;
pub const JFS_AIX: u32 = 0x80000000;

pub const PSIZE: u32 = 4096;
pub const L2PSIZE: u32 = 12;
pub const POFFSET: u32 = 4095;
pub const BPSIZE: u32 = PSIZE;
pub const PBSIZE: u32 = 512;
pub const L2PBSIZE: u32 = 9;
pub const DISIZE: u32 = 512;
pub const L2DISIZE: u32 = 9;
pub const IDATASIZE: u32 = 256;
pub const IXATTRSIZE: u32 = 128;
pub const XTPAGE_SIZE: u32 = 4096;
pub const log2_PAGESIZE: u32 = 12;
pub const IAG_SIZE: u32 = 4096;
pub const IAG_EXTENT_SIZE: u32 = 4096;
pub const INOSPERIAG: u32 = 4096;
pub const L2INOSPERIAG: u32 = 12;
pub const INOSPEREXT: u32 = 32;
pub const L2INOSPEREXT: u32 = 5;
pub const IXSIZE: u32 = DISIZE * INOSPEREXT;
pub const INOSPERPAGE: u32 = 8;
pub const L2INOSPERPAGE: u32 = 3;
pub const IAGFREELIST_LWM: u32 = 64;
pub const INODE_EXTENT_SIZE: u32 = IXSIZE;
pub const NUM_INODE_PER_EXTENT: u32 = INOSPEREXT;
pub const NUM_INODE_PER_IAG: u32 = INOSPERIAG;
pub const MINBLOCKSIZE: u32 = 512;
pub const L2MINBLOCKSIZE: u32 = 9;
pub const MAXBLOCKSIZE: u32 = 4096;
pub const L2MAXBLOCKSIZE: u32 = 12;
pub const MAXFILESIZE: i64 = 1i64 << 52;
pub const JFS_LINK_MAX: u32 = 0xffffffff;
pub const MINJFS: u32 = 0x1000000;
pub const MINJFSTEXT: &str = "16";

#[macro_export]
macro_rules! LBOFFSET { ($x:expr) => { ($x) & ($crate::PBSIZE - 1) }; }
#[macro_export]
macro_rules! LBNUMBER { ($x:expr) => { ($x) >> $crate::L2PBSIZE }; }
#[macro_export]
macro_rules! LBLK2PBLK { ($sb:expr, $b:expr) => { ($b) << (($sb).s_blocksize_bits - $crate::L2PBSIZE) }; }
#[macro_export]
macro_rules! PBLK2LBLK { ($sb:expr, $b:expr) => { ($b) >> (($sb).s_blocksize_bits - $crate::L2PBSIZE) }; }
#[macro_export]
macro_rules! SIZE2PN { ($size:expr) => { ((($size) - 1) as i64) >> $crate::L2PSIZE }; }
#[macro_export]
macro_rules! SIZE2BN { ($size:expr, $l2bsize:expr) => { ((($size) - 1) as i64) >> ($l2bsize) }; }

pub const SUPER1_B: u32 = 64;
pub const AIMAP_B: u32 = SUPER1_B + 8;
pub const AITBL_B: u32 = AIMAP_B + 16;
pub const SUPER2_B: u32 = AITBL_B + 32;
pub const BMAP_B: u32 = SUPER2_B + 8;
pub const SIZE_OF_SUPER: u32 = PSIZE;
pub const SIZE_OF_AG_TABLE: u32 = PSIZE;
pub const SIZE_OF_MAP_PAGE: u32 = PSIZE;
pub const SUPER1_OFF: u32 = 0x8000;
pub const AIMAP_OFF: u32 = SUPER1_OFF + SIZE_OF_SUPER;
pub const AITBL_OFF: u32 = AIMAP_OFF + (SIZE_OF_MAP_PAGE << 1);
pub const SUPER2_OFF: u32 = AITBL_OFF + INODE_EXTENT_SIZE;
pub const BMAP_OFF: u32 = SUPER2_OFF + SIZE_OF_SUPER;
pub const AGGR_RSVD_BLOCKS: u32 = SUPER1_B;
pub const AGGR_RSVD_BYTES: u32 = SUPER1_OFF;
pub const AGGR_INODE_TABLE_START: u32 = AITBL_OFF;

pub const AGGR_RESERVED_I: u32 = 0;
pub const AGGREGATE_I: u32 = 1;
pub const BMAP_I: u32 = 2;
pub const LOG_I: u32 = 3;
pub const BADBLOCK_I: u32 = 4;
pub const FILESYSTEM_I: u32 = 16;
pub const FILESET_RSVD_I: u32 = 0;
pub const FILESET_EXT_I: u32 = 1;
pub const ROOT_I: u32 = 2;
pub const ACL_I: u32 = 3;
pub const FILESET_OBJECT_I: u32 = 4;
pub const FIRST_FILESET_INO: u32 = 16;
pub const JFS_NAME_MAX: u32 = 255;
pub const JFS_PATH_MAX: u32 = BPSIZE;

pub const FM_CLEAN: u32 = 0x00000000;
pub const FM_MOUNT: u32 = 0x00000001;
pub const FM_DIRTY: u32 = 0x00000002;
pub const FM_LOGREDO: u32 = 0x00000004;
pub const FM_EXTENDFS: u32 = 0x00000008;
pub const FM_STATE_MAX: u32 = 0x0000000f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
