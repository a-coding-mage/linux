// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_mount.h. External kernel types and functions are supplied elsewhere. */

pub const XFS_LOWSP_1_PCNT: usize = 0;
pub const XFS_LOWSP_2_PCNT: usize = 1;
pub const XFS_LOWSP_3_PCNT: usize = 2;
pub const XFS_LOWSP_4_PCNT: usize = 3;
pub const XFS_LOWSP_5_PCNT: usize = 4;
pub const XFS_LOWSP_MAX: usize = 5;
pub const XFS_ERR_METADATA: usize = 0;
pub const XFS_ERR_CLASS_MAX: usize = 1;
pub const XFS_ERR_DEFAULT: usize = 0;
pub const XFS_ERR_EIO: usize = 1;
pub const XFS_ERR_ENOSPC: usize = 2;
pub const XFS_ERR_ENODEV: usize = 3;
pub const XFS_ERR_ERRNO_MAX: usize = 4;
pub const XFS_ERR_RETRY_FOREVER: i32 = -1;

#[repr(C)] pub struct xfs_error_cfg { pub kobj: xfs_kobj, pub max_retries: i32, pub retry_timeout: libc::c_long }
#[repr(C)] pub struct xfs_inodegc { pub mp: *mut xfs_mount, pub list: llist_head, pub work: delayed_work, pub error: i32, pub items: u32, pub shrinker_hits: u32, pub cpu: u32 }
#[repr(C)] pub struct xfs_groups { pub xa: xarray, pub blocks: u32, pub blklog: u8, pub has_daddr_gaps: bool, pub blkmask: u64, pub start_fsb: xfs_fsblock_t, pub awu_max: xfs_extlen_t }
#[repr(C)] pub struct xfs_freecounter { pub count: percpu_counter, pub res_total: u64, pub res_avail: u64, pub res_saved: u64 }

#[repr(C)] pub struct xfs_mount {
    pub m_sb: xfs_sb, pub m_super: *mut super_block, pub m_ail: *mut xfs_ail, pub m_sb_bp: *mut xfs_buf,
    pub m_rtsb_bp: *mut xfs_buf, pub m_rtname: *mut libc::c_char, pub m_logname: *mut libc::c_char,
    pub m_dir_geo: *mut xfs_da_geometry, pub m_attr_geo: *mut xfs_da_geometry, pub m_log: *mut xlog,
    pub m_rootip: *mut xfs_inode, pub m_metadirip: *mut xfs_inode, pub m_rtdirip: *mut xfs_inode,
    pub m_quotainfo: *mut xfs_quotainfo, pub m_ddev_targp: *mut xfs_buftarg, pub m_logdev_targp: *mut xfs_buftarg,
    pub m_rtdev_targp: *mut xfs_buftarg, pub m_inodegc: *mut libc::c_void, pub m_filestream: *mut xfs_mru_cache,
    pub m_buf_workqueue: *mut workqueue_struct, pub m_unwritten_workqueue: *mut workqueue_struct,
    pub m_reclaim_workqueue: *mut workqueue_struct, pub m_sync_workqueue: *mut workqueue_struct,
    pub m_blockgc_wq: *mut workqueue_struct, pub m_inodegc_wq: *mut workqueue_struct,
    pub m_bsize: i32, pub m_blkbit_log: u8, pub m_blkbb_log: u8, pub m_agno_log: u8, pub m_sectbb_log: u8, pub m_rtxblklog: i8,
    pub m_blockmask: uint, pub m_blockwsize: uint, pub m_rtx_per_rbmblock: uint, pub m_alloc_mxr: [uint;2], pub m_alloc_mnr: [uint;2],
    pub m_bmap_dmxr: [uint;2], pub m_bmap_dmnr: [uint;2], pub m_rmap_mxr: [uint;2], pub m_rmap_mnr: [uint;2],
    pub m_rtrmap_mxr: [uint;2], pub m_rtrmap_mnr: [uint;2], pub m_refc_mxr: [uint;2], pub m_refc_mnr: [uint;2],
    pub m_rtrefc_mxr: [uint;2], pub m_rtrefc_mnr: [uint;2], pub m_alloc_maxlevels: uint, pub m_bm_maxlevels: [uint;2],
    pub m_rmap_maxlevels: uint, pub m_rtrmap_maxlevels: uint, pub m_refc_maxlevels: uint, pub m_rtrefc_maxlevels: uint,
    pub m_agbtree_maxlevels: uint, pub m_rtbtree_maxlevels: uint, pub m_ag_prealloc_blocks: xfs_extlen_t, pub m_alloc_set_aside: uint,
    pub m_ag_max_usable: uint, pub m_dalign: i32, pub m_swidth: i32, pub m_maxagi: xfs_agnumber_t, pub m_allocsize_log: uint,
    pub m_allocsize_blocks: uint, pub m_logbufs: i32, pub m_logbsize: i32, pub m_rsumlevels: uint, pub m_rsumblocks: xfs_filblks_t,
    pub m_fixedfsid: [i32;2], pub m_qflags: uint, pub m_features: u64, pub m_low_space: [u64;XFS_LOWSP_MAX], pub m_low_rtexts: [u64;XFS_LOWSP_MAX],
    pub m_rtxblkmask: u64, pub m_ino_geo: xfs_ino_geometry, pub m_resv: xfs_trans_resv, pub m_opstate: libc::c_ulong,
    pub m_always_cow: bool, pub m_fail_unmount: bool, pub m_finobt_nores: bool, pub m_update_sb: bool, pub m_max_open_zones: uint,
    pub m_zonegc_low_space: uint, pub m_awu_max_bytes: u64, pub m_fs_checked: u8, pub m_fs_sick: u8, pub m_rt_checked: u8, pub m_rt_sick: u8,
    pub m_sb_lock: spinlock_t, pub m_icount: percpu_counter, pub m_ifree: percpu_counter, pub m_free: [xfs_freecounter; XC_FREE_NR],
    pub m_delalloc_blks: percpu_counter, pub m_delalloc_rtextents: percpu_counter, pub m_allocbt_blks: atomic64_t,
    pub m_groups: [xfs_groups; XG_TYPE_MAX], pub m_reclaim_work: delayed_work, pub m_zone_info: *mut xfs_zone_info,
    pub m_debugfs: *mut dentry, pub m_kobj: xfs_kobj, pub m_error_kobj: xfs_kobj, pub m_error_meta_kobj: xfs_kobj,
    pub m_error_cfg: [[xfs_error_cfg; XFS_ERR_ERRNO_MAX]; XFS_ERR_CLASS_MAX], pub m_stats: xstats, pub m_zoned_kobj: xfs_kobj,
    pub m_agfrotor: xfs_agnumber_t, pub m_agirotor: atomic_t, pub m_rtgrotor: atomic_t, pub m_metafile_resv_lock: mutex,
    pub m_metafile_resv_target: u64, pub m_metafile_resv_used: u64, pub m_metafile_resv_avail: u64, pub m_inodegc_shrinker: *mut shrinker,
    pub m_flush_inodes_work: work_struct, pub m_generation: u32, pub m_growlock: mutex, pub m_inodegc_cpumask: cpumask,
    pub m_dir_update_hooks: xfs_hooks, pub m_healthmon: *mut xfs_healthmon, pub m_uuid_table_index: uint,
    pub m_old_io_pages: libc::c_ulong, pub m_initial_io_pages: libc::c_ulong, pub m_old_ra_pages: libc::c_ulong, pub m_initial_ra_pages: libc::c_ulong,
}
pub type xfs_mount_t = xfs_mount;

pub const XFS_FEAT_ATTR:u64=1<<0; pub const XFS_FEAT_NLINK:u64=1<<1; pub const XFS_FEAT_QUOTA:u64=1<<2; pub const XFS_FEAT_ALIGN:u64=1<<3; pub const XFS_FEAT_DALIGN:u64=1<<4; pub const XFS_FEAT_LOGV2:u64=1<<5; pub const XFS_FEAT_SECTOR:u64=1<<6; pub const XFS_FEAT_EXTFLG:u64=1<<7; pub const XFS_FEAT_ASCIICI:u64=1<<8; pub const XFS_FEAT_LAZYSBCOUNT:u64=1<<9; pub const XFS_FEAT_PARENT:u64=1<<11; pub const XFS_FEAT_PROJID32:u64=1<<12; pub const XFS_FEAT_CRC:u64=1<<13; pub const XFS_FEAT_V3INODES:u64=1<<14; pub const XFS_FEAT_PQUOTINO:u64=1<<15; pub const XFS_FEAT_FTYPE:u64=1<<16; pub const XFS_FEAT_FINOBT:u64=1<<17; pub const XFS_FEAT_RMAPBT:u64=1<<18; pub const XFS_FEAT_REFLINK:u64=1<<19; pub const XFS_FEAT_SPINODES:u64=1<<20; pub const XFS_FEAT_META_UUID:u64=1<<21; pub const XFS_FEAT_REALTIME:u64=1<<22; pub const XFS_FEAT_INOBTCNT:u64=1<<23; pub const XFS_FEAT_BIGTIME:u64=1<<24; pub const XFS_FEAT_NEEDSREPAIR:u64=1<<25; pub const XFS_FEAT_NREXT64:u64=1<<26; pub const XFS_FEAT_EXCHANGE_RANGE:u64=1<<27; pub const XFS_FEAT_METADIR:u64=1<<28; pub const XFS_FEAT_ZONED:u64=1<<29;
pub const XFS_FEAT_NOLIFETIME:u64=1<<47; pub const XFS_FEAT_NOALIGN:u64=1<<49; pub const XFS_FEAT_ALLOCSIZE:u64=1<<50; pub const XFS_FEAT_LARGE_IOSIZE:u64=1<<51; pub const XFS_FEAT_WSYNC:u64=1<<52; pub const XFS_FEAT_DIRSYNC:u64=1<<53; pub const XFS_FEAT_DISCARD:u64=1<<54; pub const XFS_FEAT_GRPID:u64=1<<55; pub const XFS_FEAT_SMALL_INUMS:u64=1<<56; pub const XFS_FEAT_SWALLOC:u64=1<<58; pub const XFS_FEAT_FILESTREAMS:u64=1<<59; pub const XFS_FEAT_DAX_ALWAYS:u64=1<<60; pub const XFS_FEAT_DAX_NEVER:u64=1<<61; pub const XFS_FEAT_NORECOVERY:u64=1<<62; pub const XFS_FEAT_NOUUID:u64=1<<63;

pub const XFS_OPSTATE_UNMOUNTING: u32=0; pub const XFS_OPSTATE_CLEAN:u32=1; pub const XFS_OPSTATE_SHUTDOWN:u32=2; pub const XFS_OPSTATE_INODE32:u32=3; pub const XFS_OPSTATE_READONLY:u32=4; pub const XFS_OPSTATE_INODEGC_ENABLED:u32=5; pub const XFS_OPSTATE_BLOCKGC_ENABLED:u32=6; pub const XFS_OPSTATE_WARNED_SHRINK:u32=9; pub const XFS_OPSTATE_WARNED_LARP:u32=10; pub const XFS_OPSTATE_QUOTACHECK_RUNNING:u32=11; pub const XFS_OPSTATE_UNSET_LOG_INCOMPAT:u32=12; pub const XFS_OPSTATE_USE_LARP:u32=13; pub const XFS_OPSTATE_WARNED_LBS:u32=14; pub const XFS_OPSTATE_WARNED_METADIR:u32=17; pub const XFS_OPSTATE_RESUMING_QUOTAON:u32=18; pub const XFS_OPSTATE_ZONEGC_RUNNING:u32=20;
pub const XFS_MAX_IO_LOG:u32=30; pub const XFS_FDBLOCKS_BATCH:u64=1024; pub const XFS_MFSI_QUIET:u32=0x40;

#[inline] pub unsafe fn xfs_has_feat(mp:*const xfs_mount, f:u64)->bool { (*mp).m_features & f != 0 }
#[inline] pub unsafe fn xfs_has_rtgroups(mp:*const xfs_mount)->bool { xfs_has_feat(mp,XFS_FEAT_METADIR) }
#[inline] pub unsafe fn xfs_has_rtsb(mp:*const xfs_mount)->bool { xfs_has_rtgroups(mp)&&xfs_has_feat(mp,XFS_FEAT_REALTIME)&&!xfs_has_feat(mp,XFS_FEAT_ZONED) }
#[inline] pub unsafe fn xfs_has_rtrmapbt(mp:*const xfs_mount)->bool { xfs_has_rtgroups(mp)&&xfs_has_feat(mp,XFS_FEAT_REALTIME)&&xfs_has_feat(mp,XFS_FEAT_RMAPBT) }
#[inline] pub unsafe fn xfs_has_rtreflink(mp:*const xfs_mount)->bool { xfs_has_feat(mp,XFS_FEAT_METADIR)&&xfs_has_feat(mp,XFS_FEAT_REALTIME)&&xfs_has_feat(mp,XFS_FEAT_REFLINK) }
#[inline] pub unsafe fn xfs_has_nonzoned(mp:*const xfs_mount)->bool { !xfs_has_feat(mp,XFS_FEAT_ZONED) }
#[inline] pub unsafe fn xfs_can_sw_atomic_write(mp:*mut xfs_mount)->bool { xfs_has_feat(mp,XFS_FEAT_REFLINK) }
#[inline] pub unsafe fn xfs_should_warn(mp:*mut xfs_mount,nr:u32)->bool { !test_and_set_bit(nr,&mut (*mp).m_opstate) }

pub const SHUTDOWN_META_IO_ERROR:u32=1<<0; pub const SHUTDOWN_LOG_IO_ERROR:u32=1<<1; pub const SHUTDOWN_FORCE_UMOUNT:u32=1<<2; pub const SHUTDOWN_CORRUPT_INCORE:u32=1<<3; pub const SHUTDOWN_CORRUPT_ONDISK:u32=1<<4; pub const SHUTDOWN_DEVICE_REMOVED:u32=1<<5;
extern "C" { pub fn xfs_do_force_shutdown(mp:*mut xfs_mount,flags:u32,fname:*mut libc::c_char,lnnum:i32); pub fn xfs_uuid_table_free(); pub fn xfs_default_resblks(mp:*mut xfs_mount,ctr:xfs_free_counter)->u64; pub fn xfs_mountfs(mp:*mut xfs_mount)->i32; pub fn xfs_unmountfs(mp:*mut xfs_mount); pub fn xfs_freecounter_unavailable(mp:*mut xfs_mount,ctr:xfs_free_counter)->u64; pub fn xfs_readsb(mp:*mut xfs_mount,flags:i32)->i32; pub fn xfs_freesb(mp:*mut xfs_mount); pub fn xfs_fs_writable(mp:*mut xfs_mount,level:i32)->bool; pub fn xfs_sb_validate_fsb_count(sb:*mut xfs_sb,count:u64)->i32; pub fn xfs_dev_is_read_only(mp:*mut xfs_mount,name:*mut libc::c_char)->i32; pub fn xfs_set_low_space_thresholds(mp:*mut xfs_mount); pub fn xfs_zero_extent(ip:*mut xfs_inode,start:xfs_fsblock_t,count:xfs_off_t)->i32; pub fn xfs_error_get_cfg(mp:*mut xfs_mount,class:i32,error:i32)->*mut xfs_error_cfg; pub fn xfs_force_summary_recalc(mp:*mut xfs_mount); pub fn xfs_add_incompat_log_feature(mp:*mut xfs_mount,feature:u32)->i32; pub fn xfs_clear_incompat_log_features(mp:*mut xfs_mount)->bool; pub fn xfs_mod_delalloc(ip:*mut xfs_inode,data_delta:i64,ind_delta:i64); pub fn xfs_set_max_atomic_write_opt(mp:*mut xfs_mount,new_max_bytes:u64)->i32; }

extern "C" {
    pub fn percpu_counter_sum_positive(c:*const percpu_counter)->i64;
    pub fn percpu_counter_sum(c:*const percpu_counter)->i64;
    pub fn percpu_counter_read_positive(c:*const percpu_counter)->i64;
    pub fn __percpu_counter_compare(c:*const percpu_counter,rhs:i64,batch:i32)->i32;
    pub fn percpu_counter_set(c:*mut percpu_counter,val:u64);
    pub fn percpu_counter_add(c:*mut percpu_counter,val:i64);
    pub fn xfs_dec_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter,delta:u64,rsvd:bool)->i32;
    pub fn xfs_add_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter,delta:u64);
    pub fn test_and_set_bit(nr:u32,addr:*mut libc::c_ulong)->bool;
}
#[inline] pub unsafe fn xfs_sum_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter)->i64 { percpu_counter_sum_positive(&(*mp).m_free[ctr as usize].count) }
#[inline] pub unsafe fn xfs_sum_freecounter_raw(mp:*mut xfs_mount,ctr:xfs_free_counter)->i64 { percpu_counter_sum(&(*mp).m_free[ctr as usize].count) }
#[inline] pub unsafe fn xfs_estimate_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter)->i64 { percpu_counter_read_positive(&(*mp).m_free[ctr as usize].count) }
#[inline] pub unsafe fn xfs_compare_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter,rhs:i64,batch:i32)->i32 { __percpu_counter_compare(&(*mp).m_free[ctr as usize].count,rhs,batch) }
#[inline] pub unsafe fn xfs_set_freecounter(mp:*mut xfs_mount,ctr:xfs_free_counter,val:u64) { percpu_counter_set(&mut (*mp).m_free[ctr as usize].count,val) }
#[inline] pub unsafe fn xfs_dec_fdblocks(mp:*mut xfs_mount,delta:u64,reserved:bool)->i32 { xfs_dec_freecounter(mp,XC_FREE_BLOCKS,delta,reserved) }
#[inline] pub unsafe fn xfs_add_fdblocks(mp:*mut xfs_mount,delta:u64) { xfs_add_freecounter(mp,XC_FREE_BLOCKS,delta) }
#[inline] pub unsafe fn xfs_dec_frextents(mp:*mut xfs_mount,delta:u64)->i32 { xfs_dec_freecounter(mp,XC_FREE_RTEXTENTS,delta,false) }
#[inline] pub unsafe fn xfs_add_frextents(mp:*mut xfs_mount,delta:u64) { xfs_add_freecounter(mp,XC_FREE_RTEXTENTS,delta) }
#[inline] pub unsafe fn xfs_mod_sb_delalloc(mp:*mut xfs_mount,delta:i64) { percpu_counter_add(&mut (*mp).m_delalloc_blks,delta) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
