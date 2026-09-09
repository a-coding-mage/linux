// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext4/symlink.c
 *
 * Only fast symlinks left here - the rest is done by generic code. AV, 1999
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/symlink.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  ext4 symlink handling code
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ext4_encrypted_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const core::ffi::c_char {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let caddr: *const core::ffi::c_void;
    let max_size: u32;
    let paddr: *const core::ffi::c_char;

    if dentry.is_null() {
        return ERR_PTR(-ECHILD);
    }

    if ext4_inode_is_fast_symlink(inode) {
        caddr = unsafe { (*EXT4_I(inode)).i_data.as_ptr() as *const core::ffi::c_void };
        max_size = core::mem::size_of_val(unsafe { &(*EXT4_I(inode)).i_data }) as u32;
    } else {
        bh = ext4_bread(core::ptr::null_mut(), inode, 0, 0);
        if IS_ERR(bh) {
            return ERR_CAST(bh);
        }
        if bh.is_null() {
            EXT4_ERROR_INODE(inode, "bad symlink.");
            return ERR_PTR(-EFSCORRUPTED);
        }
        caddr = unsafe { (*bh).b_data as *const core::ffi::c_void };
        max_size = unsafe { (*(*inode).i_sb).s_blocksize };
    }

    paddr = fscrypt_get_symlink(inode, caddr, max_size, done);
    brelse(bh);
    paddr
}

unsafe fn ext4_encrypted_symlink_getattr(
    idmap: *mut mnt_idmap,
    path: *const path,
    stat: *mut kstat,
    request_mask: u32,
    query_flags: u32,
) -> int {
    ext4_getattr(idmap, path, stat, request_mask, query_flags);
    fscrypt_symlink_getattr(path, stat)
}

unsafe fn ext4_free_link(bh: *mut core::ffi::c_void) {
    brelse(bh as *mut buffer_head);
}

unsafe fn ext4_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    callback: *mut delayed_call,
) -> *const core::ffi::c_char {
    let bh: *mut buffer_head;
    let inline_link: *mut core::ffi::c_char;

    /*
     * Create a new inlined symlink is not supported, just provide a
     * method to read the leftovers.
     */
    if ext4_has_inline_data(inode) {
        if dentry.is_null() {
            return ERR_PTR(-ECHILD);
        }

        inline_link = ext4_read_inline_link(inode);
        if !IS_ERR(inline_link) {
            set_delayed_call(callback, kfree_link, inline_link);
        }
        return inline_link;
    }

    if dentry.is_null() {
        bh = ext4_getblk(
            core::ptr::null_mut(),
            inode,
            0,
            EXT4_GET_BLOCKS_CACHED_NOWAIT,
        );
        if IS_ERR_OR_NULL(bh) {
            return ERR_PTR(-ECHILD);
        }
        if !ext4_buffer_uptodate(bh) {
            brelse(bh);
            return ERR_PTR(-ECHILD);
        }
    } else {
        bh = ext4_bread(core::ptr::null_mut(), inode, 0, 0);
        if IS_ERR(bh) {
            return ERR_CAST(bh);
        }
        if bh.is_null() {
            EXT4_ERROR_INODE(inode, "bad symlink.");
            return ERR_PTR(-EFSCORRUPTED);
        }
    }

    set_delayed_call(callback, ext4_free_link, bh as *mut core::ffi::c_void);
    nd_terminate_link(
        (*bh).b_data,
        (*inode).i_size,
        (*(*inode).i_sb).s_blocksize - 1,
    );
    (*bh).b_data
}

pub static mut ext4_encrypted_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(ext4_encrypted_get_link),
    setattr: Some(ext4_setattr),
    getattr: Some(ext4_encrypted_symlink_getattr),
    listxattr: Some(ext4_listxattr),
};

pub static mut ext4_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(ext4_get_link),
    setattr: Some(ext4_setattr),
    getattr: Some(ext4_getattr),
    listxattr: Some(ext4_listxattr),
};

pub static mut ext4_fast_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(simple_get_link),
    setattr: Some(ext4_setattr),
    getattr: Some(ext4_getattr),
    listxattr: Some(ext4_listxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
