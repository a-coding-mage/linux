// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// On-disk Log Format definitions.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub type xlog_tid_t = u32;

pub const XLOG_MIN_ICLOGS: u32 = 2;
pub const XLOG_MAX_ICLOGS: u32 = 8;
pub const XLOG_HEADER_MAGIC_NUM: u32 = 0xFEED_babe;
pub const XLOG_VERSION_1: u32 = 1;
pub const XLOG_VERSION_2: u32 = 2;
pub const XLOG_VERSION_OKBITS: u32 = XLOG_VERSION_1 | XLOG_VERSION_2;
pub const XLOG_MIN_RECORD_BSIZE: u32 = 16 * 1024;
pub const XLOG_BIG_RECORD_BSIZE: u32 = 32 * 1024;
pub const XLOG_MAX_RECORD_BSIZE: u32 = 256 * 1024;
pub const XLOG_HEADER_CYCLE_SIZE: u32 = 32 * 1024;
pub const XLOG_MIN_RECORD_BSHIFT: u32 = 14;
pub const XLOG_BIG_RECORD_BSHIFT: u32 = 15;
pub const XLOG_MAX_RECORD_BSHIFT: u32 = 18;
pub const XLOG_HEADER_SIZE: u32 = 512;
pub const XFS_MIN_LOG_FACTOR: u32 = 3;

// Build-time condition retained from C; the required external helpers/types are supplied by dependencies.
#[inline]
pub unsafe fn XLOG_REC_SHIFT(log: *mut xfs_log) -> u32 {
    BTOBB(1u32 << if xfs_has_logv2((*log).l_mp) { XLOG_MAX_RECORD_BSHIFT } else { XLOG_BIG_RECORD_BSHIFT })
}
#[inline]
pub unsafe fn XLOG_TOTAL_REC_SHIFT(log: *mut xfs_log) -> u32 {
    BTOBB(XLOG_MAX_ICLOGS << if xfs_has_logv2((*log).l_mp) { XLOG_MAX_RECORD_BSHIFT } else { XLOG_BIG_RECORD_BSHIFT })
}

#[inline] pub const fn CYCLE_LSN(lsn: u64) -> u32 { (lsn >> 32) as u32 }
#[inline] pub const fn BLOCK_LSN(lsn: u64) -> u32 { lsn as u32 }

#[inline]
pub fn XFS_LSN_CMP(lsn1: u64, lsn2: u64) -> i64 {
    if CYCLE_LSN(lsn1) != CYCLE_LSN(lsn2) { return if CYCLE_LSN(lsn1) < CYCLE_LSN(lsn2) { -999 } else { 999 }; }
    if BLOCK_LSN(lsn1) != BLOCK_LSN(lsn2) { return if BLOCK_LSN(lsn1) < BLOCK_LSN(lsn2) { -999 } else { 999 }; }
    0
}

#[inline] pub const fn xlog_assign_lsn(cycle: u32, block: u32) -> u64 { ((cycle as u64) << 32) | block as u64 }
#[inline]
pub unsafe fn xlog_get_cycle(ptr: *mut i8) -> u32 {
    let p = ptr as *mut u32;
    let first = be32_to_cpu(*p);
    if first == XLOG_HEADER_MAGIC_NUM { be32_to_cpu(*p.add(1)) } else { first }
}

pub const XFS_TRANSACTION: u32 = 0x69;
pub const XFS_LOG: u32 = 0xaa;
pub const XLOG_UNMOUNT_TYPE: u16 = 0x556e;

#[repr(C)]
pub struct xfs_unmount_log_format { pub magic: u16, pub pad1: u16, pub pad2: u32 }

pub const XLOG_START_TRANS: u32 = 0x01;
pub const XLOG_COMMIT_TRANS: u32 = 0x02;
pub const XLOG_CONTINUE_TRANS: u32 = 0x04;
pub const XLOG_WAS_CONT_TRANS: u32 = 0x08;
pub const XLOG_END_TRANS: u32 = 0x10;
pub const XLOG_UNMOUNT_TRANS: u32 = 0x20;

#[repr(C)] pub struct xlog_op_header { pub oh_tid: u32, pub oh_len: u32, pub oh_clientid: u8, pub oh_flags: u8, pub oh_res2: u16 }
pub const XLOG_FMT_UNKNOWN: u32 = 0;
pub const XLOG_FMT_LINUX_LE: u32 = 1;
pub const XLOG_FMT_LINUX_BE: u32 = 2;
pub const XLOG_FMT_IRIX_BE: u32 = 3;
// XLOG_FMT is XLOG_FMT_LINUX_BE when XFS_NATIVE_HOST is defined, otherwise XLOG_FMT_LINUX_LE.

pub const XLOG_CYCLE_DATA_SIZE: usize = (XLOG_HEADER_CYCLE_SIZE / BBSIZE) as usize;
#[repr(C)] pub struct xlog_rec_ext_header { pub xh_cycle: u32, pub xh_cycle_data: [u32; XLOG_CYCLE_DATA_SIZE], pub xh_reserved: [u8; 252] }
pub const XLOG_REC_EXT_SIZE: usize = core::mem::offset_of!(xlog_rec_ext_header, xh_cycle_data) + core::mem::size_of::<[u32; XLOG_CYCLE_DATA_SIZE]>();

#[repr(C)]
pub struct xlog_rec_header {
    pub h_magicno: u32, pub h_cycle: u32, pub h_version: u32, pub h_len: u32,
    pub h_lsn: u64, pub h_tail_lsn: u64, pub h_crc: u32, pub h_prev_block: u32,
    pub h_num_logops: u32, pub h_cycle_data: [u32; XLOG_CYCLE_DATA_SIZE],
    pub h_fmt: u32, pub h_fs_uuid: uuid_t, pub h_size: u32, pub h_pad0: u32,
    pub h_reserved: [u8; 184], pub h_ext: [xlog_rec_ext_header; 0],
}
// On i386 XLOG_REC_SIZE ends at h_size and XLOG_REC_SIZE_OTHER at h_pad0; otherwise the order is reversed.

#[repr(C)] pub struct xfs_trans_header { pub th_magic: u32, pub th_type: u32, pub th_tid: i32, pub th_num_items: u32 }
pub const XFS_TRANS_HEADER_MAGIC: u32 = 0x5452_414e;
pub const XFS_TRANS_CHECKPOINT: u32 = 40;

pub const XFS_LI_EFI: u32 = 0x1236; pub const XFS_LI_EFD: u32 = 0x1237; pub const XFS_LI_IUNLINK: u32 = 0x1238;
pub const XFS_LI_INODE: u32 = 0x123b; pub const XFS_LI_BUF: u32 = 0x123c; pub const XFS_LI_DQUOT: u32 = 0x123d;
pub const XFS_LI_QUOTAOFF: u32 = 0x123e; pub const XFS_LI_ICREATE: u32 = 0x123f; pub const XFS_LI_RUI: u32 = 0x1240;
pub const XFS_LI_RUD: u32 = 0x1241; pub const XFS_LI_CUI: u32 = 0x1242; pub const XFS_LI_CUD: u32 = 0x1243;
pub const XFS_LI_BUI: u32 = 0x1244; pub const XFS_LI_BUD: u32 = 0x1245; pub const XFS_LI_ATTRI: u32 = 0x1246;
pub const XFS_LI_ATTRD: u32 = 0x1247; pub const XFS_LI_XMI: u32 = 0x1248; pub const XFS_LI_XMD: u32 = 0x1249;
pub const XFS_LI_EFI_RT: u32 = 0x124a; pub const XFS_LI_EFD_RT: u32 = 0x124b; pub const XFS_LI_RUI_RT: u32 = 0x124c;
pub const XFS_LI_RUD_RT: u32 = 0x124d; pub const XFS_LI_CUI_RT: u32 = 0x124e; pub const XFS_LI_CUD_RT: u32 = 0x124f;

#[repr(C)] pub union xfs_inode_log_format_u { pub ilfu_rdev: u32, pub __pad: [u8; 16] }
#[repr(C)] pub struct xfs_inode_log_format { pub ilf_type: u16, pub ilf_size: u16, pub ilf_fields: u32, pub ilf_asize: u16, pub ilf_dsize: u16, pub ilf_pad: u32, pub ilf_ino: u64, pub ilf_u: xfs_inode_log_format_u, pub ilf_blkno: i64, pub ilf_len: i32, pub ilf_boffset: i32 }
#[repr(C, packed)] pub struct xfs_inode_log_format_32 { pub ilf_type: u16, pub ilf_size: u16, pub ilf_fields: u32, pub ilf_asize: u16, pub ilf_dsize: u16, pub ilf_ino: u64, pub ilf_u: xfs_inode_log_format_u, pub ilf_blkno: i64, pub ilf_len: i32, pub ilf_boffset: i32 }

pub const XFS_ILOG_CORE: u32=0x001; pub const XFS_ILOG_DDATA: u32=0x002; pub const XFS_ILOG_DEXT: u32=0x004; pub const XFS_ILOG_DBROOT: u32=0x008; pub const XFS_ILOG_DEV: u32=0x010; pub const XFS_ILOG_UUID: u32=0x020; pub const XFS_ILOG_ADATA: u32=0x040; pub const XFS_ILOG_AEXT: u32=0x080; pub const XFS_ILOG_ABROOT: u32=0x100; pub const XFS_ILOG_DOWNER: u32=0x200; pub const XFS_ILOG_AOWNER: u32=0x400; pub const XFS_ILOG_TIMESTAMP: u32=0x4000; pub const XFS_ILOG_IVERSION: u32=0x8000;
pub const XFS_ILOG_DFORK: u32 = XFS_ILOG_DDATA|XFS_ILOG_DEXT|XFS_ILOG_DBROOT;
pub const XFS_ILOG_AFORK: u32 = XFS_ILOG_ADATA|XFS_ILOG_AEXT|XFS_ILOG_ABROOT;
pub const XFS_ILOG_ALL: u32 = XFS_ILOG_CORE|XFS_ILOG_DDATA|XFS_ILOG_DEXT|XFS_ILOG_DBROOT|XFS_ILOG_DEV|XFS_ILOG_ADATA|XFS_ILOG_AEXT|XFS_ILOG_ABROOT|XFS_ILOG_TIMESTAMP|XFS_ILOG_DOWNER|XFS_ILOG_AOWNER;
#[inline] pub const fn xfs_ilog_fbroot(w: i32) -> u32 { if w == XFS_DATA_FORK { XFS_ILOG_DBROOT } else { XFS_ILOG_ABROOT } }
#[inline] pub const fn xfs_ilog_fext(w: i32) -> u32 { if w == XFS_DATA_FORK { XFS_ILOG_DEXT } else { XFS_ILOG_AEXT } }
#[inline] pub const fn xfs_ilog_fdata(w: i32) -> u32 { if w == XFS_DATA_FORK { XFS_ILOG_DDATA } else { XFS_ILOG_ADATA } }

pub type xfs_log_timestamp_t = u64;
#[repr(C)] pub struct xfs_log_legacy_timestamp { pub t_sec: i32, pub t_nsec: i32 }

#[repr(C)] pub struct xfs_log_dinode {
    pub di_magic:u16, pub di_mode:u16, pub di_version:i8, pub di_format:i8, pub di_metatype:u16,
    pub di_uid:u32, pub di_gid:u32, pub di_nlink:u32, pub di_projid_lo:u16, pub di_projid_hi:u16,
    pub di_big_nextents:u64, pub di_atime:xfs_log_timestamp_t, pub di_mtime:xfs_log_timestamp_t, pub di_ctime:xfs_log_timestamp_t,
    pub di_size:xfs_fsize_t, pub di_nblocks:xfs_rfsblock_t, pub di_extsize:xfs_extlen_t,
    pub di_nextents:u32, pub di_anextents:u16, pub di_forkoff:u8, pub di_aformat:i8, pub di_dmevmask:u32,
    pub di_dmstate:u16, pub di_flags:u16, pub di_gen:u32, pub di_next_unlinked:xfs_agino_t, pub di_crc:u32,
    pub di_changecount:u64, pub di_lsn:xfs_lsn_t, pub di_flags2:u64, pub di_cowextsize:u32, pub di_pad2:[u8;12],
    pub di_crtime:xfs_log_timestamp_t, pub di_ino:xfs_ino_t, pub di_uuid:uuid_t,
}
#[inline] pub unsafe fn xfs_log_dinode_size(mp: *mut xfs_mount) -> usize { if xfs_has_v3inodes(mp) { core::mem::size_of::<xfs_log_dinode>() } else { core::mem::offset_of!(xfs_log_dinode, di_next_unlinked) } }

pub const XFS_BLF_CHUNK:u32=128; pub const XFS_BLF_SHIFT:u32=7; pub const BIT_TO_WORD_SHIFT:u32=5;
pub const NBWORD:usize = 8 * core::mem::size_of::<u32>();
pub const XFS_BLF_INODE_BUF:u16=1<<0; pub const XFS_BLF_CANCEL:u16=1<<1; pub const XFS_BLF_UDQUOT_BUF:u16=1<<2; pub const XFS_BLF_PDQUOT_BUF:u16=1<<3; pub const XFS_BLF_GDQUOT_BUF:u16=1<<4;
pub const __XFS_BLF_DATAMAP_SIZE:usize = (XFS_MAX_BLOCKSIZE as usize / XFS_BLF_CHUNK as usize) / NBWORD;
pub const XFS_BLF_DATAMAP_SIZE:usize = __XFS_BLF_DATAMAP_SIZE + 1;
#[repr(C)] pub struct xfs_buf_log_format { pub blf_type:u16, pub blf_size:u16, pub blf_flags:u16, pub blf_len:u16, pub blf_blkno:i64, pub blf_map_size:u32, pub blf_data_map:[u32;XFS_BLF_DATAMAP_SIZE] }
pub const XFS_BLFT_BITS:u32=5; pub const XFS_BLFT_SHIFT:u32=11; pub const XFS_BLFT_MASK:u16=(((1<<XFS_BLFT_BITS)-1)<<XFS_BLFT_SHIFT) as u16;
#[repr(C)] #[derive(Copy,Clone)] pub enum xfs_blft { XFS_BLFT_UNKNOWN_BUF=0, XFS_BLFT_UDQUOT_BUF, XFS_BLFT_PDQUOT_BUF, XFS_BLFT_GDQUOT_BUF, XFS_BLFT_BTREE_BUF, XFS_BLFT_AGF_BUF, XFS_BLFT_AGFL_BUF, XFS_BLFT_AGI_BUF, XFS_BLFT_DINO_BUF, XFS_BLFT_SYMLINK_BUF, XFS_BLFT_DIR_BLOCK_BUF, XFS_BLFT_DIR_DATA_BUF, XFS_BLFT_DIR_FREE_BUF, XFS_BLFT_DIR_LEAF1_BUF, XFS_BLFT_DIR_LEAFN_BUF, XFS_BLFT_DA_NODE_BUF, XFS_BLFT_ATTR_LEAF_BUF, XFS_BLFT_ATTR_RMT_BUF, XFS_BLFT_SB_BUF, XFS_BLFT_RTBITMAP_BUF, XFS_BLFT_RTSUMMARY_BUF, XFS_BLFT_MAX_BUF=1<<XFS_BLFT_BITS }
#[inline] pub unsafe fn xfs_blft_to_flags(blf:*mut xfs_buf_log_format, typ:xfs_blft) { ASSERT((typ as u32)>0 && (typ as u32)<(1<<XFS_BLFT_BITS)); (*blf).blf_flags &= !XFS_BLFT_MASK; (*blf).blf_flags |= ((typ as u16)<<XFS_BLFT_SHIFT)&XFS_BLFT_MASK; }
#[inline] pub unsafe fn xfs_blft_from_flags(blf:*mut xfs_buf_log_format)->u16 { ((*blf).blf_flags & XFS_BLFT_MASK)>>XFS_BLFT_SHIFT }

#[repr(C)] pub struct xfs_extent { pub ext_start:xfs_fsblock_t, pub ext_len:xfs_extlen_t }
#[repr(C,packed)] pub struct xfs_extent_32 { pub ext_start:u64, pub ext_len:u32 }
#[repr(C)] pub struct xfs_extent_64 { pub ext_start:u64, pub ext_len:u32, pub ext_pad:u32 }
#[repr(C)] pub struct xfs_efi_log_format { pub efi_type:u16,pub efi_size:u16,pub efi_nextents:u32,pub efi_id:u64,pub efi_extents:[xfs_extent;0] }
#[inline] pub const fn xfs_efi_log_format_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efi_log_format>()+nr*core::mem::size_of::<xfs_extent>() }
#[repr(C,packed)] pub struct xfs_efi_log_format_32 { pub efi_type:u16,pub efi_size:u16,pub efi_nextents:u32,pub efi_id:u64,pub efi_extents:[xfs_extent_32;0] }
#[inline] pub const fn xfs_efi_log_format32_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efi_log_format_32>()+nr*core::mem::size_of::<xfs_extent_32>() }
#[repr(C)] pub struct xfs_efi_log_format_64 { pub efi_type:u16,pub efi_size:u16,pub efi_nextents:u32,pub efi_id:u64,pub efi_extents:[xfs_extent_64;0] }
#[inline] pub const fn xfs_efi_log_format64_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efi_log_format_64>()+nr*core::mem::size_of::<xfs_extent_64>() }
#[repr(C)] pub struct xfs_efd_log_format { pub efd_type:u16,pub efd_size:u16,pub efd_nextents:u32,pub efd_efi_id:u64,pub efd_extents:[xfs_extent;0] }
#[inline] pub const fn xfs_efd_log_format_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efd_log_format>()+nr*core::mem::size_of::<xfs_extent>() }
#[repr(C,packed)] pub struct xfs_efd_log_format_32 { pub efd_type:u16,pub efd_size:u16,pub efd_nextents:u32,pub efd_efi_id:u64,pub efd_extents:[xfs_extent_32;0] }
#[inline] pub const fn xfs_efd_log_format32_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efd_log_format_32>()+nr*core::mem::size_of::<xfs_extent_32>() }
#[repr(C)] pub struct xfs_efd_log_format_64 { pub efd_type:u16,pub efd_size:u16,pub efd_nextents:u32,pub efd_efi_id:u64,pub efd_extents:[xfs_extent_64;0] }
#[inline] pub const fn xfs_efd_log_format64_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_efd_log_format_64>()+nr*core::mem::size_of::<xfs_extent_64>() }

#[repr(C)] pub struct xfs_map_extent { pub me_owner:u64,pub me_startblock:u64,pub me_startoff:u64,pub me_len:u32,pub me_flags:u32 }
pub const XFS_RMAP_EXTENT_MAP:u32=1; pub const XFS_RMAP_EXTENT_MAP_SHARED:u32=2; pub const XFS_RMAP_EXTENT_UNMAP:u32=3; pub const XFS_RMAP_EXTENT_UNMAP_SHARED:u32=4; pub const XFS_RMAP_EXTENT_CONVERT:u32=5; pub const XFS_RMAP_EXTENT_CONVERT_SHARED:u32=6; pub const XFS_RMAP_EXTENT_ALLOC:u32=7; pub const XFS_RMAP_EXTENT_FREE:u32=8; pub const XFS_RMAP_EXTENT_TYPE_MASK:u32=0xff; pub const XFS_RMAP_EXTENT_ATTR_FORK:u32=1<<31; pub const XFS_RMAP_EXTENT_BMBT_BLOCK:u32=1<<30; pub const XFS_RMAP_EXTENT_UNWRITTEN:u32=1<<29; pub const XFS_RMAP_EXTENT_FLAGS:u32=XFS_RMAP_EXTENT_TYPE_MASK|XFS_RMAP_EXTENT_ATTR_FORK|XFS_RMAP_EXTENT_BMBT_BLOCK|XFS_RMAP_EXTENT_UNWRITTEN;
#[repr(C)] pub struct xfs_rui_log_format { pub rui_type:u16,pub rui_size:u16,pub rui_nextents:u32,pub rui_id:u64,pub rui_extents:[xfs_map_extent;0] }
#[inline] pub const fn xfs_rui_log_format_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_rui_log_format>()+nr*core::mem::size_of::<xfs_map_extent>() }
#[repr(C)] pub struct xfs_rud_log_format { pub rud_type:u16,pub rud_size:u16,pub __pad:u32,pub rud_rui_id:u64 }
#[repr(C)] pub struct xfs_phys_extent { pub pe_startblock:u64,pub pe_len:u32,pub pe_flags:u32 }
pub const XFS_REFCOUNT_EXTENT_TYPE_MASK:u32=0xff; pub const XFS_REFCOUNT_EXTENT_FLAGS:u32=XFS_REFCOUNT_EXTENT_TYPE_MASK;
#[repr(C)] pub struct xfs_cui_log_format { pub cui_type:u16,pub cui_size:u16,pub cui_nextents:u32,pub cui_id:u64,pub cui_extents:[xfs_phys_extent;0] }
#[inline] pub const fn xfs_cui_log_format_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_cui_log_format>()+nr*core::mem::size_of::<xfs_phys_extent>() }
#[repr(C)] pub struct xfs_cud_log_format { pub cud_type:u16,pub cud_size:u16,pub __pad:u32,pub cud_cui_id:u64 }
pub const XFS_BMAP_EXTENT_TYPE_MASK:u32=0xff; pub const XFS_BMAP_EXTENT_ATTR_FORK:u32=1<<31; pub const XFS_BMAP_EXTENT_UNWRITTEN:u32=1<<30; pub const XFS_BMAP_EXTENT_REALTIME:u32=1<<29; pub const XFS_BMAP_EXTENT_FLAGS:u32=XFS_BMAP_EXTENT_TYPE_MASK|XFS_BMAP_EXTENT_ATTR_FORK|XFS_BMAP_EXTENT_UNWRITTEN|XFS_BMAP_EXTENT_REALTIME;
#[repr(C)] pub struct xfs_bui_log_format { pub bui_type:u16,pub bui_size:u16,pub bui_nextents:u32,pub bui_id:u64,pub bui_extents:[xfs_map_extent;0] }
#[inline] pub const fn xfs_bui_log_format_sizeof(nr:usize)->usize { core::mem::size_of::<xfs_bui_log_format>()+nr*core::mem::size_of::<xfs_map_extent>() }
#[repr(C)] pub struct xfs_bud_log_format { pub bud_type:u16,pub bud_size:u16,pub __pad:u32,pub bud_bui_id:u64 }

#[repr(C)] pub struct xfs_xmi_log_format { pub xmi_type:u16,pub xmi_size:u16,pub __pad:u32,pub xmi_id:u64,pub xmi_inode1:u64,pub xmi_inode2:u64,pub xmi_igen1:u32,pub xmi_igen2:u32,pub xmi_startoff1:u64,pub xmi_startoff2:u64,pub xmi_blockcount:u64,pub xmi_flags:u64,pub xmi_isize1:u64,pub xmi_isize2:u64 }
pub const XFS_EXCHMAPS_ATTR_FORK:u64=1<<0; pub const XFS_EXCHMAPS_SET_SIZES:u64=1<<1; pub const XFS_EXCHMAPS_INO1_WRITTEN:u64=1<<2; pub const XFS_EXCHMAPS_CLEAR_INO1_REFLINK:u64=1<<3; pub const XFS_EXCHMAPS_CLEAR_INO2_REFLINK:u64=1<<4; pub const XFS_EXCHMAPS_LOGGED_FLAGS:u64=XFS_EXCHMAPS_ATTR_FORK|XFS_EXCHMAPS_SET_SIZES|XFS_EXCHMAPS_INO1_WRITTEN|XFS_EXCHMAPS_CLEAR_INO1_REFLINK|XFS_EXCHMAPS_CLEAR_INO2_REFLINK;
#[repr(C)] pub struct xfs_xmd_log_format { pub xmd_type:u16,pub xmd_size:u16,pub __pad:u32,pub xmd_xmi_id:u64 }

#[repr(C)] pub struct xfs_dq_logformat { pub qlf_type:u16,pub qlf_size:u16,pub qlf_id:xfs_dqid_t,pub qlf_blkno:i64,pub qlf_len:i32,pub qlf_boffset:u32 }
#[repr(C)] pub struct xfs_qoff_logformat { pub qf_type:u16,pub qf_size:u16,pub qf_flags:u32,pub qf_pad:[i8;12] }
pub const XFS_UQUOTA_ACCT:u16=0x0001; pub const XFS_UQUOTA_ENFD:u16=0x0002; pub const XFS_UQUOTA_CHKD:u16=0x0004; pub const XFS_PQUOTA_ACCT:u16=0x0008; pub const XFS_OQUOTA_ENFD:u16=0x0010; pub const XFS_OQUOTA_CHKD:u16=0x0020; pub const XFS_GQUOTA_ACCT:u16=0x0040; pub const XFS_GQUOTA_ENFD:u16=0x0080; pub const XFS_GQUOTA_CHKD:u16=0x0100; pub const XFS_PQUOTA_ENFD:u16=0x0200; pub const XFS_PQUOTA_CHKD:u16=0x0400;
pub const XFS_ALL_QUOTA_ACCT:u16=XFS_UQUOTA_ACCT|XFS_GQUOTA_ACCT|XFS_PQUOTA_ACCT; pub const XFS_ALL_QUOTA_ENFD:u16=XFS_UQUOTA_ENFD|XFS_GQUOTA_ENFD|XFS_PQUOTA_ENFD; pub const XFS_ALL_QUOTA_CHKD:u16=XFS_UQUOTA_CHKD|XFS_GQUOTA_CHKD|XFS_PQUOTA_CHKD; pub const XFS_MOUNT_QUOTA_ALL:u16=XFS_UQUOTA_ACCT|XFS_UQUOTA_ENFD|XFS_UQUOTA_CHKD|XFS_GQUOTA_ACCT|XFS_GQUOTA_ENFD|XFS_GQUOTA_CHKD|XFS_PQUOTA_ACCT|XFS_PQUOTA_ENFD|XFS_PQUOTA_CHKD;
#[repr(C)] pub struct xfs_icreate_log { pub icl_type:u16,pub icl_size:u16,pub icl_ag:u32,pub icl_agbno:u32,pub icl_count:u32,pub icl_isize:u32,pub icl_length:u32,pub icl_gen:u32 }
pub const XFS_ATTRI_OP_FLAGS_SET:u32=1; pub const XFS_ATTRI_OP_FLAGS_REMOVE:u32=2; pub const XFS_ATTRI_OP_FLAGS_REPLACE:u32=3; pub const XFS_ATTRI_OP_FLAGS_PPTR_SET:u32=4; pub const XFS_ATTRI_OP_FLAGS_PPTR_REMOVE:u32=5; pub const XFS_ATTRI_OP_FLAGS_PPTR_REPLACE:u32=6; pub const XFS_ATTRI_OP_FLAGS_TYPE_MASK:u32=0xff;
pub const XFS_ATTRI_FILTER_MASK:u32=XFS_ATTR_ROOT|XFS_ATTR_SECURE|XFS_ATTR_PARENT|XFS_ATTR_INCOMPLETE;
#[repr(C)] pub union xfs_attri_log_format_name { pub alfi_name_len:u32, pub names:xfs_attri_log_format_names }
#[repr(C)] pub struct xfs_attri_log_format_names { pub alfi_old_name_len:u16,pub alfi_new_name_len:u16 }
#[repr(C)] pub struct xfs_attri_log_format { pub alfi_type:u16,pub alfi_size:u16,pub alfi_igen:u32,pub alfi_id:u64,pub alfi_ino:u64,pub alfi_op_flags:u32,pub alfi_name:xfs_attri_log_format_name,pub alfi_value_len:u32,pub alfi_attr_filter:u32 }
#[repr(C)] pub struct xfs_attrd_log_format { pub alfd_type:u16,pub alfd_size:u16,pub __pad:u32,pub alfd_alf_id:u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
