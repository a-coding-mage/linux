// SPDX-License-Identifier: GPL-2.0
/*
 * FUSE: Filesystem in Userspace
 * Copyright (C) 2001-2016  Miklos Szeredi <miklos@szeredi.hu>
 */

// Dependencies supplied by the surrounding kernel/FUSE sources:
// fuse_i.h, linux/xattr.h, and linux/posix_acl_xattr.h.

pub unsafe fn fuse_setxattr(
    inode: *mut inode,
    name: *const core::ffi::c_char,
    value: *const core::ffi::c_void,
    size: usize,
    flags: i32,
    extra_flags: u32,
) -> i32 {
    let fm = get_fuse_mount(inode);
    let mut args: fuse_args = core::mem::zeroed();
    let mut inarg: fuse_setxattr_in = core::mem::zeroed();
    let mut err: i32;

    if (*(*fm).fc).no_setxattr {
        return -EOPNOTSUPP;
    }

    inarg.size = size;
    inarg.flags = flags;
    inarg.setxattr_flags = extra_flags;

    args.opcode = FUSE_SETXATTR;
    args.nodeid = get_node_id(inode);
    args.in_numargs = 3;
    args.in_args[0].size = if (*(*fm).fc).setxattr_ext {
        core::mem::size_of::<fuse_setxattr_in>()
    } else {
        FUSE_COMPAT_SETXATTR_IN_SIZE
    };
    args.in_args[0].value = &mut inarg as *mut _ as *mut core::ffi::c_void;
    args.in_args[1].size = libc::strlen(name) + 1;
    args.in_args[1].value = name as *mut core::ffi::c_char as *mut core::ffi::c_void;
    args.in_args[2].size = size;
    args.in_args[2].value = value as *mut core::ffi::c_void;
    err = fuse_simple_request(fm, &mut args);
    if err == -ENOSYS {
        (*(*fm).fc).no_setxattr = 1;
        err = -EOPNOTSUPP;
    }
    if err == 0 {
        fuse_update_ctime(inode);
    }
    err
}

pub unsafe fn fuse_getxattr(
    inode: *mut inode,
    name: *const core::ffi::c_char,
    value: *mut core::ffi::c_void,
    size: usize,
) -> isize {
    let fm = get_fuse_mount(inode);
    let mut args: fuse_args = core::mem::zeroed();
    let mut inarg: fuse_getxattr_in = core::mem::zeroed();
    let mut outarg: fuse_getxattr_out = core::mem::zeroed();
    let mut ret: isize;

    if (*(*fm).fc).no_getxattr { return -EOPNOTSUPP as isize; }
    inarg.size = size;
    args.opcode = FUSE_GETXATTR;
    args.nodeid = get_node_id(inode);
    args.in_numargs = 2;
    args.in_args[0].size = core::mem::size_of::<fuse_getxattr_in>();
    args.in_args[0].value = &mut inarg as *mut _ as *mut core::ffi::c_void;
    args.in_args[1].size = libc::strlen(name) + 1;
    args.in_args[1].value = name as *mut core::ffi::c_char as *mut core::ffi::c_void;
    // This is really two different operations rolled into one
    args.out_numargs = 1;
    if size != 0 {
        args.out_argvar = true;
        args.out_args[0].size = size;
        args.out_args[0].value = value;
    } else {
        args.out_args[0].size = core::mem::size_of::<fuse_getxattr_out>();
        args.out_args[0].value = &mut outarg as *mut _ as *mut core::ffi::c_void;
    }
    ret = fuse_simple_request(fm, &mut args) as isize;
    if ret == 0 && size == 0 { ret = core::cmp::min(outarg.size, XATTR_SIZE_MAX) as isize; }
    if ret == -ENOSYS as isize {
        (*(*fm).fc).no_getxattr = 1;
        ret = -EOPNOTSUPP as isize;
    }
    ret
}

unsafe fn fuse_verify_xattr_list(mut list: *mut core::ffi::c_char, mut size: usize) -> i32 {
    let origsize = size;
    while size != 0 {
        let thislen = libc::strnlen(list, size);
        if thislen == 0 || thislen == size { return -EIO; }
        size -= thislen + 1;
        list = list.add(thislen + 1);
    }
    origsize as i32
}

pub unsafe fn fuse_listxattr(entry: *mut dentry, list: *mut core::ffi::c_char, size: usize) -> isize {
    let inode = d_inode(entry);
    let fm = get_fuse_mount(inode);
    let mut args: fuse_args = core::mem::zeroed();
    let mut inarg: fuse_getxattr_in = core::mem::zeroed();
    let mut outarg: fuse_getxattr_out = core::mem::zeroed();
    let mut ret: isize;
    if fuse_is_bad(inode) { return -EIO as isize; }
    if !fuse_allow_current_process((*fm).fc) { return -EACCES as isize; }
    if (*(*fm).fc).no_listxattr { return -EOPNOTSUPP as isize; }
    inarg.size = size;
    args.opcode = FUSE_LISTXATTR;
    args.nodeid = get_node_id(inode);
    args.in_numargs = 1;
    args.in_args[0].size = core::mem::size_of::<fuse_getxattr_in>();
    args.in_args[0].value = &mut inarg as *mut _ as *mut core::ffi::c_void;
    // This is really two different operations rolled into one
    args.out_numargs = 1;
    if size != 0 { args.out_argvar = true; args.out_args[0].size = size; args.out_args[0].value = list as *mut core::ffi::c_void; }
    else { args.out_args[0].size = core::mem::size_of::<fuse_getxattr_out>(); args.out_args[0].value = &mut outarg as *mut _ as *mut core::ffi::c_void; }
    ret = fuse_simple_request(fm, &mut args) as isize;
    if ret == 0 && size == 0 { ret = core::cmp::min(outarg.size, XATTR_LIST_MAX) as isize; }
    if ret > 0 && size != 0 { ret = fuse_verify_xattr_list(list, ret as usize) as isize; }
    if ret == -ENOSYS as isize { (*(*fm).fc).no_listxattr = 1; ret = -EOPNOTSUPP as isize; }
    ret
}

pub unsafe fn fuse_removexattr(inode: *mut inode, name: *const core::ffi::c_char) -> i32 {
    let fm = get_fuse_mount(inode);
    let mut args: fuse_args = core::mem::zeroed();
    if (*(*fm).fc).no_removexattr { return -EOPNOTSUPP; }
    args.opcode = FUSE_REMOVEXATTR;
    args.nodeid = get_node_id(inode);
    args.in_numargs = 2;
    fuse_set_zero_arg0(&mut args);
    args.in_args[1].size = libc::strlen(name) + 1;
    args.in_args[1].value = name as *mut core::ffi::c_char as *mut core::ffi::c_void;
    let mut err = fuse_simple_request(fm, &mut args);
    if err == -ENOSYS { (*(*fm).fc).no_removexattr = 1; err = -EOPNOTSUPP; }
    if err == 0 { fuse_update_ctime(inode); }
    err
}

unsafe fn fuse_xattr_get(_handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode, name: *const core::ffi::c_char, value: *mut core::ffi::c_void, size: usize) -> i32 {
    if fuse_is_bad(inode) { return -EIO; }
    fuse_getxattr(inode, name, value, size) as i32
}

unsafe fn fuse_xattr_set(_handler: *const xattr_handler, _idmap: *mut mnt_idmap, _dentry: *mut dentry, inode: *mut inode, name: *const core::ffi::c_char, value: *const core::ffi::c_void, size: usize, flags: i32) -> i32 {
    if fuse_is_bad(inode) { return -EIO; }
    if value.is_null() { return fuse_removexattr(inode, name); }
    fuse_setxattr(inode, name, value, size, flags, 0)
}

static fuse_xattr_handler: xattr_handler = xattr_handler { prefix: b"\0".as_ptr() as *const core::ffi::c_char, get: Some(fuse_xattr_get), set: Some(fuse_xattr_set) };

#[no_mangle]
pub static fuse_xattr_handlers: [*const xattr_handler; 2] = [&fuse_xattr_handler, core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
