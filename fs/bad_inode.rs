// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/bad_inode.c
 *
 *  Copyright (C) 1997, Stephen Tweedie
 *
 *  Provide stub functions for unreadable inodes
 *
 *  Fabian Frederick : August 2003 - All file operations assigned to EIO
 */

use core::ffi::c_char;

unsafe fn bad_file_open(_inode: *mut inode, _filp: *mut file) -> c_int {
    -EIO
}

static bad_file_ops: file_operations = file_operations {
    open: Some(bad_file_open),
};

unsafe fn bad_inode_create(
    _idmap: *mut mnt_idmap,
    _dir: *mut inode,
    _dentry: *mut dentry,
    _mode: umode_t,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_lookup(
    _dir: *mut inode,
    _dentry: *mut dentry,
    _flags: c_uint,
) -> *mut dentry {
    ERR_PTR(-EIO)
}

unsafe fn bad_inode_link(
    _old_dentry: *mut dentry,
    _dir: *mut inode,
    _dentry: *mut dentry,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_unlink(_dir: *mut inode, _dentry: *mut dentry) -> c_int {
    -EIO
}

unsafe fn bad_inode_symlink(
    _idmap: *mut mnt_idmap,
    _dir: *mut inode,
    _dentry: *mut dentry,
    _symname: *const c_char,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_mkdir(
    _idmap: *mut mnt_idmap,
    _dir: *mut inode,
    _dentry: *mut dentry,
    _mode: umode_t,
) -> *mut dentry {
    ERR_PTR(-EIO)
}

unsafe fn bad_inode_rmdir(_dir: *mut inode, _dentry: *mut dentry) -> c_int {
    -EIO
}

unsafe fn bad_inode_mknod(
    _idmap: *mut mnt_idmap,
    _dir: *mut inode,
    _dentry: *mut dentry,
    _mode: umode_t,
    _rdev: dev_t,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_rename2(
    _idmap: *mut mnt_idmap,
    _old_dir: *mut inode,
    _old_dentry: *mut dentry,
    _new_dir: *mut inode,
    _new_dentry: *mut dentry,
    _flags: c_uint,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_readlink(
    _dentry: *mut dentry,
    _buffer: *mut c_char,
    _buflen: c_int,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_permission(
    _idmap: *mut mnt_idmap,
    _inode: *mut inode,
    _mask: c_int,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_getattr(
    _idmap: *mut mnt_idmap,
    _path: *const path,
    _stat: *mut kstat,
    _request_mask: u32,
    _query_flags: c_uint,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_setattr(
    _idmap: *mut mnt_idmap,
    _direntry: *mut dentry,
    _attrs: *mut iattr,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_listxattr(
    _dentry: *mut dentry,
    _buffer: *mut c_char,
    _buffer_size: usize,
) -> isize {
    -EIO as isize
}

unsafe fn bad_inode_get_link(
    _dentry: *mut dentry,
    _inode: *mut inode,
    _done: *mut delayed_call,
) -> *const c_char {
    ERR_PTR(-EIO)
}

unsafe fn bad_inode_get_acl(
    _inode: *mut inode,
    _type: c_int,
    _rcu: bool,
) -> *mut posix_acl {
    ERR_PTR(-EIO)
}

unsafe fn bad_inode_fiemap(
    _inode: *mut inode,
    _fieinfo: *mut fiemap_extent_info,
    _start: u64,
    _len: u64,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_update_time(
    _inode: *mut inode,
    _type: fs_update_time,
    _flags: c_uint,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_atomic_open(
    _inode: *mut inode,
    _dentry: *mut dentry,
    _file: *mut file,
    _open_flag: c_uint,
    _create_mode: umode_t,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_tmpfile(
    _idmap: *mut mnt_idmap,
    _inode: *mut inode,
    _file: *mut file,
    _mode: umode_t,
) -> c_int {
    -EIO
}

unsafe fn bad_inode_set_acl(
    _idmap: *mut mnt_idmap,
    _dentry: *mut dentry,
    _acl: *mut posix_acl,
    _type: c_int,
) -> c_int {
    -EIO
}

static bad_inode_ops: inode_operations = inode_operations {
    create: Some(bad_inode_create),
    lookup: Some(bad_inode_lookup),
    link: Some(bad_inode_link),
    unlink: Some(bad_inode_unlink),
    symlink: Some(bad_inode_symlink),
    mkdir: Some(bad_inode_mkdir),
    rmdir: Some(bad_inode_rmdir),
    mknod: Some(bad_inode_mknod),
    rename: Some(bad_inode_rename2),
    readlink: Some(bad_inode_readlink),
    permission: Some(bad_inode_permission),
    getattr: Some(bad_inode_getattr),
    setattr: Some(bad_inode_setattr),
    listxattr: Some(bad_inode_listxattr),
    get_link: Some(bad_inode_get_link),
    get_inode_acl: Some(bad_inode_get_acl),
    fiemap: Some(bad_inode_fiemap),
    update_time: Some(bad_inode_update_time),
    atomic_open: Some(bad_inode_atomic_open),
    tmpfile: Some(bad_inode_tmpfile),
    set_acl: Some(bad_inode_set_acl),
};

/*
 * When a filesystem is unable to read an inode due to an I/O error in
 * its read_inode() function, it can call make_bad_inode() to return a
 * set of stubs which will return EIO errors as required.
 *
 * We only need to do limited initialisation: all other fields are
 * preinitialised to zero automatically.
 */

/// mark an inode bad due to an I/O error
///
/// When an inode cannot be read due to a media or remote network
/// failure this function makes the inode "bad" and causes I/O operations
/// on it to fail from this point on.
pub unsafe fn make_bad_inode(inode: *mut inode) {
    remove_inode_hash(inode);

    (*inode).i_mode = S_IFREG;
    simple_inode_init_ts(inode);
    (*inode).i_op = &bad_inode_ops;
    (*inode).i_opflags &= !IOP_XATTR;
    (*inode).i_fop = &bad_file_ops;
}

/*
 * This tests whether an inode has been flagged as bad. The test uses
 * &bad_inode_ops to cover the case of invalidated inodes as well as
 * those created by make_bad_inode() above.
 */

/// is an inode errored
///
/// Returns true if the inode in question has been marked as bad.
pub unsafe fn is_bad_inode(inode: *mut inode) -> bool {
    (*inode).i_op == &bad_inode_ops
}

/// Mark an under-construction inode as dead and release it.
pub unsafe fn iget_failed(inode: *mut inode) {
    make_bad_inode(inode);
    unlock_new_inode(inode);
    iput(inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
