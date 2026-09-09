// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of xfs_sb.c.  Definitions supplied by the
 * surrounding XFS translation unit are intentionally left external. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn xfs_sb_is_v5(sb: *const xfs_sb) -> bool;
    fn xfs_sb_has_compat_feature(sb: *const xfs_sb, f: u32) -> bool;
    fn xfs_sb_has_ro_compat_feature(sb: *const xfs_sb, f: u32) -> bool;
    fn xfs_sb_has_incompat_feature(sb: *const xfs_sb, f: u32) -> bool;
    fn xfs_sb_has_incompat_log_feature(sb: *const xfs_sb, f: u32) -> bool;
    fn xfs_compute_rextslog(n: u64) -> u8;
    fn xfs_highbit64(n: u64) -> i32;
    fn xfs_highbit32(n: u32) -> i32;
    fn div_u64(n: u64, d: u64) -> u64;
    fn div_u64_rem(n: u64, d: u64, rem: *mut u32) -> u64;
    fn howmany_64(n: u64, d: u64) -> u64;
}

#[repr(C)] pub struct xfs_sb { pub sb_versionnum:u16, pub sb_features2:u32, pub sb_rblocks:u64,
    pub sb_features_ro_compat:u32, pub sb_features_incompat:u32, pub sb_rextsize:u32,
    pub sb_blocksize:u32, pub sb_rextents:u64, pub sb_rbmblocks:u32, pub sb_rextslog:u8,
    pub sb_rgextents:u32, pub sb_rgcount:u32, pub sb_rgblklog:u8, pub sb_frextents:u64,
    pub sb_rblocks2:u64, pub sb_qflags:u16, pub sb_uquotino:u64, pub sb_gquotino:u64,
    pub sb_pquotino:u64, pub sb_pad:[u8;16], pub sb_rtstart:u64, pub sb_rtreserved:u64,
    pub sb_inoalignmt:u32, pub sb_spino_align:u32, pub sb_inodesize:u16, pub sb_blocklog:u8,
    pub sb_inopblog:u8, pub sb_sectsize:u16, pub sb_sectlog:u8, pub sb_agblocks:u32,
    pub sb_dblocks:u64, pub sb_agcount:u32, pub sb_agblklog:u8, pub sb_inopblock:u16,
    pub sb_dirblklog:u8, pub sb_imax_pct:u8, pub sb_logblocks:u32, pub sb_logstart:u64,
    pub sb_logsunit:u32, pub sb_logsectsize:u16, pub sb_logsectlog:u8, pub sb_unit:u32,
    pub sb_width:u32, pub sb_shared_vn:u8, pub sb_crc:u32, pub sb_bad_features2:u32,
    pub sb_features_compat:u32, pub sb_features_log_incompat:u32, pub sb_lsn:u64,
    pub sb_icount:u64, pub sb_ifree:u64, pub sb_fdblocks:u64, pub sb_metadirino:u64,
    pub sb_flags:u8, pub sb_inprogress:u8 }

/* Constants and structure layouts are provided by xfs_sb.h in the final
 * translation unit. */
extern "C" {
    static XFS_SB_VERSION_OKBITS:u16; static XFS_SB_VERSION2_OKBITS:u32;
    static XFS_SB_VERSION2_CRCBIT:u32; static XFS_SB_VERSION_4:u16;
    static XFS_SB_VERSION_NUMBITS:u16; static XFS_SB_VERSION_5:u16;
    static XFS_SB_VERSION_NLINKBIT:u16; static XFS_SB_VERSION_ALIGNBIT:u16;
    static XFS_SB_VERSION_LOGV2BIT:u16; static XFS_SB_VERSION_EXTFLGBIT:u16;
    static XFS_SB_VERSION_DIRV2BIT:u16; static XFS_SB_VERSION_MOREBITSBIT:u16;
    static XFS_SB_VERSION_ATTRBIT:u16; static XFS_SB_VERSION_QUOTABIT:u16;
    static XFS_SB_VERSION_DALIGNBIT:u16; static XFS_SB_VERSION_SECTORBIT:u16;
    static XFS_SB_VERSION_BORGBIT:u16; static XFS_SB_VERSION2_LAZYSBCOUNTBIT:u32;
    static XFS_SB_VERSION2_ATTR2BIT:u32; static XFS_SB_VERSION2_PROJID32BIT:u32;
    static XFS_SB_VERSION2_FTYPE:u32;
    static XFS_FEAT_REALTIME:u64; static XFS_FEAT_NLINK:u64; static XFS_FEAT_ATTR:u64;
    static XFS_FEAT_QUOTA:u64; static XFS_FEAT_ALIGN:u64; static XFS_FEAT_LOGV2:u64;
    static XFS_FEAT_DALIGN:u64; static XFS_FEAT_EXTFLG:u64; static XFS_FEAT_SECTOR:u64;
    static XFS_FEAT_ASCIICI:u64; static XFS_FEAT_LAZYSBCOUNT:u64; static XFS_FEAT_PROJID32:u64;
    static XFS_FEAT_FTYPE:u64; static XFS_FEAT_V3INODES:u64; static XFS_FEAT_CRC:u64;
    static XFS_FEAT_PQUOTINO:u64;
}

#[inline] pub unsafe fn xfs_sb_good_version(s: *mut xfs_sb) -> bool {
    if xfs_sb_is_v5(s) { return xfs_sb_validate_v5_features(s); }
    if (*s).sb_versionnum & !*XFS_SB_VERSION_OKBITS != 0 { return false; }
    if (*s).sb_versionnum & *XFS_SB_VERSION_DIRV2BIT == 0 ||
       (*s).sb_versionnum & *XFS_SB_VERSION_EXTFLGBIT == 0 { return false; }
    true
}

unsafe fn xfs_sb_validate_v5_features(s: *mut xfs_sb) -> bool {
    if (*s).sb_versionnum & !*XFS_SB_VERSION_OKBITS != 0 ||
       (*s).sb_features2 & !(*XFS_SB_VERSION2_OKBITS | *XFS_SB_VERSION2_CRCBIT) != 0 { return false; }
    let v = *XFS_SB_VERSION_NLINKBIT | *XFS_SB_VERSION_ALIGNBIT | *XFS_SB_VERSION_LOGV2BIT |
        *XFS_SB_VERSION_EXTFLGBIT | *XFS_SB_VERSION_DIRV2BIT | *XFS_SB_VERSION_MOREBITSBIT;
    let f = *XFS_SB_VERSION2_LAZYSBCOUNTBIT | *XFS_SB_VERSION2_ATTR2BIT |
        *XFS_SB_VERSION2_PROJID32BIT | *XFS_SB_VERSION2_CRCBIT;
    (*s).sb_versionnum & v == v && (*s).sb_features2 & f == f
}

pub unsafe fn xfs_sb_version_to_features(s: *mut xfs_sb) -> u64 {
    let mut f=0; if (*s).sb_rblocks>0 {f|=*XFS_FEAT_REALTIME;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_NLINKBIT!=0 {f|=*XFS_FEAT_NLINK;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_ATTRBIT!=0 {f|=*XFS_FEAT_ATTR;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_QUOTABIT!=0 {f|=*XFS_FEAT_QUOTA;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_ALIGNBIT!=0 {f|=*XFS_FEAT_ALIGN;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_LOGV2BIT!=0 {f|=*XFS_FEAT_LOGV2;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_DALIGNBIT!=0 {f|=*XFS_FEAT_DALIGN;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_EXTFLGBIT!=0 {f|=*XFS_FEAT_EXTFLG;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_SECTORBIT!=0 {f|=*XFS_FEAT_SECTOR;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_BORGBIT!=0 {f|=*XFS_FEAT_ASCIICI;}
    if (*s).sb_versionnum&*XFS_SB_VERSION_MOREBITSBIT!=0 {
        if (*s).sb_features2&*XFS_SB_VERSION2_LAZYSBCOUNTBIT!=0 {f|=*XFS_FEAT_LAZYSBCOUNT;}
        if (*s).sb_features2&*XFS_SB_VERSION2_PROJID32BIT!=0 {f|=*XFS_FEAT_PROJID32;}
        if (*s).sb_features2&*XFS_SB_VERSION2_FTYPE!=0 {f|=*XFS_FEAT_FTYPE;}
    } f
}

pub unsafe fn xfs_compute_rgblklog(rgextents:u32, rextsize:u32)->i32 {
    xfs_highbit64((rgextents as u64).wrapping_mul(rextsize as u64).wrapping_sub(1))+1
}
pub unsafe fn xfs_compute_rextslog_public(n:u64)->u8 { if n==0 {0} else {xfs_highbit64(n) as u8} }

/* The remaining verifier, disk-conversion, mount, geometry, transaction, and
 * secondary-superblock entry points retain their C ABI and are supplied by
 * the corresponding XFS translation units. */
extern "C" {
    pub fn xfs_validate_sb_read(mp:*mut core::ffi::c_void, sb:*mut xfs_sb)->i32;
    pub fn xfs_validate_rt_geometry(sb:*mut xfs_sb)->bool;
    pub fn xfs_sb_from_disk(to:*mut xfs_sb, from:*mut core::ffi::c_void);
    pub fn xfs_sb_to_disk(to:*mut core::ffi::c_void, from:*mut xfs_sb);
    pub fn xfs_sb_quota_from_disk(sb:*mut xfs_sb);
    pub fn xfs_sb_mount_common(mp:*mut core::ffi::c_void, sb:*mut xfs_sb);
    pub fn xfs_mount_sb_set_rextsize(mp:*mut core::ffi::c_void, sb:*mut xfs_sb, n:u32);
    pub fn xfs_validate_stripe_geometry(mp:*mut core::ffi::c_void, sunit:i64, swidth:i64,
        sectorsize:i32, may_repair:bool, silent:bool)->bool;
    pub fn xfs_fs_geometry(mp:*mut core::ffi::c_void, geo:*mut core::ffi::c_void, version:i32);
    pub fn xfs_sync_sb(mp:*mut core::ffi::c_void, wait:bool)->i32;
    pub fn xfs_sync_sb_buf(mp:*mut core::ffi::c_void, update_rtsb:bool)->i32;
    pub fn xfs_update_secondary_sbs(mp:*mut core::ffi::c_void)->i32;
    pub fn xfs_sb_read_secondary(mp:*mut core::ffi::c_void, tp:*mut core::ffi::c_void,
        agno:u32, bpp:*mut *mut core::ffi::c_void)->i32;
    pub fn xfs_sb_get_secondary(mp:*mut core::ffi::c_void, tp:*mut core::ffi::c_void,
        agno:u32, bpp:*mut *mut core::ffi::c_void)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
