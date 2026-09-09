/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * the_nilfs shared structure.
 *
 * Translated from the_nilfs.h.
 */

/* Linux dependencies and build-time definitions are supplied externally. */

pub const THE_NILFS_INIT: u32 = 0;
pub const THE_NILFS_DISCONTINUED: u32 = 1;
pub const THE_NILFS_GC_RUNNING: u32 = 2;
pub const THE_NILFS_SB_DIRTY: u32 = 3;
pub const THE_NILFS_PURGING: u32 = 4;

pub enum nilfs_sc_info {}
pub enum nilfs_sysfs_dev_subgroups {}

#[repr(C)]
pub struct the_nilfs {
    pub ns_flags: ::core::ffi::c_ulong,
    pub ns_flushed_device: ::core::ffi::c_int,
    pub ns_sb: *mut super_block,
    pub ns_bdev: *mut block_device,
    pub ns_sem: rw_semaphore,
    pub ns_snapshot_mount_mutex: mutex,
    pub ns_sbh: [*mut buffer_head; 2],
    pub ns_sbp: [*mut nilfs_super_block; 2],
    pub ns_sbwtime: i64,
    pub ns_sbwcount: u32,
    pub ns_sbsize: u32,
    pub ns_mount_state: u32,
    pub ns_sb_update_freq: u32,
    pub ns_seg_seq: u64,
    pub ns_segnum: u64,
    pub ns_nextnum: u64,
    pub ns_pseg_offset: ::core::ffi::c_ulong,
    pub ns_cno: u64,
    pub ns_ctime: i64,
    pub ns_nongc_ctime: i64,
    pub ns_ndirtyblks: atomic_t,
    pub ns_last_segment_lock: spinlock_t,
    pub ns_last_pseg: sector_t,
    pub ns_last_seq: u64,
    pub ns_last_cno: u64,
    pub ns_prot_seq: u64,
    pub ns_prev_seq: u64,
    pub ns_writer: *mut nilfs_sc_info,
    pub ns_segctor_sem: rw_semaphore,
    pub ns_dat: *mut inode,
    pub ns_cpfile: *mut inode,
    pub ns_sufile: *mut inode,
    pub ns_cptree: rb_root,
    pub ns_cptree_lock: spinlock_t,
    pub ns_dirty_files: list_head,
    pub ns_inode_lock: spinlock_t,
    pub ns_gc_inodes: list_head,
    pub ns_mount_opt: ::core::ffi::c_ulong,
    pub ns_resuid: uid_t,
    pub ns_resgid: gid_t,
    pub ns_interval: ::core::ffi::c_ulong,
    pub ns_watermark: ::core::ffi::c_ulong,
    pub ns_blocksize_bits: u32,
    pub ns_blocksize: u32,
    pub ns_nsegments: ::core::ffi::c_ulong,
    pub ns_blocks_per_segment: ::core::ffi::c_ulong,
    pub ns_r_segments_percentage: ::core::ffi::c_ulong,
    pub ns_nrsvsegs: ::core::ffi::c_ulong,
    pub ns_first_data_block: ::core::ffi::c_ulong,
    pub ns_inode_size: ::core::ffi::c_int,
    pub ns_first_ino: u32,
    pub ns_crc_seed: u32,
    pub ns_dev_kobj: kobject,
    pub ns_dev_kobj_unregister: completion,
    pub ns_dev_subgroups: *mut nilfs_sysfs_dev_subgroups,
}

#[repr(C)]
pub struct nilfs_root {
    pub cno: u64,
    pub rb_node: rb_node,
    pub count: refcount_t,
    pub nilfs: *mut the_nilfs,
    pub ifile: *mut inode,
    pub inodes_count: atomic64_t,
    pub blocks_count: atomic64_t,
    pub snapshot_kobj: kobject,
    pub snapshot_kobj_unregister: completion,
}

pub const NILFS_CPTREE_CURRENT_CNO: u64 = 0;
pub const NILFS_SB_FREQ: u32 = 10;

#[macro_export]
macro_rules! nilfs_clear_opt { ($nilfs:expr, $opt:ident) => { (*$nilfs).ns_mount_opt &= !NILFS_MOUNT_$opt }; }
#[macro_export]
macro_rules! nilfs_set_opt { ($nilfs:expr, $opt:ident) => { (*$nilfs).ns_mount_opt |= NILFS_MOUNT_$opt }; }
#[macro_export]
macro_rules! nilfs_test_opt { ($nilfs:expr, $opt:ident) => { (*$nilfs).ns_mount_opt & NILFS_MOUNT_$opt }; }

pub unsafe fn set_nilfs_init(nilfs: *mut the_nilfs) { set_bit(THE_NILFS_INIT, &mut (*nilfs).ns_flags); }
pub unsafe fn clear_nilfs_init(nilfs: *mut the_nilfs) { clear_bit(THE_NILFS_INIT, &mut (*nilfs).ns_flags); }
pub unsafe fn nilfs_init(nilfs: *mut the_nilfs) -> i32 { test_bit(THE_NILFS_INIT, &(*nilfs).ns_flags) }
pub unsafe fn set_nilfs_discontinued(nilfs: *mut the_nilfs) { set_bit(THE_NILFS_DISCONTINUED, &mut (*nilfs).ns_flags); }
pub unsafe fn clear_nilfs_discontinued(nilfs: *mut the_nilfs) { clear_bit(THE_NILFS_DISCONTINUED, &mut (*nilfs).ns_flags); }
pub unsafe fn nilfs_discontinued(nilfs: *mut the_nilfs) -> i32 { test_bit(THE_NILFS_DISCONTINUED, &(*nilfs).ns_flags) }
pub unsafe fn set_nilfs_gc_running(nilfs: *mut the_nilfs) { set_bit(THE_NILFS_GC_RUNNING, &mut (*nilfs).ns_flags); }
pub unsafe fn clear_nilfs_gc_running(nilfs: *mut the_nilfs) { clear_bit(THE_NILFS_GC_RUNNING, &mut (*nilfs).ns_flags); }
pub unsafe fn nilfs_gc_running(nilfs: *mut the_nilfs) -> i32 { test_bit(THE_NILFS_GC_RUNNING, &(*nilfs).ns_flags) }
pub unsafe fn set_nilfs_sb_dirty(nilfs: *mut the_nilfs) { set_bit(THE_NILFS_SB_DIRTY, &mut (*nilfs).ns_flags); }
pub unsafe fn clear_nilfs_sb_dirty(nilfs: *mut the_nilfs) { clear_bit(THE_NILFS_SB_DIRTY, &mut (*nilfs).ns_flags); }
pub unsafe fn nilfs_sb_dirty(nilfs: *mut the_nilfs) -> i32 { test_bit(THE_NILFS_SB_DIRTY, &(*nilfs).ns_flags) }
pub unsafe fn set_nilfs_purging(nilfs: *mut the_nilfs) { set_bit(THE_NILFS_PURGING, &mut (*nilfs).ns_flags); }
pub unsafe fn clear_nilfs_purging(nilfs: *mut the_nilfs) { clear_bit(THE_NILFS_PURGING, &mut (*nilfs).ns_flags); }
pub unsafe fn nilfs_purging(nilfs: *mut the_nilfs) -> i32 { test_bit(THE_NILFS_PURGING, &(*nilfs).ns_flags) }

extern "C" {
    pub fn nilfs_set_last_segment(nilfs: *mut the_nilfs, pseg: sector_t, seq: u64, cno: u64);
    pub fn alloc_nilfs(sb: *mut super_block) -> *mut the_nilfs;
    pub fn destroy_nilfs(nilfs: *mut the_nilfs);
    pub fn init_nilfs(nilfs: *mut the_nilfs, sb: *mut super_block) -> i32;
    pub fn load_nilfs(nilfs: *mut the_nilfs, sb: *mut super_block) -> i32;
    pub fn nilfs_nrsvsegs(nilfs: *mut the_nilfs, nsegs: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn nilfs_set_nsegments(nilfs: *mut the_nilfs, nsegs: ::core::ffi::c_ulong);
    pub fn nilfs_discard_segments(nilfs: *mut the_nilfs, segnums: *mut u64, n: usize) -> i32;
    pub fn nilfs_count_free_blocks(nilfs: *mut the_nilfs, nfree: *mut sector_t) -> i32;
    pub fn nilfs_lookup_root(nilfs: *mut the_nilfs, cno: u64) -> *mut nilfs_root;
    pub fn nilfs_find_or_create_root(nilfs: *mut the_nilfs, cno: u64) -> *mut nilfs_root;
    pub fn nilfs_put_root(root: *mut nilfs_root);
    pub fn nilfs_near_disk_full(nilfs: *mut the_nilfs) -> i32;
    pub fn nilfs_fall_back_super_block(nilfs: *mut the_nilfs);
    pub fn nilfs_swap_super_block(nilfs: *mut the_nilfs);
}

pub unsafe fn nilfs_get_root(root: *mut nilfs_root) { refcount_inc(&mut (*root).count); }
pub unsafe fn nilfs_valid_fs(nilfs: *mut the_nilfs) -> u32 {
    let mut valid_fs: u32;
    down_read(&mut (*nilfs).ns_sem);
    valid_fs = (*nilfs).ns_mount_state & NILFS_VALID_FS;
    up_read(&mut (*nilfs).ns_sem);
    valid_fs
}
pub unsafe fn nilfs_get_segment_range(nilfs: *mut the_nilfs, segnum: u64, seg_start: *mut sector_t, seg_end: *mut sector_t) {
    *seg_start = (*nilfs).ns_blocks_per_segment.wrapping_mul(segnum as _);
    *seg_end = (*seg_start).wrapping_add((*nilfs).ns_blocks_per_segment as _).wrapping_sub(1);
    if segnum == 0 { *seg_start = (*nilfs).ns_first_data_block as _; }
}
pub unsafe fn nilfs_get_segment_start_blocknr(nilfs: *mut the_nilfs, segnum: u64) -> sector_t {
    if segnum == 0 { (*nilfs).ns_first_data_block as _ } else { ((*nilfs).ns_blocks_per_segment as u64).wrapping_mul(segnum) as _ }
}
pub unsafe fn nilfs_get_segnum_of_block(nilfs: *mut the_nilfs, blocknr: sector_t) -> u64 {
    (blocknr as u64) / (*nilfs).ns_blocks_per_segment as u64
}
pub unsafe fn nilfs_terminate_segment(nilfs: *mut the_nilfs, seg_start: sector_t, seg_end: sector_t) { (*nilfs).ns_pseg_offset = (seg_end - seg_start + 1) as _; }
pub unsafe fn nilfs_shift_to_next_segment(nilfs: *mut the_nilfs) { (*nilfs).ns_segnum = (*nilfs).ns_nextnum; (*nilfs).ns_pseg_offset = 0; (*nilfs).ns_seg_seq = (*nilfs).ns_seg_seq.wrapping_add(1); }
pub unsafe fn nilfs_last_cno(nilfs: *mut the_nilfs) -> u64 { spin_lock(&mut (*nilfs).ns_last_segment_lock); let cno = (*nilfs).ns_last_cno; spin_unlock(&mut (*nilfs).ns_last_segment_lock); cno }
pub unsafe fn nilfs_segment_is_active(nilfs: *mut the_nilfs, n: u64) -> i32 { (n == (*nilfs).ns_segnum || n == (*nilfs).ns_nextnum) as i32 }
pub unsafe fn nilfs_flush_device(nilfs: *mut the_nilfs) -> i32 {
    if nilfs_test_opt!(nilfs, BARRIER) == 0 || (*nilfs).ns_flushed_device != 0 { return 0; }
    (*nilfs).ns_flushed_device = 1;
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    let mut err = blkdev_issue_flush((*nilfs).ns_bdev);
    if err != -EIO { err = 0; }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
