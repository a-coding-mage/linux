/* SPDX-License-Identifier: LGPL-2.1 */
/* Source-level Rust translation of xfs_fs.h. */

#[repr(C)] pub struct dioattr { pub d_mem: u32, pub d_miniosz: u32, pub d_maxiosz: u32 }
#[repr(C)] pub struct getbmap { pub bmv_offset:i64, pub bmv_block:i64, pub bmv_length:i64, pub bmv_count:i32, pub bmv_entries:i32 }
#[repr(C)] pub struct getbmapx { pub bmv_offset:i64,pub bmv_block:i64,pub bmv_length:i64,pub bmv_count:i32,pub bmv_entries:i32,pub bmv_iflags:i32,pub bmv_oflags:i32,pub bmv_unused1:i32,pub bmv_unused2:i32 }

pub const BMV_IF_ATTRFORK:u32=0x1; pub const BMV_IF_NO_DMAPI_READ:u32=0x2; pub const BMV_IF_PREALLOC:u32=0x4; pub const BMV_IF_DELALLOC:u32=0x8; pub const BMV_IF_NO_HOLES:u32=0x10; pub const BMV_IF_COWFORK:u32=0x20;
pub const BMV_IF_VALID:u32=BMV_IF_ATTRFORK|BMV_IF_NO_DMAPI_READ|BMV_IF_PREALLOC|BMV_IF_DELALLOC|BMV_IF_NO_HOLES|BMV_IF_COWFORK;
pub const BMV_OF_PREALLOC:u32=1; pub const BMV_OF_DELALLOC:u32=2; pub const BMV_OF_LAST:u32=4; pub const BMV_OF_SHARED:u32=8;

#[repr(C)] pub struct xfs_flock64_t { pub l_type:i16,pub l_whence:i16,pub l_start:i64,pub l_len:i64,pub l_sysid:i32,pub l_pid:u32,pub l_pad:[i32;4] }
#[repr(C)] pub struct xfs_fsop_geom_v1 { pub blocksize:u32,pub rtextsize:u32,pub agblocks:u32,pub agcount:u32,pub logblocks:u32,pub sectsize:u32,pub inodesize:u32,pub imaxpct:u32,pub datablocks:u64,pub rtblocks:u64,pub rtextents:u64,pub logstart:u64,pub uuid:[u8;16],pub sunit:u32,pub swidth:u32,pub version:i32,pub flags:u32,pub logsectsize:u32,pub rtsectsize:u32,pub dirblocksize:u32 }
#[repr(C)] pub struct xfs_fsop_geom_v4 { pub v1:xfs_fsop_geom_v1,pub logsunit:u32 }
#[repr(C)] pub struct xfs_fsop_geom { pub v4:xfs_fsop_geom_v4,pub sick:u32,pub checked:u32,pub rgextents:u32,pub rgcount:u32,pub rtstart:u64,pub rtreserved:u64,pub reserved:[u64;14] }
pub const XFS_FSOP_GEOM_VERSION:u32=0; pub const XFS_FSOP_GEOM_VERSION_V5:u32=5;
pub const XFS_MIN_AG_BLOCKS:u64=64; pub const XFS_MIN_LOG_BLOCKS:u64=512; pub const XFS_MAX_LOG_BLOCKS:u64=1024*1024; pub const XFS_MIN_LOG_BYTES:u64=10*1024*1024; pub const XFS_MIN_AG_BYTES:u64=1<<24; pub const XFS_MAX_AG_BYTES:u64=1<<40;

#[repr(C)] pub struct xfs_fsop_counts_t { pub freedata:u64,pub freertx:u64,pub freeino:u64,pub allocino:u64 }
#[repr(C)] pub struct xfs_fsop_resblks_t { pub resblks:u64,pub resblks_avail:u64 }
#[repr(C)] pub struct xfs_ag_geometry { pub ag_number:u32,pub ag_length:u32,pub ag_freeblks:u32,pub ag_icount:u32,pub ag_ifree:u32,pub ag_sick:u32,pub ag_checked:u32,pub ag_flags:u32,pub ag_reserved:[u64;12] }
#[repr(C)] pub struct xfs_growfs_data_t { pub newblocks:u64,pub imaxpct:u32 }
#[repr(C)] pub struct xfs_growfs_log_t { pub newblocks:u32,pub isint:u32 }
#[repr(C)] pub struct xfs_growfs_rt_t { pub newblocks:u64,pub extsize:u32 }
#[repr(C)] pub struct xfs_bstime_t { pub tv_sec:isize,pub tv_nsec:i32 }
#[repr(C)] pub struct xfs_bstat { pub bs_ino:u64,pub bs_mode:u16,pub bs_nlink:u16,pub bs_uid:u32,pub bs_gid:u32,pub bs_rdev:u32,pub bs_blksize:i32,pub bs_size:i64,pub bs_atime:xfs_bstime_t,pub bs_mtime:xfs_bstime_t,pub bs_ctime:xfs_bstime_t,pub bs_blocks:i64,pub bs_xflags:u32,pub bs_extsize:i32,pub bs_extents:i32,pub bs_gen:u32,pub bs_projid_lo:u16,pub bs_forkoff:u16,pub bs_projid_hi:u16,pub bs_sick:u16,pub bs_checked:u16,pub bs_pad:[u8;2],pub bs_cowextsize:u32,pub bs_dmevmask:u32,pub bs_dmstate:u16,pub bs_aextents:u16 }
pub type bs_projid = u16;
#[repr(C)] pub struct xfs_bulkstat { pub bs_ino:u64,pub bs_size:u64,pub bs_blocks:u64,pub bs_xflags:u64,pub bs_atime:i64,pub bs_mtime:i64,pub bs_ctime:i64,pub bs_btime:i64,pub bs_gen:u32,pub bs_uid:u32,pub bs_gid:u32,pub bs_projectid:u32,pub bs_atime_nsec:u32,pub bs_mtime_nsec:u32,pub bs_ctime_nsec:u32,pub bs_btime_nsec:u32,pub bs_blksize:u32,pub bs_rdev:u32,pub bs_cowextsize_blks:u32,pub bs_extsize_blks:u32,pub bs_nlink:u32,pub bs_extents:u32,pub bs_aextents:u32,pub bs_version:u16,pub bs_forkoff:u16,pub bs_sick:u16,pub bs_checked:u16,pub bs_mode:u16,pub bs_pad2:u16,pub bs_extents64:u64,pub bs_pad:[u64;6] }
pub const XFS_BULKSTAT_VERSION_V1:u32=1; pub const XFS_BULKSTAT_VERSION_V5:u32=5;
pub unsafe fn bstat_get_projid(bs:*const xfs_bstat)->u32 { ((*bs).bs_projid_hi as u32)<<16 | (*bs).bs_projid_lo as u32 }
#[repr(C)] pub struct xfs_fsop_bulkreq { pub lastip:*mut u64,pub icount:i32,pub ubuffer:*mut core::ffi::c_void,pub ocount:*mut i32 }
#[repr(C)] pub struct xfs_inogrp { pub xi_startino:u64,pub xi_alloccount:i32,pub xi_allocmask:u64 }
#[repr(C)] pub struct xfs_inumbers { pub xi_startino:u64,pub xi_allocmask:u64,pub xi_alloccount:u8,pub xi_version:u8,pub xi_padding:[u8;6] }
#[repr(C)] pub struct xfs_bulk_ireq { pub ino:u64,pub flags:u32,pub icount:u32,pub ocount:u32,pub agno:u32,pub reserved:[u64;5] }
pub const XFS_BULK_IREQ_AGNO:u32=1; pub const XFS_BULK_IREQ_SPECIAL:u32=2; pub const XFS_BULK_IREQ_NREXT64:u32=4; pub const XFS_BULK_IREQ_METADIR:u32=8; pub const XFS_BULK_IREQ_SPECIAL_ROOT:u32=1;
#[repr(C)] pub struct xfs_error_injection { pub fd:i32,pub errtag:i32 }
#[repr(C)] pub struct xfs_fs_eofblocks { pub eof_version:u32,pub eof_flags:u32,pub eof_uid:u32,pub eof_gid:u32,pub eof_prid:u32,pub pad32:u32,pub eof_min_file_size:u64,pub pad64:[u64;12] }
pub const XFS_EOFBLOCKS_VERSION:u32=1;
#[repr(C)] pub struct xfs_attrlist_cursor { pub opaque:[u32;4] }
#[repr(C)] pub struct xfs_fsid { pub val:[u32;2] }
#[repr(C)] pub struct xfs_fid { pub fid_len:u16,pub fid_pad:u16,pub fid_gen:u32,pub fid_ino:u64 }
#[repr(C)] pub union xfs_handle_union { pub align:i64,pub _ha_fsid:xfs_fsid }
#[repr(C)] pub struct xfs_handle { pub ha_u:xfs_handle_union,pub ha_fid:xfs_fid }
#[repr(C)] pub struct xfs_scrub_metadata { pub sm_type:u32,pub sm_flags:u32,pub sm_ino:u64,pub sm_gen:u32,pub sm_agno:u32,pub sm_reserved:[u64;5] }
#[repr(C)] pub struct xfs_scrub_vec { pub sv_type:u32,pub sv_flags:u32,pub sv_ret:i32,pub sv_reserved:u32 }
#[repr(C)] pub struct xfs_scrub_vec_head { pub svh_ino:u64,pub svh_gen:u32,pub svh_agno:u32,pub svh_flags:u32,pub svh_rest_us:u16,pub svh_nr:u16,pub svh_reserved:u64,pub svh_vectors:u64 }
#[repr(C)] pub struct xfs_exchange_range { pub file1_fd:i32,pub pad:u32,pub file1_offset:u64,pub file2_offset:u64,pub length:u64,pub flags:u64 }
#[repr(C)] pub struct xfs_commit_range { pub file1_fd:i32,pub pad:u32,pub file1_offset:u64,pub file2_offset:u64,pub length:u64,pub flags:u64,pub file2_freshness:[u64;6] }
pub const XFS_EXCHANGE_RANGE_TO_EOF:u64=1; pub const XFS_EXCHANGE_RANGE_DSYNC:u64=2; pub const XFS_EXCHANGE_RANGE_DRY_RUN:u64=4; pub const XFS_EXCHANGE_RANGE_FILE1_WRITTEN:u64=8;
#[repr(C)] pub struct xfs_rtgroup_geometry { pub rg_number:u32,pub rg_length:u32,pub rg_sick:u32,pub rg_checked:u32,pub rg_flags:u32,pub rg_writepointer:u32,pub rg_reserved:[u32;26] }
#[repr(C)] pub struct xfs_health_monitor_lost { pub count:u64 }
#[repr(C)] pub struct xfs_health_monitor_fs { pub mask:u32 }
#[repr(C)] pub struct xfs_health_monitor_group { pub mask:u32,pub gno:u32 }
#[repr(C)] pub struct xfs_health_monitor_inode { pub mask:u32,pub gen:u32,pub ino:u64 }
#[repr(C)] pub struct xfs_health_monitor_shutdown { pub reasons:u32 }
#[repr(C)] pub struct xfs_health_monitor_filerange { pub pos:u64,pub len:u64,pub ino:u64,pub gen:u32,pub error:u32 }
#[repr(C)] pub struct xfs_health_monitor_media { pub daddr:u64,pub bbcount:u64 }
#[repr(C)] pub union xfs_health_monitor_event_e { pub lost:xfs_health_monitor_lost,pub fs:xfs_health_monitor_fs,pub group:xfs_health_monitor_group,pub inode:xfs_health_monitor_inode,pub shutdown:xfs_health_monitor_shutdown,pub media:xfs_health_monitor_media,pub filerange:xfs_health_monitor_filerange }
#[repr(C)] pub struct xfs_health_monitor_event { pub domain:u32,pub type_:u32,pub time_ns:u64,pub e:xfs_health_monitor_event_e,pub pad:[u64;2] }
#[repr(C)] pub struct xfs_health_monitor { pub flags:u64,pub format:u8,pub pad:[u8;23] }
pub const XFS_HEALTH_MONITOR_VERBOSE:u64=1; pub const XFS_HEALTH_MONITOR_FMT_V0:u32=0;
#[repr(C)] pub struct xfs_verify_media { pub me_dev:u32,pub me_flags:u32,pub me_start_daddr:u64,pub me_end_daddr:u64,pub me_ioerror:u32,pub me_max_io_size:u32,pub me_rest_us:u32,pub me_pad:u32 }
#[repr(C)] pub struct xfs_device(pub u32); pub const XFS_DEV_DATA:u32=1; pub const XFS_DEV_LOG:u32=2; pub const XFS_DEV_RT:u32=3;
pub const BBSHIFT:u32=9; pub const BBSIZE:u64=1<<BBSHIFT; pub const BBMASK:u64=BBSIZE-1;
pub const fn btobb(bytes:u64)->u64 {(bytes+BBSIZE-1)>>BBSHIFT} pub const fn btobbt(bytes:u64)->u64 {bytes>>BBSHIFT} pub const fn bbtob(bbs:u64)->u64 {bbs<<BBSHIFT}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
