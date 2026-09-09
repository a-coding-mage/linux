/* SPDX-License-Identifier: GPL-2.0 */

/* This file defines generic functions used primarily by stackable
 * filesystems; none of these functions require i_rwsem to be held.
 *
 * The declarations below correspond to the types and functions supplied by
 * <linux/fs.h> in the original header.
 */

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    _private: [u8; 0],
}

extern "C" {
    /* externs for fs/stack.c */
    pub fn fsstack_copy_attr_all(dest: *mut inode, src: *const inode);
    pub fn fsstack_copy_inode_size(dst: *mut inode, src: *mut inode);

    pub fn inode_get_atime(inode: *const inode) -> timespec;
    pub fn inode_set_atime_to_ts(inode: *mut inode, ts: timespec);
    pub fn inode_get_mtime(inode: *const inode) -> timespec;
    pub fn inode_set_mtime_to_ts(inode: *mut inode, ts: timespec);
    pub fn inode_get_ctime(inode: *const inode) -> timespec;
    pub fn inode_set_ctime_to_ts(inode: *mut inode, ts: timespec);
}

/* inlines */
#[inline]
pub unsafe fn fsstack_copy_attr_atime(dest: *mut inode, src: *const inode) {
    inode_set_atime_to_ts(dest, inode_get_atime(src));
}

#[inline]
pub unsafe fn fsstack_copy_attr_times(dest: *mut inode, src: *const inode) {
    inode_set_atime_to_ts(dest, inode_get_atime(src));
    inode_set_mtime_to_ts(dest, inode_get_mtime(src));
    inode_set_ctime_to_ts(dest, inode_get_ctime(src));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
