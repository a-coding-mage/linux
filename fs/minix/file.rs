// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/minix/file.c
 *
 *  Copyright (C) 1991, 1992 Linus Torvalds
 *
 *  minix regular file handling primitives
 */

// Dependencies supplied by the Linux kernel and minix filesystem headers.

/*
 * We have mostly NULLs here: the current defaults are OK for
 * the minix filesystem.
 */
pub const minix_file_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter),
    mmap_prepare: Some(generic_file_mmap_prepare),
    fsync: Some(simple_fsync),
    splice_read: Some(filemap_splice_read),
};

unsafe fn minix_setattr(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    attr: *mut iattr,
) -> i32 {
    let inode: *mut inode = unsafe { d_inode(dentry) };
    let mut error: i32;

    error = unsafe { setattr_prepare(&nop_mnt_idmap, dentry, attr) };
    if error != 0 {
        return error;
    }

    if unsafe { ((*attr).ia_valid & ATTR_SIZE) != 0 }
        && unsafe { (*attr).ia_size != i_size_read(inode) }
    {
        error = unsafe { inode_newsize_ok(inode, (*attr).ia_size) };
        if error != 0 {
            return error;
        }

        unsafe {
            truncate_setsize(inode, (*attr).ia_size);
            minix_truncate(inode);
        }
    }

    unsafe {
        setattr_copy(&nop_mnt_idmap, inode, attr);
        mark_inode_dirty(inode);
    }
    0
}

pub const minix_file_inode_operations: inode_operations = inode_operations {
    setattr: Some(minix_setattr),
    getattr: Some(minix_getattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
