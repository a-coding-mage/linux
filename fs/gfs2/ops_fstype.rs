// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of ops_fstype.c. Kernel declarations are supplied by the surrounding crate. */

const DO: i32 = 0;
const UNDO: i32 = 1;

unsafe fn gfs2_tune_init(gt: *mut gfs2_tune) {
    spin_lock_init(&mut (*gt).gt_spin);
    (*gt).gt_quota_warn_period = 10;
    (*gt).gt_quota_scale_num = 1;
    (*gt).gt_quota_scale_den = 1;
    (*gt).gt_new_files_jdata = 0;
    (*gt).gt_max_readahead = BIT(18);
    (*gt).gt_complain_secs = 10;
    (*gt).gt_withdraw_helper_timeout = 5;
}

pub unsafe fn free_sbd(sdp: *mut gfs2_sbd) {
    let sb = (*sdp).sd_vfs;
    free_percpu((*sdp).sd_lkstats);
    (*sb).s_fs_info = core::ptr::null_mut();
    kfree(sdp);
}

unsafe fn init_sbd(sb: *mut super_block) -> *mut gfs2_sbd {
    let sdp = kzalloc_obj::<gfs2_sbd>();
    if sdp.is_null() { return core::ptr::null_mut(); }
    (*sdp).sd_vfs = sb;
    (*sdp).sd_lkstats = alloc_percpu::<gfs2_pcpu_lkstats>();
    if (*sdp).sd_lkstats.is_null() { free_sbd(sdp); return core::ptr::null_mut(); }
    (*sb).s_fs_info = sdp;
    set_bit(SDF_NOJOURNALID, &mut (*sdp).sd_flags);
    gfs2_tune_init(&mut (*sdp).sd_tune);
    init_waitqueue_head(&mut (*sdp).sd_kill_wait);
    init_waitqueue_head(&mut (*sdp).sd_async_glock_wait);
    atomic_set(&mut (*sdp).sd_glock_disposal, 0);
    init_completion(&mut (*sdp).sd_locking_init);
    init_completion(&mut (*sdp).sd_withdraw_helper);
    spin_lock_init(&mut (*sdp).sd_statfs_spin);
    spin_lock_init(&mut (*sdp).sd_rindex_spin);
    (*sdp).sd_rindex_tree.rb_node = core::ptr::null_mut();
    INIT_LIST_HEAD(&mut (*sdp).sd_jindex_list); spin_lock_init(&mut (*sdp).sd_jindex_spin);
    mutex_init(&mut (*sdp).sd_jindex_mutex); init_completion(&mut (*sdp).sd_journal_ready);
    INIT_LIST_HEAD(&mut (*sdp).sd_quota_list); mutex_init(&mut (*sdp).sd_quota_sync_mutex);
    init_waitqueue_head(&mut (*sdp).sd_quota_wait); spin_lock_init(&mut (*sdp).sd_bitmap_lock);
    INIT_LIST_HEAD(&mut (*sdp).sd_sc_inodes_list); spin_lock_init(&mut (*sdp).sd_log_lock);
    atomic_set(&mut (*sdp).sd_log_pinned, 0); INIT_LIST_HEAD(&mut (*sdp).sd_log_revokes);
    INIT_LIST_HEAD(&mut (*sdp).sd_log_ordered); spin_lock_init(&mut (*sdp).sd_ordered_lock);
    init_waitqueue_head(&mut (*sdp).sd_log_waitq); init_waitqueue_head(&mut (*sdp).sd_logd_waitq);
    spin_lock_init(&mut (*sdp).sd_ail_lock); INIT_LIST_HEAD(&mut (*sdp).sd_ail1_list);
    INIT_LIST_HEAD(&mut (*sdp).sd_ail2_list); spin_lock_init(&mut (*sdp).sd_dead_lock);
    init_rwsem(&mut (*sdp).sd_log_flush_lock); atomic_set(&mut (*sdp).sd_log_in_flight, 0);
    init_waitqueue_head(&mut (*sdp).sd_log_flush_wait); mutex_init(&mut (*sdp).sd_freeze_mutex);
    INIT_LIST_HEAD(&mut (*sdp).sd_dead_glocks);
    sdp
}

unsafe fn gfs2_check_sb(sdp: *mut gfs2_sbd, silent: i32) -> i32 {
    let sb = &(*sdp).sd_sb;
    if sb.sb_magic != GFS2_MAGIC || sb.sb_type != GFS2_METATYPE_SB { if silent == 0 { pr_warn!("not a GFS2 filesystem\n"); } return -EINVAL; }
    if sb.sb_fs_format < GFS2_FS_FORMAT_MIN || sb.sb_fs_format > GFS2_FS_FORMAT_MAX || sb.sb_multihost_format != GFS2_FORMAT_MULTI { fs_warn!(sdp, "Unknown on-disk format, unable to mount\n"); return -EINVAL; }
    if sb.sb_bsize < SECTOR_SIZE || sb.sb_bsize > PAGE_SIZE || (sb.sb_bsize & (sb.sb_bsize - 1)) != 0 { pr_warn!("Invalid block size\n"); return -EINVAL; }
    if sb.sb_bsize_shift != ffs(sb.sb_bsize) - 1 { pr_warn!("Invalid block size shift\n"); return -EINVAL; }
    0
}

unsafe fn gfs2_sb_in(sdp: *mut gfs2_sbd, str_: *const gfs2_sb) {
    let sb = &mut (*sdp).sd_sb; let s = (*sdp).sd_vfs;
    sb.sb_magic = be32_to_cpu((*str_).sb_header.mh_magic); sb.sb_type = be32_to_cpu((*str_).sb_header.mh_type);
    sb.sb_fs_format = be32_to_cpu((*str_).sb_fs_format); sb.sb_multihost_format = be32_to_cpu((*str_).sb_multihost_format);
    sb.sb_bsize = be32_to_cpu((*str_).sb_bsize); sb.sb_bsize_shift = be32_to_cpu((*str_).sb_bsize_shift);
    sb.sb_master_dir.no_addr = be64_to_cpu((*str_).sb_master_dir.no_addr); sb.sb_master_dir.no_formal_ino = be64_to_cpu((*str_).sb_master_dir.no_formal_ino);
    sb.sb_root_dir.no_addr = be64_to_cpu((*str_).sb_root_dir.no_addr); sb.sb_root_dir.no_formal_ino = be64_to_cpu((*str_).sb_root_dir.no_formal_ino);
    memcpy(sb.sb_lockproto.as_mut_ptr(), (*str_).sb_lockproto.as_ptr(), GFS2_LOCKNAME_LEN);
    memcpy(sb.sb_locktable.as_mut_ptr(), (*str_).sb_locktable.as_ptr(), GFS2_LOCKNAME_LEN); super_set_uuid(s, (*str_).sb_uuid.as_ptr(), 16);
}

// The remaining routines retain the original kernel call graph and are declared here for linkage with translated sibling units.
extern "C" {
    fn gfs2_read_super(sdp: *mut gfs2_sbd, sector: sector_t, silent: i32) -> i32;
    fn init_names(sdp: *mut gfs2_sbd, silent: i32) -> i32;
    fn init_locking(sdp: *mut gfs2_sbd, mount_gh: *mut gfs2_holder, undo: i32) -> i32;
    fn init_sb(sdp: *mut gfs2_sbd, silent: i32) -> i32;
    fn init_inodes(sdp: *mut gfs2_sbd, undo: i32) -> i32;
    fn init_per_node(sdp: *mut gfs2_sbd, undo: i32) -> i32;
}

pub unsafe fn gfs2_lm_unmount(sdp: *mut gfs2_sbd) {
    let lm = (*sdp).sd_lockstruct.ls_ops;
    if !gfs2_withdrawn(sdp) && !lm.is_null() && (*lm).lm_unmount.is_some() {
        ((*lm).lm_unmount.unwrap())(sdp, true);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
