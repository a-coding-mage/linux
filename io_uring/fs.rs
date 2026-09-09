// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/io_uring translation.

#[repr(C)]
pub struct io_rename {
    pub file: *mut file,
    pub old_dfd: i32,
    pub new_dfd: i32,
    pub oldpath: delayed_filename,
    pub newpath: delayed_filename,
    pub flags: i32,
}

#[repr(C)]
pub struct io_unlink {
    pub file: *mut file,
    pub dfd: i32,
    pub flags: i32,
    pub filename: delayed_filename,
}

#[repr(C)]
pub struct io_mkdir {
    pub file: *mut file,
    pub dfd: i32,
    pub mode: umode_t,
    pub filename: delayed_filename,
}

#[repr(C)]
pub struct io_link {
    pub file: *mut file,
    pub old_dfd: i32,
    pub new_dfd: i32,
    pub oldpath: delayed_filename,
    pub newpath: delayed_filename,
    pub flags: i32,
}

pub unsafe fn io_renameat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let ren: *mut io_rename = io_kiocb_to_cmd(req);
    let oldf: *const core::ffi::c_char;
    let newf: *const core::ffi::c_char;
    let mut err: i32;

    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    if ((*req).flags & REQ_F_FIXED_FILE) != 0 { return -EBADF; }

    (*ren).old_dfd = core::ptr::read_volatile(&(*sqe).fd);
    oldf = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr));
    newf = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr2));
    (*ren).new_dfd = core::ptr::read_volatile(&(*sqe).len);
    (*ren).flags = core::ptr::read_volatile(&(*sqe).rename_flags);

    err = delayed_getname(&mut (*ren).oldpath, oldf);
    if err != 0 { return err; }
    err = delayed_getname(&mut (*ren).newpath, newf);
    if err != 0 {
        dismiss_delayed_filename(&mut (*ren).oldpath);
        return err;
    }
    (*req).flags |= REQ_F_NEED_CLEANUP;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_renameat(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let ren: *mut io_rename = io_kiocb_to_cmd(req);
    let old = filename_complete_delayed(&mut (*ren).oldpath);
    let new = filename_complete_delayed(&mut (*ren).newpath);
    let ret: i32;
    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    ret = filename_renameat2((*ren).old_dfd, old, (*ren).new_dfd, new, (*ren).flags);
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_renameat_cleanup(req: *mut io_kiocb) {
    let ren: *mut io_rename = io_kiocb_to_cmd(req);
    dismiss_delayed_filename(&mut (*ren).oldpath);
    dismiss_delayed_filename(&mut (*ren).newpath);
}

pub unsafe fn io_unlinkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let un: *mut io_unlink = io_kiocb_to_cmd(req);
    let fname: *const core::ffi::c_char;
    let err: i32;
    if (*sqe).off != 0 || (*sqe).len != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 || (*sqe).addr3 != 0 || (*sqe).__pad2[0] != 0 { return -EINVAL; }
    if ((*req).flags & REQ_F_FIXED_FILE) != 0 { return -EBADF; }
    (*un).dfd = core::ptr::read_volatile(&(*sqe).fd);
    (*un).flags = core::ptr::read_volatile(&(*sqe).unlink_flags);
    if (*un).flags & !AT_REMOVEDIR != 0 { return -EINVAL; }
    fname = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr));
    err = delayed_getname(&mut (*un).filename, fname);
    if err != 0 { return err; }
    (*req).flags |= REQ_F_NEED_CLEANUP | REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_unlinkat(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let un: *mut io_unlink = io_kiocb_to_cmd(req);
    let name = filename_complete_delayed(&mut (*un).filename);
    let ret: i32;
    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    if (*un).flags & AT_REMOVEDIR != 0 { ret = filename_rmdir((*un).dfd, name); } else { ret = filename_unlinkat((*un).dfd, name); }
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_unlinkat_cleanup(req: *mut io_kiocb) {
    let ul: *mut io_unlink = io_kiocb_to_cmd(req);
    dismiss_delayed_filename(&mut (*ul).filename);
}

pub unsafe fn io_mkdirat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let mkd: *mut io_mkdir = io_kiocb_to_cmd(req);
    let fname: *const core::ffi::c_char;
    let err: i32;
    if (*sqe).off != 0 || (*sqe).rw_flags != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    if ((*req).flags & REQ_F_FIXED_FILE) != 0 { return -EBADF; }
    (*mkd).dfd = core::ptr::read_volatile(&(*sqe).fd);
    (*mkd).mode = core::ptr::read_volatile(&(*sqe).len);
    fname = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr));
    err = delayed_getname(&mut (*mkd).filename, fname);
    if err != 0 { return err; }
    (*req).flags |= REQ_F_NEED_CLEANUP | REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_mkdirat(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let mkd: *mut io_mkdir = io_kiocb_to_cmd(req);
    let name = filename_complete_delayed(&mut (*mkd).filename);
    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    let ret = filename_mkdirat((*mkd).dfd, name, (*mkd).mode);
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_mkdirat_cleanup(req: *mut io_kiocb) {
    let md: *mut io_mkdir = io_kiocb_to_cmd(req);
    dismiss_delayed_filename(&mut (*md).filename);
}

pub unsafe fn io_symlinkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let sl: *mut io_link = io_kiocb_to_cmd(req);
    let oldpath: *const core::ffi::c_char;
    let newpath: *const core::ffi::c_char;
    let mut err: i32;
    if (*sqe).len != 0 || (*sqe).rw_flags != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    if ((*req).flags & REQ_F_FIXED_FILE) != 0 { return -EBADF; }
    (*sl).new_dfd = core::ptr::read_volatile(&(*sqe).fd);
    oldpath = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr));
    newpath = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr2));
    err = delayed_getname(&mut (*sl).oldpath, oldpath);
    if err != 0 { return err; }
    err = delayed_getname(&mut (*sl).newpath, newpath);
    if err != 0 { dismiss_delayed_filename(&mut (*sl).oldpath); return err; }
    (*req).flags |= REQ_F_NEED_CLEANUP | REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_symlinkat(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let sl: *mut io_link = io_kiocb_to_cmd(req);
    let old = filename_complete_delayed(&mut (*sl).oldpath);
    let new = filename_complete_delayed(&mut (*sl).newpath);
    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    let ret = filename_symlinkat(old, (*sl).new_dfd, new);
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_linkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let lnk: *mut io_link = io_kiocb_to_cmd(req);
    let oldf: *const core::ffi::c_char;
    let newf: *const core::ffi::c_char;
    let mut err: i32;
    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    if ((*req).flags & REQ_F_FIXED_FILE) != 0 { return -EBADF; }
    (*lnk).old_dfd = core::ptr::read_volatile(&(*sqe).fd);
    (*lnk).new_dfd = core::ptr::read_volatile(&(*sqe).len);
    oldf = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr));
    newf = u64_to_user_ptr(core::ptr::read_volatile(&(*sqe).addr2));
    (*lnk).flags = core::ptr::read_volatile(&(*sqe).hardlink_flags);
    err = delayed_getname_uflags(&mut (*lnk).oldpath, oldf, (*lnk).flags);
    if err != 0 { return err; }
    err = delayed_getname(&mut (*lnk).newpath, newf);
    if err != 0 { dismiss_delayed_filename(&mut (*lnk).oldpath); return err; }
    (*req).flags |= REQ_F_NEED_CLEANUP | REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_linkat(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let lnk: *mut io_link = io_kiocb_to_cmd(req);
    let old = filename_complete_delayed(&mut (*lnk).oldpath);
    let new = filename_complete_delayed(&mut (*lnk).newpath);
    WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    let ret = filename_linkat((*lnk).old_dfd, old, (*lnk).new_dfd, new, (*lnk).flags);
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_link_cleanup(req: *mut io_kiocb) {
    let sl: *mut io_link = io_kiocb_to_cmd(req);
    dismiss_delayed_filename(&mut (*sl).oldpath);
    dismiss_delayed_filename(&mut (*sl).newpath);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
