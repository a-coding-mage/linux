/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of xfs_format.h.  Types supplied by dependent headers are
 * intentionally referenced but not defined here. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::mem::offset_of;

pub const XFS_SB_MAGIC: u32 = 0x58465342;
pub const XFS_SB_VERSION_1: u16 = 1;
pub const XFS_SB_VERSION_2: u16 = 2;
pub const XFS_SB_VERSION_3: u16 = 3;
pub const XFS_SB_VERSION_4: u16 = 4;
pub const XFS_SB_VERSION_5: u16 = 5;
pub const XFS_SB_VERSION_NUMBITS: u16 = 0x000f;
pub const XFS_SB_VERSION_ALLFBITS: u16 = 0xfff0;
pub const XFS_SB_VERSION_ATTRBIT: u16 = 0x0010;
pub const XFS_SB_VERSION_NLINKBIT: u16 = 0x0020;
pub const XFS_SB_VERSION_QUOTABIT: u16 = 0x0040;
pub const XFS_SB_VERSION_ALIGNBIT: u16 = 0x0080;
pub const XFS_SB_VERSION_DALIGNBIT: u16 = 0x0100;
pub const XFS_SB_VERSION_SHAREDBIT: u16 = 0x0200;
pub const XFS_SB_VERSION_LOGV2BIT: u16 = 0x0400;
pub const XFS_SB_VERSION_SECTORBIT: u16 = 0x0800;
pub const XFS_SB_VERSION_EXTFLGBIT: u16 = 0x1000;
pub const XFS_SB_VERSION_DIRV2BIT: u16 = 0x2000;
pub const XFS_SB_VERSION_BORGBIT: u16 = 0x4000;
pub const XFS_SB_VERSION_MOREBITSBIT: u16 = 0x8000;
pub const XFS_XATTR_SIZE_MAX: u32 = 1 << 16;
pub const XFSLABEL_MAX: usize = 12;

#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct xfs_buf { pub b_addr: *mut core::ffi::c_void }
#[repr(C)] pub struct xfs_ifork { _private: [u8; 0] }

pub type uuid_t = [u8; 16];
pub type __be16 = u16; pub type __be32 = u32; pub type __be64 = u64;
pub type __le32 = u32; pub type __u8 = u8; pub type __u32 = u32; pub type __s8 = i8;
pub type xfs_timestamp_t = __be64;

#[repr(C)] pub struct xfs_sb {
    pub sb_magicnum:u32, pub sb_blocksize:u32, pub sb_dblocks:u64, pub sb_rblocks:u64,
    pub sb_rextents:u64, pub sb_uuid:uuid_t, pub sb_logstart:u64, pub sb_rootino:u64,
    pub sb_rbmino:u64, pub sb_rsumino:u64, pub sb_rextsize:u32, pub sb_agblocks:u32,
    pub sb_agcount:u32, pub sb_rbmblocks:u32, pub sb_logblocks:u32, pub sb_versionnum:u16,
    pub sb_sectsize:u16, pub sb_inodesize:u16, pub sb_inopblock:u16,
    pub sb_fname:[u8;XFSLABEL_MAX], pub sb_blocklog:u8, pub sb_sectlog:u8,
    pub sb_inodelog:u8, pub sb_inopblog:u8, pub sb_agblklog:u8, pub sb_rextslog:u8,
    pub sb_inprogress:u8, pub sb_imax_pct:u8, pub sb_icount:u64, pub sb_ifree:u64,
    pub sb_fdblocks:u64, pub sb_frextents:u64, pub sb_uquotino:u64, pub sb_gquotino:u64,
    pub sb_qflags:u16, pub sb_flags:u8, pub sb_shared_vn:u8, pub sb_inoalignmt:u32,
    pub sb_unit:u32, pub sb_width:u32, pub sb_dirblklog:u8, pub sb_logsectlog:u8,
    pub sb_logsectsize:u16, pub sb_logsunit:u32, pub sb_features2:u32, pub sb_bad_features2:u32,
    pub sb_features_compat:u32, pub sb_features_ro_compat:u32, pub sb_features_incompat:u32,
    pub sb_features_log_incompat:u32, pub sb_crc:u32, pub sb_spino_align:u32,
    pub sb_pquotino:u64, pub sb_lsn:u64, pub sb_meta_uuid:uuid_t, pub sb_metadirino:u64,
    pub sb_rgcount:u32, pub sb_rgextents:u32, pub sb_rgblklog:u8, pub sb_pad:[u8;7],
    pub sb_rtstart:u64, pub sb_rtreserved:u64,
}

#[repr(C)] pub struct xfs_dsb {
    pub sb_magicnum:__be32, pub sb_blocksize:__be32, pub sb_dblocks:__be64,
    pub sb_rblocks:__be64, pub sb_rextents:__be64, pub sb_uuid:uuid_t,
    pub sb_logstart:__be64, pub sb_rootino:__be64, pub sb_rbmino:__be64, pub sb_rsumino:__be64,
    pub sb_rextsize:__be32, pub sb_agblocks:__be32, pub sb_agcount:__be32,
    pub sb_rbmblocks:__be32, pub sb_logblocks:__be32, pub sb_versionnum:__be16,
    pub sb_sectsize:__be16, pub sb_inodesize:__be16, pub sb_inopblock:__be16,
    pub sb_fname:[u8;XFSLABEL_MAX], pub sb_blocklog:u8, pub sb_sectlog:u8,
    pub sb_inodelog:u8, pub sb_inopblog:u8, pub sb_agblklog:u8, pub sb_rextslog:u8,
    pub sb_inprogress:u8, pub sb_imax_pct:u8, pub sb_icount:__be64, pub sb_ifree:__be64,
    pub sb_fdblocks:__be64, pub sb_frextents:__be64, pub sb_uquotino:__be64,
    pub sb_gquotino:__be64, pub sb_qflags:__be16, pub sb_flags:u8, pub sb_shared_vn:u8,
    pub sb_inoalignmt:__be32, pub sb_unit:__be32, pub sb_width:__be32, pub sb_dirblklog:u8,
    pub sb_logsectlog:u8, pub sb_logsectsize:__be16, pub sb_logsunit:__be32,
    pub sb_features2:__be32, pub sb_bad_features2:__be32, pub sb_features_compat:__be32,
    pub sb_features_ro_compat:__be32, pub sb_features_incompat:__be32,
    pub sb_features_log_incompat:__be32, pub sb_crc:__le32, pub sb_spino_align:__be32,
    pub sb_pquotino:__be64, pub sb_lsn:__be64, pub sb_meta_uuid:uuid_t, pub sb_metadirino:__be64,
    pub sb_rgcount:__be32, pub sb_rgextents:__be32, pub sb_rgblklog:u8, pub sb_pad:[u8;7],
    pub sb_rtstart:__be64, pub sb_rtreserved:__be64,
}

pub const XFS_SBF_NOFLAGS:u8=0; pub const XFS_SBF_READONLY:u8=1; pub const XFS_SB_MAX_SHARED_VN:u8=0;
#[inline] pub unsafe fn xfs_sb_version_num(s:&xfs_sb)->u16 { s.sb_versionnum & XFS_SB_VERSION_NUMBITS }
#[inline] pub unsafe fn xfs_sb_is_v5(s:&xfs_sb)->bool { xfs_sb_version_num(s)==XFS_SB_VERSION_5 }
#[inline] pub unsafe fn xfs_sb_has_mismatched_features2(s:&xfs_sb)->bool { s.sb_bad_features2!=s.sb_features2 }
#[inline] pub unsafe fn xfs_sb_version_hasmorebits(s:&xfs_sb)->bool { xfs_sb_is_v5(s)||(s.sb_versionnum&XFS_SB_VERSION_MOREBITSBIT)!=0 }
#[inline] pub unsafe fn xfs_sb_version_addattr(s:&mut xfs_sb){s.sb_versionnum|=XFS_SB_VERSION_ATTRBIT}
#[inline] pub unsafe fn xfs_sb_version_addquota(s:&mut xfs_sb){s.sb_versionnum|=XFS_SB_VERSION_QUOTABIT}
#[inline] pub unsafe fn xfs_sb_version_addattr2(s:&mut xfs_sb){s.sb_versionnum|=XFS_SB_VERSION_MOREBITSBIT;s.sb_features2|=8}
#[inline] pub unsafe fn xfs_sb_version_addprojid32(s:&mut xfs_sb){s.sb_versionnum|=XFS_SB_VERSION_MOREBITSBIT;s.sb_features2|=0x80}

pub const XFS_AGF_MAGIC:u32=0x58414746; pub const XFS_AGI_MAGIC:u32=0x58414749; pub const XFS_AGFL_MAGIC:u32=0x5841464c;
pub const XFS_AGF_VERSION:u32=1; pub const XFS_AGI_VERSION:u32=1; pub const XFS_AGI_UNLINKED_BUCKETS:usize=64;
pub const XFS_RTSB_MAGIC:u32=0x46726f67; pub const XFS_DINODE_MAGIC:u16=0x494e;
pub const XFS_DQUOT_MAGIC:u16=0x4451; pub const XFS_SYMLINK_MAGIC:u32=0x58534c4d;
pub const XFS_MAXLINK:u32=(1u32<<31)-1; pub const XFS_NLINK_PINNED:u32=!0;
pub const XFS_DINODE_MIN_LOG:u32=8; pub const XFS_DINODE_MAX_LOG:u32=11;
pub const XFS_MAX_RTEXTSIZE:u32=1024*1024*1024; pub const XFS_DFL_RTEXTSIZE:u32=64*1024; pub const XFS_MIN_RTEXTSIZE:u32=4*1024;

#[repr(C)] pub struct xfs_agf { pub agf_magicnum:__be32,pub agf_versionnum:__be32,pub agf_seqno:__be32,pub agf_length:__be32,pub agf_bno_root:__be32,pub agf_cnt_root:__be32,pub agf_rmap_root:__be32,pub agf_bno_level:__be32,pub agf_cnt_level:__be32,pub agf_rmap_level:__be32,pub agf_flfirst:__be32,pub agf_fllast:__be32,pub agf_flcount:__be32,pub agf_freeblks:__be32,pub agf_longest:__be32,pub agf_btreeblks:__be32,pub agf_uuid:uuid_t,pub agf_rmap_blocks:__be32,pub agf_refcount_blocks:__be32,pub agf_refcount_root:__be32,pub agf_refcount_level:__be32,pub agf_spare64:[__be64;14],pub agf_lsn:__be64,pub agf_crc:__be32,pub agf_spare2:__be32 }
#[repr(C)] pub struct xfs_agi { pub agi_magicnum:__be32,pub agi_versionnum:__be32,pub agi_seqno:__be32,pub agi_length:__be32,pub agi_count:__be32,pub agi_root:__be32,pub agi_level:__be32,pub agi_freecount:__be32,pub agi_newino:__be32,pub agi_dirino:__be32,pub agi_unlinked:[__be32;64],pub agi_uuid:uuid_t,pub agi_crc:__be32,pub agi_pad32:__be32,pub agi_lsn:__be64,pub agi_free_root:__be32,pub agi_free_level:__be32,pub agi_iblocks:__be32,pub agi_fblocks:__be32 }
#[repr(C,packed)] pub struct xfs_agfl {pub agfl_magicnum:__be32,pub agfl_seqno:__be32,pub agfl_uuid:uuid_t,pub agfl_lsn:__be64,pub agfl_crc:__be32}

#[repr(C)] pub struct xfs_legacy_timestamp {pub t_sec:__be32,pub t_nsec:__be32}
pub const XFS_LEGACY_TIME_MIN:i64=i32::MIN as i64; pub const XFS_LEGACY_TIME_MAX:i64=i32::MAX as i64; pub const XFS_BIGTIME_TIME_MIN:i64=0;
#[inline] pub fn xfs_unix_to_bigtime(x:i64)->u64 {(x as u64).wrapping_add(-(i32::MIN as i64) as u64)}
#[inline] pub fn xfs_bigtime_to_unix(x:u64)->i64 {(x as i64).wrapping_sub(-(i32::MIN as i64))}

#[repr(u32)] #[derive(Copy,Clone)] pub enum xfs_metafile_type { XFS_METAFILE_UNKNOWN, XFS_METAFILE_DIR, XFS_METAFILE_USRQUOTA, XFS_METAFILE_GRPQUOTA, XFS_METAFILE_PRJQUOTA, XFS_METAFILE_RTBITMAP, XFS_METAFILE_RTSUMMARY, XFS_METAFILE_RTRMAP, XFS_METAFILE_RTREFCOUNT, XFS_METAFILE_MAX }
#[repr(C)] pub struct xfs_rmap_rec {pub rm_startblock:__be32,pub rm_blockcount:__be32,pub rm_owner:__be64,pub rm_offset:__be64}
#[repr(C)] pub struct xfs_refcount_rec {pub rc_startblock:__be32,pub rc_blockcount:__be32,pub rc_refcount:__be32}
#[repr(C)] pub struct xfs_bmbt_rec {pub l0:__be64,pub l1:__be64}
#[repr(C)] pub struct xfs_bmdr_block {pub bb_level:__be16,pub bb_numrecs:__be16}

// Remaining declarations use dependent XFS typedefs and feature helpers exactly
// as supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
