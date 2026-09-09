// SPDX-License-Identifier: GPL-2.0
/*
 * QNX6 file system, Linux implementation.
 *
 * Version : 1.0.0
 *
 * History :
 *
 * 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
 * 16-02-2012 pagemap extension by Al Viro
 */

use std::os::raw::{c_char, c_int, c_long, c_uint};

pub const QNX6_LONG_NAME_MAX: c_int = 510;
pub const ENAMETOOLONG: c_long = 36;

#[repr(C)]
pub struct Inode {
    pub i_sb: *mut SuperBlock,
}

#[repr(C)]
pub struct SuperBlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qnx6Name {
    pub name: *const c_char,
    pub len: c_uint,
}

#[repr(C)]
pub struct Dentry {
    pub d_name: Qnx6Name,
}

extern "C" {
    pub fn qnx6_find_ino(len: c_int, dir: *mut Inode, name: *const c_char) -> c_uint;
    pub fn qnx6_iget(sb: *mut SuperBlock, ino: c_uint) -> *mut Inode;
    pub fn IS_ERR(ptr: *mut Inode) -> bool;
    pub fn PTR_ERR(ptr: *mut Inode) -> c_long;
    pub fn d_splice_alias(inode: *mut Inode, dentry: *mut Dentry) -> *mut Dentry;
    pub fn ERR_PTR(error: c_long) -> *mut Dentry;
    pub fn pr_debug(format: *const c_char, ...);
}

pub unsafe extern "C" fn qnx6_lookup(
    dir: *mut Inode,
    dentry: *mut Dentry,
    _flags: c_uint,
) -> *mut Dentry {
    let mut ino: c_uint;
    let mut foundinode: *mut Inode = std::ptr::null_mut();
    let name: *const c_char = (*dentry).d_name.name;
    let len: c_int = (*dentry).d_name.len as c_int;

    if len > QNX6_LONG_NAME_MAX {
        return ERR_PTR(-ENAMETOOLONG);
    }

    ino = qnx6_find_ino(len, dir, name);
    if ino != 0 {
        foundinode = qnx6_iget((*dir).i_sb, ino);
        if IS_ERR(foundinode) {
            let format = b"lookup->iget ->  error %ld\n\0";
            pr_debug(format.as_ptr() as *const c_char, PTR_ERR(foundinode));
        }
    } else {
        let format = b"%s(): not found %s\n\0";
        let function = b"qnx6_lookup\0";
        pr_debug(
            format.as_ptr() as *const c_char,
            function.as_ptr() as *const c_char,
            name,
        );
    }
    d_splice_alias(foundinode, dentry)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
