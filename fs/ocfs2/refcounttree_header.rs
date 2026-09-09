/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * refcounttree.h
 *
 * Copyright (C) 2009 Oracle.  All rights reserved.
 */

#[repr(C)]
pub struct ocfs2_refcount_tree {
    pub rf_node: rb_node,
    pub rf_blkno: u64,
    pub rf_generation: u32,
    pub rf_getcnt: kref,
    pub rf_sem: rw_semaphore,
    pub rf_lockres: ocfs2_lock_res,
    pub rf_removed: i32,

    /* the following 4 fields are used by caching_info. */
    pub rf_lock: spinlock_t,
    pub rf_ci: ocfs2_caching_info,
    pub rf_io_mutex: mutex,
    pub rf_sb: *mut super_block,
}

pub type ocfs2_post_refcount_func = unsafe extern "C" fn(
    inode: *mut inode,
    handle: *mut handle_t,
    para: *mut core::ffi::c_void,
) -> i32;

/*
 * Some refcount caller need to do more work after we modify the data b-tree
 * during refcount operation(including CoW and add refcount flag), and make the
 * transaction complete. So it must give us this structure so that we can do it
 * within our transaction.
 *
 */
#[repr(C)]
pub struct ocfs2_post_refcount {
    pub credits: i32, /* credits it need for journal. */
    pub func: Option<ocfs2_post_refcount_func>, /* real function. */
    pub para: *mut core::ffi::c_void,
}

extern "C" {
    pub fn ocfs2_purge_refcount_trees(osb: *mut ocfs2_super);
    pub fn ocfs2_lock_refcount_tree(
        osb: *mut ocfs2_super,
        ref_blkno: u64,
        rw: i32,
        tree: *mut *mut ocfs2_refcount_tree,
        ref_bh: *mut *mut buffer_head,
    ) -> i32;
    pub fn ocfs2_unlock_refcount_tree(
        osb: *mut ocfs2_super,
        tree: *mut ocfs2_refcount_tree,
        rw: i32,
    );

    pub fn ocfs2_decrease_refcount(
        inode: *mut inode,
        handle: *mut handle_t,
        cpos: u32,
        len: u32,
        meta_ac: *mut ocfs2_alloc_context,
        dealloc: *mut ocfs2_cached_dealloc_ctxt,
        delete: i32,
    ) -> i32;
    pub fn ocfs2_prepare_refcount_change_for_del(
        inode: *mut inode,
        refcount_loc: u64,
        phys_blkno: u64,
        clusters: u32,
        credits: *mut i32,
        ref_blocks: *mut i32,
    ) -> i32;
    pub fn ocfs2_refcount_cow(
        inode: *mut inode,
        di_bh: *mut buffer_head,
        cpos: u32,
        write_len: u32,
        max_cpos: u32,
    ) -> i32;

    pub fn ocfs2_refcounted_xattr_delete_need(
        inode: *mut inode,
        ref_ci: *mut ocfs2_caching_info,
        ref_root_bh: *mut buffer_head,
        xv: *mut ocfs2_xattr_value_root,
        meta_add: *mut i32,
        credits: *mut i32,
    ) -> i32;
    pub fn ocfs2_refcount_cow_xattr(
        inode: *mut inode,
        di: *mut ocfs2_dinode,
        vb: *mut ocfs2_xattr_value_buf,
        ref_tree: *mut ocfs2_refcount_tree,
        ref_root_bh: *mut buffer_head,
        cpos: u32,
        write_len: u32,
        post: *mut ocfs2_post_refcount,
    ) -> i32;
    pub fn ocfs2_duplicate_clusters_by_page(
        handle: *mut handle_t, inode: *mut inode, cpos: u32,
        old_cluster: u32, new_cluster: u32, new_len: u32,
    ) -> i32;
    pub fn ocfs2_duplicate_clusters_by_jbd(
        handle: *mut handle_t, inode: *mut inode, cpos: u32,
        old_cluster: u32, new_cluster: u32, new_len: u32,
    ) -> i32;
    pub fn ocfs2_cow_sync_writeback(
        sb: *mut super_block, inode: *mut inode, cpos: u32, num_clusters: u32,
    ) -> i32;
    pub fn ocfs2_add_refcount_flag(
        inode: *mut inode, data_et: *mut ocfs2_extent_tree,
        ref_ci: *mut ocfs2_caching_info, ref_root_bh: *mut buffer_head,
        cpos: u32, p_cluster: u32, num_clusters: u32,
        dealloc: *mut ocfs2_cached_dealloc_ctxt, post: *mut ocfs2_post_refcount,
    ) -> i32;
    pub fn ocfs2_remove_refcount_tree(inode: *mut inode, di_bh: *mut buffer_head) -> i32;
    pub fn ocfs2_try_remove_refcount_tree(inode: *mut inode, di_bh: *mut buffer_head) -> i32;
    pub fn ocfs2_increase_refcount(
        handle: *mut handle_t, ci: *mut ocfs2_caching_info,
        ref_root_bh: *mut buffer_head, cpos: u64, len: u32,
        meta_ac: *mut ocfs2_alloc_context,
        dealloc: *mut ocfs2_cached_dealloc_ctxt,
    ) -> i32;
    pub fn ocfs2_reflink_ioctl(
        inode: *mut inode, oldname: *const core::ffi::c_char,
        newname: *const core::ffi::c_char, preserve: bool,
    ) -> i32;
    pub fn ocfs2_reflink_remap_blocks(
        s_inode: *mut inode, s_bh: *mut buffer_head, pos_in: i64,
        t_inode: *mut inode, t_bh: *mut buffer_head, pos_out: i64, len: i64,
    ) -> i64;
    pub fn ocfs2_reflink_inodes_lock(
        s_inode: *mut inode, bh1: *mut *mut buffer_head,
        t_inode: *mut inode, bh2: *mut *mut buffer_head,
    ) -> i32;
    pub fn ocfs2_reflink_inodes_unlock(
        s_inode: *mut inode, s_bh: *mut buffer_head,
        t_inode: *mut inode, t_bh: *mut buffer_head,
    );
    pub fn ocfs2_reflink_update_dest(
        dest: *mut inode, d_bh: *mut buffer_head, newlen: i64,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
