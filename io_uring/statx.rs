// SPDX-License-Identifier: GPL-2.0
// Translated from statx.c. Kernel declarations supplied by the surrounding
// source tree are intentionally referenced rather than reimplemented here.

#[repr(C)]
pub struct io_statx {
    pub file: *mut file,
    pub dfd: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub filename: delayed_filename,
    pub buffer: *mut statx,
}

pub unsafe fn io_statx_prep(
    req: *mut io_kiocb,
    sqe: *const io_uring_sqe,
) -> ::core::ffi::c_int {
    let sx: *mut io_statx = io_kiocb_to_cmd(req);
    let mut path: *const ::core::ffi::c_char;
    let mut ret: ::core::ffi::c_int;

    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }
    if (*req).flags & REQ_F_FIXED_FILE != 0 {
        return -EBADF;
    }

    (*sx).dfd = READ_ONCE((*sqe).fd);
    (*sx).mask = READ_ONCE((*sqe).len);
    path = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    (*sx).buffer = u64_to_user_ptr(READ_ONCE((*sqe).addr2));
    (*sx).flags = READ_ONCE((*sqe).statx_flags);

    ret = delayed_getname_uflags(&mut (*sx).filename, path, (*sx).flags);

    if unlikely(ret != 0) {
        return ret;
    }

    (*req).flags |= REQ_F_NEED_CLEANUP;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_statx(
    req: *mut io_kiocb,
    issue_flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let sx: *mut io_statx = io_kiocb_to_cmd(req);
    let name = filename_complete_delayed(&mut (*sx).filename);
    let mut ret: ::core::ffi::c_int;

    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK != 0);

    ret = do_statx((*sx).dfd, name, (*sx).flags, (*sx).mask, (*sx).buffer);
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_statx_cleanup(req: *mut io_kiocb) {
    let sx: *mut io_statx = io_kiocb_to_cmd(req);

    dismiss_delayed_filename(&mut (*sx).filename);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
