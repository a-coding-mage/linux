// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfs/attr.c
 *
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Export hfs data via xattr
 */

// Linux and HFS declarations are supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
enum hfs_xattr_type {
    HFS_TYPE,
    HFS_CREATOR,
}

unsafe fn __hfs_setxattr(
    inode: *mut inode,
    type_: hfs_xattr_type,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    let mut fd: hfs_find_data = core::mem::zeroed();
    let mut rec: hfs_cat_rec = core::mem::zeroed();
    let file: *mut hfs_cat_file;
    let mut res: i32;

    if !S_ISREG((*inode).i_mode) || HFS_IS_RSRC(inode) {
        return -EOPNOTSUPP;
    }

    res = hfs_find_init((*HFS_SB((*inode).i_sb)).cat_tree, &mut fd);
    if res != 0 {
        return res;
    }
    (*fd.search_key).cat = (*HFS_I(inode)).cat_key;
    res = hfs_brec_find(&mut fd);
    if res != 0 {
        hfs_find_exit(&mut fd);
        return res;
    }
    hfs_bnode_read(
        fd.bnode,
        &mut rec,
        fd.entryoffset,
        core::mem::size_of::<hfs_cat_file>(),
    );
    file = &mut rec.file;

    match type_ {
        hfs_xattr_type::HFS_TYPE => {
            if size == 4 {
                core::ptr::copy_nonoverlapping(value, &mut (*file).UsrWds.fdType as *mut _ as *mut core::ffi::c_void, 4);
            } else {
                res = -ERANGE;
            }
        }
        hfs_xattr_type::HFS_CREATOR => {
            if size == 4 {
                core::ptr::copy_nonoverlapping(value, &mut (*file).UsrWds.fdCreator as *mut _ as *mut core::ffi::c_void, 4);
            } else {
                res = -ERANGE;
            }
        }
    }

    if res == 0 {
        hfs_bnode_write(
            fd.bnode,
            &rec,
            fd.entryoffset,
            core::mem::size_of::<hfs_cat_file>(),
        );
    }
    hfs_find_exit(&mut fd);
    res
}

unsafe fn __hfs_getxattr(
    inode: *mut inode,
    type_: hfs_xattr_type,
    value: *mut core::ffi::c_void,
    size: usize,
) -> isize {
    let mut fd: hfs_find_data = core::mem::zeroed();
    let mut rec: hfs_cat_rec = core::mem::zeroed();
    let file: *mut hfs_cat_file;
    let mut res: isize = 0;

    if !S_ISREG((*inode).i_mode) || HFS_IS_RSRC(inode) {
        return -EOPNOTSUPP as isize;
    }

    if size != 0 {
        res = hfs_find_init((*HFS_SB((*inode).i_sb)).cat_tree, &mut fd) as isize;
        if res != 0 {
            return res;
        }
        (*fd.search_key).cat = (*HFS_I(inode)).cat_key;
        res = hfs_brec_find(&mut fd) as isize;
        if res != 0 {
            hfs_find_exit(&mut fd);
            return res;
        }
        hfs_bnode_read(fd.bnode, &mut rec, fd.entryoffset, core::mem::size_of::<hfs_cat_file>());
    }
    file = &mut rec.file;

    match type_ {
        hfs_xattr_type::HFS_TYPE => {
            if size >= 4 {
                core::ptr::copy_nonoverlapping(&(*file).UsrWds.fdType as *const _ as *const core::ffi::c_void, value, 4);
                res = 4;
            } else {
                res = if size != 0 { -ERANGE as isize } else { 4 };
            }
        }
        hfs_xattr_type::HFS_CREATOR => {
            if size >= 4 {
                core::ptr::copy_nonoverlapping(&(*file).UsrWds.fdCreator as *const _ as *const core::ffi::c_void, value, 4);
                res = 4;
            } else {
                res = if size != 0 { -ERANGE as isize } else { 4 };
            }
        }
    }

    if size != 0 {
        hfs_find_exit(&mut fd);
    }
    res
}

unsafe extern "C" fn hfs_xattr_get(
    handler: *const xattr_handler,
    _unused: *mut dentry,
    inode: *mut inode,
    _name: *const core::ffi::c_char,
    value: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    __hfs_getxattr(inode, (*handler).flags, value, size) as i32
}

unsafe extern "C" fn hfs_xattr_set(
    handler: *const xattr_handler,
    _idmap: *mut mnt_idmap,
    _unused: *mut dentry,
    inode: *mut inode,
    _name: *const core::ffi::c_char,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
) -> i32 {
    if value.is_null() {
        return -EOPNOTSUPP;
    }
    __hfs_setxattr(inode, (*handler).flags, value, size, flags)
}

static hfs_creator_handler: xattr_handler = xattr_handler {
    name: b"hfs.creator\0" as *const u8 as *const core::ffi::c_char,
    flags: hfs_xattr_type::HFS_CREATOR,
    get: Some(hfs_xattr_get),
    set: Some(hfs_xattr_set),
};

static hfs_type_handler: xattr_handler = xattr_handler {
    name: b"hfs.type\0" as *const u8 as *const core::ffi::c_char,
    flags: hfs_xattr_type::HFS_TYPE,
    get: Some(hfs_xattr_get),
    set: Some(hfs_xattr_set),
};

#[no_mangle]
pub static hfs_xattr_handlers: [*const xattr_handler; 3] = [
    &hfs_creator_handler,
    &hfs_type_handler,
    core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
