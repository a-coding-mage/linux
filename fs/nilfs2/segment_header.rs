/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS Segment constructor prototypes and definitions
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi.
 *
 */

// Dependencies supplied by the surrounding kernel/project translation.

pub struct nilfs_root;

/**
 * struct nilfs_recovery_info - Recovery information
 * @ri_need_recovery: Recovery status
 * @ri_super_root: Block number of the last super root
 * @ri_cno: Number of the last checkpoint
 * @ri_lsegs_start: Region for roll-forwarding (start block number)
 * @ri_lsegs_end: Region for roll-forwarding (end block number)
 * @ri_lsegs_start_seq: Sequence value of the segment at ri_lsegs_start
 * @ri_used_segments: List of segments to be mark active
 * @ri_pseg_start: Block number of the last partial segment
 * @ri_seq: Sequence number on the last partial segment
 * @ri_segnum: Segment number on the last partial segment
 * @ri_nextnum: Next segment number on the last partial segment
 */
#[repr(C)]
pub struct nilfs_recovery_info {
    pub ri_need_recovery: ::core::ffi::c_int,
    pub ri_super_root: sector_t,
    pub ri_cno: __u64,
    pub ri_lsegs_start: sector_t,
    pub ri_lsegs_end: sector_t,
    pub ri_lsegs_start_seq: u64,
    pub ri_used_segments: list_head,
    pub ri_pseg_start: sector_t,
    pub ri_seq: u64,
    pub ri_segnum: __u64,
    pub ri_nextnum: __u64,
}

/* ri_need_recovery */
pub const NILFS_RECOVERY_SR_UPDATED: ::core::ffi::c_int = 1;
pub const NILFS_RECOVERY_ROLLFORWARD_DONE: ::core::ffi::c_int = 2;

/** Context of collection stage. */
#[repr(C)]
pub struct nilfs_cstage {
    pub scnt: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub dirty_file_ptr: *mut nilfs_inode_info,
    pub gc_inode_ptr: *mut nilfs_inode_info,
}

pub struct nilfs_segment_buffer;

#[repr(C)]
pub struct nilfs_segsum_pointer {
    pub bh: *mut buffer_head,
    pub offset: ::core::ffi::c_uint,
}

/** Segment constructor information. */
#[repr(C)]
pub struct nilfs_sc_info {
    pub sc_super: *mut super_block,
    pub sc_root: *mut nilfs_root,
    pub sc_nblk_inc: ::core::ffi::c_ulong,
    pub sc_dirty_files: list_head,
    pub sc_gc_inodes: list_head,
    pub sc_iput_queue: list_head,
    pub sc_iput_work: work_struct,
    pub sc_freesegs: *mut __u64,
    pub sc_nfreesegs: usize,
    pub sc_dsync_inode: *mut nilfs_inode_info,
    pub sc_dsync_start: loff_t,
    pub sc_dsync_end: loff_t,
    pub sc_segbufs: list_head,
    pub sc_write_logs: list_head,
    pub sc_segbuf_nblocks: ::core::ffi::c_ulong,
    pub sc_curseg: *mut nilfs_segment_buffer,
    pub sc_stage: nilfs_cstage,
    pub sc_finfo_ptr: nilfs_segsum_pointer,
    pub sc_binfo_ptr: nilfs_segsum_pointer,
    pub sc_blk_cnt: ::core::ffi::c_ulong,
    pub sc_datablk_cnt: ::core::ffi::c_ulong,
    pub sc_nblk_this_inc: ::core::ffi::c_ulong,
    pub sc_seg_ctime: time64_t,
    pub sc_cno: __u64,
    pub sc_flags: ::core::ffi::c_ulong,
    pub sc_state_lock: spinlock_t,
    pub sc_state: ::core::ffi::c_ulong,
    pub sc_flush_request: ::core::ffi::c_ulong,
    pub sc_wait_request: wait_queue_head_t,
    pub sc_wait_daemon: wait_queue_head_t,
    pub sc_seq_request: __u32,
    pub sc_seq_accepted: __u32,
    pub sc_seq_done: __u32,
    pub sc_sync: ::core::ffi::c_int,
    pub sc_interval: ::core::ffi::c_ulong,
    pub sc_mjcp_freq: ::core::ffi::c_ulong,
    pub sc_lseg_stime: ::core::ffi::c_ulong,
    pub sc_watermark: ::core::ffi::c_ulong,
    pub sc_timer: timer_list,
    pub sc_task: *mut task_struct,
}

/* sc_flags */
pub const NILFS_SC_DIRTY: ::core::ffi::c_uint = 0;
pub const NILFS_SC_UNCLOSED: ::core::ffi::c_uint = 1;
pub const NILFS_SC_SUPER_ROOT: ::core::ffi::c_uint = 2;
pub const NILFS_SC_PRIOR_FLUSH: ::core::ffi::c_uint = 3;
pub const NILFS_SC_HAVE_DELTA: ::core::ffi::c_uint = 4;

/* sc_state */
pub const NILFS_SEGCTOR_COMMIT: ::core::ffi::c_ulong = 0x0004;

/* Constant parameters */
pub const NILFS_SC_CLEANUP_RETRY: ::core::ffi::c_int = 3;
pub const NILFS_SC_DEFAULT_TIMEOUT: ::core::ffi::c_int = 5;
pub const NILFS_SC_DEFAULT_SR_FREQ: ::core::ffi::c_int = 30;
pub const NILFS_SC_DEFAULT_WATERMARK: ::core::ffi::c_int = 3600;

extern "C" {
    pub static mut nilfs_transaction_cachep: *mut kmem_cache;

    pub fn nilfs_relax_pressure_in_lock(sb: *mut super_block);
    pub fn nilfs_construct_segment(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn nilfs_construct_dsync_segment(
        sb: *mut super_block,
        inode: *mut inode,
        start: loff_t,
        end: loff_t,
    ) -> ::core::ffi::c_int;
    pub fn nilfs_clean_segments(
        sb: *mut super_block,
        argv: *mut nilfs_argv,
        ret: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn nilfs_attach_log_writer(sb: *mut super_block, root: *mut nilfs_root) -> ::core::ffi::c_int;
    pub fn nilfs_detach_log_writer(sb: *mut super_block);
    pub fn nilfs_read_super_root_block(
        nilfs: *mut the_nilfs,
        block: sector_t,
        bh: *mut *mut buffer_head,
        create: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn nilfs_search_super_root(
        nilfs: *mut the_nilfs,
        ri: *mut nilfs_recovery_info,
    ) -> ::core::ffi::c_int;
    pub fn nilfs_salvage_orphan_logs(
        nilfs: *mut the_nilfs,
        sb: *mut super_block,
        ri: *mut nilfs_recovery_info,
    ) -> ::core::ffi::c_int;
    pub fn nilfs_dispose_segment_list(list: *mut list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
