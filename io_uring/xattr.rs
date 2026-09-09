// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation in xattr.c.
// Dependencies supplied by the surrounding kernel/io_uring translation are
// intentionally referenced but not implemented here.

#[repr(C)]
pub struct io_xattr {
    pub file: *mut file,
    pub ctx: kernel_xattr_ctx,
    pub filename: delayed_filename,
}

pub unsafe fn io_xattr_cleanup(req: *mut io_kiocb) {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);

    dismiss_delayed_filename(&mut (*ix).filename);
    kfree((*ix).ctx.kname as *mut core::ffi::c_void);
    kvfree((*ix).ctx.kvalue as *mut core::ffi::c_void);
}

unsafe fn io_xattr_finish(req: *mut io_kiocb, ret: i32) {
    (*req).flags &= !REQ_F_NEED_CLEANUP;

    io_xattr_cleanup(req);
    io_req_set_res(req, ret, 0);
}

unsafe fn __io_getxattr_prep(
    req: *mut io_kiocb,
    sqe: *const io_uring_sqe,
) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let name: *const core::ffi::c_char;
    let mut ret: i32;

    INIT_DELAYED_FILENAME!(&mut (*ix).filename);
    (*ix).ctx.kvalue = core::ptr::null_mut();
    name = u64_to_user_ptr(READ_ONCE!((*sqe).addr));
    (*ix).ctx.value = u64_to_user_ptr(READ_ONCE!((*sqe).addr2));
    (*ix).ctx.size = READ_ONCE!((*sqe).len);
    (*ix).ctx.flags = READ_ONCE!((*sqe).xattr_flags);

    if (*ix).ctx.flags != 0 {
        return -EINVAL;
    }

    (*ix).ctx.kname = kmalloc_obj!((*ix).ctx.kname);
    if (*ix).ctx.kname.is_null() {
        return -ENOMEM;
    }

    ret = import_xattr_name((*ix).ctx.kname, name);
    if ret != 0 {
        kfree((*ix).ctx.kname as *mut core::ffi::c_void);
        return ret;
    }

    (*req).flags |= REQ_F_NEED_CLEANUP;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_fgetxattr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    __io_getxattr_prep(req, sqe)
}

pub unsafe fn io_getxattr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let path: *const core::ffi::c_char;
    let mut ret: i32;

    if unlikely!((*req).flags & REQ_F_FIXED_FILE != 0) {
        return -EBADF;
    }

    ret = __io_getxattr_prep(req, sqe);
    if ret != 0 {
        return ret;
    }

    path = u64_to_user_ptr(READ_ONCE!((*sqe).addr3));
    delayed_getname(&mut (*ix).filename, path)
}

pub unsafe fn io_fgetxattr(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let ret: i32;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK != 0);
    ret = file_getxattr((*req).file, &mut (*ix).ctx);
    io_xattr_finish(req, ret);
    IOU_COMPLETE
}

pub unsafe fn io_getxattr(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let name = filename_complete_delayed!(&mut (*ix).filename);
    let ret: i32;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK != 0);
    ret = filename_getxattr(AT_FDCWD, name, LOOKUP_FOLLOW, &mut (*ix).ctx);
    io_xattr_finish(req, ret);
    IOU_COMPLETE
}

unsafe fn __io_setxattr_prep(
    req: *mut io_kiocb,
    sqe: *const io_uring_sqe,
) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let name: *const core::ffi::c_char;
    let mut ret: i32;

    INIT_DELAYED_FILENAME!(&mut (*ix).filename);
    name = u64_to_user_ptr(READ_ONCE!((*sqe).addr));
    (*ix).ctx.cvalue = u64_to_user_ptr(READ_ONCE!((*sqe).addr2));
    (*ix).ctx.kvalue = core::ptr::null_mut();
    (*ix).ctx.size = READ_ONCE!((*sqe).len);
    (*ix).ctx.flags = READ_ONCE!((*sqe).xattr_flags);

    (*ix).ctx.kname = kmalloc_obj!((*ix).ctx.kname);
    if (*ix).ctx.kname.is_null() {
        return -ENOMEM;
    }

    ret = setxattr_copy(name, &mut (*ix).ctx);
    if ret != 0 {
        kfree((*ix).ctx.kname as *mut core::ffi::c_void);
        return ret;
    }

    (*req).flags |= REQ_F_NEED_CLEANUP;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_setxattr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let path: *const core::ffi::c_char;
    let mut ret: i32;

    if unlikely!((*req).flags & REQ_F_FIXED_FILE != 0) {
        return -EBADF;
    }
    ret = __io_setxattr_prep(req, sqe);
    if ret != 0 {
        return ret;
    }
    path = u64_to_user_ptr(READ_ONCE!((*sqe).addr3));
    delayed_getname(&mut (*ix).filename, path)
}

pub unsafe fn io_fsetxattr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    __io_setxattr_prep(req, sqe)
}

pub unsafe fn io_fsetxattr(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let ret: i32;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK != 0);
    ret = file_setxattr((*req).file, &mut (*ix).ctx);
    io_xattr_finish(req, ret);
    IOU_COMPLETE
}

pub unsafe fn io_setxattr(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let ix: *mut io_xattr = io_kiocb_to_cmd(req);
    let name = filename_complete_delayed!(&mut (*ix).filename);
    let ret: i32;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK != 0);
    ret = filename_setxattr(AT_FDCWD, name, LOOKUP_FOLLOW, &mut (*ix).ctx);
    io_xattr_finish(req, ret);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
