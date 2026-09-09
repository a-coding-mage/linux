/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Translated from gfs2_ondisk.h. Linux integer types and S_IFMT are supplied
// by the surrounding translation unit.

pub const GFS2_MAGIC: u32 = 0x01161970;
pub const GFS2_BASIC_BLOCK: u32 = 512;
pub const GFS2_BASIC_BLOCK_SHIFT: u32 = 9;

pub const GFS2_MOUNT_LOCK: u32 = 0;
pub const GFS2_LIVE_LOCK: u32 = 1;
pub const GFS2_FREEZE_LOCK: u32 = 2;
pub const GFS2_RENAME_LOCK: u32 = 3;
pub const GFS2_CONTROL_LOCK: u32 = 4;
pub const GFS2_MOUNTED_LOCK: u32 = 5;

pub const GFS2_FORMAT_NONE: u32 = 0;
pub const GFS2_FORMAT_SB: u32 = 100;
pub const GFS2_FORMAT_RG: u32 = 200;
pub const GFS2_FORMAT_RB: u32 = 300;
pub const GFS2_FORMAT_DI: u32 = 400;
pub const GFS2_FORMAT_IN: u32 = 500;
pub const GFS2_FORMAT_LF: u32 = 600;
pub const GFS2_FORMAT_JD: u32 = 700;
pub const GFS2_FORMAT_LH: u32 = 800;
pub const GFS2_FORMAT_LD: u32 = 900;
pub const GFS2_FORMAT_LB: u32 = 1000;
pub const GFS2_FORMAT_EA: u32 = 1600;
pub const GFS2_FORMAT_ED: u32 = 1700;
pub const GFS2_FORMAT_QC: u32 = 1400;
pub const GFS2_FORMAT_RI: u32 = 1100;
pub const GFS2_FORMAT_DE: u32 = 1200;
pub const GFS2_FORMAT_QU: u32 = 1500;
pub const GFS2_FORMAT_FS: u32 = 1802;
pub const GFS2_FORMAT_MULTI: u32 = 1900;

#[repr(C)]
pub struct gfs2_inum { pub no_formal_ino: u64, pub no_addr: u64 }

pub const GFS2_METATYPE_NONE: u32 = 0;
pub const GFS2_METATYPE_SB: u32 = 1;
pub const GFS2_METATYPE_RG: u32 = 2;
pub const GFS2_METATYPE_RB: u32 = 3;
pub const GFS2_METATYPE_DI: u32 = 4;
pub const GFS2_METATYPE_IN: u32 = 5;
pub const GFS2_METATYPE_LF: u32 = 6;
pub const GFS2_METATYPE_JD: u32 = 7;
pub const GFS2_METATYPE_LH: u32 = 8;
pub const GFS2_METATYPE_LD: u32 = 9;
pub const GFS2_METATYPE_LB: u32 = 12;
pub const GFS2_METATYPE_EA: u32 = 10;
pub const GFS2_METATYPE_ED: u32 = 11;
pub const GFS2_METATYPE_QC: u32 = 14;

#[repr(C)]
pub union gfs2_meta_header__bindgen_ty_1 { pub mh_jid: u32, pub __pad1: u32 }
#[repr(C)]
pub struct gfs2_meta_header {
    pub mh_magic: u32, pub mh_type: u32, pub __pad0: u64, pub mh_format: u32,
    pub __bindgen_anon_1: gfs2_meta_header__bindgen_ty_1,
}

pub const GFS2_SB_ADDR: u32 = 128;
pub const GFS2_SB_LOCK: u32 = 0;
pub const GFS2_LOCKNAME_LEN: usize = 64;
pub const GFS2_HAS_UUID: u32 = 1;

#[repr(C)]
pub struct gfs2_sb {
    pub sb_header: gfs2_meta_header, pub sb_fs_format: u32, pub sb_multihost_format: u32,
    pub __pad0: u32, pub sb_bsize: u32, pub sb_bsize_shift: u32, pub __pad1: u32,
    pub sb_master_dir: gfs2_inum, pub __pad2: gfs2_inum, pub sb_root_dir: gfs2_inum,
    pub sb_lockproto: [u8; GFS2_LOCKNAME_LEN], pub sb_locktable: [u8; GFS2_LOCKNAME_LEN],
    pub __pad3: gfs2_inum, pub __pad4: gfs2_inum, pub sb_uuid: [u8; 16],
}

#[repr(C)]
pub struct gfs2_rindex { pub ri_addr: u64, pub ri_length: u32, pub __pad: u32, pub ri_data0: u64, pub ri_data: u32, pub ri_bitbytes: u32, pub ri_reserved: [u8; 64] }

pub const GFS2_NBBY: u32 = 4;
pub const GFS2_BIT_SIZE: u32 = 2;
pub const GFS2_BIT_MASK: u32 = 0x00000003;
pub const GFS2_BLKST_FREE: u32 = 0;
pub const GFS2_BLKST_USED: u32 = 1;
pub const GFS2_BLKST_UNLINKED: u32 = 2;
pub const GFS2_BLKST_DINODE: u32 = 3;
pub const GFS2_RGF_JOURNAL: u32 = 0x1;
pub const GFS2_RGF_METAONLY: u32 = 0x2;
pub const GFS2_RGF_DATAONLY: u32 = 0x4;
pub const GFS2_RGF_NOALLOC: u32 = 0x8;
pub const GFS2_RGF_TRIMMED: u32 = 0x10;

#[repr(C)] pub struct gfs2_inode_lvb { pub ri_magic: u32, pub __pad: u32, pub ri_generation_deleted: u64 }
#[repr(C)] pub struct gfs2_rgrp_lvb { pub rl_magic: u32, pub rl_flags: u32, pub rl_free: u32, pub rl_dinodes: u32, pub rl_igeneration: u64, pub rl_unlinked: u32, pub __pad: u32 }
#[repr(C)]
pub union gfs2_rgrp__bindgen_ty_1 { pub __pad: u32, pub rg_skip: u32 }
#[repr(C)]
pub struct gfs2_rgrp { pub rg_header: gfs2_meta_header, pub rg_flags: u32, pub rg_free: u32, pub rg_dinodes: u32, pub __bindgen_anon_1: gfs2_rgrp__bindgen_ty_1, pub rg_igeneration: u64, pub rg_data0: u64, pub rg_data: u32, pub rg_bitbytes: u32, pub rg_crc: u32, pub rg_reserved: [u8; 60] }

#[repr(C)] pub struct gfs2_quota { pub qu_limit: u64, pub qu_warn: u64, pub qu_value: u64, pub qu_reserved: [u8; 64] }

pub const GFS2_MAX_META_HEIGHT: u32 = 10;
pub const GFS2_DIR_MAX_DEPTH: u32 = 17;
/* DT2IF(dt) = (((dt) << 12) & S_IFMT); IF2DT(sif) = (((sif) & S_IFMT) >> 12). */

#[repr(C)]
pub enum gfs2_dinode_flag {
    gfs2fl_Jdata = 0, gfs2fl_ExHash = 1, gfs2fl_Unused = 2, gfs2fl_EaIndirect = 3,
    gfs2fl_Directio = 4, gfs2fl_Immutable = 5, gfs2fl_AppendOnly = 6, gfs2fl_NoAtime = 7,
    gfs2fl_Sync = 8, gfs2fl_System = 9, gfs2fl_TopLevel = 10, gfs2fl_TruncInProg = 29,
    gfs2fl_InheritDirectio = 30, gfs2fl_InheritJdata = 31,
}

pub const GFS2_DIF_JDATA: u32 = 0x00000001; pub const GFS2_DIF_EXHASH: u32 = 0x00000002;
pub const GFS2_DIF_UNUSED: u32 = 0x00000004; pub const GFS2_DIF_EA_INDIRECT: u32 = 0x00000008;
pub const GFS2_DIF_DIRECTIO: u32 = 0x10; pub const GFS2_DIF_IMMUTABLE: u32 = 0x20;
pub const GFS2_DIF_APPENDONLY: u32 = 0x40; pub const GFS2_DIF_NOATIME: u32 = 0x80;
pub const GFS2_DIF_SYNC: u32 = 0x100; pub const GFS2_DIF_SYSTEM: u32 = 0x200;
pub const GFS2_DIF_TOPDIR: u32 = 0x400; pub const GFS2_DIF_TRUNC_IN_PROG: u32 = 0x20000000;
pub const GFS2_DIF_INHERIT_DIRECTIO: u32 = 0x40000000; pub const GFS2_DIF_INHERIT_JDATA: u32 = 0x80000000;

#[repr(C)]
pub struct gfs2_dinode { pub di_header: gfs2_meta_header, pub di_num: gfs2_inum, pub di_mode: u32, pub di_uid: u32, pub di_gid: u32, pub di_nlink: u32, pub di_size: u64, pub di_blocks: u64, pub di_atime: u64, pub di_mtime: u64, pub di_ctime: u64, pub di_major: u32, pub di_minor: u32, pub di_goal_meta: u64, pub di_goal_data: u64, pub di_generation: u64, pub di_flags: u32, pub di_payload_format: u32, pub __pad1: u16, pub di_height: u16, pub __pad2: u32, pub __pad3: u16, pub di_depth: u16, pub di_entries: u32, pub __pad4: gfs2_inum, pub di_eattr: u64, pub di_atime_nsec: u32, pub di_mtime_nsec: u32, pub di_ctime_nsec: u32, pub di_reserved: [u8; 44] }

pub const GFS2_FNAMESIZE: u32 = 255;
/* GFS2_DIRENT_SIZE(name_len) = (size_of::<gfs2_dirent>() + name_len + 7) & !7. */
/* GFS2_MIN_DIRENT_SIZE = GFS2_DIRENT_SIZE(1). */
#[repr(C)] pub struct gfs2_dirent__bindgen_ty_1__bindgen_ty_1 { pub de_cookie: u32, pub pad3: [u8; 8] }
#[repr(C)] pub union gfs2_dirent__bindgen_ty_1 { pub __pad: [u8; 12], pub __bindgen_anon_1: gfs2_dirent__bindgen_ty_1__bindgen_ty_1 }
#[repr(C)] pub struct gfs2_dirent { pub de_inum: gfs2_inum, pub de_hash: u32, pub de_rec_len: u16, pub de_name_len: u16, pub de_type: u16, pub de_rahead: u16, pub __bindgen_anon_1: gfs2_dirent__bindgen_ty_1 }

#[repr(C)] pub struct gfs2_leaf__bindgen_ty_1__bindgen_ty_1 { pub lf_inode: u64, pub lf_dist: u32, pub lf_nsec: u32, pub lf_sec: u64, pub lf_reserved2: [u8; 40] }
#[repr(C)] pub union gfs2_leaf__bindgen_ty_1 { pub lf_reserved: [u8; 64], pub __bindgen_anon_1: gfs2_leaf__bindgen_ty_1__bindgen_ty_1 }
#[repr(C)] pub struct gfs2_leaf { pub lf_header: gfs2_meta_header, pub lf_depth: u16, pub lf_entries: u16, pub lf_dirent_format: u32, pub lf_next: u64, pub __bindgen_anon_1: gfs2_leaf__bindgen_ty_1 }

pub const GFS2_EA_MAX_NAME_LEN: u32 = 255; pub const GFS2_EA_MAX_DATA_LEN: u32 = 65536;
pub const GFS2_EATYPE_UNUSED: u32 = 0; pub const GFS2_EATYPE_USR: u32 = 1; pub const GFS2_EATYPE_SYS: u32 = 2; pub const GFS2_EATYPE_SECURITY: u32 = 3; pub const GFS2_EATYPE_TRUSTED: u32 = 4; pub const GFS2_EATYPE_LAST: u32 = 4;
#[inline] pub const fn GFS2_EATYPE_VALID(x: u8) -> bool { x <= GFS2_EATYPE_LAST as u8 }
pub const GFS2_EAFLAG_LAST: u8 = 0x01;
#[repr(C)] pub struct gfs2_ea_header { pub ea_rec_len: u32, pub ea_data_len: u32, pub ea_name_len: u8, pub ea_type: u8, pub ea_flags: u8, pub ea_num_ptrs: u8, pub __pad: u32 }

pub const GFS2_LOG_HEAD_UNMOUNT: u32 = 0x1; pub const GFS2_LOG_HEAD_FLUSH_NORMAL: u32 = 0x2; pub const GFS2_LOG_HEAD_FLUSH_SYNC: u32 = 0x4; pub const GFS2_LOG_HEAD_FLUSH_SHUTDOWN: u32 = 0x8; pub const GFS2_LOG_HEAD_FLUSH_FREEZE: u32 = 0x10; pub const GFS2_LOG_HEAD_RECOVERY: u32 = 0x20; pub const GFS2_LOG_HEAD_USERSPACE: u32 = 0x80000000;
pub const GFS2_LFC_SHUTDOWN: u32 = 0x100; pub const GFS2_LFC_JDATA_WPAGES: u32 = 0x200; pub const GFS2_LFC_SET_FLAGS: u32 = 0x400; pub const GFS2_LFC_AIL_EMPTY_GL: u32 = 0x800; pub const GFS2_LFC_AIL_FLUSH: u32 = 0x1000; pub const GFS2_LFC_RGRP_GO_SYNC: u32 = 0x2000; pub const GFS2_LFC_INODE_GO_SYNC: u32 = 0x4000; pub const GFS2_LFC_INODE_GO_INVAL: u32 = 0x8000; pub const GFS2_LFC_FREEZE_GO_SYNC: u32 = 0x10000; pub const GFS2_LFC_KILL_SB: u32 = 0x20000; pub const GFS2_LFC_DO_SYNC: u32 = 0x40000; pub const GFS2_LFC_INPLACE_RESERVE: u32 = 0x80000; pub const GFS2_LFC_WRITE_INODE: u32 = 0x100000; pub const GFS2_LFC_MAKE_FS_RO: u32 = 0x200000; pub const GFS2_LFC_SYNC_FS: u32 = 0x400000; pub const GFS2_LFC_EVICT_INODE: u32 = 0x800000; pub const GFS2_LFC_TRANS_END: u32 = 0x1000000; pub const GFS2_LFC_LOGD_JFLUSH_REQD: u32 = 0x2000000; pub const GFS2_LFC_LOGD_AIL_FLUSH_REQD: u32 = 0x4000000;

#[repr(C)] pub struct gfs2_log_header { pub lh_header: gfs2_meta_header, pub lh_sequence: u64, pub lh_flags: u32, pub lh_tail: u32, pub lh_blkno: u32, pub lh_hash: u32, pub lh_crc: u32, pub lh_nsec: u32, pub lh_sec: u64, pub lh_addr: u64, pub lh_jinode: u64, pub lh_statfs_addr: u64, pub lh_quota_addr: u64, pub lh_local_total: u64, pub lh_local_free: u64, pub lh_local_dinodes: u64 }
/* LH_V1_SIZE = offsetofend(struct gfs2_log_header, lh_hash). */
pub const GFS2_LOG_DESC_METADATA: u32 = 300; pub const GFS2_LOG_DESC_REVOKE: u32 = 301; pub const GFS2_LOG_DESC_JDATA: u32 = 302;
#[repr(C)] pub struct gfs2_log_descriptor { pub ld_header: gfs2_meta_header, pub ld_type: u32, pub ld_length: u32, pub ld_data1: u32, pub ld_data2: u32, pub ld_reserved: [u8; 32] }

pub const GFS2_INUM_QUANTUM: u64 = 1048576;
#[repr(C)] pub struct gfs2_inum_range { pub ir_start: u64, pub ir_length: u64 }
#[repr(C)] pub struct gfs2_statfs_change { pub sc_total: u64, pub sc_free: u64, pub sc_dinodes: u64 }
pub const GFS2_QCF_USER: u32 = 0x1;
#[repr(C)] pub struct gfs2_quota_change { pub qc_change: u64, pub qc_flags: u32, pub qc_id: u32 }
#[repr(C)] pub struct gfs2_quota_lvb { pub qb_magic: u32, pub __pad: u32, pub qb_limit: u64, pub qb_warn: u64, pub qb_value: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
