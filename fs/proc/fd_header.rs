/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <linux/fs.h> are supplied externally.

extern "C" {
    pub static proc_fd_operations: file_operations;
    pub static proc_fd_inode_operations: inode_operations;

    pub static proc_fdinfo_operations: file_operations;
    pub static proc_fdinfo_inode_operations: inode_operations;

    pub fn proc_fd_permission(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        mask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

/// Direct translation of the C `static inline` accessor.
#[inline]
pub unsafe fn proc_fd(inode: *mut inode) -> ::std::os::raw::c_uint {
    (*PROC_I!(inode)).fd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
