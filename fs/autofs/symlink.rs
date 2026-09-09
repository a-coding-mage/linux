// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 */

// Dependency intent: declarations supplied by autofs_i.h and other repository
// sources are referenced here; C preprocessor inclusion is omitted.

use core::ffi::c_char;

unsafe fn autofs_get_link(
    dentry: *mut dentry,
    _inode: *mut inode,
    _done: *mut delayed_call,
) -> *const c_char {
    if dentry.is_null() {
        return ERR_PTR((-ECHILD) as isize);
    }

    let sbi = autofs_sbi((*dentry).d_sb);
    let ino = autofs_dentry_ino(dentry);
    if !ino.is_null() && !autofs_oz_mode(sbi) {
        (*ino).last_used = jiffies;
    }
    d_inode(dentry).i_private
}

#[repr(C)]
pub struct inode_operations {
    pub get_link: unsafe fn(
        *mut dentry,
        *mut inode,
        *mut delayed_call,
    ) -> *const c_char,
}

pub static autofs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: autofs_get_link,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
