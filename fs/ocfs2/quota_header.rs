/* SPDX-License-Identifier: GPL-2.0 */
/*
 * quota.h for OCFS2
 *
 * On disk quota structures for local and global quota file, in-memory
 * structures.
 */

// C dependencies: linux/types.h, linux/slab.h, linux/quota.h, linux/list.h,
// linux/dqblk_qtree.h, and ocfs2.h.

/* Number of quota types we support */
pub const OCFS2_MAXQUOTAS: usize = 2;

/* In-memory structures */
#[repr(C)]
pub struct ocfs2_dquot {
    pub dq_dquot: dquot,
    pub dq_local_off: loff_t,
    pub dq_local_phys_blk: u64,
    pub dq_chunk: *mut ocfs2_quota_chunk,
    pub dq_use_count: ::std::os::raw::c_uint,
    pub dq_origspace: i64,
    pub dq_originodes: i64,
    pub list: llist_node,
}

/* Description of one chunk to recover in memory */
#[repr(C)]
pub struct ocfs2_recovery_chunk {
    pub rc_list: list_head,
    pub rc_chunk: ::std::os::raw::c_int,
    pub rc_bitmap: *mut ::std::os::raw::c_ulong,
}

#[repr(C)]
pub struct ocfs2_quota_recovery {
    pub r_list: [list_head; OCFS2_MAXQUOTAS],
}

/* In-memory structure with quota header information */
#[repr(C)]
pub struct ocfs2_mem_dqinfo {
    pub dqi_type: ::std::os::raw::c_uint,
    pub dqi_flags: ::std::os::raw::c_uint,
    pub dqi_chunks: ::std::os::raw::c_uint,
    pub dqi_blocks: ::std::os::raw::c_uint,
    pub dqi_syncms: ::std::os::raw::c_uint,
    pub dqi_chunk: list_head,
    pub dqi_gqinode: *mut inode,
    pub dqi_gqlock: ocfs2_lock_res,
    pub dqi_gqi_bh: *mut buffer_head,
    pub dqi_gqi_count: ::std::os::raw::c_int,
    pub dqi_giblk: u64,
    pub dqi_lqi_bh: *mut buffer_head,
    pub dqi_libh: *mut buffer_head,
    pub dqi_gi: qtree_mem_dqinfo,
    pub dqi_sync_work: delayed_work,
    pub dqi_rec: *mut ocfs2_quota_recovery,
}

pub unsafe fn OCFS2_DQUOT(dquot: *mut dquot) -> *mut ocfs2_dquot {
    (dquot as *mut u8)
        .sub(::std::mem::offset_of!(ocfs2_dquot, dq_dquot))
        as *mut ocfs2_dquot
}

#[repr(C)]
pub struct ocfs2_quota_chunk {
    pub qc_chunk: list_head,
    pub qc_num: ::std::os::raw::c_int,
    pub qc_headerbh: *mut buffer_head,
}

extern "C" {
    pub static mut ocfs2_dquot_cachep: *mut kmem_cache;
    pub static mut ocfs2_qf_chunk_cachep: *mut kmem_cache;
    pub static ocfs2_global_ops: qtree_fmt_operations;

    pub fn ocfs2_begin_quota_recovery(osb: *mut ocfs2_super, slot_num: ::std::os::raw::c_int) -> *mut ocfs2_quota_recovery;
    pub fn ocfs2_finish_quota_recovery(osb: *mut ocfs2_super, rec: *mut ocfs2_quota_recovery, slot_num: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ocfs2_free_quota_recovery(rec: *mut ocfs2_quota_recovery);
    pub fn ocfs2_quota_read(sb: *mut super_block, type_: ::std::os::raw::c_int, data: *mut ::std::os::raw::c_char, len: usize, off: loff_t) -> isize;
    pub fn ocfs2_quota_write(sb: *mut super_block, type_: ::std::os::raw::c_int, data: *const ::std::os::raw::c_char, len: usize, off: loff_t) -> isize;
    pub fn ocfs2_global_read_info(sb: *mut super_block, type_: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ocfs2_global_write_info(sb: *mut super_block, type_: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn __ocfs2_sync_dquot(dquot: *mut dquot, freeing: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ocfs2_lock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ocfs2_unlock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: ::std::os::raw::c_int);
    pub fn ocfs2_validate_quota_block(sb: *mut super_block, bh: *mut buffer_head) -> ::std::os::raw::c_int;
    pub fn ocfs2_read_quota_phys_block(inode: *mut inode, p_block: u64, bh: *mut *mut buffer_head) -> ::std::os::raw::c_int;
    pub fn ocfs2_create_local_dquot(dquot: *mut dquot) -> ::std::os::raw::c_int;
    pub fn ocfs2_local_release_dquot(handle: *mut handle_t, dquot: *mut dquot) -> ::std::os::raw::c_int;
    pub fn ocfs2_local_write_dquot(dquot: *mut dquot) -> ::std::os::raw::c_int;
    pub fn ocfs2_drop_dquot_refs(work: *mut work_struct);
    pub static ocfs2_quota_operations: dquot_operations;
    pub static mut ocfs2_quota_format: quota_format_type;
}

pub unsafe fn ocfs2_sync_dquot(dquot: *mut dquot) -> ::std::os::raw::c_int {
    __ocfs2_sync_dquot(dquot, 0)
}

pub unsafe fn ocfs2_global_release_dquot(dquot: *mut dquot) -> ::std::os::raw::c_int {
    __ocfs2_sync_dquot(dquot, 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
