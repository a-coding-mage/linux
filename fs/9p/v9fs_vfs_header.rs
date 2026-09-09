/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V9FS VFS extensions.
 *
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

/* plan9 semantics are that created files are implicitly opened.
 * But linux semantics are that you call create, then open.
 * the plan9 approach is superior as it provides an atomic
 * open.
 * we track the create fid here. When the file is opened, if fidopen is
 * non-zero, we use the fid and can skip some steps.
 * there may be a better way to do this, but I don't know it.
 * one BAD way is to clunk the fid on create, then open it again:
 * you lose the atomicity of file open
 */

/* special case:
 * unlink calls remove, which is an implicit clunk. So we have to track
 * that kind of thing so that we don't try to clunk a dead fid.
 */
pub const P9_LOCK_TIMEOUT: usize = 30 * HZ;

/* flags for v9fs_stat2inode() & v9fs_stat2inode_dotl() */
pub const V9FS_STAT2INODE_KEEP_ISIZE: u32 = 1;

/**
 * struct v9fs_dentry - v9fs specific dentry data
 * @head: List of fid associated with this dentry
 * @expire_time: Lookup cache expiration time for negative dentries
 * @rcu: used by kfree_rcu to schedule clean up job
 */
#[repr(C)]
pub struct v9fs_dentry {
    pub head: hlist_head,
    pub expire_time: u64,
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn to_v9fs_dentry(d: *mut dentry) -> *mut v9fs_dentry {
    (*d).d_fsdata as *mut v9fs_dentry
}

/* External declarations supplied by other translation units. */
unsafe extern "C" {
    pub static mut v9fs_fs_type: file_system_type;
    pub static v9fs_addr_operations: address_space_operations;
    pub static v9fs_file_operations: file_operations;
    pub static v9fs_file_operations_dotl: file_operations;
    pub static v9fs_dir_operations: file_operations;
    pub static v9fs_dir_operations_dotl: file_operations;
    pub static v9fs_dentry_operations: dentry_operations;
    pub fn v9fs_ndentry_refresh_timeout(dentry: *mut dentry);
    pub fn v9fs_dentry_fid_remove(dentry: *mut dentry);
    pub static v9fs_cached_dentry_operations: dentry_operations;
    pub static mut v9fs_inode_cache: *mut kmem_cache;

    pub fn v9fs_alloc_inode(sb: *mut super_block) -> *mut inode;
    pub fn v9fs_free_inode(inode: *mut inode);
    pub fn v9fs_set_netfs_context(inode: *mut inode);
    pub fn v9fs_init_inode(
        v9ses: *mut v9fs_session_info,
        inode: *mut inode,
        mode: umode_t,
        rdev: dev_t,
    ) -> c_int;
    pub fn v9fs_evict_inode(inode: *mut inode);

    /* QID2INO uses the 32-bit form when BITS_PER_LONG == 32. */
    pub fn v9fs_stat2inode(
        stat: *mut p9_wstat,
        inode: *mut inode,
        sb: *mut super_block,
        flags: c_uint,
    );
    pub fn v9fs_stat2inode_dotl(stat: *mut p9_stat_dotl, inode: *mut inode, flags: c_uint);
    pub fn v9fs_dir_release(inode: *mut inode, filp: *mut file) -> c_int;
    pub fn v9fs_file_open(inode: *mut inode, file: *mut file) -> c_int;
    pub fn v9fs_uflags2omode(uflags: c_int, extended: c_int) -> c_int;

    pub fn v9fs_blank_wstat(wstat: *mut p9_wstat);
    pub fn v9fs_vfs_setattr_dotl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        iattr: *mut iattr,
    ) -> c_int;
    pub fn v9fs_file_fsync_dotl(
        filp: *mut file,
        start: loff_t,
        end: loff_t,
        datasync: c_int,
    ) -> c_int;
    pub fn v9fs_refresh_inode(fid: *mut p9_fid, inode: *mut inode) -> c_int;
    pub fn v9fs_refresh_inode_dotl(fid: *mut p9_fid, inode: *mut inode) -> c_int;

    pub fn v9fs_open_to_dotl_flags(flags: c_int) -> c_int;
}

#[inline]
pub unsafe fn v9fs_invalidate_inode_attr(inode: *mut inode) {
    let v9inode: *mut v9fs_inode = V9FS_I(inode);
    (*v9inode).cache_validity |= V9FS_INO_INVALID_ATTR;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
